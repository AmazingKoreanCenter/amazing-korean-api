use serde_json::Value;
use sqlx::{PgPool, Postgres, QueryBuilder, Row, Transaction};

use crate::api::admin::lesson::dto::{
    AdminLessonItemDetailRes, AdminLessonItemRes, AdminLessonProgressDetailRes,
    AdminLessonProgressRes, AdminLessonRes, LessonItemCreateReq, LessonItemStudyTaskDetail,
    LessonItemVideoDetail, LessonUpdateItem,
};
use crate::error::AppResult;
use crate::types::{LessonAccess, LessonState};

fn apply_lesson_filters<'a>(
    builder: &mut QueryBuilder<'a, Postgres>,
    search: Option<&'a String>,
    lesson_state: Option<LessonState>,
    lesson_access: Option<LessonAccess>,
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

    if let Some(state) = lesson_state {
        push_cond(builder);
        builder.push("lesson_state = ");
        builder.push_bind(state);
    }

    if let Some(access) = lesson_access {
        push_cond(builder);
        builder.push("lesson_access = ");
        builder.push_bind(access);
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
    lesson_state: Option<LessonState>,
    lesson_access: Option<LessonAccess>,
) -> AppResult<(i64, Vec<AdminLessonRes>)> {
    let search = q.map(|s| format!("%{}%", s));

    let mut count_builder = QueryBuilder::new("SELECT count(*) FROM lesson");
    apply_lesson_filters(&mut count_builder, search.as_ref(), lesson_state, lesson_access);

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
            lesson_state,
            lesson_access,
            lesson_created_at,
            lesson_updated_at
        FROM lesson
        "#,
    );

    apply_lesson_filters(&mut builder, search.as_ref(), lesson_state, lesson_access);

    builder.push(" ORDER BY ");
    let sort_col = match sort {
        "lesson_idx" | "idx" => "lesson_idx",
        "lesson_title" | "title" => "lesson_title",
        "lesson_state" | "state" => "lesson_state",
        "lesson_access" | "access" => "lesson_access",
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

pub async fn find_lesson_progress_tx(
    tx: &mut Transaction<'_, Postgres>,
    lesson_id: i32,
    user_id: i64,
) -> AppResult<Option<AdminLessonProgressRes>> {
    let row = sqlx::query_as::<_, AdminLessonProgressRes>(
        r#"
        SELECT
            lesson_id,
            user_id::bigint AS user_id,
            lesson_progress_percent,
            lesson_progress_last_item_seq,
            lesson_progress_last_progress_at
        FROM lesson_progress
        WHERE lesson_id = $1
          AND user_id = $2
        "#,
    )
    .bind(lesson_id)
    .bind(user_id)
    .fetch_optional(&mut **tx)
    .await?;

    Ok(row)
}

pub async fn update_lesson_progress_tx(
    tx: &mut Transaction<'_, Postgres>,
    lesson_id: i32,
    user_id: i64,
    lesson_progress_percent: Option<i32>,
    lesson_progress_last_item_seq: Option<i32>,
) -> AppResult<()> {
    let mut builder = QueryBuilder::<Postgres>::new("UPDATE lesson_progress SET ");
    let mut is_first = true;

    if let Some(percent) = lesson_progress_percent {
        if !is_first {
            builder.push(", ");
        }
        builder.push("lesson_progress_percent = ");
        builder.push_bind(percent);
        is_first = false;
    }

    if let Some(last_item_seq) = lesson_progress_last_item_seq {
        if !is_first {
            builder.push(", ");
        }
        builder.push("lesson_progress_last_item_seq = ");
        builder.push_bind(last_item_seq);
        is_first = false;
    }

    if is_first {
        return Ok(());
    }

    builder.push(", lesson_progress_last_progress_at = now()");
    builder.push(" WHERE lesson_id = ");
    builder.push_bind(lesson_id);
    builder.push(" AND user_id = ");
    builder.push_bind(user_id);

    builder.build().execute(&mut **tx).await?;

    Ok(())
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

/// Shift lesson items: all items with seq >= target_seq get incremented by 1
/// Updates from highest to lowest to avoid unique constraint violations
pub async fn shift_lesson_items_tx(
    tx: &mut Transaction<'_, Postgres>,
    lesson_id: i32,
    target_seq: i32,
) -> AppResult<i32> {
    // Get count of items that will be shifted
    let count = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM lesson_item
        WHERE lesson_id = $1
          AND lesson_item_seq >= $2
        "#,
    )
    .bind(lesson_id)
    .bind(target_seq)
    .fetch_one(&mut **tx)
    .await?;

    if count == 0 {
        return Ok(0);
    }

    // Shift items: update from highest seq to lowest to avoid unique constraint violations
    sqlx::query(
        r#"
        UPDATE lesson_item
        SET lesson_item_seq = lesson_item_seq + 1
        WHERE lesson_id = $1
          AND lesson_item_seq >= $2
        "#,
    )
    .bind(lesson_id)
    .bind(target_seq)
    .execute(&mut **tx)
    .await?;

    Ok(count as i32)
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

/// Update only the sequence of a lesson item (for atomic reordering)
pub async fn update_lesson_item_seq_tx(
    tx: &mut Transaction<'_, Postgres>,
    lesson_id: i32,
    current_seq: i32,
    new_seq: i32,
) -> AppResult<()> {
    sqlx::query(
        r#"
        UPDATE lesson_item
        SET lesson_item_seq = $3
        WHERE lesson_id = $1
          AND lesson_item_seq = $2
        "#,
    )
    .bind(lesson_id)
    .bind(current_seq)
    .bind(new_seq)
    .execute(&mut **tx)
    .await?;

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
    lesson_state: LessonState,
    lesson_access: LessonAccess,
) -> AppResult<AdminLessonRes> {
    let created = sqlx::query_as::<_, AdminLessonRes>(
        r#"
        INSERT INTO lesson (
            updated_by_user_id,
            lesson_idx,
            lesson_title,
            lesson_subtitle,
            lesson_description,
            lesson_state,
            lesson_access
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING
            lesson_id,
            updated_by_user_id,
            lesson_idx,
            lesson_title,
            lesson_subtitle,
            lesson_description,
            lesson_state,
            lesson_access,
            lesson_created_at,
            lesson_updated_at
        "#,
    )
    .bind(actor_user_id)
    .bind(lesson_idx)
    .bind(lesson_title)
    .bind(lesson_subtitle)
    .bind(lesson_description)
    .bind(lesson_state)
    .bind(lesson_access)
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
    lesson_state: LessonState,
    lesson_access: LessonAccess,
) -> AppResult<AdminLessonRes> {
    create_lesson(
        tx,
        actor_user_id,
        lesson_idx,
        lesson_title,
        lesson_subtitle,
        lesson_description,
        lesson_state,
        lesson_access,
    )
    .await
}

fn normalize_lesson_action(action: &str) -> &'static str {
    match action {
        "create" | "CREATE" | "create_lesson" | "CREATE_LESSON" => "create",
        "update" | "UPDATE" | "update_lesson" | "UPDATE_LESSON" => "update",
        "delete" | "DELETE" | "delete_lesson" | "DELETE_LESSON" => "delete",
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
            lesson_state,
            lesson_access,
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

    if let Some(state) = req.lesson_state {
        if !is_first {
            builder.push(", ");
        }
        builder.push("lesson_state = ");
        builder.push_bind(state);
        is_first = false;
    }

    if let Some(access) = req.lesson_access {
        if !is_first {
            builder.push(", ");
        }
        builder.push("lesson_access = ");
        builder.push_bind(access);
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

// ============================================
// 7-46: Lesson Detail
// ============================================

pub async fn find_lesson_by_id(pool: &PgPool, lesson_id: i32) -> AppResult<Option<AdminLessonRes>> {
    let row = sqlx::query_as::<_, AdminLessonRes>(
        r#"
        SELECT
            lesson_id,
            updated_by_user_id,
            lesson_idx,
            lesson_title,
            lesson_subtitle,
            lesson_description,
            lesson_state,
            lesson_access,
            lesson_created_at,
            lesson_updated_at
        FROM lesson
        WHERE lesson_id = $1
        "#,
    )
    .bind(lesson_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

// ============================================
// 7-52: Lesson Items Detail (with video/study_task)
// ============================================

#[derive(sqlx::FromRow)]
struct LessonItemWithDetailsRow {
    // lesson_item fields
    lesson_id: i32,
    lesson_item_seq: i32,
    lesson_item_kind: String,
    // video fields (prefixed) - from video, video_tag, video_tag_map, video_stat_daily
    v_video_id: Option<i32>,
    v_video_idx: Option<String>,
    v_video_tag_title: Option<String>,
    v_video_url_vimeo: Option<String>,
    v_video_tag_subtitle: Option<String>,
    v_video_views: Option<i64>,
    v_video_state: Option<String>,
    v_video_access: Option<String>,
    v_video_duration: Option<i32>,
    v_video_thumbnail: Option<String>,
    v_video_created_at: Option<chrono::DateTime<chrono::Utc>>,
    v_video_updated_at: Option<chrono::DateTime<chrono::Utc>>,
    // study_task fields (prefixed)
    st_study_task_id: Option<i32>,
    st_study_id: Option<i32>,
    st_study_task_kind: Option<String>,
    st_study_task_seq: Option<i32>,
    st_study_task_created_at: Option<chrono::DateTime<chrono::Utc>>,
    st_study_task_updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn get_lesson_items_with_details(
    pool: &PgPool,
    lesson_id: i32,
) -> AppResult<Vec<AdminLessonItemDetailRes>> {
    let rows = sqlx::query_as::<_, LessonItemWithDetailsRow>(
        r#"
        SELECT
            li.lesson_id,
            li.lesson_item_seq,
            li.lesson_item_kind::text AS lesson_item_kind,
            -- video fields (from video + video_tag via video_tag_map + stats)
            v.video_id AS v_video_id,
            v.video_idx AS v_video_idx,
            vt.video_tag_title AS v_video_tag_title,
            v.video_url_vimeo AS v_video_url_vimeo,
            vt.video_tag_subtitle AS v_video_tag_subtitle,
            COALESCE(vs.total_views, 0) AS v_video_views,
            v.video_state::text AS v_video_state,
            v.video_access::text AS v_video_access,
            v.video_duration AS v_video_duration,
            v.video_thumbnail AS v_video_thumbnail,
            v.video_created_at AS v_video_created_at,
            v.video_updated_at AS v_video_updated_at,
            -- study_task fields
            st.study_task_id AS st_study_task_id,
            st.study_id AS st_study_id,
            st.study_task_kind::text AS st_study_task_kind,
            st.study_task_seq AS st_study_task_seq,
            st.study_task_created_at AS st_study_task_created_at,
            st.study_task_updated_at AS st_study_task_updated_at
        FROM lesson_item li
        LEFT JOIN video v ON li.video_id = v.video_id
        LEFT JOIN video_tag_map vtm ON v.video_id = vtm.video_id
        LEFT JOIN video_tag vt ON vtm.video_tag_id = vt.video_tag_id
        LEFT JOIN (
            SELECT video_id, SUM(video_stat_views)::bigint AS total_views
            FROM video_stat_daily
            GROUP BY video_id
        ) vs ON v.video_id = vs.video_id
        LEFT JOIN study_task st ON li.study_task_id = st.study_task_id
        WHERE li.lesson_id = $1
        ORDER BY li.lesson_item_seq ASC
        "#,
    )
    .bind(lesson_id)
    .fetch_all(pool)
    .await?;

    let items = rows
        .into_iter()
        .map(|row| {
            let video = if let Some(vid) = row.v_video_id {
                Some(LessonItemVideoDetail {
                    video_id: vid,
                    video_idx: row.v_video_idx.unwrap_or_default(),
                    video_tag_title: row.v_video_tag_title,
                    video_url_vimeo: row.v_video_url_vimeo,
                    video_tag_subtitle: row.v_video_tag_subtitle,
                    video_views: row.v_video_views.unwrap_or(0),
                    video_state: row
                        .v_video_state
                        .as_deref()
                        .map(|s| match s {
                            "ready" => crate::types::VideoState::Ready,
                            "open" => crate::types::VideoState::Open,
                            "close" => crate::types::VideoState::Close,
                            _ => crate::types::VideoState::Ready,
                        })
                        .unwrap_or(crate::types::VideoState::Ready),
                    video_access: row
                        .v_video_access
                        .as_deref()
                        .map(|s| match s {
                            "public" => crate::types::VideoAccess::Public,
                            "paid" => crate::types::VideoAccess::Paid,
                            "private" => crate::types::VideoAccess::Private,
                            "promote" => crate::types::VideoAccess::Promote,
                            _ => crate::types::VideoAccess::Public,
                        })
                        .unwrap_or(crate::types::VideoAccess::Public),
                    video_duration: row.v_video_duration,
                    video_thumbnail: row.v_video_thumbnail,
                    video_created_at: row.v_video_created_at.unwrap_or_else(chrono::Utc::now),
                    video_updated_at: row.v_video_updated_at.unwrap_or_else(chrono::Utc::now),
                })
            } else {
                None
            };

            let study_task = if let Some(stid) = row.st_study_task_id {
                Some(LessonItemStudyTaskDetail {
                    study_task_id: stid,
                    study_id: row.st_study_id.unwrap_or(0),
                    study_task_kind: row.st_study_task_kind.unwrap_or_default(),
                    study_task_seq: row.st_study_task_seq.unwrap_or(1),
                    study_task_created_at: row.st_study_task_created_at.unwrap_or_else(chrono::Utc::now),
                    study_task_updated_at: row.st_study_task_updated_at.unwrap_or_else(chrono::Utc::now),
                })
            } else {
                None
            };

            AdminLessonItemDetailRes {
                lesson_id: row.lesson_id,
                lesson_item_seq: row.lesson_item_seq,
                lesson_item_kind: row.lesson_item_kind,
                video,
                study_task,
            }
        })
        .collect();

    Ok(items)
}

pub async fn count_lesson_items(pool: &PgPool, lesson_id: i32) -> AppResult<i64> {
    let count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*) FROM lesson_item WHERE lesson_id = $1
        "#,
    )
    .bind(lesson_id)
    .fetch_one(pool)
    .await?;

    Ok(count)
}

// ============================================
// 7-58: Lesson Progress Detail (with current item)
// ============================================

#[derive(sqlx::FromRow)]
struct LessonProgressWithItemRow {
    // lesson_progress fields
    lesson_id: i32,
    user_id: i64,
    lesson_progress_percent: i32,
    lesson_progress_last_item_seq: Option<i32>,
    lesson_progress_last_progress_at: Option<chrono::DateTime<chrono::Utc>>,
    // current item fields (from lesson_item)
    ci_lesson_id: Option<i32>,
    ci_lesson_item_seq: Option<i32>,
    ci_lesson_item_kind: Option<String>,
    ci_video_id: Option<i32>,
    ci_study_task_id: Option<i32>,
}

pub async fn get_lesson_progress_with_items(
    pool: &PgPool,
    lesson_id: i32,
) -> AppResult<Vec<AdminLessonProgressDetailRes>> {
    let rows = sqlx::query_as::<_, LessonProgressWithItemRow>(
        r#"
        SELECT
            lp.lesson_id,
            lp.user_id::bigint AS user_id,
            lp.lesson_progress_percent,
            lp.lesson_progress_last_item_seq,
            lp.lesson_progress_last_progress_at,
            -- current item fields
            li.lesson_id AS ci_lesson_id,
            li.lesson_item_seq AS ci_lesson_item_seq,
            li.lesson_item_kind::text AS ci_lesson_item_kind,
            li.video_id AS ci_video_id,
            li.study_task_id AS ci_study_task_id
        FROM lesson_progress lp
        LEFT JOIN lesson_item li
            ON lp.lesson_id = li.lesson_id
            AND lp.lesson_progress_last_item_seq = li.lesson_item_seq
        WHERE lp.lesson_id = $1
        ORDER BY lp.lesson_progress_last_progress_at DESC NULLS LAST
        "#,
    )
    .bind(lesson_id)
    .fetch_all(pool)
    .await?;

    let list = rows
        .into_iter()
        .map(|row| {
            let current_item = if let Some(ci_lesson_id) = row.ci_lesson_id {
                Some(AdminLessonItemRes {
                    lesson_id: ci_lesson_id,
                    lesson_item_seq: row.ci_lesson_item_seq.unwrap_or(1),
                    lesson_item_kind: row.ci_lesson_item_kind.unwrap_or_default(),
                    video_id: row.ci_video_id,
                    study_task_id: row.ci_study_task_id,
                })
            } else {
                None
            };

            AdminLessonProgressDetailRes {
                lesson_id: row.lesson_id,
                user_id: row.user_id,
                lesson_progress_percent: row.lesson_progress_percent,
                lesson_progress_last_item_seq: row.lesson_progress_last_item_seq,
                lesson_progress_last_progress_at: row.lesson_progress_last_progress_at,
                current_item,
            }
        })
        .collect();

    Ok(list)
}

pub async fn count_lesson_progress(pool: &PgPool, lesson_id: i32) -> AppResult<i64> {
    let count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*) FROM lesson_progress WHERE lesson_id = $1
        "#,
    )
    .bind(lesson_id)
    .fetch_one(pool)
    .await?;

    Ok(count)
}

// ============================================
// DELETE: Lesson Item
// ============================================

pub async fn delete_lesson_item_tx(
    tx: &mut Transaction<'_, Postgres>,
    lesson_id: i32,
    lesson_item_seq: i32,
) -> AppResult<()> {
    sqlx::query(
        r#"
        DELETE FROM lesson_item
        WHERE lesson_id = $1
          AND lesson_item_seq = $2
        "#,
    )
    .bind(lesson_id)
    .bind(lesson_item_seq)
    .execute(&mut **tx)
    .await?;

    Ok(())
}
