use std::net::IpAddr;

use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::types::{LessonAccess, LessonState, UserAuth};
use crate::AppState;

use super::dto::{
    AdminLessonItemListRes, AdminLessonItemRes, AdminLessonItemsDetailRes, AdminLessonListRes,
    AdminLessonProgressListDetailRes, AdminLessonProgressListRes, AdminLessonProgressRes,
    AdminLessonRes, InsertMode, LessonBulkCreateReq, LessonBulkCreateRes, LessonBulkResult,
    LessonBulkUpdateReq, LessonBulkUpdateRes, LessonBulkUpdateResult, LessonCreateReq,
    LessonItemBulkCreateReq, LessonItemBulkCreateRes, LessonItemBulkCreateResult,
    LessonItemBulkDeleteReq, LessonItemBulkDeleteRes, LessonItemBulkDeleteResult,
    LessonItemBulkUpdateReq, LessonItemBulkUpdateRes, LessonItemBulkUpdateResult,
    LessonItemCreateReq, LessonItemListReq, LessonItemUpdateReq, LessonListReq,
    LessonProgressBulkUpdateReq, LessonProgressBulkUpdateRes, LessonProgressBulkUpdateResult,
    LessonProgressListReq, LessonProgressUpdateReq, LessonUpdateItem, LessonUpdateReq,
};
use super::repo;

const PG_UNIQUE_VIOLATION: &str = "23505";

async fn check_admin_rbac(pool: &sqlx::PgPool, actor_user_id: i64) -> AppResult<UserAuth> {
    let actor = crate::api::user::repo::find_user(pool, actor_user_id)
        .await?
        .ok_or(AppError::Unauthorized("Actor user not found".into()))?;

    match actor.user_auth {
        UserAuth::Hymn | UserAuth::Admin | UserAuth::Manager => Ok(actor.user_auth),
        _ => Err(AppError::Forbidden),
    }
}

fn is_unique_violation(err: &AppError) -> bool {
    if let AppError::Sqlx(sqlx::Error::Database(db)) = err {
        db.code().as_deref() == Some(PG_UNIQUE_VIOLATION)
    } else {
        false
    }
}

pub async fn admin_list_lessons(
    st: &AppState,
    actor_user_id: i64,
    req: LessonListReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminLessonListRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    req.validate()?;

    let page = req.page.unwrap_or(1);
    let size = req.size.unwrap_or(20);
    let sort = req.sort.as_deref().unwrap_or("created_at");
    let order = req.order.as_deref().unwrap_or("desc");
    let q = req.q.clone();

    let details = serde_json::json!({
        "q": q.as_deref(),
        "page": page,
        "size": size,
        "sort": sort,
        "order": order
    });

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "LIST_LESSONS",
        Some("LESSON"),
        None,
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    let (total, list) = repo::admin_list_lessons(
        &st.db,
        q,
        page,
        size,
        sort,
        order,
        req.lesson_state,
        req.lesson_access,
    )
    .await?;

    Ok(AdminLessonListRes {
        list,
        total,
        page,
        size,
        total_pages: (total as f64 / size as f64).ceil() as i64,
    })
}

pub async fn admin_list_lesson_items(
    st: &AppState,
    actor_user_id: i64,
    req: LessonItemListReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminLessonItemListRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    req.validate()?;

    let page = req.page.unwrap_or(1);
    let size = req.size.unwrap_or(20);
    let sort = req.sort.as_deref().unwrap_or("lesson_id");
    let order = req.order.as_deref().unwrap_or("asc");

    let lesson_item_kind = req
        .lesson_item_kind
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty());

    let normalized_kind = match lesson_item_kind {
        Some(kind) => {
            let kind = kind.to_lowercase();
            match kind.as_str() {
                "video" => Some("video"),
                "task" => Some("task"),
                "study_task" => Some("task"),
                _ => return Err(AppError::BadRequest("invalid lesson_item_kind".into())),
            }
        }
        None => None,
    };

    
    let details = serde_json::to_value(&req).unwrap_or(serde_json::Value::Null);

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "LIST_LESSON_ITEMS",
        Some("LESSON_ITEM"),
        None,
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    let (total, list) = repo::admin_list_lesson_items(
        &st.db,
        req.lesson_id,
        normalized_kind,
        page,
        size,
        sort,
        order,
    )
    .await?;

    Ok(AdminLessonItemListRes {
        list,
        total,
        page,
        size,
        total_pages: (total as f64 / size as f64).ceil() as i64,
    })
}

pub async fn admin_list_lesson_progress(
    st: &AppState,
    actor_user_id: i64,
    req: LessonProgressListReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminLessonProgressListRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    req.validate()?;

    let page = req.page.unwrap_or(1);
    let size = req.size.unwrap_or(20);
    let sort = req
        .sort
        .as_deref()
        .unwrap_or("lesson_progress_last_progress_at");
    let order = req.order.as_deref().unwrap_or("desc");

    
    let details = serde_json::json!({
        "page": page,
        "size": size,
        "sort": sort,
        "order": order,
        "lesson_id": req.lesson_id,
        "user_id": req.user_id
    });

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "LIST_LESSON_PROGRESS",
        Some("LESSON_PROGRESS"),
        None,
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    let (total, list) = repo::admin_list_lesson_progress(
        &st.db,
        req.lesson_id,
        req.user_id,
        page,
        size,
        sort,
        order,
    )
    .await?;

    Ok(AdminLessonProgressListRes {
        list,
        total,
        page,
        size,
        total_pages: (total as f64 / size as f64).ceil() as i64,
    })
}

