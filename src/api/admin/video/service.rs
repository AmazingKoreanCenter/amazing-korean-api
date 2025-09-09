use super::dto::{VideoCreateReq, VideoRes, VideoUpdateReq};
use super::repo;
use crate::{
    api::auth::jwt::Claims,
    error::{AppError, AppResult},
    state::AppState,
};
use validator::Validate;

// TODO: 운영 전 role/DB 체크로 교체
fn require_admin(_c: &Claims) -> Result<(), AppError> {
    Ok(())
}

pub async fn create_video(
    st: &AppState,
    admin: &Claims,
    req: VideoCreateReq,
) -> AppResult<VideoRes> {
    require_admin(admin)?;
    req.validate()?;

    repo::insert_video(&st.db, admin.sub, &req).await
}

pub async fn update_video(
    st: &AppState,
    admin: &Claims,
    id: i64,
    req: VideoUpdateReq,
) -> AppResult<VideoRes> {
    require_admin(admin)?;
    req.validate()?;

    repo::update_video(&st.db, id, &req, admin.sub).await
}
