use std::borrow::Cow;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use crate::core::bot::Client;
use crate::core::data::channel::Channel;
use crate::core::data::user::User;
use crate::core::http::{BinaryFile, delete_from_discord_api};

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

#[derive(Debug, Serialize, Clone)]
pub struct DiscordMessageBuilder{
    content: Option<String>,

    #[serde(skip_serializing, skip_deserializing)]
    files: Option<Vec<BinaryFile>>,

    reference: Option<MessageReference>
}

impl DiscordMessageBuilder{
    pub fn new()-> DiscordMessageBuilder{
        DiscordMessageBuilder{
            content:None,
            files: None,
            reference: None,
        }
    }

    pub fn set_files(mut self, files: Vec<BinaryFile>) -> Self{
        self.files = Some(files);
        self
    }
    pub fn get_files(mut self) -> Option<Vec<BinaryFile>> {
        self.files
    }
    pub fn set_content(mut self, content: &str)-> Self{
        self.content = Some(String::from(content));
        self
    }
    pub fn set_reference(mut self, reference: MessageReference)-> Self {
        self.reference = Some(reference);
        self

    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct MessageReference{
    message_id: Option<String>,
    channel_id: Option<String>,
    guild_id: Option<String>,
    fail_if_not_exists: Option<bool>
}