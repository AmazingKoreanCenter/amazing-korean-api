use crate::AppState;
use axum::{
    routing::{get, put},
    Router,
};

use super::handler::{admin_list_videos, admin_update_video, create_video_handler};
use super::stats::router::admin_stats_router;

pub fn admin_video_router() -> Router<AppState> {
    Router::new()
        // B1: 생성
        .route("/", get(admin_list_videos).post(create_video_handler))
        // B2: 업데이트
        .route("/{video_id}", put(admin_update_video))
        .nest("/{video_id}/stats", admin_stats_router())
}
