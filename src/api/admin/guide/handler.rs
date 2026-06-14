//! guide admin 편집 HTTP 핸들러 (/admin 하위 — role_guard + ip_guard 적용됨)

use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::Json;

use crate::api::admin::header_utils::{extract_client_ip, extract_user_agent};
use crate::api::auth::extractor::AuthUser;
use crate::error::AppResult;
use crate::extract::AppJson;
use crate::state::AppState;

use super::dto::{
    AdminGuideDetailRes, AdminGuideListRes, AdminOkRes, DiffExportRes, GuideBlockUpdateReq,
    GuideMetaUpdateReq, GuideSentenceUpdateReq, StaleDashboardRes, StaleReq,
};
use super::service;

#[utoipa::path(get, path = "/admin/guides", tag = "admin_guide",
    responses((status = 200, body = AdminGuideListRes), (status = 403)),
    security(("bearerAuth" = [])))]
pub async fn admin_list_guides(
    State(st): State<AppState>,
    AuthUser(auth): AuthUser,
) -> AppResult<Json<AdminGuideListRes>> {
    Ok(Json(service::list(&st, auth.sub).await?))
}

#[utoipa::path(get, path = "/admin/guides/stale", tag = "admin_guide",
    params(("lang" = Option<String>, Query, description = "대상 언어(없으면 전 언어)")),
    responses((status = 200, body = StaleDashboardRes), (status = 403)),
    security(("bearerAuth" = [])))]
pub async fn admin_guide_stale(
    State(st): State<AppState>,
    AuthUser(auth): AuthUser,
    Query(req): Query<StaleReq>,
) -> AppResult<Json<StaleDashboardRes>> {
    Ok(Json(
        service::stale_dashboard(&st, auth.sub, req.lang).await?,
    ))
}

#[utoipa::path(get, path = "/admin/guides/diff-export", tag = "admin_guide",
    params(("lang" = String, Query, description = "재번역 대상 언어 (필수)")),
    responses((status = 200, body = DiffExportRes), (status = 400), (status = 403)),
    security(("bearerAuth" = [])))]
pub async fn admin_guide_diff_export(
    State(st): State<AppState>,
    AuthUser(auth): AuthUser,
    Query(req): Query<StaleReq>,
) -> AppResult<Json<DiffExportRes>> {
    let lang = req
        .lang
        .ok_or_else(|| crate::error::AppError::BadRequest("lang 필수".into()))?;
    Ok(Json(service::diff_export(&st, auth.sub, lang).await?))
}

#[utoipa::path(get, path = "/admin/guides/{guide_idx}", tag = "admin_guide",
    params(("guide_idx" = String, Path, description = "단원 안정키")),
    responses((status = 200, body = AdminGuideDetailRes), (status = 404), (status = 403)),
    security(("bearerAuth" = [])))]
pub async fn admin_get_guide(
    State(st): State<AppState>,
    AuthUser(auth): AuthUser,
    Path(guide_idx): Path<String>,
) -> AppResult<Json<AdminGuideDetailRes>> {
    Ok(Json(service::detail(&st, auth.sub, &guide_idx).await?))
}

#[utoipa::path(patch, path = "/admin/guides/{guide_idx}", tag = "admin_guide",
    params(("guide_idx" = String, Path)),
    request_body = GuideMetaUpdateReq,
    responses((status = 200, body = AdminOkRes), (status = 400), (status = 404), (status = 403)),
    security(("bearerAuth" = [])))]
pub async fn admin_update_guide_meta(
    State(st): State<AppState>,
    AuthUser(auth): AuthUser,
    Path(guide_idx): Path<String>,
    headers: HeaderMap,
    AppJson(req): AppJson<GuideMetaUpdateReq>,
) -> AppResult<Json<AdminOkRes>> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);
    Ok(Json(
        service::update_meta(&st, auth.sub, &guide_idx, req, ip, ua).await?,
    ))
}

#[utoipa::path(patch, path = "/admin/guides/blocks/{block_id}", tag = "admin_guide",
    params(("block_id" = i64, Path)),
    request_body = GuideBlockUpdateReq,
    responses((status = 200, body = AdminOkRes), (status = 400), (status = 404), (status = 403)),
    security(("bearerAuth" = [])))]
pub async fn admin_update_guide_block(
    State(st): State<AppState>,
    AuthUser(auth): AuthUser,
    Path(block_id): Path<i64>,
    headers: HeaderMap,
    AppJson(req): AppJson<GuideBlockUpdateReq>,
) -> AppResult<Json<AdminOkRes>> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);
    Ok(Json(
        service::update_block(&st, auth.sub, block_id, req, ip, ua).await?,
    ))
}

#[utoipa::path(patch, path = "/admin/guides/sentences/{sentence_no}", tag = "admin_guide",
    params(("sentence_no" = i32, Path)),
    request_body = GuideSentenceUpdateReq,
    responses((status = 200, body = AdminOkRes), (status = 400), (status = 404), (status = 403)),
    security(("bearerAuth" = [])))]
pub async fn admin_update_guide_sentence(
    State(st): State<AppState>,
    AuthUser(auth): AuthUser,
    Path(sentence_no): Path<i32>,
    headers: HeaderMap,
    AppJson(req): AppJson<GuideSentenceUpdateReq>,
) -> AppResult<Json<AdminOkRes>> {
    let ip = extract_client_ip(&headers);
    let ua = extract_user_agent(&headers);
    Ok(Json(
        service::update_sentence(&st, auth.sub, sentence_no, req, ip, ua).await?,
    ))
}
