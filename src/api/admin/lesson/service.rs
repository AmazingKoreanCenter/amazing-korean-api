use std::net::IpAddr;
use std::str::FromStr;

use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::types::UserAuth;
use crate::AppState;

use super::dto::{AdminLessonListRes, AdminLessonRes, LessonCreateReq, LessonListReq};
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
