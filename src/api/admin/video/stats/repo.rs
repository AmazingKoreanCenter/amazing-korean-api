use super::dto::{DailyStatItem, TopVideoItem};
use crate::error::AppResult;
use sqlx::{PgPool, Row};

// ==========================================
// 기존: 특정 비디오 일별 통계
// ==========================================

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
            SELECT video_stat_date AS stat_date,
                   COALESCE(SUM(video_stat_views), 0) AS views,
                   COALESCE(SUM(video_stat_completes), 0) AS completes
            FROM video_stat_daily
            WHERE video_id = $1
              AND video_stat_date BETWEEN $2 AND $3
            GROUP BY video_stat_date
        )
        SELECT d.stat_date,
               COALESCE(a.views, 0) AS views,
               COALESCE(a.completes, 0) AS completes
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
            completes: r.try_get::<i64, _>("completes").unwrap_or(0),
        })
        .collect::<Vec<_>>();

    Ok(items)
}

// ==========================================
// 신규: 전체 통계 대시보드용
// ==========================================

/// 전체 요약 통계 (총 조회수, 총 완료수, 활성 비디오 수)
pub async fn fetch_stats_summary(
    db: &PgPool,
    from: chrono::NaiveDate,
    to: chrono::NaiveDate,
) -> AppResult<(i64, i64, i64)> {
    let row = sqlx::query(
        r#"
        SELECT
            COALESCE(SUM(video_stat_views), 0) AS total_views,
            COALESCE(SUM(video_stat_completes), 0) AS total_completes,
            COUNT(DISTINCT video_id) AS active_video_count
        FROM video_stat_daily
        WHERE video_stat_date BETWEEN $1 AND $2
        "#,
    )
    .bind(from)
    .bind(to)
    .fetch_one(db)
    .await?;

    Ok((
        row.try_get::<i64, _>("total_views").unwrap_or(0),
        row.try_get::<i64, _>("total_completes").unwrap_or(0),
        row.try_get::<i64, _>("active_video_count").unwrap_or(0),
    ))
}

/// TOP 비디오 조회 (조회수 또는 완료수 기준 정렬)
pub async fn fetch_top_videos(
    db: &PgPool,
    from: chrono::NaiveDate,
    to: chrono::NaiveDate,
    limit: i32,
    sort_by: &str,
) -> AppResult<Vec<TopVideoItem>> {
    let order_clause = if sort_by == "completes" {
        "total_completes DESC, total_views DESC"
    } else {
        "total_views DESC, total_completes DESC"
    };

    let query = format!(
        r#"
        SELECT
            v.video_id,
            v.video_idx,
            vt.video_tag_title AS title,
            COALESCE(SUM(s.video_stat_views), 0) AS total_views,
            COALESCE(SUM(s.video_stat_completes), 0) AS total_completes
        FROM video v
        LEFT JOIN video_stat_daily s
            ON s.video_id = v.video_id
            AND s.video_stat_date BETWEEN $1 AND $2
        LEFT JOIN video_tag_map vtm ON vtm.video_id = v.video_id
        LEFT JOIN video_tag vt ON vt.video_tag_id = vtm.video_tag_id
        GROUP BY v.video_id, v.video_idx, vt.video_tag_title
        ORDER BY {}
        LIMIT $3
        "#,
        order_clause
    );

    let rows = sqlx::query(&query)
        .bind(from)
        .bind(to)
        .bind(limit)
        .fetch_all(db)
        .await?;

    let items = rows
        .into_iter()
        .enumerate()
        .map(|(i, r)| TopVideoItem {
            rank: (i + 1) as i32,
            video_id: r.try_get::<i64, _>("video_id").unwrap_or(0),
            video_idx: r.try_get::<String, _>("video_idx").unwrap_or_default(),
            title: r.try_get::<Option<String>, _>("title").unwrap_or(None),
            views: r.try_get::<i64, _>("total_views").unwrap_or(0),
            completes: r.try_get::<i64, _>("total_completes").unwrap_or(0),
        })
        .collect::<Vec<_>>();

    Ok(items)
}

/// 전체 비디오 일별 집계 (모든 비디오 합산)
pub async fn fetch_aggregate_daily_stats(
    db: &PgPool,
    from: chrono::NaiveDate,
    to: chrono::NaiveDate,
) -> AppResult<Vec<DailyStatItem>> {
    let rows = sqlx::query(
        r#"
        WITH days AS (
            SELECT generate_series($1::date, $2::date, interval '1 day')::date AS stat_date
        ),
        agg AS (
            SELECT video_stat_date AS stat_date,
                   COALESCE(SUM(video_stat_views), 0) AS views,
                   COALESCE(SUM(video_stat_completes), 0) AS completes
            FROM video_stat_daily
            WHERE video_stat_date BETWEEN $1 AND $2
            GROUP BY video_stat_date
        )
        SELECT d.stat_date,
               COALESCE(a.views, 0) AS views,
               COALESCE(a.completes, 0) AS completes
        FROM days d
        LEFT JOIN agg a ON a.stat_date = d.stat_date
        ORDER BY d.stat_date ASC
        "#,
    )
    .bind(from)
    .bind(to)
    .fetch_all(db)
    .await?;

    let items = rows
        .into_iter()
        .map(|r| DailyStatItem {
            date: r.try_get::<chrono::NaiveDate, _>("stat_date").unwrap(),
            views: r.try_get::<i64, _>("views").unwrap_or(0),
            completes: r.try_get::<i64, _>("completes").unwrap_or(0),
        })
        .collect::<Vec<_>>();

    Ok(items)
}
