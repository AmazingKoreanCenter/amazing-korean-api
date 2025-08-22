use axum::{extract::{Path, Query, State}, http::HeaderMap, Json};
use serde::Deserialize;

use crate::{
    api::auth::jwt,
    error::{AppError, AppResult},
    state::AppState,
};

use super::{dto::{AdminListUsersRes, AdminUserRes, AdminUpdateUserReq, UserState}, service::AdminUserService};

// ← 어트리뷰트 내의 json! 매크로를 위해 필요
#[allow(unused_imports)]
use serde_json::json;

/// Authorization: Bearer <token> 헤더에서 토큰 추출
fn bearer_from_headers(headers: &HeaderMap) -> AppResult<String> {
    let auth = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("missing Authorization header".into()))?;

    let Some(rest) = auth.strip_prefix("Bearer ") else {
        return Err(AppError::Unauthorized("invalid Authorization scheme".into()));
    };
    Ok(rest.to_string())
}

#[derive(Debug, Deserialize)]
pub struct AdminListUsersQueryParams {
    pub query: Option<String>,
    pub state: Option<UserState>,
    pub page: Option<i64>,
    pub size: Option<i64>,
}

#[utoipa::path(
    get,
    path = "/admin/users",
    tag = "admin",
    params(
        ("query", Query, description = "Search query for email, name, or nickname", example = "test"),
        ("state", Query, description = "User state (on or off)", example = "on"),
        ("page", Query, description = "Page number, defaults to 1", example = 1),
        ("size", Query, description = "Page size, defaults to 20 (max 100)", example = 20)
    ),
    responses(
        (status = 200, description = "List of users", body = AdminListUsersRes, example = json!({
            "total": 1,
            "items": [
                {
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
                }
            ]
        })),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody, example = json!({
            "error": "Invalid page or size parameter"
        })),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody, example = json!({
            "error": "missing Authorization header"
        })),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody, example = json!({
            "error": "forbidden"
        }))
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_list_users(
    State(st): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<AdminListUsersQueryParams>,
) -> AppResult<Json<AdminListUsersRes>> {
    let token = bearer_from_headers(&headers)?;
    let claims = jwt::decode_token(&token).map_err(|_| AppError::Unauthorized("invalid token".into()))?;

    let res = AdminUserService::admin_list(
        &st,
        claims.sub,
        params.query,
        params.state,
        params.page,
        params.size,
    )
    .await?;

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
    let claims = jwt::decode_token(&token).map_err(|_| AppError::Unauthorized("invalid token".into()))?;

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
    let claims = jwt::decode_token(&token).map_err(|_| AppError::Unauthorized("invalid token".into()))?;

    let res = AdminUserService::admin_update(&st, claims.sub, user_id, req).await?;

    Ok(Json(res))
}