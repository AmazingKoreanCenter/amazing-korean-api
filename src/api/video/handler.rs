use axum::extract::{Path, Query, State};
use axum::{response::IntoResponse, Json};

use crate::error::AppResult;
use crate::state::AppState;

use super::dto::{CaptionItem, HealthRes, IdParam, VideoDetail, VideoListItem, VideosQuery};
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
    Query(q): Query<VideosQuery>,
) -> AppResult<Json<Vec<VideoListItem>>> {
    let videos = VideoService::list_videos(&state, q).await?;
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
    let video = VideoService::get_video_detail(&state, params.id).await?;
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
    Path(params): Path<IdParam>,
) -> AppResult<Json<Vec<CaptionItem>>> {
    let captions = VideoService::list_video_captions(&state, params.id).await?;
    Ok(Json(captions))
}
