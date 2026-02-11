use std::collections::HashMap;

use sqlx::PgPool;

use crate::error::AppResult;
use crate::types::{ContentType, SupportedLanguage, TranslationStatus};

use super::dto::{TranslatedField, TranslationRes};

pub struct TranslationRepo;

impl TranslationRepo {
    /// 단건 번역 생성 (UPSERT — 동일 content_type+content_id+field_name+lang이면 업데이트)
    pub async fn upsert_one(
        pool: &PgPool,
        content_type: ContentType,
        content_id: i64,
        field_name: &str,
        lang: SupportedLanguage,
        translated_text: &str,
    ) -> AppResult<TranslationRes> {
        let row = sqlx::query_as::<_, TranslationRes>(
            r#"
            INSERT INTO content_translations
                (content_type, content_id, field_name, lang, translated_text)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (content_type, content_id, field_name, lang)
            DO UPDATE SET
                translated_text = EXCLUDED.translated_text,
                status = CASE
                    WHEN content_translations.translated_text = EXCLUDED.translated_text
                    THEN content_translations.status
                    ELSE 'draft'
                END,
                updated_at = NOW()
            RETURNING
                translation_id, content_type, content_id, field_name,
                lang, translated_text, status, created_at, updated_at
            "#,
        )
        .bind(content_type)
        .bind(content_id)
        .bind(field_name)
        .bind(lang)
        .bind(translated_text)
        .fetch_one(pool)
        .await?;

        Ok(row)
    }

    /// ID로 번역 조회
    pub async fn find_by_id(pool: &PgPool, translation_id: i64) -> AppResult<Option<TranslationRes>> {
        let row = sqlx::query_as::<_, TranslationRes>(
            r#"
            SELECT
                translation_id, content_type, content_id, field_name,
                lang, translated_text, status, created_at, updated_at
            FROM content_translations
            WHERE translation_id = $1
            "#,
        )
        .bind(translation_id)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// 번역 목록 개수
    pub async fn count_all(
        pool: &PgPool,
        content_type: Option<ContentType>,
        content_id: Option<i64>,
        lang: Option<SupportedLanguage>,
        status: Option<TranslationStatus>,
    ) -> AppResult<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM content_translations
            WHERE ($1::content_type_enum IS NULL OR content_type = $1)
              AND ($2::bigint IS NULL OR content_id = $2)
              AND ($3::supported_language_enum IS NULL OR lang = $3)
              AND ($4::translation_status_enum IS NULL OR status = $4)
            "#,
        )
        .bind(content_type)
        .bind(content_id)
        .bind(lang)
        .bind(status)
        .fetch_one(pool)
        .await?;

        Ok(count)
    }

    /// 번역 목록 조회 (필터 + 페이지네이션)
    pub async fn find_all(
        pool: &PgPool,
        content_type: Option<ContentType>,
        content_id: Option<i64>,
        lang: Option<SupportedLanguage>,
        status: Option<TranslationStatus>,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<TranslationRes>> {
        let rows = sqlx::query_as::<_, TranslationRes>(
            r#"
            SELECT
                translation_id, content_type, content_id, field_name,
                lang, translated_text, status, created_at, updated_at
            FROM content_translations
            WHERE ($1::content_type_enum IS NULL OR content_type = $1)
              AND ($2::bigint IS NULL OR content_id = $2)
              AND ($3::supported_language_enum IS NULL OR lang = $3)
              AND ($4::translation_status_enum IS NULL OR status = $4)
            ORDER BY updated_at DESC
            LIMIT $5
            OFFSET $6
            "#,
        )
        .bind(content_type)
        .bind(content_id)
        .bind(lang)
        .bind(status)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }

    /// 번역 수정 (텍스트 및/또는 상태)
    pub async fn update_one(
        pool: &PgPool,
        translation_id: i64,
        translated_text: Option<&str>,
        status: Option<TranslationStatus>,
    ) -> AppResult<Option<TranslationRes>> {
        let row = sqlx::query_as::<_, TranslationRes>(
            r#"
            UPDATE content_translations
            SET
                translated_text = COALESCE($2, translated_text),
                status = COALESCE($3, status),
                updated_at = NOW()
            WHERE translation_id = $1
            RETURNING
                translation_id, content_type, content_id, field_name,
                lang, translated_text, status, created_at, updated_at
            "#,
        )
        .bind(translation_id)
        .bind(translated_text)
        .bind(status)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// 번역 상태만 변경
    pub async fn update_status(
        pool: &PgPool,
        translation_id: i64,
        status: TranslationStatus,
    ) -> AppResult<Option<TranslationRes>> {
        let row = sqlx::query_as::<_, TranslationRes>(
            r#"
            UPDATE content_translations
            SET status = $2, updated_at = NOW()
            WHERE translation_id = $1
            RETURNING
                translation_id, content_type, content_id, field_name,
                lang, translated_text, status, created_at, updated_at
            "#,
        )
        .bind(translation_id)
        .bind(status)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    /// 번역 삭제
    pub async fn delete_one(pool: &PgPool, translation_id: i64) -> AppResult<bool> {
        let result = sqlx::query(
            r#"DELETE FROM content_translations WHERE translation_id = $1"#,
        )
        .bind(translation_id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    // =========================================================================
    // 공용 번역 조회 (기존 도메인 API에서 fallback 패턴으로 사용)
    // =========================================================================

    /// 특정 콘텐츠의 번역을 fallback 순서로 조회 (user_lang → en → ko 원본)
    ///
    /// 반환: HashMap<(content_id, field_name), TranslatedField>
    pub async fn find_translations_for_contents(
        pool: &PgPool,
        content_type: ContentType,
        content_ids: &[i64],
        user_lang: SupportedLanguage,
    ) -> AppResult<HashMap<(i64, String), TranslatedField>> {
        if content_ids.is_empty() {
            return Ok(HashMap::new());
        }

        // user_lang이 ko면 번역 불필요 (원본이 ko)
        if user_lang == SupportedLanguage::Ko {
            return Ok(HashMap::new());
        }

        // 사용자 언어 + en을 한 번에 가져옴 (approved만)
        let rows = sqlx::query_as::<_, TranslationRow>(
            r#"
            SELECT content_id, field_name, translated_text, lang
            FROM content_translations
            WHERE content_type = $1
              AND content_id = ANY($2)
              AND lang IN ($3, 'en')
              AND status = 'approved'
            ORDER BY
                content_id,
                field_name,
                CASE lang WHEN $3 THEN 1 WHEN 'en' THEN 2 END
            "#,
        )
        .bind(content_type)
        .bind(content_ids)
        .bind(user_lang)
        .fetch_all(pool)
        .await?;

        // content_id + field_name 별로 가장 우선순위 높은 번역 선택
        let mut result: HashMap<(i64, String), TranslatedField> = HashMap::new();
        for row in rows {
            let key = (row.content_id, row.field_name.clone());
            // 이미 사용자 언어 번역이 있으면 스킵 (ORDER BY로 user_lang이 먼저 옴)
            result.entry(key).or_insert_with(|| TranslatedField {
                text: row.translated_text,
                fallback_used: row.lang != user_lang,
            });
        }

        Ok(result)
    }
}

/// 내부 쿼리용 행
#[derive(Debug, sqlx::FromRow)]
struct TranslationRow {
    content_id: i64,
    field_name: String,
    translated_text: String,
    lang: SupportedLanguage,
}
