use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert(Default::default());
        components.add_security_scheme(
            "bearerAuth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
        components.add_security_scheme(
            "refreshCookie",
            SecurityScheme::ApiKey(utoipa::openapi::security::ApiKey::Cookie(
                utoipa::openapi::security::ApiKeyValue::new("ak_refresh".to_string()),
            )),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    info(title = "Amazing Korean API", version = "1.0.0"),
    paths(
        // health
        crate::api::health::handler::health,
        crate::api::health::handler::ready,

        // auth
        crate::api::auth::handler::login,
        crate::api::auth::handler::logout,
        crate::api::auth::handler::logout_all,
        crate::api::auth::handler::refresh,
        crate::api::auth::handler::find_id,
        crate::api::auth::handler::reset_password,

        // user (me/settings)
        crate::api::user::handler::signup,
        crate::api::user::handler::get_me,
        crate::api::user::handler::update_me,
        crate::api::user::handler::get_settings,
        crate::api::user::handler::update_settings,

        // videos (user)
        crate::api::video::handler::list_videos,
        crate::api::video::handler::get_video_detail,
        crate::api::video::handler::get_video_progress,
        crate::api::video::handler::update_video_progress,

        // study (user)
        crate::api::study::handler::list_studies,
        crate::api::study::handler::get_study_task,
        crate::api::study::handler::submit_answer,
        crate::api::study::handler::get_task_status,

        // admin - users
        crate::api::admin::user::handler::admin_list_users,
        crate::api::admin::user::handler::admin_get_user,
        crate::api::admin::user::handler::admin_update_user,

        // admin - videos
        crate::api::admin::video::handler::create_video_handler,   
        crate::api::admin::video::handler::admin_update_video,     
        crate::api::admin::video::handler::admin_delete_video,     

        // admin - video captions
        crate::api::admin::video::caption::handler::admin_create_caption, 
        crate::api::admin::video::caption::handler::admin_update_caption, 
        crate::api::admin::video::caption::handler::admin_delete_caption,  

        // admin - video tags
        crate::api::admin::video::tag::handler::admin_add_tags,
        crate::api::admin::video::tag::handler::admin_remove_tags,

        // admin - video stats
        crate::api::admin::video::stats::handler::admin_get_video_daily_stats,
    ),
    components(
        schemas(
            // base types
            crate::types::UserGender,
            crate::types::UserAuth,
            crate::error::ErrorBody,
            crate::types::LoginDevice,

            // health dto 
            crate::api::health::dto::HealthRes,
            crate::api::health::dto::ReadyRes,

            // auth dto
            crate::api::auth::dto::LoginReq,
            crate::api::auth::dto::LoginRes,
            crate::api::auth::dto::RefreshRes,
            crate::api::auth::dto::FindIdReq,
            crate::api::auth::dto::FindIdRes,
            crate::api::auth::dto::ResetPwReq,
            crate::api::auth::dto::ResetPwRes,

            // user dto
            crate::api::user::dto::SignupReq,
            crate::api::user::dto::SignupRes,
            crate::api::user::dto::ProfileRes,
            crate::api::user::dto::ProfileUpdateReq,
            crate::api::user::dto::SettingsRes,
            crate::api::user::dto::SettingsUpdateReq,
            crate::api::user::dto::StudyLangItem,

            // videos dto
            crate::api::video::dto::VideosQuery,
            crate::api::video::dto::VideoListItem,
            crate::api::video::dto::VideoTagDetail,
            crate::api::video::dto::VideoDetailRes,
            crate::api::video::dto::VideoProgressRes,
            crate::api::video::dto::VideoProgressUpdateReq,

            // studies dto
            crate::api::study::dto::StudyListRes,
            crate::api::study::dto::StudyListMeta,
            crate::api::study::dto::StudyListItem,
            crate::api::study::dto::StudyTaskDetailRes,
            crate::api::study::dto::TaskPayload,
            crate::api::study::dto::SubmitAnswerReq,
            crate::api::study::dto::SubmitAnswerRes,
            crate::api::study::dto::TaskStatusRes,

            // admin - users dto
            crate::api::admin::user::dto::AdminUserRes,
            crate::api::admin::user::dto::AdminListUsersRes,
            crate::api::admin::user::dto::AdminUpdateUserReq,

            // admin - videos dto
            crate::api::admin::video::dto::VideoCreateReq,
            crate::api::admin::video::dto::VideoRes,
            crate::api::admin::video::dto::VideoUpdateReq,

            // admin - video captions dto
            crate::api::admin::video::caption::dto::CaptionKind,
            crate::api::admin::video::caption::dto::CaptionCreateReq,
            crate::api::admin::video::caption::dto::CaptionUpdateReq,
            crate::api::admin::video::caption::dto::CaptionRes,

            // admin - video tags dto
            crate::api::admin::video::tag::dto::TagsModifyReq,
            crate::api::admin::video::tag::dto::TagItem,
            crate::api::admin::video::tag::dto::VideoTagsRes,

            // admin - video stats dto
            crate::api::admin::video::stats::dto::DailyStatsQuery,
            crate::api::admin::video::stats::dto::DailyStatItem,
            crate::api::admin::video::stats::dto::DailyStatsRes,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "health", description = "Health and Readiness checks"),
        (name = "auth", description = "Authentication management"),
        (name = "user", description = "User management"),
        (name = "videos", description = "Video APIs"),
        (name = "study", description = "Study APIs"),
        (name = "lesson", description = "Lesson APIs"),
        (name = "admin", description = "Admin user & content management")
    )
)]
pub struct ApiDoc;
