use crate::extract::AppJson;
use super::{
    dto::{CourseDetailRes, CourseListQuery, CourseListRes, CreateCourseReq},
    service::CourseService,
};
use crate::{
    api::auth::extractor::AuthUser,
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
) -> AppResult<Json<CourseListRes>> {
    let res = CourseService::list(&st, query.lang).await?;
    Ok(Json(res))
}

pub async fn create(
    State(st): State<AppState>,
    AuthUser(_claims): AuthUser,
    AppJson(payload): AppJson<CreateCourseReq>,
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
) -> AppResult<Json<CourseDetailRes>> {
    let res = CourseService::get_by_id(&st, id, query.lang).await?;
    Ok(Json(res))
}
