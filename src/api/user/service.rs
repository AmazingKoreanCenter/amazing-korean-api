use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHasher, Version,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::RngCore;
use redis::AsyncCommands;
use sha2::{Digest, Sha256};
use uuid::Uuid;
use validator::Validate;
use std::collections::HashSet;
use tracing::warn;

use crate::{
    api::auth::{jwt, repo::AuthRepo},
    error::{AppError, AppResult},
    state::AppState,
};
use super::{
    dto::{ProfileRes, ProfileUpdateReq, SettingsRes, SettingsUpdateReq, SignupReq, SignupRes},
    repo,
};

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

    fn generate_refresh_token() -> (String, String) {
        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        let token = URL_SAFE_NO_PAD.encode(bytes);
        let hash = URL_SAFE_NO_PAD.encode(Sha256::digest(bytes));
        (token, hash)
    }

    fn validate_password_policy(password: &str) -> bool {
        let has_letter = password.chars().any(|c| c.is_ascii_alphabetic());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        password.len() >= 8 && has_letter && has_digit
    }

    // =========================================================================
    // Main Business Logic
    // =========================================================================

    /// 회원가입 (RateLimit -> Validation -> DB Insert -> Token Issuance)
    pub async fn signup(
        st: &AppState,
        mut req: SignupReq,
        ip: String,
        ua: Option<String>,
    ) -> AppResult<(SignupRes, String, i64)> {
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

        // [Step 2] Rate Limiting (Redis)
        // 키 형식: rl:signup:{email}:{ip}
        let rl_key = format!("rl:signup:{}:{}", req.email, ip);
        let mut redis = st.redis.get().await.map_err(|e| AppError::Internal(e.to_string()))?;
        
        let attempts: i64 = redis.incr(&rl_key, 1).await?;
        if attempts == 1 {
            let _: () = redis.expire(&rl_key, st.cfg.rate_limit_login_window_sec).await?;
        }
        if attempts > st.cfg.rate_limit_login_max {
            return Err(AppError::TooManyRequests("Too many signup attempts".into()));
        }

        // [Step 3] Pre-check Email Duplication
        if repo::find_user_id_by_email(&st.db, &req.email).await?.is_some() {
            return Err(AppError::Conflict("Email already exists".into()));
        }

        // [Step 4] Password Hashing (Argon2)
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::new(19_456, 2, 1, None).unwrap());
        let password_hash = argon2.hash_password(req.password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(format!("Hash error: {e}")))?
            .to_string();

        // [Step 5] Prepare Session & Tokens
        let session_id = Uuid::new_v4().to_string();
        let (refresh_token, refresh_hash) = Self::generate_refresh_token();
        let refresh_ttl_secs = st.cfg.refresh_ttl_days * 24 * 3600;

        // [Step 6] DB Transaction (Insert User -> Log -> Login Record)
        let mut tx = st.db.begin().await?;

        // 6-1. Insert User
        let user = match repo::signup_tx(
            &mut tx, &req.email, &password_hash, &req.name, &req.nickname,
            &req.language, &req.country, req.birthday, req.gender,
            req.terms_service, req.terms_personal
        ).await {
            Ok(u) => u,
            Err(e) if Self::is_unique_violation(&e) => return Err(AppError::Conflict("Email exists".into())),
            Err(e) => return Err(e),
        };

        // 6-2. Audit Log (Signup)
        if let Err(e) = repo::insert_user_log_after_tx(&mut tx, Some(user.id), user.id, "signup", true).await {
            warn!(error = ?e, user_id = user.id, "Failed to insert signup log");
        }

        // 6-3. Auto Login Record
        AuthRepo::insert_login_record_tx(
            &mut tx, user.id, &session_id, &refresh_hash, &ip, None, None, None, ua.as_deref()
        ).await?;

        AuthRepo::insert_login_log_tx(
            &mut tx, user.id, "login", true, &session_id, &refresh_hash, &ip, None, None, None, ua.as_deref()
        ).await?;

        tx.commit().await?;

        // [Step 7] Redis Session Caching
        // 7-1. Session Mapping (SessionID -> UserID)
        let _: () = redis.set_ex(
            format!("ak:session:{}", session_id), 
            user.id, 
            st.cfg.jwt_access_ttl_min as u64 * 60
        ).await.map_err(|e| AppError::Internal(e.to_string()))?;

        // 7-2. Refresh Token Mapping (RefreshHash -> SessionID)
        let _: () = redis.set_ex(
            format!("ak:refresh:{}", refresh_hash), 
            &session_id, 
            refresh_ttl_secs as u64
        ).await.map_err(|e| AppError::Internal(e.to_string()))?;

        // 7-3. User Sessions Set (UserID -> Set<SessionID>) - For 'Logout All'
        let _: () = redis.sadd(
            format!("ak:user_sessions:{}", user.id), 
            &session_id
        ).await.map_err(|e| AppError::Internal(e.to_string()))?;

        // [Step 8] JWT Access Token Generation
        let access_token = jwt::create_token(
            user.id, &session_id, st.cfg.jwt_access_ttl_min, &st.cfg.jwt_secret
        )?;

        // [Step 9] Response Construction
        let res = SignupRes {
            user_id: user.id,
            email: user.email.clone(),
            name: user.name.clone(),
            nickname: user.nickname.clone().unwrap_or_default(),
            language: user.language.clone().unwrap_or_else(|| "ko".to_string()),
            country: user.country.clone().unwrap_or_default(),
            birthday: user.birthday.unwrap_or_else(|| chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
            gender: user.gender,
            user_state: user.user_state,
            user_auth: user.user_auth,
            created_at: user.created_at,
            access: access_token,
            session_id,
        };

        Ok((res, refresh_token, refresh_ttl_secs))
    }

    pub async fn get_me(st: &AppState, user_id: i64) -> AppResult<ProfileRes> {
        let user = repo::find_profile_by_id(&st.db, user_id).await?.ok_or(AppError::NotFound)?;
        if !user.user_state { return Err(AppError::NotFound); }
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

        let mut tx = st.db.begin().await?;
        
        let updated = repo::update_profile_tx(&mut tx, user_id, &req).await?.ok_or(AppError::NotFound)?;
        repo::insert_user_log_after_tx(&mut tx, Some(user_id), user_id, "update", true).await?;
        
        tx.commit().await?;

        Ok(updated)
    }

    pub async fn get_settings(st: &AppState, user_id: i64) -> AppResult<SettingsRes> {
        let user = repo::find_user(&st.db, user_id).await?.ok_or(AppError::NotFound)?;
        if !user.user_state { return Err(AppError::Forbidden); }

        let settings = repo::find_users_setting(&st.db, user_id).await?;
        
        Ok(settings.unwrap_or_else(|| SettingsRes {
            user_set_language: "ko".to_string(),
            user_set_timezone: "UTC".to_string(),
            user_set_note_email: false,
            user_set_note_push: false,
            updated_at: chrono::Utc::now(),
        }))
    }

    pub async fn update_settings(st: &AppState, user_id: i64, req: SettingsUpdateReq) -> AppResult<SettingsRes> {
        req.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;

        let user = repo::find_user(&st.db, user_id).await?.ok_or(AppError::NotFound)?;
        if !user.user_state { return Err(AppError::Forbidden); }

        if let Some(lang) = &req.user_set_language {
            if !["en", "ko"].contains(&lang.as_str()) {
                return Err(AppError::BadRequest("Invalid language".into()));
            }
        }

        let mut tx = st.db.begin().await?;
        
        let settings = repo::upsert_settings_tx(&mut tx, user_id, &req).await?;
        repo::insert_user_log_after_tx(&mut tx, Some(user_id), user_id, "update", true).await?;
        
        tx.commit().await?;

        Ok(settings)
    }
}