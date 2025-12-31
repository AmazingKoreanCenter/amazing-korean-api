use crate::api::video::dto::{
    VideoDetailRes, VideoListMeta, VideoListReq, VideoListRes, VideoProgressRes,
    VideoProgressUpdateReq,
};
use crate::api::video::repo::VideoRepo;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use sqlx::Row;
use validator::Validate;

pub struct VideoService {
    repo: VideoRepo,
}

impl VideoService {
    pub fn new(repo: VideoRepo) -> Self {
        Self { repo }
    }

    pub async fn get_video_progress(
        &self,
        _st: &AppState,
        user_id: i64,
        video_id: i64,
    ) -> AppResult<VideoProgressRes> {
        // 비디오 존재 확인
        let video_exists = self.repo.get_video_exists(video_id).await?;
        if !video_exists {
            return Err(AppError::NotFound);
        }

        let progress = self.repo.fetch_video_progress(user_id, video_id).await?;

        Ok(progress.unwrap_or_else(|| VideoProgressRes {
            video_id,
            progress_rate: 0,
            is_completed: false,
            last_watched_at: None,
        }))
    }

    pub async fn update_video_progress(
        &self,
        st: &AppState,
        user_id: i64,
        video_id: i64,
        mut req: VideoProgressUpdateReq,
    ) -> AppResult<VideoProgressRes> {
        // 비디오 존재 확인
        let video_exists = self.repo.get_video_exists(video_id).await?;
        if !video_exists {
            return Err(AppError::NotFound);
        }

        // 입력 검증
        if req.last_position_seconds < 0 {
            return Err(AppError::BadRequest(
                "last_position_seconds cannot be negative".to_string(),
            ));
        }
        if let Some(total) = req.total_duration_seconds {
            if total <= 0 {
                return Err(AppError::BadRequest(
                    "total_duration_seconds must be positive".to_string(),
                ));
            }
        }

        // 클램프 & 완료 처리
        if let Some(p) = req.progress {
            req.progress = Some(p.clamp(0, 100));
        } else if let (Some(total), last) = (req.total_duration_seconds, req.last_position_seconds)
        {
            if total > 0 {
                let p = ((last as f64 / total as f64) * 100.0).floor() as i32;
                req.progress = Some(p.clamp(0, 100));
            }
        }
        if req.completed.unwrap_or(false) {
            req.progress = Some(100);
        }

        // 10분 규칙 warn! (video_log에 last_position_seconds가 없으므로 생략)

        // 업서트 호출 (repo는 last_position_seconds: i32 수신)
        let progress_val = req.progress.unwrap_or(0);
        let completed_val = req.completed.unwrap_or(false);

        let row = VideoRepo::upsert_video_progress(
            &st.db,
            user_id,
            video_id,
            progress_val,
            completed_val,
            req.last_position_seconds,
            req.total_duration_seconds,
        )
        .await?;

        let progress_rate: Option<i32> = row.try_get("progress")?;
        Ok(VideoProgressRes {
            video_id: row.try_get("video_id")?,
            progress_rate: progress_rate.unwrap_or(0),
            is_completed: row.try_get("completed")?,
            // DB 함수가 updated_at을 돌려준다고 가정
            last_watched_at: row.try_get("updated_at")?,
        })
    }

    pub async fn get_video_detail(&self, video_id: i64) -> AppResult<VideoDetailRes> {
        let video = self.repo.find_video_by_id(video_id).await?;
        video.ok_or(AppError::NotFound)
    }

    pub async fn list_videos(&self, req: VideoListReq) -> AppResult<VideoListRes> {
        if let Err(e) = req.validate() {
            return Err(AppError::Unprocessable(e.to_string()));
        }

        let page = req.page.unwrap_or(1);
        let per_page = req.per_page.unwrap_or(10);
        let total_count = self.repo.count_open_videos().await?;

        let total_pages = if total_count == 0 {
            0
        } else {
            (total_count + per_page as i64 - 1) / per_page as i64
        };

        let offset = (page - 1) * per_page;
        let data = self
            .repo
            .find_open_videos(per_page, offset, req.sort.as_deref())
            .await?;

        Ok(VideoListRes {
            meta: VideoListMeta {
                total_count,
                total_pages,
                current_page: page,
                per_page,
            },
            data,
        })
    }
}
