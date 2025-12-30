// FILE: src/api/auth/router.rs
use axum::{routing::post, Router};

use crate::state::AppState;

use super::handler;

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(handler::login))
        .route("/logout", post(handler::logout))
        .route("/logout/all", post(handler::logout_all))
        .route("/refresh", post(handler::refresh))
        .route("/find-id", post(handler::find_id))
        .route("/reset-pw", post(handler::reset_password))
}
