use std::env;

#[derive(Clone)]
pub struct Config {
    #[allow(dead_code)]
    pub database_url: String,
    #[allow(dead_code)]
    pub bind_addr: String,
    #[allow(dead_code)]
    pub redis_url: String,
    #[allow(dead_code)]
    pub jwt_secret: String,
    #[allow(dead_code)]
    pub jwt_expire_hours: i64,
    #[allow(dead_code)]
    pub enable_docs: bool,
    #[allow(dead_code)]
    pub skip_db: bool,
    pub jwt_access_ttl_min: i64,
    pub refresh_ttl_days: i64,
    pub refresh_cookie_name: String,
    pub refresh_cookie_domain: Option<String>,
    pub refresh_cookie_secure: bool,
    pub refresh_cookie_samesite: String,
    pub rate_limit_login_window_sec: i64,
    pub rate_limit_login_max: i64,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok(); // Load .env file

        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:5432/amazing_korean_db".into()
        });
        let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".into());
        let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".into());
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expire_hours = env::var("JWT_EXPIRE_HOURS")
            .unwrap_or_else(|_| "24".into())
            .parse::<i64>()
            .expect("JWT_EXPIRE_HOURS must be a number");
        let enable_docs = env::var("ENABLE_DOCS").map(|s| s == "1").unwrap_or(false);
        let skip_db = env::var("SKIP_DB").map(|s| s == "1").unwrap_or(false);
        let jwt_access_ttl_min = env::var("JWT_ACCESS_TTL_MIN")
            .unwrap_or_else(|_| "15".into())
            .parse::<i64>()
            .expect("JWT_ACCESS_TTL_MIN must be a number");
        let refresh_ttl_days = env::var("REFRESH_TTL_DAYS")
            .unwrap_or_else(|_| "30".into())
            .parse::<i64>()
            .expect("REFRESH_TTL_DAYS must be a number");
        let refresh_cookie_name =
            env::var("REFRESH_COOKIE_NAME").unwrap_or_else(|_| "ak_refresh".into());
        let refresh_cookie_domain = env::var("REFRESH_COOKIE_DOMAIN").ok();
        let refresh_cookie_secure = env::var("REFRESH_COOKIE_SECURE")
            .map(|s| s == "true")
            .unwrap_or(false);
        let refresh_cookie_samesite =
            env::var("REFRESH_COOKIE_SAMESITE").unwrap_or_else(|_| "Lax".into());
        let rate_limit_login_window_sec = env::var("RATE_LIMIT_LOGIN_WINDOW_SEC")
            .unwrap_or_else(|_| "900".into())
            .parse::<i64>()
            .expect("RATE_LIMIT_LOGIN_WINDOW_SEC must be a number");
        let rate_limit_login_max = env::var("RATE_LIMIT_LOGIN_MAX")
            .unwrap_or_else(|_| "10".into())
            .parse::<i64>()
            .expect("RATE_LIMIT_LOGIN_MAX must be a number");

        Self {
            database_url,
            bind_addr,
            redis_url,
            jwt_secret,
            jwt_expire_hours,
            enable_docs,
            skip_db,
            jwt_access_ttl_min,
            refresh_ttl_days,
            refresh_cookie_name,
            refresh_cookie_domain,
            refresh_cookie_secure,
            refresh_cookie_samesite,
            rate_limit_login_window_sec,
            rate_limit_login_max,
        }
    }

    pub fn refresh_cookie_samesite_or<'a>(&'a self, default: &'a str) -> &'a str {
        if self.refresh_cookie_samesite.is_empty() {
            default
        } else {
            &self.refresh_cookie_samesite
        }
    }
}
