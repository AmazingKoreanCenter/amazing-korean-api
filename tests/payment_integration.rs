//! Phase 3 통합 테스트 — `PaymentService` (B1 트랙).
//!
//! ## 범위
//!
//! Paddle SDK mock 미도입 = 외부 호출 path 제외. testable subset:
//! - DB-only query: `get_subscription` / `has_active_subscription` (non-existent user)
//! - Provider 부재 path: `get_plans` / `cancel_subscription` 가 `payment=None` 시 ServiceUnavailable
//! - `cancel_subscription` user 에 sub record 없음 → BadRequest
//!
//! Paddle webhook signature 검증 path = 별도 Paddle SDK mock 트랙 (PaymentProvider trait).

mod common;

use amazing_korean_api::api::payment::service::PaymentService;
use amazing_korean_api::error::AppError;

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_get_plans_returns_service_unavailable_when_payment_provider_missing() {
    // st.payment = None (default) → ServiceUnavailable.
    let st = common::make_test_state().await;
    assert!(st.payment.is_none(), "기본 state 의 payment = None");

    let result = PaymentService::get_plans(&st).await;
    match result {
        Err(AppError::ServiceUnavailable(msg)) => {
            assert!(
                msg.contains("Payment"),
                "에러 메시지에 'Payment' 포함, got: {}",
                msg
            );
        }
        Err(e) => panic!(
            "payment None → ServiceUnavailable expected, got Err: {:?}",
            e
        ),
        Ok(_) => panic!("payment None → Err expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_get_subscription_returns_none_for_user_without_subscription() {
    // 존재하지 않는 user_id → Ok(SubscriptionRes { subscription: None }).
    let st = common::make_test_state().await;

    let result = PaymentService::get_subscription(&st, 999_999_999).await;
    let res = match result {
        Ok(r) => r,
        Err(e) => panic!("non-existent user → Ok(None) expected, got Err: {:?}", e),
    };
    assert!(
        res.subscription.is_none(),
        "subscription=None for non-existent user"
    );
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_has_active_subscription_returns_false_for_user_without_subscription() {
    // 존재하지 않는 user_id → Ok(false).
    let st = common::make_test_state().await;

    let result = PaymentService::has_active_subscription(&st, 999_999_998).await;
    let active = match result {
        Ok(b) => b,
        Err(e) => panic!("non-existent user → Ok(false) expected, got Err: {:?}", e),
    };
    assert!(!active, "active=false for non-existent user");
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_cancel_subscription_returns_service_unavailable_when_payment_provider_missing() {
    // st.payment = None → ServiceUnavailable (sub record 조회 전 차단).
    let st = common::make_test_state().await;

    let result = PaymentService::cancel_subscription(&st, 999_999_997, false).await;
    match result {
        Err(AppError::ServiceUnavailable(msg)) => {
            assert!(
                msg.contains("Payment"),
                "에러 메시지에 'Payment' 포함, got: {}",
                msg
            );
        }
        Err(e) => panic!(
            "payment None → ServiceUnavailable expected, got Err: {:?}",
            e
        ),
        Ok(_) => panic!("payment None → Err expected, got Ok"),
    }
}
