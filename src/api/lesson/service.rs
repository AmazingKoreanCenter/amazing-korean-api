use crate::error::{AppError, AppResult};

use super::dto::{
    LessonDetailReq, LessonDetailRes, LessonItemsReq, LessonItemsRes, LessonListMeta, LessonListReq,
    LessonListRes, LessonProgressRes,
};
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

    pub async fn get_lesson_detail(
        &self,
        lesson_id: i64,
        req: LessonDetailReq,
    ) -> AppResult<LessonDetailRes> {
        let lesson = self
            .repo
            .find_lesson_by_id(lesson_id)
            .await?
            .ok_or(AppError::NotFound)?;

        let page = req.page.unwrap_or(1);
        let per_page = req.per_page.unwrap_or(20);

        if page <= 0 || per_page <= 0 {
            return Err(AppError::BadRequest("page/per_page must be positive".into()));
        }

        if per_page > 50 {
            return Err(AppError::Unprocessable("per_page exceeds 50".into()));
        }

        let total_count = self.repo.count_items(lesson_id).await?;
        let total_pages = if total_count == 0 {
            0
        } else {
            (total_count + per_page - 1) / per_page
        };

        let offset = (page - 1) * per_page;
        let items = self.repo.find_items(lesson_id, per_page, offset).await?;

        Ok(LessonDetailRes {
            lesson_id: lesson.lesson_id,
            title: lesson.title,
            description: lesson.description,
            items,
            meta: LessonListMeta {
                total_count,
                total_pages,
                current_page: page,
                per_page,
            },
        })
    }

    pub async fn get_lesson_items(
        &self,
        lesson_id: i64,
        req: LessonItemsReq,
    ) -> AppResult<LessonItemsRes> {
        let exists = self.repo.exists_lesson(lesson_id).await?;
        if !exists {
            return Err(AppError::NotFound);
        }

        let page = req.page.unwrap_or(1);
        let per_page = req.per_page.unwrap_or(20);

        if page <= 0 || per_page <= 0 {
            return Err(AppError::BadRequest("page/per_page must be positive".into()));
        }

        if per_page > 50 {
            return Err(AppError::Unprocessable("per_page exceeds 50".into()));
        }

        let total_count = self.repo.count_items(lesson_id).await?;
        let total_pages = if total_count == 0 {
            0
        } else {
            (total_count + per_page - 1) / per_page
        };

        let offset = (page - 1) * per_page;
        let items = self
            .repo
            .find_items_for_study_view(lesson_id, per_page, offset)
            .await?;

        Ok(LessonItemsRes {
            items,
            meta: LessonListMeta {
                total_count,
                total_pages,
                current_page: page,
                per_page,
            },
        })
    }

    pub async fn get_lesson_progress(
        &self,
        user_id: i64,
        lesson_id: i64,
    ) -> AppResult<LessonProgressRes> {
        let exists = self.repo.exists_lesson(lesson_id).await?;
        if !exists {
            return Err(AppError::NotFound);
        }

        let progress = self.repo.find_progress(lesson_id, user_id).await?;

        Ok(progress.unwrap_or(LessonProgressRes {
            percent: 0,
            last_seq: None,
            updated_at: None,
        }))
    }
}
