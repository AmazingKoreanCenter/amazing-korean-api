use super::repo;
use crate::api::admin::video::dto::{
    AdminVideoListReq, AdminVideoListRes, Pagination, VideoCreateReq, VideoRes,
};
use crate::error::{AppError, AppResult};
use crate::AppState;
use crate::types::UserAuth;
use std::net::IpAddr;
use validator::Validate;

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

async fn check_admin_rbac(pool: &sqlx::PgPool, actor_user_id: i64) -> AppResult<UserAuth> {
    let actor = crate::api::user::repo::find_user(pool, actor_user_id)
        .await?
        .ok_or(AppError::Unauthorized("Actor user not found".into()))?;

    let actor_auth: UserAuth = actor.user_auth;
    match actor_auth {
        UserAuth::Hymn | UserAuth::Admin | UserAuth::Manager => Ok(actor_auth),
        _ => Err(AppError::Forbidden),
    }
}

pub async fn admin_list_videos(
    st: &AppState,
    actor_user_id: i64,
    req: AdminVideoListReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminVideoListRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    let page = req.page.unwrap_or(1);
    if page < 1 {
        return Err(AppError::BadRequest("page must be >= 1".into()));
    }

    let size = req.size.unwrap_or(20);
    if size < 1 {
        return Err(AppError::BadRequest("size must be >= 1".into()));
    }
    if size > 100 {
        return Err(AppError::Unprocessable("size exceeds 100".into()));
    }

    let sort = req.sort.as_deref().unwrap_or("created_at");
    if !matches!(sort, "created_at" | "views" | "title") {
        return Err(AppError::Unprocessable("invalid sort".into()));
    }

    let order = req.order.as_deref().unwrap_or("desc");
    if !matches!(order, "asc" | "desc") {
        return Err(AppError::Unprocessable("invalid order".into()));
    }

    let details = serde_json::json!({
        "q": req.q.as_deref(),
        "page": page,
        "size": size,
        "sort": sort,
        "order": order
    });

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "LIST_VIDEOS",
        Some("video"),
        None,
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    let (total_count, items) =
        repo::admin_list_videos(&st.db, req.q.as_deref(), page, size, sort, order).await?;

    let total_pages = if total_count == 0 {
        0
    } else {
        (total_count + size - 1) / size
    };

    Ok(AdminVideoListRes {
        items,
        pagination: Pagination {
            total_count,
            total_pages,
            current_page: page,
            per_page: size,
        },
    })
}
