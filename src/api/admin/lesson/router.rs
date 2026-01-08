use crate::AppState;
use axum::{routing::get, routing::post, Router};

use super::handler::{admin_bulk_create_lessons, admin_create_lesson, admin_list_lessons};

pub fn admin_lesson_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_list_lessons).post(admin_create_lesson))
        .route("/bulk", post(admin_bulk_create_lessons))
}
