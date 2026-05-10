//! Phase 3 통합 테스트 — `UserService::signup`.
//!
//! ## 셋업
//!
//! `tests/common/mod.rs` 의 helper 사용. EmailSender mock + email_provider 오버라이드 패턴.
//!
//! ## 실행
//!
//! ```bash
//! DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/amazing_korean_db \
//! REDIS_URL=redis://:redis_dev_password@127.0.0.1:16379 \
//! JWT_SECRET=test_jwt_secret_must_be_at_least_32_bytes_long \
//! HMAC_KEY=$(openssl rand -base64 32) \
//! ENCRYPTION_KEY_V1=$(openssl rand -base64 32) \
//!   cargo test --test user_signup_integration -- --ignored
//! ```
//!
//! ## 범위
//!
//! - validation: weak password / 형식 위반 → ValidationGeneric
//! - business: terms 미동의 → BadRequest
//! - email_provider=none: 자동 인증 + 이메일 발송 0건
//! - email_provider=resend (mock): 이메일 1건 + check_email=false
//! - 중복 (verified): Conflict
//! - 미인증 재가입: 덮어쓰기 + 새 이메일 발송

mod common;

use amazing_korean_api::api::user::dto::SignupReq;
use amazing_korean_api::api::user::service::UserService;
use amazing_korean_api::crypto::CryptoService;
use amazing_korean_api::error::AppError;
use amazing_korean_api::types::UserGender;
use chrono::NaiveDate;
use common::{cleanup_test_user, insert_test_user, TestUserSpec};

fn signup_req(email: &str) -> SignupReq {
    SignupReq {
        email: email.to_string(),
        password: "ValidPass123".to_string(),
        name: format!("name_{}", &email[..email.find('@').unwrap_or(5).min(5)]),
        nickname: format!("nick_{}", &email[..email.find('@').unwrap_or(5).min(5)]),
        language: "ko".to_string(),
        country: "KR".to_string(),
        birthday: NaiveDate::from_ymd_opt(1990, 1, 15).unwrap(),
        gender: UserGender::None,
        terms_service: true,
        terms_personal: true,
    }
}

async fn cleanup_signup(st: &amazing_korean_api::state::AppState, email: &str, ip: &str) {
    let mut conn = st.redis.get().await.expect("redis conn for cleanup");
    let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
    let idx = crypto.blind_index(email).expect("blind_index");
    let _: () = redis::AsyncCommands::del(&mut conn, format!("rl:signup:{}:{}", idx, ip))
        .await
        .unwrap_or(());
    let _: () = redis::AsyncCommands::del(&mut conn, format!("ak:email_verify:{}", idx))
        .await
        .unwrap_or(());
}

