use axum::{
    extract::{State, Json},
    http::StatusCode,
};
use crate::{
    state::AppState,
    error::{AppResult, ErrorBody},
    api::auth::extractor::AuthUser,
};
use super::dto::{VideoCreateReq, VideoRes};
use super::service;

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
