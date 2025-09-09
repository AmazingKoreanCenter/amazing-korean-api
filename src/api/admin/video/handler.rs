use super::dto::{VideoCreateReq, VideoRes, VideoUpdateReq};
use super::service;
use crate::{
    api::auth::extractor::AuthUser,
    error::{AppResult, ErrorBody},
    state::AppState,
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

#[utoipa::path(
    post,
    path = "/admin/videos",
    request_body = VideoCreateReq,
    responses(
        (status = 201, description = "Video created successfully", body = VideoRes),
        (status = 400, description = "Bad request", body = ErrorBody),
        (status = 401, description = "Unauthorized", body = ErrorBody),
        (status = 403, description = "Forbidden", body = ErrorBody),
        (status = 500, description = "Internal server error", body = ErrorBody),
    ),
    security(
        ("jwt_admin" = [])
    ),
    tag = "admin"
)]
pub async fn create_video_handler(
    State(app_state): State<AppState>,
    AuthUser(claims): AuthUser,
    Json(req): Json<VideoCreateReq>,
) -> AppResult<(StatusCode, Json<VideoRes>)> {
    let res = service::create_video(&app_state, &claims, req).await?;
    Ok((StatusCode::CREATED, Json(res)))
}

#[utoipa::path(
    put,
    path = "/admin/videos/{video_id}",
    request_body = VideoUpdateReq,
    responses(
        (status = 200, description = "Video updated successfully", body = VideoRes),
        (status = 400, description = "Bad request", body = ErrorBody),
        (status = 401, description = "Unauthorized", body = ErrorBody),
        (status = 403, description = "Forbidden", body = ErrorBody),
        (status = 404, description = "Not Found", body = ErrorBody),
        (status = 500, description = "Internal server error", body = ErrorBody),
    ),
    params(
        ("video_id" = i64, Path, description = "ID of the video to update")
    ),
    security(
        ("jwt_admin" = [])
    ),
    tag = "admin"
)]
pub async fn admin_update_video(
    State(app_state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(video_id): Path<i64>,
    Json(req): Json<VideoUpdateReq>,
) -> AppResult<Json<VideoRes>> {
    let res = service::update_video(&app_state, &claims, video_id, req).await?;
    Ok(Json(res))
}
