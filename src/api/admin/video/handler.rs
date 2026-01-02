use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
#[allow(unused_imports)] // Used in admin_update_video
use utoipa::ToSchema;

use crate::api::admin::video::dto::{VideoCreateReq, VideoRes};
#[allow(unused_imports)] // Used in return type
use crate::error::{AppError, AppResult};
use crate::AppState;

// NOTE: 임시 관리자 가드. 추후 RBAC(HYMN/admin/manager)로 교체.
// TODO: replace with real role check extracting actor_user_id from claims.

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
