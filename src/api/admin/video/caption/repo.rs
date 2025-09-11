use super::dto::{CaptionCreateReq, CaptionRes, CaptionUpdateReq};
use crate::error::{AppError, AppResult};
use sqlx::PgPool;

pub async fn create_caption(
    db: &PgPool,
    video_id: i64,
    req: &CaptionCreateReq,
    actor_user_id: i64,
) -> AppResult<CaptionRes> {
    let mut tx = db.begin().await?;

    if req.is_default {
        // Reset other captions' is_default for this video
        sqlx::query(
            r#"
            UPDATE video_caption
            SET is_default = FALSE,
                updated_at = now(),
                updated_by_user_id = $2
            WHERE video_id = $1 AND is_default = TRUE
            "#,
        )
        .bind(video_id)
        .bind(actor_user_id)
        .execute(&mut *tx)
        .await?;
    }

    let caption = sqlx::query_as::<_, CaptionRes>(
        r#"
        INSERT INTO video_caption (
            video_id, lang_code, kind, url, format, is_default, updated_by_user_id
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (video_id, lang_code, kind) WHERE deleted_at IS NULL DO UPDATE
        SET url = EXCLUDED.url,
            format = EXCLUDED.format,
            is_default = EXCLUDED.is_default,
            updated_at = now(),
            updated_by_user_id = EXCLUDED.updated_by_user_id
        RETURNING caption_id, video_id, lang_code, kind, url, format, is_default, is_active, created_at, updated_at, updated_by_user_id
        "#,
    )
    .bind(video_id)
    .bind(&req.lang_code)
    .bind(req.kind)
    .bind(&req.url)
    .bind(&req.format)
    .bind(req.is_default)
    .bind(actor_user_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(db_err) if db_err.code().is_some_and(|c| c == "23505") => {
            AppError::Conflict("Caption with same lang_code and kind already exists for this video.".into())
        }
        _ => AppError::Sqlx(e),
    })?;

    tx.commit().await?;

    Ok(caption)
}

pub async fn update_caption(
    db: &PgPool,
    video_id: i64,
    caption_id: i64,
    req: &CaptionUpdateReq,
    actor_user_id: i64,
) -> AppResult<CaptionRes> {
    let mut tx = db.begin().await?;

    if let Some(true) = req.is_default {
        // Reset other captions' is_default for this video
        sqlx::query(
            r#"
            UPDATE video_caption
            SET is_default = FALSE,
                updated_at = now(),
                updated_by_user_id = $2
            WHERE video_id = $1 AND is_default = TRUE AND caption_id != $3
            "#,
        )
        .bind(video_id)
        .bind(actor_user_id)
        .bind(caption_id)
        .execute(&mut *tx)
        .await?;
    }

    let caption = sqlx::query_as::<_, CaptionRes>(
        r#"
        UPDATE video_caption
        SET
            lang_code = COALESCE($1, lang_code),
            kind = COALESCE($2, kind),
            url = COALESCE($3, url),
            format = COALESCE($4, format),
            is_default = COALESCE($5, is_default),
            is_active = COALESCE($6, is_active),
            updated_at = now(),
            updated_by_user_id = $7
        WHERE video_id = $8 AND caption_id = $9 AND deleted_at IS NULL
        RETURNING caption_id, video_id, lang_code, kind, url, format, is_default, is_active, created_at, updated_at, updated_by_user_id
        "#,
    )
    .bind(req.lang_code.as_ref())
    .bind(req.kind)
    .bind(req.url.as_ref())
    .bind(req.format.as_ref())
    .bind(req.is_default)
    .bind(req.is_active)
    .bind(actor_user_id)
    .bind(video_id)
    .bind(caption_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::NotFound,
        sqlx::Error::Database(db_err) if db_err.code().is_some_and(|c| c == "23505") => {
            AppError::Conflict("Caption with same lang_code and kind already exists for this video.".into())
        }
        _ => AppError::Sqlx(e),
    })?;

    tx.commit().await?;

    Ok(caption)
}

pub async fn soft_delete_caption(
    db: &PgPool,
    video_id: i64,
    caption_id: i64,
    actor_user_id: i64,
) -> AppResult<()> {
    let mut tx = db.begin().await?;

    let res = sqlx::query(
        r#"
        UPDATE video_caption
        SET deleted_at = COALESCE(deleted_at, now()),
            updated_at = now(),
            updated_by_user_id = $3
        WHERE video_id = $1 AND caption_id = $2 AND deleted_at IS NULL
        "#,
    )
    .bind(video_id)
    .bind(caption_id)
    .bind(actor_user_id)
    .execute(&mut *tx)
    .await?;

    if res.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    tx.commit().await?;

    Ok(())
}
