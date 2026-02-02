use axum::extract::{Path, Query, State};
use axum::Json;

use crate::api::auth::extractor::{AuthUser, OptionalAuthUser};
use crate::error::AppResult;
use crate::state::AppState;

use super::dto::{
    LessonDetailReq, LessonDetailRes, LessonItemsReq, LessonItemsRes, LessonListReq, LessonListRes,
    LessonProgressRes, LessonProgressUpdateReq,
};
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
    let res = LessonService::list_lessons(&state.db, req).await?;
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
    let res = LessonService::get_lesson_detail(&state.db, lesson_id, req).await?;
    Ok(Json(res))
}

#[utoipa::path(
    get,
    path = "/lessons/{id}/items",
    params(
        ("id" = i64, Path, description = "Lesson ID"),
        ("page", Query, description = "Page number (default 1)"),
        ("per_page", Query, description = "Items per page (default 20, max 50)")
    ),
    responses(
        (status = 200, description = "Lesson items (study view)", body = LessonItemsRes),
        (status = 400, description = "Bad Request", body = crate::error::ErrorBody),
        (status = 403, description = "Forbidden (Paid content requires subscription)", body = crate::error::ErrorBody),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody),
        (status = 422, description = "Unprocessable Entity", body = crate::error::ErrorBody)
    ),
    tag = "lesson"
)]
pub async fn get_lesson_items(
    State(state): State<AppState>,
    OptionalAuthUser(auth): OptionalAuthUser,
    Path(lesson_id): Path<i64>,
    Query(req): Query<LessonItemsReq>,
) -> AppResult<Json<LessonItemsRes>> {
    let user_id = auth.map(|a| a.0.sub);
    let res = LessonService::get_lesson_items(&state, lesson_id, req, user_id).await?;
    Ok(Json(res))
}

#[utoipa::path(
    get,
    path = "/lessons/{id}/progress",
    params(
        ("id" = i64, Path, description = "Lesson ID")
    ),
    responses(
        (status = 200, description = "Lesson progress", body = LessonProgressRes),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = [])),
    tag = "lesson"
)]
pub async fn get_lesson_progress(
    State(state): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(lesson_id): Path<i64>,
) -> AppResult<Json<LessonProgressRes>> {
    let res = LessonService::get_lesson_progress(&state.db, auth_user.sub, lesson_id).await?;
    Ok(Json(res))
}

#[utoipa::path(
    post,
    path = "/lessons/{id}/progress",
    params(
        ("id" = i64, Path, description = "Lesson ID")
    ),
    request_body(
        content = LessonProgressUpdateReq,
        description = "Lesson progress update data",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Lesson progress", body = LessonProgressRes),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody),
        (status = 404, description = "Not Found", body = crate::error::ErrorBody),
        (status = 422, description = "Unprocessable Entity", body = crate::error::ErrorBody)
    ),
    security(("bearerAuth" = [])),
    tag = "lesson"
)]
pub async fn update_lesson_progress(
    State(state): State<AppState>,
    AuthUser(auth_user): AuthUser,
    Path(lesson_id): Path<i64>,
    Json(req): Json<LessonProgressUpdateReq>,
) -> AppResult<Json<LessonProgressRes>> {
    let res = LessonService::update_lesson_progress(&state.db, auth_user.sub, lesson_id, req).await?;
    Ok(Json(res))
}
