mod api;
mod docs;
mod error;
mod state;

use crate::state::AppState;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1) .env 로드(없어도 계속 진행)
    let _ = dotenvy::dotenv();

    // 2) 환경변수 기본값
    let skip_db = env::var("SKIP_DB").unwrap_or_else(|_| "0".into()) == "1";
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@127.0.0.1:5432/amk".into());
    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".into());

    // 3) 항상 lazy 풀 생성 (sqlx 0.7: Result 반환 → ? 처리)
    let db: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(10)
        .connect_lazy(&database_url)?; // ← 중요

    // 4) 실제 연결 확인은 필요할 때만(문서만 볼 땐 SKIP_DB=1로 건너뜀)
    if !skip_db {
        sqlx::query("SELECT 1").execute(&db).await?;
    }

    // 5) 라우터 (⚠️ Swagger UI는 api::app_router 쪽에 이미 등록되어 있다고 가정)
    let app_state = AppState { db };
    let app = api::app_router(app_state);

    // 6) 서버 시작
    let listener = TcpListener::bind(&bind_addr).await?;
    println!("✅ Server running at http://{bind_addr}");
    println!("📘 If Swagger UI is enabled in the router, open: http://{bind_addr}/docs");

    axum::serve(listener, app).await?;
    Ok(())
}
