use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::state::AppState;

use super::handler;

pub fn admin_payment_router() -> Router<AppState> {
    Router::new()
        .route("/subscriptions", get(handler::list_subscriptions))
        .route("/subscriptions/{id}", get(handler::get_subscription))
        .route(
            "/subscriptions/{id}/cancel",
            post(handler::cancel_subscription),
        )
        .route("/transactions", get(handler::list_transactions))
        .route("/grants", get(handler::list_grants).post(handler::create_grant))
        .route("/grants/{user_id}", delete(handler::revoke_grant))
}
