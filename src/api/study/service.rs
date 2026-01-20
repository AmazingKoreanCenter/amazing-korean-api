use tracing::warn;

use crate::api::auth::extractor::AuthUser;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use crate::types::{StudyProgram, StudyTaskKind, StudyTaskLogAction};

// [Strict Mode] Import DTOs and Repo directly from the verified files
use super::dto::{
    StudyListMeta, StudyListReq, StudyListResp, StudyListSort, StudyTaskDetailRes, SubmitAnswerReq,
    SubmitAnswerRes, TaskExplainRes, TaskStatusRes,
};
use super::repo::StudyRepo;

pub struct StudyService;

impl StudyService {
    // =========================================================================
    // 1. List
    // =========================================================================

    /// 학습 목록 조회
    pub async fn list_studies(st: &AppState, req: StudyListReq) -> AppResult<StudyListResp> {
        let page = req.page.unwrap_or(1);
        let per_page = req.per_page.unwrap_or(10);

        if page == 0 {
            return Err(AppError::BadRequest("page must be >= 1".into()));
        }

        if per_page == 0 {
            return Err(AppError::BadRequest("per_page must be >= 1".into()));
        }

        if per_page > 100 {
            return Err(AppError::Unprocessable("per_page must be <= 100".into()));
        }

        let program = match req.program.as_deref() {
            None => None,
            Some(raw) => {
                let trimmed = raw.trim();
                if trimmed.is_empty() {
                    return Err(AppError::BadRequest("program must not be empty".into()));
                }

                let parsed = parse_study_program(trimmed)
                    .ok_or_else(|| AppError::Unprocessable(invalid_program_message()))?;
                Some(parsed)
            }
        };

        let sort = match req.sort.as_deref() {
            None => StudyListSort::Latest,
            Some(raw) => {
                let trimmed = raw.trim();
                if trimmed.is_empty() {
                    return Err(AppError::BadRequest("sort must not be empty".into()));
                }

                StudyListSort::parse(trimmed)
                    .ok_or_else(|| AppError::Unprocessable(invalid_sort_message()))?
            }
        };

        let (list, total_count) =
            StudyRepo::find_open_studies(&st.db, page, per_page, program, sort).await?;

        let per_page_i64 = i64::from(per_page);
        let total_pages_i64 = if total_count == 0 {
            0
        } else {
            (total_count + per_page_i64 - 1) / per_page_i64
        };

        if total_pages_i64 > u32::MAX as i64 {
            return Err(AppError::Internal("total_pages overflow".into()));
        }

        Ok(StudyListResp {
            list,
            meta: StudyListMeta {
                page,
                per_page,
                total_count,
                total_pages: total_pages_i64 as u32,
            },
        })
    }

    // =========================================================================
    // 2. Detail
    // =========================================================================

    /// 학습 문제 상세 조회
    pub async fn get_study_task(
        st: &AppState,
        task_id: i32,
        auth: Option<AuthUser>,
    ) -> AppResult<StudyTaskDetailRes> {
        let task = StudyRepo::find_task_detail(&st.db, i64::from(task_id)).await?;
        let task = task.ok_or(AppError::NotFound)?;

        if let Some(AuthUser(claims)) = auth {
            if let Err(err) = StudyRepo::log_task_action(
                &st.db,
                claims.sub,
                &claims.session_id,
                task_id,
                StudyTaskLogAction::View,
            )
            .await
            {
                warn!(
                    error = ?err,
                    user_id = claims.sub,
                    task_id,
                    "Failed to log study task view"
                );
            }
        }

        Ok(task)
    }

    // =========================================================================
    // 3. Action (Grading & Log)
    // =========================================================================

    /// 정답 제출 및 채점
    pub async fn submit_answer(
        st: &AppState,
        auth_user: AuthUser,
        task_id: i32,
        req: SubmitAnswerReq,
    ) -> AppResult<SubmitAnswerRes> {
        let AuthUser(claims) = auth_user;
        let answer_key = StudyRepo::find_answer_key(&st.db, task_id).await?;
        let answer_key = answer_key.ok_or(AppError::NotFound)?;

        let req_kind = match &req {
            SubmitAnswerReq::Choice { .. } => StudyTaskKind::Choice,
            SubmitAnswerReq::Typing { .. } => StudyTaskKind::Typing,
            SubmitAnswerReq::Voice { .. } => StudyTaskKind::Voice,
        };

        if req_kind != answer_key.kind {
            return Err(AppError::BadRequest("Task kind mismatch".into()));
        }

        let submitted = match &req {
            SubmitAnswerReq::Choice { pick } => {
                if *pick < 1 || *pick > 4 {
                    return Err(AppError::Unprocessable(
                        "pick must be between 1 and 4".into(),
                    ));
                }
                pick.to_string()
            }
            SubmitAnswerReq::Typing { text } | SubmitAnswerReq::Voice { text } => {
                let trimmed = text.trim();
                if trimmed.is_empty() {
                    return Err(AppError::BadRequest("text must not be empty".into()));
                }
                trimmed.to_string()
            }
        };

        let answer_trimmed = answer_key.answer.trim();
        let is_correct = submitted == answer_trimmed;

        let payload = serde_json::to_value(&req)
            .map_err(|e| AppError::Internal(format!("Failed to serialize payload: {e}")))?;

        StudyRepo::submit_grade_tx(
            &st.db,
            claims.sub,
            &claims.session_id,
            task_id,
            is_correct,
            &payload,
        )
        .await?;

        let correct_answer = if is_correct {
            None
        } else {
            Some(answer_key.answer)
        };

        Ok(SubmitAnswerRes {
            is_correct,
            correct_answer,
            explanation: None,
        })
    }

    /// 내 문제 풀이 상태 조회
    pub async fn get_task_status(
        st: &AppState,
        user_id: i64,
        task_id: i64,
    ) -> AppResult<TaskStatusRes> {
        // Task 존재 확인 (유효하지 않은 task_id 요청 방지)
        if (StudyRepo::find_task_detail(&st.db, task_id).await?).is_none() {
            return Err(AppError::NotFound);
        }

        // repo.rs의 find_status는 DB에서 i32 캐스팅을 수행하여 TaskStatusRes를 반환함
        let status = StudyRepo::find_status(&st.db, task_id, user_id).await?;
        Ok(status)
    }

    // =========================================================================
    // 4. Explanation
    // =========================================================================

    /// 해설 조회
    pub async fn get_task_explanation(
        st: &AppState,
        task_id: i64,
    ) -> AppResult<TaskExplainRes> {
        let explanation = StudyRepo::find_explanation(&st.db, task_id).await?;
        explanation.ok_or(AppError::NotFound)
    }
}

fn parse_study_program(value: &str) -> Option<StudyProgram> {
    match value {
        "basic_pronunciation" => Some(StudyProgram::BasicPronunciation),
        "basic_word" => Some(StudyProgram::BasicWord),
        "basic_900" => Some(StudyProgram::Basic900),
        "topik_read" => Some(StudyProgram::TopikRead),
        "topik_listen" => Some(StudyProgram::TopikListen),
        "topik_write" => Some(StudyProgram::TopikWrite),
        "tbc" => Some(StudyProgram::Tbc),
        _ => None,
    }
}

fn invalid_program_message() -> String {
    "program must be one of: basic_pronunciation, basic_word, basic_900, topik_read, topik_listen, topik_write, tbc".into()
}

fn invalid_sort_message() -> String {
    "sort must be one of: latest, oldest, alphabetical".into()
}
