use reqwest;
use dotenv::dotenv;
use serde_json::{Result, Value};
use DiscordHM_API::core::data::channel::Channel;
use DiscordHM_API::core::data::user::User;
use DiscordHM_API::core::json;
use DiscordHM_API::core::requests;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let b = requests::get("/channels/492375411309674510").await;

    println!("{:?}", b);

    match b {
        Ok(v) => {
            println!("{}", v);
            let a: Result<Channel> = json::parse_json(&v);
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
            let a: Result<User> = json::parse_json(&v);
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



    //Ok(())
}
