use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use std::net::IpAddr;

use crate::api::admin::study::dto::{
    AdminStudyDetailRes, AdminStudyListRes, AdminStudyRes, StudyBulkCreateReq, StudyBulkCreateRes,
    StudyBulkUpdateReq, StudyBulkUpdateRes, StudyCreateReq, StudyListReq, StudyTaskBulkCreateReq,
    StudyTaskBulkCreateRes, StudyTaskBulkUpdateReq, StudyTaskBulkUpdateRes, StudyTaskCreateReq,
    StudyTaskListReq, StudyTaskUpdateReq, StudyUpdateReq, AdminStudyTaskListRes,
    AdminStudyTaskDetailRes, TaskExplainBulkCreateReq, TaskExplainBulkCreateRes,
    TaskExplainBulkUpdateReq, TaskExplainBulkUpdateRes, TaskExplainCreateReq, TaskExplainListReq,
    TaskExplainUpdateReq, TaskStatusBulkUpdateReq, TaskStatusBulkUpdateRes, TaskStatusListReq,
    TaskStatusUpdateReq, AdminTaskStatusListRes, AdminTaskStatusRes, AdminTaskExplainListRes,
    AdminTaskExplainRes,
};
use crate::api::auth::extractor::AuthUser;
use crate::error::{AppError, AppResult};
use crate::AppState;

// IP 추출 헬퍼 (기존 코드 유지)
fn extract_client_ip(headers: &HeaderMap) -> Option<IpAddr> {
    let forwarded = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .map(|v| v.trim().to_string());

    let direct = headers
        .get("x-real-ip")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string());

    if let Some(ip_str) = forwarded.or(direct) {
        if let Ok(ip) = ip_str.parse::<IpAddr>() {
            return Some(ip);
        }
    }
    None
}

fn extract_user_agent(headers: &HeaderMap) -> Option<String> {
    headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string())
}

