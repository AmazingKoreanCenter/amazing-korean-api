use super::{
    dto::{CourseDetailRes, CourseListQuery, CourseListRes, CreateCourseReq, CreateCourseRes},
    service::CourseService,
};
use crate::extract::AppJson;
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

#[utoipa::path(
    get,
    path = "/courses",
    tag = "Course",
    params(CourseListQuery),
    responses(
        (status = 200, description = "Course list", body = CourseListRes)
    )
)]
pub async fn list(
    State(st): State<AppState>,
    Query(query): Query<CourseListQuery>,
) -> AppResult<Json<CourseListRes>> {
    let res = CourseService::list(&st, query.lang).await?;
    Ok(Json(res))
}

#[utoipa::path(
    post,
    path = "/courses",
    tag = "Course",
    security(("bearerAuth" = [])),
    request_body = CreateCourseReq,
    responses(
        (status = 200, description = "Course created", body = CreateCourseRes),
        (status = 400, description = "Validation error", body = crate::error::ErrorBody),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorBody)
    )
)]
pub async fn create(
    State(st): State<AppState>,
    AuthUser(_claims): AuthUser,
    AppJson(payload): AppJson<CreateCourseReq>,
) -> AppResult<Json<CreateCourseRes>> {
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
    Ok(Json(CreateCourseRes { course_id: id }))
}

#[utoipa::path(
    get,
    path = "/courses/{id}",
    tag = "Course",
    params(
        ("id" = i64, Path, description = "Course ID"),
        CourseListQuery
    ),
    responses(
        (status = 200, description = "Course detail", body = CourseDetailRes),
        (status = 404, description = "Course not found", body = crate::error::ErrorBody)
    )
)]
pub async fn get_by_id(
    State(st): State<AppState>,
    Path(id): Path<i64>,
    Query(query): Query<CourseListQuery>,
) -> AppResult<Json<CourseDetailRes>> {
    let res = CourseService::get_by_id(&st, id, query.lang).await?;
    Ok(Json(res))
}
