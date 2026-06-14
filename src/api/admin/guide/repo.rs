//! guide admin 편집 repo (DB 접근만)

use sqlx::{PgPool, Postgres, Transaction};

use crate::error::AppResult;
use crate::types::SupportedLanguage;

use super::dto::{
    AdminGuideBlock, AdminGuideSentence, AdminGuideSummary, DiffExportItem, StaleSummaryRow,
};

pub struct AdminGuideRepo;

impl AdminGuideRepo {
    /// 단원 목록 (모든 state) + 단원별 stale 블록 수 + 총 블록 수
    pub async fn list(pool: &PgPool) -> AppResult<Vec<AdminGuideSummary>> {
        Ok(sqlx::query_as::<_, AdminGuideSummary>(
            r#"
            SELECT g.guide_id, g.guide_idx, g.guide_seq,
                   g.guide_state::text    AS guide_state,
                   g.guide_category::text AS guide_category,
                   g.guide_theme::text    AS guide_theme,
                   g.sentence_start, g.sentence_end, g.title_ko, g.title_en,
                   COALESCE(s.stale_count, 0) AS stale_count,
                   COALESCE(b.block_count, 0) AS block_count
            FROM guide g
            LEFT JOIN LATERAL (
                SELECT count(*) AS block_count FROM guide_block b WHERE b.guide_id = g.guide_id
            ) b ON true
            LEFT JOIN LATERAL (
                -- 번역이 원문보다 옛 버전인 (lang 무관) 블록 수
                SELECT count(DISTINCT b2.guide_block_id) AS stale_count
                FROM guide_block b2
                JOIN content_translations ct
                  ON ct.content_type = 'guide_block' AND ct.content_id = b2.guide_block_id
                 AND ct.field_name = 'text'
                 AND COALESCE(ct.source_version, 0) < b2.source_version
                WHERE b2.guide_id = g.guide_id
            ) s ON true
            ORDER BY g.guide_seq
            "#,
        )
        .fetch_all(pool)
        .await?)
    }

    /// guide_idx → guide_id (+ state 확인용 단건)
    pub async fn find_id(pool: &PgPool, guide_idx: &str) -> AppResult<Option<i64>> {
        Ok(
            sqlx::query_scalar::<_, i64>("SELECT guide_id FROM guide WHERE guide_idx = $1")
                .bind(guide_idx)
                .fetch_optional(pool)
                .await?,
        )
    }

    pub async fn detail_header(
        pool: &PgPool,
        guide_idx: &str,
    ) -> AppResult<Option<GuideHeaderRow>> {
        Ok(sqlx::query_as::<_, GuideHeaderRow>(
            r#"
            SELECT guide_id, guide_idx, guide_seq,
                   guide_state::text    AS guide_state,
                   guide_category::text AS guide_category,
                   guide_theme::text    AS guide_theme,
                   sentence_start, sentence_end,
                   title_ko, title_en, subtitle_ko, subtitle_en
            FROM guide WHERE guide_idx = $1
            "#,
        )
        .bind(guide_idx)
        .fetch_optional(pool)
        .await?)
    }

    pub async fn detail_blocks(pool: &PgPool, guide_id: i64) -> AppResult<Vec<AdminGuideBlock>> {
        Ok(sqlx::query_as::<_, AdminGuideBlock>(
            r#"
            SELECT guide_block_id, block_seq,
                   block_type::text AS block_type,
                   sentence_no, text_ko, text_en, marker,
                   table_no, row_no, col_no, col_span, row_span,
                   source_version, legacy_key,
                   (updated_by_user_id IS NOT NULL) AS edited
            FROM guide_block WHERE guide_id = $1 ORDER BY block_seq
            "#,
        )
        .bind(guide_id)
        .fetch_all(pool)
        .await?)
    }

    pub async fn detail_sentences(
        pool: &PgPool,
        guide_id: i64,
    ) -> AppResult<Vec<AdminGuideSentence>> {
        Ok(sqlx::query_as::<_, AdminGuideSentence>(
            r#"
            SELECT guide_sentence_id, sentence_no, pron_ko,
                   speech_level, subject_honorific, audio_url
            FROM guide_sentence WHERE guide_id = $1 ORDER BY sentence_no
            "#,
        )
        .bind(guide_id)
        .fetch_all(pool)
        .await?)
    }

