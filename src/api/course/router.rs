use super::handler;
use crate::state::AppState;
use axum::routing::{get /*post*/};

pub fn course_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/courses", get(handler::list).post(handler::create))
        .route("/courses/{id}", get(handler::get_by_id)) // ← 추가
}
