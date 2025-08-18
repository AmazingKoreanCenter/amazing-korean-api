use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHasher};
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::{
    api::auth::dto::UserOut,
    error::{AppError, AppResult},
    state::AppState,
};

use super::dto::{CreateUserReq, Gender};

pub struct UserService;

impl UserService {
    pub async fn create_user(st: &AppState, req: CreateUserReq) -> AppResult<UserOut> {
        // 1) 유효성 검사
        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(e.to_string()));
        }

        // 2) 비밀번호 해시
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(req.password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(format!("password hash error: {e}")))?
            .to_string();

        // 3) 생일 파싱 (있다면)
        let birthday_dt: Option<DateTime<Utc>> = if let Some(b) = req.birthday.as_deref() {
            let parsed = chrono::DateTime::parse_from_rfc3339(b)
                .map_err(|_| AppError::BadRequest("birthday must be RFC3339/ISO8601".into()))?;
            Some(parsed.with_timezone(&Utc))
        } else {
            None
        };

        // 4) 성별 문자열
        let gender_str = req.gender.unwrap_or(Gender::None).to_string();

        // 5) INSERT
        let q = r#"
                                INSERT INTO users (
                                    user_email, user_password, user_name,
                                    user_nickname, user_language, user_country, user_birthday, user_gender,
                                    user_terms_service, user_terms_personal
                                ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
                                RETURNING user_id, user_email, user_name, user_created_at, user_state, user_auth
                                "#;

        let res = sqlx::query_as::<_, UserOut>(q)
            .bind(&req.email)
            .bind(&password_hash)
            .bind(&req.name)
            .bind(&req.nickname)
            .bind(&req.language)
            .bind(&req.country)
            .bind(birthday_dt)
            .bind(&gender_str)
            .bind(req.terms_service)
            .bind(req.terms_personal)
            .fetch_one(&st.db)
            .await;

        match res {
            Ok(user) => Ok(user),
            Err(e) => {
                // UNIQUE(email) 충돌 매핑
                if let sqlx::Error::Database(db) = &e {
                    if db.code().as_deref() == Some("23505") {
                        return Err(AppError::Conflict("email already exists".into()));
                    }
                }
                Err(AppError::Sqlx(e.into()))
            }
        }
    }
}
