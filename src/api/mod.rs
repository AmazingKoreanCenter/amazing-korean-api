use crate::state::AppState;
use axum::routing::get;

use crate::docs::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod auth;
pub mod course;
pub mod user;

use self::auth::router::auth_router;
use self::course::router::course_router;
use self::user::router::user_router;

pub fn app_router(state: AppState) -> axum::Router {
    axum::Router::new()
        .merge(course_router())
        .merge(user_router())
        .merge(auth_router())
        .route("/healthz", get(|| async { "ok" }))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state)
}
