use super::dto::{VideoAccess, VideoCreateReq, VideoRes, VideoState, VideoUpdateReq};
use crate::error::{AppError, AppResult};
use sqlx::{PgPool, Row};
use std::str::FromStr;

pub async fn insert_video(
    pool: &PgPool,
    admin_user_id: i64,
    req: &VideoCreateReq,
) -> AppResult<VideoRes> {
    let row = sqlx::query(
        r#"
        INSERT INTO video(
            updated_by_user_id, video_state, video_access,
            video_title, video_subtitle, vimeo_video_id, video_duration_seconds,
            video_thumbnail_url, video_language, video_link
        )
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
        RETURNING video_id, video_title, video_subtitle, video_language, video_state, video_access,
                  vimeo_video_id, video_duration_seconds, video_thumbnail_url, video_link,
                  created_at, updated_at, deleted_at
        "#,
    )
    .bind(admin_user_id)
    .bind(req.video_state.as_str())
    .bind(req.video_access.as_str())
    .bind(&req.video_title)
    .bind(&req.video_subtitle)
    .bind(&req.vimeo_video_id)
    .bind(req.video_duration_seconds)
    .bind(&req.video_thumbnail_url)
    .bind(&req.video_language)
    .bind(&req.video_link)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        eprintln!("Error inserting video: {:?}", e);
        AppError::Sqlx(e)
    })?;

    Ok(VideoRes {
        video_id: row.try_get("video_id")?,
        video_title: row.try_get("video_title")?,
        video_subtitle: row.try_get("video_subtitle")?,
        video_language: row.try_get("video_language")?,
        video_state: VideoState::from_str(&row.try_get::<String, _>("video_state")?)
            .map_err(|e| AppError::Internal(e.to_string()))?,
        video_access: VideoAccess::from_str(&row.try_get::<String, _>("video_access")?)
            .map_err(|e| AppError::Internal(e.to_string()))?,
        vimeo_video_id: row.try_get("vimeo_video_id")?,
        video_duration_seconds: row.try_get("video_duration_seconds")?,
        video_thumbnail_url: row.try_get("video_thumbnail_url")?,
        video_link: row.try_get("video_link")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        deleted_at: row.try_get("deleted_at")?,
    })
}

pub async fn update_video(
    pool: &PgPool,
    id: i64,
    req: &VideoUpdateReq,
    updated_by: i64,
) -> AppResult<VideoRes> {
    let state_s = req.video_state.as_ref().map(|v| v.as_str().to_string());
    let access_s = req.video_access.as_ref().map(|v| v.as_str().to_string());

    let row = sqlx::query(
        r#"
        UPDATE video
        SET
            video_title = COALESCE($1, video_title),
            video_state = COALESCE($2, video_state),
            video_access = COALESCE($3, video_access),
            vimeo_video_id = COALESCE($4, vimeo_video_id),
            video_duration_seconds = COALESCE($5, video_duration_seconds),
            video_thumbnail_url = COALESCE($6, video_thumbnail_url),
            video_language = COALESCE($7, video_language),
            video_link = COALESCE($8, video_link),
            updated_by_user_id = $9,
            updated_at = now()
        WHERE video_id = $10 AND deleted_at IS NULL
        RETURNING video_id, video_title, video_subtitle, video_language, video_state, video_access,
                  vimeo_video_id, video_duration_seconds, video_thumbnail_url, video_link,
                  created_at, updated_at, deleted_at
        "#,
    )
    .bind(req.video_title.as_ref().map(|s| s.trim().to_string()))
    .bind(state_s)
    .bind(access_s)
    .bind(req.vimeo_video_id.as_ref().map(|s| s.trim().to_string()))
    .bind(req.video_duration_seconds)
    .bind(
        req.video_thumbnail_url
            .as_ref()
            .map(|s| s.trim().to_string()),
    )
    .bind(req.video_language.as_ref().map(|s| s.trim().to_string()))
    .bind(req.video_link.as_ref().map(|s| s.trim().to_string()))
    .bind(updated_by)
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        eprintln!("Error updating video: {:?}", e);
        AppError::Sqlx(e)
    })?;

    let row = row.ok_or(AppError::NotFound)?;

    Ok(VideoRes {
        video_id: row.try_get("video_id")?,
        video_title: row.try_get("video_title")?,
        video_subtitle: row.try_get("video_subtitle")?,
        video_language: row.try_get("video_language")?,
        video_state: VideoState::from_str(&row.try_get::<String, _>("video_state")?)
            .map_err(|e| AppError::Internal(e.to_string()))?,
        video_access: VideoAccess::from_str(&row.try_get::<String, _>("video_access")?)
            .map_err(|e| AppError::Internal(e.to_string()))?,
        vimeo_video_id: row.try_get("vimeo_video_id")?,
        video_duration_seconds: row.try_get("video_duration_seconds")?,
        video_thumbnail_url: row.try_get("video_thumbnail_url")?,
        video_link: row.try_get("video_link")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        deleted_at: row.try_get("deleted_at")?,
    })
}
