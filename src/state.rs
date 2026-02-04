use axum::extract::FromRef;
use deadpool_redis::Pool as RedisPool;
use sqlx::PgPool;
use std::time::Instant;

use crate::config::Config;
use crate::external::email::EmailClient;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: PgPool,
    pub redis: RedisPool,
    pub cfg: Config,
    pub started_at: Instant,
    /// AWS SES 이메일 클라이언트 (SES_FROM_ADDRESS 설정 시 활성화)
    pub email: Option<EmailClient>,
}

impl AsRef<AppState> for AppState {
    fn as_ref(&self) -> &AppState {
        self
    }
}

#[allow(dead_code)]
fn _assert_state_traits()
where
    AppState: Clone + Send + Sync + 'static,
{
}
