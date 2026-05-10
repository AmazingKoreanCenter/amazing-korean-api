//! G10-deep-2 통합 테스트 — `AuthUser` FromRequestParts extractor.
//!
//! ## 범위 — extractor matrix
//!
//! - 유효 Bearer JWT → 200 + Claims 추출
//! - Authorization 헤더 누락 → 401
//! - Bearer prefix 부재 → 401
//! - malformed JWT → 401
//! - 다른 secret 으로 서명된 JWT → 401

mod common;

use amazing_korean_api::api::auth::extractor::AuthUser;
use amazing_korean_api::api::auth::jwt;
use amazing_korean_api::state::AppState;
use amazing_korean_api::types::UserAuth;
use axum::body::{to_bytes, Body};
use axum::http::{header::AUTHORIZATION, Request, StatusCode};
use axum::routing::get;
use axum::Router;
use tower::util::ServiceExt;

async fn protected_handler(AuthUser(claims): AuthUser) -> String {
    format!("uid={},role={:?}", claims.sub, claims.role)
}

fn make_router(state: AppState) -> Router {
    Router::new()
        .route("/protected", get(protected_handler))
        .with_state(state)
}

fn make_token(state: &AppState, role: UserAuth) -> String {
    let (token_res, _jti) = jwt::create_token(
        4242,
        "extractor-test-session",
        role,
        state.cfg.jwt_access_ttl_min,
        &state.cfg.jwt_secret,
    )
    .expect("create token");
    token_res.access_token
}

async fn call_protected(state: AppState, auth_header: Option<&str>) -> (StatusCode, String) {
    let app = make_router(state);
    let mut builder = Request::builder().uri("/protected");
    if let Some(value) = auth_header {
        builder = builder.header(AUTHORIZATION, value);
    }
    let req = builder.body(Body::empty()).expect("build request");
    let res = app.oneshot(req).await.expect("oneshot");
    let status = res.status();
    let bytes = to_bytes(res.into_body(), 1024).await.expect("body bytes");
    let body = String::from_utf8_lossy(&bytes).to_string();
    (status, body)
}

#[ignore = "requires local PostgreSQL + Redis + .env.test"]
#[tokio::test]
async fn test_auth_user_accepts_valid_bearer_token() {
    let st = common::make_test_state().await;
    let token = make_token(&st, UserAuth::Learner);
    let (status, body) = call_protected(st, Some(&format!("Bearer {}", token))).await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("uid=4242"),
        "body must include sub, got: {}",
        body
    );
    assert!(
        body.contains("Learner"),
        "body must include role, got: {}",
        body
    );
}

#[ignore = "requires local PostgreSQL + Redis + .env.test"]
#[tokio::test]
async fn test_auth_user_rejects_missing_authorization_header() {
    let st = common::make_test_state().await;
    let (status, _) = call_protected(st, None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[ignore = "requires local PostgreSQL + Redis + .env.test"]
#[tokio::test]
async fn test_auth_user_rejects_missing_bearer_prefix() {
    let st = common::make_test_state().await;
    let token = make_token(&st, UserAuth::Admin);
    // "Bearer " prefix 없이 raw token 만
    let (status, _) = call_protected(st, Some(&token)).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[ignore = "requires local PostgreSQL + Redis + .env.test"]
#[tokio::test]
async fn test_auth_user_rejects_malformed_token() {
    let st = common::make_test_state().await;
    let (status, _) = call_protected(st, Some("Bearer not.a.jwt")).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[ignore = "requires local PostgreSQL + Redis + .env.test"]
#[tokio::test]
async fn test_auth_user_rejects_token_signed_with_wrong_secret() {
    let st = common::make_test_state().await;
    // 같은 구조의 토큰을 다른 secret 으로 발급 → secret 불일치 → 401
    let (token_res, _) =
        jwt::create_token(1, "s", UserAuth::Admin, 30, "totally-different-secret-XYZ")
            .expect("create token");
    let (status, _) = call_protected(st, Some(&format!("Bearer {}", token_res.access_token))).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}
