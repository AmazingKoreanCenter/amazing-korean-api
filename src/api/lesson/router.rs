use axum::{routing::get, Router};

use crate::state::AppState;

use super::handler;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(handler::list_lessons))
}
