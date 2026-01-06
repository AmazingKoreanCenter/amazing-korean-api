use crate::AppState;
use axum::{
    routing::{get, patch, post},
    Router,
};

use super::handler::{
    admin_bulk_create_videos, admin_bulk_update_videos, admin_list_videos, admin_update_video,
    admin_bulk_update_video_tags, admin_update_video_tags, create_video_handler,
};
use super::stats::router::admin_stats_router;

pub fn admin_video_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_list_videos).post(create_video_handler))
        .route("/bulk", post(admin_bulk_create_videos).patch(admin_bulk_update_videos))
        .route("/bulk/tags", patch(admin_bulk_update_video_tags))
        // B2: 업데이트
        .route("/{video_id}", patch(admin_update_video))
        .route("/{video_id}/tags", patch(admin_update_video_tags))
        .nest("/{video_id}/stats", admin_stats_router())
}
