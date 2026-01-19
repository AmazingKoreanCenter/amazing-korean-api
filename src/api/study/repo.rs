use sqlx::{PgPool, QueryBuilder};

use crate::error::AppResult;
use crate::types::StudyTaskKind;

use super::dto::{
    ChoicePayload, StudyListItem, StudyListReq, StudyTaskDetailRes, TaskExplainRes,
    TaskPayload, TaskStatusRes, TypingPayload, VoicePayload,
};

pub struct StudyRepo;

// 내부 사용용 Row 구조체 (DB 조회 결과 매핑)
#[derive(sqlx::FromRow)]
struct StudyTaskDetailRow {
    task_id: i32,  // [FIX] DTO Type Mismatch: i64 -> i32
    study_id: i32, // [FIX] DTO Type Mismatch: i64 -> i32
    kind: StudyTaskKind,
    seq: i32,

    // Common
    question: Option<String>,
    
    // Choice
    choice_1: Option<String>,
    choice_2: Option<String>,
    choice_3: Option<String>,
    choice_4: Option<String>,
    choice_audio_url: Option<String>, 
    choice_image_url: Option<String>,

    // Typing
    #[allow(dead_code)]
    typing_answer_text: Option<String>,
    typing_image_url: Option<String>,

    // Voice
    #[allow(dead_code)]
    voice_answer_text: Option<String>,
    voice_audio_url: Option<String>, 
    voice_image_url: Option<String>,
}

impl StudyTaskDetailRow {
    fn map_to_res(self) -> Option<StudyTaskDetailRes> {
        let payload = match self.kind {
            StudyTaskKind::Choice => {
                if self.choice_1.is_none() || self.choice_2.is_none() {
                    return None;
                }
                TaskPayload::Choice(ChoicePayload {
                    choice_1: self.choice_1.unwrap(),
                    choice_2: self.choice_2.unwrap(),
                    choice_3: self.choice_3.unwrap_or_default(),
                    choice_4: self.choice_4.unwrap_or_default(),
                    audio_url: self.choice_audio_url,
                    image_url: self.choice_image_url,
                })
            }
            StudyTaskKind::Typing => TaskPayload::Typing(TypingPayload {
                image_url: self.typing_image_url,
            }),
            StudyTaskKind::Voice => TaskPayload::Voice(VoicePayload {
                audio_url: self.voice_audio_url,
                image_url: self.voice_image_url,
            }),
        };

        let media_url = match self.kind {
            StudyTaskKind::Choice => None,
            StudyTaskKind::Typing => None,
            StudyTaskKind::Voice => None,
        };

        Some(StudyTaskDetailRes {
            task_id: self.task_id,
            study_id: self.study_id,
            kind: self.kind,
            seq: self.seq,
            question: self.question,
            media_url,
            payload,
        })
    }
}

impl StudyRepo {
    // =========================================================================
    // 1. List & Search (Dynamic Query)
    // =========================================================================

    pub async fn find_list_dynamic(
        pool: &PgPool,
        req: &StudyListReq,
    ) -> AppResult<(Vec<StudyListItem>, i64)> {
        // ---------------------------------------------------------
        // A. Count Query
        // ---------------------------------------------------------
        let mut qb_count = QueryBuilder::new("SELECT COUNT(*) FROM study WHERE study_state = 'open'");

        if let Some(program) = &req.program {
            qb_count.push(" AND study_program = ")
                .push_bind(program)
                .push("::study_program_enum");
        }

        let count: i64 = qb_count
            .build_query_scalar()
            .fetch_one(pool)
            .await?;

        // ---------------------------------------------------------
        // B. List Query
        // ---------------------------------------------------------
        let mut qb_list = QueryBuilder::new(
            r#"
            SELECT 
                study_id::INT, -- [FIX] Output Cast: i64 -> i32
                study_idx::TEXT,
                study_program,
                study_title::TEXT AS title,
                study_subtitle::TEXT AS subtitle,
                study_created_at AS created_at
            FROM study
            WHERE study_state = 'open'
            "#,
        );

        if let Some(program) = &req.program {
            qb_list.push(" AND study_program = ")
                .push_bind(program)
                .push("::study_program_enum");
        }

        // Sorting
        match req.sort.as_deref() {
            Some("oldest") => qb_list.push(" ORDER BY study_created_at ASC"),
            _ => qb_list.push(" ORDER BY study_created_at DESC"), // Default: Newest
        };

        // Pagination
        // Note: Arguments are cast to i64 for DB limit/offset safety, but result struct uses i32 for IDs
        let offset = (req.page - 1) * req.per_page;
        qb_list.push(" LIMIT ").push_bind(req.per_page as i64);
        qb_list.push(" OFFSET ").push_bind(offset as i64);

        let list = qb_list
            .build_query_as::<StudyListItem>()
            .fetch_all(pool)
            .await?;

        Ok((list, count))
    }

