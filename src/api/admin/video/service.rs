use super::repo;
use crate::api::admin::video::dto::{
    AdminVideoListReq, AdminVideoListRes, AdminVideoRes, Pagination, VideoBulkCreateReq,
    VideoBulkCreateRes, VideoBulkItemError, VideoBulkItemResult, VideoBulkSummary,
    VideoBulkUpdateReq, VideoBulkUpdateRes, VideoBulkUpdateItemResult, VideoCreateReq,
    VideoTagBulkUpdateReq, VideoTagUpdateReq, VideoUpdateReq, VimeoPreviewRes,
    VimeoUploadTicketReq, VimeoUploadTicketRes,
};
use crate::error::{AppError, AppResult};
use crate::external::vimeo::VimeoClient;
use crate::types::UserAuth;
use crate::AppState;
use sqlx::{Postgres, Transaction};
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

/// Vimeo URL에서 메타데이터를 가져와 DB에 저장
async fn sync_vimeo_meta(
    st: &AppState,
    tx: &mut Transaction<'_, Postgres>,
    video_id: i64,
    vimeo_url: &str,
) -> AppResult<()> {
    // 1. Access Token 확인
    let access_token = match &st.cfg.vimeo_access_token {
        Some(token) if !token.is_empty() => token.clone(),
        _ => {
            tracing::warn!("Vimeo access token not configured, skipping metadata sync");
            return Ok(());
        }
    };

    // 2. Vimeo Video ID 추출
    let vimeo_video_id = match VimeoClient::extract_video_id(vimeo_url) {
        Some(id) => id,
        None => {
            tracing::warn!("Could not extract Vimeo video ID from URL: {}", vimeo_url);
            return Ok(());
        }
    };

    // 3. Vimeo API 호출
    let client = VimeoClient::new(access_token);
    let meta = match client.get_video_meta(&vimeo_video_id).await {
        Ok(m) => m,
        Err(e) => {
            tracing::error!("Failed to fetch Vimeo metadata: {:?}", e);
            // Vimeo API 실패해도 영상 생성/수정은 진행
            return Ok(());
        }
    };

    // 4. DB 업데이트
    repo::update_vimeo_meta(
        tx,
        video_id,
        meta.duration,
        meta.thumbnail_url.as_deref(),
        &meta.name,
        meta.description.as_deref(),
    )
    .await?;

    tracing::info!(
        "Synced Vimeo metadata for video_id={}: duration={}, title={}",
        video_id,
        meta.duration,
        meta.name
    );

    Ok(())
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

/// Vimeo URL에서 메타데이터 미리보기 (DB 저장 없음)
pub async fn admin_get_vimeo_preview(
    st: &AppState,
    actor_user_id: i64,
    url: &str,
) -> AppResult<VimeoPreviewRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    // 1. Access Token 확인
    let access_token = st
        .cfg
        .vimeo_access_token
        .as_ref()
        .filter(|t| !t.is_empty())
        .ok_or_else(|| AppError::BadRequest("Vimeo access token not configured".into()))?;

    // 2. Vimeo Video ID 추출
    let vimeo_video_id = VimeoClient::extract_video_id(url)
        .ok_or_else(|| AppError::BadRequest("Invalid Vimeo URL".into()))?;

    // 3. Vimeo API 호출
    let client = VimeoClient::new(access_token.clone());
    let meta = client.get_video_meta(&vimeo_video_id).await?;

    Ok(VimeoPreviewRes {
        vimeo_video_id,
        title: meta.name,
        description: meta.description,
        duration: meta.duration,
        thumbnail_url: meta.thumbnail_url,
    })
}

