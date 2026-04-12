use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{PgPool, QueryBuilder};

use crate::error::{AppError, AppResult};
use crate::types::{StudyProgram, StudyTaskKind, StudyTaskLogAction};

use crate::types::{WritingLevel, WritingPracticeType};

use super::dto::{
    ChoicePayload, StudyListSort, StudySummaryDto, StudyTaskDetailRes, StudyTaskSummaryDto,
    TaskPayload, TaskStatusRes, TypingPayload, VoicePayload, WritingDailyStat, WritingLevelStat,
    WritingMistake, WritingPayload, WritingSessionListReq, WritingSessionRes, WritingWeakChar,
};

pub struct StudyRepo;

#[derive(Debug)]
pub struct AnswerKeyDto {
    pub kind: StudyTaskKind,
    pub answer: String,
}

#[derive(Debug)]
pub struct TaskExplainRow {
    pub explain_title: Option<String>,
    pub explain_text: Option<String>,
    pub explain_media_url: Option<String>,
}

// 내부 사용용 Row 구조체 (DB 조회 결과 매핑)
#[derive(sqlx::FromRow)]
struct StudyTaskDetailRow {
    task_id: i32,  // [FIX] DTO Type Mismatch: i64 -> i32
    study_id: i32, // [FIX] DTO Type Mismatch: i64 -> i32
    kind: StudyTaskKind,
    seq: i32,
    created_at: DateTime<Utc>,

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
    typing_image_url: Option<String>,

    // Voice
    voice_audio_url: Option<String>,
    voice_image_url: Option<String>,

    // Writing
    writing_level: Option<WritingLevel>,
    writing_practice_type: Option<WritingPracticeType>,
    writing_answer: Option<String>,
    writing_hint: Option<String>,
    writing_keyboard_visible: Option<bool>,
    writing_image_url: Option<String>,
    writing_audio_url: Option<String>,
}

impl StudyTaskDetailRow {
    fn map_to_res(self) -> Option<StudyTaskDetailRes> {
        let question = self.question.unwrap_or_default();
        let payload = match self.kind {
            StudyTaskKind::Choice => {
                if self.choice_1.is_none() || self.choice_2.is_none() {
                    return None;
                }
                TaskPayload::Choice(ChoicePayload {
                    question,
                    choice_1: self.choice_1.unwrap_or_default(),
                    choice_2: self.choice_2.unwrap_or_default(),
                    choice_3: self.choice_3.unwrap_or_default(),
                    choice_4: self.choice_4.unwrap_or_default(),
                    audio_url: self.choice_audio_url,
                    image_url: self.choice_image_url,
                })
            }
            StudyTaskKind::Typing => TaskPayload::Typing(TypingPayload {
                question,
                image_url: self.typing_image_url,
            }),
            StudyTaskKind::Voice => TaskPayload::Voice(VoicePayload {
                question,
                audio_url: self.voice_audio_url,
                image_url: self.voice_image_url,
            }),
            StudyTaskKind::Writing => {
                let level = self.writing_level?;
                let practice_type = self.writing_practice_type?;
                // 초급에서만 answer를 클라이언트에 전송 (실시간 피드백용)
                let answer = if level == WritingLevel::Beginner {
                    self.writing_answer
                } else {
                    None
                };
                TaskPayload::Writing(WritingPayload {
                    prompt: question,
                    answer,
                    hint: self.writing_hint,
                    level,
                    practice_type,
                    keyboard_visible: self.writing_keyboard_visible.unwrap_or(true),
                    image_url: self.writing_image_url,
                    audio_url: self.writing_audio_url,
                })
            }
        };

        Some(StudyTaskDetailRes {
            task_id: self.task_id,
            study_id: self.study_id,
            kind: self.kind,
            seq: self.seq,
            created_at: self.created_at,
            payload,
        })
    }
}

impl StudyRepo {
    // =========================================================================
    // 1. List & Search (Dynamic Query)
    // =========================================================================

