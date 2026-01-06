use crate::api::admin::video::dto::{AdminVideoRes, VideoCreateReq};
use crate::error::AppResult;
use sqlx::{PgPool, Postgres, QueryBuilder, Transaction, Row}; // [수정] Row 추가

pub async fn admin_list_videos(
    pool: &PgPool,
    q: Option<&str>,
    page: i64,
    size: i64,
    sort: &str,
    order: &str,
) -> AppResult<(i64, Vec<AdminVideoRes>)> {
    let search = q.map(|raw| format!("%{}%", raw.trim().to_lowercase()));

    // 1. 카운트 쿼리
    let mut count_builder = QueryBuilder::<Postgres>::new(
        "SELECT COUNT(DISTINCT v.video_id) 
         FROM video v
         LEFT JOIN video_tag_map m ON v.video_id = m.video_id
         LEFT JOIN video_tag t ON m.video_tag_id = t.video_tag_id"
    );
    
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

    // 2. 목록 조회 쿼리
    let mut list_builder = QueryBuilder::<Postgres>::new(
        r#"
        SELECT 
            v.video_id::bigint AS id,
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

    list_builder.push(" GROUP BY v.video_id, v.video_idx, v.video_url_vimeo, v.video_access, v.video_created_at, v.video_updated_at ");

    let order_by = match sort {
        "views" => "views",
        "title" => "MAX(t.video_tag_title)",
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

pub async fn exists_video_idx(pool: &PgPool, video_idx: &str) -> AppResult<bool> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1 
            FROM video 
            WHERE video_idx = $1
        )
        "#,
    )
    .bind(video_idx)
    .fetch_one(pool)
    .await?;

    Ok(exists)
}

pub async fn admin_create_video(
    tx: &mut Transaction<'_, Postgres>,
    actor_user_id: i64,
    req: &VideoCreateReq,
    video_idx: &str,
    video_tag_key: &str,
) -> AppResult<AdminVideoRes> {
    // 1. VIDEO 테이블 Insert
    let video_row = sqlx::query(
        r#"
        INSERT INTO video (
            updated_by_user_id,
            video_idx,
            video_state,
            video_access,
            video_url_vimeo
        )
        VALUES ($1, $2, 'ready'::video_state_enum, $3::video_access_enum, $4)
        RETURNING 
            video_id::bigint,        -- [타입 캐스팅] i64
            video_created_at, 
            video_updated_at, 
            video_access::text,      -- [타입 캐스팅] Enum -> String
            video_url_vimeo
        "#,
    )
    .bind(actor_user_id)
    .bind(video_idx)
    .bind(&req.video_access)         // "public" or "private"
    .bind(req.video_url_vimeo.trim())
    .fetch_one(&mut **tx)
    .await?;

    let video_id: i64 = video_row.try_get("video_id")?;
    let created_at = video_row.try_get("video_created_at")?;
    let updated_at = video_row.try_get("video_updated_at")?;
    let video_access: String = video_row.try_get("video_access")?;
    let video_url: String = video_row.try_get("video_url_vimeo")?;

    // 2. VIDEO_TAG 테이블 Insert
    let tag_row = sqlx::query(
        r#"
        INSERT INTO video_tag (
            video_tag_key,
            video_tag_title,
            video_tag_subtitle
        )
        VALUES ($1, $2, $3)
        RETURNING 
            video_tag_id::bigint,     -- [타입 캐스팅] i64
            video_tag_title, 
            video_tag_subtitle
        "#,
    )
    .bind(video_tag_key)
    .bind(req.video_tag_title.trim())
    .bind(req.video_tag_subtitle.as_deref())
    .fetch_one(&mut **tx)
    .await?;

    let video_tag_id: i64 = tag_row.try_get("video_tag_id")?;
    let title: String = tag_row.try_get("video_tag_title")?;
    let description: Option<String> = tag_row.try_get("video_tag_subtitle").ok();

    // 3. VIDEO_TAG_MAP 테이블 Insert
    sqlx::query(
        r#"
        INSERT INTO video_tag_map (video_id, video_tag_id)
        VALUES ($1, $2)
        "#,
    )
    .bind(video_id)
    .bind(video_tag_id)
    .execute(&mut **tx)
    .await?;

    Ok(AdminVideoRes {
        id: video_id,
        title,
        url: Some(video_url),
        description,
        views: 0,
        is_public: video_access == "public",
        created_at,
        updated_at,
    })
}