use super::dto::{TagsModifyReq, VideoTagsRes};
use crate::error::AppError;
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

/// 태그 추가/매핑 (멱등)
#[utoipa::path(
    post,
    path = "/admin/videos/{video_id}/tags",
    tag = "admin",
    request_body = TagsModifyReq,
    params( ("video_id" = i64, Path) ),
    responses( (status = 200, description = "OK", body = VideoTagsRes) )
)]
pub async fn admin_add_tags(
    State(st): State<AppState>,
    Path(video_id): Path<i64>,
    Json(req): Json<TagsModifyReq>,
) -> Result<Json<VideoTagsRes>, AppError> {
    let actor_user_id = 0; // TODO: claims.sub
    let res = super::service::add_tags(&st, video_id, req, actor_user_id).await?;
    Ok(Json(res))
}

/// 태그 매핑 해제 (멱등)
#[utoipa::path(
    delete,
    path = "/admin/videos/{video_id}/tags",
    tag = "admin",
    request_body = TagsModifyReq,
    params( ("video_id" = i64, Path) ),
    responses( (status = 204, description = "No Content") )
)]
pub async fn admin_remove_tags(
    State(st): State<AppState>,
    Path(video_id): Path<i64>,
    Json(req): Json<TagsModifyReq>,
) -> Result<StatusCode, AppError> {
    let actor_user_id = 0; // TODO: claims.sub
    super::service::remove_tags(&st, video_id, req, actor_user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
