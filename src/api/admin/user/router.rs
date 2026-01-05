use crate::state::AppState;
#[allow(unused_imports)]
use axum::routing::put;
use axum::{routing::get, routing::post, Router};

use super::handler::{
    admin_create_user, admin_create_users_bulk, admin_get_user, admin_list_users, admin_update_user,
};

pub fn admin_user_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_list_users).post(admin_create_user))
        .route("/bulk", post(admin_create_users_bulk))
        .route("/{user_id}", get(admin_get_user).put(admin_update_user))
}
