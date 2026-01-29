use serde_json::Value;
use sqlx::{PgPool, Postgres, QueryBuilder, Row, Transaction};
use crate::api::admin::study::dto::{
    AdminStudyDetailRes, AdminStudyRes, AdminStudyTaskDetailRes, AdminStudyTaskRes,
    AdminTaskExplainRes, AdminTaskStatusRes, StudyTaskCreateReq, StudyTaskUpdateReq,
    StudyUpdateReq, TaskExplainCreateReq, TaskExplainUpdateReq, TaskStatusUpdateReq,
};
use crate::error::AppResult;
use crate::types::{StudyAccess, StudyProgram, StudyState, UserSetLanguage};

/// 동적 필터링 적용 헬퍼 함수
/// 라이프타임 'a를 추가하여 builder와 바인딩 데이터(search)의 수명을 일치시킵니다.
fn apply_filters<'a>(
    builder: &mut QueryBuilder<'a, Postgres>,
    search: Option<&'a String>,
    study_state: Option<StudyState>,
    study_access: Option<StudyAccess>,
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

    // 3. 접근 권한 필터
    if let Some(access) = study_access {
        push_cond(builder);
        builder.push("study_access = ");
        builder.push_bind(access);
    }

    // 4. 프로그램 필터
    if let Some(program) = study_program {
        push_cond(builder);
        builder.push("study_program = ");
        builder.push_bind(program);
    }
}

