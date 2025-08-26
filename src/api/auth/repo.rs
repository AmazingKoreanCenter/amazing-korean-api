use crate::error::AppResult;
use sqlx::PgPool;

#[derive(sqlx::FromRow)]
pub struct UserRow {
    pub user_id: i64,
    pub user_email: String,
    pub user_password: String,
    pub user_name: String,
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
