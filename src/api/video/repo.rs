use sqlx::{PgPool, QueryBuilder, Result};

use super::dto::{CaptionItem, VideoDetail, VideoListItem, VideosQuery};

#[derive(Clone)]
#[allow(dead_code)]
pub struct VideoRepo {
    pool: PgPool,
}

impl VideoRepo {
    #[allow(dead_code)]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn fetch_videos(&self, q: &VideosQuery) -> Result<Vec<VideoListItem>> {
        let mut query_builder = QueryBuilder::new(
            r#"SELECT
                v.video_id,
                v.video_idx,
                v.video_title AS title,
                v.video_subtitle AS subtitle,
                v.video_duration_seconds AS duration_seconds,
                v.video_language AS language,
                v.video_thumbnail_url AS thumbnail_url,
                v.video_state::TEXT AS state,
                v.video_access::TEXT AS access,
                COALESCE(array_agg(DISTINCT t.tag_key) FILTER (WHERE t.tag_key IS NOT NULL), ARRAY[]::text[]) AS tags,
                EXISTS(SELECT 1 FROM video_caption c WHERE c.video_id = v.video_id AND c.is_active = TRUE) AS has_captions,
                v.video_created_at AS created_at
            FROM video v
            LEFT JOIN video_tag_map vtm ON v.video_id = vtm.video_id
            LEFT JOIN video_tag t ON vtm.tag_id = t.tag_id
            WHERE 1 = 1
            "#,
        );

        // Default filters
        query_builder
            .push(" AND v.video_state = ")
            .push_bind("open");
        query_builder
            .push(" AND v.video_access = ")
            .push_bind("public");

        if let Some(search_query) = &q.q {
            query_builder
                .push(" AND (v.video_title ILIKE ")
                .push_bind(format!("%{}%", search_query))
                .push(" OR v.video_subtitle ILIKE ")
                .push_bind(format!("%{}%", search_query))
                .push(")");
        }

        if let Some(tags) = &q.tag {
            query_builder
                .push(" AND t.tag_key = ANY(")
                .push_bind(tags)
                .push(")");
        }

        if let Some(lang) = &q.lang {
            query_builder
                .push(" AND v.video_language = ")
                .push_bind(lang);
        }

        if let Some(access) = &q.access {
            if !access.is_empty() {
                query_builder
                    .push(" AND v.video_access = ")
                    .push_bind(access);
            }
        }

        if let Some(state) = &q.state {
            if !state.is_empty() {
                query_builder.push(" AND v.video_state = ").push_bind(state);
            }
        }

        query_builder.push(" GROUP BY v.video_id");

        let order_by = q.sort.as_deref().unwrap_or("created_at");
        let order_direction = q.order.as_deref().unwrap_or("desc");

        match order_by {
            "created_at" => {
                query_builder.push(format!(" ORDER BY v.video_created_at {}", order_direction));
            }
            // TODO: Implement popular and complete_rate sorting in subsequent stages
            _ => {
                query_builder.push(format!(" ORDER BY v.video_created_at {}", order_direction));
            }
        }

        query_builder.push(" LIMIT ").push_bind(q.limit);
        query_builder.push(" OFFSET ").push_bind(q.offset);

        let query = query_builder.build_query_as::<VideoListItem>();

        query.fetch_all(&self.pool).await
    }

    pub async fn fetch_video_detail(&self, id: i64) -> Result<Option<VideoDetail>> {
        let row = sqlx::query!(
            r#"SELECT
                v.video_id,
                v.video_idx AS "video_idx!: String",
                v.video_title AS title,
                v.video_subtitle AS subtitle,
                v.video_state::TEXT AS "state!: String",
                v.video_access::TEXT AS "access!: String",
                v.video_duration_seconds AS duration_seconds,
                v.video_language AS language,
                v.video_thumbnail_url AS thumbnail_url,
                v.vimeo_video_id,
                v.video_created_at AS created_at,
                COALESCE(array_agg(DISTINCT t.tag_key) FILTER (WHERE t.tag_key IS NOT NULL), ARRAY[]::text[])::text[] AS tags,
                (EXISTS(SELECT 1 FROM video_caption c WHERE c.video_id = v.video_id AND c.is_active = TRUE))::boolean AS has_captions
            FROM video v
            LEFT JOIN video_tag_map m ON m.video_id = v.video_id
            LEFT JOIN video_tag t ON t.video_tag_id = m.video_tag_id
            WHERE v.video_id = $1
            AND v.deleted_at IS NULL
            GROUP BY v.video_id
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| VideoDetail {
            video_id: r.video_id,
            video_idx: r.video_idx,
            title: r.title,
            subtitle: r.subtitle,
            state: r.state,
            access: r.access,
            duration_seconds: r.duration_seconds,
            language: r.language,
            thumbnail_url: r.thumbnail_url,
            vimeo_video_id: r.vimeo_video_id,
            tags: r.tags,
            has_captions: r.has_captions,
            created_at: r.created_at,
        }))
    }

    pub async fn fetch_video_status(&self, id: i64) -> Result<Option<(String, String)>> {
        let row = sqlx::query!(
            r#"SELECT video_state::TEXT AS "state!: String", video_access::TEXT AS "access!: String" FROM video WHERE video_id = $1 AND deleted_at IS NULL"#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| (r.state, r.access)))
    }

    pub async fn fetch_video_captions(&self, id: i64) -> Result<Vec<CaptionItem>> {
        sqlx::query_as!(
            CaptionItem,
            r#"SELECT
                caption_id,
                lang_code,
                label,
                kind::TEXT AS "kind!: String",
                is_default,
                is_active
            FROM video_caption
            WHERE video_id = $1 AND is_active = TRUE
            ORDER BY is_default DESC, lang_code ASC
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await
    }
}
