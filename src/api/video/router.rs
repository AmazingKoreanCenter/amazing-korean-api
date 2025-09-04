use axum::{routing::get, Router};

use crate::state::AppState;

use super::handler;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(handler::health))
        .route("/", get(handler::list_videos))
        .route("/{id}", get(handler::get_video_detail))
        .route("/{id}/captions", get(handler::list_video_captions))
}
