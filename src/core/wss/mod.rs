use std::env;
use std::sync::{Arc};
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use crate::core::json;
use crate::core::data::wss_messages::{DispatchedEvent, PayloadData, Properties, ReceiveEvents, SendEvents};
use crate::core::data::wss_messages::Payload;
use std::time::{SystemTime, UNIX_EPOCH};
use futures::stream::SplitSink;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use crate::core::bot::Client;
use crate::core::handler::EventHandler;


pub async fn start_socket(bot: Arc<Client>, event_handler: Box<dyn EventHandler>) {
    let conn = connect_async(
        "wss://gateway.discord.gg",
    ).await;

    if let Err(..) = conn {
        println!("Error al establecer la conexion con el WebSocket.");
        return;
    }

    let (ws_stream, _response) = conn.unwrap();


    let (write, mut read) = ws_stream.split();

    let bot_clone = bot.clone(); // Clone the Bot reference


    // Lee mensajes del servidor.

    tokio::spawn(async move {
        println!("Thread on");
        let mut heartbeat_handle: Option<JoinHandle<()>> = None;
        let shared_write = Arc::new(Mutex::new(write));
        let main_write = shared_write.clone();

        'outer: loop {
            let msg = read.next().await.expect("Error").unwrap();

            match msg {
                Message::Close(..) => {
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

                    let mut payload_data = None;
                    if let PayloadData::Receive(data) = &json.d {
                        payload_data = Some(data);
                    }

                    if let None = payload_data {
                        continue
                    }

                    let data = payload_data.unwrap();


                    match data {
                        ReceiveEvents::Hello { heartbeat_interval } => {

                            hello_event(&mut heartbeat_handle, &shared_write, &main_write, *heartbeat_interval, bot_clone.clone()).await;
                        }
                        ReceiveEvents::Dispatch(event) => {
                            match event {
                                DispatchedEvent::MessageCreate(message) => {
                                    let is_from_bot = message.author.as_ref().unwrap().bot.unwrap_or(false);
                                    if is_from_bot && bot.client_settings.accept_from_bot {
                                        continue;
                                    }
                                    event_handler.message(message, bot.as_ref()).await;
                                }
                                DispatchedEvent::Ready => {
                                    event_handler.ready().await;

                                }
                                DispatchedEvent::PresenceUpdate {user,status,guild_id,}=> {
                                    event_handler.status_update(user, String::from(guild_id), String::from(status), bot.as_ref()).await;
                                }
                                DispatchedEvent::Dummy => {}
                            }
                        }
                        ReceiveEvents::HeartbeatACK => {}
                    }
                }
                _ => {}

            }
        }

    }).await.expect("AAAA");
}

async fn hello_event(heartbeat_handle: &mut Option<JoinHandle<()>>, shared_write: &Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>, main_write: &Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>, heartbeat_interval: u16, bot: Arc<Client>) {
    let write = shared_write.clone();

    *heartbeat_handle = Some(tokio::spawn(async move {
        heartbeat_send_func(heartbeat_interval, write).await;
    }));

    send_identify_message(&main_write, &bot).await;
}

async fn send_identify_message(main_write: &Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>, bot: &Client) {
    let mut writer = main_write.lock().await;

    let payload_data = PayloadData::Send(
        SendEvents::Identify {
            token: bot.token.clone(),
            properties: Properties {
                os: env::consts::OS.to_string(),
                browser: String::from("RustAPI"),
                device: String::from("RustAPI"),
            },
            intents: 33536,
        });

    let payload = Payload::new(2, payload_data, None, None);

    let json_obj: String = serde_json::to_string(&payload).expect("No se ha podido serializar.");

    writer.send(Message::Text(json_obj.to_string())).await.expect("TODO: panic message");

}

async fn heartbeat_send_func(heartbeat_interval: u16, write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>) {

    let mut heartbeat = HeartBeat {
        heartbeat_count: 0,
        next_heartbeat: 0,
        heartbeat_delay: 10000,
    };

    heartbeat.heartbeat_delay = heartbeat_interval - 1000;

    let actual_millis = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    heartbeat.next_heartbeat = actual_millis + heartbeat.heartbeat_delay as u128;

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