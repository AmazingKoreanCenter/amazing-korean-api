//! Phase 2/3 통합 테스트 공통 helper.
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
//! - Phase 2 = `EMAIL_PROVIDER=none` / `PAYMENT_PROVIDER=none` (mock 불필요, email 미사용 함수)
//! - Phase 3 = `make_test_state_with_capturing_email()` (CapturingEmailSender 주입, 발송 캡처)
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
use amazing_korean_api::error::{AppError, AppResult};
use amazing_korean_api::external::email::EmailSender;
use amazing_korean_api::external::ipgeo::IpGeoClient;
use amazing_korean_api::state::AppState;
use async_trait::async_trait;
use deadpool_redis::{Config as RedisConfig, Runtime};
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

/// 테스트용 AppState 생성. dotenv 자동 로드. `email = None`.
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

// =============================================================================
// EmailSender mocks — Phase 3 통합 테스트
// =============================================================================

/// 캡처된 이메일 발송 1건 (CapturingEmailSender 가 누적).
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct CapturedEmail {
    pub to: String,
    pub subject: String,
    pub html: String,
    pub text: String,
}

/// 발송 시도를 메모리에 누적하는 mock. `Ok(())` 만 반환 (성공 path 검증용).
///
/// 사용:
/// ```ignore
/// let (st, sent) = make_test_state_with_capturing_email().await;
/// // ... 서비스 호출 ...
/// let captured = sent.lock().await;
/// assert_eq!(captured.len(), 1);
/// assert!(captured[0].subject.contains("인증 코드"));
/// ```
pub struct CapturingEmailSender {
    pub sent: Arc<Mutex<Vec<CapturedEmail>>>,
}

#[async_trait]
impl EmailSender for CapturingEmailSender {
    async fn send_email(&self, to: &str, subject: &str, html: &str, text: &str) -> AppResult<()> {
        self.sent.lock().await.push(CapturedEmail {
            to: to.to_string(),
            subject: subject.to_string(),
            html: html.to_string(),
            text: text.to_string(),
        });
        Ok(())
    }
}

/// 항상 `External` 에러를 반환하는 mock. rate-limit DECR 롤백 path 검증용.
pub struct FailingEmailSender;

#[async_trait]
impl EmailSender for FailingEmailSender {
    async fn send_email(
        &self,
        _to: &str,
        _subject: &str,
        _html: &str,
        _text: &str,
    ) -> AppResult<()> {
        Err(AppError::External("test: forced email failure".to_string()))
    }
}

/// `make_test_state()` + CapturingEmailSender 주입. 캡처 핸들 함께 반환.
#[allow(dead_code)]
pub async fn make_test_state_with_capturing_email() -> (AppState, Arc<Mutex<Vec<CapturedEmail>>>) {
    let mut st = make_test_state().await;
    let sent = Arc::new(Mutex::new(Vec::new()));
    let sender = Arc::new(CapturingEmailSender { sent: sent.clone() });
    st.email = Some(sender);
    (st, sent)
}

/// `make_test_state()` + FailingEmailSender 주입. 에러 path 검증용.
#[allow(dead_code)]
pub async fn make_test_state_with_failing_email() -> AppState {
    let mut st = make_test_state().await;
    st.email = Some(Arc::new(FailingEmailSender));
    st
}

// =============================================================================
// 테스트용 user 생성/정리 — Phase 3 happy path + login/mfa
// =============================================================================

/// 테스트용 user 사양. `random()` 으로 충돌 방지된 기본값 생성, 필드 override 후 사용.
#[allow(dead_code)]
pub struct TestUserSpec {
    pub email: String,
    /// 평문. `insert_test_user` 가 argon2 해싱.
    pub password: String,
    pub name: String,
    pub nickname: String,
    pub country: String,
    /// `YYYY-MM-DD` 형식.
    pub birthday: String,
    /// `user_check_email` (true = 인증 완료). login 통과에 필요.
    pub check_email: bool,
    /// `user_state` (true = 활성).
    pub user_state: bool,
    /// `user_mfa_enabled`. login 시 MFA 챌린지 분기.
    pub mfa_enabled: bool,
    /// OAuth 전용 계정 만들 때 `true`. `password` 무시.
    pub oauth_only: bool,
}

#[allow(dead_code)]
impl TestUserSpec {
    /// UUID suffix 로 충돌 방지된 기본 spec.
    pub fn random() -> Self {
        let suffix = uuid::Uuid::new_v4();
        let short = suffix.to_string()[..8].to_string();
        Self {
            email: format!("phase3_{}@example.com", short),
            password: "TestPass123".to_string(),
            name: format!("phase3_{}", short),
            nickname: format!("nick_{}", short),
            country: "KR".to_string(),
            birthday: "1990-01-15".to_string(),
            check_email: true,
            user_state: true,
            mfa_enabled: false,
            oauth_only: false,
        }
    }
}

/// PII 암호화 + blind index + argon2 해시 후 `users` row INSERT. `user_id` 반환.
///
/// 다른 테이블 (users_setting/users_log/login 등) 은 INSERT 하지 않음 = `cleanup_test_user`
/// 가 단순 `DELETE FROM users` 만으로 충분 (login_log = ON DELETE SET NULL 자동 처리).
#[allow(dead_code)]
pub async fn insert_test_user(st: &AppState, spec: &TestUserSpec) -> i64 {
    use amazing_korean_api::crypto::CryptoService;
    let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
    let email_lower = spec.email.trim().to_lowercase();

    let email_enc = crypto
        .encrypt(&email_lower, "users.user_email")
        .expect("encrypt email");
    let email_idx = crypto.blind_index(&email_lower).expect("blind email");
    let name_enc = crypto
        .encrypt(&spec.name, "users.user_name")
        .expect("encrypt name");
    let name_idx = crypto.blind_index(&spec.name).expect("blind name");
    let birthday_enc = crypto
        .encrypt(&spec.birthday, "users.user_birthday")
        .expect("encrypt birthday");

    let password_hash: Option<String> = if spec.oauth_only {
        None
    } else {
        Some(
            amazing_korean_api::api::auth::password::hash_password(&spec.password)
                .expect("hash password"),
        )
    };

    sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO users (
            user_email, user_email_idx, user_password,
            user_name, user_name_idx, user_nickname,
            user_language, user_country, user_birthday, user_gender,
            user_check_email, user_terms_service, user_terms_personal,
            user_state, user_mfa_enabled
        )
        VALUES (
            $1, $2, $3,
            $4, $5, $6,
            'ko'::user_language_enum, $7, $8, 'none'::user_gender_enum,
            $9, true, true,
            $10, $11
        )
        RETURNING user_id
        "#,
    )
    .bind(email_enc)
    .bind(email_idx)
    .bind(password_hash)
    .bind(name_enc)
    .bind(name_idx)
    .bind(spec.nickname.clone())
    .bind(spec.country.clone())
    .bind(birthday_enc)
    .bind(spec.check_email)
    .bind(spec.user_state)
    .bind(spec.mfa_enabled)
    .fetch_one(&st.db)
    .await
    .expect("insert test user")
}

/// 테스트 종료 후 user row 삭제. `login_log` 의 user_id 는 ON DELETE SET NULL 자동 처리.
/// 우리 테스트는 successful login (login/redis_session/redis_refresh row INSERT) 까지 가지
/// 않으므로 단순 DELETE 가 충분.
#[allow(dead_code)]
pub async fn cleanup_test_user(st: &AppState, user_id: i64) {
    let _ = sqlx::query("DELETE FROM users WHERE user_id = $1")
        .bind(user_id)
        .execute(&st.db)
        .await;
}
