use axum::{routing::{get, post}, Router};

use crate::state::AppState;

use super::handler;

pub fn ebook_router() -> Router<AppState> {
    Router::new()
        .route("/catalog", get(handler::get_catalog))
        .route("/purchase", post(handler::create_purchase))
        .route("/my", get(handler::get_my_purchases))
        .route("/viewer/{code}/meta", get(handler::get_viewer_meta))
        .route("/viewer/{code}/pages/{page_num}", get(handler::get_page_image))
}
