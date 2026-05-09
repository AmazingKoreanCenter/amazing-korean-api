//! Phase 2 통합 테스트 공통 helper.
//!
//! ## 셋업
//!
//! 환경변수 = `.env.test` 또는 `.env` (자동 로드, dotenvy). 필수:
//! - `DATABASE_URL` — 로컬 PostgreSQL (amk-pg 컨테이너)
//! - `REDIS_URL` — 로컬 Redis (amk-redis 컨테이너)
//! - `JWT_SECRET` (32+ bytes)
//! - `HMAC_KEY` (base64-encoded 32 bytes)
//! - `ENCRYPTION_KEY_V1` (base64-encoded 32 bytes)
//!
//! 그 외 = `.env.example` 의 default 또는 unwrap_or 폴백.
//!
//! ## 채택 패턴
//!
//! - `EMAIL_PROVIDER=none` / `PAYMENT_PROVIDER=none` = mock 불필요
//! - `IpGeoClient::new()` = HTTP 클라이언트 default (test 함수가 ipgeo 호출 안 하면 영향 0)
//! - `payment` / `revenuecat` / `apple_oauth` = None
//! - `started_at` = `Instant::now()`
//!
//! ## 격리
//!
//! Redis key prefix `test:` 권장 (테스트 간 충돌 회피, 본 helper 는 prefix 미적용 = 각 테스트가
//! 직접 처리). PostgreSQL = 기존 amk-pg DB 사용 (transaction rollback 또는 namespace 분리는 각
//! 테스트 책임).

use amazing_korean_api::config::Config;
use amazing_korean_api::external::ipgeo::IpGeoClient;
use amazing_korean_api::state::AppState;
use deadpool_redis::{Config as RedisConfig, Runtime};
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Instant;

/// 테스트용 AppState 생성. dotenv 자동 로드.
///
/// 호출 시점에 환경변수가 있어야 함 (`.env.test` 또는 `.env`). 없으면 panic.
pub async fn make_test_state() -> AppState {
    // dotenv 자동 로드 — .env.test 우선, .env fallback. 둘 다 없으면 export 된 env vars 만 사용.
    let _ = dotenvy::from_filename(".env.test").or_else(|_| dotenvy::dotenv());

    let cfg = Config::from_env();

    let db = PgPool::connect(&cfg.database_url)
        .await
        .expect("PgPool::connect 실패 — DATABASE_URL 의 PostgreSQL 동작 확인");

    let redis_cfg = RedisConfig::from_url(&cfg.redis_url);
    let redis = redis_cfg
        .create_pool(Some(Runtime::Tokio1))
        .expect("RedisPool 생성 실패 — REDIS_URL 확인");

    let ipgeo = Arc::new(IpGeoClient::new());

    AppState {
        db,
        redis,
        cfg,
        started_at: Instant::now(),
        email: None,
        ipgeo,
        payment: None,
        revenuecat: None,
        apple_oauth: None,
    }
}
