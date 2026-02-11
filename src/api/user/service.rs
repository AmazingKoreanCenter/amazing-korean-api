use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHasher, Version,
};
use hmac::{Hmac, Mac};
use redis::AsyncCommands;
use sha2::Sha256;
use validator::Validate;
use std::collections::HashSet;
use tracing::{info, warn};

use crate::{
    api::auth::service::AuthService,
    crypto::CryptoService,
    error::{AppError, AppResult},
    external::email::{self, EmailTemplate},
    state::AppState,
};
use super::{
    dto::{ProfileRes, ProfileUpdateReq, SettingsRes, SettingsUpdateReq, SignupReq, SignupRes},
    repo,
};

type HmacSha256 = Hmac<Sha256>;

pub struct UserService;

impl UserService {
    const PG_UNIQUE_VIOLATION: &'static str = "23505";

    // =========================================================================
    // Helper Functions (Private)
    // =========================================================================

    #[inline]
    fn is_unique_violation(err: &AppError) -> bool {
        if let AppError::Sqlx(sqlx::Error::Database(db)) = err {
            db.code().as_deref() == Some(Self::PG_UNIQUE_VIOLATION)
        } else {
            false
        }
    }

    fn validate_password_policy(password: &str) -> bool {
        let has_letter = password.chars().any(|c| c.is_ascii_alphabetic());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        password.len() >= 8 && has_letter && has_digit
    }

    /// 인증 코드를 HMAC-SHA256 해시 (Redis에 평문 저장 금지)
    pub fn hmac_verification_code(hmac_key: &[u8; 32], email: &str, code: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(hmac_key)
            .expect("HMAC key length is always 32");
        mac.update(format!("{}:{}", email, code).as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }

    // =========================================================================
    // Main Business Logic
    // =========================================================================

    /// 회원가입 (RateLimit -> Validation -> DB Insert -> 인증코드 발송)
    pub async fn signup(
        st: &AppState,
        mut req: SignupReq,
        ip: String,
    ) -> AppResult<SignupRes> {
        // [Step 1] Input Validation
        req.email = req.email.trim().to_lowercase();

        // 1-1. Basic Validation (Format)
        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(e.to_string()));
        }

        // 1-2. Business Validation (Terms, Password, Birthday, Language)
        if !req.terms_service || !req.terms_personal {
            return Err(AppError::BadRequest("Terms must be accepted".into()));
        }
        if !Self::validate_password_policy(&req.password) {
            return Err(AppError::Unprocessable("Weak password: need 8+ chars, letter & digit".into()));
        }

        let today = chrono::Utc::now().date_naive();
        let min_birth = chrono::NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();
        if req.birthday < min_birth || req.birthday > today {
            return Err(AppError::Unprocessable("Invalid birthday".into()));
        }

        let allowed_langs: HashSet<&str> = ["ko", "en"].into();
        let lang = req.language.to_lowercase();
        if !allowed_langs.contains(lang.as_str()) {
            return Err(AppError::Unprocessable("Unsupported language".into()));
        }
        req.language = lang;

        // [Step 2] Rate Limiting (Redis) — blind index로 키 생성 (이메일 PII 노출 방지)
        let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
        let email_idx = crypto.blind_index(&req.email)?;
        let rl_key = format!("rl:signup:{}:{}", email_idx, ip);
        let mut redis = st.redis.get().await.map_err(|e| AppError::Internal(e.to_string()))?;

        let attempts: i64 = redis.incr(&rl_key, 1).await?;
        if attempts == 1 {
            let _: () = redis.expire(&rl_key, st.cfg.rate_limit_login_window_sec).await?;
        }
        if attempts > st.cfg.rate_limit_login_max {
            return Err(AppError::TooManyRequests("Too many signup attempts".into()));
        }

        // Field Encryption
        let email_enc = crypto.encrypt(&req.email, "users.user_email")?;
        let name_enc = crypto.encrypt(&req.name, "users.user_name")?;
        let name_idx = crypto.blind_index(&req.name)?;
        let birthday_enc = crypto.encrypt(&req.birthday.to_string(), "users.user_birthday")?;

