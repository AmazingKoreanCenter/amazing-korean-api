use crate::state::AppState;
use axum::{routing::get, Router};
#[allow(unused_imports)]
use axum::routing::put;

use super::handler::{admin_get_user, admin_list_users, admin_update_user};

pub fn admin_user_router() -> Router<AppState> {
    Router::new()
        .route("/users", get(admin_list_users))
        .route("/users/{user_id}", get(admin_get_user).put(admin_update_user))
}
