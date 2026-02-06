use sqlx::PgPool;

use crate::error::AppResult;
use crate::types::{LessonAccess, LessonState};

use super::dto::{LessonItemDetailRes, LessonItemRes, LessonProgressRes, LessonRes};

pub struct LessonRepo;

impl LessonRepo {
    pub async fn count_all(pool: &PgPool) -> AppResult<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM lesson
            WHERE lesson_state = 'open'
            "#,
        )
        .fetch_one(pool)
        .await?;

        Ok(count)
    }

    pub async fn find_all(pool: &PgPool, limit: i64, offset: i64) -> AppResult<Vec<LessonRes>> {
        let rows = sqlx::query_as::<_, LessonRes>(
            r#"
            SELECT
                lesson_id::bigint as id,
                lesson_title as title,
                lesson_description as description,
                lesson_idx,
                NULL::text as thumbnail_url,
                lesson_state,
                lesson_access
            FROM lesson
            WHERE lesson_state = 'open'
            ORDER BY lesson_idx ASC
            LIMIT $1
            OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }

    pub async fn find_lesson_by_id(pool: &PgPool, lesson_id: i64) -> AppResult<Option<LessonMetaRow>> {
        let row = sqlx::query_as::<_, LessonMetaRow>(
            r#"
            SELECT
                lesson_id::bigint as lesson_id,
                lesson_title as title,
                lesson_description as description,
                lesson_state,
                lesson_access
            FROM lesson
            WHERE lesson_id = $1
            "#,
        )
        .bind(lesson_id)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    pub async fn exists_lesson(pool: &PgPool, lesson_id: i64) -> AppResult<bool> {
        let exists = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM lesson
                WHERE lesson_id = $1
            )
            "#,
        )
        .bind(lesson_id)
        .fetch_one(pool)
        .await?;

        Ok(exists)
    }

    /// 레슨 접근 권한 조회 (상태와 접근 레벨)
    pub async fn find_lesson_access(
        pool: &PgPool,
        lesson_id: i64,
    ) -> AppResult<Option<LessonAccessInfo>> {
        let row = sqlx::query_as::<_, LessonAccessInfo>(
            r#"
            SELECT
                lesson_state,
                lesson_access
            FROM lesson
            WHERE lesson_id = $1
            "#,
        )
        .bind(lesson_id)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// 사용자가 특정 레슨에 대한 수강권이 있는지 확인
    /// (Course 도메인 구현 후 user_course 테이블과 연동)
    pub async fn has_course_access(
        pool: &PgPool,
        user_id: i64,
        lesson_id: i64,
    ) -> AppResult<bool> {
        // user_course 테이블과 course_lesson 매핑을 통해 확인
        let has_access = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM user_course uc
                JOIN course_lesson cl ON cl.course_id = uc.course_id
                WHERE uc.user_id = $1
                  AND cl.lesson_id = $2
                  AND uc.user_course_active = true
                  AND (uc.user_course_expire_at IS NULL OR uc.user_course_expire_at > NOW())
            )
            "#,
        )
        .bind(user_id)
        .bind(lesson_id)
        .fetch_one(pool)
        .await?;

        Ok(has_access)
    }

    pub async fn count_items(pool: &PgPool, lesson_id: i64) -> AppResult<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM lesson_item
            WHERE lesson_id = $1
            "#,
        )
        .bind(lesson_id)
        .fetch_one(pool)
        .await?;

        Ok(count)
    }

    pub async fn find_items(
        pool: &PgPool,
        lesson_id: i64,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<LessonItemRes>> {
        let rows = sqlx::query_as::<_, LessonItemRes>(
            r#"
            SELECT
                lesson_item_seq as seq,
                lesson_item_kind as kind,
                video_id::bigint as video_id,
                study_task_id::bigint as task_id
            FROM lesson_item
            WHERE lesson_id = $1
            ORDER BY lesson_item_seq ASC
            LIMIT $2
            OFFSET $3
            "#,
        )
        .bind(lesson_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }

    pub async fn find_items_for_study_view(
        pool: &PgPool,
        lesson_id: i64,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<LessonItemDetailRes>> {
        let rows = sqlx::query_as::<_, LessonItemDetailRes>(
            r#"
            SELECT
                lesson_item_seq as seq,
                lesson_item_kind as kind,
                video_id::bigint as video_id,
                study_task_id::bigint as study_task_id
            FROM lesson_item
            WHERE lesson_id = $1
            ORDER BY lesson_item_seq ASC
            LIMIT $2
            OFFSET $3
            "#,
        )
        .bind(lesson_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }

    pub async fn find_progress(
        pool: &PgPool,
        lesson_id: i64,
        user_id: i64,
    ) -> AppResult<Option<LessonProgressRes>> {
        let row = sqlx::query_as::<_, LessonProgressRes>(
            r#"
            SELECT
                lesson_progress_percent as percent,
                lesson_progress_last_item_seq as last_seq,
                lesson_progress_last_progress_at as updated_at
            FROM lesson_progress
            WHERE lesson_id = $1
              AND user_id = $2
            "#,
        )
        .bind(lesson_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    pub async fn upsert_progress(
        pool: &PgPool,
        lesson_id: i64,
        user_id: i64,
        percent: i32,
        last_seq: Option<i32>,
    ) -> AppResult<LessonProgressRes> {
        let row = sqlx::query_as::<_, LessonProgressRes>(
            r#"
            INSERT INTO lesson_progress (
                lesson_id,
                user_id,
                lesson_progress_percent,
                lesson_progress_last_item_seq,
                lesson_progress_last_progress_at
            )
            VALUES ($1, $2, $3, $4, NOW())
            ON CONFLICT (lesson_id, user_id)
            DO UPDATE SET
                lesson_progress_percent = EXCLUDED.lesson_progress_percent,
                lesson_progress_last_item_seq = EXCLUDED.lesson_progress_last_item_seq,
                lesson_progress_last_progress_at = EXCLUDED.lesson_progress_last_progress_at
            RETURNING
                lesson_progress_percent as percent,
                lesson_progress_last_item_seq as last_seq,
                lesson_progress_last_progress_at as updated_at
            "#,
        )
        .bind(lesson_id)
        .bind(user_id)
        .bind(percent)
        .bind(last_seq)
        .fetch_one(pool)
        .await?;

        Ok(row)
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct LessonMetaRow {
    pub lesson_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub lesson_state: LessonState,
    pub lesson_access: LessonAccess,
}

#[derive(Debug, sqlx::FromRow)]
pub struct LessonAccessInfo {
    pub lesson_state: LessonState,
    pub lesson_access: LessonAccess,
}
