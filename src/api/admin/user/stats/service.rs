use super::dto::{
    LoginStatsDailyRes, LoginStatsDevicesRes, LoginStatsSummaryRes, UserStatsQuery,
    UserStatsSignupsRes, UserStatsSummaryRes,
};
use crate::error::{AppError, AppResult};
use crate::AppState;
use chrono::NaiveDate;

// ==========================================
// 헬퍼 함수
// ==========================================

fn parse_date_range(from: &str, to: &str) -> AppResult<(NaiveDate, NaiveDate)> {
    let from_date = NaiveDate::parse_from_str(from.trim(), "%Y-%m-%d")
        .map_err(|_| AppError::BadRequest("invalid from date (YYYY-MM-DD)".into()))?;
    let to_date = NaiveDate::parse_from_str(to.trim(), "%Y-%m-%d")
        .map_err(|_| AppError::BadRequest("invalid to date (YYYY-MM-DD)".into()))?;

    if from_date > to_date {
        return Err(AppError::BadRequest("from must be <= to".into()));
    }
    if (to_date - from_date).num_days() > 366 {
        return Err(AppError::BadRequest(
            "range too large (max 366 days)".into(),
        ));
    }

    Ok((from_date, to_date))
}

// ==========================================
// User Stats Services
// ==========================================

/// 7-53: 사용자 요약 통계
pub async fn get_user_stats_summary(
    st: &AppState,
    q: UserStatsQuery,
) -> AppResult<UserStatsSummaryRes> {
    let (from, to) = parse_date_range(&q.from, &q.to)?;

    let (total_users, new_users, active_users, inactive_users, by_role) =
        super::repo::fetch_user_stats_summary(&st.db, from, to).await?;

    Ok(UserStatsSummaryRes {
        total_users,
        new_users,
        active_users,
        inactive_users,
        by_role,
        from_date: from,
        to_date: to,
    })
}

/// 7-54: 일별 가입 통계
pub async fn get_user_stats_signups(
    st: &AppState,
    q: UserStatsQuery,
) -> AppResult<UserStatsSignupsRes> {
    let (from, to) = parse_date_range(&q.from, &q.to)?;

    let items = super::repo::fetch_daily_signups(&st.db, from, to).await?;

    Ok(UserStatsSignupsRes {
        from_date: from,
        to_date: to,
        items,
    })
}

// ==========================================
// Login Stats Services
// ==========================================

/// 7-55: 로그인 요약 통계
pub async fn get_login_stats_summary(
    st: &AppState,
    q: UserStatsQuery,
) -> AppResult<LoginStatsSummaryRes> {
    let (from, to) = parse_date_range(&q.from, &q.to)?;

    let (total_logins, success_count, fail_count, unique_users, active_sessions) =
        super::repo::fetch_login_stats_summary(&st.db, from, to).await?;

    Ok(LoginStatsSummaryRes {
        total_logins,
        success_count,
        fail_count,
        unique_users,
        active_sessions,
        from_date: from,
        to_date: to,
    })
}

/// 7-56: 일별 로그인 통계
pub async fn get_login_stats_daily(
    st: &AppState,
    q: UserStatsQuery,
) -> AppResult<LoginStatsDailyRes> {
    let (from, to) = parse_date_range(&q.from, &q.to)?;

    let items = super::repo::fetch_daily_logins(&st.db, from, to).await?;

    Ok(LoginStatsDailyRes {
        from_date: from,
        to_date: to,
        items,
    })
}

/// 7-57: 디바이스별 통계
pub async fn get_login_stats_devices(
    st: &AppState,
    q: UserStatsQuery,
) -> AppResult<LoginStatsDevicesRes> {
    let (from, to) = parse_date_range(&q.from, &q.to)?;

    let items = super::repo::fetch_device_stats(&st.db, from, to).await?;

    Ok(LoginStatsDevicesRes {
        from_date: from,
        to_date: to,
        items,
    })
}
