use sqlx::PgPool;

use crate::error::AppResult;
use crate::types::{
    BillingInterval, PaymentProvider, SubscriptionStatus, TransactionStatus,
};

// =============================================================================
// Subscription Row
// =============================================================================

/// DB에서 읽어온 구독 행
#[derive(Debug, sqlx::FromRow)]
pub struct SubscriptionRow {
    pub subscription_id: i64,
    pub user_id: i64,
    pub payment_provider: PaymentProvider,
    pub provider_subscription_id: String,
    pub provider_customer_id: Option<String>,
    pub status: SubscriptionStatus,
    pub billing_interval: BillingInterval,
    pub current_price_cents: i32,
    pub trial_started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub trial_ends_at: Option<chrono::DateTime<chrono::Utc>>,
    pub current_period_start: Option<chrono::DateTime<chrono::Utc>>,
    pub current_period_end: Option<chrono::DateTime<chrono::Utc>>,
    pub canceled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub paused_at: Option<chrono::DateTime<chrono::Utc>>,
    pub provider_data: Option<serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// =============================================================================
// Transaction Row
// =============================================================================

/// DB에서 읽어온 트랜잭션 행
#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
pub struct TransactionRow {
    pub transaction_id: i64,
    pub subscription_id: Option<i64>,
    pub user_id: i64,
    pub payment_provider: PaymentProvider,
    pub provider_transaction_id: String,
    pub status: TransactionStatus,
    pub amount_cents: i32,
    pub tax_cents: i32,
    pub currency: String,
    pub billing_interval: Option<BillingInterval>,
    pub provider_data: Option<serde_json::Value>,
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct PaymentRepo;

impl PaymentRepo {
    // =========================================================================
    // Subscription Queries
    // =========================================================================

