use serde::{Deserialize};
use crate::core::data::channel::Channel;
use crate::core::data::user::User;

#[derive(Deserialize, Debug)]

pub struct DiscordMessage {
    id: String,
    pub channel_id: String,
    pub author: Option<User>,
    pub content: String,
    timestamp: String,
    edited_timestamp: Option<String>,
    tts: bool,
    mention_everyone: bool,
    mentions: Vec<User>,
    mention_roles: Vec<String>,
    mention_channels: Option<Vec<Channel>>,
    nonce: Option<String>,
    pinned: bool,
    webhook_id: Option<String>,
    message_type: Option<i32>,
    application_id: Option<String>,
    referenced_message: Option<Box<DiscordMessage>>,
    flags: Option<i32>,
    position: Option<i32>,
}