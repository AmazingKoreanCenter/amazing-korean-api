use super::dto::UserOut;
use crate::error::AppResult;
use sqlx::PgPool;

#[derive(sqlx::FromRow)]
pub struct UserRow {
    pub user_id: i64,
    pub user_email: String,
    pub user_password: String,
    pub user_name: Option<String>,
    pub user_created_at: chrono::DateTime<chrono::Utc>,
    pub user_state: String,
    pub user_auth: String,
}

// 이메일로 조회(대소문자 무시)
pub async fn find_by_email(pool: &PgPool, email: &str) -> AppResult<Option<UserRow>> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
        SELECT user_id, user_email, user_password, user_name, user_created_at, user_state, user_auth
        FROM users
        WHERE LOWER(user_email) = LOWER($1)
        LIMIT 1
    "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn create_user(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
    name: &str,
    terms_service: bool,
    terms_personal: bool,
) -> AppResult<i64> {
    let rec = sqlx::query_as::<_, (i64,)>(
        r#"
        INSERT INTO users (
            user_email, user_password, user_name,
            user_terms_service, user_terms_personal
        )
        VALUES (LOWER($1), $2, $3, $4, $5)
        RETURNING user_id
    "#,
    )
    .bind(email)
    .bind(password_hash)
    .bind(name)
    .bind(terms_service)
    .bind(terms_personal)
    .fetch_one(pool)
    .await?;
    Ok(rec.0)
}

pub async fn get_user_out(pool: &PgPool, user_id: i64) -> AppResult<Option<UserOut>> {
    let row = sqlx::query_as::<_, UserOut>(
        r#"
        SELECT user_id, user_email, user_name, user_created_at, user_state, user_auth
        FROM users
        WHERE user_id = $1
    "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}
