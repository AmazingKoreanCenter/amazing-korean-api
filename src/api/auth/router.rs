use axum::{routing::{post, get}, Router};
use crate::state::AppState;
use super::handler;

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/auth/signup", post(handler::signup))
        .route("/auth/login",  post(handler::login))
        .route("/me",          get(handler::me))
}
