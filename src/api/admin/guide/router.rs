use axum::{
    routing::{get, patch},
    Router,
};

use crate::state::AppState;

use super::handler::{
    admin_get_guide, admin_guide_diff_export, admin_guide_stale, admin_list_guides,
    admin_update_guide_block, admin_update_guide_meta, admin_update_guide_sentence,
};

pub fn admin_guide_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_list_guides))
        // 고정 경로를 동적 {guide_idx} 보다 먼저 (라우팅 충돌 회피)
        .route("/stale", get(admin_guide_stale))
        .route("/diff-export", get(admin_guide_diff_export))
        .route("/blocks/{block_id}", patch(admin_update_guide_block))
        .route(
            "/sentences/{sentence_no}",
            patch(admin_update_guide_sentence),
        )
        .route(
            "/{guide_idx}",
            get(admin_get_guide).patch(admin_update_guide_meta),
        )
}
