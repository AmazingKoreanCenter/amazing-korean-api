use amazing_korean_api::api;
use amazing_korean_api::config::Config;
use amazing_korean_api::external;
use amazing_korean_api::state::AppState;
use deadpool_redis::Pool as RedisPool;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// [CORS] 필요한 모듈 추가
use tower_http::cors::CorsLayer;
use http::{Method, HeaderValue, HeaderName, header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}};

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

    // 3.5) DB 마이그레이션 자동 실행
    sqlx::migrate!().run(&pool).await?;
    tracing::info!("📦 Database migrations applied");

    // 4) Redis 풀 생성
    let redis_cfg = deadpool_redis::Config::from_url(cfg.redis_url.clone());
    let redis: RedisPool = redis_cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Failed to create Redis pool");

    // 5) EmailSender 생성 (EMAIL_PROVIDER 설정에 따라 분기)
    let email: Option<Arc<dyn external::email::EmailSender>> = match cfg.email_provider.as_str() {
        "resend" => {
            let api_key = cfg.resend_api_key.clone()
                .expect("RESEND_API_KEY required for resend provider");
            let from = cfg.email_from_address.clone()
                .expect("EMAIL_FROM_ADDRESS required for resend provider");
            tracing::info!("📩 Email client enabled: Resend (from: {})", from);
            Some(Arc::new(external::email::ResendEmailSender::new(api_key, from)))
        }
        "none" => {
            tracing::info!("Email client disabled (EMAIL_PROVIDER=none)");
            None
        }
        other => {
            panic!("Unknown EMAIL_PROVIDER '{}'. Must be 'resend' or 'none'.", other);
        }
    };

    // 6) IpGeoClient 생성
    let ipgeo = Arc::new(external::ipgeo::IpGeoClient::new());
    tracing::info!("🌍 IP Geolocation client enabled (ip-api.com)");

    // 6.7) PaymentProvider 생성 (PAYMENT_PROVIDER 설정에 따라 분기)
    let payment: Option<Arc<dyn external::payment::PaymentProvider>> =
        match cfg.payment_provider.as_str() {
            "paddle" => {
                let api_key = cfg
                    .paddle_api_key
                    .clone()
                    .expect("PADDLE_API_KEY required for paddle provider");
                let client_token = cfg
                    .paddle_client_token
                    .clone()
                    .expect("PADDLE_CLIENT_TOKEN required for paddle provider");
                let env_label = if cfg.paddle_sandbox { "Sandbox" } else { "Production" };
                tracing::info!(
                    "💳 Payment provider enabled: Paddle Billing ({})",
                    env_label
                );
                Some(Arc::new(
                    external::payment::PaddleProvider::new(
                        &api_key,
                        cfg.paddle_sandbox,
                        client_token,
                    )
                    .expect("Failed to create Paddle client"),
                ))
            }
            "none" => {
                tracing::info!("Payment provider disabled (PAYMENT_PROVIDER=none)");
                None
            }
            other => {
                panic!(
                    "Unknown PAYMENT_PROVIDER '{}'. Must be 'paddle' or 'none'.",
                    other
                );
            }
        };

    // 6.5) RevenueCat 클라이언트 (모바일 IAP)
    let revenuecat: Option<Arc<dyn external::revenuecat::RevenueCatClient>> =
        if let Some(api_key) = &cfg.revenuecat_api_key {
            tracing::info!("RevenueCat client initialized");
            Some(Arc::new(external::revenuecat::RevenueCatApiClient::new(api_key.clone())))
        } else {
            tracing::info!("RevenueCat client disabled (REVENUECAT_API_KEY not set)");
            None
        };

    // 6.9) E-book 워터마크 폰트 초기화
    let watermark_font_path = format!("{}/NotoSans-Regular.ttf", cfg.ebook_page_images_dir);
    amazing_korean_api::api::ebook::watermark::init_font(&watermark_font_path);

    // 7) AppState 생성
    let app_state = AppState {
        db: pool,
        redis,
        cfg: cfg.clone(),
        started_at: Instant::now(),
        email,
        ipgeo,
        payment,
        revenuecat,
    };

    // 8) [CORS] 설정 정의
    // 환경변수 CORS_ORIGINS에서 허용할 origin 목록을 읽음
    // 예: CORS_ORIGINS=http://localhost:5173,https://amazing-korean-api.pages.dev
    let origins: Vec<HeaderValue> = cfg
        .cors_origins
        .iter()
        .filter_map(|o| o.parse::<HeaderValue>().ok())
        .collect();

    tracing::info!("🌐 CORS allowed origins: {:?}", cfg.cors_origins);

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS
        ])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT, HeaderName::from_static("x-ebook-viewer"), HeaderName::from_static("x-ebook-session"), HeaderName::from_static("x-ebook-signature"), HeaderName::from_static("x-ebook-timestamp"), HeaderName::from_static("x-platform")])
        .allow_credentials(true); // 쿠키(Refresh Token) 교환을 위해 필수

    // 8) 라우터에 CORS + 보안 헤더 레이어 적용
    let app = api::app_router(app_state)
        .layer(cors)
        .layer(axum::middleware::from_fn(security_headers));

    // 9) 서버 시작
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

/// PROD-4: 보안 헤더 미들웨어 — 모든 응답에 보안 헤더 추가
async fn security_headers(
    request: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert("x-content-type-options", "nosniff".parse().unwrap());
    headers.insert("x-frame-options", "DENY".parse().unwrap());
    headers.insert("x-xss-protection", "0".parse().unwrap());
    headers.insert(
        "permissions-policy",
        "camera=(), microphone=(), geolocation=()".parse().unwrap(),
    );
    response
}