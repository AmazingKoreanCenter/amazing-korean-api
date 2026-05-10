//! Phase 3 통합 테스트 — `UpgradeService` (B7 트랙).
//!
//! ## 범위 — 초대 코드 verify (Redis 미존재 → Unauthorized)

mod common;

use amazing_korean_api::api::admin::upgrade::service::UpgradeService;
use amazing_korean_api::error::AppError;

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_verify_invite_returns_unauthorized_for_unknown_code() {
    // Redis 미존재 invite code → UPGRADE_401_INVALID_CODE.
    let st = common::make_test_state().await;

    let unknown_code = format!("phase3_invite_{}", uuid::Uuid::new_v4());
    let result = UpgradeService::verify_invite(&st, &unknown_code).await;
    match result {
        Err(AppError::Unauthorized(msg)) => {
            assert_eq!(msg, "UPGRADE_401_INVALID_CODE", "got: {}", msg);
        }
        Err(e) => panic!(
            "unknown code → UPGRADE_401_INVALID_CODE expected, got Err: {:?}",
            e
        ),
        Ok(_) => panic!("unknown code → Err expected, got Ok"),
    }
}
