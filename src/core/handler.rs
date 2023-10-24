use async_trait::async_trait;
use crate::core::data::message::DiscordMessage;

#[async_trait]
pub trait EventHandler: Sync + Send{
    async fn ready(&self){}
    async fn message(&self, message: &DiscordMessage){}
}
