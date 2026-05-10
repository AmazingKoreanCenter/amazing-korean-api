//! Phase 3 통합 테스트 — `EbookService` (B2 트랙).
//!
//! ## 범위
//!
//! 외부 호출 (S3 / 파일시스템 / Paddle) 미발생 path:
//! - DB-only: get_my_purchases (non-existent user) / cancel_pending_purchase (non-existent code)
//! - 세션 verify_session: Redis 미존재 → Unauthorized

mod common;

use amazing_korean_api::api::ebook::service::EbookService;
use amazing_korean_api::error::AppError;

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_get_my_purchases_returns_empty_for_user_without_purchases() {
    // 존재하지 않는 user_id → Ok(MyPurchasesRes { items: empty }).
    let st = common::make_test_state().await;

    let result = EbookService::get_my_purchases(&st, 999_999_990).await;
    let res = match result {
        Ok(r) => r,
        Err(e) => panic!("non-existent user → Ok(empty) expected, got Err: {:?}", e),
    };
    assert!(
        res.items.is_empty(),
        "items=empty for user without purchases"
    );
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_cancel_pending_purchase_returns_not_found_for_unknown_code() {
    // 존재하지 않는 purchase_code → NotFound (deleted=0 row).
    let st = common::make_test_state().await;

    let unknown_code = format!("phase3_ebook_unknown_{}", uuid::Uuid::new_v4());
    let result = EbookService::cancel_pending_purchase(&st, 999_999_991, &unknown_code).await;
    match result {
        Err(AppError::NotFound) => {}
        Err(e) => panic!("unknown code → NotFound expected, got Err: {:?}", e),
        Ok(_) => panic!("unknown code → Err expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_verify_session_returns_unauthorized_for_unknown_session() {
    // Redis ak:ebook_session:* 미존재 → Unauthorized.
    let st = common::make_test_state().await;

    let unknown_sid = format!("phase3_ebook_sid_{}", uuid::Uuid::new_v4());
    let result = EbookService::verify_session(&st, 999_999_992, &unknown_sid).await;
    match result {
        Err(AppError::Forbidden(msg)) => {
            // 코드는 "Viewer session expired" 로 만료/미존재 동일 처리 (anti-enumeration).
            assert!(msg.contains("session"), "session 관련 메시지, got: {}", msg);
        }
        Err(e) => panic!("unknown session → Forbidden expected, got Err: {:?}", e),
        Ok(_) => panic!("unknown session → Err expected, got Ok"),
    }
}
