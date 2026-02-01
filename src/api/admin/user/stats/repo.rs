use super::dto::{DailyLoginItem, DailySignupItem, DeviceStatsItem, UsersByRole};
use crate::error::AppResult;
use sqlx::{PgPool, Row};

// ==========================================
// User Stats Queries
// ==========================================

/// 사용자 요약 통계 조회
pub async fn fetch_user_stats_summary(
    db: &PgPool,
    from: chrono::NaiveDate,
    to: chrono::NaiveDate,
) -> AppResult<(i64, i64, i64, i64, UsersByRole)> {
    // 전체 사용자 수
    let total_row = sqlx::query("SELECT COUNT(*) AS cnt FROM users")
        .fetch_one(db)
        .await?;
    let total_users: i64 = total_row.try_get("cnt").unwrap_or(0);

    // 활성/비활성 사용자 수 (user_state는 boolean)
    let state_row = sqlx::query(
        r#"
        SELECT
            COUNT(*) FILTER (WHERE user_state = true) AS active_cnt,
            COUNT(*) FILTER (WHERE user_state = false) AS inactive_cnt
        FROM users
        "#,
    )
    .fetch_one(db)
    .await?;

    let active_users: i64 = state_row.try_get("active_cnt").unwrap_or(0);
    let inactive_users: i64 = state_row.try_get("inactive_cnt").unwrap_or(0);

    // 기간 내 신규 가입자 수 및 역할별 집계
    let signup_rows = sqlx::query(
        r#"
        SELECT
            user_auth::text AS role,
            COUNT(*)::bigint AS cnt
        FROM users
        WHERE user_created_at >= $1::date
          AND user_created_at < ($2::date + interval '1 day')
        GROUP BY user_auth
        "#,
    )
    .bind(from)
    .bind(to)
    .fetch_all(db)
    .await?;

    let mut new_users: i64 = 0;
    let mut by_role = UsersByRole::default();

    for row in signup_rows {
        let role: String = row.try_get("role").unwrap_or_default();
        let cnt: i64 = row.try_get("cnt").unwrap_or(0);
        new_users += cnt;
        match role.as_str() {
            "HYMN" => by_role.hymn = cnt,
            "admin" => by_role.admin = cnt,
            "manager" => by_role.manager = cnt,
            "learner" => by_role.learner = cnt,
            _ => {}
        }
    }

    Ok((
        total_users,
        new_users,
        active_users,
        inactive_users,
        by_role,
    ))
}

/// 일별 가입 통계 조회 (zero-fill 포함)
pub async fn fetch_daily_signups(
    db: &PgPool,
    from: chrono::NaiveDate,
    to: chrono::NaiveDate,
) -> AppResult<Vec<DailySignupItem>> {
    let rows = sqlx::query(
        r#"
        WITH days AS (
            SELECT generate_series($1::date, $2::date, interval '1 day')::date AS signup_date
        ),
        agg AS (
            SELECT
                DATE(user_created_at) AS signup_date,
                COUNT(*)::bigint AS signups,
                COALESCE(SUM(CASE WHEN user_auth = 'HYMN' THEN 1 ELSE 0 END), 0)::bigint AS role_hymn,
                COALESCE(SUM(CASE WHEN user_auth = 'admin' THEN 1 ELSE 0 END), 0)::bigint AS role_admin,
                COALESCE(SUM(CASE WHEN user_auth = 'manager' THEN 1 ELSE 0 END), 0)::bigint AS role_manager,
                COALESCE(SUM(CASE WHEN user_auth = 'learner' THEN 1 ELSE 0 END), 0)::bigint AS role_learner
            FROM users
            WHERE user_created_at >= $1::date
              AND user_created_at < ($2::date + interval '1 day')
            GROUP BY DATE(user_created_at)
        )
        SELECT
            d.signup_date,
            COALESCE(a.signups, 0)::bigint AS signups,
            COALESCE(a.role_hymn, 0)::bigint AS role_hymn,
            COALESCE(a.role_admin, 0)::bigint AS role_admin,
            COALESCE(a.role_manager, 0)::bigint AS role_manager,
            COALESCE(a.role_learner, 0)::bigint AS role_learner
        FROM days d
        LEFT JOIN agg a ON a.signup_date = d.signup_date
        ORDER BY d.signup_date DESC
        "#,
    )
    .bind(from)
    .bind(to)
    .fetch_all(db)
    .await?;

    let items = rows
        .into_iter()
        .map(|r| DailySignupItem {
            date: r.try_get("signup_date").unwrap(),
            signups: r.try_get("signups").unwrap_or(0),
            by_role: UsersByRole {
                hymn: r.try_get("role_hymn").unwrap_or(0),
                admin: r.try_get("role_admin").unwrap_or(0),
                manager: r.try_get("role_manager").unwrap_or(0),
                learner: r.try_get("role_learner").unwrap_or(0),
            },
        })
        .collect();

    Ok(items)
}

// ==========================================
// Login Stats Queries
// ==========================================

