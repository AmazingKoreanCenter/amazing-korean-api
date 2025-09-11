use super::dto::DailyStatItem;
use crate::error::AppResult;
use sqlx::{PgPool, Row};

/// 기간 내 일자 제로필 + 집계값
pub async fn fetch_daily_stats(
    db: &PgPool,
    video_id: i64,
    from: chrono::NaiveDate,
    to: chrono::NaiveDate,
) -> AppResult<Vec<DailyStatItem>> {
    let rows = sqlx::query(
        r#"
        WITH days AS (
            SELECT generate_series($2::date, $3::date, interval '1 day')::date AS stat_date
        ),
        agg AS (
            SELECT stat_date::date AS stat_date,
                   COALESCE(SUM(views), 0) AS views,
                   COALESCE(SUM(watch_seconds), 0) AS watch_seconds
            FROM video_stats_daily
            WHERE video_id = $1
              AND stat_date BETWEEN $2 AND $3
            GROUP BY stat_date
        )
        SELECT d.stat_date,
               COALESCE(a.views, 0) AS views,
               COALESCE(a.watch_seconds, 0) AS watch_seconds
        FROM days d
        LEFT JOIN agg a ON a.stat_date = d.stat_date
        ORDER BY d.stat_date ASC
        "#,
    )
    .bind(video_id)
    .bind(from)
    .bind(to)
    .fetch_all(db)
    .await?;

    let items = rows
        .into_iter()
        .map(|r| DailyStatItem {
            date: r.try_get::<chrono::NaiveDate, _>("stat_date").unwrap(),
            views: r.try_get::<i64, _>("views").unwrap_or(0),
            watch_seconds: r.try_get::<i64, _>("watch_seconds").unwrap_or(0),
        })
        .collect::<Vec<_>>();

    Ok(items)
}
