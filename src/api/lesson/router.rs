use axum::{routing::get, Router};

use crate::state::AppState;

use super::handler;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list_lessons))
        .route("/{id}", get(handler::get_lesson_detail))
        .route("/{id}/items", get(handler::get_lesson_items))
        .route("/{id}/progress", get(handler::get_lesson_progress))
}
