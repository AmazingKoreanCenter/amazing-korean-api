//! Phase 3 통합 테스트 — `admin_role_guard` middleware (C-admin-rbac).
//!
//! ## 범위 — RBAC matrix 검증
//!
//! - Hymn / Admin → 200 (통과)
//! - Manager → 403 ("class-based access ... coming soon")
//! - Learner → 403 ("Insufficient permissions")
//! - Authorization 헤더 누락 → 401
//! - 잘못된 JWT → 401

mod common;

use amazing_korean_api::api::admin::role_guard::admin_role_guard;
use amazing_korean_api::api::auth::jwt;
use amazing_korean_api::state::AppState;
use amazing_korean_api::types::UserAuth;
use axum::body::Body;
use axum::http::{header::AUTHORIZATION, Request, StatusCode};
use axum::routing::get;
use axum::{middleware, Router};
use tower::util::ServiceExt;

fn make_router(state: AppState) -> Router {
    Router::new()
        .route("/admin/test", get(|| async { "ok" }))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            admin_role_guard,
        ))
        .with_state(state)
}

fn make_token(state: &AppState, role: UserAuth) -> String {
    make_token_sid(state, role, "phase3-rbac-session")
}

fn make_token_sid(state: &AppState, role: UserAuth, session_id: &str) -> String {
    let (token_res, _jti) = jwt::create_token(
        9999, // user_id
        session_id,
        role,
        state.cfg.jwt_access_ttl_min,
        &state.cfg.jwt_secret,
    )
    .expect("create token");
    token_res.access_token
}

async fn call_admin_test(state: AppState, auth_header: Option<&str>) -> StatusCode {
    let app = make_router(state);
    let mut builder = Request::builder().uri("/admin/test");
    if let Some(token) = auth_header {
        builder = builder.header(AUTHORIZATION, token);
    }
    let req = builder.body(Body::empty()).expect("build request");
    let res = app.oneshot(req).await.expect("oneshot");
    res.status()
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_admin_role_guard_allows_hymn() {
    let st = common::make_test_state().await;
    let token = make_token(&st, UserAuth::Hymn);
    common::seed_session(&st, "phase3-rbac-session", 9999).await;
    let status = call_admin_test(st, Some(&format!("Bearer {}", token))).await;
    assert_eq!(status, StatusCode::OK, "Hymn → 200, got: {}", status);
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_admin_role_guard_allows_admin() {
    let st = common::make_test_state().await;
    let token = make_token(&st, UserAuth::Admin);
    common::seed_session(&st, "phase3-rbac-session", 9999).await;
    let status = call_admin_test(st, Some(&format!("Bearer {}", token))).await;
    assert_eq!(status, StatusCode::OK, "Admin → 200, got: {}", status);
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_admin_role_guard_blocks_manager() {
    let st = common::make_test_state().await;
    let token = make_token(&st, UserAuth::Manager);
    common::seed_session(&st, "phase3-rbac-session", 9999).await;
    let status = call_admin_test(st, Some(&format!("Bearer {}", token))).await;
    assert_eq!(
        status,
        StatusCode::FORBIDDEN,
        "Manager → 403, got: {}",
        status
    );
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_admin_role_guard_blocks_learner() {
    let st = common::make_test_state().await;
    let token = make_token(&st, UserAuth::Learner);
    common::seed_session(&st, "phase3-rbac-session", 9999).await;
    let status = call_admin_test(st, Some(&format!("Bearer {}", token))).await;
    assert_eq!(
        status,
        StatusCode::FORBIDDEN,
        "Learner → 403, got: {}",
        status
    );
}

/// 2.1 회귀 — admin 토큰이라도 세션 폐기(ak:session 부재) 시 401 (role 검사 이전).
#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_admin_role_guard_rejects_revoked_session() {
    let st = common::make_test_state().await;
    // 아무 테스트도 시드하지 않는 고유 session_id → ak:session 부재 = 폐기 상태
    // (다른 테스트와 sid 공유 시 그쪽 seed_session 이 키를 살려 거짓 통과함)
    let token = make_token_sid(&st, UserAuth::Admin, "rbac-revoked-never-seeded");
    let status = call_admin_test(st, Some(&format!("Bearer {}", token))).await;
    assert_eq!(
        status,
        StatusCode::UNAUTHORIZED,
        "폐기된 세션은 admin 이라도 401, got: {}",
        status
    );
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_admin_role_guard_rejects_missing_authorization_header() {
    let st = common::make_test_state().await;
    let status = call_admin_test(st, None).await;
    assert_eq!(
        status,
        StatusCode::UNAUTHORIZED,
        "missing Authorization → 401, got: {}",
        status
    );
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_admin_role_guard_rejects_invalid_jwt() {
    let st = common::make_test_state().await;
    let status = call_admin_test(st, Some("Bearer not.a.valid.jwt")).await;
    assert_eq!(
        status,
        StatusCode::UNAUTHORIZED,
        "invalid JWT → 401, got: {}",
        status
    );
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_admin_role_guard_rejects_authorization_without_bearer_prefix() {
    let st = common::make_test_state().await;
    let token = make_token(&st, UserAuth::Hymn);
    // "Bearer " prefix 누락
    let status = call_admin_test(st, Some(&token)).await;
    assert_eq!(
        status,
        StatusCode::UNAUTHORIZED,
        "no Bearer prefix → 401, got: {}",
        status
    );
}
