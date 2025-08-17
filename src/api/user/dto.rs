use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;

/// DB의 CHECK 제약과 1:1로 맞춘 enum
#[derive(Serialize, Deserialize, ToSchema, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    None,
    Male,
    Female,
    Other,
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Gender::None => "none",
            Gender::Male => "male",
            Gender::Female => "female",
            Gender::Other => "other",
        };
        write!(f, "{s}")
    }
}

/// 사용자 생성(회원가입) 요청 바디
#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateUserReq {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 6, max = 72))]
    pub password: String,

    #[validate(length(min = 1, max = 50))]
    pub name: String,

    pub terms_service: bool,
    pub terms_personal: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 100))]
    pub nickname: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 50))]
    pub language: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 50))]
    pub country: Option<String>,

    /// ISO 8601 문자열 권장 예: "2000-05-20T00:00:00Z"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,

    /// 미지정이면 none 처리
    #[serde(default)]
    pub gender: Option<Gender>,
}
