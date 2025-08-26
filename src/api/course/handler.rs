use super::{
    dto::{CourseListItem, CreateCourseReq},
    service::CourseService,
};
use crate::{
    error::{AppError, AppResult},
    state::AppState,
};
use axum::{
    extract::{Path, State},
    Json,
};
use validator::Validate;

pub async fn list(State(st): State<AppState>) -> AppResult<Json<Vec<CourseListItem>>> {
    let items = CourseService::list(&st).await?;
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
) -> AppResult<Json<CourseListItem>> {
    // ⬇️ 매크로 버전(!) 대신 제네릭 버전 사용 → 컴파일 시 DB 접속 불필요
    let row = sqlx::query_as::<_, CourseListItem>(
        r#"
        SELECT
            course_id,
            course_title,
            course_price,
            course_type,
            course_state
        FROM course
        WHERE course_id = $1
    "#,
    )
    .bind(id)
    .fetch_optional(&st.db)
    .await?;

    match row {
        Some(course) => Ok(Json(course)),
        None => Err(AppError::NotFound),
    }
}
