use super::{
    dto::{CourseListItem, CourseListQuery, CreateCourseReq},
    service::CourseService,
};
use crate::{
    error::{AppError, AppResult},
    state::AppState,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use validator::Validate;

pub async fn list(
    State(st): State<AppState>,
    Query(query): Query<CourseListQuery>,
) -> AppResult<Json<Vec<CourseListItem>>> {
    let items = CourseService::list(&st, query.lang).await?;
    Ok(Json(items))
}

pub async fn create(
    State(st): State<AppState>,
    Json(payload): Json<CreateCourseReq>,
) -> AppResult<Json<serde_json::Value>> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let id = CourseService::create(
        &st,
        &payload.title,
        payload.price,
        &payload.course_type,
        payload.subtitle.as_deref(),
    )
    .await?;
    Ok(Json(serde_json::json!({ "course_id": id })))
}

pub async fn get_by_id(
    State(st): State<AppState>,
    Path(id): Path<i64>,
    Query(query): Query<CourseListQuery>,
) -> AppResult<Json<CourseListItem>> {
    let item = CourseService::get_by_id(&st, id, query.lang).await?;
    Ok(Json(item))
}
