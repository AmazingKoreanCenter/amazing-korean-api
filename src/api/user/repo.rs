use super::dto::ProfileRes;
use crate::error::AppResult;
use chrono::NaiveDate;
use sqlx::PgPool;

pub async fn create_user(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
    name: &str,
    nickname: Option<&str>,
    language: Option<&str>,
    country: Option<&str>,
    birthday: Option<NaiveDate>,
    gender: &str,
    terms_service: bool,
    terms_personal: bool,
) -> AppResult<ProfileRes> {
    let res = sqlx::query_as::<_, ProfileRes>(
        r#"
        INSERT INTO users (
            user_email, user_password, user_name,
            user_nickname, user_language, user_country, user_birthday, user_gender,
            user_terms_service, user_terms_personal
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING
            user_id as id, user_email as email, user_name as name,
            user_nickname as nickname, user_language as language, user_country as country,
            user_birthday as birthday, user_gender as gender,
            user_state, user_auth, user_created_at as created_at
    "#,
    )
    .bind(email)
    .bind(password_hash)
    .bind(name)
    .bind(nickname)
    .bind(language)
    .bind(country)
    .bind(birthday)
    .bind(gender)
    .bind(terms_service)
    .bind(terms_personal)
    .fetch_one(pool)
    .await?;
    Ok(res)
}

pub async fn find_by_id(pool: &PgPool, user_id: i64) -> AppResult<Option<ProfileRes>> {
    let row = sqlx::query_as::<_, ProfileRes>(
        r#"
        SELECT
            user_id as id, user_email as email, user_name as name,
            user_nickname as nickname, user_language as language, user_country as country,
            user_birthday as birthday, user_gender as gender,
            user_state, user_auth, user_created_at as created_at
        FROM users
        WHERE user_id = $1
    "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn update_profile(
    pool: &PgPool,
    user_id: i64,
    nickname: Option<&str>,
    language: Option<&str>,
    country: Option<&str>,
    birthday: Option<NaiveDate>,
    gender: Option<&str>,
) -> AppResult<ProfileRes> {
    let res = sqlx::query_as::<_, ProfileRes>(
        r#"
        UPDATE users
        SET
            user_nickname = COALESCE($2, user_nickname),
            user_language = COALESCE($3, user_language),
            user_country = COALESCE($4, user_country),
            user_birthday = COALESCE($5, user_birthday),
            user_gender = COALESCE($6, user_gender)
        WHERE user_id = $1
        RETURNING
            user_id as id, user_email as email, user_name as name,
            user_nickname as nickname, user_language as language, user_country as country,
            user_birthday as birthday, user_gender as gender,
            user_state, user_auth, user_created_at as created_at
    "#,
    )
    .bind(user_id)
    .bind(nickname)
    .bind(language)
    .bind(country)
    .bind(birthday)
    .bind(gender)
    .fetch_one(pool)
    .await?;
    Ok(res)
}
