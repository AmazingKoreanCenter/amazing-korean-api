use sqlx::PgPool;

use crate::error::AppResult;

use super::dto::{AdminGrantSummary, AdminSubDetail, AdminSubSummary, AdminSubUser, AdminTxnSummary};

pub struct AdminPaymentRepo;

impl AdminPaymentRepo {
    // =========================================================================
    // 구독 목록
    // =========================================================================

    /// 구독 목록 (검색/필터/정렬/페이지네이션)
    pub async fn list_subscriptions(
        pool: &PgPool,
        q_email_idx: Option<&str>,
        q_nickname: Option<&str>,
        status_filter: Option<&str>,
        page: i64,
        size: i64,
        sort: &str,
        order: &str,
    ) -> AppResult<(i64, Vec<AdminSubSummary>)> {
        let mut count_sql = String::from(
            "SELECT COUNT(*) FROM subscriptions s JOIN users u ON s.user_id = u.user_id WHERE 1=1",
        );
        let mut select_sql = String::from(
            r#"
            SELECT s.subscription_id, s.user_id, u.user_email as user_email,
                   s.status, s.billing_interval, s.current_price_cents,
                   s.current_period_end, s.created_at
            FROM subscriptions s
            JOIN users u ON s.user_id = u.user_id
            WHERE 1=1
            "#,
        );

        let mut query_args: Vec<String> = Vec::new();
        let mut bind_idx = 1;

        // 이메일 blind index 검색
        if let Some(idx) = q_email_idx {
            let clause = format!(" AND u.user_email_idx = ${}", bind_idx);
            count_sql.push_str(&clause);
            select_sql.push_str(&clause);
            query_args.push(idx.to_string());
            bind_idx += 1;
        } else if let Some(nick) = q_nickname {
            let clause = format!(" AND LOWER(u.user_nickname) LIKE ${}", bind_idx);
            count_sql.push_str(&clause);
            select_sql.push_str(&clause);
            query_args.push(format!("%{}%", nick.to_lowercase()));
            bind_idx += 1;
        }

        // 상태 필터
        if let Some(status) = status_filter {
            let clause = format!(" AND s.status = ${}::subscription_status_enum", bind_idx);
            count_sql.push_str(&clause);
            select_sql.push_str(&clause);
            query_args.push(status.to_string());
            bind_idx += 1;
        }

        // 카운트 쿼리
        let mut total_query = sqlx::query_scalar::<_, i64>(&count_sql);
        for arg in &query_args {
            total_query = total_query.bind(arg);
        }
        let total = total_query.fetch_one(pool).await?;

        // 정렬
        let sort_column = match sort {
            "id" => "s.subscription_id",
            "status" => "s.status",
            "billing_interval" => "s.billing_interval",
            "price" => "s.current_price_cents",
            _ => "s.created_at",
        };
        let order_dir = if order == "asc" { "ASC" } else { "DESC" };

        select_sql.push_str(&format!(
            " ORDER BY {} {} LIMIT ${} OFFSET ${}",
            sort_column,
            order_dir,
            bind_idx,
            bind_idx + 1
        ));

        let mut select_query = sqlx::query_as::<_, AdminSubSummary>(&select_sql);
        for arg in &query_args {
            select_query = select_query.bind(arg);
        }
        select_query = select_query.bind(size).bind((page - 1) * size);

        let items = select_query.fetch_all(pool).await?;

        Ok((total, items))
    }

    // =========================================================================
    // 구독 상세
    // =========================================================================

