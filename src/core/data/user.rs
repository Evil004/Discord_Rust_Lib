use serde::Deserialize;
use crate::core::bot::Client;
use crate::core::http::get_from_discord_api;
use crate::core::json;

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: Option<String>,
    pub avatar: Option<String>,
    pub discriminator: Option<String>,
    pub global_name: Option<String>,
    pub public_flags: Option<i32>,
    pub flags: Option<i32>,
    pub banner: Option<String>,
    pub accent_color: Option<i32>,
    pub avatar_decoration_data: Option<String>,
    pub banner_color: Option<String>,
    pub mfa_enabled: Option<bool>,
    pub locale: Option<String>,
    pub premium_type: Option<i32>,
    pub bot: Option<bool>,

}


impl User {
    pub async fn get_user(id: &str, client: &Client) -> User {
        let channel_data = get_from_discord_api(&format!("/users/{}", id), client).await.expect("Error");

        let channel: User = json::parse_json_from_string(&channel_data).expect("ERROR: Error parsing the guild Info");

        channel
    }

}