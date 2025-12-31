use sqlx::PgPool;

use crate::api::video::dto::{VideoDetailRes, VideoInfo, VideoProgressRes};

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
        let row_opt = sqlx::query_as::<_, VideoProgressRes>(
            r#"
            SELECT
                video_id::bigint as video_id,
                COALESCE(video_progress_log, 0) AS video_progress_log,
                video_completed_log,
                video_last_watched_at_log
            FROM video_log
            WHERE user_id = $1 AND video_id = $2
            "#,
        )
        .bind(user_id)
        .bind(video_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row_opt)
    }

    pub async fn upsert_progress_log(
        &self,
        user_id: i64,
        video_id: i64,
        progress_rate: i32,
        is_completed: bool,
    ) -> Result<VideoProgressRes, sqlx::Error> {
        let row = sqlx::query_as::<_, VideoProgressRes>(
            r#"
            INSERT INTO video_log (
                user_id,
                video_id,
                video_progress_log,
                video_completed_log,
                video_last_watched_at_log
            )
            VALUES ($1, $2, $3, $4, NOW())
            ON CONFLICT (user_id, video_id) DO UPDATE
            SET
                video_progress_log = EXCLUDED.video_progress_log,
                video_completed_log = EXCLUDED.video_completed_log,
                video_last_watched_at_log = NOW()
            RETURNING
                video_id::bigint as video_id,
                COALESCE(video_progress_log, 0) AS video_progress_log,
                video_completed_log,
                video_last_watched_at_log
            "#,
        )
        .bind(user_id)
        .bind(video_id)
        .bind(progress_rate)
        .bind(is_completed)
        .fetch_one(&self.pool)
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

    pub async fn find_video_by_id(
        &self,
        video_id: i64,
    ) -> Result<Option<VideoDetailRes>, sqlx::Error> {
        let row = sqlx::query_as::<_, VideoDetailRes>(
            r#"
            SELECT
                v.video_id::bigint as video_id,
                v.video_url_vimeo,
                v.video_state::text as video_state,
                COALESCE(
                    jsonb_agg(
                        jsonb_build_object(
                            'key', vt.video_tag_key,
                            'title', vt.video_tag_title,
                            'subtitle', vt.video_tag_subtitle
                        )
                    ) FILTER (WHERE vt.video_tag_id IS NOT NULL),
                    '[]'::jsonb
                ) as tags,
                v.video_created_at as created_at
            FROM video v
            LEFT JOIN video_tag_map vtm
                ON vtm.video_id = v.video_id
            LEFT JOIN video_tag vt
                ON vt.video_tag_id = vtm.video_tag_id
            WHERE v.video_id = $1
            GROUP BY
                v.video_id,
                v.video_url_vimeo,
                v.video_state,
                v.video_created_at
            "#,
        )
        .bind(video_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
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