    pub async fn find_open_studies(
        pool: &PgPool,
        page: u32,
        per_page: u32,
        program: Option<StudyProgram>,
        sort: StudyListSort,
    ) -> AppResult<(Vec<StudySummaryDto>, i64)> {
        // ---------------------------------------------------------
        // A. Count Query
        // ---------------------------------------------------------
        let mut qb_count = QueryBuilder::new(
            "SELECT COUNT(*) FROM study WHERE study_state = 'open'::study_state_enum",
        );

        if let Some(program) = program {
            qb_count.push(" AND study_program = ").push_bind(program);
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
                study_id::INT AS study_id,
                study_idx::TEXT AS study_idx,
                study_program AS program,
                study_title::TEXT AS title,
                study_subtitle::TEXT AS subtitle,
                study_state AS state,
                study_created_at AS created_at
            FROM study
            WHERE study_state = 'open'::study_state_enum
            "#,
        );

        if let Some(program) = program {
            qb_list.push(" AND study_program = ").push_bind(program);
        }

        match sort {
            StudyListSort::Latest => qb_list.push(" ORDER BY study_created_at DESC"),
            StudyListSort::Oldest => qb_list.push(" ORDER BY study_created_at ASC"),
            StudyListSort::Alphabetical => qb_list
                .push(" ORDER BY study_title ASC NULLS LAST, study_idx ASC"),
        };

        let offset = (i64::from(page) - 1) * i64::from(per_page);
        qb_list.push(" LIMIT ").push_bind(i64::from(per_page));
        qb_list.push(" OFFSET ").push_bind(offset);

        let list = qb_list
            .build_query_as::<StudySummaryDto>()
            .fetch_all(pool)
            .await?;

        Ok((list, count))
    }

    // =========================================================================
    // 1-2. Study Detail (Study + Task List)
    // =========================================================================