/// Vimeo 업로드 티켓 생성 (tus resumable upload용)
pub async fn admin_create_vimeo_upload_ticket(
    st: &AppState,
    actor_user_id: i64,
    req: VimeoUploadTicketReq,
) -> AppResult<VimeoUploadTicketRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    // 1. Access Token 확인
    let access_token = st
        .cfg
        .vimeo_access_token
        .as_ref()
        .filter(|t| !t.is_empty())
        .ok_or_else(|| AppError::BadRequest("Vimeo access token not configured".into()))?;

    // 2. Vimeo API로 업로드 티켓 생성
    let client = VimeoClient::new(access_token.clone());
    let (video_uri, vimeo_video_id, upload_link) = client
        .create_upload_ticket(&req.file_name, req.file_size)
        .await?;

    tracing::info!(
        "Created Vimeo upload ticket: video_uri={}, vimeo_video_id={}, user_id={}",
        video_uri,
        vimeo_video_id,
        actor_user_id
    );

    Ok(VimeoUploadTicketRes {
        video_uri,
        vimeo_video_id,
        upload_link,
    })
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

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "CREATE_VIDEO",
        Some("VIDEO"),
        None,
        &serde_json::to_value(&req).unwrap_or(serde_json::Value::Null),
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

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

    // Vimeo 메타데이터 동기화
    sync_vimeo_meta(st, &mut tx, created.id, &req.video_url_vimeo).await?;

    let after = serde_json::to_value(&created).unwrap_or_default();
    repo::create_video_log_tx(
        &mut tx,
        actor_user_id,
        "CREATE",
        Some(created.id),
        None,
        None,
        Some(&after),
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

    let details = serde_json::json!({
        "count": req.items.len()
    });

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "BULK_CREATE_VIDEOS",
        Some("VIDEO"),
        None,
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

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
        "action": "BULK_CREATE",
        "total": summary.total,
        "success": summary.success,
        "failure": summary.failure
    });

    repo::create_video_log(
        &st.db,
        actor_user_id,
        "BULK_CREATE",
        None,
        None,
        None,
        Some(&details),
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
    if !matches!(sort, "id" | "created_at" | "views" | "title" | "video_state" | "video_access") {
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

pub async fn admin_get_video(
    st: &AppState,
    actor_user_id: i64,
    video_id: i64,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminVideoRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    let details = serde_json::json!({
        "video_id": video_id
    });

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "GET_VIDEO",
        Some("video"),
        Some(video_id),
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    let video = repo::admin_get_video(&st.db, video_id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(video)
}

pub async fn admin_bulk_update_videos(
    st: &AppState,
    actor_user_id: i64,
    req: VideoBulkUpdateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<(bool, VideoBulkUpdateRes)> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    let details = serde_json::json!({
        "count": req.items.len()
    });

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "BULK_UPDATE_VIDEOS",
        Some("VIDEO"),
        None,
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    let mut results = Vec::with_capacity(req.items.len());
    let mut success = 0i64;
    let mut failure = 0i64;

    for item in req.items {
        let outcome = async {
            if let Err(e) = item.validate() {
                return Err(AppError::BadRequest(e.to_string()));
            }

            let update_req = VideoUpdateReq {
                video_tag_title: item.video_tag_title.clone(),
                video_tag_subtitle: item.video_tag_subtitle.clone(),
                video_tag_key: item.video_tag_key.clone(),
                video_url_vimeo: item.video_url_vimeo.clone(),
                video_access: item.video_access.clone(),
                video_state: item.video_state.clone(),
                video_idx: item.video_idx.clone(),
            };

            let has_any = update_req.video_tag_title.is_some()
                || update_req.video_tag_subtitle.is_some()
                || update_req.video_tag_key.is_some()
                || update_req.video_url_vimeo.is_some()
                || update_req.video_access.is_some()
                || update_req.video_state.is_some()
                || update_req.video_idx.is_some();

            if !has_any {
                return Err(AppError::BadRequest("no fields to update".into()));
            }

            let mut tx = st.db.begin().await?;

            let exists = sqlx::query_scalar::<_, bool>(
                r#"
                SELECT EXISTS(
                    SELECT 1
                    FROM video
                    WHERE video_id = $1
                )
                "#,
            )
            .bind(item.id)
            .fetch_one(&mut *tx)
            .await?;

            if !exists {
                return Err(AppError::NotFound);
            }

            if let Some(video_idx) = update_req.video_idx.as_deref() {
                if repo::exists_video_idx_for_update(&mut tx, item.id, video_idx.trim()).await? {
                    return Err(AppError::Conflict("video_idx already exists".into()));
                }
            }

            if let Some(tag_key) = update_req.video_tag_key.as_deref() {
                if repo::exists_video_tag_key_for_update(&mut tx, item.id, tag_key.trim()).await? {
                    return Err(AppError::Conflict("video_tag_key already exists".into()));
                }
            }

            let updated = repo::admin_update_video(&mut tx, item.id, actor_user_id, &update_req)
                .await;

            let updated = match updated {
                Ok(val) => val,
                Err(e) if is_unique_violation(&e) => {
                    return Err(AppError::Conflict("duplicate video data".into()));
                }
                Err(e) => return Err(e),
            };

            tx.commit().await?;

            Ok(updated)
        }
        .await;

        match outcome {
            Ok(video) => {
                success += 1;
                results.push(VideoBulkUpdateItemResult {
                    id: video.id,
                    status: 200,
                    data: Some(video),
                    error: None,
                });
            }
            Err(e) => {
                failure += 1;
                let (status, msg) = match e {
                    AppError::NotFound => (404, "Video not found".to_string()),
                    AppError::BadRequest(m) => (400, m),
                    AppError::Unprocessable(m) => (422, m),
                    AppError::Conflict(m) => (409, m),
                    AppError::Forbidden => (403, "Forbidden".to_string()),
                    _ => (500, "Internal Server Error".to_string()),
                };

                results.push(VideoBulkUpdateItemResult {
                    id: item.id,
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
        "action": "BULK_UPDATE",
        "total": summary.total,
        "success": summary.success,
        "failure": summary.failure
    });

    repo::create_video_log(
        &st.db,
        actor_user_id,
        "BULK_UPDATE",
        None,
        None,
        None,
        Some(&details),
    )
    .await?;

    let all_success = failure == 0;

    Ok((all_success, VideoBulkUpdateRes { summary, results }))
}

pub async fn admin_bulk_update_video_tags(
    st: &AppState,
    actor_user_id: i64,
    req: VideoTagBulkUpdateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<(bool, VideoBulkUpdateRes)> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    let details = serde_json::json!({
        "count": req.items.len()
    });

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "BULK_UPDATE_VIDEO_TAGS",
        Some("VIDEO"),
        None,
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    let mut results = Vec::with_capacity(req.items.len());
    let mut success = 0i64;
    let mut failure = 0i64;

    for item in req.items {
        let outcome = async {
            if let Err(e) = item.validate() {
                return Err(AppError::BadRequest(e.to_string()));
            }

            let update_req = VideoUpdateReq {
                video_tag_title: item.video_tag_title.clone(),
                video_tag_subtitle: item.video_tag_subtitle.clone(),
                video_tag_key: item.video_tag_key.clone(),
                video_url_vimeo: None,
                video_access: None,
                video_state: None,
                video_idx: None,
            };

            let has_any = update_req.video_tag_title.is_some()
                || update_req.video_tag_subtitle.is_some()
                || update_req.video_tag_key.is_some();

            if !has_any {
                return Err(AppError::BadRequest("no fields to update".into()));
            }

            let mut tx = st.db.begin().await?;

            let exists = sqlx::query_scalar::<_, bool>(
                r#"
                SELECT EXISTS(
                    SELECT 1
                    FROM video
                    WHERE video_id = $1
                )
                "#,
            )
            .bind(item.id)
            .fetch_one(&mut *tx)
            .await?;

            if !exists {
                return Err(AppError::NotFound);
            }

            if let Some(tag_key) = update_req.video_tag_key.as_deref() {
                if repo::exists_video_tag_key_for_update(&mut tx, item.id, tag_key.trim()).await? {
                    return Err(AppError::Conflict("video_tag_key already exists".into()));
                }
            }

            let updated = repo::admin_update_video(&mut tx, item.id, actor_user_id, &update_req)
                .await;

            let updated = match updated {
                Ok(val) => val,
                Err(e) if is_unique_violation(&e) => {
                    return Err(AppError::Conflict("duplicate video data".into()));
                }
                Err(e) => return Err(e),
            };

            tx.commit().await?;

            Ok(updated)
        }
        .await;

        match outcome {
            Ok(video) => {
                success += 1;
                results.push(VideoBulkUpdateItemResult {
                    id: video.id,
                    status: 200,
                    data: Some(video),
                    error: None,
                });
            }
            Err(e) => {
                failure += 1;
                let (status, msg) = match e {
                    AppError::NotFound => (404, "Video not found".to_string()),
                    AppError::BadRequest(m) => (400, m),
                    AppError::Unprocessable(m) => (422, m),
                    AppError::Conflict(m) => (409, m),
                    AppError::Forbidden => (403, "Forbidden".to_string()),
                    _ => (500, "Internal Server Error".to_string()),
                };

                results.push(VideoBulkUpdateItemResult {
                    id: item.id,
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
        "action": "BULK_UPDATE_TAG",
        "total": summary.total,
        "success": summary.success,
        "failure": summary.failure
    });

    repo::create_video_log(
        &st.db,
        actor_user_id,
        "BULK_UPDATE_TAG",
        None,
        None,
        None,
        Some(&details),
    )
    .await?;

    let all_success = failure == 0;

    Ok((all_success, VideoBulkUpdateRes { summary, results }))
}

pub async fn admin_update_video_tags(
    st: &AppState,
    actor_user_id: i64,
    video_id: i64,
    req: VideoTagUpdateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminVideoRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "UPDATE_VIDEO_TAGS",
        Some("VIDEO"),
        Some(video_id),
        &serde_json::to_value(&req).unwrap_or(serde_json::Value::Null),
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    let update_req: VideoUpdateReq = req.into();
    let has_any = update_req.video_tag_title.is_some()
        || update_req.video_tag_subtitle.is_some()
        || update_req.video_tag_key.is_some();

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

    if let Some(tag_key) = update_req.video_tag_key.as_deref() {
        if repo::exists_video_tag_key_for_update(&mut tx, video_id, tag_key.trim()).await? {
            return Err(AppError::Conflict("video_tag_key already exists".into()));
        }
    }

    let updated = repo::admin_update_video(&mut tx, video_id, actor_user_id, &update_req).await;

    let updated = match updated {
        Ok(val) => val,
        Err(e) if is_unique_violation(&e) => {
            return Err(AppError::Conflict("duplicate video data".into()));
        }
        Err(e) => return Err(e),
    };

    let after = serde_json::to_value(&updated).unwrap_or_default();
    repo::create_video_log_tx(
        &mut tx,
        actor_user_id,
        "UPDATE_TAG",
        Some(updated.id),
        None,
        None,
        Some(&after),
    )
    .await?;

    tx.commit().await?;

    Ok(updated)
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

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "UPDATE_VIDEO",
        Some("VIDEO"),
        Some(video_id),
        &serde_json::to_value(&req).unwrap_or(serde_json::Value::Null),
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

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

    // Vimeo URL 변경 시 메타데이터 동기화
    if let Some(ref vimeo_url) = req.video_url_vimeo {
        sync_vimeo_meta(st, &mut tx, video_id, vimeo_url).await?;
    }

    let after = serde_json::to_value(&updated).unwrap_or_default();
    repo::create_video_log_tx(
        &mut tx,
        actor_user_id,
        "UPDATE",
        Some(updated.id),
        None,
        None,
        Some(&after),
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
