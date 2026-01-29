use super::dto::{
    DailyStatsRes, StatsQuery, StudyStatsSummaryRes, TopStudiesQuery, TopStudiesRes,
};
use crate::api::auth::extractor::AuthUser;
use crate::error::AppError;
use crate::AppState;
use axum::{
    extract::{Query, State},
    Json,
};

// ==========================================
// Statistics Handlers
// ==========================================

/// Study 통계 요약 조회
#[utoipa::path(
    get,
    path = "/admin/studies/stats/summary",
    tag = "admin_study_stats",
    params(StatsQuery),
    responses(
        (status = 200, description = "Statistics summary", body = StudyStatsSummaryRes),
        (status = 400, description = "Invalid date or range", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_get_study_stats_summary(
    State(st): State<AppState>,
    AuthUser(_auth_user): AuthUser,
    Query(q): Query<StatsQuery>,
) -> Result<Json<StudyStatsSummaryRes>, AppError> {
    let res = super::service::get_stats_summary(&st, q).await?;
    Ok(Json(res))
}

/// TOP Study 조회
#[utoipa::path(
    get,
    path = "/admin/studies/stats/top",
    tag = "admin_study_stats",
    params(TopStudiesQuery),
    responses(
        (status = 200, description = "Top studies", body = TopStudiesRes),
        (status = 400, description = "Invalid date or range", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_get_top_studies(
    State(st): State<AppState>,
    AuthUser(_auth_user): AuthUser,
    Query(q): Query<TopStudiesQuery>,
) -> Result<Json<TopStudiesRes>, AppError> {
    let res = super::service::get_top_studies(&st, q).await?;
    Ok(Json(res))
}

/// Study 일별 통계 조회
#[utoipa::path(
    get,
    path = "/admin/studies/stats/daily",
    tag = "admin_study_stats",
    params(StatsQuery),
    responses(
        (status = 200, description = "Daily statistics", body = DailyStatsRes),
        (status = 400, description = "Invalid date or range", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_get_daily_stats(
    State(st): State<AppState>,
    AuthUser(_auth_user): AuthUser,
    Query(q): Query<StatsQuery>,
) -> Result<Json<DailyStatsRes>, AppError> {
    let res = super::service::get_daily_stats(&st, q).await?;
    Ok(Json(res))
}
