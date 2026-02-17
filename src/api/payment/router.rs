use axum::{routing::{get, post}, Router};

use crate::state::AppState;

use super::handler;

pub fn payment_router() -> Router<AppState> {
    Router::new()
        .route("/plans", get(handler::get_plans))
        .route("/subscription", get(handler::get_subscription))
        .route("/subscription/cancel", post(handler::cancel_subscription))
        .route("/webhook", post(handler::handle_webhook))
}
