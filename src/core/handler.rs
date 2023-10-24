use async_trait::async_trait;
use crate::core::bot::Client;
use crate::core::data::message::DiscordMessage;

#[async_trait]
#[allow(unused)]
pub trait EventHandler: Sync + Send{
    async fn ready(&self){}
    async fn message(&self, message: &DiscordMessage, client: &Client){}
}
