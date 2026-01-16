use sqlx::{PgPool, QueryBuilder, Row};

use crate::error::AppResult;
use crate::types::StudyTaskKind; // StudyProgram 제거

// SubmitAnswerRes 제거
use super::dto::{
    ChoicePayload, StudyListItem, StudyListReq, StudyTaskDetailRes, TaskExplainRes,
    TaskPayload, TaskStatusRes, TypingPayload, VoicePayload,
};

pub struct StudyRepo;

impl StudyRepo {
    // =========================================================================
    // 1. List & Search (Dynamic Query)
    // =========================================================================

    /// 학습 목록 조회 (동적 필터 + 페이징) -> (Data, TotalCount)
    pub async fn find_list_dynamic(
        pool: &PgPool,
        req: &StudyListReq,
    ) -> AppResult<(Vec<StudyListItem>, i64)> {
        // --- 1. Base Query ---
        let mut qb = QueryBuilder::new(
            r#"
            SELECT
                study_id::bigint as study_id,
                study_idx,
                study_program::text as study_program, -- Enum cast
                study_title as title,
                study_subtitle as subtitle,
                created_at
            FROM study
            WHERE study_state = 'open'
            "#,
        );

        // --- 2. Dynamic Filters ---
        if let Some(program) = &req.program {
            qb.push(" AND study_program = ").push_bind(program);
        }

        // --- 3. Count Query (Clone before paging) ---
        let count = qb
            .build_query_as::<StudyListItem>()
            .fetch_all(pool)
            .await?
            .len() as i64; // 간단한 카운트 (데이터가 많으면 COUNT(*) 별도 분리 권장)

        // --- 4. Sort & Paging ---
        // 정렬 (기본값: 최신순)
        match req.sort.as_deref() {
            Some("oldest") => qb.push(" ORDER BY created_at ASC"),
            _ => qb.push(" ORDER BY created_at DESC"),
        };

        let offset = (req.page - 1) * req.per_page;
        qb.push(" LIMIT ").push_bind(req.per_page as i64);
        qb.push(" OFFSET ").push_bind(offset as i64);

        // --- 5. Execution ---
        let items = qb.build_query_as::<StudyListItem>().fetch_all(pool).await?;

        Ok((items, count))
    }

    // =========================================================================
    // 2. Task Detail (Polymorphic Mapping)
    // =========================================================================

    /// 문제 상세 조회
    pub async fn find_task_detail(
        pool: &PgPool,
        task_id: i64,
    ) -> AppResult<Option<StudyTaskDetailRes>> {
        // 복잡한 컬럼 매핑을 위해 내부 Struct 사용
        let row = sqlx::query_as::<_, StudyTaskDetailRow>(
            r#"
            SELECT
                st.study_task_id as task_id,
                st.study_id,
                st.study_task_kind as kind,
                st.study_task_seq as seq,
                
                -- Common
                st.question_text as question,
                st.media_url,
                
                -- Choice Type
                st.choice_1, st.choice_2, st.choice_3, st.choice_4,
                st.choice_answer_index, -- 정답은 클라이언트에 노출하지 않지만, Payload 구성엔 불필요하므로 제외 가능 (여기선 생략)
                st.choice_image_url,
                
                -- Typing Type
                st.typing_answer_text,
                st.typing_image_url,
                
                -- Voice Type
                st.voice_answer_text,
                st.voice_image_url
                
            FROM study_task st
            WHERE st.study_task_id = $1
            "#,
        )
        .bind(task_id)
        .fetch_optional(pool)
        .await?;

        // DB Row -> API Response (Payload 변환)
        Ok(row.and_then(|r| r.map_to_res()))
    }

    // =========================================================================
    // 3. Action & Log (Answer, Status)
    // =========================================================================

    /// 정답 확인용 데이터 조회 (채점 로직)
    /// 반환: (Kind, CorrectAnswerString)
    pub async fn find_answer_info(
        pool: &PgPool,
        task_id: i64,
    ) -> AppResult<Option<(StudyTaskKind, String)>> {
        let row = sqlx::query(
            r#"
            SELECT 
                study_task_kind,
                CASE 
                    WHEN study_task_kind = 'choice' THEN choice_answer_index::text
                    WHEN study_task_kind = 'typing' THEN typing_answer_text
                    WHEN study_task_kind = 'voice' THEN voice_answer_text
                    ELSE NULL
                END as answer
            FROM study_task
            WHERE study_task_id = $1
            "#,
        )
        .bind(task_id)
        .fetch_optional(pool)
        .await?;

        if let Some(r) = row {
            let kind: StudyTaskKind = r.get("study_task_kind");
            let answer: String = r.get("answer");
            Ok(Some((kind, answer)))
        } else {
            Ok(None)
        }
    }

