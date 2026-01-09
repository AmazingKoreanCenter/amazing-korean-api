use serde_json::Value;
use sqlx::{PgPool, Postgres, QueryBuilder, Row, Transaction};

use crate::api::admin::lesson::dto::{
    AdminLessonItemRes, AdminLessonProgressRes, AdminLessonRes, LessonItemCreateReq,
    LessonUpdateItem,
};
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

fn apply_lesson_item_filters<'a>(
    builder: &mut QueryBuilder<'a, Postgres>,
    lesson_id: Option<i32>,
    lesson_item_kind: Option<&'a str>,
) {
    let mut has_where = false;

    let mut push_cond = |builder: &mut QueryBuilder<'a, Postgres>| {
        if !has_where {
            builder.push(" WHERE ");
            has_where = true;
        } else {
            builder.push(" AND ");
        }
    };

    if let Some(lesson_id) = lesson_id {
        push_cond(builder);
        builder.push("lesson_id = ");
        builder.push_bind(lesson_id);
    }

    if let Some(lesson_item_kind) = lesson_item_kind {
        push_cond(builder);
        builder.push("lesson_item_kind = ");
        builder.push_bind(lesson_item_kind);
        builder.push("::lesson_item_kind_enum");
    }
}

fn apply_lesson_progress_filters<'a>(
    builder: &mut QueryBuilder<'a, Postgres>,
    lesson_id: Option<i32>,
    user_id: Option<i64>,
) {
    let mut has_where = false;

    let mut push_cond = |builder: &mut QueryBuilder<'a, Postgres>| {
        if !has_where {
            builder.push(" WHERE ");
            has_where = true;
        } else {
            builder.push(" AND ");
        }
    };

    if let Some(lesson_id) = lesson_id {
        push_cond(builder);
        builder.push("lesson_id = ");
        builder.push_bind(lesson_id);
    }

    if let Some(user_id) = user_id {
        push_cond(builder);
        builder.push("user_id = ");
        builder.push_bind(user_id);
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

pub async fn admin_list_lesson_items(
    pool: &PgPool,
    lesson_id: Option<i32>,
    lesson_item_kind: Option<&str>,
    page: u64,
    size: u64,
    sort: &str,
    order: &str,
) -> AppResult<(i64, Vec<AdminLessonItemRes>)> {
    let mut count_builder = QueryBuilder::new("SELECT COUNT(*) FROM lesson_item");
    apply_lesson_item_filters(&mut count_builder, lesson_id, lesson_item_kind);

    let total_count: i64 = count_builder
        .build_query_scalar()
        .fetch_one(pool)
        .await?;

    let mut builder = QueryBuilder::new(
        r#"
        SELECT
            lesson_id,
            lesson_item_seq,
            lesson_item_kind::text AS lesson_item_kind,
            video_id,
            study_task_id
        FROM lesson_item
        "#,
    );
    apply_lesson_item_filters(&mut builder, lesson_id, lesson_item_kind);

    builder.push(" ORDER BY ");
    let sort_col = match sort {
        "lesson_id" => "lesson_id",
        "lesson_item_seq" => "lesson_item_seq",
        "lesson_item_kind" => "lesson_item_kind",
        "video_id" => "video_id",
        "study_task_id" => "study_task_id",
        _ => "lesson_id",
    };
    builder.push(sort_col);
    builder.push(if order == "desc" { " DESC" } else { " ASC" });

    builder.push(" LIMIT ");
    builder.push_bind(size as i64);
    builder.push(" OFFSET ");
    builder.push_bind(((page - 1) * size) as i64);

    let rows = builder
        .build_query_as::<AdminLessonItemRes>()
        .fetch_all(pool)
        .await?;

    Ok((total_count, rows))
}

pub async fn admin_list_lesson_progress(
    pool: &PgPool,
    lesson_id: Option<i32>,
    user_id: Option<i64>,
    page: u64,
    size: u64,
    sort: &str,
    order: &str,
) -> AppResult<(i64, Vec<AdminLessonProgressRes>)> {
    let mut count_builder = QueryBuilder::new("SELECT COUNT(*) FROM lesson_progress");
    apply_lesson_progress_filters(&mut count_builder, lesson_id, user_id);

    let total_count: i64 = count_builder
        .build_query_scalar()
        .fetch_one(pool)
        .await?;

    let mut builder = QueryBuilder::new(
        r#"
        SELECT
            lesson_id,
            user_id::bigint AS user_id,
            lesson_progress_percent,
            lesson_progress_last_item_seq,
            lesson_progress_last_progress_at
        FROM lesson_progress
        "#,
    );

    apply_lesson_progress_filters(&mut builder, lesson_id, user_id);

    builder.push(" ORDER BY ");
    let sort_col = match sort {
        "lesson_id" => "lesson_id",
        "user_id" => "user_id",
        "lesson_progress_percent" | "percent" => "lesson_progress_percent",
        "lesson_progress_last_item_seq" | "last_item_seq" => "lesson_progress_last_item_seq",
        "lesson_progress_last_progress_at" | "last_progress_at" | "updated_at" => {
            "lesson_progress_last_progress_at"
        }
        _ => "lesson_progress_last_progress_at",
    };
    builder.push(sort_col);
    builder.push(if order == "asc" { " ASC" } else { " DESC" });

    builder.push(" LIMIT ");
    builder.push_bind(size as i64);
    builder.push(" OFFSET ");
    builder.push_bind(((page - 1) * size) as i64);

    let rows = builder
        .build_query_as::<AdminLessonProgressRes>()
        .fetch_all(pool)
        .await?;

    Ok((total_count, rows))
}

pub async fn exists_lesson(pool: &PgPool, lesson_id: i32) -> AppResult<bool> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM lesson
            WHERE lesson_id = $1
        )
        "#,
    )
    .bind(lesson_id)
    .fetch_one(pool)
    .await?;

    Ok(exists)
}

