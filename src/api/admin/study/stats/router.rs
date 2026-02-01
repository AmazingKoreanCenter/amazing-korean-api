use super::handler::{admin_get_daily_stats, admin_get_study_stats_summary, admin_get_top_studies};
use crate::AppState;
use axum::{routing::get, Router};

/// Study 통계 라우터 (/admin/studies/stats 하위)
pub fn admin_study_stats_router() -> Router<AppState> {
    Router::new()
        .route("/summary", get(admin_get_study_stats_summary))
        .route("/top", get(admin_get_top_studies))
        .route("/daily", get(admin_get_daily_stats))
}
