use crate::error::{AppError, AppResult};
use crate::state::AppState;

use super::dto::{CaptionItem, VideoDetail, VideoListItem, VideosQuery};
use super::repo::VideoRepo;

#[allow(dead_code)]
pub struct VideoService;

impl VideoService {
    pub async fn list_videos(st: &AppState, q: VideosQuery) -> AppResult<Vec<VideoListItem>> {
        let mut q = q;
        q.limit = q.limit.clamp(1, 100);
        q.offset = q.offset.max(0);

        // TODO: Handle popular and complete_rate sorting in subsequent stages
        // For now, if sort is not 'created_at', default to 'created_at'
        if let Some(sort_by) = &q.sort {
            if !matches!(sort_by.as_str(), "created_at") {
                q.sort = Some("created_at".to_string());
            }
        }

        let repo = VideoRepo::new(st.db.clone());
        Ok(repo.fetch_videos(&q).await?)
    }

    pub async fn get_video_detail(st: &AppState, id: i64) -> AppResult<VideoDetail> {
        let repo = VideoRepo::new(st.db.clone());
        let video = repo.fetch_video_detail(id).await?;

        match video {
            Some(detail) => {
                // Gating logic: only 'open' and 'public' videos are accessible for guests
                if detail.state == "open" && detail.access == "public" {
                    Ok(detail)
                } else {
                    // TODO: Extend this with authentication/authorization for paid/private access
                    Err(AppError::Forbidden)
                }
            }
            None => Err(AppError::NotFound),
        }
    }

    pub async fn list_video_captions(st: &AppState, id: i64) -> AppResult<Vec<CaptionItem>> {
        let repo = VideoRepo::new(st.db.clone());

        // Gating logic: Check video existence and access
        let video_status = repo.fetch_video_status(id).await?;

        match video_status {
            Some((state, access)) => {
                if state == "open" && access == "public" {
                    Ok(repo.fetch_video_captions(id).await?)
                } else {
                    // TODO: Extend this with authentication/authorization for paid/private access
                    Err(AppError::Forbidden)
                }
            }
            None => Err(AppError::NotFound),
        }
    }
}
