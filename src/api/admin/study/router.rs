use crate::AppState;
use axum::{routing::get, Router};

use super::handler::admin_list_studies;

pub fn admin_study_router() -> Router<AppState> {
    Router::new().route("/", get(admin_list_studies))
}
