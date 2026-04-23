use axum::{routing::{delete, get, patch}, Router};

use crate::state::AppState;

use super::handler;

pub fn admin_textbook_router() -> Router<AppState> {
    Router::new()
        .route("/orders", get(handler::list_orders).post(handler::admin_create_order))
        .route("/orders/{id}", get(handler::get_order))
        .route("/orders/{id}/status", patch(handler::update_status))
        .route("/orders/{id}/discount", patch(handler::update_discount))
        .route("/orders/{id}/tracking", patch(handler::update_tracking))
        .route("/orders/{id}", delete(handler::delete_order))
        // Q6 (2026-04-22): admin_textbook_log 감사 로그 조회
        .route("/logs", get(handler::list_admin_logs))
}
