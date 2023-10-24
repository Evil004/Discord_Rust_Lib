use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub public_flags: Option<i32>,
    pub flags: Option<i32>,
    pub banner: Option<String>,
    pub accent_color: Option<i32>,
    pub avatar_decoration_data: Option<String>,
    pub banner_color: Option<String>,
    pub mfa_enabled: Option<bool>,
    pub locale: Option<String>,
    pub premium_type: i32,
    pub bot: Option<bool>,

}