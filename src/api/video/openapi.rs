use utoipa::OpenApi;

use super::handler;
use super::dto;

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::health,
        handler::list_videos,
        handler::get_video_detail,
        handler::list_video_captions,
        handler::get_video_progress,
        handler::update_video_progress,
    ),
    components(
        schemas(
            dto::HealthRes,
            dto::VideosQuery,
            dto::VideoListItem,
            dto::VideoDetail,
            dto::CaptionItem,
            dto::VideoProgressRes,
            dto::VideoProgressUpdateReq,
        )
    ),
    tags(
        (name = "videos", description = "Video APIs")
    )
)]
pub struct VideoApiDoc;
