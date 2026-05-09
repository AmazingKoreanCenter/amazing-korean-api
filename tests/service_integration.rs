//! Phase 2 통합 테스트 — service.rs Redis 의존 함수.
//!
//! ## 셋업
//!
//! `tests/common/mod.rs` 의 `make_test_state()` 사용. 환경변수 = `.env.test` 또는 `.env`.
//!
//! ## 실행
//!
//! ```bash
//! DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/amazing_korean_db \
//! REDIS_URL=redis://:redis_dev_password@127.0.0.1:16379 \
//! JWT_SECRET=test_jwt_secret_must_be_at_least_32_bytes \
//! HMAC_KEY=$(openssl rand -base64 32) \
//! ENCRYPTION_KEY_V1=$(openssl rand -base64 32) \
//!   cargo test --test service_integration -- --ignored
//! ```
//!
//! 또는 `.env.test` 파일 작성 후 `cargo test --test service_integration -- --ignored`.
//!
//! ## 범위 (Phase 2 첫 진입)
//!
//! Redis 의존이 가장 단순한 함수 = `verify_email`. negative cases = email/payment/외부 API 의존 0.
//! signup-verify positive flow 는 EmailSender mock 후 (Phase 3) 검증.

mod common;

use amazing_korean_api::api::auth::dto::VerifyEmailReq;
use amazing_korean_api::api::auth::service::AuthService;
use amazing_korean_api::error::AppError;

// =============================================================================
// AuthService::verify_email — Redis 의존, EmailSender 미사용
// =============================================================================

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 2 보류 정책)"]
#[tokio::test]
async fn test_verify_email_returns_unauthorized_for_missing_code() {
    let st = common::make_test_state().await;

    // Redis 에 코드 저장 안 함 → ak:email_verify:* key 없음 → Unauthorized
    let req = VerifyEmailReq {
        email: format!("phase2_test_{}@example.com", uuid::Uuid::new_v4()),
        code: "000000".to_string(),
    };

    let result = AuthService::verify_email(&st, req, "127.0.0.1".to_string()).await;
    assert!(
        matches!(result, Err(AppError::Unauthorized(ref msg)) if msg == "AUTH_401_INVALID_OR_EXPIRED_CODE"),
        "stored hash 없음 → AUTH_401_INVALID_OR_EXPIRED_CODE, got: {:?}",
        result
    );
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 2 보류 정책)"]
#[tokio::test]
async fn test_verify_email_rate_limit_increments() {
    // verify_email 의 rate limit = 동일 (email_idx, ip) 조합 1시간 내 10회 초과 시 429.
    // 본 테스트 = rate limit 11번째 호출 시 TooManyRequests 검증.
    let st = common::make_test_state().await;

    let unique_email = format!("rl_test_{}@example.com", uuid::Uuid::new_v4());
    let ip = "10.0.0.42".to_string();

    // 11번 호출. 처음 10번 = Unauthorized (코드 없음), 11번째 = TooManyRequests
    let mut last_result = None;
    for _ in 0..11 {
        let req = VerifyEmailReq {
            email: unique_email.clone(),
            code: "000000".to_string(),
        };
        last_result = Some(AuthService::verify_email(&st, req, ip.clone()).await);
    }

    let result = last_result.expect("11 호출 결과 있어야 함");
    assert!(
        matches!(result, Err(AppError::TooManyRequests(_))),
        "11번째 호출 = TooManyRequests, got: {:?}",
        result
    );

    // Cleanup: rate limit key 삭제 (다른 테스트 격리)
    let mut conn = st.redis.get().await.expect("redis conn for cleanup");
    let crypto =
        amazing_korean_api::crypto::CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
    let email_idx = crypto.blind_index(&unique_email).expect("blind_index");
    let _: () =
        redis::AsyncCommands::del(&mut conn, format!("rl:verify_email:{}:{}", email_idx, ip))
            .await
            .unwrap_or(());
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 2 보류 정책)"]
#[tokio::test]
async fn test_verify_email_validation_rejects_short_code() {
    // VerifyEmailReq 의 code = length(equal = 6) 검증. 5자리 = ValidationGeneric.
    let st = common::make_test_state().await;

    let req = VerifyEmailReq {
        email: "validate_test@example.com".to_string(),
        code: "12345".to_string(), // 5 chars (not 6)
    };

    let result = AuthService::verify_email(&st, req, "127.0.0.1".to_string()).await;
    assert!(
        matches!(result, Err(AppError::ValidationGeneric)),
        "5자리 code = ValidationGeneric, got: {:?}",
        result
    );
}

