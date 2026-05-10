//! Phase 3 통합 테스트 — `StudyService` (B3 트랙).
//!
//! ## 범위
//!
//! 외부 의존 없는 input validation + DB-only path:
//! - list_studies 페이지/sort/program validation
//! - get_study_task / get_task_explain non-existent → NotFound

mod common;

use amazing_korean_api::api::study::dto::StudyListReq;
use amazing_korean_api::api::study::service::StudyService;
use amazing_korean_api::error::AppError;

fn empty_list_req() -> StudyListReq {
    StudyListReq {
        page: None,
        per_page: None,
        program: None,
        sort: None,
        lang: None,
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_list_studies_rejects_zero_page() {
    let st = common::make_test_state().await;
    let mut req = empty_list_req();
    req.page = Some(0);

    let result = StudyService::list_studies(&st, req).await;
    match result {
        Err(AppError::BadRequest(msg)) => {
            assert!(msg.contains("page"), "msg에 'page' 포함, got: {}", msg);
        }
        Err(e) => panic!("page=0 → BadRequest expected, got Err: {:?}", e),
        Ok(_) => panic!("page=0 → BadRequest expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_list_studies_rejects_per_page_over_100() {
    let st = common::make_test_state().await;
    let mut req = empty_list_req();
    req.per_page = Some(101);

    let result = StudyService::list_studies(&st, req).await;
    match result {
        Err(AppError::Unprocessable(msg)) => {
            assert!(msg.contains("100"), "msg에 '100' 포함, got: {}", msg);
        }
        Err(e) => panic!("per_page=101 → Unprocessable expected, got Err: {:?}", e),
        Ok(_) => panic!("per_page=101 → Unprocessable expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_list_studies_rejects_invalid_program() {
    let st = common::make_test_state().await;
    let mut req = empty_list_req();
    req.program = Some("not_a_real_program".to_string());

    let result = StudyService::list_studies(&st, req).await;
    match result {
        Err(AppError::Unprocessable(_)) => {}
        Err(e) => panic!("invalid program → Unprocessable expected, got Err: {:?}", e),
        Ok(_) => panic!("invalid program → Unprocessable expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_list_studies_rejects_empty_sort() {
    let st = common::make_test_state().await;
    let mut req = empty_list_req();
    req.sort = Some("   ".to_string()); // trim → empty

    let result = StudyService::list_studies(&st, req).await;
    match result {
        Err(AppError::BadRequest(msg)) => {
            assert!(msg.contains("sort"), "msg에 'sort' 포함, got: {}", msg);
        }
        Err(e) => panic!("empty sort → BadRequest expected, got Err: {:?}", e),
        Ok(_) => panic!("empty sort → BadRequest expected, got Ok"),
    }
}

#[ignore = "requires local PostgreSQL + Redis + .env.test (Phase 3 보류 정책)"]
#[tokio::test]
async fn test_list_studies_rejects_invalid_sort() {
    let st = common::make_test_state().await;
    let mut req = empty_list_req();
    req.sort = Some("not_a_sort".to_string());

    let result = StudyService::list_studies(&st, req).await;
    match result {
        Err(AppError::Unprocessable(_)) => {}
        Err(e) => panic!("invalid sort → Unprocessable expected, got Err: {:?}", e),
        Ok(_) => panic!("invalid sort → Unprocessable expected, got Ok"),
    }
}