pub async fn admin_update_lesson_progress(
    st: &AppState,
    actor_user_id: i64,
    lesson_id: i32,
    req: LessonProgressUpdateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminLessonProgressRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    let has_any =
        req.lesson_progress_percent.is_some() || req.lesson_progress_last_item_seq.is_some();

    if !has_any {
        return Err(AppError::BadRequest("no fields to update".into()));
    }

    
    let details = serde_json::json!({
        "lesson_id": lesson_id,
        "user_id": req.user_id,
        "payload": &req
    });

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "UPDATE_LESSON_PROGRESS",
        Some("LESSON_PROGRESS"),
        Some(lesson_id as i64),
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    let mut tx = st.db.begin().await?;

    let before = repo::find_lesson_progress_tx(&mut tx, lesson_id, req.user_id)
        .await?
        .ok_or(AppError::NotFound)?;

    repo::update_lesson_progress_tx(
        &mut tx,
        lesson_id,
        req.user_id,
        req.lesson_progress_percent,
        req.lesson_progress_last_item_seq,
    )
    .await?;

    let after = repo::find_lesson_progress_tx(&mut tx, lesson_id, req.user_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let before_val = serde_json::to_value(&before).unwrap_or_default();
    let after_val = serde_json::to_value(&after).unwrap_or_default();

    repo::create_lesson_log_tx(
        &mut tx,
        actor_user_id,
        "update",
        lesson_id,
        after.lesson_progress_last_item_seq,
        None,
        None,
        Some(&before_val),
        Some(&after_val),
    )
    .await?;

    tx.commit().await?;

    Ok(after)
}

pub async fn admin_bulk_update_lesson_progress(
    st: &AppState,
    actor_user_id: i64,
    req: LessonProgressBulkUpdateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<(bool, LessonProgressBulkUpdateRes)> {
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
        "BULK_UPDATE_LESSON_PROGRESS",
        Some("LESSON_PROGRESS"),
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
        let lesson_id = item.lesson_id;
        let user_id = item.user_id;

        let outcome = async {
            if let Err(e) = item.validate() {
                return Err(AppError::BadRequest(e.to_string()));
            }

            let has_any = item.lesson_progress_percent.is_some()
                || item.lesson_progress_last_item_seq.is_some();

            if !has_any {
                return Err(AppError::BadRequest("no fields to update".into()));
            }

            let mut tx = st.db.begin().await?;

            let before = repo::find_lesson_progress_tx(&mut tx, lesson_id, user_id)
                .await?
                .ok_or(AppError::NotFound)?;

            repo::update_lesson_progress_tx(
                &mut tx,
                lesson_id,
                user_id,
                item.lesson_progress_percent,
                item.lesson_progress_last_item_seq,
            )
            .await?;

            let after = repo::find_lesson_progress_tx(&mut tx, lesson_id, user_id)
                .await?
                .ok_or(AppError::NotFound)?;

            let before_val = serde_json::to_value(&before).unwrap_or_default();
            let after_val = serde_json::to_value(&after).unwrap_or_default();

            repo::create_lesson_log_tx(
                &mut tx,
                actor_user_id,
                "update",
                lesson_id,
                after.lesson_progress_last_item_seq,
                None,
                None,
                Some(&before_val),
                Some(&after_val),
            )
            .await?;

            tx.commit().await?;

            Ok(())
        }
        .await;

        match outcome {
            Ok(_) => {
                success += 1;
                results.push(LessonProgressBulkUpdateResult {
                    lesson_id,
                    user_id,
                    success: true,
                    error: None,
                });
            }
            Err(e) => {
                failure += 1;
                let msg = match e {
                    AppError::NotFound => "Lesson progress not found".to_string(),
                    AppError::BadRequest(m) => m,
                    AppError::Unprocessable(m) => m,
                    AppError::Conflict(m) => m,
                    AppError::Forbidden => "Forbidden".to_string(),
                    _ => "Internal Server Error".to_string(),
                };

                results.push(LessonProgressBulkUpdateResult {
                    lesson_id,
                    user_id,
                    success: false,
                    error: Some(msg),
                });
            }
        }
    }

    let all_success = failure == 0;

    Ok((
        all_success,
        LessonProgressBulkUpdateRes {
            success_count: success,
            failure_count: failure,
            results,
        },
    ))
}

pub async fn admin_create_lesson_item(
    st: &AppState,
    actor_user_id: i64,
    lesson_id: i32,
    req: LessonItemCreateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminLessonItemRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    req.validate()?;

    let kind = req.lesson_item_kind.trim().to_lowercase();
    let (normalized_kind, video_id, study_task_id) = match kind.as_str() {
        "video" => {
            let video_id = req
                .video_id
                .ok_or_else(|| AppError::BadRequest("video_id is required".into()))?;
            ("video", Some(video_id), None)
        }
        "task" | "study_task" => {
            let task_id = req
                .study_task_id
                .ok_or_else(|| AppError::BadRequest("study_task_id is required".into()))?;
            ("task", None, Some(task_id))
        }
        _ => return Err(AppError::BadRequest("invalid lesson_item_kind".into())),
    };

    
    let details = serde_json::json!({
        "lesson_id": lesson_id,
        "payload": &req
    });

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "CREATE_LESSON_ITEM",
        Some("LESSON_ITEM"),
        Some(lesson_id as i64),
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    if !repo::exists_lesson(&st.db, lesson_id).await? {
        return Err(AppError::NotFound);
    }

    let mut tx = st.db.begin().await?;

    // Handle insert mode
    let seq_exists = repo::exists_lesson_item_tx(&mut tx, lesson_id, req.lesson_item_seq).await?;
    if seq_exists {
        match req.insert_mode {
            InsertMode::Error => {
                return Err(AppError::Conflict("lesson_item_seq already exists".into()));
            }
            InsertMode::Shift => {
                // Shift existing items down (seq >= target becomes seq + 1)
                repo::shift_lesson_items_tx(&mut tx, lesson_id, req.lesson_item_seq).await?;
            }
        }
    }

    let created = repo::create_lesson_item(
        &mut tx,
        lesson_id,
        normalized_kind,
        video_id,
        study_task_id,
        &req,
    )
    .await;

    let created = match created {
        Ok(val) => val,
        Err(e) if is_unique_violation(&e) => {
            return Err(AppError::Conflict("lesson_item_seq already exists".into()));
        }
        Err(e) => return Err(e),
    };

    let after = serde_json::to_value(&created).unwrap_or_default();
    repo::create_lesson_log_tx(
        &mut tx,
        actor_user_id,
        "create",
        lesson_id,
        Some(req.lesson_item_seq),
        video_id,
        study_task_id,
        None,
        Some(&after),
    )
    .await?;

    tx.commit().await?;

    Ok(created)
}

pub async fn admin_bulk_create_lesson_items(
    st: &AppState,
    actor_user_id: i64,
    req: LessonItemBulkCreateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<(bool, LessonItemBulkCreateRes)> {
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
        "BULK_CREATE_LESSON_ITEMS",
        Some("LESSON_ITEM"),
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
        let lesson_id = item.lesson_id;
        let lesson_item_seq = item.lesson_item_seq;

        let outcome = async {
            if let Err(e) = item.validate() {
                return Err(AppError::BadRequest(e.to_string()));
            }

            let mut tx = st.db.begin().await?;

            if !repo::exists_lesson_tx(&mut tx, lesson_id).await? {
                return Err(AppError::NotFound);
            }

            let kind = item.lesson_item_kind.trim().to_lowercase();
            let (normalized_kind, video_id, study_task_id) = match kind.as_str() {
                "video" => {
                    let video_id = item
                        .video_id
                        .ok_or_else(|| AppError::BadRequest("video_id is required".into()))?;
                    ("video", Some(video_id), None)
                }
                "task" | "study_task" => {
                    let task_id = item
                        .study_task_id
                        .ok_or_else(|| AppError::BadRequest("study_task_id is required".into()))?;
                    ("task", None, Some(task_id))
                }
                _ => return Err(AppError::BadRequest("invalid lesson_item_kind".into())),
            };

            if repo::exists_lesson_item_tx(&mut tx, lesson_id, lesson_item_seq).await? {
                return Err(AppError::Conflict("lesson_item_seq already exists".into()));
            }

            let created = repo::create_lesson_item_tx(
                &mut tx,
                lesson_id,
                lesson_item_seq,
                normalized_kind,
                video_id,
                study_task_id,
            )
            .await;

            let created = match created {
                Ok(val) => val,
                Err(e) if is_unique_violation(&e) => {
                    return Err(AppError::Conflict("lesson_item_seq already exists".into()));
                }
                Err(e) => return Err(e),
            };

            let after = serde_json::to_value(&created).unwrap_or_default();
            repo::create_lesson_log_tx(
                &mut tx,
                actor_user_id,
                "create",
                lesson_id,
                Some(lesson_item_seq),
                video_id,
                study_task_id,
                None,
                Some(&after),
            )
            .await?;

            tx.commit().await?;

            Ok(created)
        }
        .await;

        match outcome {
            Ok(_) => {
                success += 1;
                results.push(LessonItemBulkCreateResult {
                    lesson_id,
                    lesson_item_seq,
                    success: true,
                    error: None,
                });
            }
            Err(e) => {
                failure += 1;
                let msg = match e {
                    AppError::NotFound => "Lesson not found".to_string(),
                    AppError::BadRequest(m) => m,
                    AppError::Unprocessable(m) => m,
                    AppError::Conflict(m) => m,
                    AppError::Forbidden => "Forbidden".to_string(),
                    _ => "Internal Server Error".to_string(),
                };

                results.push(LessonItemBulkCreateResult {
                    lesson_id,
                    lesson_item_seq,
                    success: false,
                    error: Some(msg),
                });
            }
        }
    }

    let all_success = failure == 0;

    Ok((
        all_success,
        LessonItemBulkCreateRes {
            success_count: success,
            failure_count: failure,
            results,
        },
    ))
}

pub async fn admin_update_lesson_item(
    st: &AppState,
    actor_user_id: i64,
    lesson_id: i32,
    current_seq: i32,
    req: LessonItemUpdateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminLessonItemRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "UPDATE_LESSON_ITEM",
        Some("LESSON_ITEM"),
        Some(lesson_id as i64),
        &serde_json::json!({
            "lesson_id": lesson_id,
            "lesson_item_seq": current_seq,
            "payload": &req
        }),
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    let has_any = req.lesson_item_seq.is_some()
        || req.lesson_item_kind.is_some()
        || req.video_id.is_some()
        || req.study_task_id.is_some();

    if !has_any {
        return Err(AppError::BadRequest("no fields to update".into()));
    }

    let mut tx = st.db.begin().await?;

    let before = repo::find_lesson_item_tx(&mut tx, lesson_id, current_seq)
        .await?
        .ok_or(AppError::NotFound)?;

    let before_kind = before.lesson_item_kind.as_str();

    let normalized_kind = if let Some(kind_raw) = req.lesson_item_kind.as_deref() {
        let kind = kind_raw.trim().to_lowercase();
        let normalized = match kind.as_str() {
            "video" => "video",
            "task" | "study_task" => "task",
            _ => return Err(AppError::BadRequest("invalid lesson_item_kind".into())),
        };
        Some(normalized.to_string())
    } else {
        None
    };

    let kind_update = normalized_kind
        .as_deref()
        .filter(|kind| *kind != before_kind)
        .map(|kind| kind.to_string());

    let target_kind = normalized_kind
        .as_deref()
        .unwrap_or(before_kind);

    let new_seq = req
        .lesson_item_seq
        .filter(|seq| *seq != before.lesson_item_seq);

    if let Some(new_seq) = new_seq {
        if repo::exists_lesson_item_tx(&mut tx, lesson_id, new_seq).await? {
            return Err(AppError::Conflict("lesson_item_seq already exists".into()));
        }
    }

    let kind_changed = kind_update.is_some();
    let mut video_update: Option<Option<i32>> = None;
    let mut task_update: Option<Option<i32>> = None;

    match target_kind {
        "video" => {
            if req.study_task_id.is_some() {
                return Err(AppError::BadRequest(
                    "study_task_id is not allowed for video".into(),
                ));
            }
            if kind_changed {
                let video_id = req
                    .video_id
                    .ok_or_else(|| AppError::BadRequest("video_id is required".into()))?;
                video_update = Some(Some(video_id));
                task_update = Some(None);
            } else if let Some(video_id) = req.video_id {
                video_update = Some(Some(video_id));
            }
        }
        "task" => {
            if req.video_id.is_some() {
                return Err(AppError::BadRequest(
                    "video_id is not allowed for task".into(),
                ));
            }
            if kind_changed {
                let task_id = req
                    .study_task_id
                    .ok_or_else(|| AppError::BadRequest("study_task_id is required".into()))?;
                task_update = Some(Some(task_id));
                video_update = Some(None);
            } else if let Some(task_id) = req.study_task_id {
                task_update = Some(Some(task_id));
            }
        }
        _ => return Err(AppError::BadRequest("invalid lesson_item_kind".into())),
    }

    let has_update = new_seq.is_some()
        || kind_update.is_some()
        || video_update.is_some()
        || task_update.is_some();

    if !has_update {
        return Err(AppError::BadRequest("no fields to update".into()));
    }

    repo::update_lesson_item_tx(
        &mut tx,
        lesson_id,
        current_seq,
        new_seq,
        kind_update.as_deref(),
        video_update,
        task_update,
    )
    .await?;

    let final_seq = new_seq.unwrap_or(before.lesson_item_seq);
    let after = repo::find_lesson_item_tx(&mut tx, lesson_id, final_seq)
        .await?
        .ok_or(AppError::NotFound)?;

    let before_val = serde_json::to_value(&before).unwrap_or_default();
    let after_val = serde_json::to_value(&after).unwrap_or_default();

    repo::create_lesson_log_tx(
        &mut tx,
        actor_user_id,
        "update",
        lesson_id,
        Some(after.lesson_item_seq),
        after.video_id,
        after.study_task_id,
        Some(&before_val),
        Some(&after_val),
    )
    .await?;

    tx.commit().await?;

    Ok(after)
}

pub async fn admin_bulk_update_lesson_items(
    st: &AppState,
    actor_user_id: i64,
    req: LessonItemBulkUpdateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<(bool, LessonItemBulkUpdateRes)> {
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
        "BULK_UPDATE_LESSON_ITEMS",
        Some("LESSON_ITEM"),
        None,
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    // Check if this is a pure reorder operation (only seq changes for the same lesson_id)
    let is_pure_reorder = !req.items.is_empty()
        && req.items.iter().all(|item| {
            item.new_lesson_item_seq.is_some()
                && item.lesson_item_kind.is_none()
                && item.video_id.is_none()
                && item.study_task_id.is_none()
        })
        && req.items.iter().map(|i| i.lesson_id).collect::<std::collections::HashSet<_>>().len() == 1;

    if is_pure_reorder {
        // Use atomic reorder with two-phase update to avoid unique constraint violations
        return admin_bulk_reorder_lesson_items(st, actor_user_id, req).await;
    }

    // Standard bulk update (individual transactions)
    let mut results = Vec::with_capacity(req.items.len());
    let mut success = 0i64;
    let mut failure = 0i64;

    for item in req.items {
        let lesson_id = item.lesson_id;
        let current_seq = item.current_lesson_item_seq;

        let outcome = async {
            if let Err(e) = item.validate() {
                return Err(AppError::BadRequest(e.to_string()));
            }

            let has_any = item.new_lesson_item_seq.is_some()
                || item.lesson_item_kind.is_some()
                || item.video_id.is_some()
                || item.study_task_id.is_some();

            if !has_any {
                return Err(AppError::BadRequest("no fields to update".into()));
            }

            let mut tx = st.db.begin().await?;

            let before = repo::find_lesson_item_tx(&mut tx, lesson_id, current_seq)
                .await?
                .ok_or(AppError::NotFound)?;

            let before_kind = before.lesson_item_kind.as_str();

            let normalized_kind = if let Some(kind_raw) = item.lesson_item_kind.as_deref() {
                let kind = kind_raw.trim().to_lowercase();
                let normalized = match kind.as_str() {
                    "video" => "video",
                    "task" | "study_task" => "task",
                    _ => return Err(AppError::BadRequest("invalid lesson_item_kind".into())),
                };
                Some(normalized.to_string())
            } else {
                None
            };

            let kind_update = normalized_kind
                .as_deref()
                .filter(|kind| *kind != before_kind)
                .map(|kind| kind.to_string());

            let target_kind = normalized_kind
                .as_deref()
                .unwrap_or(before_kind);

            let new_seq = item
                .new_lesson_item_seq
                .filter(|seq| *seq != before.lesson_item_seq);

            if let Some(new_seq) = new_seq {
                if repo::exists_lesson_item_tx(&mut tx, lesson_id, new_seq).await? {
                    return Err(AppError::Conflict("lesson_item_seq already exists".into()));
                }
            }

            let kind_changed = kind_update.is_some();
            let mut video_update: Option<Option<i32>> = None;
            let mut task_update: Option<Option<i32>> = None;

            match target_kind {
                "video" => {
                    if item.study_task_id.is_some() {
                        return Err(AppError::BadRequest(
                            "study_task_id is not allowed for video".into(),
                        ));
                    }
                    if kind_changed {
                        let video_id = item
                            .video_id
                            .ok_or_else(|| AppError::BadRequest("video_id is required".into()))?;
                        video_update = Some(Some(video_id));
                        task_update = Some(None);
                    } else if let Some(video_id) = item.video_id {
                        video_update = Some(Some(video_id));
                    }
                }
                "task" => {
                    if item.video_id.is_some() {
                        return Err(AppError::BadRequest(
                            "video_id is not allowed for task".into(),
                        ));
                    }
                    if kind_changed {
                        let task_id = item
                            .study_task_id
                            .ok_or_else(|| AppError::BadRequest("study_task_id is required".into()))?;
                        task_update = Some(Some(task_id));
                        video_update = Some(None);
                    } else if let Some(task_id) = item.study_task_id {
                        task_update = Some(Some(task_id));
                    }
                }
                _ => return Err(AppError::BadRequest("invalid lesson_item_kind".into())),
            }

            let has_update = new_seq.is_some()
                || kind_update.is_some()
                || video_update.is_some()
                || task_update.is_some();

            if !has_update {
                return Err(AppError::BadRequest("no fields to update".into()));
            }

            repo::update_lesson_item_tx(
                &mut tx,
                lesson_id,
                current_seq,
                new_seq,
                kind_update.as_deref(),
                video_update,
                task_update,
            )
            .await?;

            let final_seq = new_seq.unwrap_or(before.lesson_item_seq);
            let after = repo::find_lesson_item_tx(&mut tx, lesson_id, final_seq)
                .await?
                .ok_or(AppError::NotFound)?;

            let before_val = serde_json::to_value(&before).unwrap_or_default();
            let after_val = serde_json::to_value(&after).unwrap_or_default();

            repo::create_lesson_log_tx(
                &mut tx,
                actor_user_id,
                "update",
                lesson_id,
                Some(after.lesson_item_seq),
                after.video_id,
                after.study_task_id,
                Some(&before_val),
                Some(&after_val),
            )
            .await?;

            tx.commit().await?;

            Ok(final_seq)
        }
        .await;

        match outcome {
            Ok(final_seq) => {
                success += 1;
                results.push(LessonItemBulkUpdateResult {
                    lesson_id,
                    lesson_item_seq: final_seq,
                    success: true,
                    error: None,
                });
            }
            Err(e) => {
                failure += 1;
                let msg = match e {
                    AppError::NotFound => "Lesson item not found".to_string(),
                    AppError::BadRequest(m) => m,
                    AppError::Unprocessable(m) => m,
                    AppError::Conflict(m) => m,
                    AppError::Forbidden => "Forbidden".to_string(),
                    _ => "Internal Server Error".to_string(),
                };

                results.push(LessonItemBulkUpdateResult {
                    lesson_id,
                    lesson_item_seq: current_seq,
                    success: false,
                    error: Some(msg),
                });
            }
        }
    }

    let all_success = failure == 0;

    Ok((
        all_success,
        LessonItemBulkUpdateRes {
            success_count: success,
            failure_count: failure,
            results,
        },
    ))
}

/// Atomic reorder of lesson items using two-phase update to avoid unique constraint violations
async fn admin_bulk_reorder_lesson_items(
    st: &AppState,
    actor_user_id: i64,
    req: LessonItemBulkUpdateReq,
) -> AppResult<(bool, LessonItemBulkUpdateRes)> {
    let lesson_id = req.items.first().map(|i| i.lesson_id).unwrap_or(0);

    let mut tx = st.db.begin().await?;

    // Collect reorder mappings: (current_seq, new_seq)
    let mut reorder_map: Vec<(i32, i32)> = Vec::with_capacity(req.items.len());

    for item in &req.items {
        let current_seq = item.current_lesson_item_seq;
        let new_seq = item.new_lesson_item_seq.unwrap_or(current_seq);

        // Verify item exists
        if !repo::exists_lesson_item_tx(&mut tx, lesson_id, current_seq).await? {
            tx.rollback().await?;
            return Err(AppError::NotFound);
        }

        reorder_map.push((current_seq, new_seq));
    }

    // Phase 1: Update all items to temporary negative sequences
    // Use offset of -10000 to avoid collision with any real sequences
    for (current_seq, _) in &reorder_map {
        let temp_seq = -10000 - current_seq;
        repo::update_lesson_item_seq_tx(&mut tx, lesson_id, *current_seq, temp_seq).await?;
    }

    // Phase 2: Update from temporary to final sequences
    for (current_seq, new_seq) in &reorder_map {
        let temp_seq = -10000 - current_seq;
        repo::update_lesson_item_seq_tx(&mut tx, lesson_id, temp_seq, *new_seq).await?;
    }

    // Log the reorder operation
    let reorder_details = serde_json::json!({
        "lesson_id": lesson_id,
        "reorder": reorder_map.iter().map(|(from, to)| {
            serde_json::json!({ "from": from, "to": to })
        }).collect::<Vec<_>>()
    });

    repo::create_lesson_log_tx(
        &mut tx,
        actor_user_id,
        "update", // Use "update" action for reorder (as reorder is a type of update)
        lesson_id,
        None,
        None,
        None,
        Some(&reorder_details),
        None,
    )
    .await?;

    tx.commit().await?;

    // Build success results
    let results: Vec<LessonItemBulkUpdateResult> = reorder_map
        .iter()
        .map(|(_, new_seq)| LessonItemBulkUpdateResult {
            lesson_id,
            lesson_item_seq: *new_seq,
            success: true,
            error: None,
        })
        .collect();

    let success_count = results.len() as i64;

    Ok((
        true,
        LessonItemBulkUpdateRes {
            success_count,
            failure_count: 0,
            results,
        },
    ))
}

pub async fn admin_bulk_delete_lesson_items(
    st: &AppState,
    actor_user_id: i64,
    req: LessonItemBulkDeleteReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<(bool, LessonItemBulkDeleteRes)> {
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
        "BULK_DELETE_LESSON_ITEMS",
        Some("LESSON_ITEM"),
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
        let lesson_id = item.lesson_id;
        let seq = item.lesson_item_seq;

        let outcome = async {
            if let Err(e) = item.validate() {
                return Err(AppError::BadRequest(e.to_string()));
            }

            let mut tx = st.db.begin().await?;

            // Find the item to delete (for logging)
            let before = repo::find_lesson_item_tx(&mut tx, lesson_id, seq)
                .await?
                .ok_or(AppError::NotFound)?;

            // Delete the item
            repo::delete_lesson_item_tx(&mut tx, lesson_id, seq).await?;

            // Log the deletion
            let before_val = serde_json::to_value(&before).unwrap_or_default();
            repo::create_lesson_log_tx(
                &mut tx,
                actor_user_id,
                "delete",
                lesson_id,
                Some(seq),
                before.video_id,
                before.study_task_id,
                Some(&before_val),
                None,
            )
            .await?;

            tx.commit().await?;

            Ok(())
        }
        .await;

        match outcome {
            Ok(()) => {
                success += 1;
                results.push(LessonItemBulkDeleteResult {
                    lesson_id,
                    lesson_item_seq: seq,
                    success: true,
                    error: None,
                });
            }
            Err(e) => {
                failure += 1;
                let msg = match e {
                    AppError::NotFound => "Lesson item not found".to_string(),
                    AppError::BadRequest(m) => m,
                    AppError::Forbidden => "Forbidden".to_string(),
                    _ => "Internal Server Error".to_string(),
                };

                results.push(LessonItemBulkDeleteResult {
                    lesson_id,
                    lesson_item_seq: seq,
                    success: false,
                    error: Some(msg),
                });
            }
        }
    }

    let all_success = failure == 0;

    Ok((
        all_success,
        LessonItemBulkDeleteRes {
            success_count: success,
            failure_count: failure,
            results,
        },
    ))
}

pub async fn admin_create_lesson(
    st: &AppState,
    actor_user_id: i64,
    req: LessonCreateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminLessonRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    req.validate()?;

    let lesson_idx = req.lesson_idx.trim();
    if lesson_idx.is_empty() {
        return Err(AppError::BadRequest("lesson_idx is required".into()));
    }

    let lesson_title = req.lesson_title.trim();
    if lesson_title.is_empty() {
        return Err(AppError::BadRequest("lesson_title is required".into()));
    }

    let lesson_subtitle = req
        .lesson_subtitle
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty());
    let lesson_description = req
        .lesson_description
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty());
    let lesson_state = req.lesson_state.unwrap_or(LessonState::Ready);
    let lesson_access = req.lesson_access.unwrap_or(LessonAccess::Public);

    
    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "CREATE_LESSON",
        Some("LESSON"),
        None,
        &serde_json::to_value(&req).unwrap_or(serde_json::Value::Null),
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    if repo::exists_lesson_idx(&st.db, lesson_idx).await? {
        return Err(AppError::Conflict("Lesson Index already exists".into()));
    }

    let mut tx = st.db.begin().await?;

    let created = repo::create_lesson(
        &mut tx,
        actor_user_id,
        lesson_idx,
        lesson_title,
        lesson_subtitle,
        lesson_description,
        lesson_state,
        lesson_access,
    )
    .await;

    let created = match created {
        Ok(val) => val,
        Err(e) if is_unique_violation(&e) => {
            return Err(AppError::Conflict("Lesson Index already exists".into()));
        }
        Err(e) => return Err(e),
    };

    let after = serde_json::to_value(&created).unwrap_or_default();
    repo::create_lesson_log(
        &mut tx,
        actor_user_id,
        "create",
        created.lesson_id,
        None,
        None,
        None,
        None,
        Some(&after),
    )
    .await?;

    tx.commit().await?;

    Ok(created)
}

