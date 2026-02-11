use validator::Validate;

use crate::api::admin::translation::repo::TranslationRepo;
use crate::api::video::dto::{
    VideoDetailRes, VideoListMeta, VideoListReq, VideoListRes, VideoProgressRes,
    VideoProgressUpdateReq,
};
use crate::api::video::repo::VideoRepo;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use crate::types::ContentType;

pub struct VideoService;

impl VideoService {
    /// 비디오 목록 조회 (검색 + 페이징 + 필터)
    pub async fn list_videos(st: &AppState, req: VideoListReq) -> AppResult<VideoListRes> {
        // 1. Validation
        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(e.to_string()));
        }

        // 2. Repo Call (Data + Total Count)
        let (mut data, total_count) = VideoRepo::list_videos(&st.db, &req).await?;

        // 2-1. 번역 주입
        if let Some(lang) = req.lang {
            let ids: Vec<i64> = data.iter().map(|v| v.video_id).collect();
            let translations = TranslationRepo::find_translations_for_contents(
                &st.db,
                ContentType::Video,
                &ids,
                lang,
            )
            .await?;

            for item in data.iter_mut() {
                if let Some(t) = translations.get(&(item.video_id, "title".to_string())) {
                    item.title = Some(t.text.clone());
                }
                if let Some(t) = translations.get(&(item.video_id, "subtitle".to_string())) {
                    item.subtitle = Some(t.text.clone());
                }
            }
        }

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
        let video = VideoRepo::get_video_detail(&st.db, video_id).await?;
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
        let progress = VideoRepo::get_progress(&st.db, user_id, video_id).await?;

        // 3. 없으면 기본값 반환 (0%)
        Ok(progress.unwrap_or_else(|| VideoProgressRes {
            video_id,
            progress_rate: 0,
            is_completed: false,
            last_watched_at: None,
            watch_duration_sec: 0,
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

        // 3. 기존 진도 조회 (통계 판단용)
        let existing = VideoRepo::get_progress(&st.db, user_id, video_id).await?;

        // 4. 통계 판단
        let is_new_view = existing.is_none(); // 최초 시청 여부
        let was_completed = existing.as_ref().map(|p| p.is_completed).unwrap_or(false);
        let is_completed = req.progress_rate >= 100;
        let is_new_complete = !was_completed && is_completed; // 이번에 처음 완료

        // 5. Upsert Log (with is_new_view flag)
        let res = VideoRepo::update_progress(
            &st.db,
            user_id,
            video_id,
            req.progress_rate,
            is_completed,
            is_new_view,
            req.watch_duration_sec,
        )
        .await?;

        // 6. 일별 통계 업데이트
        if is_new_view {
            // 최초 시청 시 views 증가
            VideoRepo::increment_daily_views(&st.db, video_id).await?;
        }
        if is_new_complete {
            // 처음 완료 시 completes 증가
            VideoRepo::increment_daily_completes(&st.db, video_id).await?;
        }

        Ok(res)
    }
}