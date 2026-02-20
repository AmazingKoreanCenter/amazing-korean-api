use std::collections::HashMap;

use sqlx::PgPool;

use crate::error::AppResult;
use crate::types::{ContentType, SupportedLanguage, TranslationStatus};

use super::dto::{ContentRecordItem, SourceFieldItem, TranslatedField, TranslationRes, TranslationSearchItem};

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
    /// content_types_csv: 쉼표 구분 복수 content_type (e.g. "study,study_task_choice")
    /// content_type: 단일 content_type (content_types_csv가 없을 때 사용)
    pub async fn count_all(
        pool: &PgPool,
        content_type: Option<ContentType>,
        content_types_csv: Option<&str>,
        content_id: Option<i64>,
        lang: Option<SupportedLanguage>,
        status: Option<TranslationStatus>,
    ) -> AppResult<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM content_translations
            WHERE (
                CASE
                    WHEN $5::text IS NOT NULL THEN content_type::text = ANY(string_to_array($5, ','))
                    WHEN $1::content_type_enum IS NOT NULL THEN content_type = $1
                    ELSE true
                END
            )
              AND ($2::bigint IS NULL OR content_id = $2)
              AND ($3::supported_language_enum IS NULL OR lang = $3)
              AND ($4::translation_status_enum IS NULL OR status = $4)
            "#,
        )
        .bind(content_type)
        .bind(content_id)
        .bind(lang)
        .bind(status)
        .bind(content_types_csv)
        .fetch_one(pool)
        .await?;

        Ok(count)
    }

    /// 번역 목록 조회 (필터 + 페이지네이션)
    /// content_types_csv: 쉼표 구분 복수 content_type (content_type보다 우선)
    pub async fn find_all(
        pool: &PgPool,
        content_type: Option<ContentType>,
        content_types_csv: Option<&str>,
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
            WHERE (
                CASE
                    WHEN $7::text IS NOT NULL THEN content_type::text = ANY(string_to_array($7, ','))
                    WHEN $1::content_type_enum IS NOT NULL THEN content_type = $1
                    ELSE true
                END
            )
              AND ($2::bigint IS NULL OR content_id = $2)
              AND ($3::supported_language_enum IS NULL OR lang = $3)
              AND ($4::translation_status_enum IS NULL OR status = $4)
            ORDER BY translation_id DESC
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
        .bind(content_types_csv)
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
    // 콘텐츠 목록 조회 (Step 4)
    // =========================================================================

    /// content_type별 레코드 목록 조회 (드롭다운용)
    pub async fn find_content_records(
        pool: &PgPool,
        content_type: ContentType,
    ) -> AppResult<Vec<ContentRecordItem>> {
        let items = match content_type {
            ContentType::Video => {
                sqlx::query_as::<_, ContentRecordItem>(
                    r#"
                    SELECT
                        v.video_id::bigint AS id,
                        v.video_idx AS label,
                        COALESCE(
                            (SELECT string_agg(vt.video_tag_title, ', ' ORDER BY vt.video_tag_title)
                             FROM video_tag_map vtm
                             JOIN video_tag vt ON vt.video_tag_id = vtm.video_tag_id
                             WHERE vtm.video_id = v.video_id),
                            ''
                        ) AS detail
                    FROM video v
                    ORDER BY v.video_id
                    "#,
                )
                .fetch_all(pool)
                .await?
            }
            ContentType::Lesson => {
                sqlx::query_as::<_, ContentRecordItem>(
                    r#"
                    SELECT
                        lesson_id::bigint AS id,
                        lesson_idx AS label,
                        lesson_title AS detail
                    FROM lesson
                    ORDER BY lesson_id
                    "#,
                )
                .fetch_all(pool)
                .await?
            }
            ContentType::Study => {
                sqlx::query_as::<_, ContentRecordItem>(
                    r#"
                    SELECT
                        study_id::bigint AS id,
                        study_idx AS label,
                        study_title AS detail
                    FROM study
                    ORDER BY study_id
                    "#,
                )
                .fetch_all(pool)
                .await?
            }
            ContentType::StudyTaskChoice => {
                sqlx::query_as::<_, ContentRecordItem>(
                    r#"
                    SELECT
                        st.study_task_id::bigint AS id,
                        CONCAT('Study#', st.study_id, ' Task#', st.study_task_seq) AS label,
                        LEFT(stc.study_task_choice_question, 50) AS detail
                    FROM study_task st
                    JOIN study_task_choice stc ON stc.study_task_id = st.study_task_id
                    WHERE st.study_task_kind = 'choice'
                    ORDER BY st.study_id, st.study_task_seq
                    "#,
                )
                .fetch_all(pool)
                .await?
            }
            ContentType::StudyTaskTyping => {
                sqlx::query_as::<_, ContentRecordItem>(
                    r#"
                    SELECT
                        st.study_task_id::bigint AS id,
                        CONCAT('Study#', st.study_id, ' Task#', st.study_task_seq) AS label,
                        LEFT(stt.study_task_typing_question, 50) AS detail
                    FROM study_task st
                    JOIN study_task_typing stt ON stt.study_task_id = st.study_task_id
                    WHERE st.study_task_kind = 'typing'
                    ORDER BY st.study_id, st.study_task_seq
                    "#,
                )
                .fetch_all(pool)
                .await?
            }
            ContentType::StudyTaskVoice => {
                sqlx::query_as::<_, ContentRecordItem>(
                    r#"
                    SELECT
                        st.study_task_id::bigint AS id,
                        CONCAT('Study#', st.study_id, ' Task#', st.study_task_seq) AS label,
                        LEFT(stv.study_task_voice_question, 50) AS detail
                    FROM study_task st
                    JOIN study_task_voice stv ON stv.study_task_id = st.study_task_id
                    WHERE st.study_task_kind = 'voice'
                    ORDER BY st.study_id, st.study_task_seq
                    "#,
                )
                .fetch_all(pool)
                .await?
            }
            ContentType::StudyTaskExplain => {
                sqlx::query_as::<_, ContentRecordItem>(
                    r#"
                    SELECT DISTINCT ON (st.study_task_id)
                        st.study_task_id::bigint AS id,
                        CONCAT('Study#', st.study_id, ' Task#', st.study_task_seq) AS label,
                        ste.explain_title AS detail
                    FROM study_task st
                    JOIN study_task_explain ste ON ste.study_task_id = st.study_task_id
                        AND ste.explain_lang = 'ko'
                    ORDER BY st.study_task_id, st.study_id, st.study_task_seq
                    "#,
                )
                .fetch_all(pool)
                .await?
            }
            // VideoTag, Course — 직접 선택하지 않음 (Video 내부에서 처리)
            _ => Vec::new(),
        };

        Ok(items)
    }

    // =========================================================================
    // 원본 텍스트 조회 (Step 5)
    // =========================================================================

    /// content_type + content_id에 해당하는 모든 번역 가능 필드 + 한국어 원본 반환
    pub async fn find_source_fields(
        pool: &PgPool,
        content_type: ContentType,
        content_id: i64,
    ) -> AppResult<Vec<SourceFieldItem>> {
        let mut fields = Vec::new();

        match content_type {
            ContentType::Video => {
                // video 테이블 자체 필드
                let row = sqlx::query_as::<_, VideoSourceRow>(
                    r#"SELECT video_idx FROM video WHERE video_id = $1"#,
                )
                .bind(content_id)
                .fetch_optional(pool)
                .await?;

                if let Some(r) = row {
                    fields.push(SourceFieldItem {
                        content_type: ContentType::Video,
                        content_id,
                        field_name: "video_idx".to_string(),
                        source_text: Some(r.video_idx),
                    });
                }

                // video에 연결된 video_tag 필드들
                let tags = sqlx::query_as::<_, VideoTagSourceRow>(
                    r#"
                    SELECT vt.video_tag_id::bigint, vt.video_tag_key, vt.video_tag_title, vt.video_tag_subtitle
                    FROM video_tag vt
                    JOIN video_tag_map vtm ON vtm.video_tag_id = vt.video_tag_id
                    WHERE vtm.video_id = $1
                    ORDER BY vt.video_tag_id
                    "#,
                )
                .bind(content_id)
                .fetch_all(pool)
                .await?;

                for tag in tags {
                    fields.push(SourceFieldItem {
                        content_type: ContentType::VideoTag,
                        content_id: tag.video_tag_id,
                        field_name: "video_tag_key".to_string(),
                        source_text: Some(tag.video_tag_key),
                    });
                    fields.push(SourceFieldItem {
                        content_type: ContentType::VideoTag,
                        content_id: tag.video_tag_id,
                        field_name: "video_tag_title".to_string(),
                        source_text: Some(tag.video_tag_title),
                    });
                    fields.push(SourceFieldItem {
                        content_type: ContentType::VideoTag,
                        content_id: tag.video_tag_id,
                        field_name: "video_tag_subtitle".to_string(),
                        source_text: tag.video_tag_subtitle,
                    });
                }
            }
            ContentType::Lesson => {
                let row = sqlx::query_as::<_, LessonSourceRow>(
                    r#"
                    SELECT lesson_idx, lesson_title, lesson_subtitle, lesson_description
                    FROM lesson WHERE lesson_id = $1
                    "#,
                )
                .bind(content_id)
                .fetch_optional(pool)
                .await?;

                if let Some(r) = row {
                    for (name, text) in [
                        ("lesson_idx", Some(r.lesson_idx)),
                        ("lesson_title", Some(r.lesson_title)),
                        ("lesson_subtitle", r.lesson_subtitle),
                        ("lesson_description", r.lesson_description),
                    ] {
                        fields.push(SourceFieldItem {
                            content_type: ContentType::Lesson,
                            content_id,
                            field_name: name.to_string(),
                            source_text: text,
                        });
                    }
                }
            }
            ContentType::Study => {
                let row = sqlx::query_as::<_, StudySourceRow>(
                    r#"
                    SELECT study_idx, study_title, study_subtitle, study_description
                    FROM study WHERE study_id = $1
                    "#,
                )
                .bind(content_id)
                .fetch_optional(pool)
                .await?;

                if let Some(r) = row {
                    for (name, text) in [
                        ("study_idx", Some(r.study_idx)),
                        ("study_title", Some(r.study_title)),
                        ("study_subtitle", r.study_subtitle),
                        ("study_description", r.study_description),
                    ] {
                        fields.push(SourceFieldItem {
                            content_type: ContentType::Study,
                            content_id,
                            field_name: name.to_string(),
                            source_text: text,
                        });
                    }
                }
            }
            ContentType::StudyTaskChoice => {
                let row = sqlx::query_as::<_, ChoiceSourceRow>(
                    r#"
                    SELECT
                        study_task_choice_question,
                        study_task_choice_1, study_task_choice_2,
                        study_task_choice_3, study_task_choice_4,
                        study_task_choice_answer
                    FROM study_task_choice WHERE study_task_id = $1
                    "#,
                )
                .bind(content_id)
                .fetch_optional(pool)
                .await?;

                if let Some(r) = row {
                    for (name, text) in [
                        ("study_task_choice_question", Some(r.study_task_choice_question)),
                        ("study_task_choice_1", Some(r.study_task_choice_1)),
                        ("study_task_choice_2", Some(r.study_task_choice_2)),
                        ("study_task_choice_3", Some(r.study_task_choice_3)),
                        ("study_task_choice_4", Some(r.study_task_choice_4)),
                        ("study_task_choice_answer", Some(r.study_task_choice_answer.to_string())),
                    ] {
                        fields.push(SourceFieldItem {
                            content_type: ContentType::StudyTaskChoice,
                            content_id,
                            field_name: name.to_string(),
                            source_text: text,
                        });
                    }
                }
            }
            ContentType::StudyTaskTyping => {
                let row = sqlx::query_as::<_, TypingSourceRow>(
                    r#"
                    SELECT study_task_typing_question, study_task_typing_answer
                    FROM study_task_typing WHERE study_task_id = $1
                    "#,
                )
                .bind(content_id)
                .fetch_optional(pool)
                .await?;

                if let Some(r) = row {
                    for (name, text) in [
                        ("study_task_typing_question", r.study_task_typing_question),
                        ("study_task_typing_answer", r.study_task_typing_answer),
                    ] {
                        fields.push(SourceFieldItem {
                            content_type: ContentType::StudyTaskTyping,
                            content_id,
                            field_name: name.to_string(),
                            source_text: text,
                        });
                    }
                }
            }
            ContentType::StudyTaskVoice => {
                let row = sqlx::query_as::<_, VoiceSourceRow>(
                    r#"
                    SELECT study_task_voice_question, study_task_voice_answer
                    FROM study_task_voice WHERE study_task_id = $1
                    "#,
                )
                .bind(content_id)
                .fetch_optional(pool)
                .await?;

                if let Some(r) = row {
                    for (name, text) in [
                        ("study_task_voice_question", r.study_task_voice_question),
                        ("study_task_voice_answer", r.study_task_voice_answer),
                    ] {
                        fields.push(SourceFieldItem {
                            content_type: ContentType::StudyTaskVoice,
                            content_id,
                            field_name: name.to_string(),
                            source_text: text,
                        });
                    }
                }
            }
            ContentType::StudyTaskExplain => {
                let row = sqlx::query_as::<_, ExplainSourceRow>(
                    r#"
                    SELECT explain_title, explain_text
                    FROM study_task_explain
                    WHERE study_task_id = $1 AND explain_lang = 'ko'
                    "#,
                )
                .bind(content_id)
                .fetch_optional(pool)
                .await?;

                if let Some(r) = row {
                    for (name, text) in [
                        ("explain_title", r.explain_title),
                        ("explain_text", r.explain_text),
                    ] {
                        fields.push(SourceFieldItem {
                            content_type: ContentType::StudyTaskExplain,
                            content_id,
                            field_name: name.to_string(),
                            source_text: text,
                        });
                    }
                }
            }
            _ => {} // VideoTag, Course — 직접 호출되지 않음
        }

        Ok(fields)
    }

    // =========================================================================
    // 번역 검색 (Step 11 — 재사용)
    // =========================================================================

    /// 기존 번역 검색 (언어 + 상태 기반 최근 번역 조회)
    pub async fn search_translations(
        pool: &PgPool,
        lang: Option<SupportedLanguage>,
    ) -> AppResult<Vec<TranslationSearchItem>> {
        let rows = sqlx::query_as::<_, TranslationSearchItem>(
            r#"
            SELECT
                translation_id, content_type, content_id, field_name,
                lang, translated_text, status
            FROM content_translations
            WHERE ($1::supported_language_enum IS NULL OR lang = $1)
              AND status IN ('approved', 'reviewed')
            ORDER BY updated_at DESC
            LIMIT 50
            "#,
        )
        .bind(lang)
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }

    // =========================================================================
    // 번역 통계 (Translation Stats)
    // =========================================================================

    /// content_type × lang × status 별 집계 통계
    pub async fn find_translation_stats(
        pool: &PgPool,
    ) -> AppResult<Vec<super::dto::TranslationStatItem>> {
        let rows = sqlx::query_as::<_, super::dto::TranslationStatItem>(
            r#"
            SELECT content_type, lang, status, COUNT(*) AS count
            FROM content_translations
            GROUP BY content_type, lang, status
            ORDER BY content_type, lang, status
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(rows)
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

// =============================================================================
// Source Fields 내부 쿼리용 행 구조체
// =============================================================================

#[derive(Debug, sqlx::FromRow)]
struct VideoSourceRow {
    video_idx: String,
}

#[derive(Debug, sqlx::FromRow)]
struct VideoTagSourceRow {
    video_tag_id: i64,
    video_tag_key: String,
    video_tag_title: String,
    video_tag_subtitle: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
struct LessonSourceRow {
    lesson_idx: String,
    lesson_title: String,
    lesson_subtitle: Option<String>,
    lesson_description: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
struct StudySourceRow {
    study_idx: String,
    study_title: String,
    study_subtitle: Option<String>,
    study_description: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
struct ChoiceSourceRow {
    study_task_choice_question: String,
    study_task_choice_1: String,
    study_task_choice_2: String,
    study_task_choice_3: String,
    study_task_choice_4: String,
    study_task_choice_answer: i32,
}

#[derive(Debug, sqlx::FromRow)]
struct TypingSourceRow {
    study_task_typing_question: Option<String>,
    study_task_typing_answer: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
struct VoiceSourceRow {
    study_task_voice_question: Option<String>,
    study_task_voice_answer: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
struct ExplainSourceRow {
    explain_title: Option<String>,
    explain_text: Option<String>,
}
