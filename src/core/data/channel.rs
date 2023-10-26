use std::fmt::Error;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use serde_repr::{Serialize_repr, Deserialize_repr};
use tokio_tungstenite::tungstenite::http::response;
use crate::core::bot::Client;
use crate::core::data::message::DiscordMessage;
use crate::core::json;
use crate::core::http::{delete_from_discord_api, get_from_discord_api, get_from_discord_api_with_body, post_to_discord_api};
use crate::core::json::parse_json_from_string;


#[derive(Deserialize, Debug, Serialize)]
pub struct Channel {
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
    nsfw: bool,
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u16)]
pub enum ChannelType {
    GuildText = 0,
    DM = 1,
    GuildVoice = 2,
    GroupDm = 3,
    GuildCategory = 4,
}

impl Channel {
    pub async fn get_channel(id: &str, client: &Client) -> Channel {
        let response = get_from_discord_api(&format!("/channels/{}", id), client).await;

        let channel_data = response.text().await.unwrap();

        let channel: Channel = json::parse_json_from_string(&channel_data).expect("ERROR: Error Obtaining the channel Info");

        channel
    }
    pub async fn send_message(&self, message: &str, client: &Client) {
        let data = format!("{{\"content\":\"{}\"}}", message);

        let mut content = serde_json::Map::new();
        content.insert(String::from("content"), Value::String(String::from(message)));

        let body = Value::Object(content);

        post_to_discord_api(&format!("/channels/{}/messages", self.id), body, client).await;
    }

    /*pub async fn get_message(&self, message_id: &str, client: &Client) -> String {
        let discord_message = get_from_discord_api(&format!("/channels/{}/messages/{}", self.id, message_id), client).await;
    }*/

    async fn get_messages(&self, num_of_messages: u8, client: &Client) -> Result<Vec<DiscordMessage>, Error> {

        let response = get_from_discord_api(&format!("/channels/{}/messages?limit={}", self.id, num_of_messages),client).await;

        if !response.status().is_success(){
            return Result::Err(Error);
        }

        let messages: Vec<DiscordMessage> = parse_json_from_string(&response.text().await.unwrap()).expect("AA");

        Ok(messages)
    }

    pub async fn bulk_delete(&self, num_of_messages: u8, client: &Client) -> Result<(), Error> {

        let result = self.get_messages(num_of_messages, client).await;

        if let Err(_) = result {
            return Err(Error)
        }

        let messages = result.unwrap();

        let messages_id: Vec<_> = messages.iter().map(|message| &message.id).collect();

        let messages_id_json  = serde_json::to_value(messages_id).unwrap();

        let mut body = Map::new();

        body.insert(String::from("messages"), messages_id_json);

        let json = Value::Object(body);


        let response = post_to_discord_api(&format!("/channels/{}/messages/bulk-delete",self.id ),json,client).await;

        if !response.status().is_success(){
            return Result::Err(Error);
        }

        return Ok(());
    }
}