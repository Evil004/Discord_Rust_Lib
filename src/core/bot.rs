use std::sync::Arc;
use crate::core::handler::EventHandler;
use crate::core::wss::start_socket;

pub struct Client {
    pub token: String,
    event_handler: Option<Box<dyn EventHandler>>,
    pub client_settings: ClientSettings
}

impl Client {

    pub fn create(token: String) -> Client {

        let client_settings = ClientSettings{
            accept_from_bot: true,
            must_start_with_prefix: true
        };

        let client = Client {
            token,
            event_handler: None,
            client_settings
        };


        return client;
    }

    pub async fn start(mut self)  {
        let event_handler = self.event_handler.take(); // Takes ownership temporarily

        if let None = event_handler{
            return;
        }


        start_socket(Arc::new(self),event_handler.unwrap()).await;
    }

    pub fn set_event_handler(&mut self, event_handler: Box<dyn EventHandler>){
        self.event_handler = Some(event_handler)
    }

    pub fn set_settins(&mut self, settings: ClientSettings){
        self.client_settings = settings;
    }
}

pub struct ClientSettings{
    pub accept_from_bot: bool,
    pub must_start_with_prefix: bool
}