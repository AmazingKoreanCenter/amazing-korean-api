use crate::error::AppResult;
use chrono::NaiveDate;
use serde_json::Value;
use sqlx::{PgPool, Postgres, Transaction};
use std::net::IpAddr;

use crate::types::UserGender;
use super::dto::{AdminUpdateUserReq, AdminUserRes, AdminUserSummary};

pub async fn admin_list_users(
    pool: &PgPool,
    q: Option<&str>,
    page: i64,
    size: i64,
    sort: &str,
    order: &str,
) -> AppResult<(i64, Vec<AdminUserSummary>)> {
    let mut count_sql = String::from("SELECT COUNT(user_id) FROM users WHERE 1 = 1");
    let mut select_sql = String::from(
        r#"
        SELECT
            user_id as id,
            user_email as email,
            user_nickname as nickname,
            user_auth as role,
            user_created_at as created_at
        FROM users
        WHERE 1 = 1
        "#,
    );

    let mut query_args: Vec<String> = Vec::new();
    let mut bind_idx = 1;

    if let Some(keyword) = q {
        let search_query = format!("%{}%", keyword.to_lowercase());
        count_sql.push_str(&format!(
            "
            AND (LOWER(user_email) LIKE ${0} OR LOWER(user_nickname) LIKE ${0})
        ",
            bind_idx
        ));
        select_sql.push_str(&format!(
            "
            AND (LOWER(user_email) LIKE ${0} OR LOWER(user_nickname) LIKE ${0})
        ",
            bind_idx
        ));
        query_args.push(search_query);
        bind_idx += 1;
    }

    let sort_column = match sort {
        "created_at" => "user_created_at",
        "email" => "user_email",
        "nickname" => "user_nickname",
        _ => "user_created_at",
    };

    let order_dir = match order {
        "asc" => "ASC",
        _ => "DESC",
    };

    let total_query = sqlx::query_scalar::<_, i64>(&count_sql);
    let mut total_query = total_query;
    for arg in &query_args {
        total_query = total_query.bind(arg);
    }
    let total = total_query.fetch_one(pool).await?;

    select_sql.push_str(&format!(
        "
        ORDER BY {sort_column} {order_dir}
        LIMIT ${} OFFSET ${}
    ",
        bind_idx,
        bind_idx + 1
    ));

    let mut select_query = sqlx::query_as::<_, AdminUserSummary>(&select_sql);
    for arg in &query_args {
        select_query = select_query.bind(arg);
    }
    select_query = select_query.bind(size).bind((page - 1) * size);

    let items = select_query.fetch_all(pool).await?;

    Ok((total, items))
}

