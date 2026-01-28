use super::handler::{
    admin_get_aggregate_daily_stats, admin_get_stats_summary, admin_get_top_videos,
    admin_get_video_daily_stats,
};
use crate::AppState;
use axum::{routing::get, Router};

/// 특정 비디오의 통계 라우터 (/{video_id}/stats 하위)
pub fn admin_stats_router() -> Router<AppState> {
    Router::new().route("/daily", get(admin_get_video_daily_stats))
}

/// 전체 통계 대시보드 라우터 (/stats 하위)
pub fn admin_global_stats_router() -> Router<AppState> {
    Router::new()
        .route("/summary", get(admin_get_stats_summary))
        .route("/top", get(admin_get_top_videos))
        .route("/daily", get(admin_get_aggregate_daily_stats))
}
