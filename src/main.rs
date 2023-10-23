use std::thread;
use std::time::Duration;
use reqwest;
use dotenv::{dotenv, Error};
use futures::SinkExt;
use serde_json::{Result, Value};
use tokio_tungstenite::tungstenite::Message;
use DiscordHM_API::core::data::channel::Channel;
use DiscordHM_API::core::data::user::User;
use DiscordHM_API::core::json;
use DiscordHM_API::core::requests;
use DiscordHM_API::core::wss::create_wss;

#[tokio::main]
async fn main() -> std::result::Result<(), Error> {
    dotenv().ok();

    thread_ws().await;


    Ok(())
}

async fn thread_ws() {
    create_wss().await.expect("TODO: panic message");

    for i in 0..10 {
        tokio::time::sleep(Duration::new(10, 0)).await
    }
}



async fn test1() {
    let b = requests::get("/channels/492375411309674510").await;

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


    let b = requests::get("/applications/@me").await;

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