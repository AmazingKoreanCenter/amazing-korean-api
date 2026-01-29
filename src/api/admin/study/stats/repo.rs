use super::dto::{DailyStatItem, ProgramStats, StateStats, TopStudyItem};
use crate::error::AppResult;
use sqlx::{PgPool, Row};

// ==========================================
// Summary Statistics
// ==========================================

/// Fetch study counts by state
pub async fn fetch_state_counts(db: &PgPool) -> AppResult<StateStats> {
    let row = sqlx::query(
        r#"
        SELECT
            COALESCE(SUM(CASE WHEN study_state = 'ready' THEN 1 ELSE 0 END), 0) AS ready,
            COALESCE(SUM(CASE WHEN study_state = 'open' THEN 1 ELSE 0 END), 0) AS open,
            COALESCE(SUM(CASE WHEN study_state = 'close' THEN 1 ELSE 0 END), 0) AS close
        FROM study
        "#,
    )
    .fetch_one(db)
    .await?;

    Ok(StateStats {
        ready: row.try_get::<i64, _>("ready").unwrap_or(0),
        open: row.try_get::<i64, _>("open").unwrap_or(0),
        close: row.try_get::<i64, _>("close").unwrap_or(0),
    })
}

/// Fetch study counts by program
pub async fn fetch_program_counts(db: &PgPool) -> AppResult<ProgramStats> {
    let row = sqlx::query(
        r#"
        SELECT
            COALESCE(SUM(CASE WHEN study_program = 'basic_pronunciation' THEN 1 ELSE 0 END), 0) AS basic_pronunciation,
            COALESCE(SUM(CASE WHEN study_program = 'basic_word' THEN 1 ELSE 0 END), 0) AS basic_word,
            COALESCE(SUM(CASE WHEN study_program = 'basic_900' THEN 1 ELSE 0 END), 0) AS basic_900,
            COALESCE(SUM(CASE WHEN study_program = 'topik_read' THEN 1 ELSE 0 END), 0) AS topik_read,
            COALESCE(SUM(CASE WHEN study_program = 'topik_listen' THEN 1 ELSE 0 END), 0) AS topik_listen,
            COALESCE(SUM(CASE WHEN study_program = 'topik_write' THEN 1 ELSE 0 END), 0) AS topik_write,
            COALESCE(SUM(CASE WHEN study_program = 'tbc' THEN 1 ELSE 0 END), 0) AS tbc
        FROM study
        "#,
    )
    .fetch_one(db)
    .await?;

    Ok(ProgramStats {
        basic_pronunciation: row.try_get::<i64, _>("basic_pronunciation").unwrap_or(0),
        basic_word: row.try_get::<i64, _>("basic_word").unwrap_or(0),
        basic_900: row.try_get::<i64, _>("basic_900").unwrap_or(0),
        topik_read: row.try_get::<i64, _>("topik_read").unwrap_or(0),
        topik_listen: row.try_get::<i64, _>("topik_listen").unwrap_or(0),
        topik_write: row.try_get::<i64, _>("topik_write").unwrap_or(0),
        tbc: row.try_get::<i64, _>("tbc").unwrap_or(0),
    })
}

/// Fetch total task count
pub async fn fetch_total_tasks(db: &PgPool) -> AppResult<i64> {
    let row = sqlx::query(
        r#"
        SELECT COUNT(*) AS total
        FROM study_task
        "#,
    )
    .fetch_one(db)
    .await?;

    Ok(row.try_get::<i64, _>("total").unwrap_or(0))
}

/// Fetch attempt and solve statistics for a date range
pub async fn fetch_attempt_stats(
    db: &PgPool,
    from: chrono::NaiveDate,
    to: chrono::NaiveDate,
) -> AppResult<(i64, i64)> {
    let row = sqlx::query(
        r#"
        SELECT
            COALESCE(SUM(study_task_status_try_count), 0) AS total_attempts,
            COALESCE(SUM(CASE WHEN study_task_status_is_solved THEN 1 ELSE 0 END), 0) AS total_solves
        FROM study_task_status
        WHERE study_task_status_last_attempt_at >= $1::date
          AND study_task_status_last_attempt_at < ($2::date + interval '1 day')
        "#,
    )
    .bind(from)
    .bind(to)
    .fetch_one(db)
    .await?;

    Ok((
        row.try_get::<i64, _>("total_attempts").unwrap_or(0),
        row.try_get::<i64, _>("total_solves").unwrap_or(0),
    ))
}

