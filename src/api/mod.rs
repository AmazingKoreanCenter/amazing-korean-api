use crate::state::AppState;
use axum::routing::get;

use crate::docs::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod admin;
pub mod auth;
pub mod course;
pub mod health;
pub mod user;
pub mod video;

use self::admin::router::admin_router;
use self::auth::router::auth_router;
use self::course::router::course_router;
use self::user::router::user_router;
use self::video::router::router as video_router;

pub fn app_router(state: AppState) -> axum::Router {
    axum::Router::new()
        .merge(course_router())
        .merge(user_router())
        .nest("/auth", auth_router()) // Nest auth_router under /auth
        .nest("/admin", admin_router())
        .nest("/videos", video_router())
        .route("/health", get(health::handler::health))
        .route("/ready", get(health::handler::ready))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state)
}
