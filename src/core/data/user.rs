use std::fmt::format;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use crate::core::bot::Client;
use crate::core::http::{delete_from_discord_api, get_from_discord_api};
use crate::core::json;

#[derive(Deserialize, Debug, Serialize)]
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
        let response = get_from_discord_api(&format!("/users/{}", id), client).await;

        let user_data = response.text().await.unwrap();

        let channel: User = json::parse_json_from_string(&user_data).expect("ERROR: Error parsing the guild Info");

        channel
    }

    pub async fn get_avatar_url(&self) -> String {
        format!("https://cdn.discordapp.com/avatars/{}/{}.webp", self.id, self.avatar.as_ref().unwrap())
    }
}

#[derive(Deserialize, Debug)]
pub struct PresenceUpdate {
    pub user: User,
    pub guild_id: String,
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct GuildMember {
    pub user: Option<User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub roles: Vec<String>,
    pub guild_id: String,
    pub joined_at: String,
    pub premium_since: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub flags: i32,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
    pub communication_disabled_until: Option<String>,
}

