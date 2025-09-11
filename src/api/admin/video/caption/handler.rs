use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use super::dto::{CaptionCreateReq, CaptionRes, CaptionUpdateReq};
use super::service;
use crate::error::AppError;
use crate::AppState;

// NOTE: 임시 관리자 가드. 추후 RBAC(HYMN/admin/manager)로 교체.
// TODO: replace with real role check extracting actor_user_id from claims.

/// 자막 생성
#[utoipa::path(
    post,
    path = "/admin/videos/{video_id}/captions",
    tag = "admin",
    params(
        ("video_id" = i64, Path, description = "Video ID")
    ),
    request_body = CaptionCreateReq,
    responses(
        (status = 201, description = "Caption created successfully", body = CaptionRes),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Video not found"),
        (status = 409, description = "Conflict: Caption with same lang_code and kind already exists"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn admin_create_caption(
    State(st): State<AppState>,
    Path(video_id): Path<i64>,
    Json(req): Json<CaptionCreateReq>,
) -> Result<(StatusCode, Json<CaptionRes>), AppError> {
    let actor_user_id = 0; // TODO: claims.sub
    let caption = service::create_caption(&st, video_id, req, actor_user_id).await?;
    Ok((StatusCode::CREATED, Json(caption)))
}

/// 자막 수정
#[utoipa::path(
    put,
    path = "/admin/videos/{video_id}/captions/{caption_id}",
    tag = "admin",
    params(
        ("video_id" = i64, Path, description = "Video ID"),
        ("caption_id" = i64, Path, description = "Caption ID")
    ),
    request_body = CaptionUpdateReq,
    responses(
        (status = 200, description = "Caption updated successfully", body = CaptionRes),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Caption not found"),
        (status = 409, description = "Conflict: Caption with same lang_code and kind already exists"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn admin_update_caption(
    State(st): State<AppState>,
    Path((video_id, caption_id)): Path<(i64, i64)>,
    Json(req): Json<CaptionUpdateReq>,
) -> Result<Json<CaptionRes>, AppError> {
    let actor_user_id = 0; // TODO: claims.sub
    let caption = service::update_caption(&st, video_id, caption_id, req, actor_user_id).await?;
    Ok(Json(caption))
}

/// 자막 삭제 (소프트 삭제)
#[utoipa::path(
    delete,
    path = "/admin/videos/{video_id}/captions/{caption_id}",
    tag = "admin",
    params(
        ("video_id" = i64, Path, description = "Video ID"),
        ("caption_id" = i64, Path, description = "Caption ID")
    ),
    responses(
        (status = 204, description = "Caption deleted successfully"),
        (status = 404, description = "Caption not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn admin_delete_caption(
    State(st): State<AppState>,
    Path((video_id, caption_id)): Path<(i64, i64)>,
) -> Result<StatusCode, AppError> {
    let actor_user_id = 0; // TODO: claims.sub
    service::delete_caption(&st, video_id, caption_id, actor_user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