pub async fn exists_lesson_tx(
    tx: &mut Transaction<'_, Postgres>,
    lesson_id: i32,
) -> AppResult<bool> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM lesson
            WHERE lesson_id = $1
        )
        "#,
    )
    .bind(lesson_id)
    .fetch_one(&mut **tx)
    .await?;

    Ok(exists)
}

pub async fn exists_lesson_item(
    pool: &PgPool,
    lesson_id: i32,
    lesson_item_seq: i32,
) -> AppResult<bool> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM lesson_item
            WHERE lesson_id = $1
              AND lesson_item_seq = $2
        )
        "#,
    )
    .bind(lesson_id)
    .bind(lesson_item_seq)
    .fetch_one(pool)
    .await?;

    Ok(exists)
}

pub async fn exists_lesson_item_tx(
    tx: &mut Transaction<'_, Postgres>,
    lesson_id: i32,
    lesson_item_seq: i32,
) -> AppResult<bool> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM lesson_item
            WHERE lesson_id = $1
              AND lesson_item_seq = $2
        )
        "#,
    )
    .bind(lesson_id)
    .bind(lesson_item_seq)
    .fetch_one(&mut **tx)
    .await?;

    Ok(exists)
}

pub async fn find_lesson_item_tx(
    tx: &mut Transaction<'_, Postgres>,
    lesson_id: i32,
    lesson_item_seq: i32,
) -> AppResult<Option<AdminLessonItemRes>> {
    let row = sqlx::query_as::<_, AdminLessonItemRes>(
        r#"
        SELECT
            lesson_id,
            lesson_item_seq,
            lesson_item_kind::text AS lesson_item_kind,
            video_id,
            study_task_id
        FROM lesson_item
        WHERE lesson_id = $1
          AND lesson_item_seq = $2
        "#,
    )
    .bind(lesson_id)
    .bind(lesson_item_seq)
    .fetch_optional(&mut **tx)
    .await?;

    Ok(row)
}

pub async fn create_lesson_item(
    tx: &mut Transaction<'_, Postgres>,
    lesson_id: i32,
    kind: &str,
    video_id: Option<i32>,
    study_task_id: Option<i32>,
    req: &LessonItemCreateReq,
) -> AppResult<AdminLessonItemRes> {
    create_lesson_item_tx(
        tx,
        lesson_id,
        req.lesson_item_seq,
        kind,
        video_id,
        study_task_id,
    )
    .await
}

pub async fn create_lesson_item_tx(
    tx: &mut Transaction<'_, Postgres>,
    lesson_id: i32,
    lesson_item_seq: i32,
    kind: &str,
    video_id: Option<i32>,
    study_task_id: Option<i32>,
) -> AppResult<AdminLessonItemRes> {
    let created = sqlx::query_as::<_, AdminLessonItemRes>(
        r#"
        INSERT INTO lesson_item (
            lesson_id,
            lesson_item_seq,
            lesson_item_kind,
            video_id,
            study_task_id
        )
        VALUES ($1, $2, $3::lesson_item_kind_enum, $4, $5)
        RETURNING
            lesson_id,
            lesson_item_seq,
            lesson_item_kind::text AS lesson_item_kind,
            video_id,
            study_task_id
        "#,
    )
    .bind(lesson_id)
    .bind(lesson_item_seq)
    .bind(kind)
    .bind(video_id)
    .bind(study_task_id)
    .fetch_one(&mut **tx)
    .await?;

    Ok(created)
}

