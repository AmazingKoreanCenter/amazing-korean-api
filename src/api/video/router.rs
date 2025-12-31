use axum::{routing::get, Router};

use crate::state::AppState;

use super::handler;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list_videos))
        .route("/{id}", get(handler::get_video_detail))
        .route(
            "/{id}/progress",
            get(handler::get_video_progress).post(handler::update_video_progress),
        )
}
