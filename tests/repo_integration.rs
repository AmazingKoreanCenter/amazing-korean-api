//! Repo 통합 테스트 (Phase 1) — sqlx::test 매크로 사용.
//!
//! ## 셋업
//!
//! 로컬 PostgreSQL 필요. `DATABASE_URL` 환경변수가 superuser 권한이 있어야
//! 임시 DB 생성 + migration 자동 실행 (sqlx::test 매크로 동작).
//!
//! ## 실행
//!
//! ```bash
//! DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/amazing_korean_db \
//!   cargo test --test repo_integration
//! ```
//!
//! ## 범위
//!
//! Phase 1 = 빈 DB 상태에서 SQL 검증 (negative cases). Postgres only,
//! Redis/Email/외부 API 의존 0. 빈 DB 반환값 검증으로 SQL 컴파일/스키마 일치
//! 회귀 캡처.
//!
//! ## CI
//!
//! 본 트랙 = G1/G2 보류 정책 유지. CI service container 미설정 = `cargo test
//! --test repo_integration` 은 로컬 실행만. PR check 워크플로 변경 X.

use sqlx::PgPool;

use amazing_korean_api::api::lesson::repo::LessonRepo;
use amazing_korean_api::api::user::repo::{
    find_user, find_user_by_nickname, find_user_id_and_check_email_by_email_idx,
    find_user_id_by_email_idx, find_users_setting,
};

// =============================================================================
// LessonRepo
// =============================================================================

#[ignore = "requires local PostgreSQL with DATABASE_URL set (Phase 1 보류 정책)"]
#[sqlx::test]
async fn test_lesson_count_all_returns_zero_for_empty_db(pool: PgPool) {
    let count = LessonRepo::count_all(&pool).await.expect("count_all 실패");
    assert_eq!(count, 0, "빈 DB 에서 count_all = 0");
}

#[ignore = "requires local PostgreSQL with DATABASE_URL set (Phase 1 보류 정책)"]
#[sqlx::test]
async fn test_lesson_count_items_returns_zero_for_missing_lesson(pool: PgPool) {
    // 존재하지 않는 lesson_id → 0
    let count = LessonRepo::count_items(&pool, 999_999)
        .await
        .expect("count_items 실패");
    assert_eq!(count, 0, "존재하지 않는 lesson 의 item 카운트 = 0");
}

#[ignore = "requires local PostgreSQL with DATABASE_URL set (Phase 1 보류 정책)"]
#[sqlx::test]
async fn test_lesson_find_by_id_returns_none_for_missing(pool: PgPool) {
    let result = LessonRepo::find_lesson_by_id(&pool, 999_999)
        .await
        .expect("find_lesson_by_id 실패");
    assert!(result.is_none(), "존재하지 않는 lesson_id → None");
}

// =============================================================================
// user::repo
// =============================================================================

#[ignore = "requires local PostgreSQL with DATABASE_URL set (Phase 1 보류 정책)"]
#[sqlx::test]
async fn test_find_user_id_by_email_idx_returns_none_for_missing(pool: PgPool) {
    let result = find_user_id_by_email_idx(&pool, "nonexistent_blind_index")
        .await
        .expect("find_user_id_by_email_idx 실패");
    assert!(result.is_none(), "blind index 매칭 user 없음 → None");
}

#[ignore = "requires local PostgreSQL with DATABASE_URL set (Phase 1 보류 정책)"]
#[sqlx::test]
async fn test_find_user_id_and_check_email_by_email_idx_returns_none_for_missing(pool: PgPool) {
    let result = find_user_id_and_check_email_by_email_idx(&pool, "nonexistent")
        .await
        .expect("find_user_id_and_check_email_by_email_idx 실패");
    assert!(result.is_none());
}

#[ignore = "requires local PostgreSQL with DATABASE_URL set (Phase 1 보류 정책)"]
#[sqlx::test]
async fn test_find_user_by_nickname_returns_none_for_missing(pool: PgPool) {
    let result = find_user_by_nickname(&pool, "nonexistent_nickname")
        .await
        .expect("find_user_by_nickname 실패");
    assert!(result.is_none(), "닉네임 매칭 user 없음 → None");
}

#[ignore = "requires local PostgreSQL with DATABASE_URL set (Phase 1 보류 정책)"]
#[sqlx::test]
async fn test_find_user_returns_none_for_missing_id(pool: PgPool) {
    let result = find_user(&pool, 999_999).await.expect("find_user 실패");
    assert!(result.is_none());
}

#[ignore = "requires local PostgreSQL with DATABASE_URL set (Phase 1 보류 정책)"]
#[sqlx::test]
async fn test_find_users_setting_returns_none_for_missing_user(pool: PgPool) {
    let result = find_users_setting(&pool, 999_999)
        .await
        .expect("find_users_setting 실패");
    assert!(result.is_none());
}
