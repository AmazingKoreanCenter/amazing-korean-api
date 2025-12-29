use super::dto::{ProfileRes, SettingsRes, SettingsUpdateReq, StudyLangItem};
use crate::{error::AppResult, types::UserGender};
use chrono::{NaiveDate, Utc};
use sqlx::{PgPool, Postgres, Transaction};

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
    let res = sqlx::query_as::<_, ProfileRes>(
        r#"
        INSERT INTO users (
            user_email, user_password, user_name,
            user_nickname, user_language, user_country, user_birthday, user_gender,
            user_terms_service, user_terms_personal
        )
        VALUES (
            $1, $2, $3, $4, 
            $5::user_language_enum,  -- 입력 시 Enum 캐스팅 필요
            $6, $7, 
            $8::user_gender_enum,    -- 입력 시 Enum 캐스팅 필요
            $9, $10
        )
        RETURNING
            user_id as id, 
            user_email as email, 
            user_name as name,
            user_nickname as nickname, 
            user_language::TEXT as language, -- DTO가 String이므로 TEXT 변환
            user_country as country,
            user_birthday as birthday, 
            user_gender as gender,           -- Enum <-> Enum (자동 매핑)
            user_state,                      -- bool <-> bool (자동 매핑)
            user_auth,                       -- [중요] ::TEXT 제거! (Enum <-> Enum 자동 매핑)
            user_created_at as created_at
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
    .fetch_one(&mut **tx)
    .await?;
    Ok(res)
}

pub async fn find_user_id_by_email(pool: &PgPool, email: &str) -> AppResult<Option<i64>> {
    let row = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT user_id
        FROM users
        WHERE user_email = $1
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

// 프로필 조회 repo
pub async fn find_user(pool: &PgPool, user_id: i64) -> AppResult<Option<ProfileRes>> {
    let row = sqlx::query_as::<_, ProfileRes>(
        r#"
        SELECT
            user_id as id, user_email as email, user_name as name,
            user_nickname as nickname, user_language::TEXT as language, user_country as country,
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

// 내 정보 조회 repo
pub async fn find_profile_by_id(pool: &PgPool, user_id: i64) -> AppResult<Option<ProfileRes>> {
    let row = sqlx::query_as::<_, ProfileRes>(
        r#"
        SELECT
            user_id as id, user_email as email, user_name as name,
            user_nickname as nickname, user_language::TEXT as language, user_country as country,
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

// 프로필 수정 repo
pub async fn update_user(
    pool: &PgPool,
    user_id: i64,
    nickname: Option<&str>,
    language: Option<&str>,
    country: Option<&str>,
    birthday: Option<NaiveDate>,
    gender: Option<UserGender>,
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
            user_nickname as nickname, user_language::TEXT as language, user_country as country,
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

// 환경설정 조회 repo
pub async fn find_users_setting(pool: &PgPool, user_id: i64) -> AppResult<SettingsRes> {
    let user_setting =
        sqlx::query_as::<_, (Option<String>, Option<String>, Option<bool>, Option<bool>)>(
            r#"
        SELECT
            us.ui_language,
            us.timezone,
            us.notifications_email,
            us.notifications_push
        FROM users_setting us
        WHERE us.user_id = $1
        "#,
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

    let study_languages = sqlx::query_as::<_, StudyLangItem>(
        r#"
        SELECT
            lang_code,
            priority,
            is_primary
        FROM users_language_pref
        WHERE user_id = $1
        ORDER BY priority ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let (ui_language, timezone, notifications_email, notifications_push) =
        user_setting.unwrap_or_default();

    Ok(SettingsRes {
        user_id,
        ui_language,
        timezone,
        notifications_email,
        notifications_push,
        study_languages,
    })
}

// 환경설정 수정 repo
pub async fn update_users_setting(
    pool: &PgPool,
    user_id: i64,
    req: &SettingsUpdateReq,
) -> AppResult<SettingsRes> {
    let mut tx = pool.begin().await?;

    // Update users_setting
    sqlx::query(
        r#"
        INSERT INTO users_setting (user_id, ui_language, timezone, notifications_email, notifications_push, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (user_id) DO UPDATE SET
            ui_language = COALESCE($2, users_setting.ui_language),
            timezone = COALESCE($3, users_setting.timezone),
            notifications_email = COALESCE($4, users_setting.notifications_email),
            notifications_push = COALESCE($5, users_setting.notifications_push),
            updated_at = $6
        "#,
    )
    .bind(user_id)
    .bind(req.ui_language.as_ref())
    .bind(req.timezone.as_ref())
    .bind(req.notifications_email)
    .bind(req.notifications_push)
    .bind(Utc::now())
    .execute(&mut *tx)
    .await?;

    // Update study_languages if provided
    if let Some(study_langs) = &req.study_languages {
        // Delete existing preferences
        sqlx::query(
            r#"
            DELETE FROM users_language_pref
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .execute(&mut *tx)
        .await?;

        // Insert new preferences with normalized priorities
        let mut sorted_langs = study_langs.clone();
        sorted_langs.sort_by_key(|item| item.priority);

        for (idx, item) in sorted_langs.iter().enumerate() {
            sqlx::query(
                r#"
                INSERT INTO users_language_pref (user_id, lang_code, priority, is_primary)
                VALUES ($1, $2, $3, $4)
                "#,
            )
            .bind(user_id)
            .bind(&item.lang_code)
            .bind((idx + 1) as i32) // Normalize priority
            .bind(item.is_primary)
            .execute(&mut *tx)
            .await?;
        }
    }

    tx.commit().await?;

    // Fetch the latest settings after update
    find_users_setting(pool, user_id).await
}

// 회원 관련 기록 log repo
// 회원 관련 기록 log repo
pub async fn insert_user_log_after_tx(
    tx: &mut Transaction<'_, Postgres>,
    actor_user_id: Option<i64>,
    user_id: i64,
    action: &str,
    success: bool,
) -> AppResult<()> {
    sqlx::query(
        r#"
        INSERT INTO public.users_log (
          updated_by_user_id, user_action_log, user_action_success, user_id,
          user_auth_log, user_state_log, user_email_log, user_password_log,
          user_nickname_log, user_language_log, user_country_log, user_birthday_log,
          user_gender_log, user_terms_service_log, user_terms_personal_log,
          user_log_created_at, user_log_quit_at, user_log_updated_at
        )
        SELECT
          $1, 
          CAST($2 AS user_action_log_enum), -- Rust String($2) -> DB Enum 변환 (필수)
          $3, 
          u.user_id,
          u.user_auth,      -- [수정] ::text 제거 (Enum -> Enum)
          u.user_state,     -- [수정] ::text 제거 (Bool -> Bool)
          u.user_email, 
          false,            -- [수정] Password 변경 아님 (Boolean default false)
          u.user_nickname, 
          u.user_language,  -- (Enum -> Enum)
          u.user_country, 
          u.user_birthday,
          u.user_gender,    -- [수정] ::text 제거 (Enum -> Enum)
          u.user_terms_service, 
          u.user_terms_personal,
          u.user_created_at, 
          u.user_quit_at, 
          now()
        FROM public.users u
        WHERE u.user_id = $4
        "#,
    )
    .bind(actor_user_id)
    .bind(action)
    .bind(success)
    .bind(user_id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

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
