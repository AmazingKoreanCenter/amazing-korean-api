use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, PgPool, Row};

use crate::api::video::dto::{VideoInfo, VideoProgressRes};

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

    pub async fn count_open_videos(&self) -> Result<i64, sqlx::Error> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM video
            WHERE video_state = 'open'::video_state_enum
            "#,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(count)
    }

    pub async fn find_open_videos(
        &self,
        per_page: u64,
        offset: u64,
        sort: Option<&str>,
    ) -> Result<Vec<VideoInfo>, sqlx::Error> {
        let order_by = match sort {
            Some("created_at_asc") => "video_created_at ASC",
            Some("created_at_desc") | None => "video_created_at DESC",
            _ => "video_created_at DESC",
        };

        let sql = format!(
            r#"
            SELECT
                video.video_id::bigint as video_id,
                video.video_url_vimeo,
                video.video_state::text as video_state,
                video.video_access::text as video_access,
                COALESCE(
                    array_agg(video_tag.video_tag_key ORDER BY video_tag.video_tag_key)
                    FILTER (WHERE video_tag.video_tag_key IS NOT NULL),
                    '{{}}'::varchar[]
                ) as tags,
                video.video_created_at as created_at
            FROM video
            LEFT JOIN video_tag_map
                ON video_tag_map.video_id = video.video_id
            LEFT JOIN video_tag
                ON video_tag.video_tag_id = video_tag_map.video_tag_id
            WHERE video.video_state = 'open'::video_state_enum
            GROUP BY
                video.video_id,
                video.video_url_vimeo,
                video.video_state,
                video.video_access,
                video.video_created_at
            ORDER BY {order_by}
            LIMIT $1 OFFSET $2
            "#
        );

        let rows = sqlx::query_as::<_, VideoInfo>(&sql)
            .bind(per_page as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }
}
