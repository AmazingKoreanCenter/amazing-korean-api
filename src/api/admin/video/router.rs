use super::handler;
use crate::state::AppState;
use axum::{
    routing::{post, put},
    Router,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(handler::create_video_handler))
        .route("/{video_id}", put(handler::admin_update_video))
}
