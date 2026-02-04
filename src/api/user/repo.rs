use super::dto::{ProfileRes, ProfileUpdateReq, SettingsRes, SettingsUpdateReq};
use crate::{
    error::AppResult,
    types::{UserAuth, UserGender, UserLanguage},
};
use chrono::{NaiveDate, Utc};
use sqlx::{PgPool, Postgres, Transaction};

// =========================================================================
// User Core (Signup & Find)
// =========================================================================

/// 회원가입 처리 (Transaction)
#[allow(clippy::too_many_arguments)]
pub async fn signup_tx(
    tx: &mut Transaction<'_, Postgres>,
    email: &str,
    password_hash: &str,
    name: &str,
    nickname: &str,
    language: &str,
    country: &str,
    birthday: NaiveDate,
    gender: UserGender,
    terms_service: bool,
    terms_personal: bool,
) -> AppResult<ProfileRes> {
    let res = sqlx::query_as::<_, ProfileRes>(r#"
        INSERT INTO users (
            user_email, user_password, user_name,
            user_nickname, user_language, user_country,
            user_birthday, user_gender,
            user_terms_service, user_terms_personal
        )
        VALUES (
            $1, $2, $3,
            $4, $5::user_language_enum, $6,
            $7, $8::user_gender_enum,
            $9, $10
        )
        RETURNING
            user_id as id,
            user_email as email,
            user_name as name,
            user_nickname as nickname,
            user_language::TEXT as language, -- DB Enum -> Rust String
            user_country as country,
            user_birthday as birthday,
            user_gender as gender,           -- DB Enum -> Rust Enum (sqlx::Type)
            user_state,
            user_auth,
            user_created_at as created_at,
            (user_password IS NOT NULL) as has_password
    "#)
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
    .fetch_one(&mut **tx)
    .await?;
    
    Ok(res)
}

/// 이메일 중복 확인용 (ID 조회)
pub async fn find_user_id_by_email(pool: &PgPool, email: &str) -> AppResult<Option<i64>> {
    let row = sqlx::query_scalar::<_, i64>(r#"
        SELECT user_id FROM users WHERE user_email = $1
    "#)
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// 이메일로 사용자 전체 정보 조회
pub async fn find_user_by_email(pool: &PgPool, email: &str) -> AppResult<Option<ProfileRes>> {
    let row = sqlx::query_as::<_, ProfileRes>(r#"
        SELECT
            user_id as id,
            user_email as email,
            user_name as name,
            user_nickname as nickname,
            user_language::TEXT as language,
            user_country as country,
            user_birthday as birthday,
            user_gender as gender,
            user_state,
            user_auth,
            user_created_at as created_at,
            (user_password IS NOT NULL) as has_password
        FROM users
        WHERE user_email = $1
    "#)
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// 닉네임으로 사용자 조회
pub async fn find_user_by_nickname(pool: &PgPool, nickname: &str) -> AppResult<Option<ProfileRes>> {
    let row = sqlx::query_as::<_, ProfileRes>(r#"
        SELECT
            user_id as id,
            user_email as email,
            user_name as name,
            user_nickname as nickname,
            user_language::TEXT as language,
            user_country as country,
            user_birthday as birthday,
            user_gender as gender,
            user_state,
            user_auth,
            user_created_at as created_at,
            (user_password IS NOT NULL) as has_password
        FROM users
        WHERE user_nickname = $1
    "#)
    .bind(nickname)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

/// 사용자 정보 공통 조회 쿼리
pub async fn find_user(pool: &PgPool, user_id: i64) -> AppResult<Option<ProfileRes>> {
    let row = sqlx::query_as::<_, ProfileRes>(r#"
        SELECT
            user_id as id,
            user_email as email,
            user_name as name,
            user_nickname as nickname,
            user_language::TEXT as language,
            user_country as country,
            user_birthday as birthday,
            user_gender as gender,
            user_state,
            user_auth,
            user_created_at as created_at,
            (user_password IS NOT NULL) as has_password
        FROM users
        WHERE user_id = $1
    "#)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

// =========================================================================
// Profile (My Page)
// =========================================================================

/// 내 프로필 상세 조회
pub async fn find_profile_by_id(pool: &PgPool, user_id: i64) -> AppResult<Option<ProfileRes>> {
    // find_user와 동일한 로직이므로 재사용
    find_user(pool, user_id).await
}

/// 내 프로필 수정 (Transaction)
pub async fn update_profile_tx(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i64,
    req: &ProfileUpdateReq,
) -> AppResult<Option<ProfileRes>> {
    let res = sqlx::query_as::<_, ProfileRes>(r#"
        UPDATE users
        SET
            user_nickname = COALESCE($2, user_nickname),
            user_language = COALESCE($3::user_language_enum, user_language),
            user_country  = COALESCE($4, user_country),
            user_birthday = COALESCE($5, user_birthday),
            user_gender   = COALESCE($6::user_gender_enum, user_gender)
        WHERE user_id = $1
        RETURNING
            user_id as id,
            user_email as email,
            user_name as name,
            user_nickname as nickname,
            user_language::TEXT as language,
            user_country as country,
            user_birthday as birthday,
            user_gender as gender,
            user_state,
            user_auth,
            user_created_at as created_at,
            (user_password IS NOT NULL) as has_password
    "#)
    .bind(user_id)
    .bind(req.nickname.as_ref())
    .bind(req.language.as_ref())
    .bind(req.country.as_ref())
    .bind(req.birthday)
    .bind(req.gender)
    .fetch_optional(&mut **tx)
    .await?;
    
    Ok(res)
}

// =========================================================================
// Settings (Preferences)
// =========================================================================

pub async fn find_users_setting(pool: &PgPool, user_id: i64) -> AppResult<Option<SettingsRes>> {
    let row = sqlx::query_as::<_, SettingsRes>(r#"
        SELECT
            user_set_language::TEXT as user_set_language,
            COALESCE(user_set_timezone, 'UTC') as user_set_timezone,
            user_set_note_email,
            user_set_note_push,
            user_set_updated_at as updated_at
        FROM users_setting
        WHERE user_id = $1
    "#)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn upsert_settings_tx(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i64,
    req: &SettingsUpdateReq,
) -> AppResult<SettingsRes> {
    let res = sqlx::query_as::<_, SettingsRes>(r#"
        INSERT INTO users_setting (
            user_id,
            user_set_language,
            user_set_timezone,
            user_set_note_email,
            user_set_note_push,
            user_set_updated_at
        )
        VALUES (
            $1,
            COALESCE($2::user_set_language_enum, 'ko'::user_set_language_enum),
            COALESCE($3, 'UTC'),
            COALESCE($4, false),
            COALESCE($5, false),
            $6
        )
        ON CONFLICT (user_id) DO UPDATE SET
            user_set_language = COALESCE(EXCLUDED.user_set_language, users_setting.user_set_language),
            user_set_timezone = COALESCE(EXCLUDED.user_set_timezone, users_setting.user_set_timezone),
            user_set_note_email = COALESCE(EXCLUDED.user_set_note_email, users_setting.user_set_note_email),
            user_set_note_push = COALESCE(EXCLUDED.user_set_note_push, users_setting.user_set_note_push),
            user_set_updated_at = EXCLUDED.user_set_updated_at
        RETURNING
            user_set_language::TEXT as user_set_language,
            COALESCE(user_set_timezone, 'UTC') as user_set_timezone,
            user_set_note_email,
            user_set_note_push,
            user_set_updated_at as updated_at
    "#)
    .bind(user_id)
    .bind(req.user_set_language.as_ref())
    .bind(req.user_set_timezone.as_ref())
    .bind(req.user_set_note_email)
    .bind(req.user_set_note_push)
    .bind(Utc::now())
    .fetch_one(&mut **tx)
    .await?;

    Ok(res)
}

// =========================================================================
// Logging (Audit)
// =========================================================================

pub async fn insert_user_log_after_tx(
    tx: &mut Transaction<'_, Postgres>,
    actor_user_id: Option<i64>,
    user_id: i64,
    action: &str,
    success: bool,
) -> AppResult<()> {
    sqlx::query(r#"
        INSERT INTO public.users_log (
            updated_by_user_id, user_action_log, user_action_success, user_id,
            user_auth_log, user_state_log, user_email_log, user_password_log,
            user_nickname_log, user_language_log, user_country_log, user_birthday_log,
            user_gender_log, user_terms_service_log, user_terms_personal_log,
            user_log_created_at, user_log_quit_at, user_log_updated_at
        )
        SELECT
            $1, 
            CAST($2 AS user_action_log_enum), 
            $3, 
            u.user_id,
            u.user_auth,      
            u.user_state,    
            u.user_email, 
            false,            
            u.user_nickname, 
            u.user_language, 
            u.user_country, 
            u.user_birthday,
            u.user_gender,   
            u.user_terms_service, 
            u.user_terms_personal,
            u.user_created_at, 
            u.user_quit_at, 
            now()
        FROM public.users u
        WHERE u.user_id = $4
    "#)
    .bind(actor_user_id)
    .bind(action)
    .bind(success)
    .bind(user_id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

/// (Utility) 트랜잭션 없이 로그를 남겨야 할 때 사용
pub async fn insert_user_log_after(
    pool: &PgPool,
    actor_user_id: Option<i64>,
    user_id: i64,
    action: &str,
    success: bool,
) -> AppResult<()> {
    let mut tx = pool.begin().await?;
    insert_user_log_after_tx(&mut tx, actor_user_id, user_id, action, success).await?;
    tx.commit().await?;
    Ok(())
}

// =========================================================================
// Admin User Creation (for invite system)
// =========================================================================

/// 관리자 계정 생성 (초대 시스템용)
#[allow(clippy::too_many_arguments)]
pub async fn create_admin_user(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
    name: &str,
    nickname: &str,
    country: &str,
    birthday: NaiveDate,
    gender: UserGender,
    language: UserLanguage,
    user_auth: UserAuth,
) -> AppResult<i64> {
    // UserLanguage와 UserAuth를 문자열로 변환
    let language_str = match language {
        UserLanguage::Ko => "ko",
        UserLanguage::En => "en",
    };
    let user_auth_str = match user_auth {
        UserAuth::Hymn => "HYMN",
        UserAuth::Admin => "admin",
        UserAuth::Manager => "manager",
        UserAuth::Learner => "learner",
    };

    let user_id = sqlx::query_scalar::<_, i64>(
        r#"
        INSERT INTO users (
            user_email, user_password, user_name,
            user_nickname, user_language, user_country,
            user_birthday, user_gender, user_auth,
            user_check_email, user_terms_service, user_terms_personal
        )
        VALUES (
            $1, $2, $3,
            $4, $5::user_language_enum, $6,
            $7, $8::user_gender_enum, $9::user_auth_enum,
            true, true, true
        )
        RETURNING user_id
    "#,
    )
    .bind(email)
    .bind(password_hash)
    .bind(name)
    .bind(nickname)
    .bind(language_str)
    .bind(country)
    .bind(birthday)
    .bind(gender)
    .bind(user_auth_str)
    .fetch_one(pool)
    .await?;

    Ok(user_id)
}