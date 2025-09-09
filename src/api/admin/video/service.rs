use super::repo;
use crate::error::AppResult;
use crate::AppState;

pub async fn delete_video(st: &AppState, video_id: i64, actor_user_id: i64) -> AppResult<()> {
    repo::soft_delete_video(&st.db, video_id, actor_user_id).await
}