    /// 단원 메타 부분 수정 (COALESCE — None 필드 유지). state/theme 은 호출부에서 검증된 문자열.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_meta(
        tx: &mut Transaction<'_, Postgres>,
        guide_id: i64,
        actor: i64,
        guide_state: Option<&str>,
        guide_theme: Option<&str>,
        title_ko: Option<&str>,
        title_en: Option<&str>,
        subtitle_ko: Option<&str>,
        subtitle_en: Option<&str>,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE guide SET
              guide_state  = COALESCE($2::guide_state_enum, guide_state),
              guide_theme  = COALESCE($3::guide_theme_enum, guide_theme),
              title_ko     = COALESCE($4, title_ko),
              title_en     = COALESCE($5, title_en),
              subtitle_ko  = COALESCE($6, subtitle_ko),
              subtitle_en  = COALESCE($7, subtitle_en),
              updated_by_user_id = $8,
              guide_updated_at = now()
            WHERE guide_id = $1
            "#,
        )
        .bind(guide_id)
        .bind(guide_state)
        .bind(guide_theme)
        .bind(title_ko)
        .bind(title_en)
        .bind(subtitle_ko)
        .bind(subtitle_en)
        .bind(actor)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    /// 블록 단건 조회 (편집 전 현재 텍스트 확인)
    pub async fn find_block(pool: &PgPool, block_id: i64) -> AppResult<Option<BlockTextRow>> {
        Ok(sqlx::query_as::<_, BlockTextRow>(
            "SELECT guide_block_id, text_ko, text_en, source_version FROM guide_block WHERE guide_block_id = $1",
        )
        .bind(block_id)
        .fetch_optional(pool)
        .await?)
    }

    /// 블록 텍스트 수정 + source_version 증가 (텍스트 실제 변경 시에만 호출).
    pub async fn update_block_text(
        tx: &mut Transaction<'_, Postgres>,
        block_id: i64,
        actor: i64,
        text_ko: Option<&str>,
        text_en: Option<&str>,
    ) -> AppResult<i32> {
        Ok(sqlx::query_scalar::<_, i32>(
            r#"
            UPDATE guide_block SET
              text_ko = $2, text_en = $3,
              source_version = source_version + 1,
              updated_by_user_id = $4,
              guide_block_updated_at = now()
            WHERE guide_block_id = $1
            RETURNING source_version
            "#,
        )
        .bind(block_id)
        .bind(text_ko)
        .bind(text_en)
        .bind(actor)
        .fetch_one(&mut **tx)
        .await?)
    }

    pub async fn sentence_guide_id(pool: &PgPool, sentence_no: i32) -> AppResult<Option<i64>> {
        Ok(sqlx::query_scalar::<_, i64>(
            "SELECT guide_sentence_id FROM guide_sentence WHERE sentence_no = $1",
        )
        .bind(sentence_no)
        .fetch_optional(pool)
        .await?)
    }

    pub async fn update_sentence_meta(
        tx: &mut Transaction<'_, Postgres>,
        sentence_no: i32,
        actor: i64,
        pron_ko: Option<&str>,
        speech_level: Option<&str>,
        subject_honorific: Option<bool>,
        audio_url: Option<&str>,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE guide_sentence SET
              pron_ko           = COALESCE($2, pron_ko),
              speech_level      = COALESCE($3, speech_level),
              subject_honorific = COALESCE($4, subject_honorific),
              audio_url         = COALESCE($5, audio_url),
              updated_by_user_id = $6,
              guide_sentence_updated_at = now()
            WHERE sentence_no = $1
            "#,
        )
        .bind(sentence_no)
        .bind(pron_ko)
        .bind(speech_level)
        .bind(subject_honorific)
        .bind(audio_url)
        .bind(actor)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    /// 언어별 stale/missing 집계 (대시보드). lang None = 전 적재 언어.
    pub async fn stale_dashboard(
        pool: &PgPool,
        lang: Option<SupportedLanguage>,
    ) -> AppResult<Vec<StaleSummaryRow>> {
        // 번역 대상 블록 = text_en 있는 블록(번역 큐 대상). stale = ct 존재하나 옛 버전, missing = ct 부재.
        Ok(sqlx::query_as::<_, StaleSummaryRow>(
            r#"
            WITH translatable AS (
                SELECT guide_block_id, source_version FROM guide_block
                WHERE text_en IS NOT NULL AND text_en <> ''
            ),
            langs AS (
                SELECT DISTINCT lang FROM content_translations
                WHERE content_type = 'guide_block'
                  AND ($1::supported_language_enum IS NULL OR lang = $1)
            )
            SELECT l.lang::text AS lang,
                   count(*) FILTER (
                     WHERE ct.content_id IS NOT NULL
                       AND COALESCE(ct.source_version, 0) < t.source_version
                   ) AS stale_count,
                   count(*) FILTER (WHERE ct.content_id IS NULL) AS missing_count
            FROM langs l
            CROSS JOIN translatable t
            LEFT JOIN content_translations ct
              ON ct.content_type = 'guide_block' AND ct.content_id = t.guide_block_id
             AND ct.field_name = 'text' AND ct.lang = l.lang
            GROUP BY l.lang
            ORDER BY l.lang
            "#,
        )
        .bind(lang)
        .fetch_all(pool)
        .await?)
    }

    /// 디프 export — 특정 언어의 stale + missing 블록(원문 text_en 동봉)
    pub async fn diff_export(
        pool: &PgPool,
        lang: SupportedLanguage,
    ) -> AppResult<Vec<DiffExportItem>> {
        Ok(sqlx::query_as::<_, DiffExportItem>(
            r#"
            SELECT COALESCE(b.legacy_key, 'db:' || b.guide_block_id::text) AS id,
                   b.guide_block_id, b.text_en AS source_text, b.source_version
            FROM guide_block b
            LEFT JOIN content_translations ct
              ON ct.content_type = 'guide_block' AND ct.content_id = b.guide_block_id
             AND ct.field_name = 'text' AND ct.lang = $1
            WHERE b.text_en IS NOT NULL AND b.text_en <> ''
              AND (ct.content_id IS NULL OR COALESCE(ct.source_version, 0) < b.source_version)
            ORDER BY b.guide_id, b.block_seq
            "#,
        )
        .bind(lang)
        .fetch_all(pool)
        .await?)
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct GuideHeaderRow {
    pub guide_id: i64,
    pub guide_idx: String,
    pub guide_seq: i32,
    pub guide_state: String,
    pub guide_category: String,
    pub guide_theme: String,
    pub sentence_start: Option<i32>,
    pub sentence_end: Option<i32>,
    pub title_ko: Option<String>,
    pub title_en: Option<String>,
    pub subtitle_ko: Option<String>,
    pub subtitle_en: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct BlockTextRow {
    pub guide_block_id: i64,
    pub text_ko: Option<String>,
    pub text_en: Option<String>,
    pub source_version: i32,
}
