use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, PgPool, Row};

use crate::api::video::dto::VideoProgressRes;

pub struct VideoRepo {
    pool: PgPool,
}

impl VideoRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_video_exists(&self, video_id: i64) -> Result<bool, sqlx::Error> {
        let exists = sqlx::query_scalar::<_, bool>(
            r#"SELECT EXISTS(SELECT 1 FROM video WHERE video_id = $1)"#,
        )
        .bind(video_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(exists)
    }

    pub async fn fetch_video_progress(
        &self,
        user_id: i64,
        video_id: i64,
    ) -> Result<Option<VideoProgressRes>, sqlx::Error> {
        let row_opt = sqlx::query(
            r#"
            SELECT
                video_id,
                user_id,
                last_position_seconds,
                total_duration_seconds,
                video_log_progress AS progress,
                video_log_completed AS completed,
                updated_at
            FROM video_log
            WHERE user_id = $1 AND video_id = $2
            "#,
        )
        .bind(user_id)
        .bind(video_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row_opt {
            let res = VideoProgressRes {
                video_id: row.try_get::<i64, _>("video_id")?,
                user_id: row.try_get::<i64, _>("user_id")?,
                last_position_seconds: row.try_get::<Option<i32>, _>("last_position_seconds")?,
                total_duration_seconds: row.try_get::<Option<i32>, _>("total_duration_seconds")?,
                progress: row.try_get::<Option<i32>, _>("progress")?,
                completed: row.try_get::<bool, _>("completed")?,
                last_watched_at: row.try_get::<DateTime<Utc>, _>("updated_at").ok(),
            };
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }

    /// DB 함수 호출 기반 업서트: api_upsert_video_progress($user,$video,$progress,$completed,$last,$total)
    /// 함수가 video_id를 반환하지 않는 경우가 있으므로, 상수 컬럼으로 주입한다.
    pub async fn upsert_video_progress(
        pool: &PgPool,
        user_id: i64,
        video_id: i64,
        progress: i32,
        completed: bool,
        last_position_seconds: i32,
        total_duration_seconds: Option<i32>,
    ) -> Result<PgRow, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT
              $2::bigint AS video_id,
              vl.video_log_progress   AS progress,
              vl.video_log_completed  AS completed,
              vl.last_position_seconds,
              vl.total_duration_seconds,
              vl.updated_at
            FROM api_upsert_video_progress($1,$2,$3,$4,$5,$6) AS vl
            "#,
        )
        .bind(user_id)
        .bind(video_id)
        .bind(progress)
        .bind(completed)
        .bind(last_position_seconds)
        .bind(total_duration_seconds)
        .fetch_one(pool)
        .await?;

        Ok(row)
    }
}
