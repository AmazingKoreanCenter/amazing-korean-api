use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::types::{BillingInterval, SubscriptionStatus};

// =============================================================================
// GET /payment/plans — 구독 플랜 목록 응답
// =============================================================================

/// 개별 플랜 정보
#[derive(Debug, Serialize, ToSchema)]
pub struct PlanInfo {
    /// 구독 주기 (month_1, month_3, month_6, month_12)
    pub interval: BillingInterval,
    /// 개월 수
    pub months: i32,
    /// 가격 (센트 단위)
    pub price_cents: i32,
    /// 가격 (달러 표시용)
    pub price_display: String,
    /// Paddle Price ID (프론트엔드에서 checkout 시 사용)
    pub price_id: String,
    /// 무료 체험 일수
    pub trial_days: i32,
    /// 레이블 (Monthly, Quarterly, Semi-Annual, Annual)
    pub label: String,
}

/// GET /payment/plans 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct PlansRes {
    /// Paddle Client-side Token (프론트엔드에서 Paddle.js 초기화용)
    pub client_token: String,
    /// Sandbox 모드 여부
    pub sandbox: bool,
    /// 사용 가능한 플랜 목록
    pub plans: Vec<PlanInfo>,
}

// =============================================================================
// GET /payment/subscription — 현재 사용자 구독 상태 응답
// =============================================================================

/// 구독 상세 정보
#[derive(Debug, Serialize, ToSchema)]
pub struct SubscriptionInfo {
    pub subscription_id: i64,
    pub status: SubscriptionStatus,
    pub billing_interval: BillingInterval,
    pub current_price_cents: i32,
    pub trial_ends_at: Option<String>,
    pub current_period_start: Option<String>,
    pub current_period_end: Option<String>,
    pub canceled_at: Option<String>,
    pub paused_at: Option<String>,
    pub created_at: String,
}

/// GET /payment/subscription 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct SubscriptionRes {
    /// 현재 구독 정보 (구독이 없으면 null)
    pub subscription: Option<SubscriptionInfo>,
}

// =============================================================================
// POST /payment/subscription/cancel — 구독 취소 요청
// =============================================================================

/// 구독 취소 요청 DTO
#[derive(Debug, Deserialize, ToSchema)]
pub struct CancelSubscriptionReq {
    /// 즉시 취소 여부 (false면 다음 결제일에 취소)
    #[serde(default)]
    pub immediately: bool,
}
