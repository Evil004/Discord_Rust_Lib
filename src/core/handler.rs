use async_trait::async_trait;
use crate::core::bot::Client;
use crate::core::data::message::DiscordMessage;
use crate::core::data::user::{GuildMember, PresenceUpdate, User};

#[async_trait]
#[allow(unused)]
pub trait EventHandler: Sync + Send{
    async fn on_ready(&self){}
    async fn on_message(&self, message: &DiscordMessage, client: &Client){}

    async fn status_update(&self, presence: &PresenceUpdate, client: &Client){}
    async fn on_guild_member_add(&self, guild_member: &GuildMember, client: &Client){}

    async fn on_guild_member_remove(&self, user: &User, guild_id: &str, client: &Client){}

}
