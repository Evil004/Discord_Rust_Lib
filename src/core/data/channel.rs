use serde::Deserialize;
use serde_json::json;
use serde_repr::{Serialize_repr, Deserialize_repr};
use crate::core::data::channel;
use crate::core::json;
use crate::core::requests::{get_from_discord_api, post_to_discord_api};


#[derive(Deserialize, Debug)]
pub struct Channel{
    id: String,
    #[serde(rename(deserialize = "type"))]
    channel_type: ChannelType,
    flags: Option<i32>,
    guild_id: String,
    name: String,
    parent_id: Option<String>,
    rate_limit_per_user: i32,
    topic: Option<String>,
    position: i32,
    nsfw: bool

}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u16)]
pub enum ChannelType{
    GuildText=0,
    DM=1,
    GuildVoice=2,
    GroupDm=3,
    GuildCategory=4
}

impl Channel{
    pub async fn get_channel(id: &str) -> Channel {
        let channel_data = get_from_discord_api(&format!("/channels/{}", id)).await.expect("Error");

        let channel: Channel = json::parse_json_from_string(&channel_data).expect("ERROR: Error Obtaining the channel Info");

        channel
    }
    pub async fn send_message(&self, message: &str){

        let data = format!("{{\"content\":\"{}\"}}", message );

        post_to_discord_api(&format!("/channels/{}/messages", self.id), data).await;
    }
}