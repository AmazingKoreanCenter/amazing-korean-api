use axum::extract::{Query, State};
use axum::Json;

use crate::error::AppResult;
use crate::state::AppState;

use axum::extract::Path;

use super::dto::{LessonDetailReq, LessonDetailRes, LessonListReq, LessonListRes};
use super::repo::LessonRepo;
use super::service::LessonService;

#[utoipa::path(
    get,
    path = "/lessons",
    params(
        ("page", Query, description = "Page number (default 1)"),
        ("per_page", Query, description = "Items per page (default 20, max 50)"),
        ("sort", Query, description = "Sort field (lesson_idx)"),
    ),
    responses(
        (status = 200, description = "List of lessons", body = LessonListRes),
        (status = 400, description = "Bad Request", body = crate::error::ErrorBody),
        (status = 422, description = "Unprocessable Entity", body = crate::error::ErrorBody)
    ),
    tag = "lesson"
)]
pub async fn list_lessons(
    State(state): State<AppState>,
    Query(req): Query<LessonListReq>,
) -> AppResult<Json<LessonListRes>> {
    let service = LessonService::new(LessonRepo::new(state.db.clone()));
    let res = service.list_lessons(req).await?;
    Ok(Json(res))
}

#[utoipa::path(
    get,
    path = "/lessons/{id}",
    params(
        ("id" = i64, Path, description = "Lesson ID"),
        ("page", Query, description = "Page number (default 1)"),
        ("per_page", Query, description = "Items per page (default 20, max 50)")
    ),
    responses(
        (status = 200, description = "Lesson detail", body = LessonDetailRes),
        (status = 400, description = "Bad Request", body = crate::error::ErrorBody),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody),
        (status = 422, description = "Unprocessable Entity", body = crate::error::ErrorBody)
    ),
    tag = "lesson"
)]
pub async fn get_lesson_detail(
    State(state): State<AppState>,
    Path(lesson_id): Path<i64>,
    Query(req): Query<LessonDetailReq>,
) -> AppResult<Json<LessonDetailRes>> {
    let service = LessonService::new(LessonRepo::new(state.db.clone()));
    let res = service.get_lesson_detail(lesson_id, req).await?;
    Ok(Json(res))
}
