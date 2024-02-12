mod encryption;

use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use log::{error, info};
use crate::client_crypto_context::ClientCryptoContext;
use crate::net::raw_request::NetworkMessagePtr;
use crate::message_parser::parse_message;
use crate::processor::print_error;
use crate::processor::response::encryption::{decode_key_response, DecodeKeyResponseError};
use crate::string::DomainDecodeError;

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

                        match decode_key_response(&r, crypto_context.clone())
                        {
                            Ok(_) => {
                                info!("successfully decoded key response from server");
                            }
                            Err(e) => {
                                match e {
                                    DecodeKeyResponseError::DomainDecode(dd) => {
                                        match dd {
                                            DomainDecodeError::UTF8Parse => {
                                                error!("failed to decode key response from server, failed to UTF-8 parse response");
                                            }
                                            DomainDecodeError::URLDecode => {
                                                error!("failed to decode key response from server, failed to URL decode response");
                                            }
                                        }
                                    }
                                    DecodeKeyResponseError::KeyDerivation => {
                                        error!("failed to decode key response from server, key derivation failed");
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        print_error(e, &peer);
                    }
                }
            }

            info!("message processing thread finishing");
        });
    }

    pub fn get_message_channel(&mut self) -> Option<Sender<NetworkMessagePtr>>
    {
        self.message_channel.clone()
    }
}