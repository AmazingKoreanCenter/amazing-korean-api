use std::collections::HashMap;
use std::env;
use std::fmt;

use crate::crypto::KeyRing;

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
    pub rate_limit_email_window_sec: i64,  // 이메일 발송 레이트리밋 윈도우 (초, 기본: 18000 = 5시간)
    pub rate_limit_email_max: i64,         // 이메일 발송 최대 횟수/윈도우 (기본: 5)
    pub cors_origins: Vec<String>,
    pub vimeo_access_token: Option<String>,
    pub admin_ip_allowlist: Vec<String>,  // Admin 접근 허용 IP 목록 (비어있으면 모든 IP 허용)
    // Google OAuth
    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,
    pub google_redirect_uri: Option<String>,
    pub oauth_state_ttl_sec: i64,         // OAuth state 유효시간 (초, 기본 300)
    pub frontend_url: String,             // OAuth 콜백 후 리다이렉트할 프론트엔드 URL
    // Email Provider
    pub email_provider: String,           // "resend" | "none" (기본: "none")
    pub resend_api_key: Option<String>,   // Resend API 키 (email_provider=resend 시 필수)
    pub email_from_address: Option<String>, // 발신자 이메일 (noreply@amazingkorean.net)
    // Password Reset (비밀번호 재설정)
    pub verification_code_ttl_sec: i64,   // 인증코드 유효시간 (초, 기본 600 = 10분)
    pub reset_token_ttl_sec: i64,         // reset_token 유효시간 (초, 기본 1800 = 30분)
    // Translation Provider
    pub translate_provider: String,              // "google" | "none" (기본: "none")
    pub google_translate_api_key: Option<String>,  // GCP Translation API 키
    pub google_translate_project_id: Option<String>, // GCP 프로젝트 ID
    // MFA (Multi-Factor Authentication)
    pub mfa_token_ttl_sec: i64,                  // MFA 토큰 유효시간 (초, 기본: 300 = 5분)
    pub rate_limit_mfa_max: i64,                 // MFA 코드 검증 최대 시도 횟수 (기본: 5)
    pub rate_limit_mfa_window_sec: i64,          // MFA 코드 검증 레이트리밋 윈도우 (초, 기본: 300)
    // Field Encryption (AES-256-GCM + HMAC-SHA256 Blind Index)
    pub app_env: String,                         // "production" | "development" (기본)
    pub encryption_ring: KeyRing,                // 다중 키 버전 (ENCRYPTION_KEY_V{n})
    pub hmac_key: [u8; 32],                      // HMAC-SHA256 키 (blind index용, 필수)
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

        // Email Rate Limit: 이메일 발송(인증코드 등) 과도한 요청 방지
        let rate_limit_email_window_sec = env::var("RATE_LIMIT_EMAIL_WINDOW_SEC")
            .unwrap_or_else(|_| "18000".into())
            .parse::<i64>()
            .expect("RATE_LIMIT_EMAIL_WINDOW_SEC must be a number");
        let rate_limit_email_max = env::var("RATE_LIMIT_EMAIL_MAX")
            .unwrap_or_else(|_| "5".into())
            .parse::<i64>()
            .expect("RATE_LIMIT_EMAIL_MAX must be a number");
        if rate_limit_email_max < 1 {
            panic!("RATE_LIMIT_EMAIL_MAX must be >= 1, got {}", rate_limit_email_max);
        }

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

        // Google OAuth (optional)
        let google_client_id = env::var("GOOGLE_CLIENT_ID").ok();
        let google_client_secret = env::var("GOOGLE_CLIENT_SECRET").ok();
        let google_redirect_uri = env::var("GOOGLE_REDIRECT_URI").ok();
        let oauth_state_ttl_sec = env::var("OAUTH_STATE_TTL_SEC")
            .unwrap_or_else(|_| "300".into())
            .parse::<i64>()
            .expect("OAUTH_STATE_TTL_SEC must be a number");
        let frontend_url = env::var("FRONTEND_URL")
            .unwrap_or_else(|_| "http://localhost:5173".into());

        // Email Provider: "resend" | "none"
        let email_provider = env::var("EMAIL_PROVIDER")
            .unwrap_or_else(|_| "none".into())
            .to_lowercase();
        let resend_api_key = env::var("RESEND_API_KEY")
            .ok()
            .filter(|s| !s.is_empty());
        let email_from_address = env::var("EMAIL_FROM_ADDRESS")
            .ok()
            .filter(|s| !s.is_empty());

        // Password Reset
        let verification_code_ttl_sec = env::var("VERIFICATION_CODE_TTL_SEC")
            .unwrap_or_else(|_| "600".into())
            .parse::<i64>()
            .expect("VERIFICATION_CODE_TTL_SEC must be a number");
        let reset_token_ttl_sec = env::var("RESET_TOKEN_TTL_SEC")
            .unwrap_or_else(|_| "1800".into())
            .parse::<i64>()
            .expect("RESET_TOKEN_TTL_SEC must be a number");

        // Translation Provider: "google" | "none"
        let translate_provider = env::var("TRANSLATE_PROVIDER")
            .unwrap_or_else(|_| "none".into())
            .to_lowercase();
        let google_translate_api_key = env::var("GOOGLE_TRANSLATE_API_KEY")
            .ok()
            .filter(|s| !s.is_empty());
        let google_translate_project_id = env::var("GOOGLE_TRANSLATE_PROJECT_ID")
            .ok()
            .filter(|s| !s.is_empty());

        // Translation provider 검증 (google 선택 시 필수 값 확인)
        if translate_provider == "google" {
            if google_translate_api_key.is_none() {
                panic!("GOOGLE_TRANSLATE_API_KEY must be set when TRANSLATE_PROVIDER=google");
            }
            if google_translate_project_id.is_none() {
                panic!("GOOGLE_TRANSLATE_PROJECT_ID must be set when TRANSLATE_PROVIDER=google");
            }
        } else if translate_provider != "none" {
            panic!(
                "Unknown TRANSLATE_PROVIDER '{}'. Must be 'google' or 'none'.",
                translate_provider
            );
        }

        // MFA (Multi-Factor Authentication)
        let mfa_token_ttl_sec = env::var("MFA_TOKEN_TTL_SEC")
            .unwrap_or_else(|_| "300".into())
            .parse::<i64>()
            .expect("MFA_TOKEN_TTL_SEC must be a number");
        let rate_limit_mfa_max = env::var("RATE_LIMIT_MFA_MAX")
            .unwrap_or_else(|_| "5".into())
            .parse::<i64>()
            .expect("RATE_LIMIT_MFA_MAX must be a number");
        let rate_limit_mfa_window_sec = env::var("RATE_LIMIT_MFA_WINDOW_SEC")
            .unwrap_or_else(|_| "300".into())
            .parse::<i64>()
            .expect("RATE_LIMIT_MFA_WINDOW_SEC must be a number");

        // Field Encryption (AES-256-GCM + HMAC-SHA256)
        let app_env = env::var("APP_ENV").unwrap_or_else(|_| "development".into());

        // Production fail-fast: 이메일 서비스 미설정 시 서버 부팅 실패
        if app_env == "production" {
            match email_provider.as_str() {
                "none" => panic!(
                    "EMAIL_PROVIDER=none is not allowed in production. Set to 'resend'."
                ),
                "resend" => {
                    if resend_api_key.is_none() {
                        panic!("RESEND_API_KEY must be set when EMAIL_PROVIDER=resend");
                    }
                    if email_from_address.is_none() {
                        panic!("EMAIL_FROM_ADDRESS must be set when EMAIL_PROVIDER=resend");
                    }
                }
                other => panic!(
                    "Unknown EMAIL_PROVIDER '{}'. Must be 'resend' or 'none'.", other
                ),
            }
        }

        // HMAC 키 (필수, 로테이션 안 함)
        let hmac_key = {
            let b64 = env::var("HMAC_KEY")
                .expect("HMAC_KEY must be set (base64-encoded 32-byte key)");
            if b64.is_empty() {
                panic!("HMAC_KEY must not be empty");
            }
            use base64::engine::{general_purpose::STANDARD, Engine};
            let decoded = STANDARD.decode(&b64)
                .unwrap_or_else(|e| panic!("HMAC_KEY must be valid base64: {e}"));
            <[u8; 32]>::try_from(decoded.as_slice())
                .unwrap_or_else(|_| panic!(
                    "HMAC_KEY must be exactly 32 bytes (got {})", decoded.len()
                ))
        };

        // 암호화 키 로딩: ENCRYPTION_KEY_V{n} 패턴 (n = 1~255)
        // 하위 호환: ENCRYPTION_KEY (레거시) → v1으로 매핑
        let encryption_ring = {
            use base64::engine::{general_purpose::STANDARD, Engine};

            let mut keys: HashMap<u8, [u8; 32]> = HashMap::new();

            // ENCRYPTION_KEY_V1 ~ V255 순회
            for ver in 1..=255u8 {
                let env_name = format!("ENCRYPTION_KEY_V{}", ver);
                if let Ok(b64) = env::var(&env_name) {
                    if b64.is_empty() { continue; }
                    let decoded = STANDARD.decode(&b64)
                        .unwrap_or_else(|e| panic!("{env_name} must be valid base64: {e}"));
                    let key = <[u8; 32]>::try_from(decoded.as_slice())
                        .unwrap_or_else(|_| panic!(
                            "{env_name} must be exactly 32 bytes (got {})", decoded.len()
                        ));
                    keys.insert(ver, key);
                }
            }

            // 하위 호환: ENCRYPTION_KEY (레거시) → v1으로 매핑 (V1 미설정 시에만)
            if !keys.contains_key(&1) {
                if let Ok(b64) = env::var("ENCRYPTION_KEY") {
                    if !b64.is_empty() {
                        let decoded = STANDARD.decode(&b64)
                            .unwrap_or_else(|e| panic!("ENCRYPTION_KEY must be valid base64: {e}"));
                        let key = <[u8; 32]>::try_from(decoded.as_slice())
                            .unwrap_or_else(|_| panic!(
                                "ENCRYPTION_KEY must be exactly 32 bytes (got {})", decoded.len()
                            ));
                        keys.insert(1, key);
                    }
                }
            }

            if keys.is_empty() {
                panic!("At least one encryption key must be set (ENCRYPTION_KEY_V1 or ENCRYPTION_KEY)");
            }

            let current_version = env::var("ENCRYPTION_CURRENT_VERSION")
                .unwrap_or_else(|_| "1".into())
                .parse::<u8>()
                .expect("ENCRYPTION_CURRENT_VERSION must be a number (1-255)");

            KeyRing::new(keys, current_version)
                .unwrap_or_else(|e| panic!("Failed to create KeyRing: {e}"))
        };

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
            rate_limit_email_window_sec,
            rate_limit_email_max,
            cors_origins,
            vimeo_access_token,
            admin_ip_allowlist,
            google_client_id,
            google_client_secret,
            google_redirect_uri,
            oauth_state_ttl_sec,
            frontend_url,
            email_provider,
            resend_api_key,
            email_from_address,
            verification_code_ttl_sec,
            reset_token_ttl_sec,
            translate_provider,
            google_translate_api_key,
            google_translate_project_id,
            mfa_token_ttl_sec,
            rate_limit_mfa_max,
            rate_limit_mfa_window_sec,
            app_env,
            encryption_ring,
            hmac_key,
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

