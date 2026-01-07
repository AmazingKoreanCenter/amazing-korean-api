use std::net::IpAddr; // [필수] IpAddr 타입 사용
use std::str::FromStr; // [필수] String -> IpAddr 변환용 trait
use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::types::UserAuth;
use crate::AppState;

use super::dto::{AdminStudyListRes, StudyListReq};
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

    // [수정] String IP를 IpAddr 타입으로 변환 (변수 선언 추가)
    let ip_addr: Option<IpAddr> = ip_address
        .as_deref()
        .and_then(|ip| IpAddr::from_str(ip).ok());

    // 3. Audit Log
    let details = serde_json::json!({
        "q": req.q,
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
        req.q,
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