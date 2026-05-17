//! 해설 콘텐츠 조회 HTTP 핸들러 (공개 읽기 — 접근 제어 컬럼 없음)

use axum::extract::{Path, Query, State};
use axum::Json;

use crate::error::AppResult;
use crate::state::AppState;

use super::dto::{
    ExplanationDetailReq, ExplanationListReq, ExplanationListRes, ExplanationUnitRes,
};
use super::service::ExplanationService;

/// 연결키로 해설 조회 (study_idx 또는 study_task_idx)
#[utoipa::path(
    get,
    path = "/explanations",
    params(
        ("study_idx" = Option<String>, Query, description = "study.study_idx (pattern_guide)"),
        ("study_task_idx" = Option<String>, Query, description = "study_task.study_task_idx = amk500-sent-NNN (sentence_explain)"),
        ("lang" = Option<String>, Query, description = "번역 언어 (없으면 ko 원본 / structured 는 en 기준)")
    ),
    responses(
        (status = 200, description = "연결된 해설 단위 목록", body = ExplanationListRes),
        (status = 400, description = "Bad Request", body = crate::error::ErrorBody)
    ),
    tag = "explanation"
)]
pub async fn list_explanations(
    State(state): State<AppState>,
    Query(req): Query<ExplanationListReq>,
) -> AppResult<Json<ExplanationListRes>> {
    let res = ExplanationService::list_by_link(
        &state,
        req.study_idx.as_deref(),
        req.study_task_idx.as_deref(),
        req.lang,
    )
    .await?;
    Ok(Json(res))
}

/// 해설 단위 상세 (unit_idx)
#[utoipa::path(
    get,
    path = "/explanations/{unit_idx}",
    params(
        ("unit_idx" = String, Path, description = "books unit_id (예: guide67:pr_105_114, sent:300)"),
        ("lang" = Option<String>, Query, description = "번역 언어 (없으면 ko 원본 / structured 는 en 기준)")
    ),
    responses(
        (status = 200, description = "해설 단위 + 블록", body = ExplanationUnitRes),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody)
    ),
    tag = "explanation"
)]
pub async fn get_explanation(
    State(state): State<AppState>,
    Path(unit_idx): Path<String>,
    Query(req): Query<ExplanationDetailReq>,
) -> AppResult<Json<ExplanationUnitRes>> {
    let res = ExplanationService::get_unit(&state, &unit_idx, req.lang).await?;
    Ok(Json(res))
}