pub async fn admin_bulk_create_lessons(
    st: &AppState,
    actor_user_id: i64,
    req: LessonBulkCreateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<(bool, LessonBulkCreateRes)> {
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
        "BULK_CREATE_LESSONS",
        Some("LESSON"),
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
        let lesson_idx = item.lesson_idx.trim().to_string();
        let lesson_title = item.lesson_title.trim().to_string();

        let outcome = async {
            if let Err(e) = item.validate() {
                return Err(AppError::BadRequest(e.to_string()));
            }

            if lesson_idx.is_empty() {
                return Err(AppError::BadRequest("lesson_idx is required".into()));
            }

            if lesson_title.is_empty() {
                return Err(AppError::BadRequest("lesson_title is required".into()));
            }

            let lesson_subtitle = item
                .lesson_subtitle
                .as_deref()
                .map(str::trim)
                .filter(|v| !v.is_empty());
            let lesson_description = item
                .lesson_description
                .as_deref()
                .map(str::trim)
                .filter(|v| !v.is_empty());
            let lesson_state = item.lesson_state.unwrap_or(LessonState::Ready);
            let lesson_access = item.lesson_access.unwrap_or(LessonAccess::Public);

            let mut tx = st.db.begin().await?;

            if repo::exists_lesson_idx_tx(&mut tx, &lesson_idx).await? {
                return Err(AppError::Conflict("Lesson Index already exists".into()));
            }

            let created = repo::create_lesson_tx(
                &mut tx,
                actor_user_id,
                &lesson_idx,
                &lesson_title,
                lesson_subtitle,
                lesson_description,
                lesson_state,
                lesson_access,
            )
            .await;

            let created = match created {
                Ok(val) => val,
                Err(e) if is_unique_violation(&e) => {
                    return Err(AppError::Conflict("Lesson Index already exists".into()));
                }
                Err(e) => return Err(e),
            };

            let after = serde_json::to_value(&created).unwrap_or_default();
            repo::create_lesson_log_tx(
                &mut tx,
                actor_user_id,
                "create",
                created.lesson_id,
                None,
                None,
                None,
                None,
                Some(&after),
            )
            .await?;

            tx.commit().await?;

            Ok(created)
        }
        .await;

        match outcome {
            Ok(created) => {
                success += 1;
                results.push(LessonBulkResult {
                    lesson_id: Some(created.lesson_id),
                    lesson_idx: created.lesson_idx,
                    success: true,
                    error: None,
                });
            }
            Err(e) => {
                failure += 1;
                let msg = match e {
                    AppError::BadRequest(m) => m,
                    AppError::Unprocessable(m) => m,
                    AppError::Conflict(m) => m,
                    AppError::Forbidden => "Forbidden".to_string(),
                    _ => "Internal Server Error".to_string(),
                };

                results.push(LessonBulkResult {
                    lesson_id: None,
                    lesson_idx: lesson_idx.clone(),
                    success: false,
                    error: Some(msg),
                });
            }
        }
    }

    let all_success = failure == 0;

    Ok((
        all_success,
        LessonBulkCreateRes {
            success_count: success,
            failure_count: failure,
            results,
        },
    ))
}

