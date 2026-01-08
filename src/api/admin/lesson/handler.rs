use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use std::net::IpAddr;

use crate::api::admin::lesson::dto::{AdminLessonListRes, AdminLessonRes, LessonCreateReq, LessonListReq};
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
