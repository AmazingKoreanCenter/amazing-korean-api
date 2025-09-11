use super::repo;
use crate::api::admin::video::dto::{VideoCreateReq, VideoRes};
use crate::error::{AppError, AppResult};
use crate::AppState;

pub async fn create_video(
    st: &AppState,
    req: VideoCreateReq,
    actor_user_id: i64,
) -> AppResult<VideoRes> {
    // 간단 검증(필요 시 실제 프로젝트 기준으로 추가 보완)
    if req.video_title.trim().is_empty() || req.video_title.trim().len() > 200 {
        return Err(AppError::BadRequest("video_title length 1..200".into()));
    }
    if let Some(d) = req.video_duration_seconds {
        if d <= 0 {
            return Err(AppError::BadRequest(
                "video_duration_seconds must be > 0".into(),
            ));
        }
    }
    // 기본값
    let state_s = req
        .video_state
        .map(|v| v.as_str().to_string())
        .unwrap_or_else(|| "draft".to_string());
    let access_s = req
        .video_access
        .map(|v| v.as_str().to_string())
        .unwrap_or_else(|| "private".to_string());

    repo::create_video(&st.db, &req, &state_s, &access_s, actor_user_id).await
}

pub async fn delete_video(st: &AppState, video_id: i64, actor_user_id: i64) -> AppResult<()> {
    repo::soft_delete_video(&st.db, video_id, actor_user_id).await
}
