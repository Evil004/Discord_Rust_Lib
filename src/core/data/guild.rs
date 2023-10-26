use serde::Deserialize;
use crate::core::bot::Client;
use crate::core::http::{get_from_discord_api};
use crate::core::json;

#[derive(Deserialize)]pub struct Guild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub owner: Option<bool>,
    pub owner_id: String,
    pub permissions: Option<String>,
    pub region: Option<String>,
    pub afk_channel_id: Option<String>,
    pub afk_timeout: i32,
    pub widget_enabled: Option<bool>,
    pub widget_channel_id: Option<String>,
    pub verification_level: i32,
    pub default_message_notifications: i32,
    pub explicit_content_filter: i32,
    pub features: Vec<String>,
    pub mfa_level: i32,
    pub application_id: Option<String>,
    pub system_channel_id: Option<String>,
    pub system_channel_flags: i32,
    pub rules_channel_id: Option<String>,
    pub max_presences: Option<i32>,
    pub max_members: i32,
    pub vanity_url_code: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
    pub premium_tier: i32,
    pub premium_subscription_count: Option<i32>,
    pub preferred_locale: String,
    pub public_updates_channel_id: Option<String>,
    pub max_video_channel_users: i32,
    pub max_stage_video_channel_users: i32,
    pub approximate_member_count: Option<i32>,
    pub approximate_presence_count: Option<i32>,
    pub nsfw_level: i32,
    pub premium_progress_bar_enabled: bool,
    pub safety_alerts_channel_id: Option<String>
}


impl Guild {

    pub async fn get_guild(id: &str, client: &Client) -> Guild {
        let response  = get_from_discord_api(&format!("/guilds/{}", id), client).await;

        let channel_data = response.text().await.unwrap();

        let channel: Guild = json::parse_json_from_string(&channel_data).expect("ERROR: Error parsing the guild Info");

        channel
    }

}