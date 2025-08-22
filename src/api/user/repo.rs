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

// 회원 관련 기록 log repo
/// - action: "create" | "update" | "deactivate" | "delete" ...
/// - updated_by_user_id: 행위자(본인/관리자/시스템). None 허용.
/// - snap: After 기준 스냅샷. 여기서는 snap.id만 사용(나머지는 DB에서 SELECT).
pub async fn insert_user_log_after(
    pool: &PgPool,
    action: &str,
    updated_by_user_id: Option<i64>,
    snap: &ProfileRes,
) -> AppResult<()> {
    // INSERT ... SELECT 로 DB 값 그대로 스냅샷 (비밀번호는 항상 NULL)
    sqlx::query(
        r#"
        INSERT INTO user_log (
            action, updated_by_user_id, user_id,
            user_auth_log, user_state_log, user_email_log, user_password_log,
            user_nickname_log, user_language_log, user_country_log,
            user_birthday_log, user_gender_log,
            user_terms_service_log, user_terms_personal_log
        )
        SELECT
            $1 AS action,
            $2 AS updated_by_user_id,
            u.user_id,
            u.user_auth,
            u.user_state,
            u.user_email,
            NULL,                        -- user_password_log (민감정보 금지)
            u.user_nickname,
            u.user_language,
            u.user_country,
            u.user_birthday,
            u.user_gender,
            u.user_terms_service,
            u.user_terms_personal
        FROM users u
        WHERE u.user_id = $3
        "#,
    )
    .bind(action)
    .bind(updated_by_user_id)
    .bind(snap.id)
    .execute(pool)
    .await?;

    Ok(())
}