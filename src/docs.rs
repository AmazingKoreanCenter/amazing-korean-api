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
        crate::api::user::handler::signup,
        crate::api::user::handler::get_me,
        crate::api::user::handler::update_me,
        crate::api::user::handler::get_settings,
        crate::api::user::handler::update_users_setting,
        crate::api::admin::user::handler::admin_list_users,
        crate::api::admin::user::handler::admin_get_user,
        crate::api::admin::user::handler::admin_update_user,
        crate::api::auth::handler::login,
        crate::api::auth::handler::refresh,
        crate::api::auth::handler::logout,
        crate::api::auth::handler::logout_all,
        crate::api::video::handler::health,
        crate::api::video::handler::list_videos,
        crate::api::video::handler::get_video_detail,
        crate::api::video::handler::list_video_captions,
        crate::api::video::handler::get_video_progress,
        crate::api::video::handler::update_video_progress,
        crate::api::health::handler::health,
        crate::api::health::handler::ready,
    ),
    components(
        schemas(
            crate::api::user::dto::SignupReq,
            crate::api::user::dto::SignupRes,
            crate::api::user::dto::ProfileRes,
            crate::api::user::dto::UpdateReq,
            crate::types::UserGender,
            crate::api::user::dto::SettingsRes,
            crate::api::user::dto::SettingsUpdateReq,
            crate::api::user::dto::StudyLangItem,
            crate::api::admin::user::dto::AdminUserRes,
            crate::api::admin::user::dto::AdminListUsersRes,
            crate::api::admin::user::dto::AdminUpdateUserReq,
            crate::types::UserAuth,
            crate::types::UserState,
            crate::api::auth::dto::LoginReq,
            crate::api::auth::dto::LoginRes,
            crate::api::auth::dto::RefreshRes,
            crate::error::ErrorBody,
            crate::api::video::dto::HealthRes,
            crate::api::video::dto::VideosQuery,
            crate::api::video::dto::VideoListItem,
            crate::api::video::dto::VideoDetail,
            crate::api::video::dto::CaptionItem,
            crate::api::video::dto::VideoProgressRes,
            crate::api::video::dto::VideoProgressUpdateReq,
            crate::types::LoginDeviceEnum,
            crate::api::health::handler::HealthRes,
            crate::api::health::handler::ReadyRes,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "user", description = "User management"),
        (name = "auth", description = "Authentication management"),
        (name = "admin", description = "Admin user management"),
        (name = "videos", description = "Video APIs"),
        (name = "health", description = "Health and Readiness checks")
    )
)]
pub struct ApiDoc;
