use axum::extract::{Path, Query, State};
use axum::Json;

use crate::api::auth::extractor::{AuthUser, OptionalAuthUser};
use crate::error::AppResult;
use crate::state::AppState;

use super::dto::{
    StudyDetailReq, StudyDetailRes, StudyListReq, StudyListResp, StudyTaskDetailRes,
    SubmitAnswerReq, SubmitAnswerRes, TaskExplainRes, TaskStatusRes,
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
        ("page" = Option<u32>, Query, description = "Page number (default 1)"),
        ("per_page" = Option<u32>, Query, description = "Items per page (default 10, max 100)"),
        ("program" = Option<String>, Query, description = "Program filter (basic_900, topik_read, etc)"),
        ("sort" = Option<String>, Query, description = "Sort order (latest, oldest, alphabetical)")
    ),
    responses(
        (status = 200, description = "List of studies", body = StudyListResp),
        (status = 400, description = "Bad Request", body = crate::error::ErrorBody),
        (status = 422, description = "Validation Error", body = crate::error::ErrorBody)
    ),
    tag = "study"
)]
pub async fn list_studies(
    State(state): State<AppState>,
    Query(req): Query<StudyListReq>,
) -> AppResult<Json<StudyListResp>> {
    let res = StudyService::list_studies(&state, req).await?;
    Ok(Json(res))
}

/// Study 상세 조회 (Study 정보 + Task 목록)
#[utoipa::path(
    get,
    path = "/studies/{id}",
    params(
        ("id" = i32, Path, description = "Study ID"),
        ("page" = Option<u32>, Query, description = "Page number (default 1)"),
        ("per_page" = Option<u32>, Query, description = "Items per page (default 10, max 100)")
    ),
    responses(
        (status = 200, description = "Study detail with task list", body = StudyDetailRes),
        (status = 400, description = "Bad Request", body = crate::error::ErrorBody),
        (status = 404, description = "Study Not Found", body = crate::error::ErrorBody),
        (status = 422, description = "Validation Error", body = crate::error::ErrorBody)
    ),
    tag = "study"
)]
pub async fn get_study_detail(
    State(state): State<AppState>,
    Path(study_id): Path<i32>,
    Query(req): Query<StudyDetailReq>,
) -> AppResult<Json<StudyDetailRes>> {
    let res = StudyService::get_study_detail(&state, study_id, req).await?;
    Ok(Json(res))
}

/// 학습 문제 상세 조회
#[utoipa::path(
    get,
    path = "/studies/tasks/{id}",
    params(
        ("id" = i32, Path, description = "Study Task ID")
    ),
    responses(
        (status = 200, description = "Task Detail", body = StudyTaskDetailRes),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody)
    ),
    tag = "study"
)]
pub async fn get_study_task(
    State(state): State<AppState>,
    OptionalAuthUser(auth): OptionalAuthUser,
    Path(task_id): Path<i32>,
) -> AppResult<Json<StudyTaskDetailRes>> {
    let res = StudyService::get_study_task(&state, task_id, auth).await?;
    Ok(Json(res))
}

/// 정답 제출 및 채점
#[utoipa::path(
    post,
    path = "/studies/tasks/{id}/answer",
    params(
        ("id" = i32, Path, description = "Study Task ID")
    ),
    request_body = SubmitAnswerReq,
    responses(
        (status = 200, description = "Submission Result (Graded)", body = SubmitAnswerRes),
        (status = 400, description = "Bad Request", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody),
        (status = 422, description = "Unprocessable Entity", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = [])),
    tag = "study"
)]
pub async fn submit_answer(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(task_id): Path<i32>,
    Json(req): Json<SubmitAnswerReq>,
) -> AppResult<Json<SubmitAnswerRes>> {
    let res = StudyService::submit_answer(&state, auth_user, task_id, req).await?;
    Ok(Json(res))
}

/// 내 문제 풀이 상태 조회
#[utoipa::path(
    get,
    path = "/studies/tasks/{id}/status",
    params(
        ("id" = i32, Path, description = "Study Task ID")
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
    auth_user: AuthUser,
    Path(task_id): Path<i32>,
) -> AppResult<Json<TaskStatusRes>> {
    let res = StudyService::get_task_status(&state, auth_user, task_id).await?;
    Ok(Json(res))
}

/// 해설 조회
#[utoipa::path(
    get,
    path = "/studies/tasks/{id}/explain",
    params(
        ("id" = i32, Path, description = "Study Task ID")
    ),
    responses(
        (status = 200, description = "Task Explanation", body = TaskExplainRes),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 403, description = "Forbidden", body = crate::error::ErrorBody),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = [])),
    tag = "study"
)]
pub async fn get_task_explain(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(task_id): Path<i32>,
) -> AppResult<Json<TaskExplainRes>> {
    let res = StudyService::get_task_explain(&state, auth_user, task_id).await?;
    Ok(Json(res))
}