pub async fn admin_get_user(pool: &PgPool, user_id: i64) -> AppResult<Option<AdminUserRes>> {
    let user = sqlx::query_as::<_, AdminUserRes>(
        r#"
        SELECT
            user_id as id,
            user_email as email,
            user_name as name,
            user_nickname as nickname,
            user_language::TEXT as language,
            user_country::TEXT as country,
            user_birthday as birthday,
            user_gender as gender,
            user_state,
            user_auth,
            user_created_at as created_at,
            user_quit_at as quit_at
        FROM users
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn admin_create_user(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
    name: &str,
    nickname: &str,
    user_auth: &str,
    language: &str,
    country: &str,
    birthday: NaiveDate,
    gender: UserGender,
    terms_service: bool,
    terms_personal: bool,
    actor_user_id: i64,
    ip_address: Option<IpAddr>,
    user_agent: Option<&str>,
    audit: bool,
) -> AppResult<AdminUserRes> {
    let mut tx = pool.begin().await?;

    let user = sqlx::query_as::<_, AdminUserRes>(
        r#"
        INSERT INTO users (
            user_email,
            user_password,
            user_name,
            user_nickname,
            user_language,
            user_country,
            user_birthday,
            user_gender,
            user_terms_service,
            user_terms_personal,
            user_auth
        )
        VALUES (
            $1, $2, $3, $4,
            $5::user_language_enum,
            $6, $7,
            $8::user_gender_enum,
            $9, $10,
            $11::user_auth_enum
        )
        RETURNING
            user_id as id,
            user_email as email,
            user_name as name,
            user_nickname as nickname,
            user_language::TEXT as language,
            user_country::TEXT as country,
            user_birthday as birthday,
            user_gender as gender,
            user_state,
            user_auth,
            user_created_at as created_at,
            user_quit_at as quit_at
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
    .bind(user_auth)
    .fetch_one(&mut *tx)
    .await?;

    let after = serde_json::to_value(&user).unwrap_or_default();

    create_history_log(&mut tx, actor_user_id, user.id, "create", None, Some(&after)).await?;

    let details = serde_json::json!({
        "created_user_id": user.id,
        "email": user.email
    });

    if audit {
        create_audit_log_tx(
            &mut tx,
            actor_user_id,
            "CREATE_USER",
            Some("users"),
            Some(user.id),
            &details,
            ip_address,
            user_agent,
        )
        .await?;
    }

    tx.commit().await?;

    Ok(user)
}

pub async fn admin_update_user(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i64,
    req: &AdminUpdateUserReq,
    password_hash: Option<&str>,
) -> AppResult<AdminUserRes> {
    let updated = sqlx::query_as::<_, AdminUserRes>(
        r#"
        UPDATE users
        SET
            user_email = COALESCE($2, user_email),
            user_password = COALESCE($3, user_password),
            user_name = COALESCE($4, user_name),
            user_nickname = COALESCE($5, user_nickname),
            user_language = COALESCE($6::user_language_enum, user_language),
            user_country = COALESCE($7, user_country),
            user_birthday = COALESCE($8, user_birthday),
            user_gender = COALESCE($9::user_gender_enum, user_gender),
            user_state = COALESCE($10, user_state),
            user_auth = COALESCE($11::user_auth_enum, user_auth),
            user_quit_at = CASE
                -- $10(입력)가 FALSE(정지)이고, 기존이 TRUE(활성)면 -> 탈퇴일 기록
                WHEN $10 IS NOT NULL AND $10 IS FALSE AND user_state IS TRUE THEN NOW()
                -- $10(입력)가 TRUE(활성)이고, 기존이 FALSE(정지)면 -> 탈퇴일 초기화
                WHEN $10 IS NOT NULL AND $10 IS TRUE AND user_state IS FALSE THEN NULL
                ELSE user_quit_at
            END
        WHERE user_id = $1
        RETURNING
            user_id as id,
            user_email as email,
            user_name as name,
            user_nickname as nickname,
            user_language::TEXT as language,
            user_country::TEXT as country,
            user_birthday as birthday,
            user_gender as gender,
            user_state,
            user_auth,
            user_created_at as created_at,
            user_quit_at as quit_at
        "#,
    )
    .bind(user_id)
    .bind(req.email.as_ref().map(|e| e.to_lowercase()))
    .bind(password_hash)
    .bind(req.name.as_ref())
    .bind(req.nickname.as_ref())
    .bind(req.language.as_ref())
    .bind(req.country.as_ref())
    .bind(req.birthday)
    .bind(req.gender)
    .bind(req.user_state)
    .bind(req.user_auth.map(|a| a.to_string()))
    .fetch_one(&mut **tx)
    .await?;

    Ok(updated)
}

pub async fn exists_email(pool: &PgPool, email: &str) -> AppResult<bool> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM users
            WHERE user_email = $1
        )
        "#,
    )
    .bind(email)
    .fetch_one(pool)
    .await?;

    Ok(exists)
}

pub async fn create_audit_log(
    pool: &PgPool,
    admin_id: i64,
    action_type: &str,
    target_table: Option<&str>,
    target_id: Option<i64>,
    details: &Value,
    ip_address: Option<IpAddr>,
    user_agent: Option<&str>,
) -> AppResult<()> {
    sqlx::query(
        r#"
        INSERT INTO admin_action_log (
            admin_id, action_type, target_table, target_id,
            details, ip_address, user_agent
        )
        VALUES ($1, $2, $3, $4, $5, $6::inet, $7)
        "#,
    )
    .bind(admin_id)
    .bind(action_type)
    .bind(target_table)
    .bind(target_id)
    .bind(details)
    .bind(ip_address.map(|ip| ip.to_string()))
    .bind(user_agent)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_audit_log_tx(
    tx: &mut Transaction<'_, Postgres>,
    admin_id: i64,
    action_type: &str,
    target_table: Option<&str>,
    target_id: Option<i64>,
    details: &Value,
    ip_address: Option<IpAddr>,
    user_agent: Option<&str>,
) -> AppResult<()> {
    sqlx::query(
        r#"
        INSERT INTO admin_action_log (
            admin_id, action_type, target_table, target_id,
            details, ip_address, user_agent
        )
        VALUES ($1, $2, $3, $4, $5, $6::inet, $7)
        "#,
    )
    .bind(admin_id)
    .bind(action_type)
    .bind(target_table)
    .bind(target_id)
    .bind(details)
    .bind(ip_address.map(|ip| ip.to_string()))
    .bind(user_agent)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn create_history_log(
    tx: &mut Transaction<'_, Postgres>,
    admin_user_id: i64,
    target_user_id: i64,
    action: &str,
    before: Option<&Value>,
    after: Option<&Value>,
) -> AppResult<()> {
    sqlx::query(
        r#"
        INSERT INTO admin_users_log (
            admin_user_id,
            admin_pick_user_id,
            admin_user_action,
            admin_user_before,
            admin_user_after
        )
        VALUES ($1, $2, CAST($3 AS admin_action_enum), $4, $5)
        "#,
    )
    .bind(admin_user_id)
    .bind(target_user_id)
    .bind(action)
    .bind(before)
    .bind(after)
    .execute(&mut **tx)
    .await?;

    Ok(())
}
