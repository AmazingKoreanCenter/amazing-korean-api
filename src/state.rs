use axum::extract::FromRef;
use deadpool_redis::Pool as RedisPool;
use sqlx::PgPool;

use crate::config::Config;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: PgPool,
    pub redis: RedisPool,
    pub cfg: Config,
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
