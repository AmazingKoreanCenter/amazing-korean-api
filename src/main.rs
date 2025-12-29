mod api;
mod config;
mod docs;
mod error;
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

    // 5) AppState 생성
    let app_state = AppState {
        db: pool,
        redis,
        cfg: cfg.clone(),
        started_at: Instant::now(),
    };
    let app = api::app_router(app_state);

    // 6) 서버 시작
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
