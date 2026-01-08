use crate::AppState;
use axum::{routing::get, Router};

use super::handler::admin_list_lessons;

pub fn admin_lesson_router() -> Router<AppState> {
    Router::new().route("/", get(admin_list_lessons))
}
