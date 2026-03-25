use axum::{routing::{get, post}, Router};

use crate::state::AppState;

use super::handler;

pub fn textbook_router() -> Router<AppState> {
    Router::new()
        .route("/catalog", get(handler::get_catalog))
        .route("/orders", post(handler::create_order))
        .route("/orders/{code}", get(handler::get_order_by_code))
        .route("/my", get(handler::get_my_orders))
}
