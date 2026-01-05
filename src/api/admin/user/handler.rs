// FILE: src/api/admin/user/handler.rs
use axum::{
    extract::{Path, Query, State},
    http::{header::USER_AGENT, HeaderMap},
    Json,
};
use std::net::IpAddr;

use crate::{
    api::auth::{extractor::AuthUser, jwt},
    error::{AppError, AppResult},
    state::AppState,
};

use super::{
    dto::{AdminUpdateUserReq, AdminUserListReq, AdminUserListRes, AdminUserRes},
    service::AdminUserService,
};

// ← 어트리뷰트 내의 json! 매크로를 위해 필요
#[allow(unused_imports)]
use serde_json::json;

fn jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev_secret_change_me".to_string())
}

/// Authorization: Bearer <token> 헤더에서 토큰 추출
fn bearer_from_headers(headers: &HeaderMap) -> AppResult<String> {
    let auth = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("missing Authorization header".into()))?;

    let Some(rest) = auth.strip_prefix("Bearer ") else {
        return Err(AppError::Unauthorized(
            "invalid Authorization scheme".into(),
        ));
    };
    Ok(rest.to_string())
}

fn extract_client_ip(headers: &HeaderMap) -> Option<IpAddr> {
    let forwarded = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .map(|v| v.trim().to_string());

    let direct = headers
        .get("x-real-ip")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.trim().to_string());

    let ip_str = forwarded.or(direct)?;
    ip_str.parse().ok()
}

fn extract_user_agent(headers: &HeaderMap) -> Option<String> {
    headers
        .get(USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string())
}

#[utoipa::path(
    get,
    path = "/admin/users",
    tag = "admin",
    params(
        ("q", Query, description = "Search email or nickname", example = "test"),
        ("sort", Query, description = "Sort field (created_at, email, nickname)", example = "created_at"),
        ("order", Query, description = "Sort order (asc, desc)", example = "desc"),
        ("page", Query, description = "Page number, defaults to 1", example = 1),
        ("size", Query, description = "Page size, defaults to 20 (max 100)", example = 20)
    ),
    responses(
        (status = 200, description = "List of users", body = AdminUserListRes, example = json!({
            "items": [
                {
                    "id": 123,
                    "email": "admin_user@example.com",
                    "nickname": "AdminNick",
                    "role": "learner",
                    "created_at": "2025-08-21T10:00:00Z"
                }
            ],
            "meta": {
                "total_count": 1,
                "total_pages": 1,
                "current_page": 1,
                "per_page": 20
            }
        })),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody),
        (status = 422, description = "Unprocessable Entity", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_list_users(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Query(params): Query<AdminUserListReq>,
) -> AppResult<Json<AdminUserListRes>> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let res =
        AdminUserService::admin_list(&st, auth_user.sub, params, ip_address, user_agent).await?;

    Ok(Json(res))
}

#[utoipa::path(
    get,
    path = "/admin/users/{user_id}",
    tag = "admin",
    params(
        ("user_id", Path, description = "ID of the user to retrieve", example = 123)
    ),
    responses(
        (status = 200, description = "User profile", body = AdminUserRes, example = json!({
            "id": 123,
            "email": "admin_user@example.com",
            "name": "Admin User",
            "nickname": "AdminNick",
            "language": "en",
            "country": "US",
            "birthday": "2000-01-01",
            "gender": "male",
            "user_state": "on",
            "user_auth": "user",
            "created_at": "2025-08-21T10:00:00Z",
            "quit_at": null
        })),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody, example = json!({
            "error": "missing Authorization header"
        })),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody, example = json!({
            "error": "forbidden"
        })),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody, example = json!({
            "error": "not found"
        }))
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_get_user(
    State(st): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<i64>,
) -> AppResult<Json<AdminUserRes>> {
    let token = bearer_from_headers(&headers)?;
    let claims = jwt::decode_token(&token, &jwt_secret())
        .map_err(|_| AppError::Unauthorized("invalid token".into()))?;

    let res = AdminUserService::admin_get(&st, claims.sub, user_id).await?;

    Ok(Json(res))
}

#[utoipa::path(
    put,
    path = "/admin/users/{user_id}",
    tag = "admin",
    params(
        ("user_id", Path, description = "ID of the user to update", example = 123)
    ),
    request_body = AdminUpdateUserReq,
    responses(
        (status = 200, description = "User updated", body = AdminUserRes, example = json!({
            "id": 123,
            "email": "updated_admin@example.com",
            "name": "Updated Admin",
            "nickname": "UpdatedAdminNick",
            "language": "ko",
            "country": "KR",
            "birthday": "1990-12-25",
            "gender": "female",
            "user_state": "off",
            "user_auth": "manager",
            "created_at": "2025-08-21T10:00:00Z",
            "quit_at": "2025-08-22T10:00:00Z"
        })),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody, example = json!({
            "error": "Validation error: email is not valid"
        })),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody, example = json!({
            "error": "missing Authorization header"
        })),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody, example = json!({
            "error": "forbidden"
        })),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody, example = json!({
            "error": "not found"
        })),
        (status = 409, description = "Email already exists", body = crate::error::ErrorBody, example = json!({
            "error": "Email already exists"
        }))
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_update_user(
    State(st): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<i64>,
    Json(req): Json<AdminUpdateUserReq>,
) -> AppResult<Json<AdminUserRes>> {
    let token = bearer_from_headers(&headers)?;
    let claims = jwt::decode_token(&token, &jwt_secret())
        .map_err(|_| AppError::Unauthorized("invalid token".into()))?;

    let res = AdminUserService::admin_update(&st, claims.sub, user_id, req).await?;

    Ok(Json(res))
}