#[utoipa::path(
    get,
    path = "/admin/studies",
    tag = "admin_study",
    params(StudyListReq),
    responses(
        (status = 200, description = "List of studies", body = AdminStudyListRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_list_studies(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Query(params): Query<StudyListReq>,
) -> AppResult<Json<AdminStudyListRes>> {
    // IpAddr -> Option<String> 변환
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let res = super::service::admin_list_studies(
        &st,
        auth_user.sub,
        params,
        ip_address, // String으로 변환된 IP 전달
        user_agent,
    )
    .await?;

    Ok(Json(res))
}

#[utoipa::path(
    get,
    path = "/admin/studies/{study_id}",
    tag = "admin_study",
    params(
        ("study_id" = i64, Path, description = "Study ID")
    ),
    responses(
        (status = 200, description = "Study details with tasks", body = AdminStudyDetailRes),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Study not found"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_get_study(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Path(study_id): Path<i64>,
) -> AppResult<Json<AdminStudyDetailRes>> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let res = super::service::admin_get_study(
        &st,
        auth_user.sub,
        study_id,
        ip_address,
        user_agent,
    )
    .await?;

    Ok(Json(res))
}

#[utoipa::path(
    post,
    path = "/admin/studies",
    tag = "admin_study",
    request_body = StudyCreateReq,
    responses(
        (status = 201, description = "Study created", body = AdminStudyRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "Conflict"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_create_study(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Json(req): Json<StudyCreateReq>,
) -> Result<(StatusCode, Json<AdminStudyRes>), AppError> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let res = super::service::admin_create_study(
        &st,
        auth_user.sub,
        req,
        ip_address,
        user_agent,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(res)))
}

#[utoipa::path(
    post,
    path = "/admin/studies/bulk",
    tag = "admin_study",
    request_body = StudyBulkCreateReq,
    responses(
        (status = 201, description = "All created", body = StudyBulkCreateRes),
        (status = 207, description = "Partial success", body = StudyBulkCreateRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "Conflict"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_bulk_create_studies(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Json(req): Json<StudyBulkCreateReq>,
) -> AppResult<(StatusCode, Json<StudyBulkCreateRes>)> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let (all_success, res) = super::service::admin_bulk_create_studies(
        &st,
        auth_user.sub,
        req,
        ip_address,
        user_agent,
    )
    .await?;

    let status = if all_success {
        StatusCode::CREATED
    } else {
        StatusCode::MULTI_STATUS
    };

    Ok((status, Json(res)))
}

#[utoipa::path(
    patch,
    path = "/admin/studies/{study_id}",
    tag = "admin_study",
    request_body = StudyUpdateReq,
    params(
        ("study_id" = i64, Path, description = "Study ID")
    ),
    responses(
        (status = 200, description = "Study updated", body = AdminStudyRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found"),
        (status = 409, description = "Conflict"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_update_study(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(study_id): Path<i64>,
    headers: HeaderMap,
    Json(req): Json<StudyUpdateReq>,
) -> AppResult<Json<AdminStudyRes>> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let res = super::service::admin_update_study(
        &st,
        auth_user.sub,
        study_id,
        req,
        ip_address,
        user_agent,
    )
    .await?;

    Ok(Json(res))
}

#[utoipa::path(
    patch,
    path = "/admin/studies/bulk",
    tag = "admin_study",
    request_body = StudyBulkUpdateReq,
    responses(
        (status = 200, description = "All updated", body = StudyBulkUpdateRes),
        (status = 207, description = "Partial success", body = StudyBulkUpdateRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "Conflict"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_bulk_update_studies(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Json(req): Json<StudyBulkUpdateReq>,
) -> AppResult<(StatusCode, Json<StudyBulkUpdateRes>)> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let (all_success, res) = super::service::admin_bulk_update_studies(
        &st,
        auth_user.sub,
        req,
        ip_address,
        user_agent,
    )
    .await?;

    let status = if all_success {
        StatusCode::OK
    } else {
        StatusCode::MULTI_STATUS
    };

    Ok((status, Json(res)))
}

#[utoipa::path(
    get,
    path = "/admin/studies/tasks",
    tag = "admin_study_task",
    params(
        ("study_id" = i32, Query, description = "Study ID"),
        ("page" = u64, Query, description = "Page number, defaults to 1", example = 1),
        ("size" = u64, Query, description = "Page size, defaults to 20 (max 100)", example = 20)
    ),
    responses(
        (status = 200, description = "List of study tasks", body = AdminStudyTaskListRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_list_study_tasks(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Query(params): Query<StudyTaskListReq>,
) -> AppResult<Json<AdminStudyTaskListRes>> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let res = super::service::admin_list_study_tasks(
        &st,
        auth_user.sub,
        params,
        ip_address,
        user_agent,
    )
    .await?;

    Ok(Json(res))
}

#[utoipa::path(
    get,
    path = "/admin/studies/tasks/{task_id}",
    tag = "admin_study_task",
    params(
        ("task_id" = i64, Path, description = "Study Task ID")
    ),
    responses(
        (status = 200, description = "Study task detail", body = AdminStudyTaskDetailRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_get_study_task(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Path(task_id): Path<i64>,
) -> AppResult<Json<AdminStudyTaskDetailRes>> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let res = super::service::admin_get_study_task(
        &st,
        auth_user.sub,
        task_id,
        ip_address,
        user_agent,
    )
    .await?;

    Ok(Json(res))
}

#[utoipa::path(
    get,
    path = "/admin/studies/tasks/explain",
    tag = "admin_study_task_explain",
    params(
        ("task_id" = i32, Query, description = "Study Task ID"),
        ("page" = u64, Query, description = "Page number, defaults to 1", example = 1),
        ("size" = u64, Query, description = "Page size, defaults to 20 (max 100)", example = 20)
    ),
    responses(
        (status = 200, description = "List of task explains", body = AdminTaskExplainListRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_list_task_explains(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Query(params): Query<TaskExplainListReq>,
) -> AppResult<Json<AdminTaskExplainListRes>> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let res = super::service::admin_list_task_explains(
        &st,
        auth_user.sub,
        params,
        ip_address,
        user_agent,
    )
    .await?;

    Ok(Json(res))
}

#[utoipa::path(
    get,
    path = "/admin/studies/tasks/status",
    tag = "admin_study_task_status",
    params(
        ("task_id" = i32, Query, description = "Study Task ID"),
        ("user_id" = i64, Query, description = "User ID"),
        ("page" = u64, Query, description = "Page number, defaults to 1", example = 1),
        ("size" = u64, Query, description = "Page size, defaults to 20 (max 100)", example = 20)
    ),
    responses(
        (status = 200, description = "List of task status", body = AdminTaskStatusListRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_list_task_status(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Query(params): Query<TaskStatusListReq>,
) -> AppResult<Json<AdminTaskStatusListRes>> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let res = super::service::admin_list_task_status(
        &st,
        auth_user.sub,
        params,
        ip_address,
        user_agent,
    )
    .await?;

    Ok(Json(res))
}

#[utoipa::path(
    patch,
    path = "/admin/studies/tasks/{task_id}/status",
    tag = "admin_study_task_status",
    request_body = TaskStatusUpdateReq,
    params(
        ("task_id" = i64, Path, description = "Study Task ID")
    ),
    responses(
        (status = 200, description = "Task status updated", body = AdminTaskStatusRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_update_task_status(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(task_id): Path<i64>,
    headers: HeaderMap,
    Json(req): Json<TaskStatusUpdateReq>,
) -> AppResult<Json<AdminTaskStatusRes>> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let res = super::service::admin_update_task_status(
        &st,
        auth_user.sub,
        task_id,
        req,
        ip_address,
        user_agent,
    )
    .await?;

    Ok(Json(res))
}

#[utoipa::path(
    patch,
    path = "/admin/studies/tasks/bulk/status",
    tag = "admin_study_task_status",
    request_body = TaskStatusBulkUpdateReq,
    responses(
        (status = 200, description = "All updated", body = TaskStatusBulkUpdateRes),
        (status = 207, description = "Partial success", body = TaskStatusBulkUpdateRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found"),
        (status = 409, description = "Conflict"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_bulk_update_task_status(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Json(req): Json<TaskStatusBulkUpdateReq>,
) -> AppResult<(StatusCode, Json<TaskStatusBulkUpdateRes>)> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let (all_success, res) = super::service::admin_bulk_update_task_status(
        &st,
        auth_user.sub,
        req,
        ip_address,
        user_agent,
    )
    .await?;

    let status = if all_success {
        StatusCode::OK
    } else {
        StatusCode::MULTI_STATUS
    };

    Ok((status, Json(res)))
}

#[utoipa::path(
    post,
    path = "/admin/studies/tasks/{task_id}/explain",
    tag = "admin_study_task_explain",
    request_body = TaskExplainCreateReq,
    params(
        ("task_id" = i64, Path, description = "Study Task ID")
    ),
    responses(
        (status = 201, description = "Task explain created", body = AdminTaskExplainRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found"),
        (status = 409, description = "Conflict"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_create_task_explain(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(task_id): Path<i64>,
    headers: HeaderMap,
    Json(req): Json<TaskExplainCreateReq>,
) -> Result<(StatusCode, Json<AdminTaskExplainRes>), AppError> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let res = super::service::admin_create_task_explain(
        &st,
        auth_user.sub,
        task_id,
        req,
        ip_address,
        user_agent,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(res)))
}

#[utoipa::path(
    patch,
    path = "/admin/studies/tasks/{task_id}/explain",
    tag = "admin_study_task_explain",
    request_body = TaskExplainUpdateReq,
    params(
        ("task_id" = i64, Path, description = "Study Task ID")
    ),
    responses(
        (status = 200, description = "Task explain updated", body = AdminTaskExplainRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_update_task_explain(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(task_id): Path<i64>,
    headers: HeaderMap,
    Json(req): Json<TaskExplainUpdateReq>,
) -> AppResult<Json<AdminTaskExplainRes>> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let res = super::service::admin_update_task_explain(
        &st,
        auth_user.sub,
        task_id,
        req,
        ip_address,
        user_agent,
    )
    .await?;

    Ok(Json(res))
}

#[utoipa::path(
    post,
    path = "/admin/studies/tasks/bulk/explain",
    tag = "admin_study_task_explain",
    request_body = TaskExplainBulkCreateReq,
    responses(
        (status = 201, description = "All created", body = TaskExplainBulkCreateRes),
        (status = 207, description = "Partial success", body = TaskExplainBulkCreateRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found"),
        (status = 409, description = "Conflict"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_bulk_create_task_explains(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Json(req): Json<TaskExplainBulkCreateReq>,
) -> AppResult<(StatusCode, Json<TaskExplainBulkCreateRes>)> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let (all_success, res) = super::service::admin_bulk_create_task_explains(
        &st,
        auth_user.sub,
        req,
        ip_address,
        user_agent,
    )
    .await?;

    let status = if all_success {
        StatusCode::CREATED
    } else {
        StatusCode::MULTI_STATUS
    };

    Ok((status, Json(res)))
}

#[utoipa::path(
    patch,
    path = "/admin/studies/tasks/bulk/explain",
    tag = "admin_study_task_explain",
    request_body = TaskExplainBulkUpdateReq,
    responses(
        (status = 200, description = "All updated", body = TaskExplainBulkUpdateRes),
        (status = 207, description = "Partial success", body = TaskExplainBulkUpdateRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found"),
        (status = 409, description = "Conflict"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_bulk_update_task_explains(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Json(req): Json<TaskExplainBulkUpdateReq>,
) -> AppResult<(StatusCode, Json<TaskExplainBulkUpdateRes>)> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let (all_success, res) = super::service::admin_bulk_update_task_explains(
        &st,
        auth_user.sub,
        req,
        ip_address,
        user_agent,
    )
    .await?;

    let status = if all_success {
        StatusCode::OK
    } else {
        StatusCode::MULTI_STATUS
    };

    Ok((status, Json(res)))
}

#[utoipa::path(
    post,
    path = "/admin/studies/tasks",
    tag = "admin_study_task",
    request_body = StudyTaskCreateReq,
    responses(
        (status = 201, description = "Study task created", body = AdminStudyTaskDetailRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "Conflict"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_create_study_task(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Json(req): Json<StudyTaskCreateReq>,
) -> Result<(StatusCode, Json<AdminStudyTaskDetailRes>), AppError> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let res = super::service::admin_create_study_task(
        &st,
        auth_user.sub,
        req,
        ip_address,
        user_agent,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(res)))
}

#[utoipa::path(
    post,
    path = "/admin/studies/tasks/bulk",
    tag = "admin_study_task",
    request_body = StudyTaskBulkCreateReq,
    responses(
        (status = 201, description = "All created", body = StudyTaskBulkCreateRes),
        (status = 207, description = "Partial success", body = StudyTaskBulkCreateRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "Conflict"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_bulk_create_study_tasks(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Json(req): Json<StudyTaskBulkCreateReq>,
) -> AppResult<(StatusCode, Json<StudyTaskBulkCreateRes>)> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let (all_success, res) = super::service::admin_bulk_create_study_tasks(
        &st,
        auth_user.sub,
        req,
        ip_address,
        user_agent,
    )
    .await?;

    let status = if all_success {
        StatusCode::CREATED
    } else {
        StatusCode::MULTI_STATUS
    };

    Ok((status, Json(res)))
}

#[utoipa::path(
    patch,
    path = "/admin/studies/tasks/bulk",
    tag = "admin_study_task",
    request_body = StudyTaskBulkUpdateReq,
    responses(
        (status = 200, description = "All updated", body = StudyTaskBulkUpdateRes),
        (status = 207, description = "Partial success", body = StudyTaskBulkUpdateRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "Conflict"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_bulk_update_study_tasks(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Json(req): Json<StudyTaskBulkUpdateReq>,
) -> AppResult<(StatusCode, Json<StudyTaskBulkUpdateRes>)> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let (all_success, res) = super::service::admin_bulk_update_study_tasks(
        &st,
        auth_user.sub,
        req,
        ip_address,
        user_agent,
    )
    .await?;

    let status = if all_success {
        StatusCode::OK
    } else {
        StatusCode::MULTI_STATUS
    };

    Ok((status, Json(res)))
}

#[utoipa::path(
    patch,
    path = "/admin/studies/tasks/{task_id}",
    tag = "admin_study_task",
    request_body = StudyTaskUpdateReq,
    params(
        ("task_id" = i64, Path, description = "Study Task ID")
    ),
    responses(
        (status = 200, description = "Study task updated", body = AdminStudyTaskDetailRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found"),
        (status = 409, description = "Conflict"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_update_study_task(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(task_id): Path<i64>,
    headers: HeaderMap,
    Json(req): Json<StudyTaskUpdateReq>,
) -> AppResult<Json<AdminStudyTaskDetailRes>> {
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);

    let res = super::service::admin_update_study_task(
        &st,
        auth_user.sub,
        task_id,
        req,
        ip_address,
        user_agent,
    )
    .await?;

    Ok(Json(res))
}
