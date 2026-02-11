use axum::{middleware, response::IntoResponse, routing::get};

use crate::docs::ApiDoc;
use crate::error::AppError;
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
    let router = axum::Router::new()
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
        .fallback(fallback_404);

    // PROD-6: enable_docs=false(기본값)이면 Swagger UI 비활성화
    let router = if state.cfg.enable_docs {
        router.merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
    } else {
        router
    };

    router.with_state(state)
}

/// PROD-8: 존재하지 않는 라우트에 JSON 404 응답
async fn fallback_404() -> impl IntoResponse {
    AppError::NotFound.into_response()
}
