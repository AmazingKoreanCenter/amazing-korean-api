use crate::api::admin::video::dto::{
    AdminVideoRes, VideoAccess, VideoCreateReq, VideoRes, VideoState,
};
use crate::error::{AppError, AppResult};
use sqlx::{PgPool, Postgres, QueryBuilder, Row};
use std::str::FromStr;

pub async fn create_video(
    db: &sqlx::PgPool,
    req: &VideoCreateReq,
    state_s: &str,
    access_s: &str,
    actor_user_id: i64,
) -> AppResult<VideoRes> {
    let row = sqlx::query(
        r#"
        INSERT INTO video (
            video_title,
            video_state,
            video_access,
            vimeo_video_id,
            video_duration_seconds,
            video_thumbnail_url,
            video_language,
            video_link,
            updated_by_user_id
        ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
        RETURNING
            video_id,
            video_title,
            video_state,
            video_access,
            vimeo_video_id,
            video_duration_seconds,
            video_thumbnail_url,
            video_language,
            video_link,
            created_at,
            updated_at,
            updated_by_user_id,
            deleted_at
        "#,
    )
    .bind(req.video_title.trim())
    .bind(state_s)
    .bind(access_s)
    .bind(req.vimeo_video_id.as_deref())
    .bind(req.video_duration_seconds)
    .bind(req.video_thumbnail_url.as_deref())
    .bind(req.video_language.as_deref())
    .bind(req.video_link.as_deref())
    .bind(actor_user_id)
    .fetch_one(db)
    .await?;

    let video_state_str: String = row.try_get("video_state")?;
    let video_access_str: String = row.try_get("video_access")?;

    Ok(VideoRes {
        video_id: row.try_get("video_id")?,
        video_title: row.try_get("video_title")?,
        video_subtitle: row.try_get("video_subtitle").ok(),
        video_language: row.try_get("video_language").ok(),
        video_state: VideoState::from_str(&video_state_str).map_err(AppError::Internal)?,
        video_access: VideoAccess::from_str(&video_access_str).map_err(AppError::Internal)?,
        vimeo_video_id: row.try_get("vimeo_video_id").ok(),
        video_duration_seconds: row.try_get("video_duration_seconds").ok(),
        video_thumbnail_url: row.try_get("video_thumbnail_url").ok(),
        video_link: row.try_get("video_link").ok(),
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        updated_by_user_id: row.try_get("updated_by_user_id")?,
        deleted_at: row.try_get("deleted_at").ok(),
    })
}

pub async fn admin_list_videos(
    pool: &PgPool,
    q: Option<&str>,
    page: i64,
    size: i64,
    sort: &str,
    order: &str,
) -> AppResult<(i64, Vec<AdminVideoRes>)> {
    let search = q.map(|raw| format!("%{}%", raw.trim().to_lowercase()));

    // 1. 카운트 쿼리 (JOIN 포함)
    // 비디오 하나에 태그가 여러 개일 수 있으므로 DISTINCT v.video_id가 필요합니다.
    let mut count_builder = QueryBuilder::<Postgres>::new(
        "SELECT COUNT(DISTINCT v.video_id) 
         FROM video v
         LEFT JOIN video_tag_map m ON v.video_id = m.video_id
         LEFT JOIN video_tag t ON m.video_tag_id = t.video_tag_id"
    );
    
    // 검색 조건 (태그의 제목/설명에서 검색)
    if let Some(ref search) = search {
        count_builder.push(" WHERE (LOWER(t.video_tag_title) LIKE ");
        count_builder.push_bind(search);
        count_builder.push(" OR LOWER(t.video_tag_subtitle) LIKE ");
        count_builder.push_bind(search);
        count_builder.push(")");
    }

    let total_count = count_builder
        .build_query_scalar::<i64>()
        .fetch_one(pool)
        .await?;

    // 2. 목록 조회 쿼리 (JOIN + GROUP BY)
    // 비디오 하나에 태그가 여러 개일 때, 목록에서는 '가장 최근 태그' 혹은 '아무거나 하나'를 제목으로 보여줘야 중복이 안 생깁니다.
    // 여기서는 MAX()를 사용하여 하나의 제목만 가져오도록 처리합니다.
    let mut list_builder = QueryBuilder::<Postgres>::new(
        r#"
        SELECT 
            v.video_id::bigint AS id,
            
            -- 제목이 없으면 video_idx라도 보여주도록 처리
            COALESCE(MAX(t.video_tag_title), v.video_idx) AS title,
            
            v.video_url_vimeo AS url,
            MAX(t.video_tag_subtitle) AS description,
            
            COALESCE(SUM(stats.views), 0)::bigint AS views,
            
            (v.video_access = 'public'::video_access_enum) AS is_public,
            v.video_created_at AS created_at,
            v.video_updated_at AS updated_at
        FROM video v
        LEFT JOIN video_tag_map m ON v.video_id = m.video_id
        LEFT JOIN video_tag t ON m.video_tag_id = t.video_tag_id
        LEFT JOIN (
            SELECT 
                video_id, 
                SUM(video_stat_views) AS views 
            FROM video_stat_daily 
            GROUP BY video_id
        ) stats ON stats.video_id = v.video_id
        "#
    );

    if let Some(ref search) = search {
        list_builder.push(" WHERE (LOWER(t.video_tag_title) LIKE ");
        list_builder.push_bind(search);
        list_builder.push(" OR LOWER(t.video_tag_subtitle) LIKE ");
        list_builder.push_bind(search);
        list_builder.push(")");
    }

    // GROUP BY 필수 (집계 함수 MAX, SUM 사용 때문)
    list_builder.push(" GROUP BY v.video_id, v.video_idx, v.video_url_vimeo, v.video_access, v.video_created_at, v.video_updated_at ");

    // 정렬 조건 매핑
    let order_by = match sort {
        "views" => "views",
        "title" => "MAX(t.video_tag_title)", // 집계 함수로 정렬
        _ => "v.video_created_at",
    };
    let order_dir = match order {
        "asc" => "ASC",
        _ => "DESC",
    };

    list_builder.push(" ORDER BY ");
    list_builder.push(order_by);
    list_builder.push(" ");
    list_builder.push(order_dir);
    list_builder.push(" LIMIT ");
    list_builder.push_bind(size);
    list_builder.push(" OFFSET ");
    list_builder.push_bind((page - 1) * size);

    let items = list_builder
        .build_query_as::<AdminVideoRes>()
        .fetch_all(pool)
        .await?;

    Ok((total_count, items))
}