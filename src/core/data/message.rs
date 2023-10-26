use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;
use crate::core::bot::Client;
use crate::core::data::channel::Channel;
use crate::core::data::user::User;
use crate::core::http::{delete_from_discord_api, post_to_discord_api};

#[derive(Deserialize, Debug, Serialize)]
pub struct DiscordMessage {
    pub id: String,
    pub channel_id: String,
    pub author: Option<User>,
    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<User>,
    pub mention_roles: Vec<String>,
    pub mention_channels: Option<Vec<Channel>>,
    pub nonce: Option<String>,
    pub pinned: bool,
    pub webhook_id: Option<String>,
    pub message_type: Option<i32>,
    pub application_id: Option<String>,
    pub referenced_message: Option<Box<DiscordMessage>>,
    pub flags: Option<i32>,
    pub position: Option<i32>,
}

impl DiscordMessage {
    pub async fn delete(&self, client: &Client) {
        delete_from_discord_api(&format!("/channels/{}/messages/{}", self.channel_id, self.id), client).await;
    }

}