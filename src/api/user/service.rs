use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher, Algorithm, Params, Version,
};
use validator::Validate;

use crate::{
    error::{AppError, AppResult},
    state::AppState,
};

use super::{
    dto::{Gender, ProfileRes, SignupReq, UpdateReq},
    repo,
};

pub struct UserService;

impl UserService {
    const PG_UNIQUE_VIOLATION: &'static str = "23505";

    #[inline]
    fn is_unique_violation(err: &AppError) -> bool {
        match err {
            AppError::Sqlx(sqlx_err) => match sqlx_err {
                sqlx::Error::Database(db) => db.code().as_deref() == Some(Self::PG_UNIQUE_VIOLATION),
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
        //let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(req.password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(format!("password hash error: {e}")))?
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
            Ok(user) => Ok(user),
            Err(e) if Self::is_unique_violation(&e) => {
                Err(AppError::Conflict("email already exists".into()))
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_me(st: &AppState, user_id: i64) -> AppResult<ProfileRes> {
        let user = repo::find_by_id(&st.db, user_id)
            .await?
            .ok_or(AppError::NotFound)?;

        if user.user_state != "on" {
            return Err(AppError::Forbidden);
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
            .ok_or(AppError::NotFound)?;

        if current_user.user_state != "on" {
            return Err(AppError::Forbidden);
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
        .await?;

        // TODO: (스펙 #37) user_log에 before/after 스냅샷 기록
        // repo::insert_user_log(&st.db, user_id, &current_user, &updated_user).await?;

        Ok(updated_user)
    }
}
