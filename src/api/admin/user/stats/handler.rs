use super::dto::{
    LoginStatsDailyRes, LoginStatsDevicesRes, LoginStatsSummaryRes, UserStatsQuery,
    UserStatsSignupsRes, UserStatsSummaryRes,
};
use crate::api::auth::extractor::AuthUser;
use crate::error::AppError;
use crate::AppState;
use axum::{
    extract::{Query, State},
    Json,
};

// ==========================================
// User Stats Handlers
// ==========================================

/// 7-53: 사용자 요약 통계 조회
#[utoipa::path(
    get,
    path = "/admin/users/stats/summary",
    tag = "admin_user_stats",
    params(UserStatsQuery),
    responses(
        (status = 200, description = "User statistics summary", body = UserStatsSummaryRes),
        (status = 400, description = "Invalid date or range", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
    ),
    security(("bearerAuth" = []))
)]
pub async fn get_user_stats_summary_handler(
    State(st): State<AppState>,
    AuthUser(_auth_user): AuthUser,
    Query(q): Query<UserStatsQuery>,
) -> Result<Json<UserStatsSummaryRes>, AppError> {
    let res = super::service::get_user_stats_summary(&st, q).await?;
    Ok(Json(res))
}

/// 7-54: 일별 가입 통계 조회
#[utoipa::path(
    get,
    path = "/admin/users/stats/signups",
    tag = "admin_user_stats",
    params(UserStatsQuery),
    responses(
        (status = 200, description = "Daily signup statistics", body = UserStatsSignupsRes),
        (status = 400, description = "Invalid date or range", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
    ),
    security(("bearerAuth" = []))
)]
pub async fn get_user_stats_signups_handler(
    State(st): State<AppState>,
    AuthUser(_auth_user): AuthUser,
    Query(q): Query<UserStatsQuery>,
) -> Result<Json<UserStatsSignupsRes>, AppError> {
    let res = super::service::get_user_stats_signups(&st, q).await?;
    Ok(Json(res))
}

// ==========================================
// Login Stats Handlers
// ==========================================

/// 7-55: 로그인 요약 통계 조회
#[utoipa::path(
    get,
    path = "/admin/logins/stats/summary",
    tag = "admin_login_stats",
    params(UserStatsQuery),
    responses(
        (status = 200, description = "Login statistics summary", body = LoginStatsSummaryRes),
        (status = 400, description = "Invalid date or range", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
    ),
    security(("bearerAuth" = []))
)]
pub async fn get_login_stats_summary_handler(
    State(st): State<AppState>,
    AuthUser(_auth_user): AuthUser,
    Query(q): Query<UserStatsQuery>,
) -> Result<Json<LoginStatsSummaryRes>, AppError> {
    let res = super::service::get_login_stats_summary(&st, q).await?;
    Ok(Json(res))
}

/// 7-56: 일별 로그인 통계 조회
#[utoipa::path(
    get,
    path = "/admin/logins/stats/daily",
    tag = "admin_login_stats",
    params(UserStatsQuery),
    responses(
        (status = 200, description = "Daily login statistics", body = LoginStatsDailyRes),
        (status = 400, description = "Invalid date or range", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
    ),
    security(("bearerAuth" = []))
)]
pub async fn get_login_stats_daily_handler(
    State(st): State<AppState>,
    AuthUser(_auth_user): AuthUser,
    Query(q): Query<UserStatsQuery>,
) -> Result<Json<LoginStatsDailyRes>, AppError> {
    let res = super::service::get_login_stats_daily(&st, q).await?;
    Ok(Json(res))
}

/// 7-57: 디바이스별 통계 조회
#[utoipa::path(
    get,
    path = "/admin/logins/stats/devices",
    tag = "admin_login_stats",
    params(UserStatsQuery),
    responses(
        (status = 200, description = "Device statistics", body = LoginStatsDevicesRes),
        (status = 400, description = "Invalid date or range", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
    ),
    security(("bearerAuth" = []))
)]
pub async fn get_login_stats_devices_handler(
    State(st): State<AppState>,
    AuthUser(_auth_user): AuthUser,
    Query(q): Query<UserStatsQuery>,
) -> Result<Json<LoginStatsDevicesRes>, AppError> {
    let res = super::service::get_login_stats_devices(&st, q).await?;
    Ok(Json(res))
}
