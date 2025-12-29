use crate::{error::AppResult};
use serde_json::Value;
use sqlx::{PgConnection, PgPool};

use super::dto::{AdminUpdateUserReq, AdminUserRes};

pub async fn admin_list_users(
    pool: &PgPool,
    query: Option<&str>,
    state: Option<bool>,
    page: i64,
    size: i64,
) -> AppResult<(i64, Vec<AdminUserRes>)> {
    let mut count_sql = String::from("SELECT COUNT(user_id) FROM users WHERE 1 = 1");
    let mut select_sql = String::from(
        r#"
        SELECT
            user_id as id, user_email as email, user_name as name,
            user_nickname as nickname, user_language as language, user_country as country,
            user_birthday as birthday, user_gender as gender,
            user_state, user_auth, user_created_at as created_at, user_quit_at as quit_at
        FROM users
        WHERE 1 = 1
        "#,
    );

    let mut query_args: Vec<String> = Vec::new();
    let mut bind_idx = 1;

    if let Some(q) = query {
        let search_query = format!("%{}%", q.to_lowercase());
        count_sql.push_str(&format!("
            AND (LOWER(user_email) LIKE ${} OR LOWER(user_name) LIKE ${} OR LOWER(user_nickname) LIKE ${})
        ", bind_idx, bind_idx, bind_idx));
        select_sql.push_str(&format!("
            AND (LOWER(user_email) LIKE ${} OR LOWER(user_name) LIKE ${} OR LOWER(user_nickname) LIKE ${})
        ", bind_idx, bind_idx, bind_idx));
        query_args.push(search_query);
        bind_idx += 1;
    }

    if let Some(s) = state {
        let state_str = s.to_string();
        count_sql.push_str(&format!(
            "
            AND user_state = ${}
        ",
            bind_idx
        ));
        select_sql.push_str(&format!(
            "
            AND user_state = ${}
        ",
            bind_idx
        ));
        query_args.push(state_str);
        bind_idx += 1;
    }

    let total_query = sqlx::query_scalar::<_, i64>(&count_sql);
    let total = match query_args.len() {
        0 => total_query.fetch_one(pool).await?,
        1 => total_query.bind(&query_args[0]).fetch_one(pool).await?,
        2 => {
            total_query
                .bind(&query_args[0])
                .bind(&query_args[1])
                .fetch_one(pool)
                .await?
        }
        _ => unreachable!(), // Should not happen with current logic
    };

    select_sql.push_str(&format!(
        "
        ORDER BY user_created_at DESC
        LIMIT ${} OFFSET ${}
    ",
        bind_idx,
        bind_idx + 1
    ));

    let mut select_query = sqlx::query_as::<_, AdminUserRes>(&select_sql);
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
            user_id as id, user_email as email, user_name as name,
            user_nickname as nickname, user_language as language, user_country as country,
            user_birthday as birthday, user_gender as gender,
            user_state, user_auth, user_created_at as created_at, user_quit_at as quit_at
        FROM users
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn admin_update_user(
    pool: &PgPool,
    actor_user_id: i64,
    user_id: i64,
    req: &AdminUpdateUserReq,
) -> AppResult<AdminUserRes> {
    let mut tx = pool.begin().await?;

    // Fetch before snapshot for logging
    let before_snap = sqlx::query_as::<_, AdminUserRes>(
        r#"
        SELECT
            user_id as id, user_email as email, user_name as name,
            user_nickname as nickname, user_language as language, user_country as country,
            user_birthday as birthday, user_gender as gender,
            user_state, user_auth, user_created_at as created_at, user_quit_at as quit_at
        FROM users
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_one(&mut *tx)
    .await?;

    let res = sqlx::query_as::<_, AdminUserRes>(
        r#"
        UPDATE users
        SET
            user_email = COALESCE($2, user_email),
            user_name = COALESCE($3, user_name),
            user_nickname = COALESCE($4, user_nickname),
            user_language = COALESCE($5, user_language),
            user_country = COALESCE($6, user_country),
            user_birthday = COALESCE($7, user_birthday),
            user_gender = COALESCE($8, user_gender),
            user_state = COALESCE($9, user_state),
            user_auth = COALESCE($10, user_auth),
            user_quit_at = CASE
                WHEN $9 = 'off' AND user_state = 'on' THEN NOW()
                WHEN $9 = 'on' AND user_state = 'off' THEN NULL
                ELSE user_quit_at
            END
        WHERE user_id = $1
        RETURNING
            user_id as id, user_email as email, user_name as name,
            user_nickname as nickname, user_language as language, user_country as country,
            user_birthday as birthday, user_gender as gender,
            user_state, user_auth, user_created_at as created_at, user_quit_at as quit_at
        "#,
    )
    .bind(user_id)
    .bind(req.email.as_ref().map(|e| e.to_lowercase()))
    .bind(req.name.as_ref())
    .bind(req.nickname.as_ref())
    .bind(req.language.as_ref())
    .bind(req.country.as_ref())
    .bind(req.birthday)
    .bind(req.gender.as_ref())
    .bind(req.user_state.map(|s| s.to_string()))
    .bind(req.user_auth.map(|a| a.to_string()))
    .fetch_one(&mut *tx)
    .await?;

    // Insert admin action log
    insert_admin_action_log(
        &mut tx,
        Some(actor_user_id), // actor_user_id will be set in service layer
        user_id,
        "admin_update",
        &serde_json::to_value(&before_snap).unwrap_or_default(),
        &serde_json::to_value(&res).unwrap_or_default(),
    )
    .await?;

    tx.commit().await?;

    Ok(res)
}

pub async fn insert_admin_action_log(
    conn: &mut PgConnection,
    actor_user_id: Option<i64>,
    target_user_id: i64,
    action: &str,
    before: &Value,
    after: &Value,
) -> AppResult<()> {
    sqlx::query(
        r#"
        INSERT INTO admin_user_action_log (
            actor_user_id, target_user_id, action, before, after
        )
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(actor_user_id)
    .bind(target_user_id)
    .bind(action)
    .bind(before)
    .bind(after)
    .execute(conn)
    .await?;

    Ok(())
}
