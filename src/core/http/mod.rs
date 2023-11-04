use std::borrow::Cow;
use bytes::Bytes;
use reqwest::{ Response};
use reqwest::multipart::{Form, Part};
use serde_json::{json, Value};
use crate::core::bot::Client;
use crate::core::data::message::DiscordMessageBuilder;

#[derive(Debug, Clone)]

pub struct BinaryFile{
    pub name: String,
    pub file: Bytes
}

pub async fn get_from_discord_api(endpoint: &str, client: &Client) -> Response {
    get_discord_request(endpoint, client, None).await
}

pub async fn get_from_discord_api_with_body(endpoint: &str, client: &Client, message: Value) -> Response {
   get_discord_request(endpoint, client, Some(message)).await
}

async fn get_discord_request(endpoint: &str, client: &Client, message: Option<Value>) -> Response {
    let reqwest_client = reqwest::Client::new();
    let authorization_header = format!("Bot {}", client.token);

    let mut request = reqwest_client.request(reqwest::Method::GET, format!("https://discord.com/api{}", endpoint))
        .header("Authorization", authorization_header);

    if let Some(message) = message {
        request = request.json(&message);
    }

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

pub async fn post_message_to_api(endpoint: &str, message: DiscordMessageBuilder, client: &Client) -> Response {
    let reqwest_client = reqwest::Client::new();
    let authorization_header = format!("Bot {}", client.token);

    let mut form = Form::new();

    let payload = serde_json::to_value(message.clone()).unwrap();


    form = form.text("payload_json", payload.to_string());

    let mut request = reqwest_client.post( format!("https://discord.com/api{}", endpoint))
        .header("Authorization", authorization_header);

    let files = message.get_files();

    if let Some(files) = files{
        let mut i = 0;
        for file in files {
            let file_name = format!("files[{}]",i);
            let f = Cow::Owned(file.file.to_vec());
            form = form.part(file_name,Part::bytes(f).file_name(file.name));
            i+=1;
        }
    }

    request = request.multipart(form);

    let response = request.send().await.expect("Error en el post");

    return response;
}


pub async fn post_to_discord_api(endpoint: &str, payload: Value, client: &Client) -> Response {
    let reqwest_client = reqwest::Client::new();
    let authorization_header = format!("Bot {}", client.token);

    let mut request = reqwest_client.post( format!("https://discord.com/api{}", endpoint))
        .header("Authorization", authorization_header)
        .json(&payload);

    let response = request.send().await.expect("Error en el post");

    return response;
}