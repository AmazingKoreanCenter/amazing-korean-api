mod api;
mod config;
mod docs;
mod error;
mod external;
mod state;
mod types;

use crate::config::Config;
use crate::state::AppState;
use deadpool_redis::Pool as RedisPool;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::time::{Duration, Instant};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// [CORS] 필요한 모듈 추가
use tower_http::cors::CorsLayer;
use http::{Method, HeaderValue, header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1) 환경변수 로드 및 설정 파싱
    let cfg = Config::from_env();

    // 2) Tracing 초기화
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "amazing_korean_api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 3) Postgres 풀 생성
    let database_url = if cfg.database_url.contains("?") {
        cfg.database_url.clone()
    } else {
        format!("{}?connect_timeout=5", cfg.database_url)
    };

    let pool: Pool<Postgres> = if std::env::var("DB_EAGER").ok().as_deref() == Some("1") {
        PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(5))
            .connect(&database_url)
            .await?
    } else {
        PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(5))
            .connect_lazy(&database_url)?
    };

    // 4) Redis 풀 생성
    let redis_cfg = deadpool_redis::Config::from_url(cfg.redis_url.clone());
    let redis: RedisPool = redis_cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Failed to create Redis pool");

    // 5) EmailClient 생성 (SES_FROM_ADDRESS 설정 시 활성화)
    let email = if let Some(ref from_address) = cfg.ses_from_address {
        tracing::info!("📧 Email client enabled (from: {})", from_address);
        Some(
            external::email::EmailClient::new(
                &cfg.aws_region,
                from_address.clone(),
                cfg.ses_reply_to.clone(),
            )
            .await,
        )
    } else {
        tracing::info!("📧 Email client disabled (SES_FROM_ADDRESS not set)");
        None
    };

    // 6) AppState 생성
    let app_state = AppState {
        db: pool,
        redis,
        cfg: cfg.clone(),
        started_at: Instant::now(),
        email,
    };

    // 7) [CORS] 설정 정의
    // 환경변수 CORS_ORIGINS에서 허용할 origin 목록을 읽음
    // 예: CORS_ORIGINS=http://localhost:5173,https://amazing-korean-api.pages.dev
    let origins: Vec<HeaderValue> = cfg
        .cors_origins
        .iter()
        .filter_map(|o| o.parse::<HeaderValue>().ok())
        .collect();

    tracing::info!("🌐 CORS allowed origins: {:?}", cfg.cors_origins);

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS
        ])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT])
        .allow_credentials(true); // 쿠키(Refresh Token) 교환을 위해 필수

    // 8) 라우터에 CORS 레이어 적용
    let app = api::app_router(app_state).layer(cors);

    // 9) 서버 시작
    let listener = TcpListener::bind(&cfg.bind_addr).await?;
    tracing::info!(
        "✅ Server listening on http://{} (pid={})",
        cfg.bind_addr,
        std::process::id()
    );
    tracing::debug!(
        "📘 If Swagger UI is enabled in the router, open: http://{}/docs",
        cfg.bind_addr
    );

    axum::serve(listener, app).await?;
    Ok(())
}