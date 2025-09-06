use crate::{
    state::AppState,
    error::{AppResult, AppError},
    api::auth::jwt::Claims,
};
use validator::Validate;
use super::dto::{VideoCreateReq, VideoRes};
use super::repo;

// TODO: 운영 전 role/DB 체크로 교체
fn require_admin(_c: &Claims) -> Result<(), AppError> {
    Ok(())
}

pub async fn create_video(st: &AppState, admin: &Claims, req: VideoCreateReq) -> AppResult<VideoRes> {
    require_admin(admin)?;
    req.validate()?;

    repo::insert_video(&st.db, admin.sub, &req).await
}
