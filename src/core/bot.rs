use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::core::handler::EventHandler;
use crate::core::wss::start_socket;

pub struct Bot {
    token: String,
    command_prefix: String,
    event_handler: Option<Box<dyn EventHandler>>
}

impl Bot {

    pub fn create(token: String, prefix: String) -> Bot {

        let bot = Bot {
            token,
            command_prefix: prefix,
            event_handler: None,
        };


        return bot;
    }

    pub async fn start(&mut self)  {
        let event_handler = self.event_handler.take(); // Takes ownership temporarily

        if let None = event_handler{
            return;
        }

        start_socket(&self, event_handler.unwrap()).await.expect("TODO: panic message");
    }

    pub fn set_event_handler(&mut self, event_handler: Box<dyn EventHandler>){

        self.event_handler = Some(event_handler)

    }
    
  

}

