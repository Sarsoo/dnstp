use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use log::{error, info};
use crate::message::question::QuestionParseError;
use crate::net::raw_request::NetworkMessagePtr;
use crate::request_parser::{HeaderParseError, parse_request, RequestParseError};

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

            for m in rx
            {
                let peer = m.peer.clone();

                match parse_request(*m) {
                    Ok(r) => {
                        info!("received dns message: {:?}", r);
                    }
                    Err(e) => {
                        match e {
                            RequestParseError::HeaderParse(he) => {
                                match he {
                                    HeaderParseError::OpcodeParse(oe) => {
                                        error!("[{}] failed to parse opcode from received message: [{}]", peer, oe);
                                    }
                                    HeaderParseError::ResponseCodeParse(rce) => {
                                        error!("[{}] failed to parse response code error from received message: [{}]", peer, rce);
                                    }
                                }
                            }
                            RequestParseError::QuesionsParse(qe) => {
                                match qe {
                                    QuestionParseError::ShortLength(sl) => {
                                        error!("[{}] failed to parse questions of received message, too short: [{} bytes]", peer, sl);
                                    }
                                    QuestionParseError::QTypeParse(te) => {
                                        error!("[{}] failed to parse questions of received message, qtype error: [{}]", peer, te);
                                    }
                                    QuestionParseError::QClassParse(ce) => {
                                        error!("[{}] failed to parse questions of received message, qclass error: [{}]", peer, ce);
                                    }
                                }
                            }
                        }
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