//! Admin Role Guard
//!
//! Admin 라우트에 대한 역할 기반 접근 제어 미들웨어
//! - HYMN, admin만 접근 허용
//! - manager, learner는 403 Forbidden

use axum::{
    body::Body,
    extract::State,
    http::{header::AUTHORIZATION, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::api::auth::jwt;
use crate::state::AppState;
use crate::types::UserAuth;

/// Admin 역할 검사 미들웨어
///
/// # 허용 역할
/// - HYMN: 모든 권한
/// - admin: Admin 영역 전체 접근
///
/// # 차단 역할
/// - manager: 403 Forbidden (차후 class 기능 구현 시 별도 영역 제공)
/// - learner: 403 Forbidden
///
/// # 사용법
/// ```rust
/// .nest("/admin", admin_router()
///     .layer(middleware::from_fn_with_state(state.clone(), admin_role_guard))
///     .layer(middleware::from_fn_with_state(state.clone(), admin_ip_guard)))
/// ```
pub async fn admin_role_guard(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    // Authorization 헤더에서 토큰 추출
    let token = match extract_bearer_token(&request) {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                "Missing or invalid Authorization header",
            )
                .into_response();
        }
    };

    // JWT 디코딩 및 검증
    let claims = match jwt::decode_token(&token, &state.cfg.jwt_secret) {
        Ok(c) => c,
        Err(_) => {
            return (StatusCode::UNAUTHORIZED, "Invalid token").into_response();
        }
    };

    // 역할 검사: HYMN 또는 admin만 허용
    match claims.role {
        UserAuth::Hymn | UserAuth::Admin => {
            // 허용된 역할 - 다음 핸들러로 진행
            next.run(request).await
        }
        UserAuth::Manager => {
            tracing::warn!(
                user_id = claims.sub,
                role = ?claims.role,
                "Admin access denied: Manager role not allowed (class feature not implemented yet)"
            );
            (
                StatusCode::FORBIDDEN,
                "Access denied: Manager role requires class-based access (coming soon)",
            )
                .into_response()
        }
        UserAuth::Learner => {
            tracing::warn!(
                user_id = claims.sub,
                role = ?claims.role,
                "Admin access denied: Learner role not allowed"
            );
            (
                StatusCode::FORBIDDEN,
                "Access denied: Insufficient permissions for admin access",
            )
                .into_response()
        }
    }
}

/// Authorization 헤더에서 Bearer 토큰 추출
fn extract_bearer_token(request: &Request<Body>) -> Option<String> {
    request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|t| t.to_string())
}
