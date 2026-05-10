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

use amazing_korean_api::api::auth::dto::{LoginReq, LogoutAllReq, MfaLoginReq};
use amazing_korean_api::api::auth::handler::ParsedUa;
use amazing_korean_api::api::auth::service::{AuthService, LoginOutcome};
use amazing_korean_api::crypto::CryptoService;
use amazing_korean_api::error::AppError;
use base64::engine::{general_purpose::URL_SAFE_NO_PAD, Engine};
use common::{
    cleanup_test_user, generate_totp_code, insert_test_user, insert_test_user_with_mfa,
    TestUserSpec,
};

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
async fn test_mfa_login_succeeds_with_valid_totp_code() {
    // 시나리오: mfa_enabled=true user + ak:mfa_pending Redis 시드 + 현재 TOTP 코드.
    // 결과: Ok((LoginRes, Cookie, user_id, refresh_token)) + login + redis_session/refresh
    //       row INSERT (cleanup_test_user 가 일괄 정리).
    let st = common::make_test_state().await;

    let mut spec = TestUserSpec::random();
    spec.mfa_enabled = true;
    let (user_id, secret_base32) = insert_test_user_with_mfa(&st, &spec).await;

    // ak:mfa_pending:<token> 시드 (login 단계가 만들어주는 것을 직접 시뮬레이트)
    let mfa_token = uuid::Uuid::new_v4().to_string();
    let pending_data = serde_json::json!({
        "user_id": user_id,
        "user_auth": "Learner",
        "login_ip": "10.0.1.20",
        "user_agent": null,
        "device": "other",
        "browser": null,
        "os": null,
        "login_method": "email",
    });

    let mut conn = st.redis.get().await.expect("redis conn for seed");
    let _: () = redis::AsyncCommands::set_ex(
        &mut conn,
        format!("ak:mfa_pending:{}", mfa_token),
        pending_data.to_string(),
        st.cfg.mfa_token_ttl_sec as u64,
    )
    .await
    .expect("seed mfa_pending");

    // 현재 시점 valid TOTP 코드
    let code = generate_totp_code(&secret_base32);

    let req = MfaLoginReq {
        mfa_token: mfa_token.clone(),
        code,
    };
    let result = AuthService::mfa_login(&st, req, "10.0.1.20".to_string()).await;
    match result {
        // 반환 = (LoginRes, Cookie, ttl_sec, refresh_token). user_id 는 LoginRes 내부에 있지만
        // public 필드 접근 미보장 → refresh_token 길이만 검증 (성공 path = 비어있지 않음).
        Ok((_login_res, _cookie, _ttl, refresh_token)) => {
            assert!(!refresh_token.is_empty(), "refresh_token 비어있지 않음");
        }
        Err(e) => panic!("valid TOTP → Ok expected, got Err: {:?}", e),
    }

    // Cleanup: mfa_pending 은 service 가 즉시 del (1회용). rate limit 도 service 가 코드 검증
    // 성공 시 del. 따라서 별도 정리 불필요. user + login + redis_session/refresh row 만 정리.
    cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_mfa_login_rejects_invalid_totp_code() {
    // 시나리오: mfa_enabled=true user + Redis pending 시드 + 잘못된 6자리 코드.
    // 결과: MFA_INVALID_CODE (백업 코드 시도 후 fail).
    let st = common::make_test_state().await;

    let mut spec = TestUserSpec::random();
    spec.mfa_enabled = true;
    let (user_id, _secret) = insert_test_user_with_mfa(&st, &spec).await;

    let mfa_token = uuid::Uuid::new_v4().to_string();
    let pending_data = serde_json::json!({
        "user_id": user_id,
        "user_auth": "Learner",
        "login_ip": "10.0.1.21",
        "user_agent": null,
        "device": "other",
        "browser": null,
        "os": null,
        "login_method": "email",
    });

    let mut conn = st.redis.get().await.expect("redis conn for seed");
    let _: () = redis::AsyncCommands::set_ex(
        &mut conn,
        format!("ak:mfa_pending:{}", mfa_token),
        pending_data.to_string(),
        st.cfg.mfa_token_ttl_sec as u64,
    )
    .await
    .expect("seed mfa_pending");

    // 잘못된 코드 (6자리 숫자, but TOTP 미일치 + 백업 코드 fail)
    let req = MfaLoginReq {
        mfa_token: mfa_token.clone(),
        code: "000000".to_string(),
    };
    let result = AuthService::mfa_login(&st, req, "10.0.1.21".to_string()).await;
    match result {
        Err(AppError::Unauthorized(msg)) => {
            assert_eq!(msg, "MFA_INVALID_CODE", "got: {}", msg);
        }
        Err(e) => panic!("invalid TOTP → MFA_INVALID_CODE expected, got Err: {:?}", e),
        Ok(_) => panic!("invalid TOTP → MFA_INVALID_CODE expected, got Ok"),
    }

    // Cleanup
    let _: () = redis::AsyncCommands::del(&mut conn, format!("ak:mfa_pending:{}", mfa_token))
        .await
        .unwrap_or(());
    let _: () = redis::AsyncCommands::del(&mut conn, format!("rl:mfa:{}:10.0.1.21", user_id))
        .await
        .unwrap_or(());
    cleanup_test_user(&st, user_id).await;
}

// =============================================================================
// MFA setup / verify_setup / disable (A4)
// =============================================================================

async fn fetch_user_email_enc(st: &amazing_korean_api::state::AppState, user_id: i64) -> String {
    sqlx::query_scalar::<_, String>("SELECT user_email FROM users WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(&st.db)
        .await
        .expect("fetch user_email_enc")
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_mfa_setup_generates_secret_and_qr_for_new_user() {
    // mfa_enabled=false user → mfa_setup → secret + qr + otpauth_uri 반환.
    let st = common::make_test_state().await;

    let spec = TestUserSpec::random(); // mfa_enabled=false (default)
    let user_id = insert_test_user(&st, &spec).await;
    let email_enc = fetch_user_email_enc(&st, user_id).await;

    let result = AuthService::mfa_setup(&st, user_id, &email_enc).await;
    let res = match result {
        Ok(r) => r,
        Err(e) => panic!("mfa_setup → Ok expected, got Err: {:?}", e),
    };

    assert!(!res.secret.is_empty(), "base32 secret 비어있지 않음");
    assert!(
        res.qr_code_data_uri.starts_with("data:image/png;base64,"),
        "QR data URI prefix, got: {}",
        &res.qr_code_data_uri[..50.min(res.qr_code_data_uri.len())]
    );
    assert!(
        res.otpauth_uri.starts_with("otpauth://totp/"),
        "otpauth URI prefix, got: {}",
        res.otpauth_uri
    );

    // Cleanup: rate limit + user
    let mut conn = st.redis.get().await.expect("redis");
    let _: () = redis::AsyncCommands::del(&mut conn, format!("rl:mfa_setup:{}", user_id))
        .await
        .unwrap_or(());
    cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_mfa_setup_returns_conflict_when_already_enabled() {
    // mfa_enabled=true user → mfa_setup → Conflict("MFA_ALREADY_ENABLED").
    let st = common::make_test_state().await;

    let mut spec = TestUserSpec::random();
    spec.mfa_enabled = true;
    let user_id = insert_test_user(&st, &spec).await;
    let email_enc = fetch_user_email_enc(&st, user_id).await;

    let result = AuthService::mfa_setup(&st, user_id, &email_enc).await;
    match result {
        Err(AppError::Conflict(msg)) => {
            assert_eq!(msg, "MFA_ALREADY_ENABLED", "got: {}", msg);
        }
        Err(e) => panic!("already enabled → Conflict expected, got Err: {:?}", e),
        Ok(_) => panic!("already enabled → Conflict expected, got Ok"),
    }

    // Cleanup
    let mut conn = st.redis.get().await.expect("redis");
    let _: () = redis::AsyncCommands::del(&mut conn, format!("rl:mfa_setup:{}", user_id))
        .await
        .unwrap_or(());
    cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_mfa_verify_setup_succeeds_with_valid_code_and_returns_backup_codes() {
    // Phase 1: mfa_setup → secret 획득
    // Phase 2: 그 secret 으로 현재 TOTP 코드 생성 → mfa_verify_setup → enabled + 10 backup codes
    let st = common::make_test_state().await;

    let spec = TestUserSpec::random();
    let user_id = insert_test_user(&st, &spec).await;
    let email_enc = fetch_user_email_enc(&st, user_id).await;

    let setup_res = AuthService::mfa_setup(&st, user_id, &email_enc)
        .await
        .expect("mfa_setup 성공");
    let valid_code = generate_totp_code(&setup_res.secret);

    let result = AuthService::mfa_verify_setup(&st, user_id, &valid_code).await;
    let verify_res = match result {
        Ok(r) => r,
        Err(e) => panic!("mfa_verify_setup → Ok expected, got Err: {:?}", e),
    };

    assert!(verify_res.enabled, "enabled=true");
    assert_eq!(
        verify_res.backup_codes.len(),
        10,
        "backup_codes 10개, got: {}",
        verify_res.backup_codes.len()
    );
    for code in &verify_res.backup_codes {
        assert_eq!(
            code.len(),
            8,
            "각 backup code 8자, got: {} ({})",
            code,
            code.len()
        );
    }

    // Cleanup
    let mut conn = st.redis.get().await.expect("redis");
    let _: () = redis::AsyncCommands::del(&mut conn, format!("rl:mfa_setup:{}", user_id))
        .await
        .unwrap_or(());
    let _: () = redis::AsyncCommands::del(&mut conn, format!("rl:mfa_verify_setup:{}", user_id))
        .await
        .unwrap_or(());
    cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_mfa_verify_setup_rejects_invalid_code() {
    // mfa_setup 후 잘못된 TOTP 코드 → MFA_INVALID_CODE.
    let st = common::make_test_state().await;

    let spec = TestUserSpec::random();
    let user_id = insert_test_user(&st, &spec).await;
    let email_enc = fetch_user_email_enc(&st, user_id).await;

    AuthService::mfa_setup(&st, user_id, &email_enc)
        .await
        .expect("mfa_setup 성공");

    let result = AuthService::mfa_verify_setup(&st, user_id, "000000").await;
    match result {
        Err(AppError::Unauthorized(msg)) => {
            assert_eq!(msg, "MFA_INVALID_CODE", "got: {}", msg);
        }
        Err(e) => panic!("invalid code → MFA_INVALID_CODE expected, got Err: {:?}", e),
        Ok(_) => panic!("invalid code → Err expected, got Ok"),
    }

    // Cleanup
    let mut conn = st.redis.get().await.expect("redis");
    let _: () = redis::AsyncCommands::del(&mut conn, format!("rl:mfa_setup:{}", user_id))
        .await
        .unwrap_or(());
    let _: () = redis::AsyncCommands::del(&mut conn, format!("rl:mfa_verify_setup:{}", user_id))
        .await
        .unwrap_or(());
    cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_mfa_verify_setup_rejects_when_setup_not_started() {
    // mfa_setup 호출 없이 verify_setup → BadRequest("MFA_SETUP_NOT_STARTED").
    let st = common::make_test_state().await;

    let spec = TestUserSpec::random(); // user_mfa_secret = NULL
    let user_id = insert_test_user(&st, &spec).await;

    let result = AuthService::mfa_verify_setup(&st, user_id, "123456").await;
    match result {
        Err(AppError::BadRequest(msg)) => {
            assert_eq!(msg, "MFA_SETUP_NOT_STARTED", "got: {}", msg);
        }
        Err(e) => panic!("setup not started → BadRequest expected, got Err: {:?}", e),
        Ok(_) => panic!("setup not started → Err expected, got Ok"),
    }

    // Cleanup
    let mut conn = st.redis.get().await.expect("redis");
    let _: () = redis::AsyncCommands::del(&mut conn, format!("rl:mfa_verify_setup:{}", user_id))
        .await
        .unwrap_or(());
    cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_mfa_disable_rejects_non_hymn_auth() {
    // Learner 가 호출 → Forbidden("MFA_DISABLE_HYMN_ONLY").
    let st = common::make_test_state().await;

    let auth_user_id = 99999; // 가짜 — 권한 체크가 먼저 차단
    let target_user_id = 99998;

    let result = AuthService::mfa_disable(
        &st,
        auth_user_id,
        amazing_korean_api::types::UserAuth::Learner,
        target_user_id,
    )
    .await;
    match result {
        Err(AppError::Forbidden(msg)) => {
            assert_eq!(msg, "MFA_DISABLE_HYMN_ONLY", "got: {}", msg);
        }
        Err(e) => panic!("non-HYMN → Forbidden expected, got Err: {:?}", e),
        Ok(_) => panic!("non-HYMN → Err expected, got Ok"),
    }

    // Cleanup rate limit
    let mut conn = st.redis.get().await.expect("redis");
    let _: () = redis::AsyncCommands::del(&mut conn, format!("rl:mfa_disable:{}", auth_user_id))
        .await
        .unwrap_or(());
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_mfa_disable_rejects_self_disable() {
    // HYMN 인 user 가 자기 자신을 target → BadRequest("MFA_CANNOT_DISABLE_SELF").
    let st = common::make_test_state().await;

    let auth_user_id = 88888;
    let result = AuthService::mfa_disable(
        &st,
        auth_user_id,
        amazing_korean_api::types::UserAuth::Hymn,
        auth_user_id, // self
    )
    .await;
    match result {
        Err(AppError::BadRequest(msg)) => {
            assert_eq!(msg, "MFA_CANNOT_DISABLE_SELF", "got: {}", msg);
        }
        Err(e) => panic!("self-disable → BadRequest expected, got Err: {:?}", e),
        Ok(_) => panic!("self-disable → Err expected, got Ok"),
    }

    let mut conn = st.redis.get().await.expect("redis");
    let _: () = redis::AsyncCommands::del(&mut conn, format!("rl:mfa_disable:{}", auth_user_id))
        .await
        .unwrap_or(());
}

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

// =============================================================================
// refresh / logout / logout_all (A5)
// =============================================================================

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_refresh_rejects_malformed_token() {
    // base64url 디코드 실패 → AUTH_401_INVALID_REFRESH (parse_refresh_token 첫 단계).
    let st = common::make_test_state().await;

    let result = AuthService::refresh(&st, "not!base64@@@", "10.0.6.1".into(), None).await;
    match result {
        Err(AppError::Unauthorized(msg)) => {
            assert_eq!(msg, "AUTH_401_INVALID_REFRESH", "got: {}", msg);
        }
        Err(e) => panic!(
            "malformed → AUTH_401_INVALID_REFRESH expected, got Err: {:?}",
            e
        ),
        Ok(_) => panic!("malformed → Err expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_refresh_rejects_unknown_session_id() {
    // 형식 OK 한 token (base64url("<session>:<random>")) 이지만 DB session 없음
    //   → AUTH_401_INVALID_REFRESH (find_login_by_session_id_for_update_tx None).
    let st = common::make_test_state().await;

    let unknown_session = uuid::Uuid::new_v4().to_string();
    let raw = format!("{}:phase3_random_part", unknown_session);
    let token = URL_SAFE_NO_PAD.encode(raw.as_bytes());

    let result = AuthService::refresh(&st, &token, "10.0.6.2".into(), None).await;
    match result {
        Err(AppError::Unauthorized(msg)) => {
            assert_eq!(msg, "AUTH_401_INVALID_REFRESH", "got: {}", msg);
        }
        Err(e) => panic!(
            "unknown session → AUTH_401_INVALID_REFRESH expected, got Err: {:?}",
            e
        ),
        Ok(_) => panic!("unknown session → Err expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_refresh_rejects_empty_token_parts() {
    // base64url 디코드는 OK 한데 splitn(":") 후 빈 part → AUTH_401_INVALID_REFRESH.
    let st = common::make_test_state().await;

    // ":random" = session_id 빈 문자열
    let token = URL_SAFE_NO_PAD.encode(b":only_random");
    let result = AuthService::refresh(&st, &token, "10.0.6.3".into(), None).await;
    match result {
        Err(AppError::Unauthorized(msg)) => {
            assert_eq!(msg, "AUTH_401_INVALID_REFRESH", "got: {}", msg);
        }
        Err(e) => panic!(
            "empty session_id → AUTH_401_INVALID_REFRESH expected, got Err: {:?}",
            e
        ),
        Ok(_) => panic!("empty → Err expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_logout_succeeds_for_unknown_session_as_noop() {
    // 존재하지 않는 session_id (UUID 형식) 로 logout → Ok (no-op).
    // login.login_session_id 컬럼이 UUID 타입 = 잘못된 형식 string 은 22P02 sqlx 에러.
    // 본 테스트 = UUID 형식 사용, login_record None 이면 DB update / login_log skip, Redis del = no-op.
    let st = common::make_test_state().await;

    let unknown_session = uuid::Uuid::new_v4().to_string();
    let result = AuthService::logout(&st, 99999, &unknown_session, "10.0.6.4".into(), None).await;
    assert!(
        result.is_ok(),
        "unknown session logout → Ok (no-op), got: {:?}",
        result
    );
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_logout_all_without_refresh_token_returns_unauthorized() {
    // refresh_token=None → user_id 미식별 → AUTH_401_INVALID_REFRESH (signed-out 으로 처리하지 않음).
    let st = common::make_test_state().await;

    let result = AuthService::logout_all(
        &st,
        None,
        LogoutAllReq { everywhere: true },
        "10.0.6.5".into(),
        None,
    )
    .await;
    match result {
        Err(AppError::Unauthorized(msg)) => {
            assert_eq!(msg, "AUTH_401_INVALID_REFRESH", "got: {}", msg);
        }
        Err(e) => panic!(
            "no refresh_token → AUTH_401_INVALID_REFRESH expected, got Err: {:?}",
            e
        ),
        Ok(_) => panic!("no refresh_token → Err expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_logout_all_with_invalid_refresh_token_returns_unauthorized() {
    // 잘못된 refresh_token (base64 깨짐) → hash 계산 fail or session 미존재 → user_id 미식별
    //   → AUTH_401_INVALID_REFRESH.
    let st = common::make_test_state().await;

    let result = AuthService::logout_all(
        &st,
        Some("not!base64!"),
        LogoutAllReq { everywhere: true },
        "10.0.6.6".into(),
        None,
    )
    .await;
    match result {
        Err(AppError::Unauthorized(msg)) => {
            assert_eq!(msg, "AUTH_401_INVALID_REFRESH", "got: {}", msg);
        }
        Err(e) => panic!(
            "invalid token → AUTH_401_INVALID_REFRESH expected, got Err: {:?}",
            e
        ),
        Ok(_) => panic!("invalid token → Err expected, got Ok"),
    }
}
