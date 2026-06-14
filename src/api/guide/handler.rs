//! guide 콘텐츠 조회 HTTP 핸들러 (공개 읽기 — state='open' 단원만 노출)

use axum::extract::{Path, Query, State};
use axum::Json;

use crate::api::auth::extractor::AuthUser;
use crate::error::AppResult;
use crate::extract::AppJson;
use crate::state::AppState;

use super::dto::{
    GuideDetailRes, GuideLangReq, GuideListRes, GuideLogReq, GuideProgressRes,
    GuideSentenceStatusRes,
};
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

/// 문장 학습 로그 기록 (시도/정오) — 인증 필요
#[utoipa::path(
    post,
    path = "/guides/{guide_idx}/sentences/{sentence_no}/log",
    params(
        ("guide_idx" = String, Path, description = "단원 안정키 (예: guidev2-05)"),
        ("sentence_no" = i32, Path, description = "전역 문장 번호 (1~500)")
    ),
    request_body = GuideLogReq,
    responses(
        (status = 200, description = "기록 직후 갱신된 문장 상태", body = GuideSentenceStatusRes),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 404, description = "미존재 또는 비공개 단원·문장", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = [])),
    tag = "guide"
)]
pub async fn log_sentence(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((guide_idx, sentence_no)): Path<(String, i32)>,
    AppJson(req): AppJson<GuideLogReq>,
) -> AppResult<Json<GuideSentenceStatusRes>> {
    Ok(Json(
        GuideService::log_sentence(&state, auth_user, &guide_idx, sentence_no, req).await?,
    ))
}

/// 내 단원 진행 상황 (문장별 시도/해결) — 인증 필요
#[utoipa::path(
    get,
    path = "/guides/{guide_idx}/progress",
    params(("guide_idx" = String, Path, description = "단원 안정키 (예: guidev2-05)")),
    responses(
        (status = 200, description = "문장별 진행 — 기록 있는 문장만(희소)", body = GuideProgressRes),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 404, description = "미존재 또는 비공개 단원", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = [])),
    tag = "guide"
)]
pub async fn get_progress(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(guide_idx): Path<String>,
) -> AppResult<Json<GuideProgressRes>> {
    Ok(Json(
        GuideService::progress(&state, auth_user, &guide_idx).await?,
    ))
}
