use serde_json::Value;
use sqlx::{PgPool, Postgres, QueryBuilder, Row, Transaction};
use crate::api::admin::study::dto::AdminStudyRes;
use crate::error::AppResult;
use crate::types::{StudyProgram, StudyState};

/// 동적 필터링 적용 헬퍼 함수
/// 라이프타임 'a를 추가하여 builder와 바인딩 데이터(search)의 수명을 일치시킵니다.
fn apply_filters<'a>(
    builder: &mut QueryBuilder<'a, Postgres>,
    search: Option<&'a String>,
    study_state: Option<StudyState>,
    study_program: Option<StudyProgram>,
) {
    let mut has_where = false;

    // WHERE 또는 AND를 상황에 맞게 붙여주는 클로저
    let mut push_cond = |builder: &mut QueryBuilder<'a, Postgres>| {
        if !has_where {
            builder.push(" WHERE ");
            has_where = true;
        } else {
            builder.push(" AND ");
        }
    };

    // 1. 검색어 필터 (Title OR Subtitle OR Idx)
    if let Some(search) = search {
        push_cond(builder);
        builder.push("(");
        builder.push("study_title ILIKE ");
        builder.push_bind(search);
        builder.push(" OR study_subtitle ILIKE ");
        builder.push_bind(search);
        builder.push(" OR study_idx ILIKE ");
        builder.push_bind(search);
        builder.push(")");
    }

    // 2. 상태 필터
    if let Some(state) = study_state {
        push_cond(builder);
        builder.push("study_state = ");
        builder.push_bind(state);
    }

    // 3. 프로그램 필터
    if let Some(program) = study_program {
        push_cond(builder);
        builder.push("study_program = ");
        builder.push_bind(program);
    }
}

pub async fn admin_list_studies(
    pool: &PgPool,
    q: Option<String>,
    page: u64,
    size: u64,
    sort: &str,
    order: &str,
    study_state: Option<StudyState>,
    study_program: Option<StudyProgram>,
) -> AppResult<(i64, Vec<AdminStudyRes>)> {
    // 검색어 포매팅 (%검색어%)
    let search = q.map(|s| format!("%{}%", s));

    // -------------------------------------------------------------------------
    // 1. Total Count Query
    // -------------------------------------------------------------------------
    let mut count_builder = QueryBuilder::new("SELECT count(*) FROM study");
    apply_filters(&mut count_builder, search.as_ref(), study_state, study_program);
    
    let total_count: i64 = count_builder
        .build()
        .fetch_one(pool)
        .await?
        .try_get(0)?;

    // -------------------------------------------------------------------------
    // 2. Data Select Query
    // -------------------------------------------------------------------------
    let mut builder = QueryBuilder::new(
        r#"
        SELECT 
            study_id, 
            study_idx, 
            study_title, 
            study_subtitle, 
            study_program, 
            study_state, 
            study_created_at, 
            study_updated_at 
        FROM study
        "#
    );
    
    apply_filters(&mut builder, search.as_ref(), study_state, study_program);

    // 정렬 (Sorting)
    builder.push(" ORDER BY ");
    let sort_col = match sort {
        "idx" => "study_idx",
        "title" => "study_title",
        "program" => "study_program",
        "state" => "study_state",
        "updated_at" => "study_updated_at",
        _ => "study_created_at", // 기본값
    };
    builder.push(sort_col);
    builder.push(if order == "asc" { " ASC" } else { " DESC" });

    // 페이지네이션 (Pagination)
    builder.push(" LIMIT ");
    builder.push_bind(size as i64);
    builder.push(" OFFSET ");
    builder.push_bind(((page - 1) * size) as i64);

    let rows = builder
        .build_query_as::<AdminStudyRes>()
        .fetch_all(pool)
        .await?;

    Ok((total_count, rows))
}

pub async fn exists_study_idx(pool: &PgPool, study_idx: &str) -> AppResult<bool> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM study
            WHERE study_idx = $1
        )
        "#,
    )
    .bind(study_idx)
    .fetch_one(pool)
    .await?;

    Ok(exists)
}

pub async fn admin_create_study(
    tx: &mut Transaction<'_, Postgres>,
    actor_user_id: i64,
    study_idx: &str,
    study_title: Option<&str>,
    study_subtitle: Option<&str>,
    study_description: Option<&str>,
    study_program: StudyProgram,
    study_state: StudyState,
) -> AppResult<AdminStudyRes> {
    let row = sqlx::query_as::<_, AdminStudyRes>(
        r#"
        INSERT INTO study (
            updated_by_user_id,
            study_idx,
            study_state,
            study_program,
            study_title,
            study_subtitle,
            study_description
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING
            study_id,
            study_idx,
            study_title,
            study_subtitle,
            study_program,
            study_state,
            study_created_at,
            study_updated_at
        "#,
    )
    .bind(actor_user_id)
    .bind(study_idx)
    .bind(study_state)
    .bind(study_program)
    .bind(study_title)
    .bind(study_subtitle)
    .bind(study_description)
    .fetch_one(&mut **tx)
    .await?;

    Ok(row)
}

fn normalize_study_action(action: &str) -> &'static str {
    match action {
        "create" | "CREATE" | "create_study" | "CREATE_STUDY" => "create",
        "update" | "UPDATE" => "update",
        "banned" | "BANNED" => "banned",
        "reorder" | "REORDER" => "reorder",
        "publish" | "PUBLISH" => "publish",
        "unpublish" | "UNPUBLISH" => "unpublish",
        _ => "update",
    }
}

pub async fn create_study_log(
    tx: &mut Transaction<'_, Postgres>,
    admin_user_id: i64,
    action: &str,
    target_study_id: i64,
    target_task_id: Option<i64>,
    before: Option<&Value>,
    after: Option<&Value>,
) -> AppResult<()> {
    sqlx::query(
        r#"
        INSERT INTO admin_study_log (
            admin_user_id,
            admin_pick_study_id,
            admin_pick_task_id,
            admin_study_action,
            admin_study_before,
            admin_study_after
        )
        VALUES ($1, $2, $3, CAST($4 AS admin_action_enum), $5, $6)
        "#,
    )
    .bind(admin_user_id)
    .bind(target_study_id)
    .bind(target_task_id)
    .bind(normalize_study_action(action))
    .bind(before)
    .bind(after)
    .execute(&mut **tx)
    .await?;

    Ok(())
}