fn apply_task_status_filters<'a>(
    builder: &mut QueryBuilder<'a, Postgres>,
    task_id: Option<i32>,
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

    if let Some(task_id) = task_id {
        push_cond(builder);
        builder.push("study_task_id = ");
        builder.push_bind(task_id);
    }

    if let Some(user_id) = user_id {
        push_cond(builder);
        builder.push("user_id = ");
        builder.push_bind(user_id);
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
    study_access: Option<StudyAccess>,
    study_program: Option<StudyProgram>,
) -> AppResult<(i64, Vec<AdminStudyRes>)> {
    // 검색어 포매팅 (%검색어%)
    let search = q.map(|s| format!("%{}%", s));

    // -------------------------------------------------------------------------
    // 1. Total Count Query
    // -------------------------------------------------------------------------
    let mut count_builder = QueryBuilder::new("SELECT count(*) FROM study");
    apply_filters(&mut count_builder, search.as_ref(), study_state, study_access, study_program);

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
            study_access,
            study_created_at,
            study_updated_at
        FROM study
        "#
    );

    apply_filters(&mut builder, search.as_ref(), study_state, study_access, study_program);

    // 정렬 (Sorting)
    builder.push(" ORDER BY ");
    let sort_col = match sort {
        "study_id" => "study_id",
        "idx" | "study_idx" => "study_idx",
        "title" | "study_title" => "study_title",
        "subtitle" | "study_subtitle" => "study_subtitle",
        "program" | "study_program" => "study_program",
        "state" | "study_state" => "study_state",
        "access" | "study_access" => "study_access",
        "updated_at" | "study_updated_at" => "study_updated_at",
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

pub async fn admin_list_study_tasks(
    pool: &PgPool,
    study_id: i32,
    page: u64,
    size: u64,
) -> AppResult<(i64, Vec<AdminStudyTaskRes>)> {
    let mut count_builder = QueryBuilder::new(
        "SELECT COUNT(*) FROM study_task st WHERE st.study_id = ",
    );
    count_builder.push_bind(study_id);

    let total_count = count_builder
        .build_query_scalar::<i64>()
        .fetch_one(pool)
        .await?;

    let mut list_builder = QueryBuilder::new(
        r#"
        SELECT
            st.study_task_id::bigint AS study_task_id,
            st.study_task_kind AS study_task_kind,
            st.study_task_seq AS study_task_seq,
            COALESCE(
                c.study_task_choice_question,
                t.study_task_typing_question,
                v.study_task_voice_question
            ) AS question
        FROM study_task st
        LEFT JOIN study_task_choice c ON c.study_task_id = st.study_task_id
        LEFT JOIN study_task_typing t ON t.study_task_id = st.study_task_id
        LEFT JOIN study_task_voice v ON v.study_task_id = st.study_task_id
        WHERE st.study_id = 
        "#,
    );
    list_builder.push_bind(study_id);
    list_builder.push(" ORDER BY st.study_task_seq ASC");
    list_builder.push(" LIMIT ");
    list_builder.push_bind(size as i64);
    list_builder.push(" OFFSET ");
    list_builder.push_bind(((page - 1) * size) as i64);

    let rows = list_builder
        .build_query_as::<AdminStudyTaskRes>()
        .fetch_all(pool)
        .await?;

    Ok((total_count, rows))
}

pub async fn admin_list_task_explains(
    pool: &PgPool,
    task_id: i32,
    page: u64,
    size: u64,
) -> AppResult<(i64, Vec<AdminTaskExplainRes>)> {
    let total_count = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM study_task_explain
        WHERE study_task_id = $1
        "#,
    )
    .bind(task_id)
    .fetch_one(pool)
    .await?;

    let rows = sqlx::query_as::<_, AdminTaskExplainRes>(
        r#"
        SELECT
            study_task_id::bigint AS study_task_id,
            explain_lang,
            explain_title,
            explain_text,
            explain_media_url,
            explain_created_at,
            explain_updated_at
        FROM study_task_explain
        WHERE study_task_id = $1
        ORDER BY explain_lang ASC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(task_id)
    .bind(size as i64)
    .bind(((page - 1) * size) as i64)
    .fetch_all(pool)
    .await?;

    Ok((total_count, rows))
}

pub async fn admin_list_task_status(
    pool: &PgPool,
    task_id: Option<i32>,
    user_id: Option<i64>,
    page: u64,
    size: u64,
) -> AppResult<(i64, Vec<AdminTaskStatusRes>)> {
    let mut count_builder = QueryBuilder::new("SELECT COUNT(*) FROM study_task_status");
    apply_task_status_filters(&mut count_builder, task_id, user_id);

    let total_count: i64 = count_builder
        .build_query_scalar()
        .fetch_one(pool)
        .await?;

    let mut list_builder = QueryBuilder::new(
        r#"
        SELECT
            study_task_id::bigint AS study_task_id,
            user_id::bigint AS user_id,
            study_task_status_try_count,
            study_task_status_is_solved,
            study_task_status_last_attempt_at
        FROM study_task_status
        "#,
    );
    apply_task_status_filters(&mut list_builder, task_id, user_id);
    list_builder.push(" ORDER BY study_task_status_last_attempt_at DESC");
    list_builder.push(" LIMIT ");
    list_builder.push_bind(size as i64);
    list_builder.push(" OFFSET ");
    list_builder.push_bind(((page - 1) * size) as i64);

    let rows = list_builder
        .build_query_as::<AdminTaskStatusRes>()
        .fetch_all(pool)
        .await?;

    Ok((total_count, rows))
}

pub async fn find_task_status(
    pool: &PgPool,
    study_task_id: i32,
    user_id: i64,
) -> AppResult<Option<AdminTaskStatusRes>> {
    let row = sqlx::query_as::<_, AdminTaskStatusRes>(
        r#"
        SELECT
            study_task_id::bigint AS study_task_id,
            user_id::bigint AS user_id,
            study_task_status_try_count,
            study_task_status_is_solved,
            study_task_status_last_attempt_at
        FROM study_task_status
        WHERE study_task_id = $1
          AND user_id = $2
        "#,
    )
    .bind(study_task_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn find_task_status_tx(
    tx: &mut Transaction<'_, Postgres>,
    study_task_id: i32,
    user_id: i64,
) -> AppResult<Option<AdminTaskStatusRes>> {
    let row = sqlx::query_as::<_, AdminTaskStatusRes>(
        r#"
        SELECT
            study_task_id::bigint AS study_task_id,
            user_id::bigint AS user_id,
            study_task_status_try_count,
            study_task_status_is_solved,
            study_task_status_last_attempt_at
        FROM study_task_status
        WHERE study_task_id = $1
          AND user_id = $2
        "#,
    )
    .bind(study_task_id)
    .bind(user_id)
    .fetch_optional(&mut **tx)
    .await?;

    Ok(row)
}

pub async fn update_task_status(
    tx: &mut Transaction<'_, Postgres>,
    study_task_id: i32,
    req: &TaskStatusUpdateReq,
) -> AppResult<()> {
    let mut builder = QueryBuilder::<Postgres>::new("UPDATE study_task_status SET ");

    // ", "를 구분자로 설정합니다.
    // 첫 번째 항목에는 구분자가 안 붙고, 두 번째 항목부터 자동으로 앞에 ", "를 붙여줍니다.
    let mut separated = builder.separated(", ");

    if let Some(try_count) = req.study_task_status_try_count {
        separated.push("study_task_status_try_count = ");
        separated.push_bind_unseparated(try_count);
    }

    if let Some(is_solved) = req.study_task_status_is_solved {
        separated.push("study_task_status_is_solved = ");
        separated.push_bind_unseparated(is_solved);
    }

    if let Some(last_attempt_at) = req.study_task_status_last_attempt_at {
        separated.push("study_task_status_last_attempt_at = ");
        separated.push_bind_unseparated(last_attempt_at);
    }

    // 주의: 만약 업데이트할 필드가 하나도 없다면 SQL 에러가 날 수 있습니다.
    // (이전 코드와 동일한 잠재적 문제이나, 보통 DTO 검증 단계에서 막습니다)

    builder.push(" WHERE study_task_id = ");
    builder.push_bind(study_task_id);
    builder.push(" AND user_id = ");
    builder.push_bind(req.user_id);

    builder.build().execute(&mut **tx).await?;

    Ok(())
}

pub async fn exists_task_explain(
    pool: &PgPool,
    study_task_id: i32,
    explain_lang: UserSetLanguage,
) -> AppResult<bool> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM study_task_explain
            WHERE study_task_id = $1
              AND explain_lang = $2
        )
        "#,
    )
    .bind(study_task_id)
    .bind(explain_lang)
    .fetch_one(pool)
    .await?;

    Ok(exists)
}

