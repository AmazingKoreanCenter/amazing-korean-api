//! Phase 3 통합 테스트 — login / mfa flow.
//!
//! ## 셋업
//!
//! `tests/common/mod.rs` 의 `make_test_state()` + `insert_test_user()` 사용.
//! 환경변수 = `.env.test` 또는 `.env`.
//!
//! ## 실행
//!
//! ```bash
//! DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/amazing_korean_db \
//! REDIS_URL=redis://:redis_dev_password@127.0.0.1:16379 \
//! JWT_SECRET=test_jwt_secret_must_be_at_least_32_bytes_long \
//! HMAC_KEY=$(openssl rand -base64 32) \
//! ENCRYPTION_KEY_V1=$(openssl rand -base64 32) \
//!   cargo test --test auth_login_integration -- --ignored
//! ```
//!
//! ## 범위
//!
//! - login: validation / anti-enumeration / wrong password / email_not_verified / MFA challenge / OAuth-only
//! - mfa_login: 만료된 token / validation
//!
//! happy path (실 세션 생성 = login + redis_session/refresh INSERT) 은 본 트랙 미포함 (cleanup 부담).
//! Google OAuth callback = wiremock 도입 후 별도 트랙.

mod common;

use amazing_korean_api::api::auth::dto::{LoginReq, MfaLoginReq};
use amazing_korean_api::api::auth::handler::ParsedUa;
use amazing_korean_api::api::auth::service::{AuthService, LoginOutcome};
use amazing_korean_api::crypto::CryptoService;
use amazing_korean_api::error::AppError;
use common::{cleanup_test_user, insert_test_user, TestUserSpec};

fn parsed_ua_default() -> ParsedUa {
    ParsedUa {
        os: None,
        browser: None,
        device: "other".into(),
    }
}

async fn cleanup_login_rl(st: &amazing_korean_api::state::AppState, email: &str, ip: &str) {
    let mut conn = st.redis.get().await.expect("redis conn for cleanup");
    let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
    let idx = crypto.blind_index(email).expect("blind_index");
    let _: () = redis::AsyncCommands::del(&mut conn, format!("rl:login:{}:{}", idx, ip))
        .await
        .unwrap_or(());
}

