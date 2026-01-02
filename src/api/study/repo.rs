use sqlx::{types::Json, PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use super::dto::{
    ChoicePayload, StudyListItem, StudyTaskDetailRes, TaskPayload, TypingPayload, VoicePayload,
};
use crate::types::{StudyProgram, StudyTaskKind, StudyTaskLogAction};

pub struct StudyRepo {
    pool: PgPool,
}

impl StudyRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn count_open_studies(
        &self,
        program: Option<StudyProgram>,
    ) -> Result<i64, sqlx::Error> {
        let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT COUNT(*) FROM study WHERE study_state = 'open'::study_state_enum",
        );

        if let Some(program) = program {
            qb.push(" AND study_program = ").push_bind(program);
        }

        let count = qb.build_query_scalar::<i64>().fetch_one(&self.pool).await?;
        Ok(count)
    }

    pub async fn find_open_studies(
        &self,
        per_page: u64,
        offset: u64,
        program: Option<StudyProgram>,
        order_by: &str,
    ) -> Result<Vec<StudyListItem>, sqlx::Error> {
        let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT study_id::bigint as study_id, study_idx, study_program, \
             study_title as title, study_subtitle as subtitle, \
             study_created_at as created_at \
             FROM study WHERE study_state = 'open'::study_state_enum",
        );

        if let Some(program) = program {
            qb.push(" AND study_program = ").push_bind(program);
        }

        qb.push(" ORDER BY ");
        qb.push(order_by);
        qb.push(" LIMIT ").push_bind(per_page as i64);
        qb.push(" OFFSET ").push_bind(offset as i64);

        let rows = qb
            .build_query_as::<StudyListItem>()
            .fetch_all(&self.pool)
            .await?;
        Ok(rows)
    }

    pub async fn find_task_detail(
        &self,
        task_id: i64,
    ) -> Result<Option<StudyTaskDetailRes>, sqlx::Error> {
        let row = sqlx::query_as::<_, StudyTaskDetailRow>(
            r#"
            SELECT
                st.study_task_id::bigint as task_id,
                st.study_id::bigint as study_id,
                st.study_task_kind as kind,
                st.study_task_seq as seq,
                c.study_task_choice_question as choice_question,
                c.study_task_choice_1 as choice_1,
                c.study_task_choice_2 as choice_2,
                c.study_task_choice_3 as choice_3,
                c.study_task_choice_4 as choice_4,
                c.study_task_choice_audio_url as choice_audio_url,
                c.study_task_choice_image_url as choice_image_url,
                t.study_task_typing_question as typing_question,
                t.study_task_typing_image_url as typing_image_url,
                v.study_task_voice_question as voice_question,
                v.study_task_voice_audio_url as voice_audio_url,
                v.study_task_voice_image_url as voice_image_url
            FROM study_task st
            LEFT JOIN study_task_choice c ON c.study_task_id = st.study_task_id
            LEFT JOIN study_task_typing t ON t.study_task_id = st.study_task_id
            LEFT JOIN study_task_voice v ON v.study_task_id = st.study_task_id
            WHERE st.study_task_id = $1
            "#,
        )
        .bind(task_id)
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else {
            return Ok(None);
        };

        let detail = row.into_detail()?;
        Ok(detail)
    }

    pub async fn find_answer_key(
        &self,
        task_id: i64,
    ) -> Result<Option<StudyTaskAnswerKey>, sqlx::Error> {
        let row = sqlx::query_as::<_, StudyTaskAnswerKey>(
            r#"
            SELECT
                st.study_task_id::bigint as task_id,
                st.study_task_kind as kind,
                c.study_task_choice_correct as choice_correct,
                t.study_task_typing_answer as typing_answer,
                v.study_task_voice_answer as voice_answer
            FROM study_task st
            LEFT JOIN study_task_choice c ON c.study_task_id = st.study_task_id
            LEFT JOIN study_task_typing t ON t.study_task_id = st.study_task_id
            LEFT JOIN study_task_voice v ON v.study_task_id = st.study_task_id
            WHERE st.study_task_id = $1
            "#,
        )
        .bind(task_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    pub async fn exists_task(&self, task_id: i64) -> Result<bool, sqlx::Error> {
        let exists = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM study_task WHERE study_task_id = $1
            )
            "#,
        )
        .bind(task_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(exists)
    }

    pub async fn find_task_status_stats(
        &self,
        task_id: i64,
        user_id: i64,
    ) -> Result<TaskStatusStats, sqlx::Error> {
        let stats = sqlx::query_as::<_, TaskStatusStats>(
            r#"
            SELECT
                COUNT(*) as attempts,
                BOOL_OR(study_task_is_correct_log) as is_solved,
                MAX(study_task_score_log) as best_score
            FROM study_task_log
            WHERE study_task_id = $1
              AND user_id = $2
              AND study_task_action_log = $3
            "#,
        )
        .bind(task_id)
        .bind(user_id)
        .bind(StudyTaskLogAction::Answer)
        .fetch_one(&self.pool)
        .await?;

        Ok(stats)
    }

    pub async fn find_last_attempt(
        &self,
        task_id: i64,
        user_id: i64,
    ) -> Result<Option<TaskLastAttempt>, sqlx::Error> {
        let attempt = sqlx::query_as::<_, TaskLastAttempt>(
            r#"
            SELECT
                study_task_score_log as last_score,
                study_task_created_at_log as last_attempt_at
            FROM study_task_log
            WHERE study_task_id = $1
              AND user_id = $2
              AND study_task_action_log = $3
            ORDER BY study_task_created_at_log DESC
            LIMIT 1
            "#,
        )
        .bind(task_id)
        .bind(user_id)
        .bind(StudyTaskLogAction::Answer)
        .fetch_optional(&self.pool)
        .await?;

        Ok(attempt)
    }

    pub async fn has_attempted(
        &self,
        task_id: i64,
        user_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let attempted = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM study_task_log
                WHERE study_task_id = $1
                  AND user_id = $2
                  AND study_task_action_log = $3
            )
            "#,
        )
        .bind(task_id)
        .bind(user_id)
        .bind(StudyTaskLogAction::Answer)
        .fetch_one(&self.pool)
        .await?;

        Ok(attempted)
    }

    pub async fn find_task_explanation(
        &self,
        task_id: i64,
    ) -> Result<Option<TaskExplanationRow>, sqlx::Error> {
        let row = sqlx::query_as::<_, TaskExplanationRow>(
            r#"
            SELECT
                study_task_id::bigint as task_id,
                explain_text,
                explain_media_url
            FROM study_task_explain
            WHERE study_task_id = $1
            "#,
        )
        .bind(task_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    pub async fn find_login_id_by_session(
        &self,
        session_id: Uuid,
    ) -> Result<Option<i64>, sqlx::Error> {
        let login_id = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT login_id
            FROM login
            WHERE login_session_id = $1
            "#,
        )
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(login_id)
    }

    pub async fn record_submission_tx(
        &self,
        task_id: i64,
        user_id: i64,
        login_id: i64,
        score: i32,
        is_correct: bool,
        payload: serde_json::Value,
    ) -> Result<i32, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let try_no = sqlx::query_scalar::<_, i32>(
            r#"
            INSERT INTO study_task_status (
                study_task_id,
                user_id,
                study_task_status_try,
                study_task_status_best,
                study_task_status_completed,
                study_task_status_last_answer
            )
            VALUES ($1, $2, 1, $3, true, NOW())
            ON CONFLICT (study_task_id, user_id) DO UPDATE
            SET
                study_task_status_try = study_task_status.study_task_status_try + 1,
                study_task_status_best = GREATEST(
                    study_task_status.study_task_status_best,
                    EXCLUDED.study_task_status_best
                ),
                study_task_status_completed = true,
                study_task_status_last_answer = NOW()
            RETURNING study_task_status_try
            "#,
        )
        .bind(task_id)
        .bind(user_id)
        .bind(score)
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO study_task_log (
                study_task_id,
                user_id,
                login_id,
                study_task_action_log,
                study_task_try_no_log,
                study_task_score_log,
                study_task_is_correct_log,
                study_task_payload_log
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(task_id)
        .bind(user_id)
        .bind(login_id)
        .bind(StudyTaskLogAction::Answer)
        .bind(try_no)
        .bind(score)
        .bind(is_correct)
        .bind(Json(payload))
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(try_no)
    }
}

