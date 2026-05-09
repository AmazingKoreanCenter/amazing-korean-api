//! Repo 통합 테스트 (Phase 1) — 기존 PostgreSQL DB 직접 사용.
//!
//! ## 셋업 변경 (2026-05-10 후속⁹)
//!
//! 초기 시도 = `#[sqlx::test]` 매크로 (자동 임시 DB + migration). 실패:
//! sqlx 의 numeric version 정렬 문제로 `20260210_i18n_add_video_content_type.sql`
//! 이 `20260210000001_i18n_content_translations.sql` 보다 먼저 실행 → ALTER TYPE
//! content_type_enum 시점에 type 미존재 → fail.
//!
//! production 에서는 점진 적용으로 우회됨 (file rename = `_sqlx_migrations`
//! checksum 깨짐 위험 = 금지).
//!
//! 해결 = `#[tokio::test]` + 수동 PgPool + 기존 DB 직접 사용 (negative-case
//! tests 는 production data 무관 = `999_999` ID / "nonexistent_*" 닉네임).
//!
//! ## 실행
//!
//! ```bash
//! DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/amazing_korean_db \
//!   cargo test --test repo_integration -- --ignored
//! ```
//!
//! ## 범위
//!
//! Phase 1 = SQL 컴파일 + 빈 결과 반환 검증. Postgres only,
//! Redis/Email/외부 API 의존 0. `999_999` / "nonexistent_*" 가 production
//! 데이터와 충돌하지 않음 보장.
//!
//! ## CI
//!
//! 본 트랙 = G1/G2 보류 정책 유지. CI service container 미설정. PR check 워크플로 변경 X.
//! 신규 부채 = G16 migration 정렬 (sqlx numeric vs production 점진 적용).

use sqlx::PgPool;
use std::env;

use amazing_korean_api::api::lesson::repo::LessonRepo;
use amazing_korean_api::api::user::repo::{
    find_user, find_user_by_nickname, find_user_id_and_check_email_by_email_idx,
    find_user_id_by_email_idx, find_users_setting,
};

async fn pool() -> PgPool {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&url)
        .await
        .expect("PgPool::connect 실패 — DATABASE_URL 의 PostgreSQL 동작 확인")
}

// =============================================================================
// LessonRepo (negative cases — production data 와 무관한 ID 사용)
// =============================================================================

#[ignore = "requires local PostgreSQL with DATABASE_URL set (Phase 1 보류 정책)"]
#[tokio::test]
async fn test_lesson_count_items_returns_zero_for_missing_lesson() {
    let pool = pool().await;
    let count = LessonRepo::count_items(&pool, 999_999)
        .await
        .expect("count_items 실패");
    assert_eq!(count, 0, "존재하지 않는 lesson 의 item 카운트 = 0");
}

#[ignore = "requires local PostgreSQL with DATABASE_URL set (Phase 1 보류 정책)"]
#[tokio::test]
async fn test_lesson_find_by_id_returns_none_for_missing() {
    let pool = pool().await;
    let result = LessonRepo::find_lesson_by_id(&pool, 999_999)
        .await
        .expect("find_lesson_by_id 실패");
    assert!(result.is_none(), "존재하지 않는 lesson_id → None");
}

// =============================================================================
// user::repo (negative cases)
// =============================================================================

#[ignore = "requires local PostgreSQL with DATABASE_URL set (Phase 1 보류 정책)"]
#[tokio::test]
async fn test_find_user_id_by_email_idx_returns_none_for_missing() {
    let pool = pool().await;
    let result = find_user_id_by_email_idx(&pool, "nonexistent_blind_index")
        .await
        .expect("find_user_id_by_email_idx 실패");
    assert!(result.is_none(), "blind index 매칭 user 없음 → None");
}

#[ignore = "requires local PostgreSQL with DATABASE_URL set (Phase 1 보류 정책)"]
#[tokio::test]
async fn test_find_user_id_and_check_email_by_email_idx_returns_none_for_missing() {
    let pool = pool().await;
    let result = find_user_id_and_check_email_by_email_idx(&pool, "nonexistent")
        .await
        .expect("find_user_id_and_check_email_by_email_idx 실패");
    assert!(result.is_none());
}

#[ignore = "requires local PostgreSQL with DATABASE_URL set (Phase 1 보류 정책)"]
#[tokio::test]
async fn test_find_user_by_nickname_returns_none_for_missing() {
    let pool = pool().await;
    let result = find_user_by_nickname(&pool, "nonexistent_nickname_xyz_unique")
        .await
        .expect("find_user_by_nickname 실패");
    assert!(result.is_none(), "닉네임 매칭 user 없음 → None");
}

#[ignore = "requires local PostgreSQL with DATABASE_URL set (Phase 1 보류 정책)"]
#[tokio::test]
async fn test_find_user_returns_none_for_missing_id() {
    let pool = pool().await;
    let result = find_user(&pool, 999_999).await.expect("find_user 실패");
    assert!(result.is_none());
}

#[ignore = "requires local PostgreSQL with DATABASE_URL set (Phase 1 보류 정책)"]
#[tokio::test]
async fn test_find_users_setting_returns_none_for_missing_user() {
    let pool = pool().await;
    let result = find_users_setting(&pool, 999_999)
        .await
        .expect("find_users_setting 실패");
    assert!(result.is_none());
}