// 민감 정보 마스킹을 위한 수동 Debug 구현
impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("database_url", &"***")
            .field("bind_addr", &self.bind_addr)
            .field("redis_url", &"***")
            .field("jwt_secret", &"***")
            .field("jwt_expire_hours", &self.jwt_expire_hours)
            .field("enable_docs", &self.enable_docs)
            .field("skip_db", &self.skip_db)
            .field("jwt_access_ttl_min", &self.jwt_access_ttl_min)
            .field("refresh_ttl_days", &self.refresh_ttl_days)
            .field("refresh_ttl_days_admin", &self.refresh_ttl_days_admin)
            .field("refresh_ttl_days_hymn", &self.refresh_ttl_days_hymn)
            .field("refresh_cookie_name", &self.refresh_cookie_name)
            .field("refresh_cookie_domain", &self.refresh_cookie_domain)
            .field("refresh_cookie_secure", &self.refresh_cookie_secure)
            .field("refresh_cookie_samesite", &self.refresh_cookie_samesite)
            .field("rate_limit_login_window_sec", &self.rate_limit_login_window_sec)
            .field("rate_limit_login_max", &self.rate_limit_login_max)
            .field("rate_limit_study_window_sec", &self.rate_limit_study_window_sec)
            .field("rate_limit_study_max", &self.rate_limit_study_max)
            .field("rate_limit_email_window_sec", &self.rate_limit_email_window_sec)
            .field("rate_limit_email_max", &self.rate_limit_email_max)
            .field("cors_origins", &self.cors_origins)
            .field("vimeo_access_token", &self.vimeo_access_token.as_ref().map(|_| "***"))
            .field("admin_ip_allowlist", &self.admin_ip_allowlist)
            .field("google_client_id", &self.google_client_id.as_ref().map(|_| "***"))
            .field("google_client_secret", &self.google_client_secret.as_ref().map(|_| "***"))
            .field("google_redirect_uri", &self.google_redirect_uri)
            .field("oauth_state_ttl_sec", &self.oauth_state_ttl_sec)
            .field("frontend_url", &self.frontend_url)
            .field("email_provider", &self.email_provider)
            .field("resend_api_key", &self.resend_api_key.as_ref().map(|_| "***"))
            .field("email_from_address", &self.email_from_address)
            .field("verification_code_ttl_sec", &self.verification_code_ttl_sec)
            .field("reset_token_ttl_sec", &self.reset_token_ttl_sec)
            .field("translate_provider", &self.translate_provider)
            .field("google_translate_api_key", &self.google_translate_api_key.as_ref().map(|_| "***"))
            .field("google_translate_project_id", &self.google_translate_project_id)
            .field("mfa_token_ttl_sec", &self.mfa_token_ttl_sec)
            .field("rate_limit_mfa_max", &self.rate_limit_mfa_max)
            .field("rate_limit_mfa_window_sec", &self.rate_limit_mfa_window_sec)
            .field("app_env", &self.app_env)
            .field("encryption_ring", &self.encryption_ring)
            .field("hmac_key", &"***")
            .finish()
    }
}
