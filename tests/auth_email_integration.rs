//! Phase 3 통합 테스트 — EmailSender mock 의존 path.
//!
//! ## 셋업
//!
//! `tests/common/mod.rs` 의 `make_test_state_with_capturing_email()` 사용.
//! 환경변수 = `.env.test` 또는 `.env`.
//!
//! ## 실행
//!
//! ```bash
//! DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/amazing_korean_db \
//! REDIS_URL=redis://:redis_dev_password@127.0.0.1:16379 \
//! JWT_SECRET=test_jwt_secret_must_be_at_least_32_bytes \
//! HMAC_KEY=$(openssl rand -base64 32) \
//! ENCRYPTION_KEY_V1=$(openssl rand -base64 32) \
//!   cargo test --test auth_email_integration -- --ignored
//! ```
//!
//! 또는 `.env.test` 파일 작성 후 `cargo test --test auth_email_integration -- --ignored`.
//!
//! ## 범위 (Phase 3 첫 진입)
//!
//! EmailSender mock (`CapturingEmailSender`) 으로 다음 검증:
//! - anti-enumeration: 존재하지 않는 유저 → generic 200 응답 + 이메일 발송 0건
//! - validation: DTO validate 실패 → ValidationGeneric (rate limit INCR 전 차단)
//! - rate limit: 11번째 호출 → TooManyRequests
//!
//! happy path (실제 유저 + 이메일 캡처 검증) = 별도 테스트 (DB user 생성 필요).

mod common;

use amazing_korean_api::api::auth::dto::{FindPasswordReq, ResendVerificationReq};
use amazing_korean_api::api::auth::service::AuthService;
use amazing_korean_api::crypto::CryptoService;
use amazing_korean_api::error::AppError;
use common::{cleanup_test_user, insert_test_user, TestUserSpec};

