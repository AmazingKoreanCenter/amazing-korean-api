use sqlx::PgPool;

use super::dto::LessonRes;

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
}
