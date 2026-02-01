use super::handler::{
    get_login_stats_daily_handler, get_login_stats_devices_handler, get_login_stats_summary_handler,
    get_user_stats_signups_handler, get_user_stats_summary_handler,
};
use crate::AppState;
use axum::{routing::get, Router};

/// User Stats 라우터 (/admin/users/stats 하위)
pub fn admin_user_stats_router() -> Router<AppState> {
    Router::new()
        .route("/summary", get(get_user_stats_summary_handler))
        .route("/signups", get(get_user_stats_signups_handler))
}

/// Login Stats 라우터 (/admin/logins/stats 하위)
pub fn admin_login_stats_router() -> Router<AppState> {
    Router::new()
        .route("/summary", get(get_login_stats_summary_handler))
        .route("/daily", get(get_login_stats_daily_handler))
        .route("/devices", get(get_login_stats_devices_handler))
}
