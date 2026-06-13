//! guide 콘텐츠 조회 HTTP 핸들러 (공개 읽기 — state='open' 단원만 노출)

use axum::extract::{Path, Query, State};
use axum::Json;

use crate::error::AppResult;
use crate::state::AppState;

use super::dto::{GuideDetailRes, GuideLangReq, GuideListRes};
use super::service::GuideService;

/// 공개 단원 목록
#[utoipa::path(
    get,
    path = "/guides",
    params(
        ("lang" = Option<String>, Query, description = "표시 언어 (예: zh-CN, id — 없으면 ko 우선)")
    ),
    responses(
        (status = 200, description = "공개(state=open) 단원 목록, guide_seq 순", body = GuideListRes)
    ),
    tag = "guide"
)]
pub async fn list_guides(
    State(state): State<AppState>,
    Query(req): Query<GuideLangReq>,
) -> AppResult<Json<GuideListRes>> {
    Ok(Json(GuideService::list(&state, req.lang).await?))
}

/// 단원 상세 (학습 페이지 전체: 블록 스트림 + 표 격자 + 문장)
#[utoipa::path(
    get,
    path = "/guides/{guide_idx}",
    params(
        ("guide_idx" = String, Path, description = "단원 안정키 (예: guidev2-05)"),
        ("lang" = Option<String>, Query, description = "표시 언어 (예: zh-CN, id — 없으면 ko 우선)")
    ),
    responses(
        (status = 200, description = "단원 상세", body = GuideDetailRes),
        (status = 404, description = "Not Found (미존재 또는 비공개)", body = crate::error::ErrorBody)
    ),
    tag = "guide"
)]
pub async fn get_guide(
    State(state): State<AppState>,
    Path(guide_idx): Path<String>,
    Query(req): Query<GuideLangReq>,
) -> AppResult<Json<GuideDetailRes>> {
    Ok(Json(
        GuideService::detail(&state, &guide_idx, req.lang).await?,
    ))
}
