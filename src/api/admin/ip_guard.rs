//! Admin IP Allowlist Guard
//!
//! Admin 라우트에 대한 IP 기반 접근 제어 미들웨어

use axum::{
    body::Body,
    extract::State,
    http::{HeaderMap, Request},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::error::AppError;
use crate::state::AppState;

/// 2.5: allowlist 검사용 **신뢰 가능한** 클라이언트 IP 추출.
///
/// 우리 토폴로지(Cloudflare → nginx → app)에서 클라가 위조 불가한 권위 출처는
/// Cloudflare 가 세팅·덮어쓰는 `CF-Connecting-IP` 뿐이다. 클라가 임의로 적을 수
/// 있는 `X-Forwarded-For`(좌측값) / `X-Real-IP` 는 allowlist 우회 수단이므로
/// **사용하지 않는다**. 부재 시 `None` → 호출부에서 fail-closed.
fn trusted_client_ip(headers: &HeaderMap) -> Option<String> {
    headers
        .get("cf-connecting-ip")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Admin IP allowlist 검사 미들웨어
///
/// # 동작
/// - `ADMIN_IP_ALLOWLIST` 환경변수가 비어있으면 모든 IP 허용
/// - 설정된 경우 해당 IP/CIDR만 허용
/// - 허용되지 않은 IP는 403 Forbidden 반환
///
/// # 사용법
/// ```ignore
/// // app_router에서 admin 라우트에 적용
/// .nest("/admin", admin_router()
///     .layer(middleware::from_fn_with_state(state.clone(), admin_ip_guard)))
/// ```
pub async fn admin_ip_guard(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    // allowlist가 비어있으면 IP 체크 없이 모든 요청 허용 (개발 환경)
    if state.cfg.admin_ip_allowlist.is_empty() {
        return next.run(request).await;
    }

    // 2.5: 위조 불가 권위 출처(CF-Connecting-IP)만 사용 (위 trusted_client_ip).
    let client_ip = trusted_client_ip(request.headers());

    // CF-Connecting-IP 부재 = Cloudflare 미경유/헤더 누락 → IP 검증 불가.
    // allowlist(보안 통제)는 검증 불가 시 fail-closed(거부)가 정공법.
    // (allowlist 미설정 시 위 early-return 으로 여기 미도달 = 영향 admin 한정)
    let Some(ip) = client_ip else {
        tracing::warn!(
            target: "security.admin_ip_guard",
            "Admin access denied: CF-Connecting-IP 부재 — IP 검증 불가 (fail-closed)"
        );
        return AppError::Forbidden("Access denied: client IP unverifiable".into()).into_response();
    };

    if state.cfg.is_admin_ip_allowed(&ip) {
        next.run(request).await
    } else {
        tracing::warn!(
            ip = %ip,
            "Admin access denied: IP not in allowlist"
        );
        AppError::Forbidden("Access denied: IP not allowed for admin access".into()).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::trusted_client_ip;
    use axum::http::{HeaderMap, HeaderValue};

    fn headers(pairs: &[(&'static str, &str)]) -> HeaderMap {
        let mut h = HeaderMap::new();
        for (k, v) in pairs {
            h.insert(*k, HeaderValue::from_str(v).unwrap());
        }
        h
    }

    #[test]
    fn cf_connecting_ip_is_used() {
        let h = headers(&[("cf-connecting-ip", "203.0.113.9")]);
        assert_eq!(trusted_client_ip(&h).as_deref(), Some("203.0.113.9"));
    }

    #[test]
    fn cf_connecting_ip_is_trimmed() {
        let h = headers(&[("cf-connecting-ip", "  203.0.113.9 ")]);
        assert_eq!(trusted_client_ip(&h).as_deref(), Some("203.0.113.9"));
    }

    #[test]
    fn spoofed_x_forwarded_for_is_ignored() {
        // 2.5 핵심: 클라가 허용 IP 를 XFF 좌측에 위조해도 무시 (우회 차단)
        let h = headers(&[("x-forwarded-for", "10.0.0.1, 1.2.3.4")]);
        assert_eq!(trusted_client_ip(&h), None);
    }

    #[test]
    fn x_real_ip_is_ignored() {
        let h = headers(&[("x-real-ip", "10.0.0.1")]);
        assert_eq!(trusted_client_ip(&h), None);
    }

    #[test]
    fn cf_wins_over_spoofed_xff() {
        let h = headers(&[
            ("x-forwarded-for", "10.0.0.1"),
            ("cf-connecting-ip", "203.0.113.9"),
        ]);
        assert_eq!(trusted_client_ip(&h).as_deref(), Some("203.0.113.9"));
    }

    #[test]
    fn absent_cf_connecting_ip_is_none_fail_closed() {
        assert_eq!(trusted_client_ip(&HeaderMap::new()), None);
    }

    #[test]
    fn empty_cf_connecting_ip_is_none() {
        let h = headers(&[("cf-connecting-ip", "   ")]);
        assert_eq!(trusted_client_ip(&h), None);
    }
}
