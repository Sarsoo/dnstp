use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use log::{error, info};
use std::str;
use crate::dns_request::DNSRequest;
use crate::raw_request::NetworkMessagePtr;
use crate::request_parser::parse_request;

pub struct RequestProcesor {
    message_channel: Option<Sender<NetworkMessagePtr>>
}

impl RequestProcesor {
    pub fn new() -> RequestProcesor {
        RequestProcesor{
            message_channel: None
        }
    }

    pub fn run(&mut self, sending_channel: Sender<NetworkMessagePtr>)
    {
        let (tx, rx): (Sender<NetworkMessagePtr>, Receiver<NetworkMessagePtr>) = mpsc::channel();
        self.message_channel = Some(tx);

        thread::spawn(move || {

            for mut m in rx
            {
                // info!("processing: {}", str::from_utf8(&(*(*m).buffer)).unwrap());

                let request = parse_request(*m);

                match request {
                    Ok(r) => {
                        info!("received dns message: {:?}", r);
                    }
                    Err(_) => {
                        error!("failed to parse message");
                    }
                }

                // match sending_channel.send(m) {
                //     Ok(_) => {}
                //     Err(_) => {}
                // }
            }

            info!("message processing thread finishing")
        });
    }

    pub fn get_message_channel(&mut self) -> Option<Sender<NetworkMessagePtr>>
    {
        self.message_channel.clone()
    }
}