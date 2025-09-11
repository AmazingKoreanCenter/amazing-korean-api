#![allow(unused_imports)]

use super::handler::{admin_create_caption, admin_delete_caption, admin_update_caption};
use crate::AppState;
use axum::{
    routing::{delete, post, put},
    Router,
};

pub fn admin_caption_router() -> Router<AppState> {
    Router::new().route("/", post(admin_create_caption)).route(
        "/{caption_id}",
        put(admin_update_caption).delete(admin_delete_caption),
    )
}
