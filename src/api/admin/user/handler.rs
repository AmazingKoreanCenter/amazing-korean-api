// FILE: src/api/admin/user/handler.rs
use axum::{
    extract::{Path, Query, State},
    http::{header::LOCATION, header::USER_AGENT, HeaderMap, HeaderValue, StatusCode},
    Json,
};
use std::net::IpAddr;

use crate::{
    api::auth::extractor::AuthUser,
    error::{AppError, AppResult},
    state::AppState,
};

use super::{
    dto::{
        AdminBulkCreateReq, AdminBulkCreateRes, AdminBulkUpdateReq, AdminBulkUpdateRes,
        AdminCreateUserReq, AdminUpdateUserReq, AdminUserListReq, AdminUserListRes, AdminUserRes,
    },
    service::AdminUserService,
};

// ← 어트리뷰트 내의 json! 매크로를 위해 필요
#[allow(unused_imports)]
use serde_json::json;

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
    tag = "admin_users",
    params(
        ("q", Query, description = "Search email or nickname", example = "test"),
        ("sort", Query, description = "Sort field (id, created_at, email, nickname, role)", example = "created_at"),
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

    let res = AdminUserService::admin_list_users(
        &st,
        auth_user.sub,
        params,
        ip_address,
        user_agent,
    )
    .await?;

    Ok(Json(res))
}

#[utoipa::path(
    post,
    path = "/admin/users",
    tag = "admin_users",
    request_body = AdminCreateUserReq,
    responses(
        (status = 201, description = "User created", body = AdminUserRes),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody),
        (status = 409, description = "Conflict", body = crate::error::ErrorBody),
        (status = 422, description = "Unprocessable Entity", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_create_user(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Json(req): Json<AdminCreateUserReq>,
) -> AppResult<(StatusCode, HeaderMap, Json<AdminUserRes>)> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let res = AdminUserService::admin_create_user(
        &st,
        auth_user.sub,
        req,
        ip_address,
        user_agent,
    )
    .await?;

    let mut resp_headers = HeaderMap::new();
    let location = format!("/admin/users/{}", res.id);
    let location_val = HeaderValue::from_str(&location)
        .map_err(|e| AppError::Internal(format!("Invalid Location header: {e}")))?;
    resp_headers.insert(LOCATION, location_val);

    Ok((StatusCode::CREATED, resp_headers, Json(res)))
}

#[utoipa::path(
    post,
    path = "/admin/users/bulk",
    tag = "admin_users",
    request_body = AdminBulkCreateReq,
    responses(
        (status = 201, description = "All users created", body = AdminBulkCreateRes),
        (status = 207, description = "Partial success", body = AdminBulkCreateRes),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_create_users_bulk(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Json(req): Json<AdminBulkCreateReq>,
) -> AppResult<(StatusCode, Json<AdminBulkCreateRes>)> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let (all_success, res) = AdminUserService::admin_create_users_bulk(
        &st,
        auth_user.sub,
        req,
        ip_address,
        user_agent,
    )
    .await?;

    let status = if all_success {
        StatusCode::CREATED
    } else {
        StatusCode::MULTI_STATUS
    };

    Ok((status, Json(res)))
}

#[utoipa::path(
    get,
    path = "/admin/users/{user_id}",
    tag = "admin_users",
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
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Path(user_id): Path<i64>,
) -> AppResult<Json<AdminUserRes>> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let res = AdminUserService::admin_get_user(
        &st,
        auth_user.sub,
        user_id,
        ip_address,
        user_agent,
    )
    .await?;

    Ok(Json(res))
}

#[utoipa::path(
    patch,
    path = "/admin/users/{user_id}",
    tag = "admin_users",
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
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Path(user_id): Path<i64>,
    Json(req): Json<AdminUpdateUserReq>,
) -> AppResult<Json<AdminUserRes>> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let res = AdminUserService::admin_update_user(
        &st,
        auth_user.sub,
        user_id,
        req,
        ip_address,
        user_agent,
    )
    .await?;

    Ok(Json(res))
}

#[utoipa::path(
    patch,
    path = "/admin/users/bulk",
    tag = "admin_users",
    request_body = AdminBulkUpdateReq,
    responses(
        (status = 200, description = "All users updated", body = AdminBulkUpdateRes),
        (status = 207, description = "Partial success", body = AdminBulkUpdateRes),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_update_users_bulk(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Json(req): Json<AdminBulkUpdateReq>,
) -> AppResult<(StatusCode, Json<AdminBulkUpdateRes>)> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let (all_success, res) = AdminUserService::admin_update_users_bulk(
        &st,
        auth_user.sub,
        req,
        ip_address,
        user_agent,
    )
    .await?;

    let status = if all_success {
        StatusCode::OK
    } else {
        StatusCode::MULTI_STATUS
    };

    Ok((status, Json(res)))
}

// ==========================================
// User Logs Handlers
// ==========================================

use super::dto::{AdminUserLogsReq, AdminUserLogsRes, UserLogsRes};

#[utoipa::path(
    get,
    path = "/admin/users/{user_id}/admin-logs",
    tag = "admin_users",
    params(
        ("user_id", Path, description = "ID of the user to get logs for"),
        ("page", Query, description = "Page number, defaults to 1"),
        ("size", Query, description = "Page size, defaults to 20")
    ),
    responses(
        (status = 200, description = "Admin change logs for the user", body = AdminUserLogsRes),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody),
        (status = 404, description = "User not found", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_get_user_logs(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(user_id): Path<i64>,
    Query(params): Query<AdminUserLogsReq>,
) -> AppResult<Json<AdminUserLogsRes>> {
    let res = AdminUserService::admin_get_user_logs(
        &st,
        auth_user.sub,
        user_id,
        params.page,
        params.size,
    )
    .await?;

    Ok(Json(res))
}

#[utoipa::path(
    get,
    path = "/admin/users/{user_id}/user-logs",
    tag = "admin_users",
    params(
        ("user_id", Path, description = "ID of the user to get logs for"),
        ("page", Query, description = "Page number, defaults to 1"),
        ("size", Query, description = "Page size, defaults to 20")
    ),
    responses(
        (status = 200, description = "User self-change logs", body = UserLogsRes),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody),
        (status = 404, description = "User not found", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_get_user_self_logs(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(user_id): Path<i64>,
    Query(params): Query<AdminUserLogsReq>,
) -> AppResult<Json<UserLogsRes>> {
    let res = AdminUserService::admin_get_user_self_logs(
        &st,
        auth_user.sub,
        user_id,
        params.page,
        params.size,
    )
    .await?;

    Ok(Json(res))
}
