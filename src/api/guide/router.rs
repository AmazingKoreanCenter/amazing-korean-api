use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AppState;

use super::handler;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list_guides))
        .route("/{guide_idx}", get(handler::get_guide))
        .route("/{guide_idx}/progress", get(handler::get_progress))
        .route(
            "/{guide_idx}/sentences/{sentence_no}/log",
            post(handler::log_sentence),
        )
}

#[cfg(test)]
mod tests {
    /// matchit 은 라우트 구성 시 충돌을 panic 으로 검증 — `/{guide_idx}` vs
    /// `/{guide_idx}/progress` vs `/{guide_idx}/sentences/{sentence_no}/log` 가
    /// 공존 가능한지 구성 강제로 회귀 보호 (통합 테스트는 서비스 직접 호출이라 미커버).
    #[test]
    fn router_builds_without_route_conflicts() {
        let _ = super::router();
    }
}
