use utoipa::{Modify, OpenApi};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert(Default::default());
        // ✨ JWT 보안 스키마는 HttpBuilder를 사용해야 bearer_format 설정 가능
        components.add_security_scheme(
            "bearer_auth",
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
        crate::api::auth::handler::signup,
        crate::api::auth::handler::login,
        crate::api::auth::handler::me,
    ),
    components(
        schemas(
            crate::api::auth::dto::SignUpReq,
            crate::api::auth::dto::LoginReq,
            crate::api::auth::dto::LoginResp,
            crate::api::auth::dto::UserOut,
            crate::error::ErrorBody,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "auth", description = "Authentication")
    )
)]
pub struct ApiDoc;
