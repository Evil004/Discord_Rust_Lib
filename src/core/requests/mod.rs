const DISCORD_BOT_TOKEN: &str = "MTE1OTU3OTMxMzc0MjU1NzMwNA.G7FBlu.fmOt-G47aLyC7TKOO7UAN4I2TSMLO8CCapWn_c";


pub async fn get_from_discord_api(endpoint: &str) -> reqwest::Result<String> {
    let client = reqwest::Client::new();
    let authorization_header = format!("Bot {}", DISCORD_BOT_TOKEN);

    let response = client.get(format!("https://discord.com/api{}", endpoint))
        .header("Authorization", authorization_header)
        .send()
        .await
        .unwrap()
        .text()
        .await;

    return response;
}

pub async fn post_to_discord_api(endpoint: &str, message: String) {

    let client = reqwest::Client::new();
    let authorization_header = format!("Bot {}", DISCORD_BOT_TOKEN);


    let json: serde_json::Value = serde_json::from_str(&message).expect("ERROR: Cant cast the POST Message");

    let request = client.request(reqwest::Method::POST, format!("https://discord.com/api{}", endpoint))
        .header("Authorization", authorization_header)
        .json(&json);


    let response =request.send().await.expect("Error en el post");

}