    /// 구독 상세 조회
    pub async fn get_subscription(
        pool: &PgPool,
        subscription_id: i64,
    ) -> AppResult<Option<AdminSubDetail>> {
        let row = sqlx::query_as::<_, AdminSubDetail>(
            r#"
            SELECT subscription_id, user_id, provider_subscription_id,
                   provider_customer_id, status, billing_interval,
                   current_price_cents, trial_started_at, trial_ends_at,
                   current_period_start, current_period_end,
                   canceled_at, paused_at, created_at, updated_at
            FROM subscriptions
            WHERE subscription_id = $1
            "#,
        )
        .bind(subscription_id)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// 구독에 연결된 사용자 정보 조회
    pub async fn get_subscription_user(
        pool: &PgPool,
        user_id: i64,
    ) -> AppResult<Option<AdminSubUser>> {
        let row = sqlx::query_as::<_, AdminSubUser>(
            r#"
            SELECT user_id, user_email as email, user_nickname as nickname, user_auth
            FROM users
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    // =========================================================================
    // 트랜잭션
    // =========================================================================

    /// 특정 구독의 트랜잭션 내역
    pub async fn list_transactions_for_subscription(
        pool: &PgPool,
        subscription_id: i64,
    ) -> AppResult<Vec<AdminTxnSummary>> {
        let items = sqlx::query_as::<_, AdminTxnSummary>(
            r#"
            SELECT t.transaction_id, t.subscription_id, t.user_id,
                   u.user_email as user_email,
                   t.status, t.amount_cents, t.tax_cents, t.currency,
                   t.billing_interval, t.occurred_at
            FROM transactions t
            JOIN users u ON t.user_id = u.user_id
            WHERE t.subscription_id = $1
            ORDER BY t.occurred_at DESC
            "#,
        )
        .bind(subscription_id)
        .fetch_all(pool)
        .await?;

        Ok(items)
    }

    /// 전체 트랜잭션 목록 (검색/필터/정렬/페이지네이션)
    pub async fn list_transactions(
        pool: &PgPool,
        q_email_idx: Option<&str>,
        status_filter: Option<&str>,
        page: i64,
        size: i64,
        sort: &str,
        order: &str,
    ) -> AppResult<(i64, Vec<AdminTxnSummary>)> {
        let mut count_sql = String::from(
            "SELECT COUNT(*) FROM transactions t JOIN users u ON t.user_id = u.user_id WHERE 1=1",
        );
        let mut select_sql = String::from(
            r#"
            SELECT t.transaction_id, t.subscription_id, t.user_id,
                   u.user_email as user_email,
                   t.status, t.amount_cents, t.tax_cents, t.currency,
                   t.billing_interval, t.occurred_at
            FROM transactions t
            JOIN users u ON t.user_id = u.user_id
            WHERE 1=1
            "#,
        );

        let mut query_args: Vec<String> = Vec::new();
        let mut bind_idx = 1;

        if let Some(idx) = q_email_idx {
            let clause = format!(" AND u.user_email_idx = ${}", bind_idx);
            count_sql.push_str(&clause);
            select_sql.push_str(&clause);
            query_args.push(idx.to_string());
            bind_idx += 1;
        }

        if let Some(status) = status_filter {
            let clause = format!(
                " AND t.status = ${}::transaction_status_enum",
                bind_idx
            );
            count_sql.push_str(&clause);
            select_sql.push_str(&clause);
            query_args.push(status.to_string());
            bind_idx += 1;
        }

        let mut total_query = sqlx::query_scalar::<_, i64>(&count_sql);
        for arg in &query_args {
            total_query = total_query.bind(arg);
        }
        let total = total_query.fetch_one(pool).await?;

        let sort_column = match sort {
            "id" => "t.transaction_id",
            "amount" => "t.amount_cents",
            "status" => "t.status",
            _ => "t.occurred_at",
        };
        let order_dir = if order == "asc" { "ASC" } else { "DESC" };

        select_sql.push_str(&format!(
            " ORDER BY {} {} LIMIT ${} OFFSET ${}",
            sort_column,
            order_dir,
            bind_idx,
            bind_idx + 1
        ));

        let mut select_query = sqlx::query_as::<_, AdminTxnSummary>(&select_sql);
        for arg in &query_args {
            select_query = select_query.bind(arg);
        }
        select_query = select_query.bind(size).bind((page - 1) * size);

        let items = select_query.fetch_all(pool).await?;

        Ok((total, items))
    }

    // =========================================================================
    // 수동 수강권
    // =========================================================================

    /// 수동 부여 수강권 목록 (활성 코스가 있지만 활성 구독이 없는 사용자)
    pub async fn list_manual_grants(
        pool: &PgPool,
        page: i64,
        size: i64,
    ) -> AppResult<(i64, Vec<AdminGrantSummary>)> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(DISTINCT uc.user_id)
            FROM user_course uc
            LEFT JOIN subscriptions s ON uc.user_id = s.user_id
                AND s.status IN ('trialing', 'active', 'past_due')
            WHERE uc.user_course_active = true
            AND s.subscription_id IS NULL
            "#,
        )
        .fetch_one(pool)
        .await?;

        let items = sqlx::query_as::<_, AdminGrantSummary>(
            r#"
            SELECT uc.user_id, u.user_email as user_email,
                   MIN(uc.user_course_expire_at) as expire_at,
                   COUNT(uc.course_id) as course_count
            FROM user_course uc
            JOIN users u ON uc.user_id = u.user_id
            LEFT JOIN subscriptions s ON uc.user_id = s.user_id
                AND s.status IN ('trialing', 'active', 'past_due')
            WHERE uc.user_course_active = true
            AND s.subscription_id IS NULL
            GROUP BY uc.user_id, u.user_email
            ORDER BY MAX(uc.user_course_updated_at) DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(size)
        .bind((page - 1) * size)
        .fetch_all(pool)
        .await?;

        Ok((count, items))
    }
}
