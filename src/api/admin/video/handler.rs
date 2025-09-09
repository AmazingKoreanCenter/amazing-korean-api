use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
#[allow(unused_imports)] // Used in admin_update_video
use utoipa::ToSchema;

#[allow(unused_imports)] // Used in admin_delete_video
use super::service;
#[allow(unused_imports)] // Used in return type
use crate::error::{AppError, AppResult};
use crate::AppState;

// NOTE: 임시 관리자 가드. 추후 RBAC(HYMN/admin/manager)로 교체.
// TODO: replace with real role check extracting actor_user_id from claims.

/// 소프트 삭제
#[utoipa::path(
    delete,
    path = "/admin/videos/{video_id}",
    tag = "admin",
    params(
        ("video_id" = i64, Path, description = "Video ID")
    ),
    responses(
        (status = 204, description = "Soft deleted (idempotent)"),
        (status = 404, description = "Video not found")
    )
)]
pub async fn admin_delete_video(
    State(st): State<AppState>,
    Path(video_id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let actor_user_id = 0; // TODO: claims.sub
    super::service::delete_video(&st, video_id, actor_user_id).await?;
    Ok(StatusCode::NO_CONTENT)
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
