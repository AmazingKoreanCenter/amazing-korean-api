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
        crate::api::user::handler::update_settings,
        crate::api::admin::user::handler::admin_list_users,
        crate::api::admin::user::handler::admin_get_user,
        crate::api::admin::user::handler::admin_update_user,
    ),
    components(
        schemas(
            crate::api::user::dto::SignupReq,
            crate::api::user::dto::SignupRes,
            crate::api::user::dto::ProfileRes,
            crate::api::user::dto::UpdateReq,
            crate::api::user::dto::Gender,
            crate::api::user::dto::SettingsRes,
            crate::api::user::dto::SettingsUpdateReq,
            crate::api::user::dto::StudyLangItem,
            crate::api::admin::user::dto::AdminUserRes,
            crate::api::admin::user::dto::AdminListUsersRes,
            crate::api::admin::user::dto::AdminUpdateUserReq,
            crate::api::admin::user::dto::UserAuth,
            crate::api::admin::user::dto::UserState,
            crate::error::ErrorBody,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "user", description = "User management"),
        (name = "admin", description = "Admin user management")
    )
)]
pub struct ApiDoc;
