use axum::{
    http::{header, StatusCode},
    middleware,
    response::IntoResponse,
    routing::get,
    Json,
};
use serde_json::json;

use crate::docs::ApiDoc;
use crate::error::AppError;
use crate::state::AppState;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod admin;
pub mod auth;
pub mod course;
pub mod ebook;
pub mod health;
pub mod lesson;
pub mod payment;
pub mod study;
pub mod textbook;
pub mod user;
pub mod util;
pub mod video;

use self::admin::ip_guard::admin_ip_guard;
use self::admin::role_guard::admin_role_guard;
use self::admin::router::admin_router;
use self::auth::router::auth_router;
use self::course::router::course_router;
use self::lesson::router::router as lesson_router;
use self::payment::router::payment_router;
use self::study::router::router as study_router;
use self::ebook::router::ebook_router;
use self::textbook::router::textbook_router;
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
        .nest("/payment", payment_router())
        .nest("/textbook", textbook_router())
        .nest("/ebook", ebook_router())
        .route("/healthz", get(health::handler::health))
        .route("/health", get(health::handler::health))
        .route("/ready", get(health::handler::ready))
        // SEO: api 서브도메인 루트 + robots.txt — Google Search Console 의
        // "찾을 수 없음(404)" + "Soft 404" 카테고리 회피용.
        // api.amazingkorean.net 은 Google OAuth redirect_uri 로 불가피하게
        // 외부 노출되므로 (1) 루트는 200 JSON 서비스 메타 응답,
        // (2) robots.txt 는 전체 크롤링 금지.
        .route("/", get(root_service_info))
        .route("/robots.txt", get(robots_txt))
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

/// 루트 경로 — 서비스 메타 정보 200 JSON 응답
/// Why: Google 이 OAuth redirect_uri 경로로 api 서브도메인을 알게 되고, 이후
/// Domain Property 자동 감지로 api.amazingkorean.net/ 을 크롤링한다. 과거엔
/// fallback_404 로 처리돼 Search Console 에 "찾을 수 없음(404)" 으로 분류됐다.
/// 200 을 돌려주면 해당 카테고리에서 빠지고, robots.txt 와 조합해 색인 제외 상태로 유지된다.
async fn root_service_info() -> impl IntoResponse {
    Json(json!({
        "service": "Amazing Korean API",
        "status": "ok",
        "docs": null
    }))
}

/// robots.txt — 전체 크롤링 금지
/// Why: api 서브도메인은 사람이 브라우저로 읽는 콘텐츠가 아니라 JSON API 다.
/// Google 색인 대상이 아님을 명시해 Search Console 의 "Soft 404" / "리디렉션" /
/// "찾을 수 없음" 카테고리에 api 경로가 뜨지 않도록 차단한다.
async fn robots_txt() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        "User-agent: *\nDisallow: /\n",
    )
}
