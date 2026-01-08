use sqlx::{PgPool, Postgres, QueryBuilder, Row};

use crate::api::admin::lesson::dto::AdminLessonRes;
use crate::error::AppResult;

fn apply_lesson_filters<'a>(builder: &mut QueryBuilder<'a, Postgres>, search: Option<&'a String>) {
    let mut has_where = false;

    let mut push_cond = |builder: &mut QueryBuilder<'a, Postgres>| {
        if !has_where {
            builder.push(" WHERE ");
            has_where = true;
        } else {
            builder.push(" AND ");
        }
    };

    if let Some(search) = search {
        push_cond(builder);
        builder.push("(");
        builder.push("lesson_title ILIKE ");
        builder.push_bind(search);
        builder.push(" OR lesson_subtitle ILIKE ");
        builder.push_bind(search);
        builder.push(" OR lesson_idx ILIKE ");
        builder.push_bind(search);
        builder.push(")");
    }
}

pub async fn admin_list_lessons(
    pool: &PgPool,
    q: Option<String>,
    page: u64,
    size: u64,
    sort: &str,
    order: &str,
) -> AppResult<(i64, Vec<AdminLessonRes>)> {
    let search = q.map(|s| format!("%{}%", s));

    let mut count_builder = QueryBuilder::new("SELECT count(*) FROM lesson");
    apply_lesson_filters(&mut count_builder, search.as_ref());

    let total_count: i64 = count_builder
        .build()
        .fetch_one(pool)
        .await?
        .try_get(0)?;

    let mut builder = QueryBuilder::new(
        r#"
        SELECT
            lesson_id,
            updated_by_user_id,
            lesson_idx,
            lesson_title,
            lesson_subtitle,
            lesson_description,
            lesson_created_at,
            lesson_updated_at
        FROM lesson
        "#,
    );

    apply_lesson_filters(&mut builder, search.as_ref());

    builder.push(" ORDER BY ");
    let sort_col = match sort {
        "lesson_idx" | "idx" => "lesson_idx",
        "lesson_title" | "title" => "lesson_title",
        "lesson_updated_at" | "updated_at" => "lesson_updated_at",
        "lesson_created_at" | "created_at" => "lesson_created_at",
        _ => "lesson_created_at",
    };
    builder.push(sort_col);
    builder.push(if order == "asc" { " ASC" } else { " DESC" });

    builder.push(" LIMIT ");
    builder.push_bind(size as i64);
    builder.push(" OFFSET ");
    builder.push_bind(((page - 1) * size) as i64);

    let rows = builder
        .build_query_as::<AdminLessonRes>()
        .fetch_all(pool)
        .await?;

    Ok((total_count, rows))
}
