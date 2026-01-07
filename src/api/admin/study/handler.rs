use axum::{
    extract::{Query, State},
    http::{HeaderMap},
    Json,
};
use std::net::IpAddr;

use crate::api::admin::study::dto::{AdminStudyListRes, StudyListReq};
use crate::api::auth::extractor::AuthUser;
use crate::error::AppResult;
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