    /// 사용자의 활성 구독 조회 (trialing, active, past_due 상태)
    pub async fn get_active_subscription(
        pool: &PgPool,
        user_id: i64,
    ) -> AppResult<Option<SubscriptionRow>> {
        let row = sqlx::query_as::<_, SubscriptionRow>(
            r#"
            SELECT subscription_id, user_id, payment_provider,
                   provider_subscription_id, provider_customer_id,
                   status, billing_interval, current_price_cents,
                   trial_started_at, trial_ends_at,
                   current_period_start, current_period_end,
                   canceled_at, paused_at, provider_data,
                   created_at, updated_at
            FROM subscriptions
            WHERE user_id = $1
              AND status IN ('trialing', 'active', 'past_due')
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// 사용자의 최신 구독 조회 (상태 무관)
    pub async fn get_latest_subscription(
        pool: &PgPool,
        user_id: i64,
    ) -> AppResult<Option<SubscriptionRow>> {
        let row = sqlx::query_as::<_, SubscriptionRow>(
            r#"
            SELECT subscription_id, user_id, payment_provider,
                   provider_subscription_id, provider_customer_id,
                   status, billing_interval, current_price_cents,
                   trial_started_at, trial_ends_at,
                   current_period_start, current_period_end,
                   canceled_at, paused_at, provider_data,
                   created_at, updated_at
            FROM subscriptions
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// provider_subscription_id로 구독 조회
    pub async fn get_subscription_by_provider_id(
        pool: &PgPool,
        provider_subscription_id: &str,
    ) -> AppResult<Option<SubscriptionRow>> {
        let row = sqlx::query_as::<_, SubscriptionRow>(
            r#"
            SELECT subscription_id, user_id, payment_provider,
                   provider_subscription_id, provider_customer_id,
                   status, billing_interval, current_price_cents,
                   trial_started_at, trial_ends_at,
                   current_period_start, current_period_end,
                   canceled_at, paused_at, provider_data,
                   created_at, updated_at
            FROM subscriptions
            WHERE provider_subscription_id = $1
            LIMIT 1
            "#,
        )
        .bind(provider_subscription_id)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// 구독 생성
    pub async fn create_subscription(
        pool: &PgPool,
        user_id: i64,
        provider: PaymentProvider,
        provider_subscription_id: &str,
        provider_customer_id: Option<&str>,
        status: SubscriptionStatus,
        billing_interval: BillingInterval,
        current_price_cents: i32,
        trial_started_at: Option<chrono::DateTime<chrono::Utc>>,
        trial_ends_at: Option<chrono::DateTime<chrono::Utc>>,
        current_period_start: Option<chrono::DateTime<chrono::Utc>>,
        current_period_end: Option<chrono::DateTime<chrono::Utc>>,
        provider_data: Option<serde_json::Value>,
    ) -> AppResult<i64> {
        let id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO subscriptions (
                user_id, payment_provider, provider_subscription_id,
                provider_customer_id, status, billing_interval,
                current_price_cents, trial_started_at, trial_ends_at,
                current_period_start, current_period_end, provider_data
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING subscription_id
            "#,
        )
        .bind(user_id)
        .bind(provider)
        .bind(provider_subscription_id)
        .bind(provider_customer_id)
        .bind(status)
        .bind(billing_interval)
        .bind(current_price_cents)
        .bind(trial_started_at)
        .bind(trial_ends_at)
        .bind(current_period_start)
        .bind(current_period_end)
        .bind(provider_data)
        .fetch_one(pool)
        .await?;

        Ok(id)
    }

    /// 구독 상태 업데이트
    pub async fn update_subscription_status(
        pool: &PgPool,
        provider_subscription_id: &str,
        status: SubscriptionStatus,
        current_period_start: Option<chrono::DateTime<chrono::Utc>>,
        current_period_end: Option<chrono::DateTime<chrono::Utc>>,
        canceled_at: Option<chrono::DateTime<chrono::Utc>>,
        paused_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE subscriptions
            SET status = $2,
                current_period_start = COALESCE($3, current_period_start),
                current_period_end = COALESCE($4, current_period_end),
                canceled_at = $5,
                paused_at = $6,
                updated_at = NOW()
            WHERE provider_subscription_id = $1
            "#,
        )
        .bind(provider_subscription_id)
        .bind(status)
        .bind(current_period_start)
        .bind(current_period_end)
        .bind(canceled_at)
        .bind(paused_at)
        .execute(pool)
        .await?;

        Ok(())
    }

    // =========================================================================
    // Transaction Queries
    // =========================================================================

    /// 트랜잭션 생성
    pub async fn create_transaction(
        pool: &PgPool,
        subscription_id: Option<i64>,
        user_id: i64,
        provider: PaymentProvider,
        provider_transaction_id: &str,
        status: TransactionStatus,
        amount_cents: i32,
        tax_cents: i32,
        currency: &str,
        billing_interval: Option<BillingInterval>,
        provider_data: Option<serde_json::Value>,
        occurred_at: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<i64> {
        let id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO transactions (
                subscription_id, user_id, payment_provider,
                provider_transaction_id, status,
                amount_cents, tax_cents, currency,
                billing_interval, provider_data, occurred_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING transaction_id
            "#,
        )
        .bind(subscription_id)
        .bind(user_id)
        .bind(provider)
        .bind(provider_transaction_id)
        .bind(status)
        .bind(amount_cents)
        .bind(tax_cents)
        .bind(currency)
        .bind(billing_interval)
        .bind(provider_data)
        .bind(occurred_at)
        .fetch_one(pool)
        .await?;

        Ok(id)
    }

    // =========================================================================
    // Webhook Event Queries (멱등성)
    // =========================================================================

    /// 웹훅 이벤트 중복 확인
    pub async fn is_webhook_event_processed(
        pool: &PgPool,
        provider: PaymentProvider,
        provider_event_id: &str,
    ) -> AppResult<bool> {
        let exists = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM webhook_events
                WHERE payment_provider = $1 AND provider_event_id = $2
            )
            "#,
        )
        .bind(provider)
        .bind(provider_event_id)
        .fetch_one(pool)
        .await?;

        Ok(exists)
    }

    // =========================================================================
    // User Course Queries (수강권 부여/해제)
    // =========================================================================

    /// 활성 코스 전체에 수강권 부여 (UPSERT)
    /// 구독 활성화 시 호출 — 모든 active 코스에 user_course 레코드 생성/갱신
    pub async fn grant_all_courses(
        pool: &PgPool,
        user_id: i64,
        expire_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> AppResult<u64> {
        let result = sqlx::query(
            r#"
            INSERT INTO user_course (user_id, course_id, user_course_active, user_course_expire_at)
            SELECT $1, course_id, true, $2
            FROM course WHERE course_state = 'active'
            ON CONFLICT (user_id, course_id) DO UPDATE SET
                user_course_active = true,
                user_course_expire_at = EXCLUDED.user_course_expire_at,
                user_course_updated_at = NOW()
            "#,
        )
        .bind(user_id)
        .bind(expire_at)
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// 모든 수강권 비활성화 (구독 일시정지/취소 시)
    pub async fn revoke_all_courses(pool: &PgPool, user_id: i64) -> AppResult<u64> {
        let result = sqlx::query(
            r#"
            UPDATE user_course
            SET user_course_active = false,
                user_course_updated_at = NOW()
            WHERE user_id = $1 AND user_course_active = true
            "#,
        )
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// 수강권 만료일 업데이트 (구독 갱신 시)
    pub async fn update_course_expiry(
        pool: &PgPool,
        user_id: i64,
        expire_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> AppResult<u64> {
        let result = sqlx::query(
            r#"
            UPDATE user_course
            SET user_course_expire_at = $2,
                user_course_updated_at = NOW()
            WHERE user_id = $1 AND user_course_active = true
            "#,
        )
        .bind(user_id)
        .bind(expire_at)
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// 웹훅 이벤트 기록 (멱등성 보장)
    pub async fn record_webhook_event(
        pool: &PgPool,
        provider: PaymentProvider,
        provider_event_id: &str,
        event_type: &str,
        payload: serde_json::Value,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO webhook_events (payment_provider, provider_event_id, event_type, payload)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (payment_provider, provider_event_id) DO NOTHING
            "#,
        )
        .bind(provider)
        .bind(provider_event_id)
        .bind(event_type)
        .bind(payload)
        .execute(pool)
        .await?;

        Ok(())
    }
}
