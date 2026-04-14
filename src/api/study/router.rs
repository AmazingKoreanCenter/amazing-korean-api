use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::state::AppState;

use super::handler;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list_studies))
        .route("/{id}", get(handler::get_study_detail))
        .route("/tasks/{id}", get(handler::get_study_task))
        .route("/tasks/{id}/answer", post(handler::submit_answer))
        .route("/tasks/{id}/status", get(handler::get_task_status))
        .route("/tasks/{id}/explain", get(handler::get_task_explain))
        // Writing practice sessions
        .route(
            "/writing/sessions",
            post(handler::start_writing_session).get(handler::list_writing_sessions),
        )
        .route(
            "/writing/sessions/{id}",
            patch(handler::finish_writing_session),
        )
        .route("/writing/stats", get(handler::get_writing_stats))
        .route("/writing/practice", get(handler::list_writing_practice_seed))
}
