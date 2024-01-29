use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use log::info;
use std::str;
use crate::raw_request::NetworkMessagePtr;

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

            for mut m in rx
            {
                info!("processing: {}", str::from_utf8(&(*(*m).buffer)).unwrap());

                // (*(*m).buffer).reverse();

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