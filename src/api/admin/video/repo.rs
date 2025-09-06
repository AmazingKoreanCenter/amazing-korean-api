use sqlx::{PgPool, Row};
use crate::error::{AppResult, AppError};
use super::dto::{VideoCreateReq, VideoRes};

pub async fn insert_video(pool: &PgPool, admin_user_id: i64, req: &VideoCreateReq) -> AppResult<VideoRes> {
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
        "#
    )
    .bind(admin_user_id)
    .bind(&req.video_state)
    .bind(&req.video_access)
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
        video_state: row.try_get("video_state")?,
        video_access: row.try_get("video_access")?,
        vimeo_video_id: row.try_get("vimeo_video_id")?,
        video_duration_seconds: row.try_get("video_duration_seconds")?,
        video_thumbnail_url: row.try_get("video_thumbnail_url")?,
        video_link: row.try_get("video_link")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        deleted_at: row.try_get("deleted_at")?,
    })
}
