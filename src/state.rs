use axum::extract::FromRef;
use deadpool_redis::Pool as RedisPool;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Instant;

use crate::config::Config;
use crate::external::email::EmailSender;
use crate::external::ipgeo::IpGeoClient;
use crate::external::translator::TranslationProvider;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: PgPool,
    pub redis: RedisPool,
    pub cfg: Config,
    pub started_at: Instant,
    /// 이메일 클라이언트 (EMAIL_PROVIDER 설정에 따라 Resend 사용)
    pub email: Option<Arc<dyn EmailSender>>,
    /// IP Geolocation 클라이언트
    pub ipgeo: Arc<IpGeoClient>,
    /// 번역 프로바이더 (TRANSLATE_PROVIDER 설정에 따라 Google Cloud Translation 사용)
    pub translator: Option<Arc<dyn TranslationProvider>>,
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
