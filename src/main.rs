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
    let db: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(10)
        .connect_lazy(&cfg.database_url)?;

    // 4) Redis 풀 생성
    let redis_cfg = deadpool_redis::Config::from_url(cfg.redis_url.clone());
    let redis: RedisPool = redis_cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Failed to create Redis pool");

    // 5) AppState 생성
    let app_state = AppState {
        db,
        redis,
        cfg: cfg.clone(),
    };
    let app = api::app_router(app_state);

    // 6) 서버 시작
    let listener = TcpListener::bind(&cfg.bind_addr).await?;
    tracing::debug!("✅ Server running at http://{}", cfg.bind_addr);
    tracing::debug!(
        "📘 If Swagger UI is enabled in the router, open: http://{}/docs",
        cfg.bind_addr
    );

    axum::serve(listener, app).await?;
    Ok(())
}
