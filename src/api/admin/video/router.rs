use axum::{
    routing::post,
    Router,
};
use crate::state::AppState;
use super::handler;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(handler::create_video_handler))
}
