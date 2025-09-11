use super::handler::{admin_add_tags, admin_remove_tags};
use crate::AppState;
use axum::{
    routing::{delete, post},
    Router,
};

pub fn admin_tag_router() -> Router<AppState> {
    Router::new()
        .route("/", post(admin_add_tags))
        .route("/", delete(admin_remove_tags))
}
