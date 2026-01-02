use crate::error::{AppError, AppResult};

use super::dto::{LessonListMeta, LessonListReq, LessonListRes};
use super::repo::LessonRepo;

pub struct LessonService {
    repo: LessonRepo,
}

impl LessonService {
    pub fn new(repo: LessonRepo) -> Self {
        Self { repo }
    }

    pub async fn list_lessons(&self, req: LessonListReq) -> AppResult<LessonListRes> {
        let page = req.page.unwrap_or(1);
        let per_page = req.per_page.unwrap_or(20);
        let sort = req.sort.as_deref().unwrap_or("lesson_idx");

        if page <= 0 || per_page <= 0 {
            return Err(AppError::BadRequest("page/per_page must be positive".into()));
        }

        if per_page > 50 {
            return Err(AppError::Unprocessable("per_page exceeds 50".into()));
        }

        if sort != "lesson_idx" {
            return Err(AppError::Unprocessable("invalid sort".into()));
        }

        let total_count = self.repo.count_all().await?;
        let total_pages = if total_count == 0 {
            0
        } else {
            (total_count + per_page - 1) / per_page
        };

        let offset = (page - 1) * per_page;
        let items = self.repo.find_all(per_page, offset).await?;

        Ok(LessonListRes {
            items,
            meta: LessonListMeta {
                total_count,
                total_pages,
                current_page: page,
                per_page,
            },
        })
    }
}