        // [Step 3] Pre-check Email Duplication (+ 미인증 재가입 처리)
        let existing = repo::find_user_id_and_check_email_by_email_idx(&st.db, &email_idx).await?;

        if let Some((existing_id, check_email)) = existing {
            if check_email {
                // 이미 인증 완료된 계정 → 409
                return Err(AppError::Conflict("Email already exists".into()));
            }

            // 미인증 계정 → 비밀번호/프로필 덮어쓰기 + 새 인증코드 발송
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::new(19_456, 2, 1, None).unwrap());
            let password_hash = argon2.hash_password(req.password.as_bytes(), &salt)
                .map_err(|e| AppError::Internal(format!("Hash error: {e}")))?
                .to_string();

            repo::overwrite_unverified_user(
                &st.db, existing_id, &email_enc, &password_hash,
                &name_enc, &req.nickname, &req.language, &req.country,
                &birthday_enc, req.gender, &name_idx,
            ).await?;

            info!(user_id = existing_id, "Overwritten unverified user data");

            // 인증코드 발송
            return Self::send_verification_code(st, &req.email, &email_idx, &mut redis).await;
        }

        // [Step 4] Password Hashing (Argon2)
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::new(19_456, 2, 1, None).unwrap());
        let password_hash = argon2.hash_password(req.password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(format!("Hash error: {e}")))?
            .to_string();

        // [Step 5] DB Transaction (Insert User)
        let mut tx = st.db.begin().await?;

        let user = match repo::signup_tx(
            &mut tx, &email_enc, &password_hash, &name_enc, &req.nickname,
            &req.language, &req.country, &birthday_enc, req.gender,
            req.terms_service, req.terms_personal,
            &email_idx, &name_idx,
        ).await {
            Ok(u) => u,
            Err(e) if Self::is_unique_violation(&e) => return Err(AppError::Conflict("Email exists".into())),
            Err(e) => return Err(e),
        };

        // Audit Log (Signup)
        if let Err(e) = repo::insert_user_log_after_tx(&mut tx, &crypto, Some(user.id), user.id, "signup", true).await {
            warn!(error = ?e, user_id = user.id, "Failed to insert signup log");
        }

        tx.commit().await?;

        // [Step 6] 이메일 인증코드 발송
        // 개발 환경에서 EMAIL_PROVIDER=none이면 자동 인증
        if st.cfg.email_provider == "none" {
            warn!("EMAIL_PROVIDER=none: auto-verifying email for user {}", user.id);
            crate::api::auth::repo::AuthRepo::update_user_check_email(&st.db, user.id, true).await?;
            return Ok(SignupRes {
                message: "Account created (email auto-verified in development mode).".to_string(),
                requires_verification: false,
            });
        }

        Self::send_verification_code(st, &req.email, &email_idx, &mut redis).await
    }

    /// 인증코드 생성 → HMAC 해시 → Redis 저장 → 이메일 발송
    async fn send_verification_code(
        st: &AppState,
        email_plain: &str,
        email_idx: &str,
        redis: &mut deadpool_redis::Connection,
    ) -> AppResult<SignupRes> {
        let code = AuthService::generate_verification_code();
        let code_hash = Self::hmac_verification_code(&st.cfg.hmac_key, email_plain, &code);
        let ttl_sec = st.cfg.verification_code_ttl_sec;

        // Redis에 HMAC 해시 저장 (blind index 키)
        let redis_key = format!("ak:email_verify:{}", email_idx);
        let _: () = redis.set_ex(&redis_key, &code_hash, ttl_sec as u64)
            .await.map_err(|e| AppError::Internal(e.to_string()))?;

        // 이메일 발송
        let email_sender = st.email.as_ref()
            .ok_or_else(|| AppError::ServiceUnavailable("Email service not configured".into()))?;

        let expires_in_min = (ttl_sec / 60) as i32;
        email::send_templated(
            email_sender.as_ref(),
            email_plain,
            EmailTemplate::EmailVerification { code, expires_in_min },
        ).await?;

        info!(email_idx = %email_idx, "Verification code sent");

        Ok(SignupRes {
            message: "Verification code sent to your email.".to_string(),
            requires_verification: true,
        })
    }

    pub async fn get_me(st: &AppState, user_id: i64) -> AppResult<ProfileRes> {
        let mut user = repo::find_profile_by_id(&st.db, user_id).await?.ok_or(AppError::NotFound)?;
        if !user.user_state { return Err(AppError::NotFound); }

        // Decrypt encrypted fields
        let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
        user.email = crypto.decrypt(&user.email, "users.user_email")?;
        user.name = crypto.decrypt(&user.name, "users.user_name")?;
        if let Some(ref bday) = user.birthday {
            user.birthday = Some(crypto.decrypt(bday, "users.user_birthday")?);
        }

        Ok(user)
    }

    pub async fn update_me(st: &AppState, user_id: i64, req: ProfileUpdateReq) -> AppResult<ProfileRes> {
        req.validate().map_err(|e| AppError::Unprocessable(e.to_string()))?;

        if let Some(birthday) = req.birthday {
            if birthday > chrono::Utc::now().date_naive() {
                return Err(AppError::Unprocessable("Invalid birthday".into()));
            }
        }

        let user = repo::find_profile_by_id(&st.db, user_id).await?.ok_or(AppError::NotFound)?;
        if !user.user_state { return Err(AppError::NotFound); }

        // Encrypt birthday if updated
        let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
        let birthday_enc = req.birthday
            .map(|b| crypto.encrypt(&b.to_string(), "users.user_birthday"))
            .transpose()?;

        let mut tx = st.db.begin().await?;

        let mut updated = repo::update_profile_tx(&mut tx, user_id, &req, birthday_enc.as_deref()).await?.ok_or(AppError::NotFound)?;
        repo::insert_user_log_after_tx(&mut tx, &crypto, Some(user_id), user_id, "update", true).await?;

        tx.commit().await?;

        // Decrypt encrypted fields for response
        updated.email = crypto.decrypt(&updated.email, "users.user_email")?;
        updated.name = crypto.decrypt(&updated.name, "users.user_name")?;
        if let Some(ref bday) = updated.birthday {
            updated.birthday = Some(crypto.decrypt(bday, "users.user_birthday")?);
        }

        Ok(updated)
    }

    pub async fn get_settings(st: &AppState, user_id: i64) -> AppResult<SettingsRes> {
        let user = repo::find_user(&st.db, user_id).await?.ok_or(AppError::NotFound)?;
        if !user.user_state { return Err(AppError::Forbidden("Forbidden".to_string())); }

        let settings = repo::find_users_setting(&st.db, user_id).await?;

        let mut result = settings.unwrap_or_else(|| SettingsRes {
            user_set_language: "ko".to_string(),
            user_set_timezone: "UTC".to_string(),
            user_set_note_email: false,
            user_set_note_push: false,
            updated_at: chrono::Utc::now(),
        });
        // DB enum("zh_cn") → 프론트엔드("zh-CN") 변환
        result.user_set_language = crate::types::UserSetLanguage::db_to_frontend(&result.user_set_language);
        Ok(result)
    }

    pub async fn update_settings(st: &AppState, user_id: i64, mut req: SettingsUpdateReq) -> AppResult<SettingsRes> {
        req.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;

        let user = repo::find_user(&st.db, user_id).await?.ok_or(AppError::NotFound)?;
        if !user.user_state { return Err(AppError::Forbidden("Forbidden".to_string())); }

        if let Some(lang) = &mut req.user_set_language {
            serde_json::from_value::<crate::types::UserSetLanguage>(
                serde_json::Value::String(lang.clone()),
            )
            .map_err(|_| AppError::BadRequest("Invalid language".into()))?;
            // 프론트엔드("zh-CN") → DB enum("zh_cn") 변환
            *lang = crate::types::UserSetLanguage::frontend_to_db(lang);
        }

        let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
        let mut tx = st.db.begin().await?;

        let settings = repo::upsert_settings_tx(&mut tx, user_id, &req).await?;
        repo::insert_user_log_after_tx(&mut tx, &crypto, Some(user_id), user_id, "update", true).await?;

        tx.commit().await?;

        // DB enum("zh_cn") → 프론트엔드("zh-CN") 변환
        let mut result = settings;
        result.user_set_language = crate::types::UserSetLanguage::db_to_frontend(&result.user_set_language);
        Ok(result)
    }
}
