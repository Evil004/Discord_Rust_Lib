use std::sync::{Arc};
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::Error;
use crate::core::json;
use crate::core::data::wss_messages::{DispatchedEvent, ReceiveEvents};
use crate::core::data::wss_messages::Payload;
use std::time::{SystemTime, UNIX_EPOCH};
use futures::stream::SplitSink;
use serde_json::json;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use crate::core::bot::Bot;
use crate::core::handler::EventHandler;


pub async fn start_socket(bot: & Bot, event_handler: Box<dyn EventHandler>) -> Result<(), Error> {
    let (ws_stream, _response) = connect_async(
        "wss://gateway.discord.gg",
    ).await?;



    let (write, mut read) = ws_stream.split();

    let mut bot_container = Arc::new(Mutex::new(bot));

    // Lee mensajes del servidor.

    tokio::spawn(async move {
        println!("Thread on");
        let mut heartbeat_handle: Option<JoinHandle<()>> = None;
        let shared_write = Arc::new(Mutex::new(write));

        let main_write = shared_write.clone();
        'outer: loop {
            let msg = read.next().await.expect("Error").unwrap();

            match msg {
                Message::Close(msg) => {
                    println!("Cerrando conexión");

                    if let Some(handle) = heartbeat_handle {
                        handle.abort();
                    }
                    break 'outer;
                }

                Message::Text(text) => {
                    let recived_json: Result<Payload, serde_json::Error> = json::parse_json_from_string(&text);

                    if let Err(a) = recived_json {
                        println!("WARNING: No se ha podido parsear el mensaje. {}", a);
                        println!("{}", text);
                        continue;
                    }

                    let json = recived_json.unwrap();

                    match json.d {
                        ReceiveEvents::HeartbeatACK =>{}

                        ReceiveEvents::Hello { heartbeat_interval } => {
                            hello_event(&mut heartbeat_handle, &shared_write, &main_write, heartbeat_interval).await;
                            continue;
                        }

                        ReceiveEvents::Dispatch(dispatched_event) => {
                            if let DispatchedEvent::MessageCreate(message) = &dispatched_event{
                                event_handler.message(message).await;
                            }

                            if let DispatchedEvent::Ready = &dispatched_event {
                                event_handler.ready().await;
                            }

                        }
                        _ => {
                            println!("Another Message Arrived: {:?}", json)
                        }
                    }
                }
                _ => {
                    println!("Another Message Arrived: {:?}", msg)
                }
            }
        }
    }).await.expect("AAAA");


    Ok(())
}

async fn hello_event(heartbeat_handle: &mut Option<JoinHandle<()>>, shared_write: &Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>, main_write: &Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>, heartbeat_interval: u16) {
    let write = shared_write.clone();

    *heartbeat_handle = Some(tokio::spawn(async move {
        heartbeat_send_func(heartbeat_interval, write).await;
    }));

    send_identify_message(&main_write).await;
}

async fn send_identify_message(main_write: &Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>) {
    let mut writer = main_write.lock().await;

    let json_obj = json!({
                                    "op": 2,
                                    "d": {
                                        "token": "MTE1OTU3OTMxMzc0MjU1NzMwNA.G7FBlu.fmOt-G47aLyC7TKOO7UAN4I2TSMLO8CCapWn_c",
                                        "properties": {
                                            "os": "linux",
                                            "browser": "RustBotAPI",
                                            "device": "RustBotAPI"
                                        },
                                        "intents": 33280
                                    }
                                });
    writer.send(Message::Text(json_obj.to_string())).await.expect("TODO: panic message");

    println!("INFO: Se ha enviado el mensaje de identificación.");
}

async fn heartbeat_send_func(heartbeat_interval: u16, write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>) {
    println!("INFO: Starting HeartBeat Thread.");


    let mut heartbeat = HeartBeat {
        heartbeat_count: 0,
        next_heartbeat: 0,
        heartbeat_delay: 10000,
    };

    heartbeat.heartbeat_delay = heartbeat_interval-1000;

    let actual_millis = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    heartbeat.next_heartbeat = actual_millis + heartbeat.heartbeat_delay as u128;


    println!("INFO: Sending heartbeat every {}ms", heartbeat.heartbeat_delay);


    loop {
        let actual_millis = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

        if actual_millis >= heartbeat.next_heartbeat {
            heartbeat.heartbeat_count += 1;
            heartbeat.next_heartbeat = actual_millis + heartbeat.heartbeat_delay as u128;

            let mut writer = write.lock().await;
            writer.send(Message::Text(format!(r#"{{"op": 1,"d": {}}}"#, heartbeat.heartbeat_count))).await.expect("TODO: panic message");
        };
    }
}

#[derive(Debug)]
struct HeartBeat {
    heartbeat_count: u32,
    next_heartbeat: u128,
    heartbeat_delay: u16,
}