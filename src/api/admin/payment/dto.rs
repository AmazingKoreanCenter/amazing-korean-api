use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::types::{BillingInterval, SubscriptionStatus, TransactionStatus, UserAuth};

// =============================================================================
// 공통 페이지네이션 메타
// =============================================================================

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminPaymentMeta {
    pub total_count: i64,
    pub total_pages: i64,
    pub current_page: i64,
    pub per_page: i64,
}

// =============================================================================
// 구독 목록
// =============================================================================

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct AdminSubListReq {
    pub page: Option<i64>,
    pub size: Option<i64>,
    /// 이메일(@포함) 또는 닉네임 검색
    pub q: Option<String>,
    /// 상태 필터 (trialing, active, past_due, paused, canceled)
    pub status: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
pub struct AdminSubSummary {
    pub subscription_id: i64,
    pub user_id: i64,
    pub user_email: String,
    pub status: SubscriptionStatus,
    pub billing_interval: BillingInterval,
    pub current_price_cents: i32,
    #[schema(value_type = Option<String>, format = "date-time")]
    pub current_period_end: Option<DateTime<Utc>>,
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminSubListRes {
    pub items: Vec<AdminSubSummary>,
    pub meta: AdminPaymentMeta,
}

// =============================================================================
// 구독 상세
// =============================================================================

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminSubDetailRes {
    pub subscription: AdminSubDetail,
    pub user: AdminSubUser,
    pub transactions: Vec<AdminTxnSummary>,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
pub struct AdminSubDetail {
    pub subscription_id: i64,
    pub user_id: i64,
    pub provider_subscription_id: String,
    pub provider_customer_id: Option<String>,
    pub status: SubscriptionStatus,
    pub billing_interval: BillingInterval,
    pub current_price_cents: i32,
    #[schema(value_type = Option<String>, format = "date-time")]
    pub trial_started_at: Option<DateTime<Utc>>,
    #[schema(value_type = Option<String>, format = "date-time")]
    pub trial_ends_at: Option<DateTime<Utc>>,
    #[schema(value_type = Option<String>, format = "date-time")]
    pub current_period_start: Option<DateTime<Utc>>,
    #[schema(value_type = Option<String>, format = "date-time")]
    pub current_period_end: Option<DateTime<Utc>>,
    #[schema(value_type = Option<String>, format = "date-time")]
    pub canceled_at: Option<DateTime<Utc>>,
    #[schema(value_type = Option<String>, format = "date-time")]
    pub paused_at: Option<DateTime<Utc>>,
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = "date-time")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
pub struct AdminSubUser {
    pub user_id: i64,
    pub email: String,
    pub nickname: Option<String>,
    pub user_auth: UserAuth,
}

// =============================================================================
// 트랜잭션 목록
// =============================================================================

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct AdminTxnListReq {
    pub page: Option<i64>,
    pub size: Option<i64>,
    /// 이메일(@포함) 검색
    pub q: Option<String>,
    /// 상태 필터 (completed, refunded, partially_refunded)
    pub status: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
pub struct AdminTxnSummary {
    pub transaction_id: i64,
    pub subscription_id: Option<i64>,
    pub user_id: i64,
    pub user_email: String,
    pub status: TransactionStatus,
    pub amount_cents: i32,
    pub tax_cents: i32,
    pub currency: String,
    pub billing_interval: Option<BillingInterval>,
    #[schema(value_type = String, format = "date-time")]
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminTxnListRes {
    pub items: Vec<AdminTxnSummary>,
    pub meta: AdminPaymentMeta,
}

// =============================================================================
// 수동 수강권 부여
// =============================================================================

#[derive(Debug, Deserialize, ToSchema)]
pub struct AdminGrantReq {
    pub user_id: i64,
    #[schema(value_type = Option<String>, format = "date-time")]
    pub expire_at: Option<DateTime<Utc>>,
    pub reason: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminGrantRes {
    pub user_id: i64,
    pub courses_granted: u64,
    pub expire_at: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct AdminGrantListReq {
    pub page: Option<i64>,
    pub size: Option<i64>,
}

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
pub struct AdminGrantSummary {
    pub user_id: i64,
    pub user_email: String,
    #[schema(value_type = Option<String>, format = "date-time")]
    pub expire_at: Option<DateTime<Utc>>,
    pub course_count: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminGrantListRes {
    pub items: Vec<AdminGrantSummary>,
    pub meta: AdminPaymentMeta,
}

// =============================================================================
// 관리자 구독 취소 요청
// =============================================================================

#[derive(Debug, Deserialize, ToSchema)]
pub struct AdminCancelSubReq {
    /// 즉시 취소 여부 (false면 다음 결제일에 취소)
    #[serde(default)]
    pub immediately: bool,
}
