use std::net::IpAddr; // [필수] IpAddr 타입 사용
use std::str::FromStr; // [필수] String -> IpAddr 변환용 trait
use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::types::{StudyProgram, StudyState, UserAuth};
use crate::AppState;

use super::dto::{
    AdminStudyListRes, AdminStudyRes, StudyBulkCreateReq, StudyBulkCreateRes, StudyBulkResult,
    StudyBulkUpdateReq, StudyBulkUpdateRes, StudyBulkUpdateResult, StudyCreateReq, StudyListReq,
    StudyUpdateReq,
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
    ip_address: Option<String>,
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

    // [수정] String IP를 IpAddr 타입으로 변환 (변수 선언 추가)
    let ip_addr: Option<IpAddr> = ip_address
        .as_deref()
        .and_then(|ip| IpAddr::from_str(ip).ok());

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
        None, // target_id
        None, // target_sub_id (5번째 인자)
        &details,
        ip_addr, // [수정] 위에서 변환한 ip_addr 변수 사용
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

pub async fn admin_create_study(
    st: &AppState,
    actor_user_id: i64,
    req: StudyCreateReq,
    ip_address: Option<String>,
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

    let _ip_addr: Option<IpAddr> = ip_address
        .as_deref()
        .and_then(|ip| IpAddr::from_str(ip).ok());
    let _user_agent = user_agent;

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
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> AppResult<(bool, StudyBulkCreateRes)> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    let ip_addr: Option<IpAddr> = ip_address
        .as_deref()
        .and_then(|ip| IpAddr::from_str(ip).ok());

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
        ip_addr,
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
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> AppResult<AdminStudyRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    if let Some(idx) = req.study_idx.as_mut() {
        *idx = idx.trim().to_string();
    }

    let has_any = req.study_idx.is_some()
        || req.study_state.is_some()
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

    let _ip_addr: Option<IpAddr> = ip_address
        .as_deref()
        .and_then(|ip| IpAddr::from_str(ip).ok());
    let _user_agent = user_agent;

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

pub async fn admin_bulk_update_studies(
    st: &AppState,
    actor_user_id: i64,
    req: StudyBulkUpdateReq,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> AppResult<(bool, StudyBulkUpdateRes)> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    let ip_addr: Option<IpAddr> = ip_address
        .as_deref()
        .and_then(|ip| IpAddr::from_str(ip).ok());

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
        ip_addr,
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
            };

            if let Some(idx) = update_req.study_idx.as_mut() {
                *idx = idx.trim().to_string();
            }

            let has_any = update_req.study_idx.is_some()
                || update_req.study_state.is_some()
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