    pub async fn get_study_by_id(
        pool: &PgPool,
        study_id: i32,
    ) -> AppResult<Option<StudySummaryDto>> {
        let row = sqlx::query_as::<_, StudySummaryDto>(
            r#"
            SELECT
                study_id::INT AS study_id,
                study_idx::TEXT AS study_idx,
                study_program AS program,
                study_title::TEXT AS title,
                study_subtitle::TEXT AS subtitle,
                study_state AS state,
                study_created_at AS created_at
            FROM study
            WHERE study_id = $1 AND study_state = 'open'::study_state_enum
            "#,
        )
        .bind(study_id)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    pub async fn get_tasks_by_study_id(
        pool: &PgPool,
        study_id: i32,
        page: u32,
        per_page: u32,
    ) -> AppResult<(Vec<StudyTaskSummaryDto>, i64)> {
        // Count
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM study_task WHERE study_id = $1",
        )
        .bind(study_id)
        .fetch_one(pool)
        .await?;

        // List
        let offset = (i64::from(page) - 1) * i64::from(per_page);
        let tasks = sqlx::query_as::<_, StudyTaskSummaryDto>(
            r#"
            SELECT
                study_task_id::INT AS task_id,
                study_task_kind AS kind,
                study_task_seq::INT AS seq
            FROM study_task
            WHERE study_id = $1
            ORDER BY study_task_seq ASC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(study_id)
        .bind(i64::from(per_page))
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok((tasks, count))
    }

    // =========================================================================
    // 2. Submit & Grading
    // =========================================================================

    pub async fn find_answer_key(
        pool: &PgPool,
        task_id: i32,
    ) -> AppResult<Option<AnswerKeyDto>> {
        #[derive(sqlx::FromRow)]
        struct AnswerKeyRow {
            kind: StudyTaskKind,
            answer: Option<String>,
        }

        let row = sqlx::query_as!(
            AnswerKeyRow,
            r#"
            SELECT
                t.study_task_kind AS "kind!: StudyTaskKind",
                CASE t.study_task_kind
                    WHEN 'choice' THEN stc.study_task_choice_answer::TEXT
                    WHEN 'typing' THEN stt.study_task_typing_answer
                    WHEN 'voice' THEN stv.study_task_voice_answer
                    WHEN 'writing' THEN stw.study_task_writing_answer
                END AS "answer?"
            FROM study_task t
            INNER JOIN study s ON t.study_id = s.study_id
            LEFT JOIN study_task_choice stc ON t.study_task_id = stc.study_task_id
            LEFT JOIN study_task_typing stt ON t.study_task_id = stt.study_task_id
            LEFT JOIN study_task_voice stv  ON t.study_task_id = stv.study_task_id
            LEFT JOIN study_task_writing stw ON t.study_task_id = stw.study_task_id
            WHERE t.study_task_id = $1
              AND s.study_state = 'open'::study_state_enum
            "#,
            task_id
        )
        .fetch_optional(pool)
        .await?;

        match row {
            Some(row) => {
                let answer = row
                    .answer
                    .ok_or_else(|| AppError::Internal("Answer key missing".into()))?;
                Ok(Some(AnswerKeyDto {
                    kind: row.kind,
                    answer,
                }))
            }
            None => Ok(None),
        }
    }

    pub async fn submit_grade_tx(
        pool: &PgPool,
        user_id: i64,
        session_id: &str,
        task_id: i32,
        is_correct: bool,
        payload: &Value,
    ) -> AppResult<()> {
        let mut tx = pool.begin().await?;

        let try_count: i32 = sqlx::query_scalar(
            r#"
            INSERT INTO study_task_status (
                study_task_id,
                user_id,
                study_task_status_try_count,
                study_task_status_is_solved,
                study_task_status_last_attempt_at
            )
            VALUES ($1, $2, 1, $3, NOW())
            ON CONFLICT (study_task_id, user_id) DO UPDATE
            SET study_task_status_try_count = study_task_status.study_task_status_try_count + 1,
                study_task_status_is_solved =
                    study_task_status.study_task_status_is_solved OR EXCLUDED.study_task_status_is_solved,
                study_task_status_last_attempt_at = NOW()
            RETURNING study_task_status_try_count
            "#,
        )
        .bind(task_id)
        .bind(user_id)
        .bind(is_correct)
        .fetch_one(&mut *tx)
        .await?;

        let log_res = sqlx::query(
            r#"
            INSERT INTO study_task_log (
                study_task_id,
                user_id,
                login_id,
                study_task_action_log,
                study_task_try_no_log,
                study_task_is_correct_log,
                study_task_answer_log
            )
            SELECT
                $1,
                $2,
                l.login_id,
                $3,
                $4,
                $5,
                $6
            FROM login l
            WHERE l.login_session_id = CAST($7 AS uuid)
              AND l.user_id = $2
            "#,
        )
        .bind(task_id)
        .bind(user_id)
        .bind(StudyTaskLogAction::Finish)
        .bind(try_count)
        .bind(is_correct)
        .bind(payload)
        .bind(session_id)
        .execute(&mut *tx)
        .await?;

        if log_res.rows_affected() == 0 {
            return Err(AppError::Internal("Login record not found".into()));
        }

        tx.commit().await?;
        Ok(())
    }

    // =========================================================================
    // 3. Task Detail
    // =========================================================================

    pub async fn find_task_detail(
        pool: &PgPool,
        task_id: i64,
    ) -> AppResult<Option<StudyTaskDetailRes>> {
        // [FIX] Output Cast: study_task_id -> INT (i32)
        // [FIX] Input Cast: task_id arg -> i32
        // [FIX] JOIN study table and check study_state = 'open'
        let row = sqlx::query_as!(
            StudyTaskDetailRow,
            r#"
            SELECT
                t.study_task_id::INT AS task_id,
                t.study_id::INT AS study_id,
                t.study_task_kind AS "kind!: StudyTaskKind",
                t.study_task_seq AS seq,
                t.study_task_created_at AS created_at,

                -- Question: 이미 "question?"로 되어 있어서 OK
                COALESCE(stc.study_task_choice_question, stt.study_task_typing_question, stv.study_task_voice_question, stw.study_task_writing_prompt)::TEXT AS "question?",

                -- [수정] Choice Fields: LEFT JOIN이므로 값이 없을 수 있음 -> "?" 추가
                stc.study_task_choice_1::TEXT AS "choice_1?",
                stc.study_task_choice_2::TEXT AS "choice_2?",
                stc.study_task_choice_3::TEXT AS "choice_3?",
                stc.study_task_choice_4::TEXT AS "choice_4?",
                stc.study_task_choice_audio_url::TEXT AS "choice_audio_url?",
                stc.study_task_choice_image_url::TEXT AS "choice_image_url?",

                -- [수정] Typing Fields: "?" 추가
                stt.study_task_typing_image_url::TEXT AS "typing_image_url?",

                -- [수정] Voice Fields: "?" 추가
                stv.study_task_voice_audio_url::TEXT AS "voice_audio_url?",
                stv.study_task_voice_image_url::TEXT AS "voice_image_url?",

                -- Writing Fields
                stw.study_task_writing_level AS "writing_level?: WritingLevel",
                stw.study_task_writing_practice_type AS "writing_practice_type?: WritingPracticeType",
                stw.study_task_writing_answer::TEXT AS "writing_answer?",
                stw.study_task_writing_hint::TEXT AS "writing_hint?",
                stw.study_task_writing_keyboard_visible AS "writing_keyboard_visible?",
                stw.study_task_writing_image_url::TEXT AS "writing_image_url?",
                stw.study_task_writing_audio_url::TEXT AS "writing_audio_url?"

            FROM study_task t
            INNER JOIN study s ON t.study_id = s.study_id
            LEFT JOIN study_task_choice stc ON t.study_task_id = stc.study_task_id
            LEFT JOIN study_task_typing stt ON t.study_task_id = stt.study_task_id
            LEFT JOIN study_task_voice stv  ON t.study_task_id = stv.study_task_id
            LEFT JOIN study_task_writing stw ON t.study_task_id = stw.study_task_id
            WHERE t.study_task_id = $1
              AND s.study_state = 'open'::study_state_enum
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

    pub async fn log_task_action(
        pool: &PgPool,
        user_id: i64,
        session_id: &str,
        task_id: i32,
        action: StudyTaskLogAction,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO study_task_log (
                study_task_id,
                user_id,
                login_id,
                study_task_action_log
            )
            SELECT
                $1,
                $2,
                l.login_id,
                $3
            FROM login l
            WHERE l.login_session_id = CAST($4 AS uuid)
              AND l.user_id = $2
            "#,
        )
        .bind(task_id)
        .bind(user_id)
        .bind(action)
        .bind(session_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    // =========================================================================
    // 3. Explanation
    // =========================================================================

    pub async fn get_try_count(pool: &PgPool, user_id: i64, task_id: i32) -> AppResult<i32> {
        let try_count = sqlx::query_scalar!(
            r#"
            SELECT sts.study_task_status_try_count
            FROM study_task_status sts
            INNER JOIN study_task t ON sts.study_task_id = t.study_task_id
            INNER JOIN study s ON t.study_id = s.study_id
            WHERE sts.study_task_id = $1
              AND sts.user_id = $2
              AND s.study_state = 'open'::study_state_enum
            "#,
            task_id,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(try_count.unwrap_or(0))
    }

    pub async fn find_task_explain(
        pool: &PgPool,
        task_id: i32,
    ) -> AppResult<Option<TaskExplainRow>> {
        let row = sqlx::query_as!(
            TaskExplainRow,
            r#"
            SELECT
                e.explain_title::TEXT AS "explain_title?",
                e.explain_text::TEXT AS "explain_text?",
                e.explain_media_url::TEXT AS "explain_media_url?"
            FROM study_task_explain e
            INNER JOIN study_task t ON e.study_task_id = t.study_task_id
            INNER JOIN study s ON t.study_id = s.study_id
            WHERE e.study_task_id = $1
              AND s.study_state = 'open'::study_state_enum
            "#,
            task_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    // =========================================================================
    // 4. Status
    // =========================================================================

    pub async fn exists_task(pool: &PgPool, task_id: i32) -> AppResult<bool> {
        let exists = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM study_task t
                INNER JOIN study s ON t.study_id = s.study_id
                WHERE t.study_task_id = $1
                  AND s.study_state = 'open'::study_state_enum
            )
            "#,
        )
        .bind(task_id)
        .fetch_one(pool)
        .await?;

        Ok(exists)
    }

    pub async fn find_task_status(
        pool: &PgPool,
        user_id: i64,
        task_id: i32,
    ) -> AppResult<Option<TaskStatusRes>> {
        let status = sqlx::query_as!(
            TaskStatusRes,
            r#"
            SELECT
                study_task_status_try_count AS "try_count!",
                study_task_status_is_solved AS "is_solved!",
                study_task_status_last_attempt_at AS "last_attempt_at?"
            FROM study_task_status
            WHERE study_task_id = $1 AND user_id = $2
            "#,
            task_id,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(status)
    }

    // =========================================================================
    // 5. Writing Practice Sessions
    // =========================================================================

    /// writing 태스크가 존재하고 study가 open 상태인지 검증
    pub async fn exists_writing_task(pool: &PgPool, task_id: i32) -> AppResult<bool> {
        let exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM study_task t
                INNER JOIN study s ON t.study_id = s.study_id
                INNER JOIN study_task_writing stw ON t.study_task_id = stw.study_task_id
                WHERE t.study_task_id = $1
                  AND t.study_task_kind = 'writing'::study_task_kind_enum
                  AND s.study_state = 'open'::study_state_enum
            ) AS "exists!"
            "#,
            task_id
        )
        .fetch_one(pool)
        .await?;

        Ok(exists)
    }

    pub async fn create_writing_session(
        pool: &PgPool,
        user_id: i64,
        study_task_id: Option<i32>,
        writing_level: WritingLevel,
        writing_practice_type: WritingPracticeType,
    ) -> AppResult<WritingSessionRes> {
        let row = sqlx::query!(
            r#"
            INSERT INTO writing_practice_session (
                user_id,
                study_task_id,
                writing_level,
                writing_practice_type
            )
            VALUES ($1, $2, $3, $4)
            RETURNING
                session_id,
                user_id,
                study_task_id,
                writing_level AS "writing_level!: WritingLevel",
                writing_practice_type AS "writing_practice_type!: WritingPracticeType",
                started_at,
                finished_at,
                total_chars,
                correct_chars,
                accuracy_rate::float8 AS "accuracy_rate!",
                chars_per_minute::float8 AS "chars_per_minute!",
                mistakes
            "#,
            user_id,
            study_task_id,
            writing_level as WritingLevel,
            writing_practice_type as WritingPracticeType,
        )
        .fetch_one(pool)
        .await?;

        let mistakes: Vec<WritingMistake> =
            serde_json::from_value(row.mistakes).unwrap_or_default();

        Ok(WritingSessionRes {
            session_id: row.session_id,
            user_id: row.user_id,
            study_task_id: row.study_task_id,
            writing_level: row.writing_level,
            writing_practice_type: row.writing_practice_type,
            started_at: row.started_at,
            finished_at: row.finished_at,
            total_chars: row.total_chars,
            correct_chars: row.correct_chars,
            accuracy_rate: row.accuracy_rate,
            chars_per_minute: row.chars_per_minute,
            mistakes,
        })
    }

    /// 세션 완료 (user_id로 소유권 검증). 미존재/타 유저 세션이면 None 반환.
    #[allow(clippy::too_many_arguments)]
    pub async fn finish_writing_session(
        pool: &PgPool,
        session_id: i64,
        user_id: i64,
        total_chars: i32,
        correct_chars: i32,
        accuracy_rate: f64,
        chars_per_minute: f64,
        mistakes_json: Value,
    ) -> AppResult<Option<WritingSessionRes>> {
        let row = sqlx::query!(
            r#"
            UPDATE writing_practice_session
            SET finished_at = NOW(),
                total_chars = $3,
                correct_chars = $4,
                accuracy_rate = $5::float8::numeric,
                chars_per_minute = $6::float8::numeric,
                mistakes = $7
            WHERE session_id = $1
              AND user_id = $2
            RETURNING
                session_id,
                user_id,
                study_task_id,
                writing_level AS "writing_level!: WritingLevel",
                writing_practice_type AS "writing_practice_type!: WritingPracticeType",
                started_at,
                finished_at,
                total_chars,
                correct_chars,
                accuracy_rate::float8 AS "accuracy_rate!",
                chars_per_minute::float8 AS "chars_per_minute!",
                mistakes
            "#,
            session_id,
            user_id,
            total_chars,
            correct_chars,
            accuracy_rate,
            chars_per_minute,
            mistakes_json,
        )
        .fetch_optional(pool)
        .await?;

        let Some(row) = row else {
            return Ok(None);
        };

        let mistakes: Vec<WritingMistake> =
            serde_json::from_value(row.mistakes).unwrap_or_default();

        Ok(Some(WritingSessionRes {
            session_id: row.session_id,
            user_id: row.user_id,
            study_task_id: row.study_task_id,
            writing_level: row.writing_level,
            writing_practice_type: row.writing_practice_type,
            started_at: row.started_at,
            finished_at: row.finished_at,
            total_chars: row.total_chars,
            correct_chars: row.correct_chars,
            accuracy_rate: row.accuracy_rate,
            chars_per_minute: row.chars_per_minute,
            mistakes,
        }))
    }

    pub async fn list_writing_sessions(
        pool: &PgPool,
        user_id: i64,
        req: &WritingSessionListReq,
    ) -> AppResult<(Vec<WritingSessionRes>, i64)> {
        let page = req.page.unwrap_or(1);
        let per_page = req.per_page.unwrap_or(20);
        let offset = (i64::from(page) - 1) * i64::from(per_page);
        let finished_only = req.finished_only.unwrap_or(false);

        // Count
        let mut qb_count = QueryBuilder::new(
            "SELECT COUNT(*) FROM writing_practice_session WHERE user_id = ",
        );
        qb_count.push_bind(user_id);
        if let Some(level) = req.level {
            qb_count.push(" AND writing_level = ").push_bind(level);
        }
        if finished_only {
            qb_count.push(" AND finished_at IS NOT NULL");
        }

        let total: i64 = qb_count
            .build_query_scalar()
            .fetch_one(pool)
            .await?;

        // List
        let mut qb = QueryBuilder::new(
            r#"
            SELECT
                session_id,
                user_id,
                study_task_id,
                writing_level,
                writing_practice_type,
                started_at,
                finished_at,
                total_chars,
                correct_chars,
                accuracy_rate::float8 AS accuracy_rate,
                chars_per_minute::float8 AS chars_per_minute,
                mistakes
            FROM writing_practice_session
            WHERE user_id = "#,
        );
        qb.push_bind(user_id);
        if let Some(level) = req.level {
            qb.push(" AND writing_level = ").push_bind(level);
        }
        if finished_only {
            qb.push(" AND finished_at IS NOT NULL");
        }
        qb.push(" ORDER BY started_at DESC LIMIT ")
            .push_bind(i64::from(per_page))
            .push(" OFFSET ")
            .push_bind(offset);

        use sqlx::Row;
        let rows = qb.build().fetch_all(pool).await?;

        let mut list: Vec<WritingSessionRes> = Vec::with_capacity(rows.len());
        for r in rows {
            let mistakes_json: Value = r.try_get("mistakes")?;
            let mistakes: Vec<WritingMistake> =
                serde_json::from_value(mistakes_json).unwrap_or_default();
            list.push(WritingSessionRes {
                session_id: r.try_get("session_id")?,
                user_id: r.try_get("user_id")?,
                study_task_id: r.try_get::<Option<i32>, _>("study_task_id")?,
                writing_level: r.try_get("writing_level")?,
                writing_practice_type: r.try_get("writing_practice_type")?,
                started_at: r.try_get("started_at")?,
                finished_at: r.try_get::<Option<DateTime<Utc>>, _>("finished_at")?,
                total_chars: r.try_get("total_chars")?,
                correct_chars: r.try_get("correct_chars")?,
                accuracy_rate: r.try_get("accuracy_rate")?,
                chars_per_minute: r.try_get("chars_per_minute")?,
                mistakes,
            });
        }

        Ok((list, total))
    }

    /// 전체 세션 요약: 총 건수, 평균 정확도/CPM
    pub async fn writing_stats_overall(
        pool: &PgPool,
        user_id: i64,
        days: i32,
    ) -> AppResult<(i64, f64, f64)> {
        let row = sqlx::query!(
            r#"
            SELECT
                COUNT(*)::BIGINT AS "total!",
                COALESCE(AVG(accuracy_rate)::float8, 0.0) AS "avg_accuracy!",
                COALESCE(AVG(chars_per_minute)::float8, 0.0) AS "avg_cpm!"
            FROM writing_practice_session
            WHERE user_id = $1
              AND finished_at IS NOT NULL
              AND started_at >= NOW() - make_interval(days => $2)
            "#,
            user_id,
            days,
        )
        .fetch_one(pool)
        .await?;

        Ok((row.total, row.avg_accuracy, row.avg_cpm))
    }

    /// 레벨별 집계
    pub async fn writing_stats_by_level(
        pool: &PgPool,
        user_id: i64,
        days: i32,
    ) -> AppResult<Vec<WritingLevelStat>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                writing_level AS "writing_level!: WritingLevel",
                COUNT(*)::BIGINT AS "sessions!",
                COALESCE(AVG(accuracy_rate)::float8, 0.0) AS "avg_accuracy!",
                COALESCE(AVG(chars_per_minute)::float8, 0.0) AS "avg_cpm!"
            FROM writing_practice_session
            WHERE user_id = $1
              AND finished_at IS NOT NULL
              AND started_at >= NOW() - make_interval(days => $2)
            GROUP BY writing_level
            ORDER BY writing_level
            "#,
            user_id,
            days,
        )
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| WritingLevelStat {
                writing_level: r.writing_level,
                sessions: r.sessions,
                avg_accuracy: r.avg_accuracy,
                avg_cpm: r.avg_cpm,
            })
            .collect())
    }

