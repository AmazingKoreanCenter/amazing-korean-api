use axum::{
    routing::{delete, get, patch},
    Router,
};

use crate::state::AppState;

use super::handler;

pub fn admin_ebook_router() -> Router<AppState> {
    Router::new()
        .route("/purchases", get(handler::list_purchases))
        .route("/purchases/{id}", get(handler::get_purchase))
        .route("/purchases/{id}/status", patch(handler::update_status))
        .route("/purchases/{id}", delete(handler::delete_purchase))
        .route("/verify/{watermark_id}", get(handler::verify_watermark))
}
