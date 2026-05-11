// Track 4 의 make_subscription_created_event_json 의 nested serde_json::json! 가
// default macro recursion limit (128) 초과 = 본 attribute 필수.
#![recursion_limit = "512"]

//! Phase 3 통합 테스트 — `PaymentService` (B1 트랙).
//!
//! ## 범위
//!
//! Paddle SDK mock 미도입 = 외부 호출 path 제외. testable subset:
//! - DB-only query: `get_subscription` / `has_active_subscription` (non-existent user)
//! - Provider 부재 path: `get_plans` / `cancel_subscription` 가 `payment=None` 시 ServiceUnavailable
//! - `cancel_subscription` user 에 sub record 없음 → BadRequest
//! - Webhook signature path: missing / unconfigured / invalid / valid-sig+invalid-payload
//! - **Track 4 (2026-05-11)**: subscription.created happy path = process_webhook_event 직접 호출
//!   + DB 부작용 verify + idempotency (2회 호출 = 1 row)
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
use paddle_rust_sdk::entities::Event;
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

// =============================================================================
// Track 4 (2026-05-11) — subscription.created happy path
// process_webhook_event 직접 호출 (signature path 우회, wire-format은 위 4 tests 가 검증).
// =============================================================================

/// Track 4 helper — minimal valid subscription.* Event JSON 구성.
///
/// `event_type` = `subscription.created` / `subscription.activated` / `subscription.updated` /
///   `subscription.canceled` / `subscription.paused` / `subscription.past_due` /
///   `subscription.trialing` / `subscription.resumed`.
/// `status` = paddle SubscriptionStatus snake_case (`trialing` / `active` / `past_due` /
///   `paused` / `canceled`).
/// `user_id` = custom_data 에 stringified i64 로 주입.
/// `provider_sub_id` = subscription.id (DB unique constraint = 테스트마다 random suffix 권장).
#[allow(dead_code)]
fn make_subscription_event_json(
    event_type: &str,
    status: &str,
    event_id: &str,
    provider_sub_id: &str,
    customer_id: &str,
    user_id: i64,
    price_id: &str,
) -> serde_json::Value {
    let now = chrono::Utc::now().to_rfc3339();
    let period_end = (chrono::Utc::now() + chrono::Duration::days(30)).to_rfc3339();
    serde_json::json!({
        "event_id": event_id,
        "occurred_at": now,
        "event_type": event_type,
        "data": {
            "id": provider_sub_id,
            "status": status,
            "customer_id": customer_id,
            "address_id": "add_test_01",
            "currency_code": "USD",
            "created_at": now,
            "updated_at": now,
            "started_at": null,
            "first_billed_at": null,
            "next_billed_at": period_end,
            "paused_at": null,
            "canceled_at": null,
            "discount": null,
            "collection_mode": "automatic",
            "billing_details": null,
            "current_billing_period": {
                "starts_at": now,
                "ends_at": period_end,
            },
            "billing_cycle": {
                "interval": "month",
                "frequency": 1,
            },
            "scheduled_change": null,
            "management_urls": null,
            "items": [{
                // SubscriptionItemStatus = active / inactive / trialing. 핸들러 (handle_subscription_*)
                // 가 item.status 를 읽지 않음 = 모든 variants 에서 "active" 안전.
                "status": "active",
                "quantity": 1,
                "recurring": true,
                "created_at": now,
                "updated_at": now,
                "previously_billed_at": null,
                "next_billed_at": period_end,
                "trial_dates": {
                    "starts_at": now,
                    "ends_at": period_end,
                },
                "price": {
                    "id": price_id,
                    "product_id": "pro_test_01",
                    "description": "Test price",
                    "type": "standard",
                    "name": "Monthly",
                    "billing_cycle": {
                        "interval": "month",
                        "frequency": 1,
                    },
                    "trial_period": null,
                    "tax_mode": "account_setting",
                    "unit_price": {
                        "amount": "1000",
                        "currency_code": "USD",
                    },
                    "unit_price_overrides": [],
                    "quantity": {
                        "minimum": 1,
                        "maximum": 1,
                    },
                    "status": "active",
                    "custom_data": null,
                    "import_meta": null,
                    "created_at": now,
                    "updated_at": now,
                },
                "product": {
                    "id": "pro_test_01",
                    "name": "Test Product",
                    "description": null,
                    "type": "standard",
                    "tax_category": "standard",
                    "image_url": null,
                    "custom_data": null,
                    "status": "active",
                    "import_meta": null,
                    "created_at": now,
                    "updated_at": now,
                    "prices": null,
                },
            }],
            "custom_data": {
                "user_id": user_id.to_string(),
            },
            "import_meta": null,
        }
    })
}