pub async fn admin_bulk_update_lessons(
    st: &AppState,
    actor_user_id: i64,
    req: LessonBulkUpdateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<(bool, LessonBulkUpdateRes)> {
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
        "BULK_UPDATE_LESSONS",
        Some("LESSON"),
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
        let lesson_id = item.lesson_id;
        let outcome = async {
            if let Err(e) = item.validate() {
                return Err(AppError::BadRequest(e.to_string()));
            }

            let has_any = item.lesson_idx.is_some()
                || item.lesson_title.is_some()
                || item.lesson_subtitle.is_some()
                || item.lesson_description.is_some()
                || item.lesson_state.is_some()
                || item.lesson_access.is_some();

            if !has_any {
                return Err(AppError::BadRequest("no fields to update".into()));
            }

            let mut tx = st.db.begin().await?;

            let before = repo::find_lesson_by_id_tx(&mut tx, lesson_id)
                .await?
                .ok_or(AppError::NotFound)?;

            if let Some(ref idx) = item.lesson_idx {
                let trimmed = idx.trim();
                if trimmed.is_empty() {
                    return Err(AppError::BadRequest("lesson_idx is required".into()));
                }
                if trimmed != before.lesson_idx
                    && repo::exists_lesson_idx_excluding_id_tx(&mut tx, trimmed, lesson_id).await?
                {
                    return Err(AppError::Conflict("Lesson Index already exists".into()));
                }
            }

            if let Some(ref title) = item.lesson_title {
                if title.trim().is_empty() {
                    return Err(AppError::BadRequest("lesson_title is required".into()));
                }
            }

            let lesson_idx = item.lesson_idx.as_ref().map(|v| v.trim().to_string());
            let lesson_title = item.lesson_title.as_ref().map(|v| v.trim().to_string());
            let lesson_subtitle = item
                .lesson_subtitle
                .as_ref()
                .map(|v| v.trim().to_string())
                .filter(|v| !v.is_empty());
            let lesson_description = item
                .lesson_description
                .as_ref()
                .map(|v| v.trim().to_string())
                .filter(|v| !v.is_empty());

            let update_req = LessonUpdateItem {
                lesson_id,
                lesson_idx,
                lesson_title,
                lesson_subtitle,
                lesson_description,
                lesson_state: item.lesson_state,
                lesson_access: item.lesson_access,
            };

            repo::update_lesson_tx(&mut tx, actor_user_id, lesson_id, &update_req).await?;

            let after = repo::find_lesson_by_id_tx(&mut tx, lesson_id)
                .await?
                .ok_or(AppError::NotFound)?;

            let before_val = serde_json::to_value(&before).unwrap_or_default();
            let after_val = serde_json::to_value(&after).unwrap_or_default();

            repo::create_lesson_log_tx(
                &mut tx,
                actor_user_id,
                "update",
                lesson_id,
                None,
                None,
                None,
                Some(&before_val),
                Some(&after_val),
            )
            .await?;

            tx.commit().await?;

            Ok(after)
        }
        .await;

        match outcome {
            Ok(_) => {
                success += 1;
                results.push(LessonBulkUpdateResult {
                    lesson_id,
                    success: true,
                    error: None,
                });
            }
            Err(e) => {
                failure += 1;
                let msg = match e {
                    AppError::NotFound => "Lesson not found".to_string(),
                    AppError::BadRequest(m) => m,
                    AppError::Unprocessable(m) => m,
                    AppError::Conflict(m) => m,
                    AppError::Forbidden => "Forbidden".to_string(),
                    _ => "Internal Server Error".to_string(),
                };

                results.push(LessonBulkUpdateResult {
                    lesson_id,
                    success: false,
                    error: Some(msg),
                });
            }
        }
    }

    let all_success = failure == 0;

    Ok((
        all_success,
        LessonBulkUpdateRes {
            success_count: success,
            failure_count: failure,
            results,
        },
    ))
}

