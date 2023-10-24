use async_trait::async_trait;
use reqwest;
use dotenv::{dotenv, Error, var};
use serde_json::{Result};
use DiscordHM_API::core::bot::Bot;
use DiscordHM_API::core::data::channel::Channel;
use DiscordHM_API::core::data::message::DiscordMessage;
use DiscordHM_API::core::data::user::User;
use DiscordHM_API::core::handler::EventHandler;
use DiscordHM_API::core::json;
use DiscordHM_API::core::requests;

struct Handler;

#[async_trait]
impl EventHandler for Handler{
    async fn message(&self, message: &DiscordMessage) {


        let channel = Channel::get_channel(&message.channel_id).await;
        if message.content =="Ping" {

            channel.send_message("Pong").await;
        }
    }

    async fn ready(&self) {
        println!("Bot listo")
    }

}

#[tokio::main]
async fn main() -> std::result::Result<(), Error> {

    dotenv().ok();

    let token = var("DISCORD_BOT_TOKEN")?;
    let command_prefix = var("COMMAND_PREFIX")?;


    let mut bot = Bot::create(token, command_prefix);

    bot.set_event_handler(Box::new(Handler{}));
    bot.start().await;



    Ok(())
}

fn test(){
    println!("AAAAH")
}




async fn test1() {
    let b = requests::get_from_discord_api("/channels/492375411309674510").await;

    println!("{:?}", b);

    match b {
        Ok(v) => {
            println!("{}", v);
            let a: Result<Channel> = json::parse_json_from_string(&v);
            match a {
                Ok(v) => {
                    println!("{:?}", v)
                }
                Err(_) => {}
            }
        }
        Err(a) => {
            println!("{}", a)
        }
    }


    let b = requests::get_from_discord_api("/applications/@me").await;

    println!("{:?}", b);

    match b {
        Ok(v) => {
            println!("{}", v);
            let a: Result<User> = json::parse_json_from_string(&v);
            match a {
                Ok(v) => {
                    println!("{:?}", v)
                }
                Err(_) => {}
            }
        }
        Err(a) => {
            println!("{}", a)
        }
    }
}