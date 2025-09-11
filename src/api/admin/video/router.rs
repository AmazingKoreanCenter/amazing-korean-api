use crate::AppState;
use axum::{
    routing::{delete, post, put},
    Router,
};

use super::caption::router::admin_caption_router;
use super::handler::{admin_delete_video, admin_update_video, create_video_handler};
use super::stats::router::admin_stats_router;
use super::tag::router::admin_tag_router; // ← C1 추가

pub fn admin_video_router() -> Router<AppState> {
    Router::new()
        // B1: 생성
        .route("/", post(create_video_handler))
        // B2: 업데이트
        .route("/{video_id}", put(admin_update_video))
        // B3: 소프트 삭제
        .route("/{video_id}", delete(admin_delete_video))
        // B4-1: 자막 생성
        .nest("/{video_id}/captions", admin_caption_router())
        // B5: 태그 매핑
        .nest("/{video_id}/tags", admin_tag_router())
        .nest("/{video_id}/stats", admin_stats_router()) // C1 ← 추가
}
