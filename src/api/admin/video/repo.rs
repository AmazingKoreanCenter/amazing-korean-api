use crate::error::{AppError, AppResult};
use sqlx::PgPool;

/// 비디오 소프트 삭제(멱등). 없으면 404.
/// 이미 deleted_at이 있으면 값 유지 + updated_*만 갱신.
pub async fn soft_delete_video(db: &PgPool, video_id: i64, actor_user_id: i64) -> AppResult<()> {
    let res = sqlx::query(
        r#"
        UPDATE video
        SET deleted_at = COALESCE(deleted_at, now()),
            updated_by_user_id = $2,
            updated_at = now()
        WHERE video_id = $1
        "#,
    )
    .bind(video_id)
    .bind(actor_user_id)
    .execute(db)
    .await?;

    if res.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}
