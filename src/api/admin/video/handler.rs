use axum::{
    extract::{Path, Query, State},
    http::{header::USER_AGENT, HeaderMap, StatusCode},
    Json,
};
#[allow(unused_imports)] // Used in admin_update_video
use utoipa::ToSchema;

use crate::api::admin::video::dto::{
    AdminVideoListReq, AdminVideoListRes, VideoCreateReq, VideoRes,
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
    responses( (status = 201, body = VideoRes) )
)]
pub async fn create_video_handler(
    State(st): State<AppState>,
    Json(req): Json<VideoCreateReq>,
) -> Result<(StatusCode, Json<VideoRes>), AppError> {
    // TODO: claims에서 sub 추출
    let actor_user_id = 0;
    let res = super::service::create_video(&st, req, actor_user_id).await?;
    Ok((StatusCode::CREATED, Json(res)))
}

// Placeholder for B2: admin_update_video
#[utoipa::path(
    put,
    path = "/admin/videos/{video_id}",
    tag = "admin",
    params(
        ("video_id" = i64, Path, description = "Video ID")
    ),
    responses(
        (status = 200, description = "Video updated"),
        (status = 404, description = "Video not found")
    )
)]
pub async fn admin_update_video(
    State(_st): State<AppState>,
    Path(_video_id): Path<i64>,
    Json(_body): Json<serde_json::Value>,
) -> Result<StatusCode, AppError> {
    // TODO: Implement actual update logic
    Ok(StatusCode::OK)
}
