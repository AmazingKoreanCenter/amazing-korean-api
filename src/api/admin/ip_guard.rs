//! Admin IP Allowlist Guard
//!
//! Admin 라우트에 대한 IP 기반 접근 제어 미들웨어

use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::state::AppState;

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

    // X-Forwarded-For 헤더에서 첫 번째 IP 추출 (프록시/로드밸런서 환경)
    // 없으면 X-Real-IP 확인
    let client_ip = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .or_else(|| {
            request
                .headers()
                .get("x-real-ip")
                .and_then(|h| h.to_str().ok())
                .map(|s| s.trim().to_string())
        });

    // IP를 식별할 수 없으면 보안상 접근 거부
    let Some(ip) = client_ip else {
        tracing::warn!("Admin access denied: Could not determine client IP from headers");
        return (
            StatusCode::FORBIDDEN,
            "Access denied: Client IP could not be determined",
        )
            .into_response();
    };

    if state.cfg.is_admin_ip_allowed(&ip) {
        next.run(request).await
    } else {
        tracing::warn!(
            ip = %ip,
            "Admin access denied: IP not in allowlist"
        );
        (
            StatusCode::FORBIDDEN,
            "Access denied: IP not allowed for admin access",
        )
            .into_response()
    }
}
