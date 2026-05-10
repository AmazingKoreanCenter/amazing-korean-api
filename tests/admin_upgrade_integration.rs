//! Phase 3 통합 테스트 — `UpgradeService` (B7 트랙).
//!
//! ## 범위 — 초대 코드 verify (Redis 미존재 → Unauthorized)

mod common;

use amazing_korean_api::api::admin::upgrade::dto::UpgradeInviteReq;
use amazing_korean_api::api::admin::upgrade::service::UpgradeService;
use amazing_korean_api::error::AppError;
use common::{cleanup_test_user, insert_test_user, TestUserSpec};

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

// =============================================================================
// C-admin-invite — create_invite (RBAC validation + AdminInvite 이메일 발송 happy)
// =============================================================================

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_create_invite_rejects_invalid_email_format() {
    let st = common::make_test_state().await;
    let req = UpgradeInviteReq {
        email: "not-an-email".to_string(),
        role: "admin".to_string(),
    };
    let result = UpgradeService::create_invite(&st, 999_999_600, req).await;
    match result {
        Err(AppError::BadRequest(_)) => {}
        Err(e) => panic!("invalid email → BadRequest expected, got Err: {:?}", e),
        Ok(_) => panic!("invalid email → Err expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_create_invite_rejects_invalid_role() {
    let st = common::make_test_state().await;
    let req = UpgradeInviteReq {
        email: "phase3-invite@example.com".to_string(),
        role: "superuser".to_string(), // admin/manager 외
    };
    let result = UpgradeService::create_invite(&st, 999_999_601, req).await;
    match result {
        Err(AppError::BadRequest(_)) => {}
        Err(e) => panic!("invalid role → BadRequest expected, got Err: {:?}", e),
        Ok(_) => panic!("invalid role → Err expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_create_invite_rejects_when_actor_not_found() {
    // Validation 통과 후 actor lookup → 존재하지 않는 actor → Unauthorized.
    let st = common::make_test_state().await;
    let unique = uuid::Uuid::new_v4();
    let req = UpgradeInviteReq {
        email: format!("phase3-invite-{}@example.com", unique),
        role: "admin".to_string(),
    };
    let result = UpgradeService::create_invite(&st, 999_999_602, req).await;
    match result {
        Err(AppError::Unauthorized(msg)) => {
            assert!(msg.contains("Actor"), "msg에 'Actor' 포함, got: {}", msg);
        }
        Err(e) => panic!("actor missing → Unauthorized expected, got Err: {:?}", e),
        Ok(_) => panic!("actor missing → Err expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_create_invite_rejects_when_actor_lacks_permission() {
    // Learner (권한 부족) 가 admin 역할 초대 시도 → Forbidden.
    let st = common::make_test_state().await;

    // Actor = Learner (default user_auth)
    let spec = TestUserSpec::random();
    let actor_id = insert_test_user(&st, &spec).await;

    let req = UpgradeInviteReq {
        email: format!("phase3-target-{}@example.com", uuid::Uuid::new_v4()),
        role: "admin".to_string(),
    };
    let result = UpgradeService::create_invite(&st, actor_id, req).await;
    match result {
        Err(AppError::Forbidden(msg)) => {
            assert_eq!(msg, "UPGRADE_403_INSUFFICIENT_PERMISSION", "got: {}", msg);
        }
        Err(e) => panic!(
            "Learner → admin invite → Forbidden expected, got Err: {:?}",
            e
        ),
        Ok(_) => panic!("Learner → admin invite → Err expected, got Ok"),
    }

    cleanup_test_user(&st, actor_id).await;
}