    /// 학습 로그 기록 (Upsert)
    pub async fn upsert_log(
        pool: &PgPool,
        user_id: i64,
        task_id: i64,
        is_correct: bool,
        score: i32,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO study_task_log (
                user_id, study_task_id, 
                is_correct, score, try_count, 
                last_tried_at
            )
            VALUES ($1, $2, $3, $4, 1, NOW())
            ON CONFLICT (user_id, study_task_id) DO UPDATE
            SET
                is_correct = EXCLUDED.is_correct OR study_task_log.is_correct, -- 한번 맞으면 true 유지
                score = GREATEST(study_task_log.score, EXCLUDED.score),
                try_count = study_task_log.try_count + 1,
                last_tried_at = NOW()
            "#,
        )
        .bind(user_id)
        .bind(task_id)
        .bind(is_correct)
        .bind(score)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// 내 문제 풀이 상태 조회
    pub async fn find_task_status(
        pool: &PgPool,
        user_id: i64,
        task_id: i64,
    ) -> AppResult<TaskStatusRes> {
        let row = sqlx::query(
            r#"
            SELECT 
                try_count, is_correct, score
            FROM study_task_log
            WHERE user_id = $1 AND study_task_id = $2
            "#,
        )
        .bind(user_id)
        .bind(task_id)
        .fetch_optional(pool)
        .await?;

        if let Some(r) = row {
            Ok(TaskStatusRes {
                task_id,
                attempts: r.get::<i32, _>("try_count") as i64,
                is_solved: r.get("is_correct"),
                last_score: Some(r.get("score")),
            })
        } else {
            Ok(TaskStatusRes {
                task_id,
                attempts: 0,
                is_solved: false,
                last_score: None,
            })
        }
    }

    // =========================================================================
    // 4. Explanation
    // =========================================================================

    /// 해설 및 정답 상세 조회
    pub async fn find_explanation(
        pool: &PgPool,
        task_id: i64,
    ) -> AppResult<Option<TaskExplainRes>> {
        let row = sqlx::query(
            r#"
            SELECT
                study_task_id as task_id,
                
                -- 정답 텍스트 병합
                CASE 
                    WHEN study_task_kind = 'choice' THEN choice_answer_index::text
                    WHEN study_task_kind = 'typing' THEN typing_answer_text
                    WHEN study_task_kind = 'voice' THEN voice_answer_text
                    ELSE ''
                END as correct_answer,
                
                explanation_text,
                explanation_media_url,
                related_video_url
            FROM study_task
            WHERE study_task_id = $1
            "#,
        )
        .bind(task_id)
        .fetch_optional(pool)
        .await?;

        if let Some(r) = row {
            Ok(Some(TaskExplainRes {
                task_id: r.get("task_id"),
                correct_answer: r.get("correct_answer"),
                explanation_text: r.get("explanation_text"),
                explanation_media_url: r.get("explanation_media_url"),
                related_video_url: r.get("related_video_url"),
            }))
        } else {
            Ok(None)
        }
    }
}

// =============================================================================
// Internal: DB Row Mapping Helpers
// =============================================================================

#[derive(sqlx::FromRow)]
struct StudyTaskDetailRow {
    task_id: i64,
    study_id: i64,
    kind: StudyTaskKind,
    seq: i32,
    question: Option<String>,
    media_url: Option<String>,

    // Choice
    choice_1: Option<String>,
    choice_2: Option<String>,
    choice_3: Option<String>,
    choice_4: Option<String>,
    choice_image_url: Option<String>,

    // Typing
    #[allow(dead_code)]
    typing_answer_text: Option<String>, // 로직상 필요할 수 있어 가져옴
    typing_image_url: Option<String>,

    // Voice
    #[allow(dead_code)]
    voice_answer_text: Option<String>,
    voice_image_url: Option<String>,
}

impl StudyTaskDetailRow {
    fn map_to_res(self) -> Option<StudyTaskDetailRes> {
        let payload = match self.kind {
            StudyTaskKind::Choice => {
                // 필수값 체크 (선택지는 반드시 있어야 함)
                if self.choice_1.is_none() || self.choice_2.is_none() {
                    return None;
                }
                TaskPayload::Choice(ChoicePayload {
                    choice_1: self.choice_1.unwrap(),
                    choice_2: self.choice_2.unwrap(),
                    choice_3: self.choice_3.unwrap_or_default(),
                    choice_4: self.choice_4.unwrap_or_default(),
                    image_url: self.choice_image_url,
                })
            }
            StudyTaskKind::Typing => TaskPayload::Typing(TypingPayload {
                image_url: self.typing_image_url,
            }),
            StudyTaskKind::Voice => TaskPayload::Voice(VoicePayload {
                image_url: self.voice_image_url,
            }),
        };

        Some(StudyTaskDetailRes {
            task_id: self.task_id,
            study_id: self.study_id,
            kind: self.kind,
            seq: self.seq,
            question: self.question,
            media_url: self.media_url,
            payload,
        })
    }
}