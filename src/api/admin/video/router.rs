use crate::AppState;
use axum::{
    routing::{get, patch, post},
    Router,
};

use super::handler::{
    admin_bulk_create_videos, admin_bulk_update_videos, admin_get_video, admin_list_videos,
    admin_update_video, admin_bulk_update_video_tags, admin_update_video_tags, admin_create_video,
    admin_get_vimeo_preview, admin_create_vimeo_upload_ticket,
};
use super::stats::router::{admin_global_stats_router, admin_stats_router};

pub fn admin_video_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_list_videos).post(admin_create_video))
        .route("/bulk", post(admin_bulk_create_videos).patch(admin_bulk_update_videos))
        .route("/bulk/tags", patch(admin_bulk_update_video_tags))
        .route("/vimeo/preview", get(admin_get_vimeo_preview))
        .route("/vimeo/upload-ticket", post(admin_create_vimeo_upload_ticket))
        // 전체 통계 대시보드
        .nest("/stats", admin_global_stats_router())
        // B2: 조회/업데이트
        .route("/{video_id}", get(admin_get_video).patch(admin_update_video))
        .route("/{video_id}/tags", patch(admin_update_video_tags))
        .nest("/{video_id}/stats", admin_stats_router())
}