pub async fn admin_update_lesson(
    st: &AppState,
    actor_user_id: i64,
    lesson_id: i32,
    req: LessonUpdateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminLessonRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "UPDATE_LESSON",
        Some("LESSON"),
        Some(lesson_id as i64),
        &serde_json::to_value(&req).unwrap_or(serde_json::Value::Null),
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    let has_any = req.lesson_idx.is_some()
        || req.lesson_title.is_some()
        || req.lesson_subtitle.is_some()
        || req.lesson_description.is_some()
        || req.lesson_state.is_some()
        || req.lesson_access.is_some();

    if !has_any {
        return Err(AppError::BadRequest("no fields to update".into()));
    }

    let mut tx = st.db.begin().await?;

    let before = repo::find_lesson_by_id_tx(&mut tx, lesson_id)
        .await?
        .ok_or(AppError::NotFound)?;

    if let Some(ref idx) = req.lesson_idx {
        let trimmed = idx.trim();
        if trimmed.is_empty() {
            return Err(AppError::BadRequest("lesson_idx is required".into()));
        }
        if trimmed != before.lesson_idx
            && repo::exists_lesson_idx_excluding_id_tx(&mut tx, trimmed, lesson_id).await?
        {
            return Err(AppError::Conflict("Lesson Index already exists".into()));
        }
    }

    if let Some(ref title) = req.lesson_title {
        if title.trim().is_empty() {
            return Err(AppError::BadRequest("lesson_title is required".into()));
        }
    }

    let lesson_idx = req.lesson_idx.as_ref().map(|v| v.trim().to_string());
    let lesson_title = req.lesson_title.as_ref().map(|v| v.trim().to_string());
    let lesson_subtitle = req
        .lesson_subtitle
        .as_ref()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty());
    let lesson_description = req
        .lesson_description
        .as_ref()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty());

    let update_req = LessonUpdateItem {
        lesson_id,
        lesson_idx,
        lesson_title,
        lesson_subtitle,
        lesson_description,
        lesson_state: req.lesson_state,
        lesson_access: req.lesson_access,
    };

    repo::update_lesson_tx(&mut tx, actor_user_id, lesson_id, &update_req).await?;

    let after = repo::find_lesson_by_id_tx(&mut tx, lesson_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let before_val = serde_json::to_value(&before).unwrap_or_default();
    let after_val = serde_json::to_value(&after).unwrap_or_default();

    repo::create_lesson_log_tx(
        &mut tx,
        actor_user_id,
        "update",
        lesson_id,
        None,
        None,
        None,
        Some(&before_val),
        Some(&after_val),
    )
    .await?;

    tx.commit().await?;

    Ok(after)
}

