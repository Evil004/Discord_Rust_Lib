use serde::{Deserialize, Deserializer};
use serde_json::Value;
use crate::core::json::parse_json_from_value;

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
            10 => {
                let d_data = value
                    .get("d")
                    .ok_or_else(|| serde::de::Error::missing_field("d"))?;
                ReceiveEvents::Hello{
                    heartbeat_interval: d_data.get("heartbeat_interval").and_then(|v| v.as_u64()).unwrap() as u16,
                }
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

fn deserialize_d_field<'de, D>(deserializer: D) -> Result<ReceiveEvents, D::Error>
    where
        D: Deserializer<'de>,
{
    let d: Value = Deserialize::deserialize(deserializer)?;
    println!("{:?}", d);
    let op: u16 = d["op"].as_u64().unwrap_or(0) as u16;

    match op {
        10 => {
            Ok(parse_json_from_value::<ReceiveEvents>(d).unwrap())
        }
        _ => Err(serde::de::Error::custom(format!("Unknown op: {}", op))),
    }
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
    Hello {
        heartbeat_interval: u16
    }= 10,
}
