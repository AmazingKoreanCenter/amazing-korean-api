use std::net::IpAddr;
use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::types::{StudyAccess, StudyProgram, StudyState, UserAuth};
use crate::AppState;

use super::dto::{
    AdminStudyDetailRes, AdminStudyListRes, AdminStudyRes, AdminStudyTaskDetailRes,
    AdminStudyTaskListRes, AdminTaskExplainListRes, AdminTaskExplainRes, StudyBulkCreateReq,
    StudyBulkCreateRes, StudyBulkResult, StudyBulkUpdateReq, StudyBulkUpdateRes,
    StudyBulkUpdateResult, StudyCreateReq, StudyListReq, StudyTaskBulkCreateReq,
    StudyTaskBulkCreateRes, StudyTaskBulkResult, StudyTaskBulkUpdateReq, StudyTaskBulkUpdateRes,
    StudyTaskBulkUpdateResult, StudyTaskCreateReq, StudyTaskListReq, StudyTaskUpdateReq,
    StudyUpdateReq, TaskExplainBulkCreateReq, TaskExplainBulkCreateRes, TaskExplainBulkResult,
    TaskExplainBulkUpdateReq, TaskExplainBulkUpdateRes, TaskExplainBulkUpdateResult,
    TaskExplainCreateReq, TaskExplainListReq, TaskExplainUpdateReq, TaskStatusBulkUpdateReq,
    TaskStatusBulkUpdateRes, TaskStatusBulkUpdateResult, TaskStatusListReq, TaskStatusUpdateReq,
    AdminTaskStatusListRes, AdminTaskStatusRes,
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

pub async fn admin_list_studies(
    st: &AppState,
    actor_user_id: i64,
    req: StudyListReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminStudyListRes> {
    // 1. RBAC
    check_admin_rbac(&st.db, actor_user_id).await?;

    // 2. Validate
    req.validate()?;

    let page = req.page.unwrap_or(1);
    let size = req.size.unwrap_or(20);
    let sort = req.sort.as_deref().unwrap_or("created_at");
    let order = req.order.as_deref().unwrap_or("desc");
    let q = req.q.clone();

    
    // 3. Audit Log
    let details = serde_json::json!({
        "q": q.as_deref(),
        "page": page,
        "size": size,
        "program": req.study_program,
        "state": req.study_state
    });

    // [수정] 인자 개수 8개로 맞춤 (trace_id 제거)
    // 순서: (db, actor, action, target_id, target_sub_id, details, ip, ua)
    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "LIST_STUDIES",
        Some("STUDY"), // target_id
        None, // target_sub_id (5번째 인자)
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    // 4. Repo Call
    let (total, list) = repo::admin_list_studies(
        &st.db,
        q,
        page,
        size,
        sort,
        order,
        req.study_state,
        req.study_access,
        req.study_program,
    )
    .await?;

    Ok(AdminStudyListRes {
        list,
        total,
        page,
        size,
        total_pages: (total as f64 / size as f64).ceil() as i64,
    })
}

/// Study 상세 조회
pub async fn admin_get_study(
    st: &AppState,
    actor_user_id: i64,
    study_id: i64,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminStudyDetailRes> {
    // 1. RBAC
    check_admin_rbac(&st.db, actor_user_id).await?;

    
    // 3. Audit Log
    let details = serde_json::json!({
        "study_id": study_id
    });

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "GET_STUDY",
        Some("STUDY"),
        Some(study_id),
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    // 4. Repo Call
    let study = repo::admin_get_study_detail(&st.db, study_id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(study)
}

pub async fn admin_create_study(
    st: &AppState,
    actor_user_id: i64,
    req: StudyCreateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminStudyRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    req.validate()?;

    let study_idx = req.study_idx.trim();
    if repo::exists_study_idx(&st.db, study_idx).await? {
        return Err(AppError::Conflict("study_idx already exists".into()));
    }

    let study_program = req.study_program.unwrap_or(StudyProgram::Tbc);
    let study_state = req.study_state.unwrap_or(StudyState::Ready);
    let study_access = req.study_access.unwrap_or(StudyAccess::Public);

    let study_title = req
        .study_title
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty());
    let study_subtitle = req
        .study_subtitle
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty());
    let study_description = req
        .study_description
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty());

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "CREATE_STUDY",
        Some("STUDY"),
        None,
        &serde_json::to_value(&req).unwrap_or(serde_json::Value::Null),
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    let mut tx = st.db.begin().await?;

    let created = repo::admin_create_study(
        &mut tx,
        actor_user_id,
        study_idx,
        study_title,
        study_subtitle,
        study_description,
        study_program,
        study_state,
        study_access,
    )
    .await;

    let created = match created {
        Ok(val) => val,
        Err(e) if is_unique_violation(&e) => {
            return Err(AppError::Conflict("study_idx already exists".into()));
        }
        Err(e) => return Err(e),
    };

    let after = serde_json::to_value(&created).unwrap_or_default();
    repo::create_study_log(
        &mut tx,
        actor_user_id,
        "CREATE_STUDY",
        created.study_id as i64,
        None,
        None,
        Some(&after),
    )
    .await?;

    tx.commit().await?;

    Ok(created)
}

pub async fn admin_bulk_create_studies(
    st: &AppState,
    actor_user_id: i64,
    req: StudyBulkCreateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<(bool, StudyBulkCreateRes)> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    
    let details = serde_json::json!({
        "total": req.items.len()
    });

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "BULK_CREATE_STUDIES",
        Some("study"),
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
        let study_idx = item.study_idx.trim().to_string();
        let outcome = async {
            if let Err(e) = item.validate() {
                return Err(AppError::BadRequest(e.to_string()));
            }

            if repo::exists_study_idx(&st.db, &study_idx).await? {
                return Err(AppError::Conflict("study_idx already exists".into()));
            }

            let study_program = item.study_program.unwrap_or(StudyProgram::Tbc);
            let study_state = item.study_state.unwrap_or(StudyState::Ready);
            let study_access = item.study_access.unwrap_or(StudyAccess::Public);

            let study_title = item
                .study_title
                .as_deref()
                .map(str::trim)
                .filter(|v| !v.is_empty());
            let study_subtitle = item
                .study_subtitle
                .as_deref()
                .map(str::trim)
                .filter(|v| !v.is_empty());
            let study_description = item
                .study_description
                .as_deref()
                .map(str::trim)
                .filter(|v| !v.is_empty());

            let mut tx = st.db.begin().await?;

            let created = repo::admin_create_study(
                &mut tx,
                actor_user_id,
                &study_idx,
                study_title,
                study_subtitle,
                study_description,
                study_program,
                study_state,
                study_access,
            )
            .await;

            let created = match created {
                Ok(val) => val,
                Err(e) if is_unique_violation(&e) => {
                    return Err(AppError::Conflict("study_idx already exists".into()));
                }
                Err(e) => return Err(e),
            };

            let after = serde_json::to_value(&created).unwrap_or_default();
            repo::create_study_log(
                &mut tx,
                actor_user_id,
                "CREATE_STUDY",
                created.study_id as i64,
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
                results.push(StudyBulkResult {
                    id: Some(created.study_id as i64),
                    idx: created.study_idx,
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

                results.push(StudyBulkResult {
                    id: None,
                    idx: study_idx,
                    success: false,
                    error: Some(msg),
                });
            }
        }
    }

    let all_success = failure == 0;

    Ok((
        all_success,
        StudyBulkCreateRes {
            success_count: success,
            failure_count: failure,
            results,
        },
    ))
}

pub async fn admin_update_study(
    st: &AppState,
    actor_user_id: i64,
    study_id: i64,
    mut req: StudyUpdateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminStudyRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "UPDATE_STUDY",
        Some("STUDY"),
        Some(study_id),
        &serde_json::to_value(&req).unwrap_or(serde_json::Value::Null),
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    if let Some(idx) = req.study_idx.as_mut() {
        *idx = idx.trim().to_string();
    }

    let has_any = req.study_idx.is_some()
        || req.study_state.is_some()
        || req.study_access.is_some()
        || req.study_program.is_some()
        || req.study_title.is_some()
        || req.study_subtitle.is_some()
        || req.study_description.is_some();

    if !has_any {
        return Err(AppError::BadRequest("no fields to update".into()));
    }

    let before = repo::find_study_by_id(&st.db, study_id)
        .await?
        .ok_or(AppError::NotFound)?;

    if let Some(idx) = req.study_idx.as_deref() {
        if idx != before.study_idx
            && repo::exists_study_idx_for_update(&st.db, study_id, idx).await?
        {
            return Err(AppError::Conflict("study_idx already exists".into()));
        }
    }

    // audit log를 통해 기록됨

    let mut tx = st.db.begin().await?;

    let updated = repo::admin_update_study(&mut tx, study_id, actor_user_id, &req).await?;

    let before_val = serde_json::to_value(&before).unwrap_or_default();
    let after_val = serde_json::to_value(&req).unwrap_or_default();

    repo::create_study_log(
        &mut tx,
        actor_user_id,
        "UPDATE_STUDY",
        updated.study_id as i64,
        None,
        Some(&before_val),
        Some(&after_val),
    )
    .await?;

    tx.commit().await?;

    Ok(updated)
}

pub async fn admin_list_study_tasks(
    st: &AppState,
    actor_user_id: i64,
    req: StudyTaskListReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminStudyTaskListRes> {
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

    
    let details = serde_json::json!({
        "study_id": req.study_id
    });

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "LIST_STUDY_TASKS",
        Some("STUDY_TASK"),
        Some(req.study_id as i64),
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    let (total, list) =
        repo::admin_list_study_tasks(&st.db, req.study_id, page, size).await?;

    let total_pages = if total == 0 {
        0
    } else {
        (total + size as i64 - 1) / size as i64
    };

    Ok(AdminStudyTaskListRes {
        list,
        total,
        page,
        size,
        total_pages,
    })
}

pub async fn admin_get_study_task(
    st: &AppState,
    actor_user_id: i64,
    task_id: i64,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminStudyTaskDetailRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    
    let details = serde_json::json!({
        "task_id": task_id
    });

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "VIEW_STUDY_TASK",
        Some("STUDY_TASK"),
        Some(task_id),
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    let task = repo::find_study_task_by_id(&st.db, task_id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(task)
}

pub async fn admin_list_task_explains(
    st: &AppState,
    actor_user_id: i64,
    req: TaskExplainListReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminTaskExplainListRes> {
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

    
    let details = serde_json::json!({
        "page": page,
        "size": size
    });

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "LIST_TASK_EXPLAINS",
        Some("STUDY_TASK_EXPLAIN"),
        Some(req.task_id as i64),
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    let (total, list) =
        repo::admin_list_task_explains(&st.db, req.task_id, page, size).await?;

    let total_pages = if total == 0 {
        0
    } else {
        (total + size as i64 - 1) / size as i64
    };

    Ok(AdminTaskExplainListRes {
        list,
        total,
        page,
        size,
        total_pages,
    })
}

pub async fn admin_list_task_status(
    st: &AppState,
    actor_user_id: i64,
    req: TaskStatusListReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminTaskStatusListRes> {
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

    
    let details = serde_json::json!({
        "task_id": req.task_id,
        "user_id": req.user_id
    });

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "LIST_TASK_STATUS",
        Some("STUDY_TASK_STATUS"),
        req.task_id.map(|id| id as i64),
        &details,
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    let (total, list) =
        repo::admin_list_task_status(&st.db, req.task_id, req.user_id, page, size).await?;

    let total_pages = if total == 0 {
        0
    } else {
        (total + size as i64 - 1) / size as i64
    };

    Ok(AdminTaskStatusListRes {
        list,
        total,
        page,
        size,
        total_pages,
    })
}

pub async fn admin_update_task_status(
    st: &AppState,
    actor_user_id: i64,
    task_id: i64,
    req: TaskStatusUpdateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminTaskStatusRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "UPDATE_TASK_STATUS",
        Some("STUDY_TASK_STATUS"),
        Some(task_id),
        &serde_json::to_value(&req).unwrap_or(serde_json::Value::Null),
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    let has_any = req.study_task_status_try_count.is_some()
        || req.study_task_status_is_solved.is_some()
        || req.study_task_status_last_attempt_at.is_some();

    if !has_any {
        return Err(AppError::BadRequest("no fields to update".into()));
    }

    let task_id_i32 = i32::try_from(task_id)
        .map_err(|_| AppError::BadRequest("task_id out of range".into()))?;

    let before = repo::find_task_status(&st.db, task_id_i32, req.user_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let task = repo::find_study_task_by_id(&st.db, task_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let mut tx = st.db.begin().await?;

    repo::update_task_status(&mut tx, task_id_i32, &req).await?;

    let after = repo::find_task_status_tx(&mut tx, task_id_i32, req.user_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let before_val = serde_json::to_value(&before).unwrap_or_default();
    let after_val = serde_json::to_value(&after).unwrap_or_default();

    repo::create_study_log(
        &mut tx,
        actor_user_id,
        "update",
        task.study_id as i64,
        Some(task.study_task_id),
        Some(&before_val),
        Some(&after_val),
    )
    .await?;

    tx.commit().await?;

    Ok(after)
}

pub async fn admin_create_task_explain(
    st: &AppState,
    actor_user_id: i64,
    task_id: i64,
    req: TaskExplainCreateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminTaskExplainRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "CREATE_TASK_EXPLAIN",
        Some("STUDY_TASK_EXPLAIN"),
        Some(task_id),
        &serde_json::to_value(&req).unwrap_or(serde_json::Value::Null),
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    let before = repo::find_study_task_by_id(&st.db, task_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let task_id_i32 = i32::try_from(task_id)
        .map_err(|_| AppError::BadRequest("task_id out of range".into()))?;

    if repo::exists_task_explain(&st.db, task_id_i32, req.explain_lang).await? {
        return Err(AppError::Conflict("task explain already exists".into()));
    }

    let mut tx = st.db.begin().await?;

    let created = repo::create_task_explain(&mut tx, task_id_i32, &req).await;
    let created = match created {
        Ok(val) => val,
        Err(e) if is_unique_violation(&e) => {
            return Err(AppError::Conflict("task explain already exists".into()));
        }
        Err(e) => return Err(e),
    };

    let after = serde_json::to_value(&created).unwrap_or_default();
    repo::create_study_log(
        &mut tx,
        actor_user_id,
        "create",
        before.study_id as i64,
        Some(before.study_task_id),
        None,
        Some(&after),
    )
    .await?;

    tx.commit().await?;

    Ok(created)
}

pub async fn admin_update_task_explain(
    st: &AppState,
    actor_user_id: i64,
    task_id: i64,
    req: TaskExplainUpdateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminTaskExplainRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "UPDATE_TASK_EXPLAIN",
        Some("STUDY_TASK_EXPLAIN"),
        Some(task_id),
        &serde_json::to_value(&req).unwrap_or(serde_json::Value::Null),
        ip_address,
        user_agent.as_deref(),
    )
    .await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    let has_any = req.explain_title.is_some()
        || req.explain_text.is_some()
        || req.explain_media_url.is_some();

    if !has_any {
        return Err(AppError::BadRequest("no fields to update".into()));
    }

    let task_id_i32 = i32::try_from(task_id)
        .map_err(|_| AppError::BadRequest("task_id out of range".into()))?;

    let before = repo::find_task_explain(&st.db, task_id_i32, req.explain_lang)
        .await?
        .ok_or(AppError::NotFound)?;

    let task = repo::find_study_task_by_id(&st.db, task_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let mut tx = st.db.begin().await?;

    repo::update_task_explain(&mut tx, task_id_i32, &req).await?;

    let after = repo::find_task_explain_tx(&mut tx, task_id_i32, req.explain_lang)
        .await?
        .ok_or(AppError::NotFound)?;

    let before_val = serde_json::to_value(&before).unwrap_or_default();
    let after_val = serde_json::to_value(&after).unwrap_or_default();

    repo::create_study_log(
        &mut tx,
        actor_user_id,
        "update",
        task.study_id as i64,
        Some(task.study_task_id),
        Some(&before_val),
        Some(&after_val),
    )
    .await?;

    tx.commit().await?;

    Ok(after)
}

pub async fn admin_bulk_create_task_explains(
    st: &AppState,
    actor_user_id: i64,
    req: TaskExplainBulkCreateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<(bool, TaskExplainBulkCreateRes)> {
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
        "BULK_CREATE_TASK_EXPLAINS",
        Some("STUDY_TASK_EXPLAIN"),
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
        let task_id = item.study_task_id;
        let lang = item.explain_lang;
        let outcome = async {
            if let Err(e) = item.validate() {
                return Err(AppError::BadRequest(e.to_string()));
            }

            let before = repo::find_study_task_by_id(&st.db, task_id as i64)
                .await?
                .ok_or(AppError::NotFound)?;

            if repo::exists_task_explain(&st.db, task_id, lang).await? {
                return Err(AppError::Conflict("task explain already exists".into()));
            }

            let create_req = TaskExplainCreateReq {
                explain_lang: lang,
                explain_title: item.explain_title.clone(),
                explain_text: item.explain_text.clone(),
                explain_media_url: item.explain_media_url.clone(),
            };

            let mut tx = st.db.begin().await?;

            let created = repo::create_task_explain(&mut tx, task_id, &create_req).await;
            let created = match created {
                Ok(val) => val,
                Err(e) if is_unique_violation(&e) => {
                    return Err(AppError::Conflict("task explain already exists".into()));
                }
                Err(e) => return Err(e),
            };

            let after = serde_json::to_value(&created).unwrap_or_default();
            repo::create_study_log(
                &mut tx,
                actor_user_id,
                "create",
                before.study_id as i64,
                Some(before.study_task_id),
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
                results.push(TaskExplainBulkResult {
                    study_task_id: task_id,
                    explain_lang: lang,
                    success: true,
                    error: None,
                });
            }
            Err(e) => {
                failure += 1;
                let msg = match e {
                    AppError::NotFound => "Study task not found".to_string(),
                    AppError::BadRequest(m) => m,
                    AppError::Unprocessable(m) => m,
                    AppError::Conflict(m) => m,
                    AppError::Forbidden => "Forbidden".to_string(),
                    _ => "Internal Server Error".to_string(),
                };

                results.push(TaskExplainBulkResult {
                    study_task_id: task_id,
                    explain_lang: lang,
                    success: false,
                    error: Some(msg),
                });
            }
        }
    }

    let all_success = failure == 0;

    Ok((
        all_success,
        TaskExplainBulkCreateRes {
            success_count: success,
            failure_count: failure,
            results,
        },
    ))
}

pub async fn admin_bulk_update_task_explains(
    st: &AppState,
    actor_user_id: i64,
    req: TaskExplainBulkUpdateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<(bool, TaskExplainBulkUpdateRes)> {
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
        "BULK_UPDATE_TASK_EXPLAINS",
        Some("STUDY_TASK_EXPLAIN"),
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
        let task_id = item.study_task_id;
        let lang = item.explain_lang;
        let outcome = async {
            if let Err(e) = item.validate() {
                return Err(AppError::BadRequest(e.to_string()));
            }

            let has_any = item.explain_title.is_some()
                || item.explain_text.is_some()
                || item.explain_media_url.is_some();

            if !has_any {
                return Err(AppError::BadRequest("no fields to update".into()));
            }

            let before = repo::find_task_explain(&st.db, task_id, lang)
                .await?
                .ok_or(AppError::NotFound)?;

            let task = repo::find_study_task_by_id(&st.db, task_id as i64)
                .await?
                .ok_or(AppError::NotFound)?;

            let update_req = TaskExplainUpdateReq {
                explain_lang: lang,
                explain_title: item.explain_title.clone(),
                explain_text: item.explain_text.clone(),
                explain_media_url: item.explain_media_url.clone(),
            };

            let mut tx = st.db.begin().await?;

            repo::update_task_explain(&mut tx, task_id, &update_req).await?;

            let after = repo::find_task_explain_tx(&mut tx, task_id, lang)
                .await?
                .ok_or(AppError::NotFound)?;

            let before_val = serde_json::to_value(&before).unwrap_or_default();
            let after_val = serde_json::to_value(&after).unwrap_or_default();

            repo::create_study_log(
                &mut tx,
                actor_user_id,
                "update",
                task.study_id as i64,
                Some(task.study_task_id),
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
                results.push(TaskExplainBulkUpdateResult {
                    study_task_id: task_id,
                    explain_lang: lang,
                    success: true,
                    error: None,
                });
            }
            Err(e) => {
                failure += 1;
                let msg = match e {
                    AppError::NotFound => "Task explain not found".to_string(),
                    AppError::BadRequest(m) => m,
                    AppError::Unprocessable(m) => m,
                    AppError::Conflict(m) => m,
                    AppError::Forbidden => "Forbidden".to_string(),
                    _ => "Internal Server Error".to_string(),
                };

                results.push(TaskExplainBulkUpdateResult {
                    study_task_id: task_id,
                    explain_lang: lang,
                    success: false,
                    error: Some(msg),
                });
            }
        }
    }

    let all_success = failure == 0;

    Ok((
        all_success,
        TaskExplainBulkUpdateRes {
            success_count: success,
            failure_count: failure,
            results,
        },
    ))
}

pub async fn admin_bulk_update_task_status(
    st: &AppState,
    actor_user_id: i64,
    req: TaskStatusBulkUpdateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<(bool, TaskStatusBulkUpdateRes)> {
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
        "BULK_UPDATE_TASK_STATUS",
        Some("STUDY_TASK_STATUS"),
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
        let task_id = item.study_task_id;
        let user_id = item.user_id;
        let outcome = async {
            if let Err(e) = item.validate() {
                return Err(AppError::BadRequest(e.to_string()));
            }

            let has_any = item.study_task_status_try_count.is_some()
                || item.study_task_status_is_solved.is_some()
                || item.study_task_status_last_attempt_at.is_some();

            if !has_any {
                return Err(AppError::BadRequest("no fields to update".into()));
            }

            let mut tx = st.db.begin().await?;

            let before = repo::find_task_status_tx(&mut tx, task_id, user_id).await?;

            let task = repo::find_study_task_by_id_tx(&mut tx, task_id as i64).await?;

            let update_req: TaskStatusUpdateReq = item.into(); repo::update_task_status(&mut tx, task_id, &update_req).await?;

            let after = repo::find_task_status_tx(&mut tx, task_id, user_id).await?;

            let before_val = serde_json::to_value(&before).unwrap_or_default();
            let after_val = serde_json::to_value(&after).unwrap_or_default();

            repo::create_study_log(
                &mut tx,
                actor_user_id,
                "update",
                task.study_id as i64,
                Some(task.study_task_id),
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
                results.push(TaskStatusBulkUpdateResult {
                    study_task_id: task_id,
                    user_id,
                    success: true,
                    error: None,
                });
            }
            Err(e) => {
                failure += 1;
                let msg = match e {
                    AppError::NotFound => "Task status not found".to_string(),
                    AppError::BadRequest(m) => m,
                    AppError::Unprocessable(m) => m,
                    AppError::Conflict(m) => m,
                    AppError::Forbidden => "Forbidden".to_string(),
                    _ => "Internal Server Error".to_string(),
                };

                results.push(TaskStatusBulkUpdateResult {
                    study_task_id: task_id,
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
        TaskStatusBulkUpdateRes {
            success_count: success,
            failure_count: failure,
            results,
        },
    ))
}

pub async fn admin_create_study_task(
    st: &AppState,
    actor_user_id: i64,
    req: StudyTaskCreateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminStudyTaskDetailRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    // ✅ [추가 필요 1] API 요청 로그 (admin_action_log) 기록
    // 이 부분이 빠져 있어서 ACTION LOG에 남지 않았던 것입니다.
    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "CREATE_TASK",           // action_type
        Some("STUDY_TASK"),      // target_table
        Some(req.study_id as i64), // target_id (생성 전이라 부모 ID 기록)
        &serde_json::to_value(&req).unwrap_or(serde_json::Value::Null), // ✅ [수정 후] 변환 실패 시 Null을 사용하고, 참조(&)를 전달
        ip_address,
        user_agent.as_deref(),
    ).await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    let is_blank = |value: &Option<String>| {
        value
            .as_deref()
            .map(|v| v.trim().is_empty())
            .unwrap_or(true)
    };

    match req.study_task_kind {
        crate::types::StudyTaskKind::Choice => {
            if is_blank(&req.question)
                || is_blank(&req.choice_1)
                || is_blank(&req.choice_2)
                || is_blank(&req.choice_3)
                || is_blank(&req.choice_4)
            {
                return Err(AppError::BadRequest(
                    "choice requires question and 4 choices".into(),
                ));
            }
            let correct = req.choice_correct.ok_or_else(|| {
                AppError::BadRequest("choice_correct is required".into())
            })?;
            if !(1..=4).contains(&correct) {
                return Err(AppError::BadRequest(
                    "choice_correct must be between 1 and 4".into(),
                ));
            }
        }
        crate::types::StudyTaskKind::Typing => {
            if is_blank(&req.question) || is_blank(&req.answer) {
                return Err(AppError::BadRequest(
                    "typing requires question and answer".into(),
                ));
            }
        }
        crate::types::StudyTaskKind::Voice => {
            if is_blank(&req.question) || is_blank(&req.answer) {
                return Err(AppError::BadRequest(
                    "voice requires question and answer".into(),
                ));
            }
        }
    }

    // audit log를 통해 기록됨

    let mut tx = st.db.begin().await?;

    let study_task_seq = req.study_task_seq.unwrap_or(1);
    let created_id = match repo::create_study_task(
        &mut tx,
        actor_user_id,
        req.study_id,
        req.study_task_kind,
        study_task_seq,
    )
    .await
    {
        Ok(val) => val,
        Err(e) if is_unique_violation(&e) => {
            return Err(AppError::Conflict("study_task_seq already exists".into()));
        }
        Err(e) => return Err(e),
    };

    match req.study_task_kind {
        crate::types::StudyTaskKind::Choice => {
            repo::create_task_choice(&mut tx, created_id, &req).await?;
        }
        crate::types::StudyTaskKind::Typing => {
            repo::create_task_typing(&mut tx, created_id, &req).await?;
        }
        crate::types::StudyTaskKind::Voice => {
            repo::create_task_voice(&mut tx, created_id, &req).await?;
        }
    }

    let created = repo::find_study_task_by_id_tx(&mut tx, created_id).await?;

    let after = serde_json::to_value(&created).unwrap_or_default();
    repo::create_study_log(
        &mut tx,
        actor_user_id,
        "create",
        req.study_id as i64,
        Some(created.study_task_id),
        None,
        Some(&after),
    )
    .await?;

    tx.commit().await?;

    Ok(created)
}

pub async fn admin_bulk_create_study_tasks(
    st: &AppState,
    actor_user_id: i64,
    req: StudyTaskBulkCreateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<(bool, StudyTaskBulkCreateRes)> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    
    let details = serde_json::json!({
        "total": req.items.len()
    });

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "BULK_CREATE_TASKS",
        Some("study_task"),
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
        let seq = item.study_task_seq.unwrap_or(1);
        let kind = item.study_task_kind;
        let outcome = async {
            if let Err(e) = item.validate() {
                return Err(AppError::BadRequest(e.to_string()));
            }

            let study_exists = repo::find_study_by_id(&st.db, item.study_id as i64)
                .await?
                .is_some();
            if !study_exists {
                return Err(AppError::NotFound);
            }

            let is_blank = |value: &Option<String>| {
                value
                    .as_deref()
                    .map(|v| v.trim().is_empty())
                    .unwrap_or(true)
            };

            match item.study_task_kind {
                crate::types::StudyTaskKind::Choice => {
                    if is_blank(&item.question)
                        || is_blank(&item.choice_1)
                        || is_blank(&item.choice_2)
                        || is_blank(&item.choice_3)
                        || is_blank(&item.choice_4)
                    {
                        return Err(AppError::BadRequest(
                            "choice requires question and 4 choices".into(),
                        ));
                    }
                    let correct = item.choice_correct.ok_or_else(|| {
                        AppError::BadRequest("choice_correct is required".into())
                    })?;
                    if !(1..=4).contains(&correct) {
                        return Err(AppError::BadRequest(
                            "choice_correct must be between 1 and 4".into(),
                        ));
                    }
                }
                crate::types::StudyTaskKind::Typing => {
                    if is_blank(&item.question) || is_blank(&item.answer) {
                        return Err(AppError::BadRequest(
                            "typing requires question and answer".into(),
                        ));
                    }
                }
                crate::types::StudyTaskKind::Voice => {
                    if is_blank(&item.question) || is_blank(&item.answer) {
                        return Err(AppError::BadRequest(
                            "voice requires question and answer".into(),
                        ));
                    }
                }
            }

            let mut tx = st.db.begin().await?;

            let created_id = match repo::create_study_task(
                &mut tx,
                actor_user_id,
                item.study_id,
                item.study_task_kind,
                seq,
            )
            .await
            {
                Ok(val) => val,
                Err(e) if is_unique_violation(&e) => {
                    return Err(AppError::Conflict("study_task_seq already exists".into()));
                }
                Err(e) => return Err(e),
            };

            match item.study_task_kind {
                crate::types::StudyTaskKind::Choice => {
                    repo::create_task_choice(&mut tx, created_id, &item).await?;
                }
                crate::types::StudyTaskKind::Typing => {
                    repo::create_task_typing(&mut tx, created_id, &item).await?;
                }
                crate::types::StudyTaskKind::Voice => {
                    repo::create_task_voice(&mut tx, created_id, &item).await?;
                }
            }

            let created = repo::find_study_task_by_id_tx(&mut tx, created_id).await?;

            let after = serde_json::to_value(&created).unwrap_or_default();
            repo::create_study_log(
                &mut tx,
                actor_user_id,
                "create",
                item.study_id as i64,
                Some(created.study_task_id),
                None,
                Some(&after),
            )
            .await?;

            tx.commit().await?;

            Ok(created_id)
        }
        .await;

        match outcome {
            Ok(task_id) => {
                success += 1;
                results.push(StudyTaskBulkResult {
                    task_id: Some(task_id),
                    seq,
                    kind,
                    error: None,
                });
            }
            Err(e) => {
                failure += 1;
                let msg = match e {
                    AppError::NotFound => "Study not found".to_string(),
                    AppError::BadRequest(m) => m,
                    AppError::Unprocessable(m) => m,
                    AppError::Conflict(m) => m,
                    AppError::Forbidden => "Forbidden".to_string(),
                    _ => "Internal Server Error".to_string(),
                };

                results.push(StudyTaskBulkResult {
                    task_id: None,
                    seq,
                    kind,
                    error: Some(msg),
                });
            }
        }
    }

    let all_success = failure == 0;

    Ok((
        all_success,
        StudyTaskBulkCreateRes {
            success_count: success,
            failure_count: failure,
            results,
        },
    ))
}

pub async fn admin_bulk_update_study_tasks(
    st: &AppState,
    actor_user_id: i64,
    req: StudyTaskBulkUpdateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<(bool, StudyTaskBulkUpdateRes)> {
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
        "BULK_UPDATE_TASKS",
        Some("STUDY_TASK"),
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
        let task_id = item.study_task_id as i64;
        let outcome = async {
            if let Err(e) = item.validate() {
                return Err(AppError::BadRequest(e.to_string()));
            }

            let update_req = StudyTaskUpdateReq {
                study_task_seq: item.study_task_seq,
                question: item.question.clone(),
                answer: item.answer.clone(),
                image_url: item.image_url.clone(),
                audio_url: item.audio_url.clone(),
                choice_1: item.choice_1.clone(),
                choice_2: item.choice_2.clone(),
                choice_3: item.choice_3.clone(),
                choice_4: item.choice_4.clone(),
                choice_correct: item.choice_correct,
            };

            let has_any = update_req.study_task_seq.is_some()
                || update_req.question.is_some()
                || update_req.answer.is_some()
                || update_req.image_url.is_some()
                || update_req.audio_url.is_some()
                || update_req.choice_1.is_some()
                || update_req.choice_2.is_some()
                || update_req.choice_3.is_some()
                || update_req.choice_4.is_some()
                || update_req.choice_correct.is_some();

            if !has_any {
                return Err(AppError::BadRequest("no fields to update".into()));
            }

            let before = repo::find_study_task_by_id(&st.db, task_id)
                .await?
                .ok_or(AppError::NotFound)?;

            let mut tx = st.db.begin().await?;

            let updated = repo::admin_update_study_task(
                &mut tx,
                task_id,
                actor_user_id,
                before.study_task_kind,
                &update_req,
            )
            .await?;

            let before_val = serde_json::to_value(&before).unwrap_or_default();
            let after_val = serde_json::to_value(&update_req).unwrap_or_default();

            repo::create_study_log(
                &mut tx,
                actor_user_id,
                "update",
                before.study_id as i64,
                Some(before.study_task_id),
                Some(&before_val),
                Some(&after_val),
            )
            .await?;

            tx.commit().await?;

            Ok(updated)
        }
        .await;

        match outcome {
            Ok(_) => {
                success += 1;
                results.push(StudyTaskBulkUpdateResult {
                    task_id,
                    success: true,
                    error: None,
                });
            }
            Err(e) => {
                failure += 1;
                let msg = match e {
                    AppError::NotFound => "Study task not found".to_string(),
                    AppError::BadRequest(m) => m,
                    AppError::Unprocessable(m) => m,
                    AppError::Conflict(m) => m,
                    AppError::Forbidden => "Forbidden".to_string(),
                    _ => "Internal Server Error".to_string(),
                };

                results.push(StudyTaskBulkUpdateResult {
                    task_id,
                    success: false,
                    error: Some(msg),
                });
            }
        }
    }

    let all_success = failure == 0;

    Ok((
        all_success,
        StudyTaskBulkUpdateRes {
            success_count: success,
            failure_count: failure,
            results,
        },
    ))
}

pub async fn admin_update_study_task(
    st: &AppState,
    actor_user_id: i64,
    study_task_id: i64,
    req: StudyTaskUpdateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<AdminStudyTaskDetailRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    // ✅ [추가] API 요청 로그 (admin_action_log) 기록
    // 이 부분이 없어서 API 호출 기록이 안 남았던 것입니다.
    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "UPDATE_TASK",           // action_type (API 로그는 보통 대문자 사용)
        Some("STUDY_TASK"),      // target_table
        Some(study_task_id as i64),    // target_id
        &serde_json::to_value(&req).unwrap_or(serde_json::Value::Null), // details
        ip_address,
        user_agent.as_deref(),
    ).await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    let has_any = req.study_task_seq.is_some()
        || req.question.is_some()
        || req.answer.is_some()
        || req.image_url.is_some()
        || req.audio_url.is_some()
        || req.choice_1.is_some()
        || req.choice_2.is_some()
        || req.choice_3.is_some()
        || req.choice_4.is_some()
        || req.choice_correct.is_some();

    if !has_any {
        return Err(AppError::BadRequest("no fields to update".into()));
    }

    let before = repo::find_study_task_by_id(&st.db, study_task_id)
        .await?
        .ok_or(AppError::NotFound)?;

    // audit log를 통해 기록됨

    let mut tx = st.db.begin().await?;

    let updated = repo::admin_update_study_task(
        &mut tx,
        study_task_id,
        actor_user_id,
        before.study_task_kind,
        &req,
    )
    .await?;

    let before_val = serde_json::to_value(&before).unwrap_or_default();
    let after_val = serde_json::to_value(&req).unwrap_or_default();

    repo::create_study_log(
        &mut tx,
        actor_user_id,
        "UPDATE_TASK",
        before.study_id as i64,
        Some(before.study_task_id),
        Some(&before_val),
        Some(&after_val),
    )
    .await?;

    tx.commit().await?;

    Ok(updated)
}

pub async fn admin_bulk_update_studies(
    st: &AppState,
    actor_user_id: i64,
    req: StudyBulkUpdateReq,
    ip_address: Option<IpAddr>,
    user_agent: Option<String>,
) -> AppResult<(bool, StudyBulkUpdateRes)> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    
    let details = serde_json::json!({
        "total": req.items.len()
    });

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "BULK_UPDATE_STUDIES",
        Some("study"),
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
        let item_id = item.id;
        let outcome = async {
            if let Err(e) = item.validate() {
                return Err(AppError::BadRequest(e.to_string()));
            }

            let mut update_req = StudyUpdateReq {
                study_idx: item.study_idx.clone(),
                study_title: item.study_title.clone(),
                study_subtitle: item.study_subtitle.clone(),
                study_description: item.study_description.clone(),
                study_program: item.study_program,
                study_state: item.study_state,
                study_access: item.study_access,
            };

            if let Some(idx) = update_req.study_idx.as_mut() {
                *idx = idx.trim().to_string();
            }

            let has_any = update_req.study_idx.is_some()
                || update_req.study_state.is_some()
                || update_req.study_access.is_some()
                || update_req.study_program.is_some()
                || update_req.study_title.is_some()
                || update_req.study_subtitle.is_some()
                || update_req.study_description.is_some();

            if !has_any {
                return Err(AppError::BadRequest("no fields to update".into()));
            }

            let before = repo::find_study_by_id(&st.db, item_id)
                .await?
                .ok_or(AppError::NotFound)?;

            if let Some(idx) = update_req.study_idx.as_deref() {
                if idx != before.study_idx
                    && repo::exists_study_idx_for_update(&st.db, item_id, idx).await?
                {
                    return Err(AppError::Conflict("study_idx already exists".into()));
                }
            }

            let mut tx = st.db.begin().await?;

            let updated = repo::admin_update_study(&mut tx, item_id, actor_user_id, &update_req)
                .await?;

            let before_val = serde_json::to_value(&before).unwrap_or_default();
            let after_val = serde_json::to_value(&update_req).unwrap_or_default();

            repo::create_study_log(
                &mut tx,
                actor_user_id,
                "UPDATE_STUDY",
                updated.study_id as i64,
                None,
                Some(&before_val),
                Some(&after_val),
            )
            .await?;

            tx.commit().await?;

            Ok(updated)
        }
        .await;

        match outcome {
            Ok(_) => {
                success += 1;
                results.push(StudyBulkUpdateResult {
                    id: item_id,
                    success: true,
                    error: None,
                });
            }
            Err(e) => {
                failure += 1;
                let msg = match e {
                    AppError::NotFound => "Study not found".to_string(),
                    AppError::BadRequest(m) => m,
                    AppError::Unprocessable(m) => m,
                    AppError::Conflict(m) => m,
                    AppError::Forbidden => "Forbidden".to_string(),
                    _ => "Internal Server Error".to_string(),
                };

                results.push(StudyBulkUpdateResult {
                    id: item_id,
                    success: false,
                    error: Some(msg),
                });
            }
        }
    }

    let all_success = failure == 0;

    Ok((
        all_success,
        StudyBulkUpdateRes {
            success_count: success,
            failure_count: failure,
            results,
        },
    ))
}
