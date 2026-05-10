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

// =============================================================================
// C-ebook — 세션 라이프사이클 happy (register_session → heartbeat → verify_session)
// =============================================================================

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_register_session_creates_redis_entry_with_hmac_secret() {
    // register_session → ebook_viewer:<user_id> Redis key 생성 + (session_id, hmac_secret) 반환.
    let st = common::make_test_state().await;

    let user_id: i64 = 999_999_700;
    let purchase_code = "phase3-ebook-purchase-test";
    let (session_id, hmac_secret) = EbookService::register_session(&st, user_id, purchase_code)
        .await
        .expect("register_session 성공");

    assert!(!session_id.is_empty(), "session_id 발급");
    assert_eq!(
        hmac_secret.len(),
        64,
        "HMAC secret 32 bytes (hex 64자), got: {}",
        hmac_secret.len()
    );

    // Redis 검증
    let mut conn = st.redis.get().await.expect("redis");
    let session_key = format!("ebook_viewer:{}", user_id);
    let stored: Option<String> = redis::AsyncCommands::get(&mut conn, &session_key)
        .await
        .ok();
    assert!(stored.is_some(), "Redis 에 ebook_viewer:{} 저장됨", user_id);

    let parsed: serde_json::Value = serde_json::from_str(&stored.unwrap()).expect("JSON parse");
    assert_eq!(parsed["session_id"].as_str(), Some(session_id.as_str()));
    assert_eq!(parsed["purchase_code"].as_str(), Some(purchase_code));

    // Cleanup
    let _: () = redis::AsyncCommands::del(&mut conn, &session_key)
        .await
        .unwrap_or(());
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_heartbeat_returns_valid_for_matching_session() {
    // register_session 후 heartbeat (같은 session_id) → valid=true + TTL 갱신.
    let st = common::make_test_state().await;

    let user_id: i64 = 999_999_701;
    let (session_id, _secret) = EbookService::register_session(&st, user_id, "phase3-ebook-hb")
        .await
        .expect("register");

    let res = EbookService::heartbeat(&st, user_id, &session_id)
        .await
        .expect("heartbeat 성공");
    assert!(res.valid, "matching session → valid=true");

    // Cleanup
    let mut conn = st.redis.get().await.expect("redis");
    let _: () = redis::AsyncCommands::del(&mut conn, format!("ebook_viewer:{}", user_id))
        .await
        .unwrap_or(());
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_heartbeat_returns_invalid_for_mismatched_session_id() {
    // 다른 session_id 로 heartbeat → valid=false (세션 탈취 방지 검증).
    let st = common::make_test_state().await;

    let user_id: i64 = 999_999_702;
    let (_session_id, _secret) =
        EbookService::register_session(&st, user_id, "phase3-ebook-mismatch")
            .await
            .expect("register");

    let other_sid = uuid::Uuid::new_v4().to_string();
    let res = EbookService::heartbeat(&st, user_id, &other_sid)
        .await
        .expect("heartbeat 호출 자체는 Ok");
    assert!(!res.valid, "다른 session_id → valid=false");

    // Cleanup
    let mut conn = st.redis.get().await.expect("redis");
    let _: () = redis::AsyncCommands::del(&mut conn, format!("ebook_viewer:{}", user_id))
        .await
        .unwrap_or(());
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_verify_session_succeeds_for_matching_session() {
    // register_session 후 verify_session (같은 session_id) → Ok(()) (페이지/타일 요청 통과).
    let st = common::make_test_state().await;

    let user_id: i64 = 999_999_703;
    let (session_id, _secret) = EbookService::register_session(&st, user_id, "phase3-ebook-verify")
        .await
        .expect("register");

    let result = EbookService::verify_session(&st, user_id, &session_id).await;
    assert!(result.is_ok(), "matching session → Ok, got: {:?}", result);

    // Cleanup
    let mut conn = st.redis.get().await.expect("redis");
    let _: () = redis::AsyncCommands::del(&mut conn, format!("ebook_viewer:{}", user_id))
        .await
        .unwrap_or(());
}
