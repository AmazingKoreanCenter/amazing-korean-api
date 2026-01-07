use crate::AppState;
use axum::{routing::get, routing::post, Router};

use super::handler::{admin_bulk_create_studies, admin_create_study, admin_list_studies};

pub fn admin_study_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_list_studies).post(admin_create_study))
        .route("/bulk", post(admin_bulk_create_studies))
}
