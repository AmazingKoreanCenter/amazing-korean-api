use crate::AppState;
use axum::{routing::get, /*routing::post,*/ Router};

use super::handler::{admin_create_lesson, admin_list_lessons};

pub fn admin_lesson_router() -> Router<AppState> {
    Router::new().route("/", get(admin_list_lessons).post(admin_create_lesson))
}