    // =========================================================================
    // 2. Task Detail
    // =========================================================================

    pub async fn find_task_detail(
        pool: &PgPool,
        task_id: i64,
    ) -> AppResult<Option<StudyTaskDetailRes>> {
        // [FIX] Output Cast: study_task_id -> INT (i32)
        // [FIX] Input Cast: task_id arg -> i32
        let row = sqlx::query_as!(
            StudyTaskDetailRow,
            r#"
            SELECT
                t.study_task_id::INT AS task_id,
                t.study_id::INT AS study_id,
                t.study_task_kind AS "kind!: StudyTaskKind",
                t.study_task_seq AS seq,
                
                -- Question (Coalesce from each type)
                COALESCE(stc.study_task_choice_question, stt.study_task_typing_question, stv.study_task_voice_question)::TEXT AS question,

                -- Choice Fields
                stc.study_task_choice_1::TEXT AS choice_1,
                stc.study_task_choice_2::TEXT AS choice_2,
                stc.study_task_choice_3::TEXT AS choice_3,
                stc.study_task_choice_4::TEXT AS choice_4,
                stc.study_task_choice_audio_url::TEXT AS choice_audio_url,
                stc.study_task_choice_image_url::TEXT AS choice_image_url,

                -- Typing Fields
                stt.study_task_typing_answer::TEXT AS typing_answer_text,
                stt.study_task_typing_image_url::TEXT AS typing_image_url,

                -- Voice Fields
                stv.study_task_voice_answer::TEXT AS voice_answer_text,
                stv.study_task_voice_audio_url::TEXT AS voice_audio_url,
                stv.study_task_voice_image_url::TEXT AS voice_image_url

            FROM study_task t
            LEFT JOIN study_task_choice stc ON t.study_task_id = stc.study_task_id
            LEFT JOIN study_task_typing stt ON t.study_task_id = stt.study_task_id
            LEFT JOIN study_task_voice stv  ON t.study_task_id = stv.study_task_id
            WHERE t.study_task_id = $1
            "#,
            task_id as i32
        )
        .fetch_optional(pool)
        .await?;

        match row {
            Some(r) => Ok(r.map_to_res()),
            None => Ok(None),
        }
    }

    // =========================================================================
    // 3. Explanation
    // =========================================================================

    pub async fn find_explanation(
        pool: &PgPool,
        task_id: i64,
    ) -> AppResult<Option<TaskExplainRes>> {
        // [FIX] Output Cast: study_task_id -> INT (i32)
        // [FIX] Nullable Handling: COALESCE(..., '') for correct_answer (String)
        let res = sqlx::query_as!(
            TaskExplainRes,
            r#"
            SELECT
                t.study_task_id::INT AS task_id,
                te.explain_title::TEXT AS title,
                COALESCE(
                    stc.study_task_choice_correct::TEXT,
                    stt.study_task_typing_answer,
                    stv.study_task_voice_answer,
                    ''
                )::TEXT AS correct_answer,
                te.explain_text::TEXT AS explanation_text,
                te.explain_media_url::TEXT AS explanation_media_url
            FROM study_task t
            LEFT JOIN study_task_explain te ON t.study_task_id = te.study_task_id
            LEFT JOIN study_task_choice stc ON t.study_task_id = stc.study_task_id
            LEFT JOIN study_task_typing stt ON t.study_task_id = stt.study_task_id
            LEFT JOIN study_task_voice stv  ON t.study_task_id = stv.study_task_id
            WHERE t.study_task_id = $1
            "#,
            task_id as i32
        )
        .fetch_optional(pool)
        .await?;

        Ok(res)
    }

    // =========================================================================
    // 4. Status
    // =========================================================================

    pub async fn find_status(
        pool: &PgPool,
        task_id: i64,
        user_id: i64,
    ) -> AppResult<TaskStatusRes> {
        // [FIX] Output Cast: COUNT(*) -> INT (i32)
        let rec = sqlx::query!(
            r#"
            SELECT
                COUNT(*)::INT as "attempts!",
                MAX(study_task_score_log) as best_score,
                BOOL_OR(study_task_is_correct_log) as "is_solved!"
            FROM study_task_log
            WHERE study_task_id = $1 AND user_id = $2
            "#,
            task_id as i32,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(TaskStatusRes {
            task_id: task_id as i32,
            attempts: rec.attempts,
            is_solved: rec.is_solved,
            last_score: rec.best_score.map(|s| s as i32),
        })
    }
}