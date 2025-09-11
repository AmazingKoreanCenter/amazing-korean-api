use super::handler::admin_get_video_daily_stats;
use crate::AppState;
use axum::{routing::get, Router};

pub fn admin_stats_router() -> Router<AppState> {
    Router::new().route("/daily", get(admin_get_video_daily_stats))
}
