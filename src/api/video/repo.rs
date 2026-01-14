use sqlx::{PgPool, QueryBuilder, Row};

use crate::api::video::dto::{
    VideoDetailRes, VideoListItem, VideoListReq, VideoProgressRes,
};
use crate::error::AppResult;

pub struct VideoRepo;

impl VideoRepo {
    // =========================================================================
    // List & Search (Dynamic Query)
    // =========================================================================

    /// 비디오 목록 조회 (검색, 필터, 페이징)
    pub async fn find_list_dynamic(
        pool: &PgPool,
        req: &VideoListReq,
    ) -> AppResult<(Vec<VideoListItem>, i64)> {
        // 1. Base Query
        let mut qb = QueryBuilder::new(
            r#"
            SELECT
                v.video_id,
                v.video_idx,
                v.video_title as title,
                v.video_subtitle as subtitle,
                v.video_duration as duration_seconds,
                v.video_language::text as language,
                v.video_thumbnail as thumbnail_url,
                v.video_state::text as state,
                v.video_access::text as access,
                v.video_has_captions as has_captions,
                v.video_created_at as created_at,
                COALESCE(
                    array_agg(vt.video_tag_key) FILTER (WHERE vt.video_tag_key IS NOT NULL),
                    '{}'::varchar[]
                ) as tags,
                COUNT(*) OVER() as total_count -- 전체 개수 (Window Function)
            FROM video v
            LEFT JOIN video_tag_map vtm ON vtm.video_id = v.video_id
            LEFT JOIN video_tag vt ON vt.video_tag_id = vtm.video_tag_id
            WHERE 1=1
            "#,
        );

        // 2. Dynamic Filters
        // 2-1. State (기본값: open, 관리자 등이 아니면 open만)
        if let Some(state) = &req.state {
             qb.push(" AND v.video_state = ");
             qb.push_bind(state);
             qb.push("::video_state_enum");
        } else {
             // 필터가 없으면 기본적으로 'open' 상태만 조회 (안전장치)
             qb.push(" AND v.video_state = 'open'::video_state_enum");
        }

        // 2-2. Search (Title or Subtitle)
        if let Some(q) = &req.q {
            if !q.trim().is_empty() {
                qb.push(" AND (v.video_title ILIKE ");
                qb.push_bind(format!("%{}%", q));
                qb.push(" OR v.video_subtitle ILIKE ");
                qb.push_bind(format!("%{}%", q));
                qb.push(" )");
            }
        }

        // 2-3. Language
        if let Some(lang) = &req.lang {
            qb.push(" AND v.video_language = ");
            qb.push_bind(lang);
            qb.push("::video_language_enum");
        }

        // 2-4. Tag Filter
        // (주의: Join된 상태에서 Where절을 걸면 Aggregation에 영향이 있을 수 있으나, 
        //  여기서는 검색된 영상의 '모든' 태그를 보여주기보다, 해당 태그가 포함된 영상을 찾는 것이 목적)
        //  정확한 동작: 해당 태그를 가진 Video를 찾고, 그 Video의 태그들을 모음 -> HAVING 절 또는 SubQuery 필요.
        //  간단한 구현을 위해 여기서는 WHERE EXISTS 사용으로 분리하는 것이 가장 정확함.
        if let Some(tag_key) = &req.tag {
            qb.push(" AND EXISTS (SELECT 1 FROM video_tag_map vtm2 JOIN video_tag vt2 ON vt2.video_tag_id = vtm2.video_tag_id WHERE vtm2.video_id = v.video_id AND vt2.video_tag_key = ");
            qb.push_bind(tag_key);
            qb.push(" )");
        }

        // 3. Group By (Aggregation for tags)
        qb.push(
            r#"
            GROUP BY
                v.video_id, v.video_idx, v.video_title, v.video_subtitle,
                v.video_duration, v.video_language, v.video_thumbnail,
                v.video_state, v.video_access, v.video_has_captions, v.video_created_at
            "#
        );

        // 4. Sort & Order
        let sort_column = match req.sort.as_deref() {
            Some("oldest") => "v.video_created_at ASC",
            Some("views") => "v.video_view_count DESC", // 뷰 카운트가 있다면
            _ => "v.video_created_at DESC", // default: latest
        };
        qb.push(format!(" ORDER BY {}", sort_column));

        // 5. Pagination
        let offset = (req.page - 1) * req.per_page;
        qb.push(" LIMIT ");
        qb.push_bind(req.per_page as i64);
        qb.push(" OFFSET ");
        qb.push_bind(offset as i64);

        // 6. Execute
        let rows = qb.build().fetch_all(pool).await?;

        // 7. Parse Result
        // total_count는 첫 번째 로우에서 추출 (데이터가 없으면 0)
        let total_count: i64 = rows.first().map(|r| r.try_get("total_count").unwrap_or(0)).unwrap_or(0);

        let list: Vec<VideoListItem> = rows.iter().map(|row| VideoListItem {
            video_id: row.get("video_id"),
            video_idx: row.get("video_idx"),
            title: row.get("title"),
            subtitle: row.get("subtitle"),
            duration_seconds: row.get("duration_seconds"),
            language: row.get("language"),
            thumbnail_url: row.get("thumbnail_url"),
            state: row.get("state"),
            access: row.get("access"),
            tags: row.get("tags"),
            has_captions: row.get("has_captions"),
            created_at: row.get("created_at"),
        }).collect();

        Ok((list, total_count))
    }

