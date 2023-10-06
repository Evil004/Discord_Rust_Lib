use crate::core::data::user::User;

struct Message{
    id: String,
    message_type: i32,
    content: String,
    channel_id: String,
    author:User,
    attachments: Vec<String>,
    embeds: Vec<String>,
    mentions: Vec<String>,
    mention_roles: Vec<String>,
    pinned: bool,
    mention_everyone: bool,
    tts: bool,
    timestamp: String,
    edited_timestamp: Option<String>,
    flags: i32,
    components: Vec<String>,
}