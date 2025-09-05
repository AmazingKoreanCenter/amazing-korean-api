use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "user_gender_enum")]
#[serde(rename_all = "lowercase")]
pub enum UserGender {
    #[sqlx(rename = "none")]
    None,
    #[sqlx(rename = "male")]
    Male,
    #[sqlx(rename = "female")]
    Female,
    #[sqlx(rename = "other")]
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "user_auth_enum")]
#[serde(rename_all = "lowercase")]
pub enum UserAuth {
    #[sqlx(rename = "HYMN")]
    Hymn,
    #[sqlx(rename = "admin")]
    Admin,
    #[sqlx(rename = "manager")]
    Manager,
    #[sqlx(rename = "learner")]
    Learner,
}

impl std::fmt::Display for UserAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UserAuth::Hymn => "HYMN",
            UserAuth::Admin => "admin",
            UserAuth::Manager => "manager",
            UserAuth::Learner => "learner",
        };
        write!(f, "{s}")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "user_state_enum")]
#[serde(rename_all = "lowercase")]
pub enum UserState {
    #[sqlx(rename = "on")]
    On,
    #[sqlx(rename = "off")]
    Off,
}

impl std::fmt::Display for UserState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UserState::On => "on",
            UserState::Off => "off",
        };
        write!(f, "{s}")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "login_device_enum")]
#[serde(rename_all = "lowercase")]
pub enum LoginDeviceEnum {
    #[sqlx(rename = "web")]
    Web,
    #[sqlx(rename = "ios")]
    Ios,
    #[sqlx(rename = "android")]
    Android,
    #[sqlx(rename = "other")]
    Other,
}
