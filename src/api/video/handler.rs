use axum::extract::{Path, Query, State};
use axum::Json;

use crate::api::auth::extractor::AuthUser;
use crate::error::AppResult;
use crate::state::AppState;

use super::dto::{
    IdParam, VideoDetailRes, VideoListReq, VideoListRes, VideoProgressRes, VideoProgressUpdateReq,
};
use super::service::VideoService;

// =========================================================================
// Video Handlers
// =========================================================================

/// 비디오 목록 조회 (검색, 필터, 페이징)
#[utoipa::path(
    get,
    path = "/videos",
    params(
        ("page" = Option<u64>, Query, description = "Page number (default 1)"),
        ("per_page" = Option<u64>, Query, description = "Items per page (default 20, max 100)"),
        ("q" = Option<String>, Query, description = "Search query (title or subtitle)"),
        ("tag" = Option<String>, Query, description = "Filter by tag key"),
        ("lang" = Option<String>, Query, description = "Filter by language (ko, en)"),
        ("state" = Option<String>, Query, description = "Filter by state (open, etc)"),
        ("sort" = Option<String>, Query, description = "Sort order (latest, oldest, views)")
    ),
    responses(
        (status = 200, description = "List of videos", body = VideoListRes),
        (status = 400, description = "Bad Request", body = crate::error::ErrorBody),
        (status = 422, description = "Validation Error", body = crate::error::ErrorBody)
    ),
    tag = "videos"
)]
pub async fn list_videos(
    State(state): State<AppState>,
    Query(req): Query<VideoListReq>,
) -> AppResult<Json<VideoListRes>> {
    // Stateless Service 호출
    let res = VideoService::list_videos(&state, req).await?;
    Ok(Json(res))
}

/// 비디오 상세 조회
#[utoipa::path(
    get,
    path = "/videos/{id}",
    params(
        ("id" = i64, Path, description = "Video ID")
    ),
    responses(
        (status = 200, description = "Video Detail", body = VideoDetailRes),
        (status = 404, description = "Video Not Found", body = crate::error::ErrorBody)
    ),
    tag = "videos"
)]
pub async fn get_video_detail(
    State(state): State<AppState>,
    Path(IdParam { id }): Path<IdParam>,
) -> AppResult<Json<VideoDetailRes>> {
    let video = VideoService::get_video_detail(&state, id).await?;
    Ok(Json(video))
}

/// 내 학습 진도 조회
#[utoipa::path(
    get,
    path = "/videos/{id}/progress",
    params(
        ("id" = i64, Path, description = "Video ID")
    ),
    responses(
        (status = 200, description = "My Progress", body = VideoProgressRes),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 404, description = "Video Not Found", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = [])),
    tag = "videos"
)]
pub async fn get_video_progress(
    State(state): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(IdParam { id }): Path<IdParam>,
) -> AppResult<Json<VideoProgressRes>> {
    let progress = VideoService::get_video_progress(&state, auth_user.sub, id).await?;
    Ok(Json(progress))
}

/// 학습 진도 업데이트 (이어보기/완료)
#[utoipa::path(
    post,
    path = "/videos/{id}/progress",
    params(
        ("id" = i64, Path, description = "Video ID")
    ),
    request_body = VideoProgressUpdateReq,
    responses(
        (status = 200, description = "Progress Updated", body = VideoProgressRes),
        (status = 400, description = "Bad Request", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 404, description = "Video Not Found", body = crate::error::ErrorBody),
        (status = 422, description = "Validation Error", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = [])),
    tag = "videos"
)]
pub async fn update_video_progress(
    State(state): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(IdParam { id }): Path<IdParam>,
    Json(req): Json<VideoProgressUpdateReq>,
) -> AppResult<Json<VideoProgressRes>> {
    let progress = VideoService::update_video_progress(&state, auth_user.sub, id, req).await?;
    Ok(Json(progress))
}