use super::handler;
use crate::state::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/auth/signup", post(handler::signup))
        .route("/auth/login", post(handler::login))
        .route("/me", get(handler::me))
}
