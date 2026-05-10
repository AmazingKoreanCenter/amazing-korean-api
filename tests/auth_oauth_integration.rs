//! Phase 3 통합 테스트 — Google OAuth (wiremock 도입 범위 포함).
//!
//! ## 셋업
//!
//! `tests/common/mod.rs` + `wiremock::MockServer` (Google /token + /jwks 엔드포인트
//! mock). `Config.google_token_url_override` / `google_jwks_url_override` 로 wiremock
//! URL 주입. RSA 2048 keypair 는 `OnceLock` 으로 1회 생성 (~1-2s, 후속 테스트는 재사용).
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

use amazing_korean_api::api::auth::dto::{AppleMobileLoginReq, GoogleMobileLoginReq};
use amazing_korean_api::api::auth::handler::ParsedUa;
use amazing_korean_api::api::auth::service::{AuthService, OAuthLoginOutcome};
use amazing_korean_api::crypto::CryptoService;
use amazing_korean_api::error::AppError;
use amazing_korean_api::external::apple::AppleOAuthClient;
use base64::engine::{general_purpose::URL_SAFE_NO_PAD, Engine};
use common::cleanup_test_user;
use rsa::pkcs8::EncodePrivateKey;
use rsa::traits::PublicKeyParts;
use rsa::{RsaPrivateKey, RsaPublicKey};
use std::sync::Arc;
use std::sync::OnceLock;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// =============================================================================
// 테스트용 RSA 2048 keypair + JWT 서명 helper
// =============================================================================

struct TestKey {
    private_pem: String,
    n_b64: String,
    e_b64: String,
    kid: String,
}

/// 첫 호출 시 RSA 2048 생성 (1-2초). 후속 호출은 캐시 재사용.
fn test_key() -> &'static TestKey {
    static CACHE: OnceLock<TestKey> = OnceLock::new();
    CACHE.get_or_init(|| {
        let mut rng = rand::thread_rng();
        let private = RsaPrivateKey::new(&mut rng, 2048).expect("RSA keygen");
        let public = RsaPublicKey::from(&private);
        let private_pem = private
            .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
            .expect("pkcs8 pem")
            .to_string();
        let n_b64 = URL_SAFE_NO_PAD.encode(public.n().to_bytes_be());
        let e_b64 = URL_SAFE_NO_PAD.encode(public.e().to_bytes_be());
        TestKey {
            private_pem,
            n_b64,
            e_b64,
            kid: "phase3-test-key".to_string(),
        }
    })
}

/// 주어진 claims 를 RS256 으로 서명하여 JWT 문자열 반환.
fn sign_test_id_token(claims: &serde_json::Value) -> String {
    let key = test_key();
    let mut header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256);
    header.kid = Some(key.kid.clone());
    let encoding_key = jsonwebtoken::EncodingKey::from_rsa_pem(key.private_pem.as_bytes())
        .expect("encoding key from pem");
    jsonwebtoken::encode(&header, claims, &encoding_key).expect("jwt encode")
}

/// JWKS JSON (단일 키) 반환. /jwks endpoint 응답에 사용.
fn test_jwks_json() -> serde_json::Value {
    let key = test_key();
    serde_json::json!({
        "keys": [{
            "kty": "RSA",
            "use": "sig",
            "alg": "RS256",
            "kid": key.kid,
            "n": key.n_b64,
            "e": key.e_b64,
        }],
    })
}

/// Google /token + /jwks 두 엔드포인트를 한 번에 mount. 호출 측은 MockServer 만 보유.
async fn mount_google_mocks(server: &MockServer, id_token: &str) {
    let token_response = serde_json::json!({
        "access_token": "fake-access",
        "id_token": id_token,
        "token_type": "Bearer",
        "expires_in": 3600,
        "scope": "openid email profile",
    });
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&token_response))
        .mount(server)
        .await;
    Mock::given(method("GET"))
        .and(path("/jwks"))
        .respond_with(ResponseTemplate::new(200).set_body_json(test_jwks_json()))
        .mount(server)
        .await;
}

/// `Config.google_*_override` 4개를 wiremock 주소 + test client_id 로 채우는 helper.
fn inject_google_test_config(st: &mut amazing_korean_api::state::AppState, mock_uri: &str) {
    st.cfg.google_client_id = Some("phase3-client-id".into());
    st.cfg.google_client_secret = Some("phase3-secret".into());
    st.cfg.google_redirect_uri = Some("http://localhost:3000/auth/google/callback".into());
    st.cfg.google_token_url_override = Some(format!("{}/token", mock_uri));
    st.cfg.google_jwks_url_override = Some(format!("{}/jwks", mock_uri));
}

