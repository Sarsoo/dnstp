use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use log::info;
use crate::net::raw_request::NetworkMessagePtr;
use crate::message_parser::parse_message;
use crate::processor::print_error;

pub struct ResponseProcesor {
    message_channel: Option<Sender<NetworkMessagePtr>>
}

impl ResponseProcesor {
    pub fn new() -> ResponseProcesor {
        ResponseProcesor{
            message_channel: None
        }
    }

    pub fn run(&mut self)
    {
        let (tx, rx): (Sender<NetworkMessagePtr>, Receiver<NetworkMessagePtr>) = mpsc::channel();
        self.message_channel = Some(tx);

        thread::spawn(move || {

            for m in rx
            {
                let peer = m.peer.clone();

                match parse_message(*m) {
                    Ok(r) => {
                        info!("received dns message: {:?}", r);
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