// ==========================================
// Top Studies
// ==========================================

/// Fetch top studies by attempts, solves, or solve_rate
pub async fn fetch_top_studies(
    db: &PgPool,
    from: chrono::NaiveDate,
    to: chrono::NaiveDate,
    limit: i32,
    sort_by: &str,
) -> AppResult<Vec<TopStudyItem>> {
    let order_clause = match sort_by {
        "solves" => "solve_count DESC, attempt_count DESC",
        "solve_rate" => "solve_rate DESC, solve_count DESC",
        _ => "attempt_count DESC, solve_count DESC", // default: attempts
    };

    let query = format!(
        r#"
        WITH study_stats AS (
            SELECT
                st.study_id,
                COUNT(DISTINCT st.study_task_id) AS task_count,
                COALESCE(SUM(sts.study_task_status_try_count), 0) AS attempt_count,
                COALESCE(SUM(CASE WHEN sts.study_task_status_is_solved THEN 1 ELSE 0 END), 0) AS solve_count
            FROM study_task st
            LEFT JOIN study_task_status sts
                ON sts.study_task_id = st.study_task_id
                AND sts.study_task_status_last_attempt_at >= $1::date
                AND sts.study_task_status_last_attempt_at < ($2::date + interval '1 day')
            GROUP BY st.study_id
        )
        SELECT
            s.study_id,
            s.study_idx,
            s.study_title,
            s.study_program::text AS study_program,
            COALESCE(ss.task_count, 0) AS task_count,
            COALESCE(ss.attempt_count, 0) AS attempt_count,
            COALESCE(ss.solve_count, 0) AS solve_count,
            CASE
                WHEN COALESCE(ss.task_count, 0) > 0
                THEN (COALESCE(ss.solve_count, 0)::float / COALESCE(ss.task_count, 0)::float) * 100
                ELSE 0
            END AS solve_rate
        FROM study s
        LEFT JOIN study_stats ss ON ss.study_id = s.study_id
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
        .map(|(i, r)| TopStudyItem {
            rank: (i + 1) as i32,
            study_id: r.try_get::<i32, _>("study_id").unwrap_or(0) as i64,
            study_idx: r.try_get::<String, _>("study_idx").unwrap_or_default(),
            study_title: r.try_get::<Option<String>, _>("study_title").unwrap_or(None),
            study_program: r.try_get::<String, _>("study_program").unwrap_or_default(),
            task_count: r.try_get::<i64, _>("task_count").unwrap_or(0),
            attempt_count: r.try_get::<i64, _>("attempt_count").unwrap_or(0),
            solve_count: r.try_get::<i64, _>("solve_count").unwrap_or(0),
            solve_rate: r.try_get::<f64, _>("solve_rate").unwrap_or(0.0),
        })
        .collect::<Vec<_>>();

    Ok(items)
}

// ==========================================
// Daily Statistics
// ==========================================

/// Fetch daily statistics (attempts, solves, active users)
pub async fn fetch_daily_stats(
    db: &PgPool,
    from: chrono::NaiveDate,
    to: chrono::NaiveDate,
) -> AppResult<Vec<DailyStatItem>> {
    let rows = sqlx::query(
        r#"
        WITH days AS (
            SELECT generate_series($1::date, $2::date, interval '1 day')::date AS stat_date
        ),
        daily_agg AS (
            SELECT
                DATE(study_task_status_last_attempt_at) AS stat_date,
                SUM(study_task_status_try_count) AS attempts,
                SUM(CASE WHEN study_task_status_is_solved THEN 1 ELSE 0 END) AS solves,
                COUNT(DISTINCT user_id) AS active_users
            FROM study_task_status
            WHERE study_task_status_last_attempt_at >= $1::date
              AND study_task_status_last_attempt_at < ($2::date + interval '1 day')
            GROUP BY DATE(study_task_status_last_attempt_at)
        )
        SELECT
            d.stat_date,
            COALESCE(da.attempts, 0) AS attempts,
            COALESCE(da.solves, 0) AS solves,
            COALESCE(da.active_users, 0) AS active_users
        FROM days d
        LEFT JOIN daily_agg da ON da.stat_date = d.stat_date
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
            attempts: r.try_get::<i64, _>("attempts").unwrap_or(0),
            solves: r.try_get::<i64, _>("solves").unwrap_or(0),
            active_users: r.try_get::<i64, _>("active_users").unwrap_or(0),
        })
        .collect::<Vec<_>>();

    Ok(items)
}
