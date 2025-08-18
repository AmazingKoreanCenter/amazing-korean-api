use axum::{routing::get};
use crate::state::AppState;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::docs::ApiDoc;        

pub mod course;
pub mod auth;
pub mod user;

use self::course::router::course_router;
use self::auth::router::auth_router;
pub fn app_router(state: AppState) -> axum::Router {
    axum::Router::new()
        .merge(course_router())
        .merge(auth_router())
        .merge(user::router())
        .route("/healthz", get(|| async { "ok" }))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state)
}
