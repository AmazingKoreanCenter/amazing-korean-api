use crate::api::video::dto::{VideoProgressRes, VideoProgressUpdateReq};
use crate::api::video::repo::VideoRepo;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use chrono::Utc;
use sqlx::Row;
use tracing::warn;

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
            user_id,
            last_position_seconds: Some(0),
            total_duration_seconds: None,
            progress: Some(0),
            completed: false,
            // 기록 없음 GET 기본값은 updated_at/null이 자연스럽지만,
            // 기존 코드 흐름 유지: 최초 PUT에서 갱신되므로 임시 now() 사용
            last_watched_at: Some(Utc::now()),
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

        // 10분 규칙 warn!
        if let Some(prev) = self.repo.fetch_video_progress(user_id, video_id).await? {
            if req.last_position_seconds >= prev.last_position_seconds.unwrap_or(0) + 600 {
                warn!(
                    "User {} watched video {} for more than 10 minutes. Previous position: {:?}, New position: {}",
                    user_id, video_id, prev.last_position_seconds, req.last_position_seconds
                );
            }
        }

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

        Ok(VideoProgressRes {
            video_id: row.try_get("video_id")?,
            user_id,
            last_position_seconds: row.try_get("last_position_seconds")?,
            total_duration_seconds: row.try_get("total_duration_seconds")?,
            progress: row.try_get("progress")?,
            completed: row.try_get("completed")?,
            // DB 함수가 updated_at을 돌려준다고 가정
            last_watched_at: row.try_get("updated_at")?,
        })
    }
}
