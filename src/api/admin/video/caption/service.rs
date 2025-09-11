use super::dto::{CaptionCreateReq, CaptionRes, CaptionUpdateReq};
use super::repo;
use crate::error::{AppError, AppResult};
use crate::AppState;

fn check_len(value: &Option<String>, min: usize, max: usize, field_name: &str) -> AppResult<()> {
    if let Some(s) = value {
        if s.len() < min || s.len() > max {
            return Err(AppError::BadRequest(format!(
                "{} length must be between {} and {} characters.",
                field_name, min, max
            )));
        }
    }
    Ok(())
}

pub async fn create_caption(
    st: &AppState,
    video_id: i64,
    req: CaptionCreateReq,
    actor_user_id: i64,
) -> AppResult<CaptionRes> {
    // Basic validation already handled by validator derive macro on CaptionCreateReq
    // Additional business logic validation can go here if needed

    repo::create_caption(&st.db, video_id, &req, actor_user_id).await
}

pub async fn update_caption(
    st: &AppState,
    video_id: i64,
    caption_id: i64,
    req: CaptionUpdateReq,
    actor_user_id: i64,
) -> AppResult<CaptionRes> {
    check_len(&req.lang_code, 2, 8, "lang_code")?;
    check_len(&req.url, 1, 1024, "url")?;
    check_len(&req.format, 0, 16, "format")?;

    repo::update_caption(&st.db, video_id, caption_id, &req, actor_user_id).await
}

pub async fn delete_caption(
    st: &AppState,
    video_id: i64,
    caption_id: i64,
    actor_user_id: i64,
) -> AppResult<()> {
    repo::soft_delete_caption(&st.db, video_id, caption_id, actor_user_id).await
}
