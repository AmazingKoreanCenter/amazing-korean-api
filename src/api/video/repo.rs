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
                -- 1. [중요] INT4 -> INT8 캐스팅 (패닉 방지)
                v.video_id::bigint as video_id,
                v.video_idx,
                
                -- 2. 1:N 관계에서 제목 가져오기 (MAX 사용)
                MAX(vt.video_tag_title) as title,
                MAX(vt.video_tag_subtitle) as subtitle,
                
                -- 3. DB에 없는 컬럼 NULL/기본값 처리
                NULL::integer as duration_seconds,
                NULL::text as language,
                NULL::text as thumbnail_url,
                false as has_captions,
                
                v.video_state::text as state,
                v.video_access::text as access,
                v.video_created_at as created_at,
                
                -- 태그 집계
                COALESCE(
                    array_agg(vt.video_tag_key) FILTER (WHERE vt.video_tag_key IS NOT NULL),
                    '{}'::varchar[]
                ) as tags,
                
                COUNT(*) OVER() as total_count
            FROM video v
            LEFT JOIN video_tag_map vtm ON vtm.video_id = v.video_id
            LEFT JOIN video_tag vt ON vt.video_tag_id = vtm.video_tag_id
            WHERE 1=1
            "#,
        );

        // 2. Dynamic Filters
        // 2-1. State
        if let Some(state) = &req.state {
            qb.push(" AND v.video_state = ");
            qb.push_bind(state);
            qb.push("::video_state_enum");
        } else {
            qb.push(" AND v.video_state = 'open'::video_state_enum");
        }

        // 2-2. Search (Tag Title/Subtitle 기준)
        if let Some(q) = &req.q {
            if !q.trim().is_empty() {
                qb.push(" AND (vt.video_tag_title ILIKE ");
                qb.push_bind(format!("%{}%", q));
                qb.push(" OR vt.video_tag_subtitle ILIKE ");
                qb.push_bind(format!("%{}%", q));
                qb.push(" )");
            }
        }

        // 2-3. Language (DB 컬럼이 없으므로 필터 기능도 작동 안 함 - 무시하거나 에러처리)
        // 일단 DB에 없으므로 조건절 추가하지 않음 (필요 시 나중에 컬럼 추가 후 복구)
        // if let Some(lang) = &req.lang { ... } 

        // 2-4. Tag Filter
        if let Some(tag_key) = &req.tag {
            qb.push(" AND vt.video_tag_key = ");
            qb.push_bind(tag_key);
        }

        // 3. Group By
        // 집계 함수(MAX, ARRAY_AGG)를 제외한 컬럼들만 GROUP BY
        qb.push(
            r#"
            GROUP BY
                v.video_id, v.video_idx,
                v.video_state, v.video_access, v.video_created_at
            "#
        );

        // 4. Sort & Order
        // views 등 없는 컬럼 정렬 제거
        let sort_column = match req.sort.as_deref() {
            Some("oldest") => "v.video_created_at ASC",
            _ => "v.video_created_at DESC",
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
        let total_count: i64 = rows.first().map(|r| r.try_get("total_count").unwrap_or(0)).unwrap_or(0);

        let list: Vec<VideoListItem> = rows.iter().map(|row| VideoListItem {
            video_id: row.get("video_id"),
            video_idx: row.get("video_idx"),
            title: row.get("title"),
            subtitle: row.get("subtitle"),
            duration_seconds: row.get("duration_seconds"), // NULL
            language: row.get("language"),                 // NULL
            thumbnail_url: row.get("thumbnail_url"),       // NULL
            state: row.get("state"),
            access: row.get("access"),
            tags: row.get("tags"),
            has_captions: row.get("has_captions"),         // false
            created_at: row.get("created_at"),
        }).collect();

        Ok((list, total_count))
    }

    // =========================================================================
    // Detail & Single Fetch
    // =========================================================================

    /// 비디오 상세 조회 (공개 상태인 영상만)
    pub async fn find_detail_by_id(
        pool: &PgPool,
        video_id: i64,
    ) -> AppResult<Option<VideoDetailRes>> {
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
            LEFT JOIN video_tag_map vtm ON vtm.video_id = v.video_id
            LEFT JOIN video_tag vt ON vt.video_tag_id = vtm.video_tag_id
            WHERE v.video_id = $1
              AND v.video_state = 'open'
            GROUP BY v.video_id
            "#
        )
        .bind(video_id)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// 비디오 존재 여부 확인 (공개 상태인 영상만)
    pub async fn exists_by_id(pool: &PgPool, video_id: i64) -> AppResult<bool> {
        let exists = sqlx::query_scalar::<_, bool>(
            r#"SELECT EXISTS(SELECT 1 FROM video WHERE video_id = $1 AND video_state = 'open')"#,
        )
        .bind(video_id)
        .fetch_one(pool)
        .await?;

        Ok(exists)
    }

    // =========================================================================
    // Progress (Learning Logs)
    // =========================================================================
    
    // 내 학습 진도 조회(이전과 동일)
    pub async fn find_progress(
        pool: &PgPool,
        user_id: i64,
        video_id: i64,
    ) -> AppResult<Option<VideoProgressRes>> {
        let row = sqlx::query_as::<_, VideoProgressRes>(
            r#"
            SELECT
                video_id::bigint as video_id, -- [여기 수정]
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

    /// 학습 진도 업데이트 (Upsert) - 확장 버전
    /// is_new_view: true면 watch_count++, first_watched_at 설정
    pub async fn upsert_progress(
        pool: &PgPool,
        user_id: i64,
        video_id: i64,
        progress_rate: i32,
        is_completed: bool,
        is_new_view: bool,
    ) -> AppResult<VideoProgressRes> {
        let row = sqlx::query_as::<_, VideoProgressRes>(
            r#"
            INSERT INTO video_log (
                user_id, video_id,
                video_progress_log, video_completed_log, video_last_watched_at_log,
                video_watch_count_log, video_first_watched_at_log
            )
            VALUES ($1, $2, $3, $4, NOW(),
                CASE WHEN $5 THEN 1 ELSE 0 END,
                CASE WHEN $5 THEN NOW() ELSE NULL END
            )
            ON CONFLICT (user_id, video_id) DO UPDATE
            SET
                video_progress_log = EXCLUDED.video_progress_log,
                video_completed_log = CASE
                    WHEN video_log.video_completed_log = true THEN true
                    ELSE EXCLUDED.video_completed_log
                END,
                video_last_watched_at_log = NOW(),
                video_watch_count_log = CASE
                    WHEN $5 THEN video_log.video_watch_count_log + 1
                    ELSE video_log.video_watch_count_log
                END
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
        .bind(is_new_view)
        .fetch_one(pool)
        .await?;
        Ok(row)
    }

    // =========================================================================
    // Daily Stats (video_stat_daily)
    // =========================================================================

    /// 일별 통계 views 증가 (UPSERT)
    pub async fn increment_daily_views(
        pool: &PgPool,
        video_id: i64,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO video_stat_daily (video_stat_date, video_id, video_stat_views, video_stat_completes)
            VALUES (CURRENT_DATE, $1, 1, 0)
            ON CONFLICT (video_stat_date, video_id) DO UPDATE
            SET video_stat_views = video_stat_daily.video_stat_views + 1
            "#,
        )
        .bind(video_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// 일별 통계 completes 증가 (UPSERT)
    pub async fn increment_daily_completes(
        pool: &PgPool,
        video_id: i64,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO video_stat_daily (video_stat_date, video_id, video_stat_views, video_stat_completes)
            VALUES (CURRENT_DATE, $1, 0, 1)
            ON CONFLICT (video_stat_date, video_id) DO UPDATE
            SET video_stat_completes = video_stat_daily.video_stat_completes + 1
            "#,
        )
        .bind(video_id)
        .execute(pool)
        .await?;
        Ok(())
    }
}