use crate::api::admin::video::dto::{AdminVideoRes, VideoAccess, VideoCreateReq, VideoState, VideoUpdateReq};
use std::str::FromStr;
use crate::error::AppResult;
use serde_json::Value;
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
            v.video_state::text AS video_state,
            v.video_access::text AS video_access,
            v.video_idx AS video_idx,
            MAX(t.video_tag_key) AS video_tag_key,
            v.updated_by_user_id::bigint AS updated_by_user_id,
            v.video_created_at AS created_at,
            v.video_updated_at AS updated_at,
            v.video_duration AS video_duration,
            v.video_thumbnail AS video_thumbnail
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

    list_builder.push(" GROUP BY v.video_id, v.video_idx, v.video_url_vimeo, v.video_state, v.video_access, v.updated_by_user_id, v.video_created_at, v.video_updated_at, v.video_duration, v.video_thumbnail ");

    let order_by = match sort {
        "id" => "v.video_id",
        "views" => "views",
        "title" => "MAX(t.video_tag_title)",
        "video_state" => "v.video_state",
        "video_access" => "v.video_access",
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

    let rows = list_builder
        .build()
        .fetch_all(pool)
        .await?;

    let items: Vec<AdminVideoRes> = rows
        .into_iter()
        .map(|row| {
            let state_str: String = row.try_get("video_state").unwrap_or_default();
            let access_str: String = row.try_get("video_access").unwrap_or_default();

            AdminVideoRes {
                id: row.try_get("id").unwrap_or(0),
                title: row.try_get("title").unwrap_or_default(),
                url: row.try_get("url").ok(),
                description: row.try_get("description").ok(),
                views: row.try_get("views").unwrap_or(0),
                video_state: VideoState::from_str(&state_str).unwrap_or(VideoState::Ready),
                video_access: VideoAccess::from_str(&access_str).unwrap_or(VideoAccess::Private),
                video_idx: row.try_get("video_idx").unwrap_or_default(),
                video_tag_key: row.try_get("video_tag_key").ok(),
                updated_by_user_id: Some(row.try_get::<i64, _>("updated_by_user_id").unwrap_or(0)),
                created_at: row.try_get("created_at").unwrap_or_default(),
                updated_at: row.try_get("updated_at").unwrap_or_default(),
                video_duration: row.try_get("video_duration").ok(),
                video_thumbnail: row.try_get("video_thumbnail").ok(),
            }
        })
        .collect();

    Ok((total_count, items))
}

pub async fn admin_get_video(pool: &PgPool, video_id: i64) -> AppResult<Option<AdminVideoRes>> {
    let row = sqlx::query(
        r#"
        SELECT
            v.video_id::bigint AS id,
            COALESCE(MAX(t.video_tag_title), v.video_idx) AS title,
            v.video_url_vimeo AS url,
            MAX(t.video_tag_subtitle) AS description,
            COALESCE(SUM(stats.views), 0)::bigint AS views,
            v.video_state::text AS video_state,
            v.video_access::text AS video_access,
            v.video_idx AS video_idx,
            MAX(t.video_tag_key) AS video_tag_key,
            v.updated_by_user_id::bigint AS updated_by_user_id,
            v.video_created_at AS created_at,
            v.video_updated_at AS updated_at,
            v.video_duration AS video_duration,
            v.video_thumbnail AS video_thumbnail
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
        WHERE v.video_id = $1
        GROUP BY v.video_id, v.video_idx, v.video_url_vimeo, v.video_state, v.video_access, v.updated_by_user_id, v.video_created_at, v.video_updated_at, v.video_duration, v.video_thumbnail
        "#,
    )
    .bind(video_id)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(row) => {
            let state_str: String = row.try_get("video_state").unwrap_or_default();
            let access_str: String = row.try_get("video_access").unwrap_or_default();

            Ok(Some(AdminVideoRes {
                id: row.try_get("id").unwrap_or(0),
                title: row.try_get("title").unwrap_or_default(),
                url: row.try_get("url").ok(),
                description: row.try_get("description").ok(),
                views: row.try_get("views").unwrap_or(0),
                video_state: VideoState::from_str(&state_str).unwrap_or(VideoState::Ready),
                video_access: VideoAccess::from_str(&access_str).unwrap_or(VideoAccess::Private),
                video_idx: row.try_get("video_idx").unwrap_or_default(),
                video_tag_key: row.try_get("video_tag_key").ok(),
                updated_by_user_id: Some(row.try_get::<i64, _>("updated_by_user_id").unwrap_or(0)),
                created_at: row.try_get("created_at").unwrap_or_default(),
                updated_at: row.try_get("updated_at").unwrap_or_default(),
                video_duration: row.try_get("video_duration").ok(),
                video_thumbnail: row.try_get("video_thumbnail").ok(),
            }))
        }
        None => Ok(None),
    }
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
    // video_state 기본값: ready
    let video_state = req.video_state.as_deref().unwrap_or("ready");

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
        VALUES ($1, $2, $3::video_state_enum, $4::video_access_enum, $5)
        RETURNING
            video_id::bigint,
            video_created_at,
            video_updated_at,
            video_state::text,
            video_access::text,
            video_url_vimeo
        "#,
    )
    .bind(actor_user_id)
    .bind(video_idx)
    .bind(video_state)
    .bind(&req.video_access)
    .bind(req.video_url_vimeo.trim())
    .fetch_one(&mut **tx)
    .await?;

    let video_id: i64 = video_row.try_get("video_id")?;
    let created_at = video_row.try_get("video_created_at")?;
    let updated_at = video_row.try_get("video_updated_at")?;
    let v_state: String = video_row.try_get("video_state")?;
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
        video_state: VideoState::from_str(&v_state).unwrap_or(VideoState::Ready),
        video_access: VideoAccess::from_str(&video_access).unwrap_or(VideoAccess::Private),
        video_idx: video_idx.to_string(),
        video_tag_key: Some(video_tag_key.to_string()),
        updated_by_user_id: Some(actor_user_id),
        created_at,
        updated_at,
        video_duration: None, // Vimeo 동기화 후 업데이트됨
        video_thumbnail: None, // Vimeo 동기화 후 업데이트됨
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

    // (3) video_state (Enum 캐스팅)
    if let Some(ref state) = req.video_state {
        if !is_first { builder.push(", "); }
        builder.push("video_state = ");
        builder.push_bind(state);
        builder.push("::video_state_enum");
        is_first = false;
    }

    // (4) video_url
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
    builder.push(" RETURNING video_id::bigint, video_created_at, video_updated_at, video_state::text, video_access::text, video_url_vimeo, video_idx, updated_by_user_id");

    let video_row = builder.build().fetch_one(&mut **tx).await?;

    let v_id: i64 = video_row.try_get("video_id")?;
    let created_at = video_row.try_get("video_created_at")?;
    let updated_at = video_row.try_get("video_updated_at")?;
    let v_state: String = video_row.try_get("video_state")?;
    let v_access: String = video_row.try_get("video_access")?;
    let v_url: String = video_row.try_get("video_url_vimeo")?;
    let v_idx: String = video_row.try_get("video_idx")?;
    let v_updated_by: Option<i64> = video_row.try_get("updated_by_user_id").ok();

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
            COALESCE(SUM(s.video_stat_views), 0)::bigint as views,
            MAX(t.video_tag_key) as video_tag_key,
            v.video_duration as video_duration,
            v.video_thumbnail as video_thumbnail
        FROM video v
        LEFT JOIN video_tag_map m ON v.video_id = m.video_id
        LEFT JOIN video_tag t ON m.video_tag_id = t.video_tag_id
        LEFT JOIN video_stat_daily s ON v.video_id = s.video_id
        WHERE v.video_id = $1
        GROUP BY v.video_id, v.video_idx, v.video_url_vimeo, v.video_duration, v.video_thumbnail
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
        video_state: VideoState::from_str(&v_state).unwrap_or(VideoState::Ready),
        video_access: VideoAccess::from_str(&v_access).unwrap_or(VideoAccess::Private),
        video_idx: v_idx,
        video_tag_key: row.try_get("video_tag_key").ok(),
        updated_by_user_id: v_updated_by,
        created_at,
        updated_at,
        video_duration: row.try_get("video_duration").ok(),
        video_thumbnail: row.try_get("video_thumbnail").ok(),
    })
}

