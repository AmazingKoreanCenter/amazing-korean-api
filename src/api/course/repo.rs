use super::dto::CourseListItem;
use crate::error::AppResult;
use sqlx::{FromRow, PgPool};

#[derive(FromRow)]
struct InsertedId {
    course_id: i64,
}

pub async fn list(pool: &PgPool) -> AppResult<Vec<CourseListItem>> {
    let rows = sqlx::query_as::<_, CourseListItem>(
        r#"SELECT course_id, course_title, course_subtitle, course_price, course_type, course_state
           FROM course
           WHERE course_state <> 'deleted'
           ORDER BY course_id DESC"#,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn find_by_id(pool: &PgPool, id: i64) -> AppResult<Option<CourseListItem>> {
    let row = sqlx::query_as::<_, CourseListItem>(
        r#"SELECT course_id, course_title, course_subtitle, course_price, course_type, course_state
           FROM course
           WHERE course_id = $1"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn create(
    pool: &PgPool,
    title: &str,
    price: i32,
    ctype: &str,
    subtitle: Option<&str>,
) -> AppResult<i64> {
    let rec: InsertedId = sqlx::query_as(
        r#"INSERT INTO course (course_title, course_price, course_type, course_subtitle)
           VALUES ($1,$2,$3,$4)
           RETURNING course_id"#,
    )
    .bind(title)
    .bind(price)
    .bind(ctype)
    .bind(subtitle)
    .fetch_one(pool)
    .await?;
    Ok(rec.course_id)
}
