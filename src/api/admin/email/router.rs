use axum::{routing::post, Router};

use crate::state::AppState;

use super::handler;

pub fn admin_email_router() -> Router<AppState> {
    Router::new().route("/test", post(handler::send_test_email))
}
