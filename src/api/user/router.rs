use crate::state::AppState;
use axum::{routing::{get, post}, Router};
use super::handler::{get_me, signup, update_me};

/// 서브 라우터는 Router<AppState> 반환(프로젝트 규칙)
pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/users", post(signup))
        .route("/users/me", get(get_me).put(update_me))
}