    // =========================================================================
    // Detail & Single Fetch
    // =========================================================================

    /// 비디오 상세 조회
    pub async fn find_detail_by_id(
        pool: &PgPool,
        video_id: i64,
    ) -> AppResult<Option<VideoDetailRes>> {
        let row = sqlx::query_as::<_, VideoDetailRes>(
            r#"
            SELECT
                v.video_id,
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
            LEFT JOIN video_tag_map vtm ON vtm.video_id = v.video_id
            LEFT JOIN video_tag vt ON vt.video_tag_id = vtm.video_tag_id
            WHERE v.video_id = $1
            GROUP BY v.video_id
            "#
        )
        .bind(video_id)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// 비디오 존재 여부 확인 (가벼운 쿼리)
    pub async fn exists_by_id(pool: &PgPool, video_id: i64) -> AppResult<bool> {
        let exists = sqlx::query_scalar::<_, bool>(
            r#"SELECT EXISTS(SELECT 1 FROM video WHERE video_id = $1)"#,
        )
        .bind(video_id)
        .fetch_one(pool)
        .await?;
        
        Ok(exists)
    }

    // =========================================================================
    // Progress (Learning Logs)
    // =========================================================================

    /// 내 학습 진도 조회
    pub async fn find_progress(
        pool: &PgPool,
        user_id: i64,
        video_id: i64,
    ) -> AppResult<Option<VideoProgressRes>> {
        let row = sqlx::query_as::<_, VideoProgressRes>(
            r#"
            SELECT
                video_id,
                COALESCE(video_progress_log, 0) AS video_progress_log,
                video_completed_log,
                video_last_watched_at_log
            FROM video_log
            WHERE user_id = $1 AND video_id = $2
            "#,
        )
        .bind(user_id)
        .bind(video_id)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// 학습 진도 업데이트 (Upsert)
    pub async fn upsert_progress(
        pool: &PgPool,
        user_id: i64,
        video_id: i64,
        progress_rate: i32,
        is_completed: bool,
    ) -> AppResult<VideoProgressRes> {
        let row = sqlx::query_as::<_, VideoProgressRes>(
            r#"
            INSERT INTO video_log (
                user_id, video_id, 
                video_progress_log, video_completed_log, video_last_watched_at_log
            )
            VALUES ($1, $2, $3, $4, NOW())
            ON CONFLICT (user_id, video_id) DO UPDATE
            SET
                video_progress_log = EXCLUDED.video_progress_log,
                video_completed_log = CASE 
                    WHEN video_log.video_completed_log = true THEN true 
                    ELSE EXCLUDED.video_completed_log 
                END, -- 한 번 완료되면 true 유지 (선택 사항)
                video_last_watched_at_log = NOW()
            RETURNING
                video_id,
                COALESCE(video_progress_log, 0) AS video_progress_log,
                video_completed_log,
                video_last_watched_at_log
            "#,
        )
        .bind(user_id)
        .bind(video_id)
        .bind(progress_rate)
        .bind(is_completed)
        .fetch_one(pool)
        .await?;

        Ok(row)
    }
}