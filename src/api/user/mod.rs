pub mod dto;
pub mod handler;
pub mod service;

use axum::{routing::post, Router};
use crate::state::AppState;
use handler::create_user;

/// 서브 라우터는 Router<AppState> 반환(프로젝트 규칙에 맞춤)
pub fn router() -> Router<AppState> {
    Router::new().route("/users", post(create_user))
}
