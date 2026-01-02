use crate::api::admin::video::dto::{VideoAccess, VideoCreateReq, VideoRes, VideoState};
use crate::error::{AppError, AppResult};
use sqlx::Row;
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
