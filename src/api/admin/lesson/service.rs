use std::net::IpAddr;
use std::str::FromStr;

use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::types::UserAuth;
use crate::AppState;

use super::dto::{
    AdminLessonListRes, AdminLessonRes, LessonBulkCreateReq, LessonBulkCreateRes,
    LessonBulkResult, LessonBulkUpdateReq, LessonBulkUpdateRes, LessonBulkUpdateResult,
    LessonCreateReq, LessonListReq, LessonUpdateItem,
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
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> AppResult<AdminLessonListRes> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    req.validate()?;

    let page = req.page.unwrap_or(1);
    let size = req.size.unwrap_or(20);
    let sort = req.sort.as_deref().unwrap_or("created_at");
    let order = req.order.as_deref().unwrap_or("desc");
    let q = req.q.clone();

    let ip_addr: Option<IpAddr> = ip_address
        .as_deref()
        .and_then(|ip| IpAddr::from_str(ip).ok());

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
        ip_addr,
        user_agent.as_deref(),
    )
    .await?;

    let (total, list) = repo::admin_list_lessons(&st.db, q, page, size, sort, order).await?;

    Ok(AdminLessonListRes {
        list,
        total,
        page,
        size,
        total_pages: (total as f64 / size as f64).ceil() as i64,
    })
}

pub async fn admin_create_lesson(
    st: &AppState,
    actor_user_id: i64,
    req: LessonCreateReq,
    ip_address: Option<String>,
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

    let ip_addr: Option<IpAddr> = ip_address
        .as_deref()
        .and_then(|ip| IpAddr::from_str(ip).ok());

    crate::api::admin::user::repo::create_audit_log(
        &st.db,
        actor_user_id,
        "CREATE_LESSON",
        Some("LESSON"),
        None,
        &serde_json::to_value(&req).unwrap_or(serde_json::Value::Null),
        ip_addr,
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
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> AppResult<(bool, LessonBulkCreateRes)> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    let ip_addr: Option<IpAddr> = ip_address
        .as_deref()
        .and_then(|ip| IpAddr::from_str(ip).ok());

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
        ip_addr,
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
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> AppResult<(bool, LessonBulkUpdateRes)> {
    check_admin_rbac(&st.db, actor_user_id).await?;

    if let Err(e) = req.validate() {
        return Err(AppError::BadRequest(e.to_string()));
    }

    let ip_addr: Option<IpAddr> = ip_address
        .as_deref()
        .and_then(|ip| IpAddr::from_str(ip).ok());

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
        ip_addr,
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
                || item.lesson_description.is_some();

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
