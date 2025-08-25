use super::handler;
use crate::state::AppState;
use axum::{
    routing::{post},
    Router,
};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(handler::login))
        .route("/refresh", post(handler::refresh))
        .route("/logout", post(handler::logout))
        .route("/logout-all", post(handler::logout_all))
}