/// Track 4 helper — Event JSON 을 paddle_rust_sdk::entities::Event 로 deserialize.
#[allow(dead_code)]
fn parse_event(value: serde_json::Value) -> Result<Event, serde_json::Error> {
    serde_json::from_value(value)
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_subscription_created_event_json_deserializes_via_paddle_sdk() {
    // wire-format 검증 = JSON 이 Paddle SDK 의 Event 로 deserialize 가능한지 확인.
    // 본 test 가 fail = JSON 필드 누락/형식 오류 → 다른 happy-path test 진입 불가.
    let value = make_subscription_event_json(
        "subscription.created",
        "trialing",
        "evt_test_01",
        "sub_test_01",
        "ctm_test_01",
        12345_i64,
        "pri_test_01",
    );
    let event =
        parse_event(value).expect("Subscription event JSON 이 Paddle SDK 로 deserialize 되어야 함");
    assert_eq!(event.event_id.to_string(), "evt_test_01");
    match event.data {
        paddle_rust_sdk::enums::EventData::SubscriptionCreated(sub) => {
            assert_eq!(sub.id.to_string(), "sub_test_01");
            assert_eq!(sub.customer_id.to_string(), "ctm_test_01");
            assert!(sub.custom_data.is_some());
        }
        _ => panic!("event.data 가 SubscriptionCreated 가 아님"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_process_webhook_event_subscription_created_inserts_db_row() {
    // happy path = subscription.created 이벤트 처리 → payment_subscription 테이블에 row INSERT.
    use amazing_korean_api::api::payment::repo::PaymentRepo;

    let st = common::make_test_state().await;

    // 0) 사전 시드 = users 행 (subscription FK 조건)
    let spec = common::TestUserSpec::random();
    let user_id = common::insert_test_user(&st, &spec).await;

    // 1) Event 구성 = unique sub_id (테스트 격리)
    let unique = uuid::Uuid::new_v4().to_string()[..8].to_string();
    let provider_sub_id = format!("sub_track4_{}", unique);
    let event_id = format!("evt_track4_{}", unique);

    let value = make_subscription_event_json(
        "subscription.created",
        "trialing",
        &event_id,
        &provider_sub_id,
        &format!("ctm_track4_{}", unique),
        user_id,
        "pri_track4_no_match", // billing_interval_for_price = None → fallback Month1
    );
    let raw_body = value.to_string();
    let event = parse_event(value).expect("event must deserialize");

    // 2) process_webhook_event 호출
    PaymentService::process_webhook_event(&st, event, &raw_body)
        .await
        .expect("process_webhook_event 가 happy path 에서 Ok 반환");

    // 3) DB 부작용 verify = payment_subscription 행 INSERT
    let existing = PaymentRepo::get_subscription_by_provider_id(&st.db, &provider_sub_id)
        .await
        .expect("find by provider id");
    assert!(
        existing.is_some(),
        "payment_subscription 행이 INSERT 되어야 함"
    );

    // 4) cleanup = users 삭제 (subscription 도 CASCADE)
    common::cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_process_webhook_event_is_idempotent_for_same_event_id() {
    // 멱등성 = 동일 event_id 로 2회 호출 = 1 row 만 INSERT (PaymentRepo::is_webhook_event_processed 검사).
    use amazing_korean_api::api::payment::repo::PaymentRepo;

    let st = common::make_test_state().await;
    let spec = common::TestUserSpec::random();
    let user_id = common::insert_test_user(&st, &spec).await;

    let unique = uuid::Uuid::new_v4().to_string()[..8].to_string();
    let provider_sub_id = format!("sub_idem_{}", unique);
    let event_id = format!("evt_idem_{}", unique);
    let value = make_subscription_event_json(
        "subscription.created",
        "trialing",
        &event_id,
        &provider_sub_id,
        &format!("ctm_idem_{}", unique),
        user_id,
        "pri_idem_no_match",
    );
    let raw_body = value.to_string();

    // 1회차
    let event1 = parse_event(value.clone()).expect("event must deserialize");
    PaymentService::process_webhook_event(&st, event1, &raw_body)
        .await
        .expect("1st call ok");

    // 2회차 (동일 event_id) — is_webhook_event_processed = true → skip
    let event2 = parse_event(value).expect("event must deserialize");
    PaymentService::process_webhook_event(&st, event2, &raw_body)
        .await
        .expect("2nd call ok (멱등성)");

    // DB row count = 1 (멱등성 보장)
    let existing = PaymentRepo::get_subscription_by_provider_id(&st.db, &provider_sub_id)
        .await
        .expect("find by provider id");
    assert!(existing.is_some(), "subscription 행 1건");

    common::cleanup_test_user(&st, user_id).await;
}

// =============================================================================
// T-Subset-Txn (2026-05-11) — transaction.completed + adjustment.created/updated helpers
// =============================================================================

/// T-Subset-Txn helper — minimal valid transaction.completed Event JSON.
/// `subscription_id` = 연결된 구독 (None 가능, ebook 결제는 custom_data.type="ebook" 필요).
/// `total_cents` / `tax_cents` = 정수 문자열 (Paddle 규약).
#[allow(dead_code, clippy::too_many_arguments)]
fn make_transaction_completed_event_json(
    event_id: &str,
    txn_id: &str,
    customer_id: &str,
    subscription_id: Option<&str>,
    user_id: i64,
    price_id: &str,
    total_cents: &str,
    tax_cents: &str,
) -> serde_json::Value {
    let now = chrono::Utc::now().to_rfc3339();
    let line_item_id = format!("txnitm_{}", &txn_id[4..]);
    // Totals (line_items 의 unit_totals/totals) = subtotal/discount/tax/total 4 필드.
    let totals_block = serde_json::json!({
        "subtotal": total_cents,
        "discount": "0",
        "tax": tax_cents,
        "total": total_cents,
    });
    serde_json::json!({
        "event_id": event_id,
        "occurred_at": now,
        "event_type": "transaction.completed",
        "data": {
            "id": txn_id,
            "status": "completed",
            "customer_id": customer_id,
            "address_id": "add_test_01",
            "business_id": null,
            "custom_data": { "user_id": user_id.to_string() },
            "currency_code": "USD",
            "origin": "web",
            "subscription_id": subscription_id,
            "invoice_id": null,
            "invoice_number": null,
            "collection_mode": "automatic",
            "discount_id": null,
            "billing_details": null,
            "billing_period": {
                "starts_at": now,
                "ends_at": now,
            },
            "items": [{
                "price": {
                    "id": price_id,
                    "product_id": "pro_test_01",
                    "description": "Test price",
                    "type": "standard",
                    "name": "Monthly",
                    "billing_cycle": { "interval": "month", "frequency": 1 },
                    "trial_period": null,
                    "tax_mode": "account_setting",
                    "unit_price": { "amount": total_cents, "currency_code": "USD" },
                    "unit_price_overrides": [],
                    "quantity": { "minimum": 1, "maximum": 1 },
                    "status": "active",
                    "custom_data": null,
                    "import_meta": null,
                    "created_at": now,
                    "updated_at": now,
                },
                "quantity": 1,
                "proration": null,
            }],
            "details": {
                "tax_rates_used": [],
                "totals": {
                    "subtotal": total_cents,
                    "discount": "0",
                    "tax": tax_cents,
                    "total": total_cents,
                    "credit": "0",
                    "credit_to_balance": "0",
                    "balance": "0",
                    "grand_total": total_cents,
                    "fee": null,
                    "earnings": null,
                    "currency_code": "USD",
                },
                "adjusted_totals": {
                    "subtotal": total_cents,
                    "tax": tax_cents,
                    "total": total_cents,
                    "grand_total": total_cents,
                    "fee": null,
                    "earnings": null,
                    "currency_code": "USD",
                },
                "payout_totals": null,
                "adjusted_payout_totals": null,
                "line_items": [{
                    "id": line_item_id,
                    "price_id": price_id,
                    "quantity": 1,
                    "proration": null,
                    "tax_rate": "0",
                    "unit_totals": totals_block,
                    "totals": totals_block,
                    "product": {
                        "id": "pro_test_01",
                        "name": "Test Product",
                        "description": null,
                        "type": "standard",
                        "tax_category": "standard",
                        "image_url": null,
                        "custom_data": null,
                        "status": "active",
                        "import_meta": null,
                        "created_at": now,
                        "updated_at": now,
                        "prices": null,
                    },
                }],
            },
            "payments": [],
            "checkout": { "url": null },
            "created_at": now,
            "updated_at": now,
            "billed_at": now,
            "revised_at": null,
        }
    })
}

/// T-Subset-Txn helper — adjustment.created Event JSON (refund + approved 가 핸들러 진입 조건).
#[allow(dead_code)]
fn make_adjustment_created_event_json(
    event_id: &str,
    adj_id: &str,
    txn_id: &str,
    customer_id: &str,
    action: &str, // "refund" / "credit" / etc.
    status: &str, // "approved" / "pending_approval" / etc.
    amount_cents: &str,
) -> serde_json::Value {
    let now = chrono::Utc::now().to_rfc3339();
    serde_json::json!({
        "event_id": event_id,
        "occurred_at": now,
        "event_type": "adjustment.created",
        "data": {
            "id": adj_id,
            "action": action,
            "type": "full",
            "transaction_id": txn_id,
            "subscription_id": null,
            "customer_id": customer_id,
            "reason": "test refund",
            "credit_applied_to_balance": null,
            "currency_code": "USD",
            "status": status,
            "items": [],
            "totals": {
                "subtotal": amount_cents,
                "tax": "0",
                "total": amount_cents,
                "fee": "0",
                "earnings": amount_cents,
                "currency_code": "USD",
            },
            "payout_totals": null,
            "created_at": now,
            "updated_at": now,
        }
    })
}

// =============================================================================
// T-Subset-Cont (2026-05-11) — 나머지 6 Subscription variants 상태 전이 검증
// 패턴: subscription.created (INSERT) → variant 이벤트 (UPDATE status) → 상태 verify.
// =============================================================================

/// T-Subset-Cont helper — create + variant 시퀀스 실행 후 sub row 상태 반환.
#[allow(dead_code)]
async fn run_create_then_variant(
    st: &amazing_korean_api::state::AppState,
    user_id: i64,
    unique: &str,
    variant_event_type: &str,
    variant_status: &str,
) -> amazing_korean_api::api::payment::repo::SubscriptionRow {
    use amazing_korean_api::api::payment::repo::PaymentRepo;

    let provider_sub_id = format!("sub_{}_{}", variant_event_type.replace('.', "_"), unique);
    let customer_id = format!("ctm_{}_{}", variant_event_type.replace('.', "_"), unique);
    let price_id = "pri_variant_no_match";

    // 1) create
    let create_event_id = format!("evt_create_{}", unique);
    let create_value = make_subscription_event_json(
        "subscription.created",
        "trialing",
        &create_event_id,
        &provider_sub_id,
        &customer_id,
        user_id,
        price_id,
    );
    let create_raw = create_value.to_string();
    let create_event = parse_event(create_value).expect("create event must deserialize");
    PaymentService::process_webhook_event(st, create_event, &create_raw)
        .await
        .expect("create ok");

    // 2) variant
    let variant_event_id = format!("evt_variant_{}", unique);
    let variant_value = make_subscription_event_json(
        variant_event_type,
        variant_status,
        &variant_event_id,
        &provider_sub_id,
        &customer_id,
        user_id,
        price_id,
    );
    let variant_raw = variant_value.to_string();
    let variant_event = parse_event(variant_value).expect("variant event must deserialize");
    PaymentService::process_webhook_event(st, variant_event, &variant_raw)
        .await
        .expect("variant ok");

    // 3) read back
    PaymentRepo::get_subscription_by_provider_id(&st.db, &provider_sub_id)
        .await
        .expect("find by provider id")
        .expect("row must exist after create")
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_subscription_activated_transitions_status_to_active() {
    use amazing_korean_api::types::SubscriptionStatus;
    let st = common::make_test_state().await;
    let spec = common::TestUserSpec::random();
    let user_id = common::insert_test_user(&st, &spec).await;
    let unique = uuid::Uuid::new_v4().to_string()[..8].to_string();

    let row =
        run_create_then_variant(&st, user_id, &unique, "subscription.activated", "active").await;
    assert_eq!(row.status, SubscriptionStatus::Active);

    common::cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_subscription_updated_transitions_status_to_active() {
    // subscription.updated = sub.status 값에 따라 SubscriptionStatus 전환 (paddle_status_to_internal).
    use amazing_korean_api::types::SubscriptionStatus;
    let st = common::make_test_state().await;
    let spec = common::TestUserSpec::random();
    let user_id = common::insert_test_user(&st, &spec).await;
    let unique = uuid::Uuid::new_v4().to_string()[..8].to_string();

    let row =
        run_create_then_variant(&st, user_id, &unique, "subscription.updated", "active").await;
    assert_eq!(row.status, SubscriptionStatus::Active);

    common::cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_subscription_canceled_transitions_status_to_canceled() {
    use amazing_korean_api::types::SubscriptionStatus;
    let st = common::make_test_state().await;
    let spec = common::TestUserSpec::random();
    let user_id = common::insert_test_user(&st, &spec).await;
    let unique = uuid::Uuid::new_v4().to_string()[..8].to_string();

    let row =
        run_create_then_variant(&st, user_id, &unique, "subscription.canceled", "canceled").await;
    assert_eq!(row.status, SubscriptionStatus::Canceled);

    common::cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_subscription_paused_transitions_status_to_paused() {
    use amazing_korean_api::types::SubscriptionStatus;
    let st = common::make_test_state().await;
    let spec = common::TestUserSpec::random();
    let user_id = common::insert_test_user(&st, &spec).await;
    let unique = uuid::Uuid::new_v4().to_string()[..8].to_string();

    let row = run_create_then_variant(&st, user_id, &unique, "subscription.paused", "paused").await;
    assert_eq!(row.status, SubscriptionStatus::Paused);

    common::cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_subscription_past_due_transitions_status_to_past_due() {
    use amazing_korean_api::types::SubscriptionStatus;
    let st = common::make_test_state().await;
    let spec = common::TestUserSpec::random();
    let user_id = common::insert_test_user(&st, &spec).await;
    let unique = uuid::Uuid::new_v4().to_string()[..8].to_string();

    let row =
        run_create_then_variant(&st, user_id, &unique, "subscription.past_due", "past_due").await;
    assert_eq!(row.status, SubscriptionStatus::PastDue);

    common::cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_subscription_trialing_transitions_status_to_trialing() {
    // create 가 이미 Trialing 으로 INSERT 함. trialing 이벤트는 정확히 Trialing 으로 갱신 (idempotent at state level).
    use amazing_korean_api::types::SubscriptionStatus;
    let st = common::make_test_state().await;
    let spec = common::TestUserSpec::random();
    let user_id = common::insert_test_user(&st, &spec).await;
    let unique = uuid::Uuid::new_v4().to_string()[..8].to_string();

    let row =
        run_create_then_variant(&st, user_id, &unique, "subscription.trialing", "trialing").await;
    assert_eq!(row.status, SubscriptionStatus::Trialing);

    common::cleanup_test_user(&st, user_id).await;
}

// =============================================================================
// T-Subset-Txn (2026-05-11) — Transaction + Adjustment 이벤트 happy path
// =============================================================================

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_transaction_completed_event_json_deserializes_via_paddle_sdk() {
    // wire-format 검증 = Transaction Event JSON 이 Paddle SDK 로 deserialize 되는지.
    let value = make_transaction_completed_event_json(
        "evt_txn_01",
        "txn_test_01",
        "ctm_test_01",
        Some("sub_test_01"),
        12345_i64,
        "pri_test_01",
        "1000",
        "0",
    );
    let event =
        parse_event(value).expect("Transaction event JSON 이 Paddle SDK 로 deserialize 되어야 함");
    match event.data {
        paddle_rust_sdk::enums::EventData::TransactionCompleted(txn) => {
            assert_eq!(txn.id.to_string(), "txn_test_01");
            assert_eq!(
                txn.subscription_id.as_ref().map(|s| s.to_string()),
                Some("sub_test_01".to_string())
            );
        }
        _ => panic!("event.data 가 TransactionCompleted 가 아님"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_process_webhook_event_transaction_completed_inserts_db_row() {
    // happy path = 사전 subscription 생성 → transaction.completed → payment_transaction INSERT.
    let st = common::make_test_state().await;
    let spec = common::TestUserSpec::random();
    let user_id = common::insert_test_user(&st, &spec).await;

    let unique = uuid::Uuid::new_v4().to_string()[..8].to_string();
    let provider_sub_id = format!("sub_txn_{}", unique);
    let customer_id = format!("ctm_txn_{}", unique);
    let provider_txn_id = format!("txn_track4_{}", unique);

    // 1) 사전 subscription 시드 (handle_transaction_completed 가 sub 조회로 user_id 추출)
    let create_value = make_subscription_event_json(
        "subscription.created",
        "trialing",
        &format!("evt_create_{}", unique),
        &provider_sub_id,
        &customer_id,
        user_id,
        "pri_txn_no_match",
    );
    let create_raw = create_value.to_string();
    let create_event = parse_event(create_value).expect("create event must deserialize");
    PaymentService::process_webhook_event(&st, create_event, &create_raw)
        .await
        .expect("subscription create ok");

    // 2) transaction.completed
    let txn_value = make_transaction_completed_event_json(
        &format!("evt_txn_{}", unique),
        &provider_txn_id,
        &customer_id,
        Some(&provider_sub_id),
        user_id,
        "pri_txn_no_match",
        "1000",
        "0",
    );
    let txn_raw = txn_value.to_string();
    let txn_event = parse_event(txn_value).expect("txn event must deserialize");
    PaymentService::process_webhook_event(&st, txn_event, &txn_raw)
        .await
        .expect("transaction.completed ok");

    // 3) DB 부작용 = payment_transaction 행 INSERT
    let txn_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM transactions WHERE provider_transaction_id = $1")
            .bind(&provider_txn_id)
            .fetch_one(&st.db)
            .await
            .expect("count txn");
    assert_eq!(txn_count, 1, "payment_transaction 행이 INSERT 되어야 함");

    common::cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_adjustment_event_json_deserializes_via_paddle_sdk() {
    // wire-format 검증 = Adjustment Event JSON deserialize.
    let value = make_adjustment_created_event_json(
        "evt_adj_01",
        "adj_test_01",
        "txn_test_01",
        "ctm_test_01",
        "refund",
        "approved",
        "1000",
    );
    let event =
        parse_event(value).expect("Adjustment event JSON 이 Paddle SDK 로 deserialize 되어야 함");
    match event.data {
        paddle_rust_sdk::enums::EventData::AdjustmentCreated(adj) => {
            assert_eq!(adj.id.to_string(), "adj_test_01");
            assert_eq!(adj.transaction_id.to_string(), "txn_test_01");
        }
        _ => panic!("event.data 가 AdjustmentCreated 가 아님"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_process_webhook_event_adjustment_refund_approved_marks_transaction_refunded() {
    // happy path = sub 시드 → transaction.completed → adjustment.created (refund + approved)
    //            → payment_transaction.status = Refunded.
    use amazing_korean_api::types::TransactionStatus;

    let st = common::make_test_state().await;
    let spec = common::TestUserSpec::random();
    let user_id = common::insert_test_user(&st, &spec).await;

    let unique = uuid::Uuid::new_v4().to_string()[..8].to_string();
    let provider_sub_id = format!("sub_adj_{}", unique);
    let customer_id = format!("ctm_adj_{}", unique);
    let provider_txn_id = format!("txn_adj_{}", unique);

    // 1) sub create
    let sub_value = make_subscription_event_json(
        "subscription.created",
        "trialing",
        &format!("evt_sc_{}", unique),
        &provider_sub_id,
        &customer_id,
        user_id,
        "pri_adj_no_match",
    );
    let sub_raw = sub_value.to_string();
    let sub_event = parse_event(sub_value).expect("sub event must deserialize");
    PaymentService::process_webhook_event(&st, sub_event, &sub_raw)
        .await
        .expect("sub create ok");

    // 2) transaction.completed (refund 대상)
    let txn_value = make_transaction_completed_event_json(
        &format!("evt_tc_{}", unique),
        &provider_txn_id,
        &customer_id,
        Some(&provider_sub_id),
        user_id,
        "pri_adj_no_match",
        "1000",
        "0",
    );
    let txn_raw = txn_value.to_string();
    let txn_event = parse_event(txn_value).expect("txn event must deserialize");
    PaymentService::process_webhook_event(&st, txn_event, &txn_raw)
        .await
        .expect("txn ok");

    // 3) adjustment.created (refund + approved)
    let adj_value = make_adjustment_created_event_json(
        &format!("evt_adj_{}", unique),
        &format!("adj_{}", unique),
        &provider_txn_id,
        &customer_id,
        "refund",
        "approved",
        "1000",
    );
    let adj_raw = adj_value.to_string();
    let adj_event = parse_event(adj_value).expect("adj event must deserialize");
    PaymentService::process_webhook_event(&st, adj_event, &adj_raw)
        .await
        .expect("adjustment ok");

    // 4) DB 부작용 = transaction status = Refunded
    let txn_status: TransactionStatus =
        sqlx::query_scalar("SELECT status FROM transactions WHERE provider_transaction_id = $1")
            .bind(&provider_txn_id)
            .fetch_one(&st.db)
            .await
            .expect("fetch txn status");
    assert_eq!(
        txn_status,
        TransactionStatus::Refunded,
        "refund + approved → transaction.status = Refunded"
    );

    common::cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_adjustment_credit_action_is_skipped() {
    // refund 가 아닌 action (credit) = handler 가 early return → DB 변화 없음.
    use amazing_korean_api::types::TransactionStatus;

    let st = common::make_test_state().await;
    let spec = common::TestUserSpec::random();
    let user_id = common::insert_test_user(&st, &spec).await;

    let unique = uuid::Uuid::new_v4().to_string()[..8].to_string();
    let provider_sub_id = format!("sub_cre_{}", unique);
    let customer_id = format!("ctm_cre_{}", unique);
    let provider_txn_id = format!("txn_cre_{}", unique);

    // sub + txn 시드
    let sub_value = make_subscription_event_json(
        "subscription.created",
        "trialing",
        &format!("evt_sc_{}", unique),
        &provider_sub_id,
        &customer_id,
        user_id,
        "pri_cre_no_match",
    );
    let sub_event = parse_event(sub_value.clone()).expect("sub deser");
    PaymentService::process_webhook_event(&st, sub_event, &sub_value.to_string())
        .await
        .expect("sub ok");

    let txn_value = make_transaction_completed_event_json(
        &format!("evt_tc_{}", unique),
        &provider_txn_id,
        &customer_id,
        Some(&provider_sub_id),
        user_id,
        "pri_cre_no_match",
        "1000",
        "0",
    );
    let txn_event = parse_event(txn_value.clone()).expect("txn deser");
    PaymentService::process_webhook_event(&st, txn_event, &txn_value.to_string())
        .await
        .expect("txn ok");

    // adjustment = credit (refund 아님) → skip
    let adj_value = make_adjustment_created_event_json(
        &format!("evt_adj_{}", unique),
        &format!("adj_{}", unique),
        &provider_txn_id,
        &customer_id,
        "credit",
        "approved",
        "1000",
    );
    let adj_event = parse_event(adj_value.clone()).expect("adj deser");
    PaymentService::process_webhook_event(&st, adj_event, &adj_value.to_string())
        .await
        .expect("adjustment (credit) ok");

    // DB 변화 없음 = transaction.status = Completed 그대로
    let txn_status: TransactionStatus =
        sqlx::query_scalar("SELECT status FROM transactions WHERE provider_transaction_id = $1")
            .bind(&provider_txn_id)
            .fetch_one(&st.db)
            .await
            .expect("fetch txn status");
    assert_eq!(
        txn_status,
        TransactionStatus::Completed,
        "credit adjustment 은 skip → Completed 유지"
    );

    common::cleanup_test_user(&st, user_id).await;
}
