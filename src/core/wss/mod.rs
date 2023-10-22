use std::sync::{Arc};
use std::thread;
use futures::{SinkExt, StreamExt, TryFutureExt};
use tokio_tungstenite::{connect_async};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::Error;
use crate::core::json;
use crate::core::data::wss_messages::ReceiveEvents;
use crate::core::data::wss_messages::Payload;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;


pub async fn create_wss() -> Result<(), Error> {
    let (ws_stream, _response) = connect_async(
        "wss://gateway.discord.gg",
    ).await?;


    println!("Conexión establecida.");

    let (mut write, mut read) = ws_stream.split();

    // Lee mensajes del servidor.


    let handle = tokio::spawn(async move {
        println!("Thread on");
        let mut heartbeat_handle;
        let shared_write = Arc::new(Mutex::new(write));

        let write = shared_write.clone();
        'outer: loop {
            let msg = read.next().await.expect("Error").unwrap();

            match msg {
                Message::Close(msg) => {
                    println!("Cerrando conexión");

                    break 'outer;
                }

                Message::Text(text) => {
                    println!("{}", text);

                    let recived_json: Result<Payload, serde_json::Error> = json::parse_json_from_string(&text);


                    if let Ok(v) = recived_json {
                        match v.d {
                            ReceiveEvents::Hello { heartbeat_interval } => {
                                let write = shared_write.clone();


                                heartbeat_handle = tokio::spawn(async move {

                                    println!("INFO: Starting HeatBeat Thread.");

                                    let mut writer = write.lock().await;

                                    let mut heartbeat = HeartBeat {
                                        heartbeat_count: 0,
                                        next_heartbeat: 0,
                                        heartbeat_delay: 10000,
                                    };

                                    heartbeat.heartbeat_delay = heartbeat_interval;

                                    println!("INFO: Sending heartbeat every {}ms", heartbeat.heartbeat_delay);


                                    loop {
                                        let actual_millis = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

                                        if actual_millis >= heartbeat.next_heartbeat {
                                            heartbeat.heartbeat_count += 1;
                                            heartbeat.next_heartbeat = actual_millis + heartbeat.heartbeat_delay as u128;
                                            writer.send(Message::Text(format!(r#"{{"op": 1,"d": {}}}"#, heartbeat.heartbeat_count))).await.expect("TODO: panic message");

                                        };
                                    }
                                });

                                continue

                            }
                        }
                    }
                }
                _ => {
                    println!("Another Message Arrived: {:?}", msg)
                }
            }
        }


    });


    Ok(())
}

#[derive(Debug)]
struct HeartBeat {
    heartbeat_count: u32,
    next_heartbeat: u128,
    heartbeat_delay: u16,
}