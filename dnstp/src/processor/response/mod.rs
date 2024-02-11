mod encryption;

use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use log::info;
use crate::client_crypto_context::ClientCryptoContext;
use crate::net::raw_request::NetworkMessagePtr;
use crate::message_parser::parse_message;
use crate::processor::print_error;
use crate::processor::response::encryption::decode_key_response;

pub struct ResponseProcesor {
    message_channel: Option<Sender<NetworkMessagePtr>>,
    crypto_context: Arc<Mutex<ClientCryptoContext>>
}

impl ResponseProcesor {
    pub fn new(crypto_context: Arc<Mutex<ClientCryptoContext>>) -> ResponseProcesor {
        ResponseProcesor{
            message_channel: None,
            crypto_context
        }
    }

    pub fn run(&mut self)
    {
        let (tx, rx): (Sender<NetworkMessagePtr>, Receiver<NetworkMessagePtr>) = mpsc::channel();
        self.message_channel = Some(tx);

        let crypto_context = self.crypto_context.clone();

        thread::spawn(move || {

            for m in rx
            {
                let peer = m.peer.clone();

                match parse_message(*m) {
                    Ok(r) => {
                        info!("received dns message: {:?}", r);

                        decode_key_response(&r, crypto_context.clone());
                    }
                    Err(e) => {
                        print_error(e, &peer);
                    }
                }
            }

            info!("message processing thread finishing")
        });
    }

    pub fn get_message_channel(&mut self) -> Option<Sender<NetworkMessagePtr>>
    {
        self.message_channel.clone()
    }
}