use validator::Validate;

use crate::api::video::dto::{
    VideoDetailRes, VideoListMeta, VideoListReq, VideoListRes, VideoProgressRes,
    VideoProgressUpdateReq,
};
use crate::api::video::repo::VideoRepo;
use crate::error::{AppError, AppResult};
use crate::state::AppState;

pub struct VideoService;

impl VideoService {
    /// 비디오 목록 조회 (검색 + 페이징 + 필터)
    pub async fn list_videos(st: &AppState, req: VideoListReq) -> AppResult<VideoListRes> {
        // 1. Validation
        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(e.to_string()));
        }

        // 2. Repo Call (Data + Total Count)
        let (data, total_count) = VideoRepo::find_list_dynamic(&st.db, &req).await?;

        // 3. Calc Meta
        let total_pages = if total_count == 0 {
            0
        } else {
            (total_count + req.per_page as i64 - 1) / req.per_page as i64
        };

        Ok(VideoListRes {
            meta: VideoListMeta {
                total_count,
                total_pages,
                current_page: req.page,
                per_page: req.per_page,
            },
            data,
        })
    }

    /// 비디오 상세 조회
    pub async fn get_video_detail(st: &AppState, video_id: i64) -> AppResult<VideoDetailRes> {
        let video = VideoRepo::find_detail_by_id(&st.db, video_id).await?;
        video.ok_or(AppError::NotFound)
    }

    /// 내 진도율 조회
    pub async fn get_video_progress(
        st: &AppState,
        user_id: i64,
        video_id: i64,
    ) -> AppResult<VideoProgressRes> {
        // 1. 비디오 존재 확인
        let exists = VideoRepo::exists_by_id(&st.db, video_id).await?;
        if !exists {
            return Err(AppError::NotFound);
        }

        // 2. 진도율 조회
        let progress = VideoRepo::find_progress(&st.db, user_id, video_id).await?;

        // 3. 없으면 기본값 반환 (0%)
        Ok(progress.unwrap_or_else(|| VideoProgressRes {
            video_id,
            progress_rate: 0,
            is_completed: false,
            last_watched_at: None,
        }))
    }

    /// 내 진도율 업데이트
    pub async fn update_video_progress(
        st: &AppState,
        user_id: i64,
        video_id: i64,
        req: VideoProgressUpdateReq,
    ) -> AppResult<VideoProgressRes> {
        // 1. Validation
        if let Err(e) = req.validate() {
            return Err(AppError::Unprocessable(e.to_string()));
        }

        // 2. 비디오 존재 확인
        let exists = VideoRepo::exists_by_id(&st.db, video_id).await?;
        if !exists {
            return Err(AppError::NotFound);
        }

        // 3. 완료 여부 판단 (100%면 완료)
        let is_completed = req.progress_rate >= 100;

        // 4. Upsert Log
        let res = VideoRepo::upsert_progress(
            &st.db,
            user_id,
            video_id,
            req.progress_rate,
            is_completed,
        )
        .await?;

        Ok(res)
    }
}