// ============================================
// 7-46: Lesson Detail
// ============================================

pub async fn admin_get_lesson(
    st: &AppState,
    actor_user_id: i64,
    lesson_id: i32,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminLessonRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    
    let details = serde_json::json!({
        "lesson_id": lesson_id
    });

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "GET_LESSON_DETAIL",
        Some("LESSON"),
        Some(lesson_id as i64),
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    let lesson = repo::find_lesson_by_id(&st.db, lesson_id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(lesson)
}

// ============================================
// 7-52: Lesson Items Detail (with video/study_task)
// ============================================

pub async fn admin_get_lesson_items_detail(
    st: &AppState,
    actor_user_id: i64,
    lesson_id: i32,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminLessonItemsDetailRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    
    let details = serde_json::json!({
        "lesson_id": lesson_id
    });

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "GET_LESSON_ITEMS_DETAIL",
        Some("LESSON_ITEM"),
        Some(lesson_id as i64),
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    // Verify lesson exists
    let lesson = repo::find_lesson_by_id(&st.db, lesson_id)
        .await?
        .ok_or(AppError::NotFound)?;

    // Get items with video/study_task details
    let items = repo::get_lesson_items_with_details(&st.db, lesson_id).await?;
    let total_items = repo::count_lesson_items(&st.db, lesson_id).await?;

    Ok(AdminLessonItemsDetailRes {
        lesson_id,
        lesson_title: lesson.lesson_title,
        total_items,
        items,
    })
}

// ============================================
// 7-58: Lesson Progress Detail (with current item)
// ============================================

pub async fn admin_get_lesson_progress_detail(
    st: &AppState,
    actor_user_id: i64,
    lesson_id: i32,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminLessonProgressListDetailRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    
    let details = serde_json::json!({
        "lesson_id": lesson_id
    });

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "GET_LESSON_PROGRESS_DETAIL",
        Some("LESSON_PROGRESS"),
        Some(lesson_id as i64),
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    // Verify lesson exists
    let lesson = repo::find_lesson_by_id(&st.db, lesson_id)
        .await?
        .ok_or(AppError::NotFound)?;

    // Get progress with current item details
    let list = repo::get_lesson_progress_with_items(&st.db, lesson_id).await?;
    let total_progress = repo::count_lesson_progress(&st.db, lesson_id).await?;

    Ok(AdminLessonProgressListDetailRes {
        lesson_id,
        lesson_title: lesson.lesson_title,
        total_progress,
        list,
    })
}

// ============================================
// DELETE: Lesson Item
// ============================================

pub async fn admin_delete_lesson_item(
    st: &AppState,
    actor_user_id: i64,
    lesson_id: i32,
    seq: i32,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<()> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    
    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "DELETE_LESSON_ITEM",
        Some("LESSON_ITEM"),
        Some(lesson_id as i64),
        &serde_json::json!({
            "lesson_id": lesson_id,
            "lesson_item_seq": seq
        }),
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    let mut tx = st.db.begin().await?;

    // Find the item to delete (for logging)
    let before = repo::find_lesson_item_tx(&mut tx, lesson_id, seq)
        .await?
        .ok_or(AppError::NotFound)?;

    // Delete the item
    repo::delete_lesson_item_tx(&mut tx, lesson_id, seq).await?;

    // Log the deletion
    let before_val = serde_json::to_value(&before).unwrap_or_default();
    repo::create_lesson_log_tx(
        &mut tx,
        actor_user_id,
        "delete",
        lesson_id,
        Some(seq),
        before.video_id,
        before.study_task_id,
        Some(&before_val),
        None,
    )
    .await?;

    tx.commit().await?;

    Ok(())
}
