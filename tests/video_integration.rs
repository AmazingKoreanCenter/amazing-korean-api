//! Phase 3 통합 테스트 — `VideoService` (B6 트랙).
//!
//! ## 범위 — pagination validation + non-existent lookup

mod common;

use amazing_korean_api::api::video::dto::VideoListReq;
use amazing_korean_api::api::video::service::VideoService;
use amazing_korean_api::error::AppError;

fn list_req(page: u64, per_page: u64) -> VideoListReq {
    VideoListReq {
        page,
        per_page,
        q: None,
        tag: None,
        state: None,
        sort: None,
        lang: None,
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_list_videos_rejects_zero_page() {
    let st = common::make_test_state().await;
    let req = list_req(0, 20);

    let result = VideoService::list_videos(&st, req).await;
    match result {
        Err(AppError::BadRequest(_)) => {}
        Err(e) => panic!("page=0 → BadRequest expected, got Err: {:?}", e),
        Ok(_) => panic!("page=0 → BadRequest expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_list_videos_rejects_per_page_over_100() {
    let st = common::make_test_state().await;
    let req = list_req(1, 101);

    let result = VideoService::list_videos(&st, req).await;
    match result {
        Err(AppError::BadRequest(_)) => {}
        Err(e) => panic!("per_page=101 → BadRequest expected, got Err: {:?}", e),
        Ok(_) => panic!("per_page=101 → BadRequest expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_get_video_detail_returns_not_found_for_unknown_id() {
    let st = common::make_test_state().await;

    let result = VideoService::get_video_detail(&st, 999_999_988, None).await;
    match result {
        Err(AppError::NotFound) => {}
        Err(e) => panic!("unknown id → NotFound expected, got Err: {:?}", e),
        Ok(_) => panic!("unknown id → Err expected, got Ok"),
    }
}

// =============================================================================
// C-video — list_videos happy (default pagination)
// =============================================================================

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_list_videos_happy_returns_default_pagination() {
    let st = common::make_test_state().await;
    let req = list_req(1, 20);

    let result = VideoService::list_videos(&st, req).await;
    let res = match result {
        Ok(r) => r,
        Err(e) => panic!("default → Ok expected, got Err: {:?}", e),
    };
    assert_eq!(res.meta.current_page, 1, "page=1");
    assert_eq!(res.meta.per_page, 20, "per_page=20");
}
