use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use std::net::IpAddr;

use crate::api::admin::lesson::dto::{
    AdminLessonListRes, AdminLessonRes, LessonBulkCreateReq, LessonBulkCreateRes,
    LessonBulkUpdateReq, LessonBulkUpdateRes, LessonCreateReq, LessonListReq, LessonUpdateReq,
};
use crate::api::auth::extractor::AuthUser;
use crate::error::AppResult;
use crate::AppState;

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
    path = "/admin/lessons",
    tag = "admin_lesson",
    params(LessonListReq),
    responses(
        (status = 200, description = "List of lessons", body = AdminLessonListRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_list_lessons(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Query(params): Query<LessonListReq>,
) -> AppResult<Json<AdminLessonListRes>> {
    let ip_address = extract_client_ip(&headers).map(|ip| ip.to_string());
    let user_agent = extract_user_agent(&headers);

    let res = super::service::admin_list_lessons(
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
    post,
    path = "/admin/lessons",
    tag = "admin_lesson",
    request_body = LessonCreateReq,
    responses(
        (status = 201, description = "Lesson created", body = AdminLessonRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "Conflict"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_create_lesson(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Json(req): Json<LessonCreateReq>,
) -> AppResult<(StatusCode, Json<AdminLessonRes>)> {
    let ip_address = extract_client_ip(&headers).map(|ip| ip.to_string());
    let user_agent = extract_user_agent(&headers);

    let res = super::service::admin_create_lesson(
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
    path = "/admin/lessons/bulk",
    tag = "admin_lesson",
    request_body = LessonBulkCreateReq,
    responses(
        (status = 201, description = "All created", body = LessonBulkCreateRes),
        (status = 207, description = "Partial success", body = LessonBulkCreateRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "Conflict"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_bulk_create_lessons(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Json(req): Json<LessonBulkCreateReq>,
) -> AppResult<(StatusCode, Json<LessonBulkCreateRes>)> {
    let ip_address = extract_client_ip(&headers).map(|ip| ip.to_string());
    let user_agent = extract_user_agent(&headers);

    let (all_success, res) = super::service::admin_bulk_create_lessons(
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
    path = "/admin/lessons/bulk",
    tag = "admin_lesson",
    request_body = LessonBulkUpdateReq,
    responses(
        (status = 200, description = "All updated", body = LessonBulkUpdateRes),
        (status = 207, description = "Partial success", body = LessonBulkUpdateRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found"),
        (status = 409, description = "Conflict"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_bulk_update_lessons(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    headers: HeaderMap,
    Json(req): Json<LessonBulkUpdateReq>,
) -> AppResult<(StatusCode, Json<LessonBulkUpdateRes>)> {
    let ip_address = extract_client_ip(&headers).map(|ip| ip.to_string());
    let user_agent = extract_user_agent(&headers);

    let (all_success, res) = super::service::admin_bulk_update_lessons(
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
    path = "/admin/lessons/{lesson_id}",
    tag = "admin_lesson",
    request_body = LessonUpdateReq,
    params(
        ("lesson_id" = i32, Path, description = "Lesson ID")
    ),
    responses(
        (status = 200, description = "Lesson updated", body = AdminLessonRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found"),
        (status = 409, description = "Conflict"),
        (status = 422, description = "Unprocessable Entity"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_update_lesson(
    State(st): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(lesson_id): Path<i32>,
    headers: HeaderMap,
    Json(req): Json<LessonUpdateReq>,
) -> AppResult<Json<AdminLessonRes>> {
    let ip_address = extract_client_ip(&headers).map(|ip| ip.to_string());
    let user_agent = extract_user_agent(&headers);

    let res = super::service::admin_update_lesson(
        &st,
        auth_user.sub,
        lesson_id,
        req,
        ip_address,
        user_agent,
    )
    .await?;

    Ok(Json(res))
}
