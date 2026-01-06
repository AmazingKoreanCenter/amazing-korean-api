use super::repo;
use crate::api::admin::video::dto::{
    AdminVideoListReq, AdminVideoListRes, AdminVideoRes, Pagination, VideoCreateReq,
};
use crate::error::{AppError, AppResult};
use crate::types::UserAuth;
use crate::AppState;
use std::net::IpAddr;
use uuid::Uuid;
use validator::Validate;

const PG_UNIQUE_VIOLATION: &str = "23505";

fn is_unique_violation(err: &AppError) -> bool {
    if let AppError::Sqlx(sqlx::Error::Database(db)) = err {
        db.code().as_deref() == Some(PG_UNIQUE_VIOLATION)
    } else {
        false
    }
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

pub async fn admin_create_video(
    st: &AppState,
    actor_user_id: i64,
    req: VideoCreateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminVideoRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    // 1. video_idx 자동 생성 (변경 없음)
    let video_idx = req
        .video_idx
        .as_deref()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| format!("video_{}", Uuid::new_v4()));

    // 중복 체크
    if repo::exists_video_idx(&st.db, &video_idx).await? {
        return Err(AppError::Conflict("video_idx already exists".into()));
    }

    // 2. video_tag_key 자동 생성 & 길이 제한 (30자 이하로)
    // tag_(4자) + UUID앞20자 = 24자 -> VARCHAR(30) 안전
    let video_tag_key = req
        .video_tag_key
        .as_deref()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| {
            let uuid = Uuid::new_v4().to_string();
            format!("tag_{}", &uuid[..20])
        });

    let mut tx = st.db.begin().await?;

    // Repo 호출 시 변경된 변수명 전달
    let created = repo::admin_create_video(
        &mut tx,
        actor_user_id,
        &req,
        &video_idx,
        &video_tag_key,
    )
    .await;

    let created = match created {
        Ok(val) => val,
        Err(e) if is_unique_violation(&e) => {
            return Err(AppError::Conflict("duplicate video data".into()));
        }
        Err(e) => return Err(e),
    };

    // 로그 데이터도 컬럼명에 맞게 저장
    let details = serde_json::json!({
        "video_id": created.id,
        "video_idx": video_idx,
        "video_tag_key": video_tag_key
    });

    crate::api::admin::user::repo::create_audit_log_tx(
        &mut tx,
        actor_user_id,
        "CREATE_VIDEO",
        Some("video"),
        Some(created.id),
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    tx.commit().await?;

    Ok(created)
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