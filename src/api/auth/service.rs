use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use password_hash::{PasswordHash, /*PasswordHasher as _*/ SaltString};
use rand_core::OsRng;

use super::{dto::UserOut, jwt, repo};
use crate::{
    error::{AppError, AppResult},
    state::AppState,
};

pub struct AuthService;

impl AuthService {
    pub async fn signup(
        st: &AppState,
        email: &str,
        password: &str,
        name: &str,
        terms_service: bool,
        terms_personal: bool,
    ) -> AppResult<i64> {
        if repo::find_by_email(&st.db, email).await?.is_some() {
            return Err(AppError::Conflict("email already exists".into()));
        }

        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(e.to_string()))?
            .to_string();

        // UNIQUE 위반(동시가입 등)은 여기서도 잡아준다.
        let res =
            repo::create_user(&st.db, email, &hash, name, terms_service, terms_personal).await;
        match res {
            Ok(user_id) => Ok(user_id),
            Err(AppError::Sqlx(sqlx::Error::Database(db_err)))
                if db_err.code().as_deref() == Some("23505") =>
            {
                Err(AppError::Conflict("email already exists".into()))
            }
            Err(e) => Err(e), // 나머지는 그대로 전파 (Internal/Sqlx 등)
        }
    }

    pub async fn login(
        st: &AppState,
        email: &str,
        password: &str,
    ) -> AppResult<(String, i64, UserOut)> {
        let user = repo::find_by_email(&st.db, email)
            .await?
            .ok_or(AppError::Unauthorized("invalid credentials".into()))?;

        let parsed = PasswordHash::new(&user.user_password)
            .map_err(|_| AppError::Unauthorized("invalid credentials".into()))?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .map_err(|_| AppError::Unauthorized("invalid credentials".into()))?;

        if user.user_state != "on" {
            return Err(AppError::Unauthorized("user is not active".into()));
        }

        let (token, expires_in) =
            jwt::encode_token(user.user_id).map_err(|e| AppError::Internal(e.to_string()))?;

        let out = UserOut {
            user_id: user.user_id,
            user_email: user.user_email,
            user_name: user.user_name,
            user_created_at: user.user_created_at,
            user_state: user.user_state,
            user_auth: user.user_auth,
        };

        Ok((token, expires_in, out))
    }

    pub async fn me(st: &AppState, user_id: i64) -> AppResult<UserOut> {
        let out = repo::get_user_out(&st.db, user_id)
            .await?
            .ok_or(AppError::NotFound)?;
        Ok(out)
    }
}
