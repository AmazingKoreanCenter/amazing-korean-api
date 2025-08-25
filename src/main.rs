mod api;
mod docs;
mod error;
mod state;

use crate::state::AppState;
use deadpool_redis::Pool as RedisPool;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1) .env 로드(없어도 계속 진행)
    let _ = dotenvy::dotenv();

    // 2) 환경변수 기본값
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432/amk".into());
    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".into());
    let redis_url = env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1:6379/".into());

    // 3) Tracing 초기화
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG")
                .unwrap_or_else(|_| "amazing_korean_api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 4) Postgres 풀 생성
    let db: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(10)
        .connect_lazy(&database_url)?;

    // 5) Redis 풀 생성
    let redis_cfg = deadpool_redis::Config::from_url(redis_url);
    let redis: RedisPool = redis_cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Failed to create Redis pool");

    // 6) AppState 생성
    let app_state = AppState { db, redis };
    let app = api::app_router(app_state);

    // 7) 서버 시작
    let listener = TcpListener::bind(&bind_addr).await?;
    tracing::debug!("✅ Server running at http://{bind_addr}");
    tracing::debug!("📘 If Swagger UI is enabled in the router, open: http://{bind_addr}/docs");

    axum::serve(listener, app).await?;
    Ok(())
}
