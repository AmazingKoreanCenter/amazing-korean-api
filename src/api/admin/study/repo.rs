use sqlx::{PgPool, Postgres, QueryBuilder, Row};
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