use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::types::StudyProgram;

use super::dto::{StudyListMeta, StudyListReq, StudyListRes, StudyTaskDetailRes};
use super::repo::StudyRepo;

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
