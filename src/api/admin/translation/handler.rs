use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;

use crate::api::auth::extractor::AuthUser;
use crate::error::{AppError, AppResult};
use crate::state::AppState;

use super::dto::{
    AutoTranslateBulkReq, AutoTranslateBulkRes, AutoTranslateReq, AutoTranslateRes,
    ContentRecordsReq, ContentRecordsRes, SourceFieldsReq, SourceFieldsRes,
    TranslationBulkCreateReq, TranslationBulkCreateRes, TranslationCreateReq,
    TranslationListReq, TranslationListRes, TranslationRes, TranslationSearchReq,
    TranslationSearchRes, TranslationStatsRes, TranslationStatusReq, TranslationUpdateReq,
};
use super::service::TranslationService;

#[utoipa::path(
    get,
    path = "/admin/translations",
    tag = "admin_translation",
    params(TranslationListReq),
    responses(
        (status = 200, description = "Translation list", body = TranslationListRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_list_translations(
    State(st): State<AppState>,
    AuthUser(_auth): AuthUser,
    Query(req): Query<TranslationListReq>,
) -> AppResult<Json<TranslationListRes>> {
    let res = TranslationService::list_translations(&st.db, req).await?;
    Ok(Json(res))
}

#[utoipa::path(
    post,
    path = "/admin/translations",
    tag = "admin_translation",
    request_body(content = TranslationCreateReq, content_type = "application/json"),
    responses(
        (status = 201, description = "Translation created", body = TranslationRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_create_translation(
    State(st): State<AppState>,
    AuthUser(_auth): AuthUser,
    Json(req): Json<TranslationCreateReq>,
) -> AppResult<(StatusCode, Json<TranslationRes>)> {
    let res = TranslationService::create_translation(&st.db, req).await?;
    Ok((StatusCode::CREATED, Json(res)))
}

#[utoipa::path(
    post,
    path = "/admin/translations/bulk",
    tag = "admin_translation",
    request_body(content = TranslationBulkCreateReq, content_type = "application/json"),
    responses(
        (status = 200, description = "Bulk creation result", body = TranslationBulkCreateRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_bulk_create_translations(
    State(st): State<AppState>,
    AuthUser(_auth): AuthUser,
    Json(req): Json<TranslationBulkCreateReq>,
) -> AppResult<Json<TranslationBulkCreateRes>> {
    let res = TranslationService::bulk_create_translations(&st.db, req).await?;
    Ok(Json(res))
}

#[utoipa::path(
    get,
    path = "/admin/translations/{id}",
    tag = "admin_translation",
    params(("id" = i64, Path, description = "Translation ID")),
    responses(
        (status = 200, description = "Translation detail", body = TranslationRes),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_get_translation(
    State(st): State<AppState>,
    AuthUser(_auth): AuthUser,
    Path(id): Path<i64>,
) -> AppResult<Json<TranslationRes>> {
    let res = TranslationService::get_translation(&st.db, id).await?;
    Ok(Json(res))
}

#[utoipa::path(
    patch,
    path = "/admin/translations/{id}",
    tag = "admin_translation",
    params(("id" = i64, Path, description = "Translation ID")),
    request_body(content = TranslationUpdateReq, content_type = "application/json"),
    responses(
        (status = 200, description = "Translation updated", body = TranslationRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_update_translation(
    State(st): State<AppState>,
    AuthUser(_auth): AuthUser,
    Path(id): Path<i64>,
    Json(req): Json<TranslationUpdateReq>,
) -> AppResult<Json<TranslationRes>> {
    let res = TranslationService::update_translation(&st.db, id, req).await?;
    Ok(Json(res))
}

#[utoipa::path(
    patch,
    path = "/admin/translations/{id}/status",
    tag = "admin_translation",
    params(("id" = i64, Path, description = "Translation ID")),
    request_body(content = TranslationStatusReq, content_type = "application/json"),
    responses(
        (status = 200, description = "Translation status updated", body = TranslationRes),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_update_translation_status(
    State(st): State<AppState>,
    AuthUser(_auth): AuthUser,
    Path(id): Path<i64>,
    Json(req): Json<TranslationStatusReq>,
) -> AppResult<Json<TranslationRes>> {
    let res = TranslationService::update_translation_status(&st.db, id, req).await?;
    Ok(Json(res))
}

#[utoipa::path(
    delete,
    path = "/admin/translations/{id}",
    tag = "admin_translation",
    params(("id" = i64, Path, description = "Translation ID")),
    responses(
        (status = 204, description = "Translation deleted"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_delete_translation(
    State(st): State<AppState>,
    AuthUser(_auth): AuthUser,
    Path(id): Path<i64>,
) -> AppResult<StatusCode> {
    TranslationService::delete_translation(&st.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/admin/translations/auto",
    tag = "admin_translation",
    request_body(content = AutoTranslateReq, content_type = "application/json"),
    responses(
        (status = 200, description = "Auto translation result", body = AutoTranslateRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 502, description = "Translation provider not configured"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_auto_translate(
    State(st): State<AppState>,
    AuthUser(_auth): AuthUser,
    Json(req): Json<AutoTranslateReq>,
) -> AppResult<Json<AutoTranslateRes>> {
    let translator = st.translator.as_ref().ok_or_else(|| {
        AppError::External("Translation provider not configured. Set TRANSLATE_PROVIDER=google.".to_string())
    })?;
    let res = TranslationService::auto_translate(&st.db, translator.as_ref(), req).await?;
    Ok(Json(res))
}

// =============================================================================
// 콘텐츠 목록 조회 (Step 4)
// =============================================================================

#[utoipa::path(
    get,
    path = "/admin/translations/content-records",
    tag = "admin_translation",
    params(ContentRecordsReq),
    responses(
        (status = 200, description = "Content records for dropdown", body = ContentRecordsRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_list_content_records(
    State(st): State<AppState>,
    AuthUser(_auth): AuthUser,
    Query(req): Query<ContentRecordsReq>,
) -> AppResult<Json<ContentRecordsRes>> {
    let res = TranslationService::list_content_records(&st.db, req).await?;
    Ok(Json(res))
}

// =============================================================================
// 원본 텍스트 조회 (Step 5)
// =============================================================================

#[utoipa::path(
    get,
    path = "/admin/translations/source-fields",
    tag = "admin_translation",
    params(SourceFieldsReq),
    responses(
        (status = 200, description = "Source fields with Korean text", body = SourceFieldsRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_get_source_fields(
    State(st): State<AppState>,
    AuthUser(_auth): AuthUser,
    Query(req): Query<SourceFieldsReq>,
) -> AppResult<Json<SourceFieldsRes>> {
    let res = TranslationService::get_source_fields(&st.db, req).await?;
    Ok(Json(res))
}

// =============================================================================
// 벌크 자동 번역 (Step 6)
// =============================================================================

#[utoipa::path(
    post,
    path = "/admin/translations/auto-bulk",
    tag = "admin_translation",
    request_body(content = AutoTranslateBulkReq, content_type = "application/json"),
    responses(
        (status = 200, description = "Bulk auto translation result", body = AutoTranslateBulkRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 502, description = "Translation provider not configured"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_auto_translate_bulk(
    State(st): State<AppState>,
    AuthUser(_auth): AuthUser,
    Json(req): Json<AutoTranslateBulkReq>,
) -> AppResult<Json<AutoTranslateBulkRes>> {
    let translator = st.translator.as_ref().ok_or_else(|| {
        AppError::External("Translation provider not configured. Set TRANSLATE_PROVIDER=google.".to_string())
    })?;
    let res = TranslationService::auto_translate_bulk(&st.db, translator.as_ref(), req).await?;
    Ok(Json(res))
}

// =============================================================================
// 번역 검색 (Step 11 — 재사용)
// =============================================================================

#[utoipa::path(
    get,
    path = "/admin/translations/search",
    tag = "admin_translation",
    params(TranslationSearchReq),
    responses(
        (status = 200, description = "Translation search results", body = TranslationSearchRes),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_search_translations(
    State(st): State<AppState>,
    AuthUser(_auth): AuthUser,
    Query(req): Query<TranslationSearchReq>,
) -> AppResult<Json<TranslationSearchRes>> {
    let res = TranslationService::search_translations(&st.db, req).await?;
    Ok(Json(res))
}

// =============================================================================
// 번역 통계 (Step 5A)
// =============================================================================

#[utoipa::path(
    get,
    path = "/admin/translations/stats",
    tag = "admin_translation",
    responses(
        (status = 200, description = "Translation statistics", body = TranslationStatsRes),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
    security(("bearerAuth" = []))
)]
pub async fn admin_get_translation_stats(
    State(st): State<AppState>,
    AuthUser(_auth): AuthUser,
) -> AppResult<Json<TranslationStatsRes>> {
    let res = TranslationService::get_translation_stats(&st.db).await?;
    Ok(Json(res))
}
