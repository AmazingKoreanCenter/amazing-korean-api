use crate::api::video::dto::{
    VideoDetailRes, VideoListMeta, VideoListReq, VideoListRes, VideoProgressRes,
    VideoProgressUpdateReq,
};
use crate::api::video::repo::VideoRepo;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
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
        user_id: i64,
        video_id: i64,
        req: VideoProgressUpdateReq,
    ) -> AppResult<VideoProgressRes> {
        // 비디오 존재 확인
        let video_exists = self.repo.get_video_exists(video_id).await?;
        if !video_exists {
            return Err(AppError::NotFound);
        }

        if let Err(e) = req.validate() {
            return Err(AppError::Unprocessable(e.to_string()));
        }

        let is_completed = req.progress_rate == 100;
        let res = self
            .repo
            .upsert_progress_log(user_id, video_id, req.progress_rate, is_completed)
            .await?;

        Ok(res)
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
