use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHasher, Version,
};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;
use rand::RngCore;
use redis::AsyncCommands;
use sha2::{Digest, Sha256};
use uuid::Uuid;
use validator::Validate;

use crate::{
    api::auth::{jwt, repo::AuthRepo},
    error::{AppError, AppResult},
    state::AppState,
};

use super::{
    dto::{ProfileRes, SettingsRes, SettingsUpdateReq, SignupReq, SignupRes, UpdateReq},
    repo,
};
use std::collections::HashSet;

use tracing::warn;

pub struct UserService;

impl UserService {
    const PG_UNIQUE_VIOLATION: &'static str = "23505";

    #[inline]
    fn is_unique_violation(err: &AppError) -> bool {
        if let AppError::Sqlx(sqlx::Error::Database(db)) = err {
            db.code().as_deref() == Some(Self::PG_UNIQUE_VIOLATION)
        } else {
            false
        }
    }

    fn generate_refresh_token_and_hash() -> (String, String) {
        let mut refresh_bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut refresh_bytes);
        let refresh_token = URL_SAFE_NO_PAD.encode(refresh_bytes);
        let refresh_hash = URL_SAFE_NO_PAD.encode(Sha256::digest(refresh_bytes));
        (refresh_token, refresh_hash)
    }

    fn validate_password_policy(password: &str) -> bool {
        let has_letter = password.chars().any(|c| c.is_ascii_alphabetic());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        password.len() >= 8 && has_letter && has_digit
    }

    pub async fn signup(
        st: &AppState,
        mut req: SignupReq,
        signup_ip: String,
        user_agent: Option<String>,
    ) -> AppResult<(SignupRes, String, i64)> {
        req.email = req.email.trim().to_lowercase();

        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(e.to_string()));
        }

        if !req.terms_service || !req.terms_personal {
            return Err(AppError::BadRequest("terms must be accepted".into()));
        }

        if !Self::validate_password_policy(&req.password) {
            return Err(AppError::Unprocessable(
                "password policy violation".into(),
            ));
        }

        let today = chrono::Utc::now().date_naive();
        let min_birth = chrono::NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();
        if req.birthday < min_birth || req.birthday > today {
            return Err(AppError::Unprocessable("invalid birthday".into()));
        }

        let allowed_langs: HashSet<&str> = ["ko", "en"].iter().cloned().collect();
        let lang = req.language.to_lowercase();
        if !allowed_langs.contains(lang.as_str()) {
            return Err(AppError::Unprocessable("invalid language".into()));
        }
        req.language = lang;

        let rl_key = format!("rl:signup:{}:{}", req.email, signup_ip);
        let mut redis_conn = st
            .redis
            .get()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let attempts: i64 = redis_conn.incr(&rl_key, 1).await?;
        if attempts == 1 {
            let _: () = redis_conn
                .expire(&rl_key, st.cfg.rate_limit_login_window_sec)
                .await?;
        }
        if attempts > st.cfg.rate_limit_login_max {
            return Err(AppError::TooManyRequests(
                "AUTH_429_TOO_MANY_ATTEMPTS".into(),
            ));
        }

        if repo::find_user_id_by_email(&st.db, &req.email)
            .await?
            .is_some()
        {
            return Err(AppError::Conflict("email already exists".into()));
        }

        let salt = SaltString::generate(&mut OsRng);
        let params = Params::new(19_456, 2, 1, None).unwrap();
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

        let password_hash = argon2
            .hash_password(req.password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(format!("password hash error: {e}")))?
            .to_string();

        let session_id = Uuid::new_v4().to_string();
        let (refresh_token_value, refresh_hash) = Self::generate_refresh_token_and_hash();
        let refresh_ttl_secs = st.cfg.refresh_ttl_days * 24 * 3600;

        let mut tx = st.db.begin().await?;

        let user = repo::signup_tx(
            &mut tx,
            &req.email,
            &password_hash,
            &req.name,
            &req.nickname,
            &req.language,
            &req.country,
            req.birthday,
            req.gender,
            req.terms_service,
            req.terms_personal,
        )
        .await;

        let user = match user {
            Ok(user) => user,
            Err(e) if Self::is_unique_violation(&e) => {
                return Err(AppError::Conflict("email already exists".into()));
            }
            Err(e) => return Err(e),
        };

        if let Err(le) = repo::insert_user_log_after_tx(
            &mut tx,
            Some(user.id),
            user.id,
            "signup",
            true,
        )
        .await
        {
            warn!(error=?le, user_id = user.id, "users_log(signup) insert failed");
        }

        AuthRepo::insert_login_record_tx(
            &mut tx,
            user.id,
            &session_id,
            &refresh_hash,
            &signup_ip,
            None,
            None,
            None,
            user_agent.as_deref(),
        )
        .await?;

        AuthRepo::insert_login_log_tx(
            &mut tx,
            user.id,
            "login",
            true,
            &session_id,
            &refresh_hash,
            &signup_ip,
            None,
            None,
            None,
            user_agent.as_deref(),
        )
        .await?;

        tx.commit().await?;

        let _: () = redis_conn
            .set_ex(
                format!("ak:session:{}", session_id),
                user.id,
                st.cfg.jwt_access_ttl_min as u64 * 60,
            )
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let _: () = redis_conn
            .set_ex(
                format!("ak:refresh:{}", refresh_hash),
                &session_id,
                refresh_ttl_secs as u64,
            )
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let _: () = redis_conn
            .sadd(format!("ak:user_sessions:{}", user.id), &session_id)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let access = jwt::create_token(user.id, st.cfg.jwt_access_ttl_min, &st.cfg.jwt_secret)?;

        let res = SignupRes {
            user_id: user.id,
            email: user.email.clone(),
            name: user.name.clone(),
            nickname: user.nickname.clone().unwrap_or_default(),
            language: user.language.clone().unwrap_or_else(|| "ko".to_string()),
            country: user.country.clone().unwrap_or_default(),
            birthday: user
                .birthday
                .unwrap_or_else(|| chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
            gender: user.gender,
            user_state: user.user_state,
            user_auth: user.user_auth,
            created_at: user.created_at,
            access,
            session_id: session_id.clone(),
        };

        Ok((res, refresh_token_value, refresh_ttl_secs))
    }

    pub async fn get_me(st: &AppState, user_id: i64) -> AppResult<ProfileRes> {
        let user = repo::find_profile_by_id(&st.db, user_id)
            .await?
            .ok_or(AppError::NotFound)?;

        if !user.user_state {
            return Err(AppError::NotFound);
        }
        Ok(user)
    }

    pub async fn update_me(st: &AppState, user_id: i64, req: UpdateReq) -> AppResult<ProfileRes> {
        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(e.to_string()));
        }

        let current_user = repo::find_user(&st.db, user_id)
            .await?
            .ok_or(AppError::NotFound)?;

            if !current_user.user_state {
                return Err(AppError::Forbidden);
            }

        let updated_user = repo::update_user(
            &st.db,
            user_id,
            req.nickname.as_deref(),
            req.language.as_deref(),
            req.country.as_deref(),
            req.birthday,
            req.gender,
        )
        .await?;

        if let Err(le) =
            repo::insert_user_log_after(&st.db, Some(user_id), user_id, "update", true).await
        {
            warn!(error=?le, user_id = user_id, "users_log(update) insert failed");
        }

        Ok(updated_user)
    }

    pub async fn get_settings(st: &AppState, user_id: i64) -> AppResult<SettingsRes> {
        let user = repo::find_user(&st.db, user_id)
            .await?
            .ok_or(AppError::NotFound)?;

            if !user.user_state {
                return Err(AppError::Forbidden);
            }

        repo::find_users_setting(&st.db, user_id).await
    }

    pub async fn update_users_setting(
        st: &AppState,
        user_id: i64,
        req: SettingsUpdateReq,
    ) -> AppResult<SettingsRes> {
        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(e.to_string()));
        }

        let current_user = repo::find_user(&st.db, user_id)
            .await?
            .ok_or(AppError::NotFound)?;

            if !current_user.user_state {
                return Err(AppError::Forbidden);
            }

        if let Some(study_langs) = &req.study_languages {
            if study_langs.len() > 8 {
                return Err(AppError::BadRequest(
                    "Too many study languages (max 8)".into(),
                ));
            }

            let allowed_lang_codes: HashSet<&str> = ["en", "ko", "ne", "si", "id", "vi", "th"]
                .iter()
                .cloned()
                .collect();

            let mut seen_lang_codes = HashSet::new();
            let mut primary_count = 0;

            for item in study_langs {
                if !allowed_lang_codes.contains(item.lang_code.as_str()) {
                    return Err(AppError::BadRequest(format!(
                        "Invalid lang_code: {}",
                        item.lang_code
                    )));
                }
                if !seen_lang_codes.insert(&item.lang_code) {
                    return Err(AppError::BadRequest(format!(
                        "Duplicate lang_code in study_languages: {}",
                        item.lang_code
                    )));
                }
                if item.is_primary {
                    primary_count += 1;
                }
            }

            if primary_count > 1 {
                return Err(AppError::BadRequest(
                    "Only one primary study language is allowed".into(),
                ));
            }
        }

        if let Some(ui_lang) = &req.ui_language {
            let allowed_lang_codes: HashSet<&str> = ["en", "ko", "ne", "si", "id", "vi", "th"]
                .iter()
                .cloned()
                .collect();
            if !allowed_lang_codes.contains(ui_lang.as_str()) {
                return Err(AppError::BadRequest(format!(
                    "Invalid ui_language: {}",
                    ui_lang
                )));
            }
        }

        let update_users_setting = repo::update_users_setting(&st.db, user_id, &req).await;

        match update_users_setting {
            Ok(settings) => Ok(settings),
            Err(e) => {
                if let AppError::Sqlx(sqlx::Error::Database(db_err)) = &e {
                    if db_err.code().as_deref() == Some(Self::PG_UNIQUE_VIOLATION) {
                        return Err(AppError::BadRequest(
                            "Duplicate lang_code in study_languages".into(),
                        ));
                    }
                }
                Err(e)
            }
        }
    }
}
