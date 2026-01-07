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

        // lessons (user)
        crate::api::lesson::handler::list_lessons,
        crate::api::lesson::handler::get_lesson_detail,
        crate::api::lesson::handler::get_lesson_items,
        crate::api::lesson::handler::get_lesson_progress,
        crate::api::lesson::handler::update_lesson_progress,

        // study (user)
        crate::api::study::handler::list_studies,
        crate::api::study::handler::get_study_task,
        crate::api::study::handler::submit_answer,
        crate::api::study::handler::get_task_status,
        crate::api::study::handler::get_task_explanation,

        // admin - users
        crate::api::admin::user::handler::admin_list_users,
        crate::api::admin::user::handler::admin_create_user,
        crate::api::admin::user::handler::admin_create_users_bulk,
        crate::api::admin::user::handler::admin_get_user,
        crate::api::admin::user::handler::admin_update_user,
        crate::api::admin::user::handler::admin_update_users_bulk,

        // admin - videos
        crate::api::admin::video::handler::admin_list_videos,
        crate::api::admin::video::handler::create_video_handler,   
        crate::api::admin::video::handler::admin_bulk_create_videos,
        crate::api::admin::video::handler::admin_update_video,     
        crate::api::admin::video::handler::admin_bulk_update_videos,
        crate::api::admin::video::handler::admin_update_video_tags,
        crate::api::admin::video::handler::admin_bulk_update_video_tags,

        // admin - studies
        crate::api::admin::study::handler::admin_list_studies,
        crate::api::admin::study::handler::admin_create_study,
        crate::api::admin::study::handler::admin_bulk_create_studies,
        crate::api::admin::study::handler::admin_update_study,
        crate::api::admin::study::handler::admin_bulk_update_studies,
        crate::api::admin::study::handler::admin_list_study_tasks,
        crate::api::admin::study::handler::admin_update_study_task,
        crate::api::admin::study::handler::admin_create_study_task,
        crate::api::admin::study::handler::admin_bulk_create_study_tasks,


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

            // lessons dto
            crate::api::lesson::dto::LessonListReq,
            crate::api::lesson::dto::LessonRes,
            crate::api::lesson::dto::LessonListMeta,
            crate::api::lesson::dto::LessonListRes,
            crate::api::lesson::dto::LessonDetailReq,
            crate::api::lesson::dto::LessonItemRes,
            crate::api::lesson::dto::LessonDetailRes,
            crate::api::lesson::dto::LessonItemsReq,
            crate::api::lesson::dto::LessonItemDetailRes,
            crate::api::lesson::dto::LessonItemsRes,
            crate::api::lesson::dto::LessonProgressRes,
            crate::api::lesson::dto::LessonProgressUpdateReq,

            // studies dto
            crate::api::study::dto::StudyListRes,
            crate::api::study::dto::StudyListMeta,
            crate::api::study::dto::StudyListItem,
            crate::api::study::dto::StudyTaskDetailRes,
            crate::api::study::dto::TaskPayload,
            crate::api::study::dto::SubmitAnswerReq,
            crate::api::study::dto::SubmitAnswerRes,
            crate::api::study::dto::TaskStatusRes,
            crate::api::study::dto::TaskExplainRes,

            // admin - users dto
            crate::api::admin::user::dto::AdminUserRes,
            crate::api::admin::user::dto::AdminUserListReq,
            crate::api::admin::user::dto::AdminUserSummary,
            crate::api::admin::user::dto::AdminUserListMeta,
            crate::api::admin::user::dto::AdminUserListRes,
            crate::api::admin::user::dto::AdminCreateUserReq,
            crate::api::admin::user::dto::AdminBulkCreateReq,
            crate::api::admin::user::dto::BulkSummary,
            crate::api::admin::user::dto::BulkItemError,
            crate::api::admin::user::dto::BulkItemResult,
            crate::api::admin::user::dto::AdminBulkCreateRes,
            crate::api::admin::user::dto::AdminUpdateUserReq,
            crate::api::admin::user::dto::AdminBulkUpdateItemReq,
            crate::api::admin::user::dto::AdminBulkUpdateReq,
            crate::api::admin::user::dto::BulkUpdateItemResult,
            crate::api::admin::user::dto::AdminBulkUpdateRes,

            // admin - videos dto
            crate::api::admin::video::dto::AdminVideoListReq,
            crate::api::admin::video::dto::AdminVideoRes,
            crate::api::admin::video::dto::AdminVideoListRes,
            crate::api::admin::video::dto::Pagination,
            crate::api::admin::video::dto::VideoBulkCreateReq,
            crate::api::admin::video::dto::VideoBulkCreateRes,
            crate::api::admin::video::dto::VideoBulkItemError,
            crate::api::admin::video::dto::VideoBulkItemResult,
            crate::api::admin::video::dto::VideoBulkSummary,
            crate::api::admin::video::dto::VideoBulkUpdateItem,
            crate::api::admin::video::dto::VideoBulkUpdateReq,
            crate::api::admin::video::dto::VideoBulkUpdateItemResult,
            crate::api::admin::video::dto::VideoBulkUpdateRes,
            crate::api::admin::video::dto::VideoCreateReq,
            crate::api::admin::video::dto::VideoRes,
            crate::api::admin::video::dto::VideoTagBulkUpdateItem,
            crate::api::admin::video::dto::VideoTagBulkUpdateReq,
            crate::api::admin::video::dto::VideoTagUpdateReq,
            crate::api::admin::video::dto::VideoUpdateReq,

            // admin - studies dto
            crate::api::admin::study::dto::StudyListReq,
            crate::api::admin::study::dto::AdminStudyRes,
            crate::api::admin::study::dto::AdminStudyListRes,
            crate::api::admin::study::dto::StudyCreateReq,
            crate::api::admin::study::dto::StudyBulkCreateReq,
            crate::api::admin::study::dto::StudyBulkCreateRes,
            crate::api::admin::study::dto::StudyBulkResult,
            crate::api::admin::study::dto::StudyUpdateReq,
            crate::api::admin::study::dto::StudyBulkUpdateReq,
            crate::api::admin::study::dto::StudyBulkUpdateItem,
            crate::api::admin::study::dto::StudyBulkUpdateRes,
            crate::api::admin::study::dto::StudyBulkUpdateResult,
            crate::api::admin::study::dto::StudyTaskListReq,
            crate::api::admin::study::dto::AdminStudyTaskRes,
            crate::api::admin::study::dto::AdminStudyTaskListRes,
            crate::api::admin::study::dto::StudyTaskUpdateReq,
            crate::api::admin::study::dto::AdminStudyTaskDetailRes,
            crate::api::admin::study::dto::StudyTaskCreateReq,
            crate::api::admin::study::dto::StudyTaskBulkCreateReq,
            crate::api::admin::study::dto::StudyTaskBulkCreateRes,
            crate::api::admin::study::dto::StudyTaskBulkResult,


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