#[derive(Debug, sqlx::FromRow)]
struct StudyTaskDetailRow {
    task_id: i64,
    study_id: i64,
    kind: StudyTaskKind,
    seq: i32,
    choice_question: Option<String>,
    choice_1: Option<String>,
    choice_2: Option<String>,
    choice_3: Option<String>,
    choice_4: Option<String>,
    choice_audio_url: Option<String>,
    choice_image_url: Option<String>,
    typing_question: Option<String>,
    typing_image_url: Option<String>,
    voice_question: Option<String>,
    voice_audio_url: Option<String>,
    voice_image_url: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
pub struct StudyTaskAnswerKey {
    pub task_id: i64,
    pub kind: StudyTaskKind,
    pub choice_correct: Option<i32>,
    pub typing_answer: Option<String>,
    pub voice_answer: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TaskStatusStats {
    pub attempts: i64,
    pub is_solved: Option<bool>,
    pub best_score: Option<i32>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TaskLastAttempt {
    pub last_score: i32,
    pub last_attempt_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct TaskExplanationRow {
    pub task_id: i64,
    pub explain_text: Option<String>,
    pub explain_media_url: Option<String>,
}

impl StudyTaskDetailRow {
    fn into_detail(self) -> Result<Option<StudyTaskDetailRes>, sqlx::Error> {
        let (question, media_url, payload) = match self.kind {
            StudyTaskKind::Choice => {
                let Some(choice_1) = self.choice_1 else {
                    return Ok(None);
                };
                let Some(choice_2) = self.choice_2 else {
                    return Ok(None);
                };
                let Some(choice_3) = self.choice_3 else {
                    return Ok(None);
                };
                let Some(choice_4) = self.choice_4 else {
                    return Ok(None);
                };

                let payload = TaskPayload::Choice(ChoicePayload {
                    choice_1,
                    choice_2,
                    choice_3,
                    choice_4,
                    image_url: self.choice_image_url,
                });

                (self.choice_question, self.choice_audio_url, payload)
            }
            StudyTaskKind::Typing => {
                let payload = TaskPayload::Typing(TypingPayload {
                    image_url: self.typing_image_url,
                });

                (self.typing_question, None, payload)
            }
            StudyTaskKind::Voice => {
                let payload = TaskPayload::Voice(VoicePayload {
                    image_url: self.voice_image_url,
                });

                (self.voice_question, self.voice_audio_url, payload)
            }
        };

        Ok(Some(StudyTaskDetailRes {
            task_id: self.task_id,
            study_id: self.study_id,
            kind: self.kind,
            seq: self.seq,
            question,
            media_url,
            payload,
        }))
    }
}