    /// 일별 집계 (최근 days일)
    pub async fn writing_stats_daily(
        pool: &PgPool,
        user_id: i64,
        days: i32,
    ) -> AppResult<Vec<WritingDailyStat>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                TO_CHAR(DATE_TRUNC('day', started_at), 'YYYY-MM-DD') AS "day!",
                COUNT(*)::BIGINT AS "sessions!",
                COALESCE(AVG(accuracy_rate)::float8, 0.0) AS "avg_accuracy!",
                COALESCE(AVG(chars_per_minute)::float8, 0.0) AS "avg_cpm!"
            FROM writing_practice_session
            WHERE user_id = $1
              AND finished_at IS NOT NULL
              AND started_at >= NOW() - make_interval(days => $2)
            GROUP BY DATE_TRUNC('day', started_at)
            ORDER BY DATE_TRUNC('day', started_at) ASC
            "#,
            user_id,
            days,
        )
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| WritingDailyStat {
                day: r.day,
                sessions: r.sessions,
                avg_accuracy: r.avg_accuracy,
                avg_cpm: r.avg_cpm,
            })
            .collect())
    }

    /// 취약 글자 Top N (mistakes JSONB 펼쳐서 expected 기준 집계)
    pub async fn writing_stats_weak_chars(
        pool: &PgPool,
        user_id: i64,
        days: i32,
        limit: i32,
    ) -> AppResult<Vec<WritingWeakChar>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                m.value->>'expected' AS "expected?",
                COUNT(*)::BIGINT AS "miss_count!"
            FROM writing_practice_session s
            CROSS JOIN LATERAL jsonb_array_elements(s.mistakes) AS m(value)
            WHERE s.user_id = $1
              AND s.finished_at IS NOT NULL
              AND s.started_at >= NOW() - make_interval(days => $2)
              AND m.value->>'expected' IS NOT NULL
            GROUP BY m.value->>'expected'
            ORDER BY COUNT(*) DESC
            LIMIT $3
            "#,
            user_id,
            days,
            i64::from(limit),
        )
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .filter_map(|r| {
                r.expected.map(|expected| WritingWeakChar {
                    expected,
                    miss_count: r.miss_count,
                })
            })
            .collect())
    }
}