pub async fn update_lesson_item_tx(
    tx: &mut Transaction<'_, Postgres>,
    lesson_id: i32,
    current_seq: i32,
    new_seq: Option<i32>,
    lesson_item_kind: Option<&str>,
    video_id: Option<Option<i32>>,
    study_task_id: Option<Option<i32>>,
) -> AppResult<()> {
    let mut builder = QueryBuilder::<Postgres>::new("UPDATE lesson_item SET ");
    let mut is_first = true;

    if let Some(new_seq) = new_seq {
        if !is_first {
            builder.push(", ");
        }
        builder.push("lesson_item_seq = ");
        builder.push_bind(new_seq);
        is_first = false;
    }

    if let Some(kind) = lesson_item_kind {
        if !is_first {
            builder.push(", ");
        }
        builder.push("lesson_item_kind = ");
        builder.push_bind(kind);
        builder.push("::lesson_item_kind_enum");
        is_first = false;
    }

    if let Some(video_id) = video_id {
        if !is_first {
            builder.push(", ");
        }
        builder.push("video_id = ");
        if let Some(video_id) = video_id {
            builder.push_bind(video_id);
        } else {
            builder.push("NULL");
        }
        is_first = false;
    }

    if let Some(study_task_id) = study_task_id {
        if !is_first {
            builder.push(", ");
        }
        builder.push("study_task_id = ");
        if let Some(study_task_id) = study_task_id {
            builder.push_bind(study_task_id);
        } else {
            builder.push("NULL");
        }
        is_first = false;
    }

    if is_first {
        return Ok(());
    }

    builder.push(" WHERE lesson_id = ");
    builder.push_bind(lesson_id);
    builder.push(" AND lesson_item_seq = ");
    builder.push_bind(current_seq);

    builder.build().execute(&mut **tx).await?;

    Ok(())
}

pub async fn exists_lesson_idx(pool: &PgPool, lesson_idx: &str) -> AppResult<bool> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM lesson
            WHERE lesson_idx = $1
        )
        "#,
    )
    .bind(lesson_idx)
    .fetch_one(pool)
    .await?;

    Ok(exists)
}

pub async fn exists_lesson_idx_tx(
    tx: &mut Transaction<'_, Postgres>,
    lesson_idx: &str,
) -> AppResult<bool> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM lesson
            WHERE lesson_idx = $1
        )
        "#,
    )
    .bind(lesson_idx)
    .fetch_one(&mut **tx)
    .await?;

    Ok(exists)
}

pub async fn create_lesson(
    tx: &mut Transaction<'_, Postgres>,
    actor_user_id: i64,
    lesson_idx: &str,
    lesson_title: &str,
    lesson_subtitle: Option<&str>,
    lesson_description: Option<&str>,
) -> AppResult<AdminLessonRes> {
    let created = sqlx::query_as::<_, AdminLessonRes>(
        r#"
        INSERT INTO lesson (
            updated_by_user_id,
            lesson_idx,
            lesson_title,
            lesson_subtitle,
            lesson_description
        )
        VALUES ($1, $2, $3, $4, $5)
        RETURNING
            lesson_id,
            updated_by_user_id,
            lesson_idx,
            lesson_title,
            lesson_subtitle,
            lesson_description,
            lesson_created_at,
            lesson_updated_at
        "#,
    )
    .bind(actor_user_id)
    .bind(lesson_idx)
    .bind(lesson_title)
    .bind(lesson_subtitle)
    .bind(lesson_description)
    .fetch_one(&mut **tx)
    .await?;

    Ok(created)
}

pub async fn create_lesson_tx(
    tx: &mut Transaction<'_, Postgres>,
    actor_user_id: i64,
    lesson_idx: &str,
    lesson_title: &str,
    lesson_subtitle: Option<&str>,
    lesson_description: Option<&str>,
) -> AppResult<AdminLessonRes> {
    create_lesson(
        tx,
        actor_user_id,
        lesson_idx,
        lesson_title,
        lesson_subtitle,
        lesson_description,
    )
    .await
}

