use crate::state::AppState;
#[allow(unused_imports)]
use axum::routing::patch;
use axum::{routing::get, routing::post, Router};

use super::handler::{
    admin_create_user, admin_create_users_bulk, admin_get_user, admin_list_users, admin_update_user,
    admin_update_users_bulk,
};

pub fn admin_user_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_list_users).post(admin_create_user))
        .route("/bulk", post(admin_create_users_bulk).patch(admin_update_users_bulk))
        .route("/{user_id}", get(admin_get_user).patch(admin_update_user))
}
