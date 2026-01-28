use super::dto::{
    AggregateDailyStatsRes, DailyStatsQuery, DailyStatsRes, StatsSummaryRes, TopVideosQuery,
    TopVideosRes,
};
use crate::api::auth::extractor::AuthUser;
use crate::error::AppError;
use crate::AppState;
use axum::{
    extract::{Path, Query, State},
    Json,
};

// ==========================================
// 기존: 특정 비디오 일별 통계
// ==========================================

/// 비디오 일별 통계 조회
#[utoipa::path(
    get,
    path = "/admin/videos/{video_id}/stats/daily",
    tag = "admin_video_stats",
    params(
        ("video_id" = i64, Path, description = "Video ID"),
        ("from" = String, Query, description = "YYYY-MM-DD"),
        ("to" = String, Query, description = "YYYY-MM-DD")
    ),
    responses(
        (status = 200, description = "OK", body = DailyStatsRes),
        (status = 400, description = "Invalid date or range")
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_get_video_daily_stats(
    State(st): State<AppState>,
    Path(video_id): Path<i64>,
    Query(q): Query<DailyStatsQuery>,
) -> Result<Json<DailyStatsRes>, AppError> {
    let res = super::service::get_daily_stats(&st, video_id, q).await?;
    Ok(Json(res))
}

// ==========================================
// 신규: 전체 통계 대시보드용
// ==========================================

/// 전체 통계 요약 조회
#[utoipa::path(
    get,
    path = "/admin/videos/stats/summary",
    tag = "admin_video_stats",
    params(DailyStatsQuery),
    responses(
        (status = 200, description = "Statistics summary", body = StatsSummaryRes),
        (status = 400, description = "Invalid date or range", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_get_stats_summary(
    State(st): State<AppState>,
    AuthUser(_auth_user): AuthUser,
    Query(q): Query<DailyStatsQuery>,
) -> Result<Json<StatsSummaryRes>, AppError> {
    let res = super::service::get_stats_summary(&st, q).await?;
    Ok(Json(res))
}

/// TOP 비디오 조회
#[utoipa::path(
    get,
    path = "/admin/videos/stats/top",
    tag = "admin_video_stats",
    params(TopVideosQuery),
    responses(
        (status = 200, description = "Top videos", body = TopVideosRes),
        (status = 400, description = "Invalid date or range", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_get_top_videos(
    State(st): State<AppState>,
    AuthUser(_auth_user): AuthUser,
    Query(q): Query<TopVideosQuery>,
) -> Result<Json<TopVideosRes>, AppError> {
    let res = super::service::get_top_videos(&st, q).await?;
    Ok(Json(res))
}

/// 전체 비디오 일별 집계 조회
#[utoipa::path(
    get,
    path = "/admin/videos/stats/daily",
    tag = "admin_video_stats",
    params(DailyStatsQuery),
    responses(
        (status = 200, description = "Aggregate daily stats", body = AggregateDailyStatsRes),
        (status = 400, description = "Invalid date or range", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_get_aggregate_daily_stats(
    State(st): State<AppState>,
    AuthUser(_auth_user): AuthUser,
    Query(q): Query<DailyStatsQuery>,
) -> Result<Json<AggregateDailyStatsRes>, AppError> {
    let res = super::service::get_aggregate_daily_stats(&st, q).await?;
    Ok(Json(res))
}
