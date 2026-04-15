use axum::extract::FromRef;
use deadpool_redis::Pool as RedisPool;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Instant;

use crate::config::Config;
use crate::external::apple::AppleOAuthClient;
use crate::external::email::EmailSender;
use crate::external::ipgeo::IpGeoClient;
use crate::external::payment::PaymentProvider;
use crate::external::revenuecat::RevenueCatClient;

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
    /// 결제 프로바이더 (PAYMENT_PROVIDER 설정에 따라 Paddle Billing 사용)
    pub payment: Option<Arc<dyn PaymentProvider>>,
    /// RevenueCat 클라이언트 (모바일 IAP 영수증 검증)
    pub revenuecat: Option<Arc<dyn RevenueCatClient>>,
    /// Apple OAuth 클라이언트 (Sign in with Apple — JWKS 캐시 + reqwest 커넥션 풀 싱글톤)
    pub apple_oauth: Option<Arc<AppleOAuthClient>>,
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
