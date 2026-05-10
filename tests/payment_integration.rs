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

use amazing_korean_api::api::payment::handler::handle_webhook;
use amazing_korean_api::api::payment::service::PaymentService;
use amazing_korean_api::error::AppError;
use axum::body::Bytes;
use axum::extract::State;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use hmac::{Hmac, Mac};
use sha2::Sha256;

/// Paddle 시그니처 형식 = `ts=<unix_timestamp>;h1=<hex_hmac_sha256(timestamp + ":" + body, secret)>`
fn paddle_signature(secret: &str, timestamp: i64, body: &str) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).expect("HMAC key");
    mac.update(format!("{}:{}", timestamp, body).as_bytes());
    let sig = mac.finalize().into_bytes();
    let hex_sig: String = sig.iter().map(|b| format!("{:02x}", b)).collect();
    format!("ts={};h1={}", timestamp, hex_sig)
}

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

// =============================================================================
// C-payment — Paddle webhook handler signature path (handler 직접 호출)
// =============================================================================

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_paddle_webhook_returns_bad_request_for_missing_signature() {
    // Paddle-Signature 헤더 없음 → BadRequest (400).
    let st = common::make_test_state().await;
    let headers = HeaderMap::new();
    let body = Bytes::from(r#"{"event_type":"subscription.created"}"#);

    let status = handle_webhook(State(st), headers, body).await;
    assert_eq!(
        status,
        StatusCode::BAD_REQUEST,
        "missing signature → 400, got: {}",
        status
    );
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_paddle_webhook_returns_ok_when_secret_unconfigured() {
    // Config.paddle_webhook_secret = None → 200 OK (Paddle 재시도 방지).
    let mut st = common::make_test_state().await;
    st.cfg.paddle_webhook_secret = None;

    let mut headers = HeaderMap::new();
    headers.insert(
        "Paddle-Signature",
        HeaderValue::from_static("ts=1234567890;h1=irrelevant"),
    );
    let body = Bytes::from(r#"{"event_type":"subscription.created"}"#);

    let status = handle_webhook(State(st), headers, body).await;
    assert_eq!(
        status,
        StatusCode::OK,
        "secret None → 200 (no-retry), got: {}",
        status
    );
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_paddle_webhook_returns_bad_request_for_invalid_signature() {
    // 임의 (잘못된) HMAC 시그니처 → BadRequest (Paddle::unmarshal verification fail).
    let mut st = common::make_test_state().await;
    st.cfg.paddle_webhook_secret = Some("phase3-test-paddle-secret".to_string());

    let mut headers = HeaderMap::new();
    headers.insert(
        "Paddle-Signature",
        HeaderValue::from_static(
            "ts=1234567890;h1=0000000000000000000000000000000000000000000000000000000000000000",
        ),
    );
    let body = Bytes::from(r#"{"event_type":"subscription.created","data":{}}"#);

    let status = handle_webhook(State(st), headers, body).await;
    assert_eq!(
        status,
        StatusCode::BAD_REQUEST,
        "invalid signature → 400, got: {}",
        status
    );
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_paddle_webhook_returns_bad_request_for_valid_signature_but_invalid_event_payload() {
    // 유효 HMAC 시그니처 + Paddle SDK 가 deserialize 할 수 없는 JSON → Paddle::unmarshal Event 파싱 fail → BadRequest.
    let secret = "phase3-test-paddle-secret";
    let mut st = common::make_test_state().await;
    st.cfg.paddle_webhook_secret = Some(secret.to_string());

    let timestamp = chrono::Utc::now().timestamp();
    let body_str = r#"{"not_a_paddle_event":true}"#;
    let signature = paddle_signature(secret, timestamp, body_str);

    let mut headers = HeaderMap::new();
    headers.insert(
        "Paddle-Signature",
        HeaderValue::from_str(&signature).expect("valid header"),
    );
    let body = Bytes::from(body_str.to_string());

    let status = handle_webhook(State(st), headers, body).await;
    assert_eq!(
        status,
        StatusCode::BAD_REQUEST,
        "valid sig + invalid event JSON → 400, got: {}",
        status
    );
}
