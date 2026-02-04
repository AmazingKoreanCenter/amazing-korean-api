//! 관리자 초대/승격 라우터

use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AppState;

use super::handler;

/// Admin Upgrade 라우터
///
/// - POST /admin/upgrade: 관리자 초대 (인증 필요 - HYMN/Admin)
/// - GET /admin/upgrade/verify: 초대 코드 검증 (Public)
/// - POST /admin/upgrade/accept: 관리자 계정 생성 (Public)
pub fn admin_upgrade_router() -> Router<AppState> {
    Router::new()
        // 인증 필요 (role_guard에서 처리)
        .route("/", post(handler::create_invite))
        // Public endpoints (초대 코드로 검증)
        .route("/verify", get(handler::verify_invite))
        .route("/accept", post(handler::accept_invite))
}
