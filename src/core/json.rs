use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;

pub fn parse_json_from_string<'a, T: Deserialize<'a>>(str_json: &'a str) ->Result<T, serde_json::Error> {
    let result = serde_json::from_str(&str_json);
    result
}
pub fn parse_json_from_value<'a, T: DeserializeOwned>(json_value: Value) ->Result<T, serde_json::Error> {
    let result = serde_json::from_value::<T>(json_value);
    result
}