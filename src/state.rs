use deadpool_redis::Pool as RedisPool;
use sqlx::PgPool;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: RedisPool,
    pub cfg: Config,
}

#[allow(dead_code)]
fn _assert_state_traits()
where
    AppState: Clone + Send + Sync + 'static,
{
}
