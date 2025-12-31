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
        crate::api::admin::video::handler::create_video_handler,
    ),
    components(
        schemas(
            dto::HealthRes,
            dto::VideoListReq,
            dto::VideoInfo,
            dto::VideoListMeta,
            dto::VideoListRes,
            dto::VideoTagDetail,
            dto::VideoDetailRes,
            dto::CaptionItem,
            dto::VideoProgressRes,
            dto::VideoProgressUpdateReq,
            crate::api::admin::video::dto::VideoCreateReq,
            crate::api::admin::video::dto::VideoRes,
        )
    ),
    tags(
        (name = "videos", description = "Video APIs"),
        (name = "Admin - Videos", description = "Admin video management APIs")
    )
)]
pub struct VideoApiDoc;
