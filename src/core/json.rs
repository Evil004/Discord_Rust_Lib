use std::error::Error;
use serde::Deserialize;

pub fn parse_json<'a, T: Deserialize<'a>>(str_json: &'a str) ->Result<T, serde_json::Error> {
    let result = serde_json::from_str(&str_json);

    result
}