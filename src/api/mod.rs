use axum::{middleware, routing::get};

use crate::docs::ApiDoc;
use crate::state::AppState;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod admin;
pub mod auth;
pub mod course;
pub mod health;
pub mod lesson;
pub mod study;
pub mod user;
pub mod video;

use self::admin::ip_guard::admin_ip_guard;
use self::admin::role_guard::admin_role_guard;
use self::admin::router::admin_router;
use self::auth::router::auth_router;
use self::course::router::course_router;
use self::lesson::router::router as lesson_router;
use self::study::router::router as study_router;
use self::user::router::user_router;
use self::video::router::router as video_router;

pub fn app_router(state: AppState) -> axum::Router {
    axum::Router::new()
        .merge(course_router())
        .merge(user_router())
        .nest("/auth", auth_router())
        // Admin 라우트에 IP allowlist + Role Guard 미들웨어 적용
        // Layer 순서: ip_guard → role_guard (IP 먼저 체크 후 역할 체크)
        .nest(
            "/admin",
            admin_router()
                .layer(middleware::from_fn_with_state(state.clone(), admin_role_guard))
                .layer(middleware::from_fn_with_state(state.clone(), admin_ip_guard)),
        )
        .nest("/lessons", lesson_router())
        .nest("/videos", video_router())
        .nest("/studies", study_router())
        .route("/healthz", get(health::handler::health))
        .route("/health", get(health::handler::health))
        .route("/ready", get(health::handler::ready))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state)
}
