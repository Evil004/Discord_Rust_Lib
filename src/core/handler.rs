use async_trait::async_trait;
use crate::core::bot::Client;
use crate::core::data::message::DiscordMessage;
use crate::core::data::user::{PresenceUpdate, User};

#[async_trait]
#[allow(unused)]
pub trait EventHandler: Sync + Send{
    async fn ready(&self){}
    async fn message(&self, message: &DiscordMessage, client: &Client){}

    async fn status_update(&self, presence: &PresenceUpdate, client: &Client){}
}
