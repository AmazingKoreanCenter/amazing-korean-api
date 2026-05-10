//! Phase 3 통합 테스트 — `LessonService` (B4 트랙).
//!
//! ## 범위 — pagination / sort validation + non-existent lesson lookup

mod common;

use amazing_korean_api::api::lesson::dto::{LessonDetailReq, LessonListReq};
use amazing_korean_api::api::lesson::service::LessonService;
use amazing_korean_api::error::AppError;

fn empty_list() -> LessonListReq {
    LessonListReq {
        page: None,
        per_page: None,
        sort: None,
        lang: None,
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_list_lessons_rejects_zero_page() {
    let st = common::make_test_state().await;
    let mut req = empty_list();
    req.page = Some(0);

    let result = LessonService::list_lessons(&st.db, req).await;
    match result {
        Err(AppError::BadRequest(_)) => {}
        Err(e) => panic!("page=0 → BadRequest expected, got Err: {:?}", e),
        Ok(_) => panic!("page=0 → BadRequest expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_list_lessons_rejects_per_page_over_50() {
    let st = common::make_test_state().await;
    let mut req = empty_list();
    req.per_page = Some(51);

    let result = LessonService::list_lessons(&st.db, req).await;
    match result {
        Err(AppError::Unprocessable(msg)) => {
            assert!(msg.contains("50"), "msg에 '50' 포함, got: {}", msg);
        }
        Err(e) => panic!("per_page=51 → Unprocessable expected, got Err: {:?}", e),
        Ok(_) => panic!("per_page=51 → Unprocessable expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_list_lessons_rejects_invalid_sort() {
    let st = common::make_test_state().await;
    let mut req = empty_list();
    req.sort = Some("created_desc".to_string());

    let result = LessonService::list_lessons(&st.db, req).await;
    match result {
        Err(AppError::Unprocessable(msg)) => {
            assert!(msg.contains("sort"), "msg에 'sort' 포함, got: {}", msg);
        }
        Err(e) => panic!("invalid sort → Unprocessable expected, got Err: {:?}", e),
        Ok(_) => panic!("invalid sort → Unprocessable expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_get_lesson_detail_returns_not_found_for_unknown_id() {
    let st = common::make_test_state().await;

    let req = LessonDetailReq {
        page: None,
        per_page: None,
        lang: None,
    };
    let result = LessonService::get_lesson_detail(&st.db, 999_999_989, req).await;
    match result {
        Err(AppError::NotFound) => {}
        Err(e) => panic!("unknown id → NotFound expected, got Err: {:?}", e),
        Ok(_) => panic!("unknown id → Err expected, got Ok"),
    }
}
