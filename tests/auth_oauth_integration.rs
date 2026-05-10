//! Phase 3 통합 테스트 — Google OAuth (wiremock 미도입 범위).
//!
//! ## 셋업
//!
//! `tests/common/mod.rs` 의 `make_test_state()` 사용. Google API 호출 path 검증은
//! `wiremock` crate 도입 후 별도 트랙. 본 파일은 외부 HTTP 미발생 path 만 커버.
//!
//! ## 실행
//!
//! ```bash
//! DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/amazing_korean_db \
//! REDIS_URL=redis://:redis_dev_password@127.0.0.1:16379 \
//! JWT_SECRET=test_jwt_secret_must_be_at_least_32_bytes_long \
//! HMAC_KEY=$(openssl rand -base64 32) \
//! ENCRYPTION_KEY_V1=$(openssl rand -base64 32) \
//!   cargo test --test auth_oauth_integration -- --ignored
//! ```
//!
//! ## 범위
//!
//! - `google_auth_start`: 미설정 (Config Optional None) → Internal / 설정됨 → Ok(URL) + Redis state 저장
//! - `google_auth_callback`: 잘못된 state (Redis 미존재) → AUTH_401_INVALID_OAUTH_STATE
//!
//! happy path callback (Google token exchange + JWKS 검증) = wiremock 도입 후 별도.

mod common;

use amazing_korean_api::api::auth::handler::ParsedUa;
use amazing_korean_api::api::auth::service::AuthService;
use amazing_korean_api::error::AppError;

fn parsed_ua_default() -> ParsedUa {
    ParsedUa {
        os: None,
        browser: None,
        device: "other".into(),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_google_auth_start_returns_internal_when_unconfigured() {
    // GOOGLE_CLIENT_ID 미설정 (Config.google_client_id = None) → Internal.
    let mut st = common::make_test_state().await;
    // 명시적으로 None 강제 (.env 가 우연히 GOOGLE_CLIENT_ID 정의해도 미설정 path 검증)
    st.cfg.google_client_id = None;

    let result = AuthService::google_auth_start(&st).await;
    match result {
        Err(AppError::Internal(msg)) => {
            assert!(
                msg.contains("GOOGLE_CLIENT_ID"),
                "에러 메시지에 GOOGLE_CLIENT_ID 포함, got: {}",
                msg
            );
        }
        Err(e) => panic!("unconfigured → Internal expected, got Err: {:?}", e),
        Ok(_) => panic!("unconfigured → Internal expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_google_auth_start_returns_url_with_state_when_configured() {
    // Config 에 Google OAuth 설정 주입 → URL 생성 + Redis ak:oauth_state:<state> 저장.
    let mut st = common::make_test_state().await;
    st.cfg.google_client_id = Some("test-client-id-phase3".into());
    st.cfg.google_client_secret = Some("test-secret".into());
    st.cfg.google_redirect_uri = Some("http://localhost:3000/auth/google/callback".into());

    let result = AuthService::google_auth_start(&st).await;
    let url = match result {
        Ok(u) => u,
        Err(e) => panic!("configured → Ok(URL) expected, got Err: {:?}", e),
    };

    // URL 형식 검증
    assert!(
        url.starts_with("https://accounts.google.com/o/oauth2/v2/auth?"),
        "Google auth URL prefix, got: {}",
        url
    );
    assert!(
        url.contains("client_id=test-client-id-phase3"),
        "client_id query param, got: {}",
        url
    );
    assert!(url.contains("scope=openid"), "scope=openid, got: {}", url);
    assert!(url.contains("state="), "state query param, got: {}", url);
    assert!(url.contains("nonce="), "nonce query param, got: {}", url);

    // Redis 에 state→nonce 저장 검증 + cleanup
    // URL 에서 state 값 추출
    let state = url
        .split("state=")
        .nth(1)
        .and_then(|s| s.split('&').next())
        .expect("state value in URL");

    let mut conn = st.redis.get().await.expect("redis conn for cleanup");
    let stored: Option<String> =
        redis::AsyncCommands::get(&mut conn, format!("ak:oauth_state:{}", state))
            .await
            .ok()
            .flatten();
    assert!(stored.is_some(), "Redis 에 ak:oauth_state:{} 저장됨", state);
    let _: () = redis::AsyncCommands::del(&mut conn, format!("ak:oauth_state:{}", state))
        .await
        .unwrap_or(());
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_google_auth_callback_returns_unauthorized_for_invalid_state() {
    // Redis 미존재 state → AUTH_401_INVALID_OAUTH_STATE (Google API 호출 전 차단).
    let mut st = common::make_test_state().await;
    st.cfg.google_client_id = Some("test-client-id-phase3".into());
    st.cfg.google_client_secret = Some("test-secret".into());
    st.cfg.google_redirect_uri = Some("http://localhost:3000/auth/google/callback".into());

    let unknown_state = uuid::Uuid::new_v4().to_string();
    let result = AuthService::google_auth_callback(
        &st,
        "fake-auth-code",
        &unknown_state,
        "10.0.2.1".to_string(),
        None,
        parsed_ua_default(),
    )
    .await;

    match result {
        Err(AppError::Unauthorized(msg)) => {
            assert_eq!(msg, "AUTH_401_INVALID_OAUTH_STATE", "got: {}", msg);
        }
        Err(e) => panic!(
            "missing state → Unauthorized(AUTH_401_INVALID_OAUTH_STATE) expected, got Err: {:?}",
            e
        ),
        Ok(_) => panic!("missing state → Unauthorized expected, got Ok"),
    }
}
