//! guide 콘텐츠 조회 repo (DB 접근만)

use std::collections::HashMap;

use sqlx::PgPool;

use crate::error::AppResult;
use crate::types::SupportedLanguage;

/// guide 행 (enum 은 ::text 캐스트 — explanation 선례)
#[derive(Debug, sqlx::FromRow)]
pub struct GuideRow {
    pub guide_id: i64,
    pub guide_idx: String,
    pub guide_seq: i32,
    pub guide_category: String,
    pub guide_theme: String,
    pub sentence_start: Option<i32>,
    pub sentence_end: Option<i32>,
    pub title_ko: Option<String>,
    pub title_en: Option<String>,
    pub subtitle_ko: Option<String>,
    pub subtitle_en: Option<String>,
}

/// 목록용 guide 행 + 첫 블록(=제목) 번역
#[derive(Debug, sqlx::FromRow)]
pub struct GuideListRow {
    pub guide_idx: String,
    pub guide_seq: i32,
    pub guide_category: String,
    pub guide_theme: String,
    pub sentence_start: Option<i32>,
    pub sentence_end: Option<i32>,
    pub title_ko: Option<String>,
    pub title_en: Option<String>,
    pub subtitle_ko: Option<String>,
    pub subtitle_en: Option<String>,
    pub title_tr: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct BlockRow {
    pub guide_block_id: i64,
    pub block_seq: i32,
    pub block_type: String,
    pub sentence_no: Option<i32>,
    pub text_ko: Option<String>,
    pub text_en: Option<String>,
    pub marker: Option<String>,
    pub table_no: Option<i32>,
    pub row_no: Option<i32>,
    pub col_no: Option<i32>,
    pub col_span: Option<i32>,
    pub row_span: Option<i32>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct SentenceRow {
    pub sentence_no: i32,
    pub guide_block_id: i64,
    pub pron_ko: Option<String>,
    pub audio_url: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
struct TrRow {
    content_id: i64,
    translated_text: String,
}

const GUIDE_COLS: &str = r#"
    guide_id, guide_idx, guide_seq,
    guide_category::text AS guide_category,
    guide_theme::text    AS guide_theme,
    sentence_start, sentence_end,
    title_ko, title_en, subtitle_ko, subtitle_en
"#;

pub struct GuideRepo;

impl GuideRepo {
    /// 공개 단원 목록 (state='open', guide_seq 순) + 표시 언어의 제목 번역
    /// (제목 = 단원 첫 블록 — 시드 변환기 규칙. LATERAL 로 첫 번역 블록 1건)
    pub async fn list_open(
        pool: &PgPool,
        lang: Option<SupportedLanguage>,
    ) -> AppResult<Vec<GuideListRow>> {
        Ok(sqlx::query_as::<_, GuideListRow>(
            r#"
            SELECT g.guide_idx, g.guide_seq,
                   g.guide_category::text AS guide_category,
                   g.guide_theme::text    AS guide_theme,
                   g.sentence_start, g.sentence_end,
                   g.title_ko, g.title_en, g.subtitle_ko, g.subtitle_en,
                   t.translated_text AS title_tr
            FROM guide g
            LEFT JOIN LATERAL (
                SELECT ct.translated_text
                FROM guide_block b
                JOIN content_translations ct
                  ON ct.content_type = 'guide_block'
                 AND ct.content_id = b.guide_block_id
                 AND ct.field_name = 'text'
                 AND ct.lang = $1
                 AND ct.status = 'approved'
                WHERE b.guide_id = g.guide_id
                ORDER BY b.block_seq
                LIMIT 1
            ) t ON $1::supported_language_enum IS NOT NULL
            WHERE g.guide_state = 'open'
            ORDER BY g.guide_seq
            "#,
        )
        .bind(lang)
        .fetch_all(pool)
        .await?)
    }

    /// 공개 단원 단건 (state='open')
    pub async fn find_open_by_idx(pool: &PgPool, guide_idx: &str) -> AppResult<Option<GuideRow>> {
        let sql =
            format!("SELECT {GUIDE_COLS} FROM guide WHERE guide_idx = $1 AND guide_state = 'open'");
        Ok(sqlx::query_as::<_, GuideRow>(&sql)
            .bind(guide_idx)
            .fetch_optional(pool)
            .await?)
    }

    pub async fn find_blocks(pool: &PgPool, guide_id: i64) -> AppResult<Vec<BlockRow>> {
        Ok(sqlx::query_as::<_, BlockRow>(
            r#"
            SELECT guide_block_id, block_seq,
                   block_type::text AS block_type,
                   sentence_no, text_ko, text_en, marker,
                   table_no, row_no, col_no, col_span, row_span
            FROM guide_block
            WHERE guide_id = $1
            ORDER BY block_seq
            "#,
        )
        .bind(guide_id)
        .fetch_all(pool)
        .await?)
    }

    pub async fn find_sentences(pool: &PgPool, guide_id: i64) -> AppResult<Vec<SentenceRow>> {
        Ok(sqlx::query_as::<_, SentenceRow>(
            r#"
            SELECT sentence_no, guide_block_id, pron_ko, audio_url
            FROM guide_sentence
            WHERE guide_id = $1
            ORDER BY sentence_no
            "#,
        )
        .bind(guide_id)
        .fetch_all(pool)
        .await?)
    }

    /// 블록 번역 맵: guide_block_id → 표시 언어 번역 (field='text', approved).
    /// en/ko 는 도메인 컬럼이 원천이라 번역 행이 없음 — 요청 언어만 조회.
    pub async fn find_block_translations(
        pool: &PgPool,
        block_ids: &[i64],
        lang: SupportedLanguage,
    ) -> AppResult<HashMap<i64, String>> {
        if block_ids.is_empty() {
            return Ok(HashMap::new());
        }
        let rows = sqlx::query_as::<_, TrRow>(
            r#"
            SELECT content_id, translated_text
            FROM content_translations
            WHERE content_type = 'guide_block'
              AND content_id = ANY($1)
              AND field_name = 'text'
              AND lang = $2
              AND status = 'approved'
            "#,
        )
        .bind(block_ids)
        .bind(lang)
        .fetch_all(pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|r| (r.content_id, r.translated_text))
            .collect())
    }
}