// =============================================================================
// AuthService::login
// =============================================================================

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_login_validation_rejects_short_password() {
    // LoginReq.password = #[validate(length(min = 6, max = 72))]. 5자리 → ValidationGeneric.
    let st = common::make_test_state().await;

    let req = LoginReq {
        email: "test@example.com".to_string(),
        password: "12345".to_string(), // 5 chars (< min 6)
    };
    let result =
        AuthService::login(&st, req, "10.0.1.1".to_string(), None, parsed_ua_default()).await;
    match result {
        Err(AppError::ValidationGeneric) => {}
        Err(e) => panic!(
            "5자리 password → ValidationGeneric expected, got Err: {:?}",
            e
        ),
        Ok(_) => panic!("5자리 password → ValidationGeneric expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_login_anti_enumeration_for_non_existent_user() {
    // 존재하지 않는 email + 형식적으로 valid 한 password → AUTH_401_BAD_CREDENTIALS (anti-enumeration).
    let st = common::make_test_state().await;

    let unique_email = format!("login_noexist_{}@example.com", uuid::Uuid::new_v4());
    let ip = "10.0.1.2".to_string();

    let req = LoginReq {
        email: unique_email.clone(),
        password: "ValidPass123".to_string(),
    };
    let result = AuthService::login(&st, req, ip.clone(), None, parsed_ua_default()).await;
    match result {
        Err(AppError::Unauthorized(msg)) => {
            assert_eq!(msg, "AUTH_401_BAD_CREDENTIALS", "got: {}", msg);
        }
        Err(e) => panic!(
            "non-existent user → Unauthorized(AUTH_401_BAD_CREDENTIALS) expected, got Err: {:?}",
            e
        ),
        Ok(_) => panic!("non-existent user → Unauthorized expected, got Ok"),
    }

    cleanup_login_rl(&st, &unique_email.to_lowercase(), &ip).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_login_returns_bad_credentials_for_wrong_password() {
    // 존재하는 user + 잘못된 password → AUTH_401_BAD_CREDENTIALS.
    let st = common::make_test_state().await;

    let spec = TestUserSpec::random(); // password = "TestPass123", check_email=true
    let user_id = insert_test_user(&st, &spec).await;

    let req = LoginReq {
        email: spec.email.clone(),
        password: "WrongPass999".to_string(),
    };
    let ip = "10.0.1.3".to_string();
    let result = AuthService::login(&st, req, ip.clone(), None, parsed_ua_default()).await;
    match result {
        Err(AppError::Unauthorized(msg)) => {
            assert_eq!(msg, "AUTH_401_BAD_CREDENTIALS", "got: {}", msg);
        }
        Err(e) => panic!(
            "wrong password → Unauthorized(AUTH_401_BAD_CREDENTIALS) expected, got Err: {:?}",
            e
        ),
        Ok(_) => panic!("wrong password → Unauthorized expected, got Ok"),
    }

    cleanup_login_rl(&st, &spec.email.to_lowercase(), &ip).await;
    cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_login_returns_email_not_verified_for_unverified_user() {
    // 존재하는 user + 올바른 password + check_email=false → 403 AUTH_403_EMAIL_NOT_VERIFIED:<email>.
    let st = common::make_test_state().await;

    let mut spec = TestUserSpec::random();
    spec.check_email = false;
    let user_id = insert_test_user(&st, &spec).await;

    let req = LoginReq {
        email: spec.email.clone(),
        password: spec.password.clone(),
    };
    let ip = "10.0.1.4".to_string();
    let result = AuthService::login(&st, req, ip.clone(), None, parsed_ua_default()).await;
    match result {
        Err(AppError::Forbidden(msg)) => {
            assert!(
                msg.starts_with("AUTH_403_EMAIL_NOT_VERIFIED:"),
                "AUTH_403_EMAIL_NOT_VERIFIED:<email> 형식, got: {}",
                msg
            );
            assert!(
                msg.contains(&spec.email.to_lowercase()),
                "응답에 email 포함 (재발송 UI 용도), got: {}",
                msg
            );
        }
        Err(e) => panic!(
            "미인증 → Forbidden(AUTH_403_EMAIL_NOT_VERIFIED:...) expected, got Err: {:?}",
            e
        ),
        Ok(_) => panic!("미인증 → Forbidden expected, got Ok"),
    }

    cleanup_login_rl(&st, &spec.email.to_lowercase(), &ip).await;
    cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_login_returns_account_disabled_for_inactive_user() {
    // 존재하는 user + 올바른 password + user_state=false → 403 ACCOUNT_DISABLED.
    let st = common::make_test_state().await;

    let mut spec = TestUserSpec::random();
    spec.user_state = false;
    let user_id = insert_test_user(&st, &spec).await;

    let req = LoginReq {
        email: spec.email.clone(),
        password: spec.password.clone(),
    };
    let ip = "10.0.1.5".to_string();
    let result = AuthService::login(&st, req, ip.clone(), None, parsed_ua_default()).await;
    match result {
        Err(AppError::Forbidden(msg)) => {
            assert_eq!(msg, "ACCOUNT_DISABLED", "got: {}", msg);
        }
        Err(e) => panic!(
            "user_state=false → Forbidden(ACCOUNT_DISABLED) expected, got Err: {:?}",
            e
        ),
        Ok(_) => panic!("user_state=false → Forbidden expected, got Ok"),
    }

    cleanup_login_rl(&st, &spec.email.to_lowercase(), &ip).await;
    cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_login_returns_mfa_challenge_for_mfa_enabled_user() {
    // 존재하는 user + 올바른 password + check_email=true + mfa_enabled=true
    //   → LoginOutcome::MfaChallenge { mfa_token, user_id }
    // (MFA secret 검증은 후속 mfa_login 에서, login 단계는 챌린지 발급만)
    let st = common::make_test_state().await;

    let mut spec = TestUserSpec::random();
    spec.mfa_enabled = true;
    let user_id = insert_test_user(&st, &spec).await;

    let req = LoginReq {
        email: spec.email.clone(),
        password: spec.password.clone(),
    };
    let ip = "10.0.1.6".to_string();
    let result = AuthService::login(&st, req, ip.clone(), None, parsed_ua_default()).await;

    let mfa_token = match result {
        Ok(LoginOutcome::MfaChallenge {
            mfa_token,
            user_id: returned_user_id,
        }) => {
            assert_eq!(returned_user_id, user_id, "응답 user_id = inserted user_id");
            assert!(!mfa_token.is_empty(), "mfa_token 비어있지 않음");
            mfa_token
        }
        Ok(LoginOutcome::Success(_)) => {
            panic!("mfa_enabled=true → MfaChallenge expected, got Success")
        }
        Err(e) => panic!("mfa_enabled=true → MfaChallenge expected, got Err: {:?}", e),
    };

    // Redis 에 mfa_pending key 저장 검증 + cleanup
    let mut conn = st.redis.get().await.expect("redis conn for cleanup");
    let pending: Option<String> =
        redis::AsyncCommands::get(&mut conn, format!("ak:mfa_pending:{}", mfa_token))
            .await
            .ok();
    assert!(pending.is_some(), "Redis 에 mfa_pending key 저장됨");

    let _: () = redis::AsyncCommands::del(&mut conn, format!("ak:mfa_pending:{}", mfa_token))
        .await
        .unwrap_or(());
    // login 통과 시 rate limit 은 service 내부에서 자동 del 되므로 cleanup 불필요
    cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_login_returns_social_only_for_oauth_only_account() {
    // OAuth 전용 계정 (user_password = NULL) → 401 AUTH_401_SOCIAL_ONLY_ACCOUNT:<providers>.
    let st = common::make_test_state().await;

    let mut spec = TestUserSpec::random();
    spec.oauth_only = true;
    let user_id = insert_test_user(&st, &spec).await;

    let req = LoginReq {
        email: spec.email.clone(),
        password: "AnyPass123".to_string(),
    };
    let ip = "10.0.1.7".to_string();
    let result = AuthService::login(&st, req, ip.clone(), None, parsed_ua_default()).await;
    match result {
        Err(AppError::Unauthorized(msg)) => {
            assert!(
                msg.starts_with("AUTH_401_SOCIAL_ONLY_ACCOUNT:"),
                "AUTH_401_SOCIAL_ONLY_ACCOUNT:<providers> 형식, got: {}",
                msg
            );
        }
        Err(e) => panic!(
            "OAuth-only → Unauthorized(AUTH_401_SOCIAL_ONLY_ACCOUNT:...) expected, got Err: {:?}",
            e
        ),
        Ok(_) => panic!("OAuth-only → Unauthorized expected, got Ok"),
    }

    cleanup_login_rl(&st, &spec.email.to_lowercase(), &ip).await;
    cleanup_test_user(&st, user_id).await;
}

// =============================================================================
// AuthService::mfa_login
// =============================================================================

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_mfa_login_returns_token_expired_for_unknown_token() {
    // 존재하지 않는 mfa_token → MFA_TOKEN_EXPIRED.
    let st = common::make_test_state().await;

    let unique_token = uuid::Uuid::new_v4().to_string();
    let req = MfaLoginReq {
        mfa_token: unique_token,
        code: "123456".to_string(),
    };
    let result = AuthService::mfa_login(&st, req, "10.0.1.10".to_string()).await;
    match result {
        Err(AppError::Unauthorized(msg)) => {
            assert_eq!(msg, "MFA_TOKEN_EXPIRED", "got: {}", msg);
        }
        Err(e) => panic!(
            "unknown mfa_token → MFA_TOKEN_EXPIRED expected, got Err: {:?}",
            e
        ),
        Ok(_) => panic!("unknown mfa_token → MFA_TOKEN_EXPIRED expected, got Ok"),
    }
}
