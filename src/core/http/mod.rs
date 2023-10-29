use std::time::Instant;
use reqwest::{Error, Response};
use serde_json::{json, Map, Value};
use crate::core::bot::Client;
use crate::core::json::parse_json_from_string;

pub async fn get_from_discord_api(endpoint: &str, client: &Client) -> Response {
    let reqwest_client = reqwest::Client::new();
    let authorization_header = format!("Bot {}", client.token);



    let request = reqwest_client.request(reqwest::Method::GET, format!("https://discord.com/api{}", endpoint))
        .header("Authorization", authorization_header);


    let response = request.send().await.expect("Error en el post");

    return response;
}


pub async fn get_from_discord_api_with_body(endpoint: &str, client: &Client, message: Value) -> Response {
   get_request(endpoint, client, message).await
}

async fn get_request(endpoint: &str, client: &Client, message: Value) -> Response {
    let reqwest_client = reqwest::Client::new();
    let authorization_header = format!("Bot {}", client.token);



    let request = reqwest_client.request(reqwest::Method::POST, format!("https://discord.com/api{}", endpoint))
        .header("Authorization", authorization_header)
        .json(&message);


    let response = request.send().await.expect("Error en el post");

    return response;
}

pub async fn delete_from_discord_api(endpoint: &str, client: &Client) -> Response {
    let reqwest_client = reqwest::Client::new();
    let authorization_header = format!("Bot {}", client.token);



    let request = reqwest_client.request(reqwest::Method::GET, format!("https://discord.com/api{}", endpoint))
        .header("Authorization", authorization_header);


    let response = request.send().await.expect("Error en el post");

    return response;
}

pub async fn post_to_discord_api(endpoint: &str, message: Value, client: &Client) -> Response {
    let reqwest_client = reqwest::Client::new();
    let authorization_header = format!("Bot {}", client.token);

    let request = reqwest_client.post( format!("https://discord.com/api{}", endpoint))
        .header("Authorization", authorization_header)
        .json(&message);

    let response = request.send().await.expect("Error en el post");

    return response;
}