fn normalize_video_action(action: &str) -> &'static str {
    match action {
        "create" | "CREATE" | "bulk_create" | "BULK_CREATE" => "create",
        _ => "update",
    }
}

pub async fn create_video_log_tx(
    tx: &mut Transaction<'_, Postgres>,
    admin_user_id: i64,
    action: &str,
    video_id: Option<i64>,
    video_tag_id: Option<i64>,
    before: Option<&Value>,
    after: Option<&Value>,
) -> AppResult<()> {
    sqlx::query(
        r#"
        INSERT INTO admin_video_log (
            admin_user_id,
            admin_pick_video_id,
            admin_pick_video_tag_id,
            admin_video_action,
            admin_video_before,
            admin_video_after
        )
        VALUES ($1, $2, $3, CAST($4 AS admin_action_enum), $5, $6)
        "#,
    )
    .bind(admin_user_id)
    .bind(video_id)
    .bind(video_tag_id)
    .bind(normalize_video_action(action))
    .bind(before)
    .bind(after)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn create_video_log(
    pool: &PgPool,
    admin_user_id: i64,
    action: &str,
    video_id: Option<i64>,
    video_tag_id: Option<i64>,
    before: Option<&Value>,
    after: Option<&Value>,
) -> AppResult<()> {
    sqlx::query(
        r#"
        INSERT INTO admin_video_log (
            admin_user_id,
            admin_pick_video_id,
            admin_pick_video_tag_id,
            admin_video_action,
            admin_video_before,
            admin_video_after
        )
        VALUES ($1, $2, $3, CAST($4 AS admin_action_enum), $5, $6)
        "#,
    )
    .bind(admin_user_id)
    .bind(video_id)
    .bind(video_tag_id)
    .bind(normalize_video_action(action))
    .bind(before)
    .bind(after)
    .execute(pool)
    .await?;

    Ok(())
}

/// Vimeo 메타데이터로 video, video_tag 테이블 업데이트
pub async fn update_vimeo_meta(
    tx: &mut Transaction<'_, Postgres>,
    video_id: i64,
    duration: i32,
    thumbnail_url: Option<&str>,
    title: &str,
    description: Option<&str>,
) -> AppResult<()> {
    // 1. video 테이블에 duration, thumbnail 업데이트
    sqlx::query(
        r#"
        UPDATE video
        SET video_duration = $2,
            video_thumbnail = $3
        WHERE video_id = $1
        "#,
    )
    .bind(video_id)
    .bind(duration)
    .bind(thumbnail_url)
    .execute(&mut **tx)
    .await?;

    // 2. video_tag 테이블에 title, subtitle 업데이트
    sqlx::query(
        r#"
        UPDATE video_tag t
        SET video_tag_title = $2,
            video_tag_subtitle = $3
        FROM video_tag_map m
        WHERE m.video_tag_id = t.video_tag_id
          AND m.video_id = $1
        "#,
    )
    .bind(video_id)
    .bind(title)
    .bind(description)
    .execute(&mut **tx)
    .await?;

    Ok(())
}
