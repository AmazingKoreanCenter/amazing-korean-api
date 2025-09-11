use super::dto::{DailyStatsQuery, DailyStatsRes};
use crate::error::{AppError, AppResult};
use crate::AppState;
use chrono::NaiveDate;

pub async fn get_daily_stats(
    st: &AppState,
    video_id: i64,
    q: DailyStatsQuery,
) -> AppResult<DailyStatsRes> {
    let from = NaiveDate::parse_from_str(q.from.trim(), "%Y-%m-%d")
        .map_err(|_| AppError::BadRequest("invalid from date (YYYY-MM-DD)".into()))?;
    let to = NaiveDate::parse_from_str(q.to.trim(), "%Y-%m-%d")
        .map_err(|_| AppError::BadRequest("invalid to date (YYYY-MM-DD)".into()))?;

    if from > to {
        return Err(AppError::BadRequest("from must be <= to".into()));
    }
    if (to - from).num_days() > 366 {
        return Err(AppError::BadRequest(
            "range too large (max 366 days)".into(),
        ));
    }

    let items = super::repo::fetch_daily_stats(&st.db, video_id, from, to).await?;
    Ok(DailyStatsRes {
        video_id,
        from_date: from,
        to_date: to,
        items,
    })
}