fn normalize_lesson_action(action: &str) -> &'static str {
    match action {
        "create" | "CREATE" | "create_lesson" | "CREATE_LESSON" => "create",
        "update" | "UPDATE" | "update_lesson" | "UPDATE_LESSON" => "update",
        _ => "update",
    }
}

pub async fn create_lesson_log(
    tx: &mut Transaction<'_, Postgres>,
    admin_user_id: i64,
    action: &str,
    lesson_id: i32,
    lesson_item_seq: Option<i32>,
    video_id: Option<i32>,
    task_id: Option<i32>,
    before: Option<&Value>,
    after: Option<&Value>,
) -> AppResult<()> {
    sqlx::query(
        r#"
        INSERT INTO admin_lesson_log (
            admin_user_id,
            admin_pick_lesson_id,
            admin_pick_item_seq,
            admin_pick_video_id,
            admin_pick_task_id,
            admin_lesson_action,
            admin_lesson_before,
            admin_lesson_after
        )
        VALUES ($1, $2, $3, $4, $5, CAST($6 AS admin_action_enum), $7, $8)
        "#,
    )
    .bind(admin_user_id)
    .bind(lesson_id)
    .bind(lesson_item_seq)
    .bind(video_id)
    .bind(task_id)
    .bind(normalize_lesson_action(action))
    .bind(before)
    .bind(after)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn create_lesson_log_tx(
    tx: &mut Transaction<'_, Postgres>,
    admin_user_id: i64,
    action: &str,
    lesson_id: i32,
    lesson_item_seq: Option<i32>,
    video_id: Option<i32>,
    task_id: Option<i32>,
    before: Option<&Value>,
    after: Option<&Value>,
) -> AppResult<()> {
    create_lesson_log(
        tx,
        admin_user_id,
        action,
        lesson_id,
        lesson_item_seq,
        video_id,
        task_id,
        before,
        after,
    )
    .await
}

pub async fn find_lesson_by_id_tx(
    tx: &mut Transaction<'_, Postgres>,
    lesson_id: i32,
) -> AppResult<Option<AdminLessonRes>> {
    let row = sqlx::query_as::<_, AdminLessonRes>(
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
        WHERE lesson_id = $1
        "#,
    )
    .bind(lesson_id)
    .fetch_optional(&mut **tx)
    .await?;

    Ok(row)
}

pub async fn exists_lesson_idx_excluding_id_tx(
    tx: &mut Transaction<'_, Postgres>,
    lesson_idx: &str,
    lesson_id: i32,
) -> AppResult<bool> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM lesson
            WHERE lesson_idx = $1
              AND lesson_id != $2
        )
        "#,
    )
    .bind(lesson_idx)
    .bind(lesson_id)
    .fetch_one(&mut **tx)
    .await?;

    Ok(exists)
}

pub async fn update_lesson_tx(
    tx: &mut Transaction<'_, Postgres>,
    actor_user_id: i64,
    lesson_id: i32,
    req: &LessonUpdateItem,
) -> AppResult<()> {
    let mut builder = QueryBuilder::<Postgres>::new("UPDATE lesson SET ");
    let mut is_first = true;

    if let Some(ref lesson_idx) = req.lesson_idx {
        if !is_first {
            builder.push(", ");
        }
        builder.push("lesson_idx = ");
        builder.push_bind(lesson_idx);
        is_first = false;
    }

    if let Some(ref lesson_title) = req.lesson_title {
        if !is_first {
            builder.push(", ");
        }
        builder.push("lesson_title = ");
        builder.push_bind(lesson_title);
        is_first = false;
    }

    if let Some(ref lesson_subtitle) = req.lesson_subtitle {
        if !is_first {
            builder.push(", ");
        }
        builder.push("lesson_subtitle = ");
        builder.push_bind(lesson_subtitle);
        is_first = false;
    }

    if let Some(ref lesson_description) = req.lesson_description {
        if !is_first {
            builder.push(", ");
        }
        builder.push("lesson_description = ");
        builder.push_bind(lesson_description);
        is_first = false;
    }

    if !is_first {
        builder.push(", ");
    }
    builder.push("updated_by_user_id = ");
    builder.push_bind(actor_user_id);
    builder.push(", lesson_updated_at = now()");

    builder.push(" WHERE lesson_id = ");
    builder.push_bind(lesson_id);

    builder.build().execute(&mut **tx).await?;

    Ok(())
}