async fn seed_oauth_state(st: &amazing_korean_api::state::AppState, state: &str, nonce: &str) {
    let mut conn = st.redis.get().await.expect("redis conn");
    let _: () = redis::AsyncCommands::set_ex(
        &mut conn,
        format!("ak:oauth_state:{}", state),
        nonce,
        st.cfg.oauth_state_ttl_sec as u64,
    )
    .await
    .expect("seed oauth state");
}

async fn cleanup_user_by_email(st: &amazing_korean_api::state::AppState, email: &str) {
    let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
    let email_idx = crypto.blind_index(email).expect("blind");
    if let Ok(Some(user_id)) =
        sqlx::query_scalar::<_, i64>("SELECT user_id FROM users WHERE user_email_idx = $1")
            .bind(&email_idx)
            .fetch_optional(&st.db)
            .await
    {
        cleanup_test_user(st, user_id).await;
    }
}

fn google_id_token_claims(aud: &str, sub: &str, email: &str, nonce: &str) -> serde_json::Value {
    let now = chrono::Utc::now().timestamp();
    serde_json::json!({
        "iss": "https://accounts.google.com",
        "azp": aud,
        "aud": aud,
        "sub": sub,
        "email": email,
        "email_verified": true,
        "name": "Phase3 Test User",
        "iat": now,
        "exp": now + 3600,
        "nonce": nonce,
    })
}

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

