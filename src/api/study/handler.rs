use axum::extract::{Query, State};
use axum::Json;

use crate::api::study::repo::StudyRepo;
use crate::error::AppResult;
use crate::state::AppState;

use super::dto::{StudyListReq, StudyListRes};
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