// =============================================================================
// AuthService::request_password_reset — 이메일 발송 path
// =============================================================================

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_request_password_reset_anti_enumeration_for_non_existent_user() {
    // 존재하지 않는 이메일 → generic 200 응답 + email 발송 0건 (anti-enumeration).
    let (st, sent) = common::make_test_state_with_capturing_email().await;

    let unique_email = format!("phase3_noexist_{}@example.com", uuid::Uuid::new_v4());
    let result =
        AuthService::request_password_reset(&st, &unique_email, "10.0.0.50".to_string()).await;

    assert!(
        result.is_ok(),
        "non-existent user → generic Ok, got: {:?}",
        result
    );
    let captured = sent.lock().await;
    assert_eq!(
        captured.len(),
        0,
        "non-existent user → 이메일 발송 0건 (anti-enumeration), got: {}",
        captured.len()
    );
    drop(captured);

    // Cleanup: rate limit key
    let mut conn = st.redis.get().await.expect("redis conn for cleanup");
    let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
    let idx = crypto.blind_index(&unique_email).expect("blind_index");
    let _: () = redis::AsyncCommands::del(&mut conn, format!("rl:request_reset:{}:10.0.0.50", idx))
        .await
        .unwrap_or(());
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_request_password_reset_rate_limit_429() {
    // 11번째 호출 = TooManyRequests. 존재하지 않는 유저로도 검증 가능 (Step 1 INCR 가 Step 2 검색 전).
    let (st, sent) = common::make_test_state_with_capturing_email().await;

    let unique_email = format!("phase3_rl_{}@example.com", uuid::Uuid::new_v4());
    let ip = "10.0.0.51".to_string();

    let mut last_result = None;
    for _ in 0..11 {
        last_result =
            Some(AuthService::request_password_reset(&st, &unique_email, ip.clone()).await);
    }

    let result = last_result.expect("11 호출 결과 있어야 함");
    assert!(
        matches!(result, Err(AppError::TooManyRequests(_))),
        "11번째 호출 = TooManyRequests, got: {:?}",
        result
    );

    // anti-enumeration 동시 검증 — 11회 시도 모두 이메일 0건 (존재하지 않는 user)
    let captured = sent.lock().await;
    assert_eq!(
        captured.len(),
        0,
        "11회 시도 = 이메일 0건, got: {}",
        captured.len()
    );
    drop(captured);

    // Cleanup
    let mut conn = st.redis.get().await.expect("redis conn for cleanup");
    let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
    let idx = crypto.blind_index(&unique_email).expect("blind_index");
    let _: () = redis::AsyncCommands::del(&mut conn, format!("rl:request_reset:{}:{}", idx, ip))
        .await
        .unwrap_or(());
}

// =============================================================================
// AuthService::resend_verification — 이메일 발송 path
// =============================================================================

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_resend_verification_anti_enumeration_for_non_existent_user() {
    // 존재하지 않는 이메일 → generic 200 응답 + email 발송 0건.
    let (st, sent) = common::make_test_state_with_capturing_email().await;

    let unique_email = format!("phase3_resend_{}@example.com", uuid::Uuid::new_v4());
    let req = ResendVerificationReq {
        email: unique_email.clone(),
    };

    let result = AuthService::resend_verification(&st, req, "10.0.0.52".to_string()).await;

    assert!(result.is_ok(), "non-existent user → Ok, got: {:?}", result);
    let captured = sent.lock().await;
    assert_eq!(
        captured.len(),
        0,
        "non-existent user → 이메일 발송 0건, got: {}",
        captured.len()
    );
    drop(captured);

    // Cleanup
    let mut conn = st.redis.get().await.expect("redis conn for cleanup");
    let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
    let idx = crypto.blind_index(&unique_email).expect("blind_index");
    let _: () = redis::AsyncCommands::del(&mut conn, format!("rl:resend_verify:{}:10.0.0.52", idx))
        .await
        .unwrap_or(());
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_resend_verification_validation_rejects_invalid_email() {
    // ResendVerificationReq.email = #[validate(email)]. 형식 위반 → ValidationGeneric.
    let (st, sent) = common::make_test_state_with_capturing_email().await;

    let req = ResendVerificationReq {
        email: "not-an-email-format".to_string(),
    };

    let result = AuthService::resend_verification(&st, req, "10.0.0.53".to_string()).await;
    assert!(
        matches!(result, Err(AppError::ValidationGeneric)),
        "잘못된 email 형식 → ValidationGeneric, got: {:?}",
        result
    );

    // 검증 실패 = rate limit INCR 전 차단 = 이메일 발송 0건
    let captured = sent.lock().await;
    assert_eq!(captured.len(), 0, "validation 실패 시 이메일 0건");
}

// =============================================================================
// AuthService::find_password — 이메일 발송 path
// =============================================================================

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_find_password_validation_rejects_invalid_birthday() {
    // FindPasswordReq.birthday = #[validate(custom = validate_birthday)]. 형식 위반 → ValidationGeneric.
    let (st, sent) = common::make_test_state_with_capturing_email().await;

    let req = FindPasswordReq {
        name: "테스트".to_string(),
        birthday: "not-a-date".to_string(),
        email: "test@example.com".to_string(),
    };

    let result = AuthService::find_password(&st, req, "10.0.0.54".to_string()).await;
    assert!(
        matches!(result, Err(AppError::ValidationGeneric)),
        "잘못된 birthday → ValidationGeneric, got: {:?}",
        result
    );

    let captured = sent.lock().await;
    assert_eq!(captured.len(), 0, "validation 실패 시 이메일 0건");
}

// =============================================================================
// happy path — 실제 user INSERT + 이메일 캡처 검증
// =============================================================================

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_request_password_reset_sends_email_for_existing_user() {
    // 존재하는 user (password 있음, check_email=true) → 이메일 1건 캡처.
    // 캡처 내용 검증: 본인에게 발송 + 6자리 숫자 코드 + 비밀번호 재설정 subject.
    let (st, sent) = common::make_test_state_with_capturing_email().await;

    let spec = TestUserSpec::random();
    let user_id = insert_test_user(&st, &spec).await;

    let result =
        AuthService::request_password_reset(&st, &spec.email, "10.0.0.60".to_string()).await;
    assert!(result.is_ok(), "happy path → Ok, got: {:?}", result);

    let captured = sent.lock().await;
    assert_eq!(
        captured.len(),
        1,
        "이메일 1건 발송, got: {}",
        captured.len()
    );
    let mail = &captured[0];
    assert_eq!(
        mail.to,
        spec.email.to_lowercase(),
        "발송 대상 = 입력 email (lowercase)"
    );
    assert!(
        mail.subject.contains("비밀번호 재설정"),
        "subject 에 '비밀번호 재설정' 포함, got: {}",
        mail.subject
    );
    // text body 에 6자리 숫자 코드 (Verify code 가 generate_verification_code() = 100000~999999)
    let has_six_digit = mail
        .text
        .chars()
        .collect::<String>()
        .split(|c: char| !c.is_ascii_digit())
        .any(|s| s.len() == 6);
    assert!(
        has_six_digit,
        "text body 에 6자리 숫자 코드 포함, got: {}",
        mail.text
    );
    drop(captured);

    // Cleanup
    let mut conn = st.redis.get().await.expect("redis conn for cleanup");
    let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
    let idx = crypto
        .blind_index(&spec.email.to_lowercase())
        .expect("blind_index");
    let _: () = redis::AsyncCommands::del(&mut conn, format!("rl:request_reset:{}:10.0.0.60", idx))
        .await
        .unwrap_or(());
    let _: () = redis::AsyncCommands::del(&mut conn, format!("ak:reset_code:{}", idx))
        .await
        .unwrap_or(());
    cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_resend_verification_sends_email_for_unverified_user() {
    // 존재하는 user + check_email=false → 이메일 1건 캡처.
    let (st, sent) = common::make_test_state_with_capturing_email().await;

    let mut spec = TestUserSpec::random();
    spec.check_email = false; // 미인증 상태
    let user_id = insert_test_user(&st, &spec).await;

    let req = ResendVerificationReq {
        email: spec.email.clone(),
    };
    let result = AuthService::resend_verification(&st, req, "10.0.0.61".to_string()).await;
    assert!(result.is_ok(), "happy path → Ok, got: {:?}", result);

    let captured = sent.lock().await;
    assert_eq!(
        captured.len(),
        1,
        "미인증 user → 이메일 1건, got: {}",
        captured.len()
    );
    let mail = &captured[0];
    assert_eq!(mail.to, spec.email.to_lowercase());
    assert!(
        mail.subject.contains("이메일 인증"),
        "subject 에 '이메일 인증' 포함, got: {}",
        mail.subject
    );
    drop(captured);

    // Cleanup
    let mut conn = st.redis.get().await.expect("redis conn for cleanup");
    let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
    let idx = crypto
        .blind_index(&spec.email.to_lowercase())
        .expect("blind_index");
    let _: () = redis::AsyncCommands::del(&mut conn, format!("rl:resend_verify:{}:10.0.0.61", idx))
        .await
        .unwrap_or(());
    let _: () = redis::AsyncCommands::del(&mut conn, format!("ak:email_verify:{}", idx))
        .await
        .unwrap_or(());
    cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_resend_verification_no_email_for_already_verified_user() {
    // 존재하는 user + check_email=true (이미 인증됨) → generic 200 + 이메일 0건 (anti-enum).
    let (st, sent) = common::make_test_state_with_capturing_email().await;

    let spec = TestUserSpec::random(); // check_email=true (default)
    let user_id = insert_test_user(&st, &spec).await;

    let req = ResendVerificationReq {
        email: spec.email.clone(),
    };
    let result = AuthService::resend_verification(&st, req, "10.0.0.62".to_string()).await;
    assert!(
        result.is_ok(),
        "이미 인증된 user → generic Ok, got: {:?}",
        result
    );

    let captured = sent.lock().await;
    assert_eq!(
        captured.len(),
        0,
        "이미 인증된 user → 이메일 0건 (anti-enum), got: {}",
        captured.len()
    );
    drop(captured);

    // Cleanup
    let mut conn = st.redis.get().await.expect("redis conn for cleanup");
    let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
    let idx = crypto
        .blind_index(&spec.email.to_lowercase())
        .expect("blind_index");
    let _: () = redis::AsyncCommands::del(&mut conn, format!("rl:resend_verify:{}:10.0.0.62", idx))
        .await
        .unwrap_or(());
    cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_find_password_sends_email_for_matching_user() {
    // (name, birthday, email) 모두 일치 → 이메일 1건 캡처.
    let (st, sent) = common::make_test_state_with_capturing_email().await;

    let spec = TestUserSpec::random();
    let user_id = insert_test_user(&st, &spec).await;

    let req = FindPasswordReq {
        name: spec.name.clone(),
        birthday: spec.birthday.clone(),
        email: spec.email.clone(),
    };
    let ip = "10.0.0.63".to_string();

    let result = AuthService::find_password(&st, req, ip.clone()).await;
    assert!(result.is_ok(), "matching user → Ok, got: {:?}", result);

    let captured = sent.lock().await;
    assert_eq!(
        captured.len(),
        1,
        "matching user → 이메일 1건, got: {}",
        captured.len()
    );
    let mail = &captured[0];
    assert_eq!(mail.to, spec.email.to_lowercase());
    assert!(
        mail.subject.contains("비밀번호 재설정"),
        "subject 에 '비밀번호 재설정' 포함, got: {}",
        mail.subject
    );
    drop(captured);

    // Cleanup
    let mut conn = st.redis.get().await.expect("redis conn for cleanup");
    let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
    let idx = crypto
        .blind_index(&spec.email.to_lowercase())
        .expect("blind_index");
    let _: () = redis::AsyncCommands::del(&mut conn, format!("rl:find_password:{}", ip))
        .await
        .unwrap_or(());
    let _: () = redis::AsyncCommands::del(&mut conn, format!("ak:reset_code:{}", idx))
        .await
        .unwrap_or(());
    cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_find_password_no_email_for_non_matching_user() {
    // (name, birthday, email) 모두 임의값 → blind_index 일치자 없음 → generic 200 응답 + 이메일 0건.
    let (st, sent) = common::make_test_state_with_capturing_email().await;

    let unique_suffix = uuid::Uuid::new_v4();
    let req = FindPasswordReq {
        name: format!("phase3_{}", unique_suffix),
        birthday: "1990-01-15".to_string(),
        email: format!("phase3_find_{}@example.com", unique_suffix),
    };
    let ip = "10.0.0.55".to_string();

    let result = AuthService::find_password(&st, req, ip.clone()).await;
    assert!(
        result.is_ok(),
        "non-matching user → generic Ok, got: {:?}",
        result
    );

    let captured = sent.lock().await;
    assert_eq!(
        captured.len(),
        0,
        "non-matching user → 이메일 발송 0건, got: {}",
        captured.len()
    );
    drop(captured);

    // Cleanup: find_password 의 rate limit 은 IP 기반
    let mut conn = st.redis.get().await.expect("redis conn for cleanup");
    let _: () = redis::AsyncCommands::del(&mut conn, format!("rl:find_password:{}", ip))
        .await
        .unwrap_or(());
}
