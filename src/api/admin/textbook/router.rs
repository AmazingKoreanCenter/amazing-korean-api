use axum::{routing::{delete, get, patch}, Router};

use crate::state::AppState;

use super::handler;

pub fn admin_textbook_router() -> Router<AppState> {
    Router::new()
        .route("/orders", get(handler::list_orders))
        .route("/orders/{id}", get(handler::get_order))
        .route("/orders/{id}/status", patch(handler::update_status))
        .route("/orders/{id}/tracking", patch(handler::update_tracking))
        .route("/orders/{id}", delete(handler::delete_order))
}
