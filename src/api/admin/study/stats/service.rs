use super::dto::{DailyStatsRes, StatsQuery, StudyStatsSummaryRes, TopStudiesQuery, TopStudiesRes};
use crate::error::{AppError, AppResult};
use crate::AppState;
use chrono::NaiveDate;

// ==========================================
// Helper Functions
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
// Summary Statistics
// ==========================================

/// Get overall study statistics summary
pub async fn get_stats_summary(st: &AppState, q: StatsQuery) -> AppResult<StudyStatsSummaryRes> {
    let (from, to) = parse_date_range(&q.from, &q.to)?;

    // Fetch all stats in parallel
    let (state_counts, program_counts, total_tasks, (total_attempts, total_solves)) = tokio::try_join!(
        super::repo::fetch_state_counts(&st.db),
        super::repo::fetch_program_counts(&st.db),
        super::repo::fetch_total_tasks(&st.db),
        super::repo::fetch_attempt_stats(&st.db, from, to),
    )?;

    let total_studies = state_counts.ready + state_counts.open + state_counts.close;
    let solve_rate = if total_attempts > 0 {
        (total_solves as f64 / total_attempts as f64) * 100.0
    } else {
        0.0
    };

    Ok(StudyStatsSummaryRes {
        total_studies,
        open_studies: state_counts.open,
        total_tasks,
        total_attempts,
        total_solves,
        solve_rate,
        by_program: program_counts,
        by_state: state_counts,
        from_date: from,
        to_date: to,
    })
}

// ==========================================
// Top Studies
// ==========================================

/// Get top studies by attempts, solves, or solve_rate
pub async fn get_top_studies(st: &AppState, q: TopStudiesQuery) -> AppResult<TopStudiesRes> {
    let (from, to) = parse_date_range(&q.from, &q.to)?;

    // Validate limit (1~50)
    let limit = q.limit.clamp(1, 50);

    // Validate sort_by
    let sort_by = match q.sort_by.as_str() {
        "solves" => "solves",
        "solve_rate" => "solve_rate",
        _ => "attempts", // default
    };

    let items = super::repo::fetch_top_studies(&st.db, from, to, limit, sort_by).await?;

    Ok(TopStudiesRes {
        from_date: from,
        to_date: to,
        sort_by: sort_by.to_string(),
        items,
    })
}

// ==========================================
// Daily Statistics
// ==========================================

/// Get daily statistics (attempts, solves, active users)
pub async fn get_daily_stats(st: &AppState, q: StatsQuery) -> AppResult<DailyStatsRes> {
    let (from, to) = parse_date_range(&q.from, &q.to)?;

    let items = super::repo::fetch_daily_stats(&st.db, from, to).await?;

    Ok(DailyStatsRes {
        from_date: from,
        to_date: to,
        items,
    })
}
