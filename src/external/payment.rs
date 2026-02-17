use async_trait::async_trait;
use paddle_rust_sdk::enums::EffectiveFrom;

use crate::error::{AppError, AppResult};

// =============================================================================
// PaymentProvider trait
// =============================================================================

/// 결제 서비스 추상화 trait
///
/// `PAYMENT_PROVIDER` 환경변수로 구현체 전환:
/// - `paddle`: Paddle Billing (PaddleProvider)
/// - `none`: 비활성 (개발 환경 전용)
///
/// Provider 교체 시 (예: Stripe 전환) 이 trait만 구현하면 됨.
#[async_trait]
pub trait PaymentProvider: Send + Sync {
    /// 구독 조회
    async fn get_subscription(
        &self,
        provider_subscription_id: &str,
    ) -> AppResult<ProviderSubscription>;

    /// 구독 취소
    async fn cancel_subscription(
        &self,
        provider_subscription_id: &str,
        effective_from: CancelEffectiveFrom,
    ) -> AppResult<ProviderSubscription>;

    /// Client-side token 반환 (프론트엔드 전달용)
    fn client_token(&self) -> &str;

    /// Sandbox 여부
    fn is_sandbox(&self) -> bool;
}

// =============================================================================
// Provider-agnostic Types
// =============================================================================

/// Provider-agnostic 구독 정보
#[derive(Debug)]
pub struct ProviderSubscription {
    pub provider_subscription_id: String,
    pub provider_customer_id: String,
    pub status: String,
    pub current_period_start: Option<chrono::DateTime<chrono::Utc>>,
    pub current_period_end: Option<chrono::DateTime<chrono::Utc>>,
    pub canceled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub paused_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// 구독 취소 시점
pub enum CancelEffectiveFrom {
    /// 다음 결제일에 취소 (현재 기간은 유지)
    NextBillingPeriod,
    /// 즉시 취소
    Immediately,
}

// =============================================================================
// Paddle 구현
// =============================================================================

/// Paddle Billing 결제 프로바이더
///
/// paddle-rust-sdk를 사용하여 Paddle Billing API와 통신.
/// Webhook 검증은 handler에서 Paddle::unmarshal()을 직접 호출.
pub struct PaddleProvider {
    client: paddle_rust_sdk::Paddle,
    client_token: String,
    sandbox: bool,
}

impl PaddleProvider {
    pub fn new(api_key: &str, sandbox: bool, client_token: String) -> AppResult<Self> {
        let environment = if sandbox {
            paddle_rust_sdk::Paddle::SANDBOX
        } else {
            paddle_rust_sdk::Paddle::PRODUCTION
        };

        let client = paddle_rust_sdk::Paddle::new(api_key, environment).map_err(|e| {
            AppError::Internal(format!("Failed to create Paddle client: {}", e))
        })?;

        Ok(Self {
            client,
            client_token,
            sandbox,
        })
    }

    /// 내부 Paddle SDK 클라이언트 참조 (webhook 검증 등에 활용)
    #[allow(dead_code)]
    pub fn sdk_client(&self) -> &paddle_rust_sdk::Paddle {
        &self.client
    }
}

/// Paddle Subscription → ProviderSubscription 변환 헬퍼
fn to_provider_subscription(sub: &paddle_rust_sdk::entities::Subscription) -> ProviderSubscription {
    ProviderSubscription {
        provider_subscription_id: sub.id.to_string(),
        provider_customer_id: sub.customer_id.to_string(),
        status: format!("{:?}", sub.status).to_lowercase(),
        current_period_start: sub.current_billing_period.as_ref().map(|p| p.starts_at),
        current_period_end: sub.current_billing_period.as_ref().map(|p| p.ends_at),
        canceled_at: sub.canceled_at,
        paused_at: sub.paused_at,
    }
}

#[async_trait]
impl PaymentProvider for PaddleProvider {
    async fn get_subscription(
        &self,
        provider_subscription_id: &str,
    ) -> AppResult<ProviderSubscription> {
        let resp = self
            .client
            .subscription_get(provider_subscription_id)
            .send()
            .await
            .map_err(|e| {
                tracing::error!(error = %e, sub_id = %provider_subscription_id, "Paddle get_subscription failed");
                AppError::External(format!("Paddle API error: {}", e))
            })?;

        Ok(to_provider_subscription(&resp.data.subscription))
    }

    async fn cancel_subscription(
        &self,
        provider_subscription_id: &str,
        effective_from: CancelEffectiveFrom,
    ) -> AppResult<ProviderSubscription> {
        let effective = match effective_from {
            CancelEffectiveFrom::NextBillingPeriod => EffectiveFrom::NextBillingPeriod,
            CancelEffectiveFrom::Immediately => EffectiveFrom::Immediately,
        };

        let resp = self
            .client
            .subscription_cancel(provider_subscription_id)
            .effective_from(effective)
            .send()
            .await
            .map_err(|e| {
                tracing::error!(error = %e, sub_id = %provider_subscription_id, "Paddle cancel_subscription failed");
                AppError::External(format!("Paddle API error: {}", e))
            })?;

        Ok(to_provider_subscription(&resp.data))
    }

    fn client_token(&self) -> &str {
        &self.client_token
    }

    fn is_sandbox(&self) -> bool {
        self.sandbox
    }
}
