use axum::extract::{Path, Query, State};
use axum::{response::IntoResponse, Json};

use crate::api::auth::extractor::AuthUser;
use crate::api::video::repo::VideoRepo;
use crate::error::AppResult;
use crate::state::AppState;

use super::dto::{
    CaptionItem, HealthRes, IdParam, VideoDetail, VideoListItem, VideoProgressRes,
    VideoProgressUpdateReq, VideosQuery,
};
use super::service::VideoService;

#[utoipa::path(
    get,
    path = "/videos/health",
    responses(
        (status = 200, description = "Health check successful", body = HealthRes)
    ),
    tag = "videos"
)]
pub async fn health() -> impl IntoResponse {
    Json(HealthRes { ok: true })
}

#[utoipa::path(
    get,
    path = "/videos",
    params(
        ("q", Query, description = "Search query for title or subtitle"),
        ("tag", Query, description = "Filter by tags (multiple values allowed)"),
        ("lang", Query, description = "Filter by video language"),
        ("access", Query, description = "Filter by access type (e.g., public, paid)"),
        ("state", Query, description = "Filter by video state (e.g., open, ready)"),
        ("limit", Query, description = "Number of items to return (default 20, max 100)"),
        ("offset", Query, description = "Number of items to skip (default 0)"),
        ("sort", Query, description = "Sort by field (created_at, popular, complete_rate - only created_at supported for now)"),
        ("order", Query, description = "Sort order (asc or desc, default desc)"),
    ),
    responses(
        (status = 200, description = "List of videos", body = Vec<VideoListItem>)
    ),
    tag = "videos"
)]
pub async fn list_videos(
    State(state): State<AppState>,
    Query(_q): Query<VideosQuery>,
) -> AppResult<Json<Vec<VideoListItem>>> {
    let _video_service = VideoService::new(VideoRepo::new(state.db.clone()));
    // TODO: Implement list_videos in VideoService
    // let videos = video_service.list_videos(&state, q).await?;
    let videos = vec![]; // Placeholder
    Ok(Json(videos))
}

#[utoipa::path(
    get,
    path = "/videos/{id}",
    params(
        ("id" = i64, Path, description = "Video ID")
    ),
    responses(
        (status = 200, description = "OK", body = VideoDetail),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not Found")
    ),
    tag = "videos"
)]
pub async fn get_video_detail(
    State(state): State<AppState>,
    Path(params): Path<IdParam>,
) -> AppResult<Json<VideoDetail>> {
    let _video_service = VideoService::new(VideoRepo::new(state.db.clone()));
    // TODO: Implement get_video_detail in VideoService
    // let video = video_service.get_video_detail(&state, params.id).await?;
    let video = VideoDetail {
        video_id: params.id,
        video_idx: "".to_string(),
        title: Some("".to_string()),
        subtitle: None,
        duration_seconds: Some(0),
        language: Some("en".to_string()),
        thumbnail_url: Some("".to_string()),
        state: "open".to_string(),
        access: "public".to_string(),
        vimeo_video_id: Some("".to_string()),
        tags: Some(vec![]),
        has_captions: Some(false),
        created_at: chrono::Utc::now(),
    }; // Placeholder
    Ok(Json(video))
}

#[utoipa::path(
    get,
    path = "/videos/{id}/captions",
    params(
        ("id" = i64, Path, description = "Video ID")
    ),
    responses(
        (status = 200, description = "OK", body = Vec<CaptionItem>),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not Found")
    ),
    tag = "videos"
)]
pub async fn list_video_captions(
    State(state): State<AppState>,
    Path(_params): Path<IdParam>,
) -> AppResult<Json<Vec<CaptionItem>>> {
    let _video_service = VideoService::new(VideoRepo::new(state.db.clone()));
    // TODO: Implement list_video_captions in VideoService
    // let captions = video_service.list_video_captions(&state, params.id).await?;
    let captions = vec![]; // Placeholder
    Ok(Json(captions))
}

#[utoipa::path(
    get,
    path = "/videos/{id}/progress",
    params(
        ("id" = i64, Path, description = "Video ID")
    ),
    responses(
        (status = 200, description = "OK", body = VideoProgressRes),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not Found")
    ),
    security(("bearerAuth" = [])),
    tag = "videos"
)]
pub async fn get_video_progress(
    State(state): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(params): Path<IdParam>,
) -> AppResult<Json<VideoProgressRes>> {
    let video_service = VideoService::new(VideoRepo::new(state.db.clone()));
    let progress = video_service
        .get_video_progress(&state, auth_user.sub, params.id)
        .await?;
    Ok(Json(progress))
}

#[utoipa::path(
    put,
    path = "/videos/{id}/progress",
    params(
        ("id" = i64, Path, description = "Video ID")
    ),
    request_body(content = VideoProgressUpdateReq, description = "Video progress update data", content_type = "application/json"),
    responses(
        (status = 200, description = "OK", body = VideoProgressRes),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not Found")
    ),
    security(("bearerAuth" = [])),
    tag = "videos"
)]
pub async fn update_video_progress(
    State(state): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(params): Path<IdParam>,
    Json(req): Json<VideoProgressUpdateReq>,
) -> AppResult<Json<VideoProgressRes>> {
    let video_service = VideoService::new(VideoRepo::new(state.db.clone()));
    let progress = video_service
        .update_video_progress(&state, auth_user.sub, params.id, req)
        .await?;
    Ok(Json(progress))
}
