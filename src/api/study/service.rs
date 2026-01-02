use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::types::StudyProgram;

use super::dto::{
    StudyListMeta, StudyListReq, StudyListRes, StudyTaskDetailRes, SubmitAnswerReq,
    SubmitAnswerRes, TaskStatusRes, TaskExplainRes,
};
use super::repo::StudyRepo;
use crate::types::StudyTaskKind;
use uuid::Uuid;

pub struct StudyService {
    repo: StudyRepo,
}

impl StudyService {
    pub fn new(repo: StudyRepo) -> Self {
        Self { repo }
    }

    pub async fn list_studies(&self, req: StudyListReq) -> AppResult<StudyListRes> {
        if let Err(e) = req.validate() {
            return Err(AppError::Unprocessable(e.to_string()));
        }

        let page = req.page.unwrap_or(1);
        let per_page = req.per_page.unwrap_or(10);
        let program = Self::parse_program(req.program.as_deref())?;
        let order_by = Self::parse_sort(req.sort.as_deref())?;

        let total = self.repo.count_open_studies(program).await?;
        let total_pages = if total == 0 {
            0
        } else {
            (total + per_page as i64 - 1) / per_page as i64
        };

        let offset = (page - 1) * per_page;
        let data = self
            .repo
            .find_open_studies(per_page, offset, program, order_by)
            .await?;

        Ok(StudyListRes {
            data,
            meta: StudyListMeta {
                page,
                per_page,
                total,
                total_pages,
            },
        })
    }

    pub async fn get_task_detail(&self, task_id: i64) -> AppResult<StudyTaskDetailRes> {
        let task = self
            .repo
            .find_task_detail(task_id)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(task)
    }

    pub async fn submit_answer(
        &self,
        user_id: i64,
        session_id: &str,
        task_id: i64,
        req: SubmitAnswerReq,
    ) -> AppResult<SubmitAnswerRes> {
        let answer_key = self
            .repo
            .find_answer_key(task_id)
            .await?
            .ok_or(AppError::NotFound)?;

        let session_uuid = Uuid::parse_str(session_id)
            .map_err(|_| AppError::Unauthorized("Invalid session".into()))?;
        let login_id = self
            .repo
            .find_login_id_by_session(session_uuid)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid session".into()))?;

        let (is_correct, score, correct_answer, payload) = match (answer_key.kind, req) {
            (StudyTaskKind::Choice, SubmitAnswerReq::Choice { pick }) => {
                if !(1..=4).contains(&pick) {
                    return Err(AppError::Unprocessable(
                        "choice pick must be between 1 and 4".into(),
                    ));
                }
                let correct = answer_key
                    .choice_correct
                    .ok_or_else(|| AppError::Internal("choice answer missing".into()))?;
                let is_correct = pick == correct;
                let score = if is_correct { 100 } else { 0 };
                let correct_answer = if is_correct {
                    None
                } else {
                    Some(correct.to_string())
                };
                let payload = serde_json::json!({ "selected": pick });

                (is_correct, score, correct_answer, payload)
            }
            (StudyTaskKind::Typing, SubmitAnswerReq::Typing { text }) => {
                if text.trim().is_empty() {
                    return Err(AppError::BadRequest("text is empty".into()));
                }
                let answer = answer_key
                    .typing_answer
                    .ok_or_else(|| AppError::Internal("typing answer missing".into()))?;
                let normalized_input = Self::normalize_typing(&text);
                let normalized_answer = Self::normalize_typing(&answer);
                let is_correct = normalized_input == normalized_answer;
                let score = if is_correct { 100 } else { 0 };
                let correct_answer = if is_correct { None } else { Some(answer) };
                let payload = serde_json::json!({ "typed": text });

                (is_correct, score, correct_answer, payload)
            }
            (StudyTaskKind::Voice, SubmitAnswerReq::Voice { audio_url }) => {
                if audio_url.trim().is_empty() {
                    return Err(AppError::BadRequest("audio_url is empty".into()));
                }
                let payload = serde_json::json!({ "audio_url": audio_url });
                (true, 100, None, payload)
            }
            _ => {
                return Err(AppError::Unprocessable(
                    "task kind does not match submission".into(),
                ))
            }
        };

        self.repo
            .record_submission_tx(task_id, user_id, login_id, score, is_correct, payload)
            .await?;

        Ok(SubmitAnswerRes {
            task_id,
            is_correct,
            score,
            correct_answer,
        })
    }

    pub async fn get_task_status(&self, user_id: i64, task_id: i64) -> AppResult<TaskStatusRes> {
        let exists = self.repo.exists_task(task_id).await?;
        if !exists {
            return Err(AppError::NotFound);
        }

        let stats = self.repo.find_task_status_stats(task_id, user_id).await?;
        let last_attempt = self.repo.find_last_attempt(task_id, user_id).await?;

        let attempts = stats.attempts;
        let is_solved = stats.is_solved.unwrap_or(false);
        let best_score = stats.best_score.unwrap_or(0);
        let (last_score, last_attempt_at) = match last_attempt {
            Some(last_attempt) => (last_attempt.last_score, Some(last_attempt.last_attempt_at)),
            None => (0, None),
        };
        let progress = if is_solved { 100 } else { 0 };

        Ok(TaskStatusRes {
            task_id,
            attempts,
            is_solved,
            best_score,
            last_score,
            progress,
            last_attempt_at,
        })
    }

    pub async fn get_task_explanation(
        &self,
        user_id: i64,
        task_id: i64,
    ) -> AppResult<TaskExplainRes> {
        let has_attempted = self.repo.has_attempted(task_id, user_id).await?;
        if !has_attempted {
            return Err(AppError::Forbidden);
        }

        let explanation = self
            .repo
            .find_task_explanation(task_id)
            .await?
            .ok_or(AppError::NotFound)?;

        let resources = explanation
            .explain_media_url
            .clone()
            .map(|url| vec![url])
            .unwrap_or_default();

        Ok(TaskExplainRes {
            task_id: explanation.task_id,
            explanation: explanation.explain_text,
            resources,
        })
    }

    fn normalize_typing(text: &str) -> String {
        text.split_whitespace().collect::<String>()
    }

    fn parse_program(raw: Option<&str>) -> AppResult<Option<StudyProgram>> {
        let Some(raw) = raw else {
            return Ok(None);
        };

        let value = raw.trim();
        if value.is_empty() {
            return Err(AppError::BadRequest("program is empty".into()));
        }

        let program = match value {
            "basic_pronunciation" => StudyProgram::BasicPronunciation,
            "basic_word" => StudyProgram::BasicWord,
            "basic_900" => StudyProgram::Basic900,
            "topik_read" => StudyProgram::TopikRead,
            "topik_listen" => StudyProgram::TopikListen,
            "topik_write" => StudyProgram::TopikWrite,
            "tbc" => StudyProgram::Tbc,
            _ => return Err(AppError::Unprocessable("invalid program".into())),
        };

        Ok(Some(program))
    }

    fn parse_sort(raw: Option<&str>) -> AppResult<&'static str> {
        let Some(raw) = raw else {
            return Ok("study_created_at DESC");
        };

        let value = raw.trim();
        if value.is_empty() {
            return Err(AppError::BadRequest("sort is empty".into()));
        }

        match value {
            "created_at_desc" => Ok("study_created_at DESC"),
            "created_at_asc" => Ok("study_created_at ASC"),
            "title_asc" => Ok("study_title ASC"),
            "title_desc" => Ok("study_title DESC"),
            _ => Err(AppError::Unprocessable("invalid sort".into())),
        }
    }
}