// =============================================================================
// AuthService::verify_reset_code — Redis 의존, EmailSender 미사용
// =============================================================================

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 2 보류 정책)"]
#[tokio::test]
async fn test_verify_reset_code_returns_unauthorized_for_missing_code() {
    let st = common::make_test_state().await;

    let unique_email = format!("reset_test_{}@example.com", uuid::Uuid::new_v4());
    let result =
        AuthService::verify_reset_code(&st, &unique_email, "000000", "127.0.0.1".into()).await;
    assert!(
        matches!(result, Err(AppError::Unauthorized(ref msg)) if msg == "AUTH_401_INVALID_OR_EXPIRED_CODE"),
        "stored hash 없음 → AUTH_401_INVALID_OR_EXPIRED_CODE, got: {:?}",
        result
    );
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 2 보류 정책)"]
#[tokio::test]
async fn test_verify_reset_code_rate_limit_increments() {
    // 동일 (email_idx, ip) 11번 호출 → 11번째 = TooManyRequests
    let st = common::make_test_state().await;

    let unique_email = format!("reset_rl_{}@example.com", uuid::Uuid::new_v4());
    let ip = "10.0.0.43".to_string();

    let mut last_result = None;
    for _ in 0..11 {
        last_result =
            Some(AuthService::verify_reset_code(&st, &unique_email, "000000", ip.clone()).await);
    }

    let result = last_result.expect("11 호출 결과 있어야 함");
    assert!(
        matches!(result, Err(AppError::TooManyRequests(_))),
        "11번째 호출 = TooManyRequests, got: {:?}",
        result
    );

    // Cleanup
    let mut conn = st.redis.get().await.expect("redis conn for cleanup");
    let crypto =
        amazing_korean_api::crypto::CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
    let email_idx = crypto.blind_index(&unique_email).expect("blind_index");
    let _: () =
        redis::AsyncCommands::del(&mut conn, format!("rl:verify_reset:{}:{}", email_idx, ip))
            .await
            .unwrap_or(());
}

// =============================================================================
// AuthService::reset_password_with_token — Redis 의존, EmailSender 미사용
// =============================================================================

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 2 보류 정책)"]
#[tokio::test]
async fn test_reset_password_rejects_weak_password() {
    // password policy (8+chars + letter + digit) 위반 → 즉시 Unprocessable, Redis hit 없음
    let st = common::make_test_state().await;

    let result = AuthService::reset_password_with_token(
        &st,
        "ak_reset_dummy",
        "weak", // 4 chars only, 정책 위반
        "127.0.0.1".to_string(),
    )
    .await;

    match &result {
        Err(AppError::Unprocessable(msg)) => {
            assert_eq!(msg, "AUTH_422_PASSWORD_POLICY_VIOLATION");
        }
        _ => panic!("weak password → Unprocessable expected"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 2 보류 정책)"]
#[tokio::test]
async fn test_reset_password_rejects_unknown_redis_token() {
    // ak_reset_ prefix + Redis 조회 None → AUTH_401_INVALID_OR_EXPIRED_TOKEN
    let st = common::make_test_state().await;

    let unknown_token = format!("ak_reset_{}", uuid::Uuid::new_v4());
    let result = AuthService::reset_password_with_token(
        &st,
        &unknown_token,
        "ValidPass1", // 정책 통과 (8+chars + letter + digit)
        "10.0.0.44".to_string(),
    )
    .await;

    match &result {
        Err(AppError::Unauthorized(msg)) => {
            assert_eq!(msg, "AUTH_401_INVALID_OR_EXPIRED_TOKEN");
        }
        _ => panic!("unknown ak_reset_ token → AUTH_401_INVALID_OR_EXPIRED_TOKEN expected"),
    }

    // Cleanup rate limit key
    let mut conn = st.redis.get().await.expect("redis conn for cleanup");
    let _: () = redis::AsyncCommands::del(&mut conn, "rl:reset_pw:10.0.0.44")
        .await
        .unwrap_or(());
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 2 보류 정책)"]
#[tokio::test]
async fn test_reset_password_rejects_invalid_jwt_token() {
    // non-"ak_reset_" prefix = JWT decode 시도 → 잘못된 JWT → AUTH_401_INVALID_RESET_TOKEN
    let st = common::make_test_state().await;

    let result = AuthService::reset_password_with_token(
        &st,
        "not.a.valid.jwt",
        "ValidPass1",
        "10.0.0.45".to_string(),
    )
    .await;

    match &result {
        Err(AppError::Unauthorized(msg)) => {
            assert_eq!(msg, "AUTH_401_INVALID_RESET_TOKEN");
        }
        _ => panic!("invalid JWT → AUTH_401_INVALID_RESET_TOKEN expected"),
    }

    // Cleanup rate limit key
    let mut conn = st.redis.get().await.expect("redis conn for cleanup");
    let _: () = redis::AsyncCommands::del(&mut conn, "rl:reset_pw:10.0.0.45")
        .await
        .unwrap_or(());
}
