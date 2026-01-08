use std::net::IpAddr;
use std::str::FromStr;

use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::types::UserAuth;
use crate::AppState;

use super::dto::{AdminLessonListRes, LessonListReq};
use super::repo;

async fn check_admin_rbac(pool: &sqlx::PgPool, actor_user_id: i64) -> AppResult<UserAuth> {
    let actor = crate::api::user::repo::find_user(pool, actor_user_id)
        .await?
        .ok_or(AppError::Unauthorized("Actor user not found".into()))?;

    match actor.user_auth {
        UserAuth::Hymn | UserAuth::Admin | UserAuth::Manager => Ok(actor.user_auth),
        _ => Err(AppError::Forbidden),
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
