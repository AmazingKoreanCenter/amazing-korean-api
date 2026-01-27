use crate::state::AppState;
#[allow(unused_imports)]
use axum::routing::patch;
use axum::{routing::get, routing::post, Router};

use super::handler::{
    admin_create_user, admin_create_users_bulk, admin_get_user, admin_list_users,
    admin_update_user, admin_update_users_bulk, get_admin_user_logs, get_user_self_logs,
};

pub fn admin_user_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_list_users).post(admin_create_user))
        .route("/bulk", post(admin_create_users_bulk).patch(admin_update_users_bulk))
        .route("/{user_id}", get(admin_get_user).patch(admin_update_user))
        .route("/{user_id}/admin-logs", get(get_admin_user_logs))
        .route("/{user_id}/user-logs", get(get_user_self_logs))
}
