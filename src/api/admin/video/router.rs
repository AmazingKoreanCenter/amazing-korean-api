use crate::AppState;
use axum::{
    routing::{delete, put},
    Router,
};

use super::handler::{admin_delete_video, /* B2 */ admin_update_video};

pub fn admin_video_router() -> Router<AppState> {
    Router::new()
        // B2: 업데이트
        .route("/{video_id}", put(admin_update_video))
        // B3: 소프트 삭제
        .route("/{video_id}", delete(admin_delete_video))
}
