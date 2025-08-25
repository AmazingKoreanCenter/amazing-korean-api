use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHasher, Version,
};
use validator::Validate;

use crate::{
    error::{AppError, AppResult},
    state::AppState,
};

use super::{
    dto::{Gender, ProfileRes, SettingsRes, SettingsUpdateReq, SignupReq, UpdateReq},
    repo,
};
use std::collections::HashSet;

// 로깅 실패 무시용
use tracing::warn;

pub struct UserService;

impl UserService {
    const PG_UNIQUE_VIOLATION: &'static str = "23505";

    #[inline]
    fn is_unique_violation(err: &AppError) -> bool {
        match err {
            AppError::Sqlx(sqlx_err) => match sqlx_err {
                sqlx::Error::Database(db) => {
                    db.code().as_deref() == Some(Self::PG_UNIQUE_VIOLATION)
                }
                _ => false,
            },
            _ => false,
        }
    }

    pub async fn signup(st: &AppState, mut req: SignupReq) -> AppResult<ProfileRes> {
        // 0) 이메일 정규화 (중복/로그인 일관성)
        req.email = req.email.trim().to_lowercase();

        // 1) 유효성 검사
        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(e.to_string()));
        }

        // 1-1) 약관 동의 강제 (스펙 #37)
        if !req.terms_service || !req.terms_personal {
            return Err(AppError::BadRequest("terms must be accepted".into()));
        }

        // 2) 비밀번호 해시
        let salt = SaltString::generate(&mut OsRng);
        let params = Params::new(19_456, 2, 1, None).unwrap();
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        // let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(req.password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(format!("password hash error: {e}")))? // 500
            .to_string();

        // 3) 성별 문자열
        let gender_str = req.gender.unwrap_or(Gender::None).to_string();

        // 4) INSERT
        let res = repo::create_user(
            &st.db,
            &req.email,
            &password_hash,
            &req.name,
            req.nickname.as_deref(),
            req.language.as_deref(),
            req.country.as_deref(),
            req.birthday,
            &gender_str,
            req.terms_service,
            req.terms_personal,
        )
        .await;

        match res {
            Ok(user) => {
                // ⭐ (NEW) 사용자 스냅샷 기록: action = "create"
                // DB의 실제 저장값을 INSERT ... SELECT로 스냅샷하는 repo::insert_user_log_after 호출
                if let Err(le) =
                    repo::insert_user_log_after(&st.db, "create", Some(user.id), &user).await
                {
                    warn!(error=?le, user_id = user.id, "user_log(create) insert failed");
                }
                Ok(user)
            }
            Err(e) if Self::is_unique_violation(&e) => {
                Err(AppError::Conflict("email already exists".into())) // 409
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_me(st: &AppState, user_id: i64) -> AppResult<ProfileRes> {
        let user = repo::find_by_id(&st.db, user_id)
            .await?
            .ok_or(AppError::NotFound)?; // 404

        if user.user_state != "on" {
            return Err(AppError::Forbidden); // 403
        }
        Ok(user)
    }

    pub async fn update_me(st: &AppState, user_id: i64, req: UpdateReq) -> AppResult<ProfileRes> {
        // 1) 유효성 검사
        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(e.to_string()));
        }

        // 2) 사용자 상태 확인
        let current_user = repo::find_by_id(&st.db, user_id)
            .await?
            .ok_or(AppError::NotFound)?; // 404

        if current_user.user_state != "on" {
            return Err(AppError::Forbidden); // 403
        }

        // 3) 성별 문자열
        let gender_str = req.gender.map(|g| g.to_string());

        // 4) UPDATE
        let updated_user = repo::update_profile(
            &st.db,
            user_id,
            req.nickname.as_deref(),
            req.language.as_deref(),
            req.country.as_deref(),
            req.birthday,
            gender_str.as_deref(),
        )
        .await?; // 200

        // ⭐ (NEW) 사용자 스냅샷 기록: action = "update"
        if let Err(le) =
            repo::insert_user_log_after(&st.db, "update", Some(user_id), &updated_user).await
        {
            warn!(error=?le, user_id = user_id, "user_log(update) insert failed");
        }

        Ok(updated_user)
    }

    pub async fn get_settings(st: &AppState, user_id: i64) -> AppResult<SettingsRes> {
        let user = repo::find_by_id(&st.db, user_id)
            .await?
            .ok_or(AppError::NotFound)?;

        if user.user_state != "on" {
            return Err(AppError::Forbidden);
        }

        repo::find_settings_by_user_id(&st.db, user_id).await
    }

    pub async fn update_settings(
        st: &AppState,
        user_id: i64,
        req: SettingsUpdateReq,
    ) -> AppResult<SettingsRes> {
        // 1) 유효성 검사
        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(e.to_string()));
        }

        // 2) 사용자 상태 확인
        let current_user = repo::find_by_id(&st.db, user_id)
            .await?
            .ok_or(AppError::NotFound)?;

        if current_user.user_state != "on" {
            return Err(AppError::Forbidden);
        }

        // 3) study_languages 유효성 검사
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
                    return Err(AppError::BadRequest(
                        format!("Invalid lang_code: {}", item.lang_code).into(),
                    ));
                }
                if !seen_lang_codes.insert(&item.lang_code) {
                    return Err(AppError::BadRequest(
                        format!("Duplicate lang_code in study_languages: {}", item.lang_code)
                            .into(),
                    ));
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

        // 4) ui_language 유효성 검사
        if let Some(ui_lang) = &req.ui_language {
            let allowed_lang_codes: HashSet<&str> = ["en", "ko", "ne", "si", "id", "vi", "th"]
                .iter()
                .cloned()
                .collect();
            if !allowed_lang_codes.contains(ui_lang.as_str()) {
                return Err(AppError::BadRequest(
                    format!("Invalid ui_language: {}", ui_lang).into(),
                ));
            }
        }

        // 5) UPDATE
        let updated_settings = repo::upsert_settings(&st.db, user_id, &req).await;

        match updated_settings {
            Ok(settings) => {
                // ⭐ (NEW) 사용자 스냅샷 기록: action = "update_settings"
                // Note: user_log currently only supports ProfileRes, so we'll skip this for now
                // if let Err(le) = repo::insert_user_log_after(&st.db, "update_settings", Some(user_id), &settings).await {
                //     warn!(error=?le, user_id = user_id, "user_log(update_settings) insert failed");
                // }
                Ok(settings)
            }
            Err(e) => {
                // Map unique violation for study_languages to BadRequest
                if let AppError::Sqlx(sqlx_err) = &e {
                    if let sqlx::Error::Database(db_err) = sqlx_err {
                        if db_err.code().as_deref() == Some(Self::PG_UNIQUE_VIOLATION) {
                            return Err(AppError::BadRequest(
                                "Duplicate lang_code in study_languages".into(),
                            ));
                        }
                    }
                }
                Err(e)
            }
        }
    }
}
