use sqlx::PgPool;
use tracing::warn;

use crate::error::{AppError, AppResult};
use crate::state::AppState;
use crate::types::{LessonAccess, LessonState};

use super::dto::{
    LessonDetailReq, LessonDetailRes, LessonItemsReq, LessonItemsRes, LessonListMeta, LessonListReq,
    LessonListRes, LessonProgressRes, LessonProgressUpdateReq,
};
use super::repo::LessonRepo;

pub struct LessonService;

impl LessonService {
    pub async fn list_lessons(pool: &PgPool, req: LessonListReq) -> AppResult<LessonListRes> {
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

        let total_count = LessonRepo::count_all(pool).await?;
        let total_pages = if total_count == 0 {
            0
        } else {
            (total_count + per_page - 1) / per_page
        };

        let offset = (page - 1) * per_page;
        let items = LessonRepo::find_all(pool, per_page, offset).await?;

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
        pool: &PgPool,
        lesson_id: i64,
        req: LessonDetailReq,
    ) -> AppResult<LessonDetailRes> {
        let lesson = LessonRepo::find_lesson_by_id(pool, lesson_id)
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

        let total_count = LessonRepo::count_items(pool, lesson_id).await?;
        let total_pages = if total_count == 0 {
            0
        } else {
            (total_count + per_page - 1) / per_page
        };

        let offset = (page - 1) * per_page;
        let items = LessonRepo::find_items(pool, lesson_id, per_page, offset).await?;

        Ok(LessonDetailRes {
            lesson_id: lesson.lesson_id,
            title: lesson.title,
            description: lesson.description,
            lesson_state: lesson.lesson_state,
            lesson_access: lesson.lesson_access,
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
        st: &AppState,
        lesson_id: i64,
        req: LessonItemsReq,
        user_id: Option<i64>,
    ) -> AppResult<LessonItemsRes> {
        // 1. 레슨 존재 및 접근 권한 확인
        let access_info = LessonRepo::find_lesson_access(&st.db, lesson_id)
            .await?
            .ok_or(AppError::NotFound)?;

        // 2. 레슨 상태 검증 (open 상태만 접근 가능)
        if access_info.lesson_state != LessonState::Open {
            return Err(AppError::NotFound); // ready/close 상태는 404 반환
        }

        // 3. 접근 권한 검증
        match access_info.lesson_access {
            LessonAccess::Private => {
                // private: 비공개 - 접근 불가 (admin은 별도 엔드포인트 사용)
                return Err(AppError::Forbidden);
            }
            LessonAccess::Paid => {
                // paid: 유료 - 로그인 필수 + 수강권 확인
                match user_id {
                    None => {
                        return Err(AppError::Unauthorized(
                            "LOGIN_REQUIRED_FOR_PAID_CONTENT".into(),
                        ));
                    }
                    Some(uid) => {
                        // 수강권 확인 (user_course 테이블 연동)
                        let has_access =
                            LessonRepo::has_course_access(&st.db, uid, lesson_id).await?;
                        if !has_access {
                            warn!(
                                user_id = uid,
                                lesson_id,
                                "User attempted to access paid content without subscription"
                            );
                            return Err(AppError::Forbidden);
                        }
                    }
                }
            }
            LessonAccess::Public | LessonAccess::Promote => {
                // public/promote: 누구나 접근 가능
            }
        }

        // 4. 페이지네이션 검증
        let page = req.page.unwrap_or(1);
        let per_page = req.per_page.unwrap_or(20);

        if page <= 0 || per_page <= 0 {
            return Err(AppError::BadRequest("page/per_page must be positive".into()));
        }

        if per_page > 50 {
            return Err(AppError::Unprocessable("per_page exceeds 50".into()));
        }

        // 5. 아이템 조회
        let total_count = LessonRepo::count_items(&st.db, lesson_id).await?;
        let total_pages = if total_count == 0 {
            0
        } else {
            (total_count + per_page - 1) / per_page
        };

        let offset = (page - 1) * per_page;
        let items =
            LessonRepo::find_items_for_study_view(&st.db, lesson_id, per_page, offset).await?;

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
        pool: &PgPool,
        user_id: i64,
        lesson_id: i64,
    ) -> AppResult<LessonProgressRes> {
        let exists = LessonRepo::exists_lesson(pool, lesson_id).await?;
        if !exists {
            return Err(AppError::NotFound);
        }

        let progress = LessonRepo::find_progress(pool, lesson_id, user_id).await?;

        Ok(progress.unwrap_or(LessonProgressRes {
            percent: 0,
            last_seq: None,
            updated_at: None,
        }))
    }

    pub async fn update_lesson_progress(
        pool: &PgPool,
        user_id: i64,
        lesson_id: i64,
        req: LessonProgressUpdateReq,
    ) -> AppResult<LessonProgressRes> {
        let exists = LessonRepo::exists_lesson(pool, lesson_id).await?;
        if !exists {
            return Err(AppError::NotFound);
        }

        if req.percent < 0 || req.percent > 100 {
            return Err(AppError::Unprocessable(
                "percent must be between 0 and 100".into(),
            ));
        }

        if let Some(last_seq) = req.last_seq {
            if last_seq <= 0 {
                return Err(AppError::Unprocessable("last_seq must be positive".into()));
            }
        }

        let progress = LessonRepo::upsert_progress(pool, lesson_id, user_id, req.percent, req.last_seq).await?;

        Ok(progress)
    }
}
