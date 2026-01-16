use axum::extract::{Path, Query, State};
use axum::Json;

use crate::api::auth::extractor::AuthUser;
use crate::error::AppResult;
use crate::state::AppState;

use super::dto::{
    StudyListReq, StudyListRes, StudyTaskDetailRes, SubmitAnswerReq, SubmitAnswerRes,
    TaskExplainRes, TaskStatusRes,
};
use super::service::StudyService;

// =========================================================================
// Study Handlers
// =========================================================================

/// 학습 목록 조회 (검색, 필터, 페이징)
#[utoipa::path(
    get,
    path = "/studies",
    params(
        ("page" = Option<u64>, Query, description = "Page number (default 1)"),
        ("per_page" = Option<u64>, Query, description = "Items per page (default 20, max 100)"),
        ("program" = Option<String>, Query, description = "Program filter (basic_900, topik_read, etc)"),
        ("sort" = Option<String>, Query, description = "Sort order (created_at_desc, etc)")
    ),
    responses(
        (status = 200, description = "List of studies", body = StudyListRes),
        (status = 400, description = "Bad Request", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 422, description = "Validation Error", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = [])),
    tag = "study"
)]
pub async fn list_studies(
    State(state): State<AppState>,
    _auth: AuthUser, // 수정: 매크로 오류 방지를 위해 패턴 매칭 제거하고 변수명 할당
    Query(req): Query<StudyListReq>,
) -> AppResult<Json<StudyListRes>> {
    let res = StudyService::list_studies(&state, req).await?;
    Ok(Json(res))
}

/// 학습 문제 상세 조회
#[utoipa::path(
    get,
    path = "/studies/tasks/{id}",
    params(
        ("id" = i64, Path, description = "Study Task ID")
    ),
    responses(
        (status = 200, description = "Task Detail", body = StudyTaskDetailRes),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = [])),
    tag = "study"
)]
pub async fn get_study_task(
    State(state): State<AppState>,
    _auth: AuthUser, // 수정: 패턴 매칭 제거
    Path(task_id): Path<i64>,
) -> AppResult<Json<StudyTaskDetailRes>> {
    let res = StudyService::get_study_task(&state, task_id).await?;
    Ok(Json(res))
}

/// 정답 제출 및 채점
#[utoipa::path(
    post,
    path = "/studies/tasks/{id}/answer",
    params(
        ("id" = i64, Path, description = "Study Task ID")
    ),
    request_body = SubmitAnswerReq,
    responses(
        (status = 200, description = "Submission Result (Graded)", body = SubmitAnswerRes),
        (status = 400, description = "Bad Request", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = [])),
    tag = "study"
)]
pub async fn submit_answer(
    State(state): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(task_id): Path<i64>,
    Json(req): Json<SubmitAnswerReq>,
) -> AppResult<Json<SubmitAnswerRes>> {
    let res = StudyService::submit_answer(&state, auth_user.sub, task_id, req).await?;
    Ok(Json(res))
}

/// 내 문제 풀이 상태 조회
#[utoipa::path(
    get,
    path = "/studies/tasks/{id}/status",
    params(
        ("id" = i64, Path, description = "Study Task ID")
    ),
    responses(
        (status = 200, description = "My Task Status", body = TaskStatusRes),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = [])),
    tag = "study"
)]
pub async fn get_task_status(
    State(state): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(task_id): Path<i64>,
) -> AppResult<Json<TaskStatusRes>> {
    let res = StudyService::get_task_status(&state, auth_user.sub, task_id).await?;
    Ok(Json(res))
}

/// 해설 조회
#[utoipa::path(
    get,
    path = "/studies/tasks/{id}/explain",
    params(
        ("id" = i64, Path, description = "Study Task ID")
    ),
    responses(
        (status = 200, description = "Task Explanation", body = TaskExplainRes),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = [])),
    tag = "study"
)]
pub async fn get_task_explanation(
    State(state): State<AppState>,
    _auth: AuthUser, // 수정: 패턴 매칭 제거
    Path(task_id): Path<i64>,
) -> AppResult<Json<TaskExplainRes>> {
    let res = StudyService::get_task_explanation(&state, task_id).await?;
    Ok(Json(res))
}