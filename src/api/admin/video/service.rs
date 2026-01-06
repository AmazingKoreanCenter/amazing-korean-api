use super::repo;
use crate::api::admin::video::dto::{
    AdminVideoListReq, AdminVideoListRes, AdminVideoRes, Pagination, VideoBulkCreateReq,
    VideoBulkCreateRes, VideoBulkItemError, VideoBulkItemResult, VideoBulkSummary,
    VideoCreateReq, VideoUpdateReq,
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

    let (video_idx, tag_key) = build_video_keys(&req);

    if repo::exists_video_idx(&st.db, &video_idx).await? {
        return Err(AppError::Conflict("video_idx already exists".into()));
    }

    let mut tx = st.db.begin().await?;

    // Repo 호출 시 변경된 변수명 전달
    let created = repo::admin_create_video(
        &mut tx,
        actor_user_id,
        &req,
        &video_idx,
        &tag_key,
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
        "video_tag_key": tag_key
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

pub async fn admin_bulk_create_videos(
    st: &AppState,
    actor_user_id: i64,
    req: VideoBulkCreateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<(bool, VideoBulkCreateRes)> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    let mut results = Vec::with_capacity(req.items.len());
    let mut success = 0i64;
    let mut failure = 0i64;

    for item in req.items {
        let outcome = async {
            if let Err(e) = item.validate() {
                return Err(AppError::BadRequest(e.to_string()));
            }

            let (video_idx, tag_key) = build_video_keys(&item);

            if repo::exists_video_idx(&st.db, &video_idx).await? {
                return Err(AppError::Conflict("video_idx already exists".into()));
            }

            let mut tx = st.db.begin().await?;

            let created =
                repo::admin_create_video(&mut tx, actor_user_id, &item, &video_idx, &tag_key).await;

            let created = match created {
                Ok(val) => val,
                Err(e) if is_unique_violation(&e) => {
                    return Err(AppError::Conflict("duplicate video data".into()));
                }
                Err(e) => return Err(e),
            };

            tx.commit().await?;

            Ok(created)
        }
        .await;

        match outcome {
            Ok(video) => {
                success += 1;
                results.push(VideoBulkItemResult {
                    id: Some(video.id),
                    status: 201,
                    data: Some(video),
                    error: None,
                });
            }
            Err(e) => {
                failure += 1;
                let (status, msg) = match e {
                    AppError::BadRequest(m) => (400, m),
                    AppError::Unprocessable(m) => (422, m),
                    AppError::Conflict(m) => (409, m),
                    AppError::Forbidden => (403, "Forbidden".to_string()),
                    _ => (500, "Internal Server Error".to_string()),
                };

                results.push(VideoBulkItemResult {
                    id: None,
                    status,
                    data: None,
                    error: Some(VideoBulkItemError {
                        code: "ERR".into(),
                        message: msg,
                    }),
                });
            }
        }
    }

    let summary = VideoBulkSummary {
        total: success + failure,
        success,
        failure,
    };

    let details = serde_json::json!({
        "total": summary.total,
        "success": summary.success,
        "failure": summary.failure
    });

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "BULK_CREATE_VIDEO",
        Some("video"),
        None,
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    let all_success = failure == 0;

    Ok((all_success, VideoBulkCreateRes { summary, results }))
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

pub async fn admin_update_video(
    st: &AppState,
    actor_user_id: i64,
    video_id: i64,
    req: VideoUpdateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminVideoRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    let has_any = req.video_tag_title.is_some()
        || req.video_tag_subtitle.is_some()
        || req.video_tag_key.is_some()
        || req.video_url_vimeo.is_some()
        || req.video_access.is_some()
        || req.video_idx.is_some();

    if !has_any {
        return Err(AppError::BadRequest("no fields to update".into()));
    }

    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM video
            WHERE video_id = $1
        )
        "#,
    )
    .bind(video_id)
    .fetch_one(&st.db)
    .await?;

    if !exists {
        return Err(AppError::NotFound);
    }

    let mut tx = st.db.begin().await?;

    if let Some(video_idx) = req.video_idx.as_deref() {
        if repo::exists_video_idx_for_update(&mut tx, video_id, video_idx.trim()).await? {
            return Err(AppError::Conflict("video_idx already exists".into()));
        }
    }

    if let Some(tag_key) = req.video_tag_key.as_deref() {
        if repo::exists_video_tag_key_for_update(&mut tx, video_id, tag_key.trim()).await? {
            return Err(AppError::Conflict("video_tag_key already exists".into()));
        }
    }

    let updated = repo::admin_update_video(&mut tx, video_id, actor_user_id, &req).await;

    let updated = match updated {
        Ok(val) => val,
        Err(e) if is_unique_violation(&e) => {
            return Err(AppError::Conflict("duplicate video data".into()));
        }
        Err(e) => return Err(e),
    };

    let details = serde_json::json!({
        "video_id": updated.id
    });

    crate::api::admin::user::repo::create_audit_log_tx(
        &mut tx,
        actor_user_id,
        "UPDATE_VIDEO",
        Some("video"),
        Some(updated.id),
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    tx.commit().await?;

    Ok(updated)
}

fn build_video_keys(req: &VideoCreateReq) -> (String, String) {
    let video_idx = req
        .video_idx
        .as_deref()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| format!("video_{}", Uuid::new_v4()));

    let tag_key = req
        .video_tag_key
        .as_deref()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| {
            let uuid = Uuid::new_v4().to_string();
            format!("tag_{}", &uuid[..20])
        });

    (video_idx, tag_key)
}
