use super::dto::{
    AggregateDailyStatsRes, DailyStatsQuery, DailyStatsRes, StatsSummaryRes, TopVideosQuery,
    TopVideosRes,
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
// 기존: 특정 비디오 일별 통계
// ==========================================

pub async fn get_daily_stats(
    st: &AppState,
    video_id: i64,
    q: DailyStatsQuery,
) -> AppResult<DailyStatsRes> {
    let (from, to) = parse_date_range(&q.from, &q.to)?;

    let items = super::repo::fetch_daily_stats(&st.db, video_id, from, to).await?;
    Ok(DailyStatsRes {
        video_id,
        from_date: from,
        to_date: to,
        items,
    })
}

// ==========================================
// 신규: 전체 통계 대시보드용
// ==========================================

/// 전체 요약 통계
pub async fn get_stats_summary(st: &AppState, q: DailyStatsQuery) -> AppResult<StatsSummaryRes> {
    let (from, to) = parse_date_range(&q.from, &q.to)?;

    let (total_views, total_completes, active_video_count) =
        super::repo::fetch_stats_summary(&st.db, from, to).await?;

    Ok(StatsSummaryRes {
        total_views,
        total_completes,
        active_video_count,
        from_date: from,
        to_date: to,
    })
}

/// TOP 비디오 조회
pub async fn get_top_videos(st: &AppState, q: TopVideosQuery) -> AppResult<TopVideosRes> {
    let (from, to) = parse_date_range(&q.from, &q.to)?;

    // limit 검증 (1~50)
    let limit = q.limit.clamp(1, 50);

    // sort_by 검증
    let sort_by = if q.sort_by == "completes" {
        "completes"
    } else {
        "views"
    };

    let items = super::repo::fetch_top_videos(&st.db, from, to, limit, sort_by).await?;

    Ok(TopVideosRes {
        from_date: from,
        to_date: to,
        sort_by: sort_by.to_string(),
        items,
    })
}

/// 전체 비디오 일별 집계
pub async fn get_aggregate_daily_stats(
    st: &AppState,
    q: DailyStatsQuery,
) -> AppResult<AggregateDailyStatsRes> {
    let (from, to) = parse_date_range(&q.from, &q.to)?;

    let items = super::repo::fetch_aggregate_daily_stats(&st.db, from, to).await?;

    Ok(AggregateDailyStatsRes {
        from_date: from,
        to_date: to,
        items,
    })
}
