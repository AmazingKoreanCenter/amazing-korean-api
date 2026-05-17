//! 해설 콘텐츠 조회 repo (DB 접근만)

use std::collections::HashMap;

use sqlx::PgPool;

use crate::error::AppResult;
use crate::types::SupportedLanguage;

/// explanation_unit 행 (enum/jsonb 는 ::text 캐스트로 fetch — sqlx json feature 미사용)
#[derive(Debug, sqlx::FromRow)]
pub struct UnitRow {
    pub explanation_unit_id: i32,
    pub unit_idx: String,
    pub unit_kind: String,
    pub unit_source: String,
    pub study_idx: Option<String>,
    pub study_task_idx: Option<String>,
    pub sentence_num: Option<i32>,
    pub section_id: Option<String>,
    pub title_ko: Option<String>,
    pub title_en: Option<String>,
    pub subtitle_ko: Option<String>,
    pub subtitle_en: Option<String>,
}

/// explanation_block 행
#[derive(Debug, sqlx::FromRow)]
pub struct BlockRow {
    pub explanation_block_id: i32,
    pub block_seq: i32,
    pub block_type: String,
    pub block_level: Option<i16>,
    pub text_ko: Option<String>,
    pub text_en: Option<String>,
    pub raw: Option<String>,
    pub structured_txt: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
struct TrRow {
    content_id: i64,
    field_name: String,
    translated_text: String,
}

const UNIT_COLS: &str = r#"
    explanation_unit_id, unit_idx,
    unit_kind::text   AS unit_kind,
    unit_source::text AS unit_source,
    study_idx, study_task_idx, sentence_num, section_id,
    title_ko, title_en, subtitle_ko, subtitle_en
"#;

pub struct ExplanationRepo;

impl ExplanationRepo {
    pub async fn find_unit_by_idx(pool: &PgPool, unit_idx: &str) -> AppResult<Option<UnitRow>> {
        let sql = format!("SELECT {UNIT_COLS} FROM explanation_unit WHERE unit_idx = $1");
        Ok(sqlx::query_as::<_, UnitRow>(&sql)
            .bind(unit_idx)
            .fetch_optional(pool)
            .await?)
    }

    /// 연결키(study_idx 또는 study_task_idx)로 unit 조회 (0..N, unit_seq 순)
    pub async fn find_units_by_link(
        pool: &PgPool,
        study_idx: Option<&str>,
        study_task_idx: Option<&str>,
    ) -> AppResult<Vec<UnitRow>> {
        let sql = format!(
            "SELECT {UNIT_COLS} FROM explanation_unit
             WHERE ($1::text IS NOT NULL AND study_idx = $1)
                OR ($2::text IS NOT NULL AND study_task_idx = $2)
             ORDER BY unit_seq"
        );
        Ok(sqlx::query_as::<_, UnitRow>(&sql)
            .bind(study_idx)
            .bind(study_task_idx)
            .fetch_all(pool)
            .await?)
    }

    pub async fn find_blocks(pool: &PgPool, unit_id: i32) -> AppResult<Vec<BlockRow>> {
        Ok(sqlx::query_as::<_, BlockRow>(
            r#"
            SELECT explanation_block_id,
                   block_seq,
                   block_type::text  AS block_type,
                   block_level,
                   text_ko, text_en, raw,
                   structured::text  AS structured_txt
            FROM explanation_block
            WHERE explanation_unit_id = $1
            ORDER BY block_seq
            "#,
        )
        .bind(unit_id)
        .fetch_all(pool)
        .await?)
    }

    /// content_translations 해소 맵: (content_id, field_name) → 텍스트.
    /// user_lang 우선, 없으면 en (산출 B 가 en base). ko 단락 회피(설명 structured 는
    /// ko 원본 없음 — content_translations 의 en 으로 폴백돼야 함).
    pub async fn find_translations(
        pool: &PgPool,
        content_type: &str,
        content_ids: &[i64],
        user_lang: SupportedLanguage,
    ) -> AppResult<HashMap<(i64, String), String>> {
        if content_ids.is_empty() {
            return Ok(HashMap::new());
        }
        let rows = sqlx::query_as::<_, TrRow>(
            r#"
            SELECT content_id, field_name, translated_text
            FROM content_translations
            WHERE content_type = $1::content_type_enum
              AND content_id = ANY($2)
              AND lang IN ($3, 'en')
              AND status = 'approved'
            ORDER BY content_id, field_name,
                     CASE lang WHEN $3 THEN 1 ELSE 2 END
            "#,
        )
        .bind(content_type)
        .bind(content_ids)
        .bind(user_lang)
        .fetch_all(pool)
        .await?;

        let mut map: HashMap<(i64, String), String> = HashMap::new();
        for r in rows {
            // ORDER BY 로 user_lang 이 먼저 → 첫 값 우선
            map.entry((r.content_id, r.field_name))
                .or_insert(r.translated_text);
        }
        Ok(map)
    }
}
