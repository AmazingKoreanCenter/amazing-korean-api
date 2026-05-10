//! Phase 3 통합 테스트 — `TextbookService` (B5 트랙).
//!
//! ## 범위 — 주문 lookup non-existent path

mod common;

use amazing_korean_api::api::textbook::service::TextbookService;
use amazing_korean_api::error::AppError;

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_get_my_orders_returns_empty_for_user_without_orders() {
    let st = common::make_test_state().await;

    let result = TextbookService::get_my_orders(&st, 999_999_980).await;
    let orders = match result {
        Ok(o) => o,
        Err(e) => panic!("non-existent user → Ok(empty) expected, got Err: {:?}", e),
    };
    assert!(orders.is_empty(), "orders=empty for user without orders");
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_get_order_by_code_returns_not_found_for_unknown_code() {
    let st = common::make_test_state().await;

    let unknown_code = format!("phase3_textbook_{}", uuid::Uuid::new_v4());
    let result = TextbookService::get_order_by_code(&st, &unknown_code).await;
    match result {
        Err(AppError::NotFound) => {}
        Err(e) => panic!("unknown code → NotFound expected, got Err: {:?}", e),
        Ok(_) => panic!("unknown code → Err expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_get_order_by_id_returns_not_found_for_unknown_id() {
    let st = common::make_test_state().await;

    let result = TextbookService::get_order_by_id(&st, 999_999_981).await;
    match result {
        Err(AppError::NotFound) => {}
        Err(e) => panic!("unknown id → NotFound expected, got Err: {:?}", e),
        Ok(_) => panic!("unknown id → Err expected, got Ok"),
    }
}
