use axum::extract::{Query, State};
use axum::Json;

use crate::error::AppResult;
use crate::state::AppState;

use super::dto::{LessonListReq, LessonListRes};
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
