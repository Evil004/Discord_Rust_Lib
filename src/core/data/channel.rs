use serde::Deserialize;
use serde_repr::{Serialize_repr, Deserialize_repr};


#[derive(Deserialize, Debug)]
pub struct Channel{
    id: String,
    #[serde(rename(deserialize = "type"))]
    channel_type: ChannelType,
    flags: i32,
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