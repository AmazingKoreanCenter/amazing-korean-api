use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use axum_extra::extract::cookie::CookieJar;

use crate::{
    api::{
        auth::{dto::*, service::AuthService},
        user::handler::bearer_from_headers,
    },
    error::{AppError, AppResult},
    state::AppState,
};

// IP 주소 추출 헬퍼
fn get_client_ip(headers: &HeaderMap) -> String {
    if let Some(x_forwarded_for) = headers.get("x-forwarded-for") {
        if let Ok(ip) = x_forwarded_for.to_str() {
            return ip.split(',').next().unwrap_or("unknown").to_string();
        }
    }
    "unknown".to_string()
}

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = LoginReq,
    responses(
        (status = 200, description = "Login successful", body = LoginRes, example = json!({
            "token": "eyJ...",
            "expires_in": 900,
            "user": {
                "id": 1,
                "email": "test@example.com",
                "name": "Test User",
                "user_state": "on",
                "user_auth": "user",
                "created_at": "2025-08-21T10:00:00Z"
            }
        })),
        (status = 401, description = "Invalid credentials", body = crate::error::ErrorBody, example = json!({ "error": { "code": "AUTH_INVALID_CREDENTIALS", "http_status": 401, "message": "Invalid credentials" } })),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody, example = json!({ "error": { "code": "AUTH_FORBIDDEN", "http_status": 403, "message": "Forbidden" } })),
        (status = 429, description = "Too many requests", body = crate::error::ErrorBody, example = json!({ "error": { "code": "RATE_LIMIT_EXCEEDED", "http_status": 429, "message": "Too many login attempts" } })),
        (status = 500, description = "Internal server error", body = crate::error::ErrorBody)
    )
)]
pub async fn login(
    State(st): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<LoginReq>,
) -> AppResult<(CookieJar, Json<LoginRes>)> {
    let ip_addr = get_client_ip(&headers);
    let (res, jar) = AuthService::login(&st, req, ip_addr).await?;
    Ok((jar, Json(res)))
}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    tag = "auth",
    responses(
        (status = 200, description = "Token refreshed", body = RefreshRes, example = json!({
            "token": "eyJ...",
            "expires_in": 900
        })),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody, example = json!({ "error": { "code": "AUTH_UNAUTHORIZED", "http_status": 401, "message": "Refresh token missing" } })),
        (status = 500, description = "Internal server error", body = crate::error::ErrorBody)
    ),
    security(("refreshCookie" = []))
)]
pub async fn refresh(
    State(st): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
) -> AppResult<(CookieJar, Json<RefreshRes>)> {
    let ip_addr = get_client_ip(&headers);
    let (res, jar) = AuthService::refresh(&st, jar, ip_addr).await?;
    Ok((jar, Json(res)))
}

#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = "auth",
    responses(
        (status = 204, description = "Logout successful"), 
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody, example = json!({ "error": { "code": "AUTH_UNAUTHORIZED", "http_status": 401, "message": "Refresh token missing" } })),
        (status = 500, description = "Internal server error", body = crate::error::ErrorBody)
    ),
    security(("refreshCookie" = []))
)]
pub async fn logout(
    State(st): State<AppState>,
    jar: CookieJar,
) -> AppResult<(CookieJar, StatusCode)> {
    let jar = AuthService::logout(&st, jar).await?;
    Ok((jar, StatusCode::NO_CONTENT))
}

#[utoipa::path(
    post,
    path = "/auth/logout-all",
    tag = "auth",
    responses(
        (status = 204, description = "Logout all successful"), 
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody, example = json!({ "error": { "code": "AUTH_UNAUTHORIZED", "http_status": 401, "message": "Access token missing" } })),
        (status = 500, description = "Internal server error", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn logout_all(State(st): State<AppState>, headers: HeaderMap) -> AppResult<StatusCode> {
    let token = bearer_from_headers(&headers)?;
    let claims = crate::api::auth::jwt::decode_token(&token)
        .map_err(|_| AppError::Unauthorized("invalid token".into()))?;
    AuthService::logout_all(&st, claims.sub).await?;
    Ok(StatusCode::NO_CONTENT)
}