pub async fn create_task_explain(
    tx: &mut Transaction<'_, Postgres>,
    study_task_id: i32,
    req: &TaskExplainCreateReq,
) -> AppResult<AdminTaskExplainRes> {
    let created = sqlx::query_as::<_, AdminTaskExplainRes>(
        r#"
        INSERT INTO study_task_explain (
            study_task_id,
            explain_lang,
            explain_title,
            explain_text,
            explain_media_url
        )
        VALUES ($1, $2, $3, $4, $5)
        RETURNING
            study_task_id::bigint AS study_task_id,
            explain_lang,
            explain_title,
            explain_text,
            explain_media_url,
            explain_created_at,
            explain_updated_at
        "#,
    )
    .bind(study_task_id)
    .bind(req.explain_lang)
    .bind(req.explain_title.as_deref())
    .bind(req.explain_text.as_deref())
    .bind(req.explain_media_url.as_deref())
    .fetch_one(&mut **tx)
    .await?;

    Ok(created)
}

pub async fn find_task_explain(
    pool: &PgPool,
    study_task_id: i32,
    explain_lang: UserSetLanguage,
) -> AppResult<Option<AdminTaskExplainRes>> {
    let row = sqlx::query_as::<_, AdminTaskExplainRes>(
        r#"
        SELECT
            study_task_id::bigint AS study_task_id,
            explain_lang,
            explain_title,
            explain_text,
            explain_media_url,
            explain_created_at,
            explain_updated_at
        FROM study_task_explain
        WHERE study_task_id = $1
          AND explain_lang = $2
        "#,
    )
    .bind(study_task_id)
    .bind(explain_lang)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn find_task_explain_tx(
    tx: &mut Transaction<'_, Postgres>,
    study_task_id: i32,
    explain_lang: UserSetLanguage,
) -> AppResult<Option<AdminTaskExplainRes>> {
    let row = sqlx::query_as::<_, AdminTaskExplainRes>(
        r#"
        SELECT
            study_task_id::bigint AS study_task_id,
            explain_lang,
            explain_title,
            explain_text,
            explain_media_url,
            explain_created_at,
            explain_updated_at
        FROM study_task_explain
        WHERE study_task_id = $1
          AND explain_lang = $2
        "#,
    )
    .bind(study_task_id)
    .bind(explain_lang)
    .fetch_optional(&mut **tx)
    .await?;

    Ok(row)
}

pub async fn update_task_explain(
    tx: &mut Transaction<'_, Postgres>,
    study_task_id: i32,
    req: &TaskExplainUpdateReq,
) -> AppResult<()> {
    let mut builder = QueryBuilder::<Postgres>::new("UPDATE study_task_explain SET ");
    let mut is_first = true;

    if let Some(ref title) = req.explain_title {
        if !is_first {
            builder.push(", ");
        }
        builder.push("explain_title = ");
        builder.push_bind(title);
        is_first = false;
    }

    if let Some(ref text) = req.explain_text {
        if !is_first {
            builder.push(", ");
        }
        builder.push("explain_text = ");
        builder.push_bind(text);
        is_first = false;
    }

    if let Some(ref media_url) = req.explain_media_url {
        if !is_first {
            builder.push(", ");
        }
        builder.push("explain_media_url = ");
        builder.push_bind(media_url);
        is_first = false;
    }

    if !is_first {
        builder.push(", ");
    }
    builder.push("explain_updated_at = now()");

    builder.push(" WHERE study_task_id = ");
    builder.push_bind(study_task_id);
    builder.push(" AND explain_lang = ");
    builder.push_bind(req.explain_lang);

    builder.build().execute(&mut **tx).await?;

    Ok(())
}

pub async fn find_study_task_by_id(
    pool: &PgPool,
    study_task_id: i64,
) -> AppResult<Option<AdminStudyTaskDetailRes>> {
    let task = sqlx::query_as::<_, AdminStudyTaskDetailRes>(
        r#"
        SELECT
            st.study_task_id::bigint AS study_task_id,
            st.study_id::bigint AS study_id,
            st.study_task_kind AS study_task_kind,
            st.study_task_seq AS study_task_seq,
            COALESCE(
                c.study_task_choice_question,
                t.study_task_typing_question,
                v.study_task_voice_question
            ) AS question,
            COALESCE(
                t.study_task_typing_answer,
                v.study_task_voice_answer
            ) AS answer,
            COALESCE(
                c.study_task_choice_image_url,
                t.study_task_typing_image_url,
                v.study_task_voice_image_url
            ) AS image_url,
            COALESCE(
                c.study_task_choice_audio_url,
                v.study_task_voice_audio_url
            ) AS audio_url,
            c.study_task_choice_1 AS choice_1,
            c.study_task_choice_2 AS choice_2,
            c.study_task_choice_3 AS choice_3,
            c.study_task_choice_4 AS choice_4,
            c.study_task_choice_answer AS choice_correct
        FROM study_task st
        LEFT JOIN study_task_choice c ON c.study_task_id = st.study_task_id
        LEFT JOIN study_task_typing t ON t.study_task_id = st.study_task_id
        LEFT JOIN study_task_voice v ON v.study_task_id = st.study_task_id
        WHERE st.study_task_id = $1
        "#,
    )
    .bind(study_task_id)
    .fetch_optional(pool)
    .await?;

    Ok(task)
}

pub async fn find_study_task_by_id_tx(
    tx: &mut Transaction<'_, Postgres>,
    study_task_id: i64,
) -> AppResult<AdminStudyTaskDetailRes> {
    let task = sqlx::query_as::<_, AdminStudyTaskDetailRes>(
        r#"
        SELECT
            st.study_task_id::bigint AS study_task_id,
            st.study_id::bigint AS study_id,
            st.study_task_kind AS study_task_kind,
            st.study_task_seq AS study_task_seq,
            COALESCE(
                c.study_task_choice_question,
                t.study_task_typing_question,
                v.study_task_voice_question
            ) AS question,
            COALESCE(
                t.study_task_typing_answer,
                v.study_task_voice_answer
            ) AS answer,
            COALESCE(
                c.study_task_choice_image_url,
                t.study_task_typing_image_url,
                v.study_task_voice_image_url
            ) AS image_url,
            COALESCE(
                c.study_task_choice_audio_url,
                v.study_task_voice_audio_url
            ) AS audio_url,
            c.study_task_choice_1 AS choice_1,
            c.study_task_choice_2 AS choice_2,
            c.study_task_choice_3 AS choice_3,
            c.study_task_choice_4 AS choice_4,
            c.study_task_choice_answer AS choice_correct
        FROM study_task st
        LEFT JOIN study_task_choice c ON c.study_task_id = st.study_task_id
        LEFT JOIN study_task_typing t ON t.study_task_id = st.study_task_id
        LEFT JOIN study_task_voice v ON v.study_task_id = st.study_task_id
        WHERE st.study_task_id = $1
        "#,
    )
    .bind(study_task_id)
    .fetch_one(&mut **tx)
    .await?;

    Ok(task)
}

pub async fn find_study_by_id(pool: &PgPool, study_id: i64) -> AppResult<Option<AdminStudyRes>> {
    let study = sqlx::query_as::<_, AdminStudyRes>(
        r#"
        SELECT
            study_id,
            study_idx,
            study_title,
            study_subtitle,
            study_program,
            study_state,
            study_access,
            study_created_at,
            study_updated_at
        FROM study
        WHERE study_id = $1
        "#,
    )
    .bind(study_id)
    .fetch_optional(pool)
    .await?;

    Ok(study)
}

/// Study 상세 조회 (tasks 포함)
pub async fn admin_get_study_detail(
    pool: &PgPool,
    study_id: i64,
) -> AppResult<Option<AdminStudyDetailRes>> {
    // 1. Study 기본 정보 조회 (description 포함)
    let study_row = sqlx::query(
        r#"
        SELECT
            study_id,
            study_idx,
            study_title,
            study_subtitle,
            study_description,
            study_program,
            study_state,
            study_access,
            study_created_at,
            study_updated_at
        FROM study
        WHERE study_id = $1
        "#,
    )
    .bind(study_id)
    .fetch_optional(pool)
    .await?;

    let study_row = match study_row {
        Some(row) => row,
        None => return Ok(None),
    };

    // 2. 연관 tasks 조회 (최대 100개)
    let (task_count, tasks) = admin_list_study_tasks(pool, study_id as i32, 1, 100).await?;

    // 3. AdminStudyDetailRes 구성
    Ok(Some(AdminStudyDetailRes {
        study_id: study_row.try_get("study_id")?,
        study_idx: study_row.try_get("study_idx")?,
        study_title: study_row.try_get("study_title")?,
        study_subtitle: study_row.try_get("study_subtitle")?,
        study_description: study_row.try_get("study_description")?,
        study_program: study_row.try_get("study_program")?,
        study_state: study_row.try_get("study_state")?,
        study_access: study_row.try_get("study_access")?,
        study_created_at: study_row.try_get("study_created_at")?,
        study_updated_at: study_row.try_get("study_updated_at")?,
        task_count,
        tasks,
    }))
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

pub async fn exists_study_idx_for_update(
    pool: &PgPool,
    study_id: i64,
    study_idx: &str,
) -> AppResult<bool> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM study
            WHERE study_idx = $1
              AND study_id <> $2
        )
        "#,
    )
    .bind(study_idx)
    .bind(study_id)
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
    study_access: StudyAccess,
) -> AppResult<AdminStudyRes> {
    let row = sqlx::query_as::<_, AdminStudyRes>(
        r#"
        INSERT INTO study (
            updated_by_user_id,
            study_idx,
            study_state,
            study_access,
            study_program,
            study_title,
            study_subtitle,
            study_description
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING
            study_id,
            study_idx,
            study_title,
            study_subtitle,
            study_program,
            study_state,
            study_access,
            study_created_at,
            study_updated_at
        "#,
    )
    .bind(actor_user_id)
    .bind(study_idx)
    .bind(study_state)
    .bind(study_access)
    .bind(study_program)
    .bind(study_title)
    .bind(study_subtitle)
    .bind(study_description)
    .fetch_one(&mut **tx)
    .await?;

    Ok(row)
}

pub async fn admin_update_study(
    tx: &mut Transaction<'_, Postgres>,
    study_id: i64,
    actor_user_id: i64,
    req: &StudyUpdateReq,
) -> AppResult<AdminStudyRes> {
    let mut builder = QueryBuilder::<Postgres>::new("UPDATE study SET ");
    let mut is_first = true;

    if let Some(ref idx) = req.study_idx {
        if !is_first {
            builder.push(", ");
        }
        builder.push("study_idx = ");
        builder.push_bind(idx);
        is_first = false;
    }

    if let Some(state) = req.study_state {
        if !is_first {
            builder.push(", ");
        }
        builder.push("study_state = ");
        builder.push_bind(state);
        is_first = false;
    }

    if let Some(program) = req.study_program {
        if !is_first {
            builder.push(", ");
        }
        builder.push("study_program = ");
        builder.push_bind(program);
        is_first = false;
    }

    if let Some(ref title) = req.study_title {
        if !is_first {
            builder.push(", ");
        }
        builder.push("study_title = ");
        builder.push_bind(title);
        is_first = false;
    }

    if let Some(ref subtitle) = req.study_subtitle {
        if !is_first {
            builder.push(", ");
        }
        builder.push("study_subtitle = ");
        builder.push_bind(subtitle);
        is_first = false;
    }

    if let Some(ref description) = req.study_description {
        if !is_first {
            builder.push(", ");
        }
        builder.push("study_description = ");
        builder.push_bind(description);
        is_first = false;
    }

    if let Some(access) = req.study_access {
        if !is_first {
            builder.push(", ");
        }
        builder.push("study_access = ");
        builder.push_bind(access);
        is_first = false;
    }

    if !is_first {
        builder.push(", ");
    }
    builder.push("updated_by_user_id = ");
    builder.push_bind(actor_user_id);
    builder.push(", study_updated_at = now()");

    builder.push(" WHERE study_id = ");
    builder.push_bind(study_id);
    builder.push(
        " RETURNING study_id, study_idx, study_title, study_subtitle, \
         study_program, study_state, study_access, study_created_at, study_updated_at",
    );

    let updated = builder
        .build_query_as::<AdminStudyRes>()
        .fetch_one(&mut **tx)
        .await?;

    Ok(updated)
}

pub async fn admin_update_study_task(
    tx: &mut Transaction<'_, Postgres>,
    study_task_id: i64,
    actor_user_id: i64,
    kind: crate::types::StudyTaskKind,
    req: &StudyTaskUpdateReq,
) -> AppResult<AdminStudyTaskDetailRes> {
    let mut builder = QueryBuilder::<Postgres>::new("UPDATE study_task SET ");
    let mut is_first = true;

    if let Some(seq) = req.study_task_seq {
        if !is_first {
            builder.push(", ");
        }
        builder.push("study_task_seq = ");
        builder.push_bind(seq);
        is_first = false;
    }

    if !is_first {
        builder.push(", ");
    }
    builder.push("updated_by_user_id = ");
    builder.push_bind(actor_user_id);
    builder.push(", study_task_updated_at = now()");

    builder.push(" WHERE study_task_id = ");
    builder.push_bind(study_task_id);

    builder.build().execute(&mut **tx).await?;

    match kind {
        crate::types::StudyTaskKind::Choice => {
            let mut qb = QueryBuilder::<Postgres>::new("UPDATE study_task_choice SET ");
            let mut has_any = false;

            if let Some(ref question) = req.question {
                if has_any { qb.push(", "); }
                qb.push("study_task_choice_question = ");
                qb.push_bind(question);
                has_any = true;
            }
            if let Some(ref choice) = req.choice_1 {
                if has_any { qb.push(", "); }
                qb.push("study_task_choice_1 = ");
                qb.push_bind(choice);
                has_any = true;
            }
            if let Some(ref choice) = req.choice_2 {
                if has_any { qb.push(", "); }
                qb.push("study_task_choice_2 = ");
                qb.push_bind(choice);
                has_any = true;
            }
            if let Some(ref choice) = req.choice_3 {
                if has_any { qb.push(", "); }
                qb.push("study_task_choice_3 = ");
                qb.push_bind(choice);
                has_any = true;
            }
            if let Some(ref choice) = req.choice_4 {
                if has_any { qb.push(", "); }
                qb.push("study_task_choice_4 = ");
                qb.push_bind(choice);
                has_any = true;
            }
            if let Some(correct) = req.choice_correct {
                if has_any { qb.push(", "); }
                qb.push("study_task_choice_answer = ");
                qb.push_bind(correct);
                has_any = true;
            }
            if let Some(ref image) = req.image_url {
                if has_any { qb.push(", "); }
                qb.push("study_task_choice_image_url = ");
                qb.push_bind(image);
                has_any = true;
            }
            if let Some(ref audio) = req.audio_url {
                if has_any { qb.push(", "); }
                qb.push("study_task_choice_audio_url = ");
                qb.push_bind(audio);
                has_any = true;
            }

            if has_any {
                qb.push(" WHERE study_task_id = ");
                qb.push_bind(study_task_id);
                qb.build().execute(&mut **tx).await?;
            }
        }
        crate::types::StudyTaskKind::Typing => {
            let mut qb = QueryBuilder::<Postgres>::new("UPDATE study_task_typing SET ");
            let mut has_any = false;

            if let Some(ref question) = req.question {
                if has_any { qb.push(", "); }
                qb.push("study_task_typing_question = ");
                qb.push_bind(question);
                has_any = true;
            }
            if let Some(ref answer) = req.answer {
                if has_any { qb.push(", "); }
                qb.push("study_task_typing_answer = ");
                qb.push_bind(answer);
                has_any = true;
            }
            if let Some(ref image) = req.image_url {
                if has_any { qb.push(", "); }
                qb.push("study_task_typing_image_url = ");
                qb.push_bind(image);
                has_any = true;
            }

            if has_any {
                qb.push(" WHERE study_task_id = ");
                qb.push_bind(study_task_id);
                qb.build().execute(&mut **tx).await?;
            }
        }
        crate::types::StudyTaskKind::Voice => {
            let mut qb = QueryBuilder::<Postgres>::new("UPDATE study_task_voice SET ");
            let mut has_any = false;

            if let Some(ref question) = req.question {
                if has_any { qb.push(", "); }
                qb.push("study_task_voice_question = ");
                qb.push_bind(question);
                has_any = true;
            }
            if let Some(ref answer) = req.answer {
                if has_any { qb.push(", "); }
                qb.push("study_task_voice_answer = ");
                qb.push_bind(answer);
                has_any = true;
            }
            if let Some(ref image) = req.image_url {
                if has_any { qb.push(", "); }
                qb.push("study_task_voice_image_url = ");
                qb.push_bind(image);
                has_any = true;
            }
            if let Some(ref audio) = req.audio_url {
                if has_any { qb.push(", "); }
                qb.push("study_task_voice_audio_url = ");
                qb.push_bind(audio);
                has_any = true;
            }

            if has_any {
                qb.push(" WHERE study_task_id = ");
                qb.push_bind(study_task_id);
                qb.build().execute(&mut **tx).await?;
            }
        }
    }

    let updated = sqlx::query_as::<_, AdminStudyTaskDetailRes>(
        r#"
        SELECT
            st.study_task_id::bigint AS study_task_id,
            st.study_id::bigint AS study_id,
            st.study_task_kind AS study_task_kind,
            st.study_task_seq AS study_task_seq,
            COALESCE(
                c.study_task_choice_question,
                t.study_task_typing_question,
                v.study_task_voice_question
            ) AS question,
            COALESCE(
                t.study_task_typing_answer,
                v.study_task_voice_answer
            ) AS answer,
            COALESCE(
                c.study_task_choice_image_url,
                t.study_task_typing_image_url,
                v.study_task_voice_image_url
            ) AS image_url,
            COALESCE(
                c.study_task_choice_audio_url,
                v.study_task_voice_audio_url
            ) AS audio_url,
            c.study_task_choice_1 AS choice_1,
            c.study_task_choice_2 AS choice_2,
            c.study_task_choice_3 AS choice_3,
            c.study_task_choice_4 AS choice_4,
            c.study_task_choice_answer AS choice_correct
        FROM study_task st
        LEFT JOIN study_task_choice c ON c.study_task_id = st.study_task_id
        LEFT JOIN study_task_typing t ON t.study_task_id = st.study_task_id
        LEFT JOIN study_task_voice v ON v.study_task_id = st.study_task_id
        WHERE st.study_task_id = $1
        "#,
    )
    .bind(study_task_id)
    .fetch_one(&mut **tx)
    .await?;

    Ok(updated)
}

pub async fn create_study_task(
    tx: &mut Transaction<'_, Postgres>,
    actor_user_id: i64,
    study_id: i32,
    study_task_kind: crate::types::StudyTaskKind,
    study_task_seq: i32,
) -> AppResult<i64> {
    let row = sqlx::query(
        r#"
        INSERT INTO study_task (
            study_id,
            updated_by_user_id,
            study_task_kind,
            study_task_seq
        )
        VALUES ($1, $2, $3, $4)
        RETURNING study_task_id::bigint AS study_task_id
        "#,
    )
    .bind(study_id)
    .bind(actor_user_id)
    .bind(study_task_kind)
    .bind(study_task_seq)
    .fetch_one(&mut **tx)
    .await?;

    let study_task_id: i64 = row.try_get("study_task_id")?;

    Ok(study_task_id)
}

pub async fn create_task_choice(
    tx: &mut Transaction<'_, Postgres>,
    study_task_id: i64,
    req: &StudyTaskCreateReq,
) -> AppResult<()> {
    sqlx::query(
        r#"
        INSERT INTO study_task_choice (
            study_task_id,
            study_task_choice_question,
            study_task_choice_1,
            study_task_choice_2,
            study_task_choice_3,
            study_task_choice_4,
            study_task_choice_answer,
            study_task_choice_audio_url,
            study_task_choice_image_url
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(study_task_id)
    .bind(req.question.as_deref())
    .bind(req.choice_1.as_deref())
    .bind(req.choice_2.as_deref())
    .bind(req.choice_3.as_deref())
    .bind(req.choice_4.as_deref())
    .bind(req.choice_correct)
    .bind(req.audio_url.as_deref())
    .bind(req.image_url.as_deref())
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn create_task_typing(
    tx: &mut Transaction<'_, Postgres>,
    study_task_id: i64,
    req: &StudyTaskCreateReq,
) -> AppResult<()> {
    sqlx::query(
        r#"
        INSERT INTO study_task_typing (
            study_task_id,
            study_task_typing_question,
            study_task_typing_answer,
            study_task_typing_image_url
        )
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(study_task_id)
    .bind(req.question.as_deref())
    .bind(req.answer.as_deref())
    .bind(req.image_url.as_deref())
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn create_task_voice(
    tx: &mut Transaction<'_, Postgres>,
    study_task_id: i64,
    req: &StudyTaskCreateReq,
) -> AppResult<()> {
    sqlx::query(
        r#"
        INSERT INTO study_task_voice (
            study_task_id,
            study_task_voice_question,
            study_task_voice_answer,
            study_task_voice_audio_url,
            study_task_voice_image_url
        )
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(study_task_id)
    .bind(req.question.as_deref())
    .bind(req.answer.as_deref())
    .bind(req.audio_url.as_deref())
    .bind(req.image_url.as_deref())
    .execute(&mut **tx)
    .await?;

    Ok(())
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
