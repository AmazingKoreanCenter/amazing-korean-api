use axum::{routing::{get, post}, Router};
use crate::state::AppState;
use super::handler;

pub fn auth_router() -> Router<AppState> {
    Router::new()
        // 세션/토큰 관련
        .route("/login", post(handler::login))
        .route("/logout", post(handler::logout))
        .route("/logout/all", post(handler::logout_all)) // 모든 기기 로그아웃
        .route("/refresh", post(handler::refresh))

        // 계정 찾기/복구
        .route("/find-id", post(handler::find_id))
        .route("/find-password", post(handler::find_password))
        .route("/reset-pw", post(handler::reset_password))

        // 비밀번호 재설정 (이메일 인증 기반)
        .route("/request-reset", post(handler::request_reset))
        .route("/verify-reset", post(handler::verify_reset))

        // 회원가입 이메일 인증
        .route("/verify-email", post(handler::verify_email))
        .route("/resend-verification", post(handler::resend_verification))

        // Google OAuth
        .route("/google", get(handler::google_auth_start))
        .route("/google/callback", get(handler::google_auth_callback))
}