/// 로그인 요약 통계 조회
pub async fn fetch_login_stats_summary(
    db: &PgPool,
    from: chrono::NaiveDate,
    to: chrono::NaiveDate,
) -> AppResult<(i64, i64, i64, i64, i64)> {
    // 로그인 성공/실패 집계
    // login_event_enum: 'login', 'logout', 'refresh', 'rotate', 'fail', 'reuse_detected'
    // login_success_log: boolean (성공 여부)
    let login_row = sqlx::query(
        r#"
        SELECT
            COUNT(*) FILTER (WHERE login_event_log = 'login' AND login_success_log = true) AS success_count,
            COUNT(*) FILTER (WHERE login_event_log = 'fail' OR (login_event_log = 'login' AND login_success_log = false)) AS fail_count,
            COUNT(DISTINCT user_id) FILTER (WHERE login_event_log = 'login' AND login_success_log = true) AS unique_users
        FROM login_log
        WHERE login_begin_at_log >= $1::date
          AND login_begin_at_log < ($2::date + interval '1 day')
        "#,
    )
    .bind(from)
    .bind(to)
    .fetch_one(db)
    .await?;

    let success_count: i64 = login_row.try_get("success_count").unwrap_or(0);
    let fail_count: i64 = login_row.try_get("fail_count").unwrap_or(0);
    let unique_users: i64 = login_row.try_get("unique_users").unwrap_or(0);
    let total_logins = success_count + fail_count;

    // 현재 활성 세션 수 (login 테이블에서 만료되지 않은 세션)
    let session_row = sqlx::query(
        r#"
        SELECT COUNT(*) AS cnt
        FROM login
        WHERE login_expired_at > NOW()
        "#,
    )
    .fetch_one(db)
    .await?;

    let active_sessions: i64 = session_row.try_get("cnt").unwrap_or(0);

    Ok((
        total_logins,
        success_count,
        fail_count,
        unique_users,
        active_sessions,
    ))
}

/// 일별 로그인 통계 조회 (zero-fill 포함)
pub async fn fetch_daily_logins(
    db: &PgPool,
    from: chrono::NaiveDate,
    to: chrono::NaiveDate,
) -> AppResult<Vec<DailyLoginItem>> {
    let rows = sqlx::query(
        r#"
        WITH days AS (
            SELECT generate_series($1::date, $2::date, interval '1 day')::date AS login_date
        ),
        agg AS (
            SELECT
                DATE(login_begin_at_log) AS login_date,
                (COUNT(*) FILTER (WHERE login_event_log = 'login' AND login_success_log = true))::bigint AS success,
                (COUNT(*) FILTER (WHERE login_event_log = 'fail' OR (login_event_log = 'login' AND login_success_log = false)))::bigint AS fail,
                (COUNT(DISTINCT user_id) FILTER (WHERE login_event_log = 'login' AND login_success_log = true))::bigint AS unique_users
            FROM login_log
            WHERE login_begin_at_log >= $1::date
              AND login_begin_at_log < ($2::date + interval '1 day')
            GROUP BY DATE(login_begin_at_log)
        )
        SELECT
            d.login_date,
            COALESCE(a.success, 0)::bigint AS success,
            COALESCE(a.fail, 0)::bigint AS fail,
            COALESCE(a.unique_users, 0)::bigint AS unique_users
        FROM days d
        LEFT JOIN agg a ON a.login_date = d.login_date
        ORDER BY d.login_date DESC
        "#,
    )
    .bind(from)
    .bind(to)
    .fetch_all(db)
    .await?;

    let items = rows
        .into_iter()
        .map(|r| DailyLoginItem {
            date: r.try_get("login_date").unwrap(),
            success: r.try_get("success").unwrap_or(0),
            fail: r.try_get("fail").unwrap_or(0),
            unique_users: r.try_get("unique_users").unwrap_or(0),
        })
        .collect();

    Ok(items)
}

/// 디바이스별 통계 조회
pub async fn fetch_device_stats(
    db: &PgPool,
    from: chrono::NaiveDate,
    to: chrono::NaiveDate,
) -> AppResult<Vec<DeviceStatsItem>> {
    let rows = sqlx::query(
        r#"
        WITH device_counts AS (
            SELECT
                login_device_log::text AS device,
                COUNT(*) AS count
            FROM login_log
            WHERE login_event_log = 'login' AND login_success_log = true
              AND login_begin_at_log >= $1::date
              AND login_begin_at_log < ($2::date + interval '1 day')
            GROUP BY login_device_log
        ),
        total AS (
            SELECT COALESCE(SUM(count), 0) AS total_count FROM device_counts
        )
        SELECT
            dc.device,
            dc.count,
            CASE
                WHEN t.total_count > 0 THEN (dc.count * 100.0 / t.total_count)::float8
                ELSE 0.0::float8
            END AS percentage
        FROM device_counts dc
        CROSS JOIN total t
        ORDER BY dc.count DESC
        "#,
    )
    .bind(from)
    .bind(to)
    .fetch_all(db)
    .await?;

    let items = rows
        .into_iter()
        .map(|r| DeviceStatsItem {
            device: r.try_get("device").unwrap_or_default(),
            count: r.try_get("count").unwrap_or(0),
            percentage: r.try_get::<f64, _>("percentage").unwrap_or(0.0),
        })
        .collect();

    Ok(items)
}
