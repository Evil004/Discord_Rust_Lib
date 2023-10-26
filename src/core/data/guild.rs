use serde::Deserialize;
use crate::core::bot::Client;
use crate::core::http::{delete_from_discord_api, get_from_discord_api};
use crate::core::json;

#[derive(Deserialize)]
struct Guild {
    id: String,
    name: String,
    icon: Option<String>,
    icon_hash: Option<String>,
    splash: Option<String>,
    discovery_splash: Option<String>,
    owner: Option<bool>,
    owner_id: String,
    permissions: Option<String>,
    region: Option<String>,
    afk_channel_id: Option<String>,
    afk_timeout: i32,
    widget_enabled: Option<bool>,
    widget_channel_id: Option<String>,
    verification_level: i32,
    default_message_notifications: i32,
    explicit_content_filter: i32,
    features: Vec<String>,
    mfa_level: i32,
    application_id: Option<String>,
    system_channel_id: Option<String>,
    system_channel_flags: i32,
    rules_channel_id: Option<String>,
    max_presences: Option<i32>,
    max_members: i32,
    vanity_url_code: Option<String>,
    description: Option<String>,
    banner: Option<String>,
    premium_tier: i32,
    premium_subscription_count: Option<i32>,
    preferred_locale: String,
    public_updates_channel_id: Option<String>,
    max_video_channel_users: i32,
    max_stage_video_channel_users: i32,
    approximate_member_count: Option<i32>,
    approximate_presence_count: Option<i32>,
    nsfw_level: i32,
    premium_progress_bar_enabled: bool,
    safety_alerts_channel_id: Option<String>,
}

impl Guild {
    pub async fn get_channel(id: &str, client: &Client) -> Guild {
        let response  = get_from_discord_api(&format!("/guilds/{}", id), client).await;

        let channel_data = response.text().await.unwrap();

        let channel: Guild = json::parse_json_from_string(&channel_data).expect("ERROR: Error parsing the guild Info");

        channel
    }

}