async fn find_user_id_by_email(
    st: &amazing_korean_api::state::AppState,
    email: &str,
) -> Option<i64> {
    let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
    let idx = crypto.blind_index(email).expect("blind_index");
    sqlx::query_scalar::<_, i64>("SELECT user_id FROM users WHERE user_email_idx = $1")
        .bind(idx)
        .fetch_optional(&st.db)
        .await
        .expect("find user")
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_signup_validation_rejects_weak_password() {
    // SignupReq.password = #[validate(length(min = 8, max = 72))]. 7자리 → ValidationGeneric.
    let st = common::make_test_state().await;

    let mut req = signup_req(&format!("phase3_weak_{}@example.com", uuid::Uuid::new_v4()));
    req.password = "Pass1".to_string(); // 5 chars (< min 8)

    let result = UserService::signup(&st, req, "10.0.3.1".to_string()).await;
    match result {
        Err(AppError::ValidationGeneric) => {}
        Err(e) => panic!(
            "weak password → ValidationGeneric expected, got Err: {:?}",
            e
        ),
        Ok(_) => panic!("weak password → ValidationGeneric expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_signup_rejects_when_terms_not_accepted() {
    // terms_service=false → BadRequest("Terms must be accepted").
    let st = common::make_test_state().await;

    let mut req = signup_req(&format!(
        "phase3_terms_{}@example.com",
        uuid::Uuid::new_v4()
    ));
    req.terms_service = false;

    let result = UserService::signup(&st, req, "10.0.3.2".to_string()).await;
    match result {
        Err(AppError::BadRequest(msg)) => {
            assert!(msg.contains("Terms"), "msg 에 'Terms' 포함, got: {}", msg);
        }
        Err(e) => panic!("terms 미동의 → BadRequest expected, got Err: {:?}", e),
        Ok(_) => panic!("terms 미동의 → BadRequest expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_signup_with_email_provider_none_auto_verifies() {
    // EMAIL_PROVIDER=none (default) → user 생성 + check_email 자동 true + requires_verification=false +
    // 이메일 발송 0건 (email_sender 가 캡처용으로 설정되어도 short-circuit).
    let (st, sent) = common::make_test_state_with_capturing_email().await;
    // make_test_state 의 .env 에서 EMAIL_PROVIDER=none 가정. 명시적 강제:
    let mut st = st;
    st.cfg.email_provider = "none".to_string();

    let unique_email = format!("phase3_signup_none_{}@example.com", uuid::Uuid::new_v4());
    let req = signup_req(&unique_email);
    let ip = "10.0.3.3".to_string();

    let result = UserService::signup(&st, req, ip.clone()).await;
    let res = match result {
        Ok(r) => r,
        Err(e) => panic!("EMAIL_PROVIDER=none signup → Ok expected, got Err: {:?}", e),
    };

    assert!(
        !res.requires_verification,
        "EMAIL_PROVIDER=none → requires_verification=false, got: {}",
        res.requires_verification
    );

    // 이메일 발송 0건 (short-circuit)
    let captured = sent.lock().await;
    assert_eq!(captured.len(), 0, "EMAIL_PROVIDER=none → 이메일 0건");
    drop(captured);

    // user_check_email = true 검증
    let user_id = find_user_id_by_email(&st, &unique_email)
        .await
        .expect("user inserted");
    let check_email: bool =
        sqlx::query_scalar("SELECT user_check_email FROM users WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&st.db)
            .await
            .expect("query user_check_email");
    assert!(check_email, "user_check_email = true (auto-verify)");

    // Cleanup
    cleanup_signup(&st, &unique_email, &ip).await;
    cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_signup_with_email_provider_resend_sends_verification_email() {
    // EMAIL_PROVIDER=resend + CapturingEmailSender → 이메일 1건 + check_email=false + requires_verification=true.
    let (mut st, sent) = common::make_test_state_with_capturing_email().await;
    st.cfg.email_provider = "resend".to_string(); // mock = CapturingEmailSender 가 처리

    let unique_email = format!("phase3_signup_resend_{}@example.com", uuid::Uuid::new_v4());
    let req = signup_req(&unique_email);
    let ip = "10.0.3.4".to_string();

    let result = UserService::signup(&st, req, ip.clone()).await;
    let res = match result {
        Ok(r) => r,
        Err(e) => panic!(
            "EMAIL_PROVIDER=resend signup → Ok expected, got Err: {:?}",
            e
        ),
    };

    assert!(
        res.requires_verification,
        "EMAIL_PROVIDER=resend → requires_verification=true, got: {}",
        res.requires_verification
    );

    let captured = sent.lock().await;
    assert_eq!(captured.len(), 1, "이메일 1건, got: {}", captured.len());
    let mail = &captured[0];
    assert_eq!(mail.to, unique_email.to_lowercase());
    assert!(
        mail.subject.contains("이메일 인증"),
        "subject 에 '이메일 인증' 포함, got: {}",
        mail.subject
    );
    drop(captured);

    // user_check_email = false 검증
    let user_id = find_user_id_by_email(&st, &unique_email)
        .await
        .expect("user inserted");
    let check_email: bool =
        sqlx::query_scalar("SELECT user_check_email FROM users WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&st.db)
            .await
            .expect("query user_check_email");
    assert!(!check_email, "user_check_email = false (대기중)");

    // Cleanup
    cleanup_signup(&st, &unique_email, &ip).await;
    cleanup_test_user(&st, user_id).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_signup_returns_conflict_for_already_verified_email() {
    // 이미 인증 완료된 user 가 존재하는 email 로 signup → Conflict.
    let st = common::make_test_state().await;

    let spec = TestUserSpec::random(); // check_email=true (기본)
    let user_id = insert_test_user(&st, &spec).await;

    let req = signup_req(&spec.email);
    let ip = "10.0.3.5".to_string();
    let result = UserService::signup(&st, req, ip.clone()).await;
    match result {
        Err(AppError::Conflict(msg)) => {
            assert!(
                msg.to_lowercase().contains("email"),
                "Conflict msg 에 'email' 포함, got: {}",
                msg
            );
        }
        Err(e) => panic!("verified email 중복 → Conflict expected, got Err: {:?}", e),
        Ok(_) => panic!("verified email 중복 → Conflict expected, got Ok"),
    }

    cleanup_signup(&st, &spec.email, &ip).await;
    cleanup_test_user(&st, user_id).await;
}
