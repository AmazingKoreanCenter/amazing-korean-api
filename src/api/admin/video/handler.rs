use axum::{
    extract::{Path, Query, State},
    http::{header::USER_AGENT, HeaderMap, StatusCode},
    Json,
};
#[allow(unused_imports)] // Used in admin_update_video
use utoipa::ToSchema;

use crate::api::admin::video::dto::{
    AdminVideoListReq, AdminVideoListRes, AdminVideoRes, VideoBulkCreateReq, VideoBulkCreateRes,
    VideoBulkUpdateReq, VideoBulkUpdateRes, VideoCreateReq, VideoTagUpdateReq, VideoUpdateReq,
};
use crate::api::auth::extractor::AuthUser;
#[allow(unused_imports)] // Used in return type
use crate::error::{AppError, AppResult};
use crate::AppState;
use std::net::IpAddr;

// NOTE: 임시 관리자 가드. 추후 RBAC(HYMN/admin/manager)로 교체.
// TODO: replace with real role check extracting actor_user_id from claims.

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
    path = "/admin/videos",
    tag = "admin",
    params(
        ("q", Query, description = "Search title or description", example = "korean"),
        ("sort", Query, description = "Sort field (created_at, views, title)", example = "created_at"),
        ("order", Query, description = "Sort order (asc, desc)", example = "desc"),
        ("page", Query, description = "Page number, defaults to 1", example = 1),
        ("size", Query, description = "Page size, defaults to 20 (max 100)", example = 20)
    ),
    responses(
        (status = 200, description = "List of videos", body = AdminVideoListRes),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody),
        (status = 422, description = "Unprocessable Entity", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_list_videos(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Query(params): Query<AdminVideoListReq>,
) -> AppResult<Json<AdminVideoListRes>> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let res = super::service::admin_list_videos(
        &st,
        auth_user.sub,
        params,
        ip_address,
        user_agent,
    )
    .await?;

    Ok(Json(res))
}

/// 영상 생성
#[utoipa::path(
    post,
    path = "/admin/videos",
    tag = "admin",
    request_body = VideoCreateReq,
    responses(
        (status = 201, description = "Video created", body = AdminVideoRes),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody),
        (status = 409, description = "Conflict", body = crate::error::ErrorBody),
        (status = 422, description = "Unprocessable Entity", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn create_video_handler(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Json(req): Json<VideoCreateReq>,
) -> Result<(StatusCode, Json<AdminVideoRes>), AppError> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let res = super::service::admin_create_video(
        &st,
        auth_user.sub,
        req,
        ip_address,
        user_agent,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(res)))
}

#[utoipa::path(
    post,
    path = "/admin/videos/bulk",
    tag = "admin",
    request_body = VideoBulkCreateReq,
    responses(
        (status = 201, description = "All created", body = VideoBulkCreateRes),
        (status = 207, description = "Partial success", body = VideoBulkCreateRes),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody),
        (status = 409, description = "Conflict", body = crate::error::ErrorBody),
        (status = 422, description = "Unprocessable Entity", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_bulk_create_videos(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Json(req): Json<VideoBulkCreateReq>,
) -> AppResult<(StatusCode, Json<VideoBulkCreateRes>)> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let (all_success, res) = super::service::admin_bulk_create_videos(
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
    patch,
    path = "/admin/videos/bulk",
    tag = "admin",
    request_body = VideoBulkUpdateReq,
    responses(
        (status = 200, description = "All updated", body = VideoBulkUpdateRes),
        (status = 207, description = "Partial success", body = VideoBulkUpdateRes),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody),
        (status = 409, description = "Conflict", body = crate::error::ErrorBody),
        (status = 422, description = "Unprocessable Entity", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_bulk_update_videos(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Json(req): Json<VideoBulkUpdateReq>,
) -> AppResult<(StatusCode, Json<VideoBulkUpdateRes>)> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let (all_success, res) = super::service::admin_bulk_update_videos(
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

#[utoipa::path(
    patch,
    path = "/admin/videos/{video_id}",
    tag = "admin",
    params(
        ("video_id" = i64, Path, description = "Video ID")
    ),
    request_body = VideoUpdateReq,
    responses(
        (status = 200, description = "Video updated", body = AdminVideoRes),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody),
        (status = 404, description = "Video not found", body = crate::error::ErrorBody),
        (status = 409, description = "Conflict", body = crate::error::ErrorBody),
        (status = 422, description = "Unprocessable Entity", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_update_video(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Path(video_id): Path<i64>,
    Json(req): Json<VideoUpdateReq>,
) -> AppResult<Json<AdminVideoRes>> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let res = super::service::admin_update_video(
        &st,
        auth_user.sub,
        video_id,
        req,
        ip_address,
        user_agent,
    )
    .await?;

    Ok(Json(res))
}

#[utoipa::path(
    patch,
    path = "/admin/videos/{video_id}/tags",
    tag = "admin",
    params(
        ("video_id" = i64, Path, description = "Video ID")
    ),
    request_body = VideoTagUpdateReq,
    responses(
        (status = 200, description = "Video tags updated", body = AdminVideoRes),
        (status = 400, description = "Bad request", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody),
        (status = 404, description = "Video not found", body = crate::error::ErrorBody),
        (status = 409, description = "Conflict", body = crate::error::ErrorBody),
        (status = 422, description = "Unprocessable Entity", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_update_video_tags(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Path(video_id): Path<i64>,
    Json(req): Json<VideoTagUpdateReq>,
) -> AppResult<Json<AdminVideoRes>> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let res = super::service::admin_update_video_tags(
        &st,
        auth_user.sub,
        video_id,
        req,
        ip_address,
        user_agent,
    )
    .await?;

    Ok(Json(res))
}
