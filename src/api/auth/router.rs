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
        .route("/reset-pw", post(handler::reset_password))

        // Google OAuth
        .route("/google", get(handler::google_auth_start))
        .route("/google/callback", get(handler::google_auth_callback))
}