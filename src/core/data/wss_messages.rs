use serde::{Deserialize, Deserializer};
use serde_json::Value;
use crate::core::data::message::DiscordMessage;
use crate::core::json::{parse_json_from_value};

#[derive(Debug)]
pub struct Payload {
    op: u16,
    pub d: ReceiveEvents,
    s: Option<u16>,
    t: Option<String>,
}

impl<'de> Deserialize<'de> for Payload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value: Value = serde::Deserialize::deserialize(deserializer)?;

        let op = value
            .get("op")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| serde::de::Error::missing_field("op"))? as u16;

        let d = match op {
            0 => {
                let t = value.get("t").and_then(|t| t.as_str()).unwrap_or("");
                let d_data = value
                    .get("d")
                    .ok_or_else(|| serde::de::Error::missing_field("d"))?;

                let dispatched_event = get_dispatched_event(t, d_data.clone()).unwrap();
                ReceiveEvents::Dispatch(dispatched_event)
            }
            10 => {
                let d_data = value
                    .get("d")
                    .ok_or_else(|| serde::de::Error::missing_field("d"))?;
                ReceiveEvents::Hello {
                    heartbeat_interval: d_data.get("heartbeat_interval").and_then(|v| v.as_u64()).unwrap() as u16,
                }
            }
            11 => {
                ReceiveEvents::HeartbeatACK
            }
            _ => {
                return Err(serde::de::Error::custom(format!("Unknown op: {}", op)));
            }
        };

        let s = value.get("s").and_then(|v| v.as_u64()).map(|s| s as u16);
        let t = value.get("t").and_then(|v| v.as_str()).map(|t| t.to_string());

        Ok(Payload { op, d, s, t })
    }
}

fn get_dispatched_event(t: &str, d_data: Value) -> Option<DispatchedEvent> {
    match t {
        "MESSAGE_CREATE" => {
            let message = parse_json_from_value::<DiscordMessage>(d_data).expect("TODO: panic message");

            return Some(DispatchedEvent::MessageCreate(
                message
            ));
        }
        "READY" => {
            return Some(DispatchedEvent::Ready);
        }
        _ => {}
    }

    return None;
}


struct Properties {
    os: String,
    browser: String,
    device: String,
}

enum SendEvents {
    Identify {
        token: String,
        properties: Properties,
        intents: u64,
    },
    Resume {
        token: String,
        session_id: String,
        seq: u32,
    },
    Heartbeat(u32),
}

#[derive(Deserialize, Debug)]
#[repr(u8)]
pub enum ReceiveEvents {
    Dispatch(
        DispatchedEvent,
    ) = 0,
    Hello {
        heartbeat_interval: u16
    }= 10,
    HeartbeatACK = 11,
}

#[derive(Deserialize, Debug)]
pub enum DispatchedEvent {
    MessageCreate(DiscordMessage),
    Ready,
    Dummy,
}