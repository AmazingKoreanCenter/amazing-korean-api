use sqlx::PgPool;

use super::dto::{LessonItemRes, LessonRes};

pub struct LessonRepo {
    pool: PgPool,
}

impl LessonRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn count_all(&self) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM lesson
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }

    pub async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<LessonRes>, sqlx::Error> {
        let rows = sqlx::query_as::<_, LessonRes>(
            r#"
            SELECT
                lesson_id::bigint as id,
                lesson_title as title,
                lesson_description as description,
                lesson_idx,
                NULL::text as thumbnail_url
            FROM lesson
            ORDER BY lesson_idx ASC
            LIMIT $1
            OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn find_lesson_by_id(
        &self,
        lesson_id: i64,
    ) -> Result<Option<LessonMetaRow>, sqlx::Error> {
        let row = sqlx::query_as::<_, LessonMetaRow>(
            r#"
            SELECT
                lesson_id::bigint as lesson_id,
                lesson_title as title,
                lesson_description as description
            FROM lesson
            WHERE lesson_id = $1
            "#,
        )
        .bind(lesson_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    pub async fn count_items(&self, lesson_id: i64) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM lesson_item
            WHERE lesson_id = $1
            "#,
        )
        .bind(lesson_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }

    pub async fn find_items(
        &self,
        lesson_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<LessonItemRes>, sqlx::Error> {
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
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct LessonMetaRow {
    pub lesson_id: i64,
    pub title: String,
    pub description: Option<String>,
}
