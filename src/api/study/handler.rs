use axum::extract::{Path, Query, State};
use axum::Json;

use crate::api::auth::extractor::AuthUser;
use crate::api::study::repo::StudyRepo;
use crate::error::AppResult;
use crate::state::AppState;

use super::dto::{StudyListReq, StudyListRes, StudyTaskDetailRes, SubmitAnswerReq, SubmitAnswerRes};
use super::service::StudyService;

#[utoipa::path(
    get,
    path = "/studies",
    params(
        ("page", Query, description = "Page number (default 1)"),
        ("per_page", Query, description = "Items per page (default 10, max 100)"),
        ("program", Query, description = "Study program (basic_pronunciation, basic_word, basic_900, topik_read, topik_listen, topik_write, tbc)"),
        ("sort", Query, description = "Sort by field (created_at_desc, created_at_asc, title_asc, title_desc)")
    ),
    responses(
        (status = 200, description = "List of studies", body = StudyListRes),
        (status = 400, description = "Bad Request", body = crate::error::ErrorBody),
        (status = 422, description = "Unprocessable Entity", body = crate::error::ErrorBody)
    ),
    tag = "study"
)]
pub async fn list_studies(
    State(state): State<AppState>,
    Query(req): Query<StudyListReq>,
) -> AppResult<Json<StudyListRes>> {
    let service = StudyService::new(StudyRepo::new(state.db.clone()));
    let res = service.list_studies(req).await?;
    Ok(Json(res))
}

#[utoipa::path(
    get,
    path = "/studies/tasks/{id}",
    params(
        ("id" = i64, Path, description = "Study Task ID")
    ),
    responses(
        (status = 200, description = "Study task detail", body = StudyTaskDetailRes),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody)
    ),
    tag = "study"
)]
pub async fn get_study_task(
    State(state): State<AppState>,
    Path(task_id): Path<i64>,
) -> AppResult<Json<StudyTaskDetailRes>> {
    let service = StudyService::new(StudyRepo::new(state.db.clone()));
    let res = service.get_task_detail(task_id).await?;
    Ok(Json(res))
}

#[utoipa::path(
    post,
    path = "/studies/tasks/{id}/answer",
    params(
        ("id" = i64, Path, description = "Study Task ID")
    ),
    request_body(content = SubmitAnswerReq, description = "Study task answer submission", content_type = "application/json"),
    responses(
        (status = 200, description = "Submission result", body = SubmitAnswerRes),
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
    AuthUser(auth_user): AuthUser,
    Path(task_id): Path<i64>,
    Json(req): Json<SubmitAnswerReq>,
) -> AppResult<Json<SubmitAnswerRes>> {
    let service = StudyService::new(StudyRepo::new(state.db.clone()));
    let res = service
        .submit_answer(auth_user.sub, &auth_user.session_id, task_id, req)
        .await?;
    Ok(Json(res))
}
