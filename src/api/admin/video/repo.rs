use crate::api::admin::video::dto::{AdminVideoRes, VideoCreateReq, VideoUpdateReq};
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

pub async fn exists_video_idx_for_update(
    tx: &mut Transaction<'_, Postgres>,
    video_id: i64,
    video_idx: &str,
) -> AppResult<bool> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM video
            WHERE video_idx = $1
              AND video_id <> $2
        )
        "#,
    )
    .bind(video_idx)
    .bind(video_id)
    .fetch_one(&mut **tx)
    .await?;

    Ok(exists)
}

pub async fn exists_video_tag_key_for_update(
    tx: &mut Transaction<'_, Postgres>,
    video_id: i64,
    video_tag_key: &str,
) -> AppResult<bool> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM video_tag t
            JOIN video_tag_map m ON m.video_tag_id = t.video_tag_id
            WHERE t.video_tag_key = $1
              AND m.video_id <> $2
        )
        "#,
    )
    .bind(video_tag_key)
    .bind(video_id)
    .fetch_one(&mut **tx)
    .await?;

    Ok(exists)
}

pub async fn admin_update_video(
    tx: &mut Transaction<'_, Postgres>,
    video_id: i64,
    actor_user_id: i64,
    req: &VideoUpdateReq,
) -> AppResult<AdminVideoRes> {
    // -------------------------------------------------------------------------
    // 1. VIDEO 테이블 업데이트 (매뉴얼 콤마 관리 - 가장 안전함)
    // -------------------------------------------------------------------------
    // "UPDATE video SET "까지 만들고 시작
    let mut builder = QueryBuilder::<Postgres>::new("UPDATE video SET ");
    let mut is_first = true; 

    // (1) video_idx
    if let Some(ref idx) = req.video_idx {
        if !is_first { builder.push(", "); }
        builder.push("video_idx = ");
        builder.push_bind(idx);
        is_first = false;
    }

    // (2) video_access (Enum 캐스팅)
    if let Some(ref access) = req.video_access {
        if !is_first { builder.push(", "); }
        builder.push("video_access = ");
        builder.push_bind(access);
        builder.push("::video_access_enum"); // 문자열로 붙여서 캐스팅
        is_first = false;
    }

    // (3) video_url
    if let Some(ref url) = req.video_url_vimeo {
        if !is_first { builder.push(", "); }
        builder.push("video_url_vimeo = ");
        builder.push_bind(url);
        is_first = false;
    }

    // (4) 필수 업데이트 필드 (updated_by, updated_at)
    if !is_first { builder.push(", "); }
    builder.push("updated_by_user_id = ");
    builder.push_bind(actor_user_id);
    builder.push(", video_updated_at = now()"); 

    // WHERE 절
    builder.push(" WHERE video_id = ");
    builder.push_bind(video_id);
    
    // RETURNING
    builder.push(" RETURNING video_id::bigint, video_created_at, video_updated_at, video_access::text, video_url_vimeo");

    let video_row = builder.build().fetch_one(&mut **tx).await?;
    
    let v_id: i64 = video_row.try_get("video_id")?;
    let created_at = video_row.try_get("video_created_at")?;
    let updated_at = video_row.try_get("video_updated_at")?;
    let v_access: String = video_row.try_get("video_access")?;
    let v_url: String = video_row.try_get("video_url_vimeo")?;

    // -------------------------------------------------------------------------
    // 2. VIDEO_TAG 테이블 업데이트
    // -------------------------------------------------------------------------
    let has_tag_update = req.video_tag_title.is_some() 
        || req.video_tag_subtitle.is_some() 
        || req.video_tag_key.is_some();

    if has_tag_update {
        // 연결된 tag_id 찾기
        let map_row = sqlx::query("SELECT video_tag_id FROM video_tag_map WHERE video_id = $1")
            .bind(video_id)
            .fetch_optional(&mut **tx)
            .await?;

        if let Some(map_row) = map_row {
            let tag_id: i64 = map_row.try_get::<i32, _>("video_tag_id")? as i64; // i32 -> i64 변환

            let mut t_builder = QueryBuilder::<Postgres>::new("UPDATE video_tag SET ");
            let mut t_first = true;

            if let Some(ref key) = req.video_tag_key {
                if !t_first { t_builder.push(", "); }
                t_builder.push("video_tag_key = ");
                t_builder.push_bind(key);
                t_first = false;
            }
            if let Some(ref title) = req.video_tag_title {
                if !t_first { t_builder.push(", "); }
                t_builder.push("video_tag_title = ");
                t_builder.push_bind(title);
                t_first = false;
            }
            if let Some(ref sub) = req.video_tag_subtitle {
                if !t_first { t_builder.push(", "); }
                t_builder.push("video_tag_subtitle = ");
                t_builder.push_bind(sub);
            }

            t_builder.push(" WHERE video_tag_id = ");
            t_builder.push_bind(tag_id);

            t_builder.build().execute(&mut **tx).await?;
        }
        // else: 매핑된 태그가 없으면 업데이트 스킵 (Medium severity 이슈 - 일단은 무시하거나 404 처리 가능)
    }

    // -------------------------------------------------------------------------
    // 3. 최종 결과 조회 (Codex High Issue 해결: MAX 집계 사용)
    // -------------------------------------------------------------------------
    // GROUP BY에서 t.video_tag_title 등을 제외하고 MAX()를 써서 
    // 혹시라도 여러 태그가 매핑되어 있어도 1개의 Row만 반환되도록 보장합니다.
    let row = sqlx::query(
        r#"
        SELECT 
            v.video_id::bigint as id,
            COALESCE(MAX(t.video_tag_title), v.video_idx) as title,
            v.video_url_vimeo as url,
            MAX(t.video_tag_subtitle) as description,
            COALESCE(SUM(s.video_stat_views), 0)::bigint as views
        FROM video v
        LEFT JOIN video_tag_map m ON v.video_id = m.video_id
        LEFT JOIN video_tag t ON m.video_tag_id = t.video_tag_id
        LEFT JOIN video_stat_daily s ON v.video_id = s.video_id
        WHERE v.video_id = $1
        GROUP BY v.video_id, v.video_idx, v.video_url_vimeo
        "#
    )
    .bind(video_id)
    .fetch_one(&mut **tx)
    .await?;

    Ok(AdminVideoRes {
        id: v_id,
        title: row.try_get("title")?,
        url: Some(v_url),
        description: row.try_get("description").ok(),
        views: row.try_get("views")?,
        is_public: v_access == "public",
        created_at,
        updated_at,
    })
}
