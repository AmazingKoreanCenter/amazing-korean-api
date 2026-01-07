use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use std::net::IpAddr;

use crate::api::admin::study::dto::{
    AdminStudyListRes, AdminStudyRes, StudyBulkCreateReq, StudyBulkCreateRes, StudyBulkUpdateReq,
    StudyBulkUpdateRes, StudyCreateReq, StudyListReq, StudyTaskListReq, StudyUpdateReq,
    AdminStudyTaskListRes,
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
    let ip_address = extract_client_ip(&headers).map(|ip| ip.to_string());
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
    let ip_address = extract_client_ip(&headers).map(|ip| ip.to_string());
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
    let ip_address = extract_client_ip(&headers).map(|ip| ip.to_string());
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
    let ip_address = extract_client_ip(&headers).map(|ip| ip.to_string());
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
    let ip_address = extract_client_ip(&headers).map(|ip| ip.to_string());
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
    tag = "admin_study",
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
    let ip_address = extract_client_ip(&headers).map(|ip| ip.to_string());
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
