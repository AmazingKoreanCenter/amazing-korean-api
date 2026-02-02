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
    pub refresh_ttl_days_admin: i64,  // HYMN, Admin, Manager용 (더 짧은 TTL)
    pub refresh_ttl_days_hymn: i64,   // HYMN 전용 (가장 짧은 TTL)
    pub refresh_cookie_name: String,
    pub refresh_cookie_domain: Option<String>,
    pub refresh_cookie_secure: bool,
    pub refresh_cookie_samesite: String,
    pub rate_limit_login_window_sec: i64,
    pub rate_limit_login_max: i64,
    pub rate_limit_study_window_sec: i64,  // Study 답안 제출 레이트리밋 윈도우 (초)
    pub rate_limit_study_max: i64,         // Study 답안 제출 최대 횟수/윈도우
    pub cors_origins: Vec<String>,
    pub vimeo_access_token: Option<String>,
    pub admin_ip_allowlist: Vec<String>,  // Admin 접근 허용 IP 목록 (비어있으면 모든 IP 허용)
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

        // Security: JWT secret should be at least 32 bytes for HS256
        if jwt_secret.len() < 32 {
            panic!("JWT_SECRET must be at least 32 bytes for security. Current length: {}", jwt_secret.len());
        }
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
        let refresh_ttl_days_admin = env::var("REFRESH_TTL_DAYS_ADMIN")
            .unwrap_or_else(|_| "7".into())
            .parse::<i64>()
            .expect("REFRESH_TTL_DAYS_ADMIN must be a number");
        let refresh_ttl_days_hymn = env::var("REFRESH_TTL_DAYS_HYMN")
            .unwrap_or_else(|_| "1".into())
            .parse::<i64>()
            .expect("REFRESH_TTL_DAYS_HYMN must be a number");
        let refresh_cookie_name =
            env::var("REFRESH_COOKIE_NAME").unwrap_or_else(|_| "ak_refresh".into());
        let refresh_cookie_domain = env::var("REFRESH_COOKIE_DOMAIN")
            .ok()
            .filter(|s| !s.is_empty());
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

        // Study Rate Limit: 답안 제출 과도한 요청 방지
        let rate_limit_study_window_sec = env::var("RATE_LIMIT_STUDY_WINDOW_SEC")
            .unwrap_or_else(|_| "60".into())
            .parse::<i64>()
            .expect("RATE_LIMIT_STUDY_WINDOW_SEC must be a number");
        let rate_limit_study_max = env::var("RATE_LIMIT_STUDY_MAX")
            .unwrap_or_else(|_| "30".into())
            .parse::<i64>()
            .expect("RATE_LIMIT_STUDY_MAX must be a number");

        // CORS_ORIGINS: 쉼표로 구분된 origin 목록
        // 예: "http://localhost:5173,https://amazing-korean-api.pages.dev"
        let cors_origins = env::var("CORS_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:5173".into())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        // Vimeo API Access Token (optional)
        let vimeo_access_token = env::var("VIMEO_ACCESS_TOKEN").ok();

        // Admin IP Allowlist (optional, 쉼표로 구분)
        // 예: "127.0.0.1,192.168.1.0/24,10.0.0.0/8"
        // 비어있으면 모든 IP 허용
        let admin_ip_allowlist = env::var("ADMIN_IP_ALLOWLIST")
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

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
            refresh_ttl_days_admin,
            refresh_ttl_days_hymn,
            refresh_cookie_name,
            refresh_cookie_domain,
            refresh_cookie_secure,
            refresh_cookie_samesite,
            rate_limit_login_window_sec,
            rate_limit_login_max,
            rate_limit_study_window_sec,
            rate_limit_study_max,
            cors_origins,
            vimeo_access_token,
            admin_ip_allowlist,
        }
    }

    pub fn refresh_cookie_samesite_or<'a>(&'a self, default: &'a str) -> &'a str {
        if self.refresh_cookie_samesite.is_empty() {
            default
        } else {
            &self.refresh_cookie_samesite
        }
    }

    /// 역할에 따른 Refresh Token TTL (days) 반환
    /// - HYMN: 1일 (기본)
    /// - Admin/Manager: 7일 (기본)
    /// - Learner: 30일 (기본)
    pub fn refresh_ttl_days_for_role(&self, role: &crate::types::UserAuth) -> i64 {
        match role {
            crate::types::UserAuth::Hymn => self.refresh_ttl_days_hymn,
            crate::types::UserAuth::Admin | crate::types::UserAuth::Manager => {
                self.refresh_ttl_days_admin
            }
            crate::types::UserAuth::Learner => self.refresh_ttl_days,
        }
    }

    /// Admin IP allowlist 확인
    /// - allowlist가 비어있으면 모든 IP 허용 (true)
    /// - allowlist에 IP가 있으면 해당 IP만 허용
    /// - CIDR 표기법 지원 (예: 192.168.1.0/24)
    pub fn is_admin_ip_allowed(&self, ip: &str) -> bool {
        // allowlist가 비어있으면 모든 IP 허용
        if self.admin_ip_allowlist.is_empty() {
            return true;
        }

        // IP 파싱
        let client_ip: std::net::IpAddr = match ip.parse() {
            Ok(ip) => ip,
            Err(_) => return false, // 파싱 실패하면 거부
        };

        for allowed in &self.admin_ip_allowlist {
            // CIDR 표기법 확인
            if allowed.contains('/') {
                if let Some((network, prefix)) = allowed.split_once('/') {
                    if let (Ok(network_ip), Ok(prefix_len)) =
                        (network.parse::<std::net::IpAddr>(), prefix.parse::<u8>())
                    {
                        if Self::ip_in_cidr(&client_ip, &network_ip, prefix_len) {
                            return true;
                        }
                    }
                }
            } else {
                // 단일 IP 비교
                if let Ok(allowed_ip) = allowed.parse::<std::net::IpAddr>() {
                    if client_ip == allowed_ip {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// CIDR 범위 내 IP 확인
    fn ip_in_cidr(ip: &std::net::IpAddr, network: &std::net::IpAddr, prefix_len: u8) -> bool {
        match (ip, network) {
            (std::net::IpAddr::V4(ip), std::net::IpAddr::V4(net)) => {
                if prefix_len > 32 {
                    return false; // Invalid prefix length for IPv4
                }
                let ip_bits = u32::from(*ip);
                let net_bits = u32::from(*net);
                let mask = if prefix_len == 0 {
                    0 // /0 means all IPs match
                } else if prefix_len == 32 {
                    u32::MAX
                } else {
                    u32::MAX << (32 - prefix_len)
                };
                (ip_bits & mask) == (net_bits & mask)
            }
            (std::net::IpAddr::V6(ip), std::net::IpAddr::V6(net)) => {
                if prefix_len > 128 {
                    return false; // Invalid prefix length for IPv6
                }
                let ip_bits = u128::from(*ip);
                let net_bits = u128::from(*net);
                let mask = if prefix_len == 0 {
                    0 // /0 means all IPs match
                } else if prefix_len == 128 {
                    u128::MAX
                } else {
                    u128::MAX << (128 - prefix_len)
                };
                (ip_bits & mask) == (net_bits & mask)
            }
            _ => false, // IPv4/IPv6 mismatch
        }
    }
}
