use std::time::Instant;
use reqwest::Response;
use crate::core::bot::Client;

pub async fn get_from_discord_api(endpoint: &str, client: &Client) -> reqwest::Result<String> {
    let reqwest_client = reqwest::Client::new();
    let authorization_header = format!("Bot {}", client.token);

    let response = reqwest_client.get(format!("https://discord.com/api{}", endpoint))
        .header("Authorization", authorization_header)
        .send()
        .await
        .unwrap()
        .text()
        .await;

    return response;
}

pub async fn post_to_discord_api(endpoint: &str, message: String, client: &Client) -> Response {
    let start_time = Instant::now();

    let reqwest_client = reqwest::Client::new();
    let authorization_header = format!("Bot {}", client.token);


    let json: serde_json::Value = serde_json::from_str(&message).expect("ERROR: Cant cast the POST Message");

    let request = reqwest_client.request(reqwest::Method::POST, format!("https://discord.com/api{}", endpoint))
        .header("Authorization", authorization_header)
        .json(&json);


    let response = request.send().await.expect("Error en el post");

    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    println!("Post Elapsed time: {}ms", elapsed_time.as_millis());

    return response;
}