// =============================================================================
// google_auth_callback — wiremock + 서명된 ID token (happy + error paths)
// =============================================================================

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_google_auth_callback_creates_new_user_with_valid_id_token() {
    // wiremock /token + /jwks → 서명된 ID token → google_auth_callback Ok(Success { is_new_user=true }).
    let mock = MockServer::start().await;
    let mut st = common::make_test_state().await;
    inject_google_test_config(&mut st, &mock.uri());

    let state = uuid::Uuid::new_v4().to_string();
    let nonce = uuid::Uuid::new_v4().to_string();
    seed_oauth_state(&st, &state, &nonce).await;

    let google_email = format!("phase3_oauth_new_{}@example.com", uuid::Uuid::new_v4());
    let google_sub = format!("google-sub-{}", uuid::Uuid::new_v4());
    let claims = google_id_token_claims("phase3-client-id", &google_sub, &google_email, &nonce);
    let id_token = sign_test_id_token(&claims);
    mount_google_mocks(&mock, &id_token).await;

    let result = AuthService::google_auth_callback(
        &st,
        "fake-auth-code",
        &state,
        "10.0.4.1".to_string(),
        None,
        ParsedUa {
            os: None,
            browser: None,
            device: "other".into(),
        },
    )
    .await;

    match result {
        Ok(OAuthLoginOutcome::Success(success)) => {
            assert!(success.is_new_user, "신규 user 생성, got is_new_user=false");
            assert!(!success.refresh_token.is_empty(), "refresh_token 발급");
        }
        Ok(_) => panic!("MFA challenge unexpected for new user"),
        Err(e) => panic!("happy path → Ok(Success) expected, got Err: {:?}", e),
    }

    cleanup_user_by_email(&st, &google_email).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_google_auth_callback_rejects_invalid_nonce() {
    // ID token 의 nonce 가 Redis 에 저장된 값과 불일치 → AUTH_401_INVALID_NONCE.
    let mock = MockServer::start().await;
    let mut st = common::make_test_state().await;
    inject_google_test_config(&mut st, &mock.uri());

    let state = uuid::Uuid::new_v4().to_string();
    let stored_nonce = uuid::Uuid::new_v4().to_string();
    seed_oauth_state(&st, &state, &stored_nonce).await;

    let claims = google_id_token_claims(
        "phase3-client-id",
        "google-sub-invalid-nonce",
        &format!("phase3_nonce_{}@example.com", uuid::Uuid::new_v4()),
        "wrong-nonce-from-token", // stored 와 다름
    );
    let id_token = sign_test_id_token(&claims);
    mount_google_mocks(&mock, &id_token).await;

    let result = AuthService::google_auth_callback(
        &st,
        "fake-auth-code",
        &state,
        "10.0.4.2".to_string(),
        None,
        ParsedUa {
            os: None,
            browser: None,
            device: "other".into(),
        },
    )
    .await;

    match result {
        Err(AppError::Unauthorized(msg)) => {
            assert_eq!(msg, "AUTH_401_INVALID_NONCE", "got: {}", msg);
        }
        Err(e) => panic!(
            "invalid nonce → Unauthorized(AUTH_401_INVALID_NONCE) expected, got Err: {:?}",
            e
        ),
        Ok(_) => panic!("invalid nonce → Unauthorized expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_google_auth_callback_rejects_invalid_audience() {
    // ID token 의 aud 가 Config.google_client_id 와 불일치 → JWT validation fail.
    // jsonwebtoken 라이브러리가 audience 검증을 수행 → AppError::External (decode 단계).
    let mock = MockServer::start().await;
    let mut st = common::make_test_state().await;
    inject_google_test_config(&mut st, &mock.uri());

    let state = uuid::Uuid::new_v4().to_string();
    let nonce = uuid::Uuid::new_v4().to_string();
    seed_oauth_state(&st, &state, &nonce).await;

    let claims = google_id_token_claims(
        "wrong-audience-not-our-client", // Config.google_client_id 와 불일치
        "google-sub-invalid-aud",
        &format!("phase3_aud_{}@example.com", uuid::Uuid::new_v4()),
        &nonce,
    );
    let id_token = sign_test_id_token(&claims);
    mount_google_mocks(&mock, &id_token).await;

    let result = AuthService::google_auth_callback(
        &st,
        "fake-auth-code",
        &state,
        "10.0.4.3".to_string(),
        None,
        ParsedUa {
            os: None,
            browser: None,
            device: "other".into(),
        },
    )
    .await;

    // jsonwebtoken decode 가 InvalidAudience 으로 fail → External 에러로 wrap
    match result {
        Err(AppError::External(msg)) => {
            assert!(
                msg.contains("verification") || msg.contains("audience") || msg.contains("Invalid"),
                "audience 검증 실패 메시지, got: {}",
                msg
            );
        }
        Err(e) => panic!(
            "invalid audience → External (JWT 검증 fail) expected, got Err: {:?}",
            e
        ),
        Ok(_) => panic!("invalid audience → Err expected, got Ok"),
    }
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

// =============================================================================
// google_mobile_login (A2) — wiremock /jwks 재사용. /token 불필요 (id_token 직접).
// =============================================================================

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_google_mobile_login_returns_internal_when_unconfigured() {
    // GOOGLE_MOBILE_CLIENT_ID 미설정 → Internal (JWKS 호출 전 차단).
    let mut st = common::make_test_state().await;
    st.cfg.google_mobile_client_id = None;

    let req = GoogleMobileLoginReq {
        id_token: "irrelevant.fake.token".to_string(),
    };
    let result = AuthService::google_mobile_login(
        &st,
        req,
        "10.0.5.1".to_string(),
        None,
        ParsedUa {
            os: None,
            browser: None,
            device: "other".into(),
        },
    )
    .await;

    match result {
        Err(AppError::Internal(msg)) => {
            assert!(
                msg.contains("GOOGLE_MOBILE_CLIENT_ID"),
                "에러 메시지에 GOOGLE_MOBILE_CLIENT_ID 포함, got: {}",
                msg
            );
        }
        Err(e) => panic!("unconfigured → Internal expected, got Err: {:?}", e),
        Ok(_) => panic!("unconfigured → Internal expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_google_mobile_login_creates_new_user_with_valid_id_token() {
    // wiremock /jwks → 서명된 ID token → google_mobile_login Ok(Success { is_new_user=true }).
    // 모바일은 /token 호출 없음 (id_token 을 직접 받음).
    let mock = MockServer::start().await;
    let mut st = common::make_test_state().await;
    // 모바일 client_id 설정 (web client_id 와 다른 컬럼)
    st.cfg.google_mobile_client_id = Some("phase3-mobile-client-id".into());
    st.cfg.google_token_url_override = Some(format!("{}/token", mock.uri()));
    st.cfg.google_jwks_url_override = Some(format!("{}/jwks", mock.uri()));

    // /jwks 만 mount (mobile 은 /token 호출 안 함)
    Mock::given(method("GET"))
        .and(path("/jwks"))
        .respond_with(ResponseTemplate::new(200).set_body_json(test_jwks_json()))
        .mount(&mock)
        .await;

    let google_email = format!("phase3_mobile_{}@example.com", uuid::Uuid::new_v4());
    let google_sub = format!("mobile-sub-{}", uuid::Uuid::new_v4());
    // 모바일은 nonce 검증 안 함 (web 만 검증). 임의 값 사용.
    let claims = google_id_token_claims(
        "phase3-mobile-client-id",
        &google_sub,
        &google_email,
        "ignored-mobile-nonce",
    );
    let id_token = sign_test_id_token(&claims);

    let req = GoogleMobileLoginReq { id_token };
    let result = AuthService::google_mobile_login(
        &st,
        req,
        "10.0.5.2".to_string(),
        None,
        ParsedUa {
            os: None,
            browser: None,
            device: "mobile".into(),
        },
    )
    .await;

    match result {
        Ok(OAuthLoginOutcome::Success(success)) => {
            assert!(success.is_new_user, "신규 user 생성, got is_new_user=false");
            assert!(!success.refresh_token.is_empty(), "refresh_token 발급");
        }
        Ok(_) => panic!("MFA challenge unexpected for new user"),
        Err(e) => panic!("happy path → Ok(Success) expected, got Err: {:?}", e),
    }

    cleanup_user_by_email(&st, &google_email).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_google_mobile_login_rejects_wrong_audience() {
    // ID token 의 aud 가 google_mobile_client_id 와 불일치 → JWT validation fail.
    let mock = MockServer::start().await;
    let mut st = common::make_test_state().await;
    st.cfg.google_mobile_client_id = Some("phase3-mobile-client-id".into());
    st.cfg.google_token_url_override = Some(format!("{}/token", mock.uri()));
    st.cfg.google_jwks_url_override = Some(format!("{}/jwks", mock.uri()));

    Mock::given(method("GET"))
        .and(path("/jwks"))
        .respond_with(ResponseTemplate::new(200).set_body_json(test_jwks_json()))
        .mount(&mock)
        .await;

    let claims = google_id_token_claims(
        "wrong-mobile-aud", // mobile_client_id 와 불일치
        "mobile-sub-bad-aud",
        &format!("phase3_mobile_aud_{}@example.com", uuid::Uuid::new_v4()),
        "any-nonce",
    );
    let id_token = sign_test_id_token(&claims);

    let req = GoogleMobileLoginReq { id_token };
    let result = AuthService::google_mobile_login(
        &st,
        req,
        "10.0.5.3".to_string(),
        None,
        ParsedUa {
            os: None,
            browser: None,
            device: "mobile".into(),
        },
    )
    .await;

    match result {
        Err(AppError::External(_)) => { /* JWT validation fail wrap */ }
        Err(e) => panic!(
            "wrong audience → External (JWT 검증 fail) expected, got Err: {:?}",
            e
        ),
        Ok(_) => panic!("wrong audience → Err expected, got Ok"),
    }
}

// =============================================================================
// apple_mobile_login — wiremock /jwks 재사용 (RSA 동일). issuer = appleid.apple.com.
// =============================================================================

const APPLE_BUNDLE_ID: &str = "phase3-apple-bundle-id";

/// Apple ID token claims (Google 와 다른 구조 = email_verified="true" 문자열, name 없음).
fn apple_id_token_claims(aud: &str, sub: &str, email: Option<&str>) -> serde_json::Value {
    let now = chrono::Utc::now().timestamp();
    let mut claims = serde_json::json!({
        "iss": "https://appleid.apple.com",
        "aud": aud,
        "sub": sub,
        "iat": now,
        "exp": now + 3600,
    });
    if let Some(e) = email {
        claims["email"] = serde_json::Value::String(e.to_string());
        claims["email_verified"] = serde_json::Value::String("true".to_string());
    }
    claims
}

async fn mount_apple_jwks(server: &MockServer) {
    Mock::given(method("GET"))
        .and(path("/apple_jwks"))
        .respond_with(ResponseTemplate::new(200).set_body_json(test_jwks_json()))
        .mount(server)
        .await;
}

fn inject_apple_test_client(st: &mut amazing_korean_api::state::AppState, mock_uri: &str) {
    let client = AppleOAuthClient::with_url(
        APPLE_BUNDLE_ID.to_string(),
        format!("{}/apple_jwks", mock_uri),
    );
    st.apple_oauth = Some(Arc::new(client));
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_apple_mobile_login_returns_internal_when_unconfigured() {
    // st.apple_oauth = None → Internal (JWKS 호출 전 차단).
    let st = common::make_test_state().await; // apple_oauth = None default

    let req = AppleMobileLoginReq {
        id_token: "irrelevant.fake.token".to_string(),
        user_name: None,
    };
    let result = AuthService::apple_mobile_login(
        &st,
        req,
        "10.0.7.1".to_string(),
        None,
        ParsedUa {
            os: None,
            browser: None,
            device: "mobile".into(),
        },
    )
    .await;

    match result {
        Err(AppError::Internal(msg)) => {
            assert!(
                msg.contains("APPLE_CLIENT_ID"),
                "에러 메시지에 APPLE_CLIENT_ID 포함, got: {}",
                msg
            );
        }
        Err(e) => panic!("unconfigured → Internal expected, got Err: {:?}", e),
        Ok(_) => panic!("unconfigured → Internal expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_apple_mobile_login_creates_new_user_with_email() {
    // wiremock /apple_jwks → 서명된 Apple ID token + email 포함 → 신규 user 생성.
    let mock = MockServer::start().await;
    let mut st = common::make_test_state().await;
    inject_apple_test_client(&mut st, &mock.uri());
    mount_apple_jwks(&mock).await;

    let apple_email = format!("phase3_apple_{}@example.com", uuid::Uuid::new_v4());
    let apple_sub = format!("apple-sub-{}", uuid::Uuid::new_v4());
    let claims = apple_id_token_claims(APPLE_BUNDLE_ID, &apple_sub, Some(&apple_email));
    let id_token = sign_test_id_token(&claims);

    let req = AppleMobileLoginReq {
        id_token,
        user_name: Some("Phase3 Apple User".to_string()),
    };
    let result = AuthService::apple_mobile_login(
        &st,
        req,
        "10.0.7.2".to_string(),
        None,
        ParsedUa {
            os: None,
            browser: None,
            device: "mobile".into(),
        },
    )
    .await;

    match result {
        Ok(OAuthLoginOutcome::Success(success)) => {
            assert!(success.is_new_user, "신규 user 생성, got is_new_user=false");
            assert!(!success.refresh_token.is_empty(), "refresh_token 발급");
        }
        Ok(_) => panic!("MFA challenge unexpected for new user"),
        Err(e) => panic!("happy path → Ok(Success) expected, got Err: {:?}", e),
    }

    cleanup_user_by_email(&st, &apple_email).await;
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_apple_mobile_login_rejects_when_email_missing_for_new_user() {
    // Apple 특이사항: email 은 최초 인증에만 제공. email 없고 + oauth 매핑도 없음 → BadRequest.
    let mock = MockServer::start().await;
    let mut st = common::make_test_state().await;
    inject_apple_test_client(&mut st, &mock.uri());
    mount_apple_jwks(&mock).await;

    let apple_sub = format!("apple-sub-no-email-{}", uuid::Uuid::new_v4());
    // email = None → claims 에 email 필드 없음
    let claims = apple_id_token_claims(APPLE_BUNDLE_ID, &apple_sub, None);
    let id_token = sign_test_id_token(&claims);

    let req = AppleMobileLoginReq {
        id_token,
        user_name: None,
    };
    let result = AuthService::apple_mobile_login(
        &st,
        req,
        "10.0.7.3".to_string(),
        None,
        ParsedUa {
            os: None,
            browser: None,
            device: "mobile".into(),
        },
    )
    .await;

    match result {
        Err(AppError::BadRequest(msg)) => {
            assert!(
                msg.contains("Apple") || msg.contains("이메일"),
                "Apple email 안내 메시지, got: {}",
                msg
            );
        }
        Err(e) => panic!(
            "missing email + new user → BadRequest expected, got Err: {:?}",
            e
        ),
        Ok(_) => panic!("missing email + new user → BadRequest expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_apple_mobile_login_rejects_malformed_id_token() {
    // 형식 깨진 id_token → External (decode header fail).
    let mock = MockServer::start().await;
    let mut st = common::make_test_state().await;
    inject_apple_test_client(&mut st, &mock.uri());
    mount_apple_jwks(&mock).await;

    let req = AppleMobileLoginReq {
        id_token: "not-a-valid-jwt".to_string(),
        user_name: None,
    };
    let result = AuthService::apple_mobile_login(
        &st,
        req,
        "10.0.7.4".to_string(),
        None,
        ParsedUa {
            os: None,
            browser: None,
            device: "mobile".into(),
        },
    )
    .await;

    match result {
        Err(AppError::External(_)) => { /* JWT decode header fail */ }
        Err(e) => panic!("malformed token → External expected, got Err: {:?}", e),
        Ok(_) => panic!("malformed token → Err expected, got Ok"),
    }
}
