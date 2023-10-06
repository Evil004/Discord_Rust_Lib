const DISCORD_BOT_TOKEN: &str = "MTE1OTU3OTMxMzc0MjU1NzMwNA.G7FBlu.fmOt-G47aLyC7TKOO7UAN4I2TSMLO8CCapWn_c";


pub async fn get(endpoint: &str) -> reqwest::Result<String> {


    let client = reqwest::Client::new();

    let authorization_header = format!("Bot {}", DISCORD_BOT_TOKEN);

    let response = client.get(format!("https://discord.com/api{}",endpoint))
        .header("Authorization", authorization_header)
        .send()
        .await
        .unwrap()
        .text()
        .await;

    return response
}