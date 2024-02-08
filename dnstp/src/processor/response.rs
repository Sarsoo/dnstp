use std::net::Ipv4Addr;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use log::{error, info};
use std::str;
use crate::message::{DNSResponse, QuestionParseError, RecordParseError};
use crate::net::NetworkMessage;
use crate::net::raw_request::NetworkMessagePtr;
use crate::request_parser::{HeaderParseError, parse_request, RequestParseError};

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
                            RequestParseError::RecordParse(rp) => {
                                match rp {
                                    RecordParseError::ShortLength(sl) => {
                                        error!("[{}] failed to parse records of received message, too short: [{} bytes]", peer, sl);
                                    }
                                    RecordParseError::QTypeParse(te) => {
                                        error!("[{}] failed to parse records of received message, qtype error: [{}]", peer, te);
                                    }
                                    RecordParseError::QClassParse(ce) => {
                                        error!("[{}] failed to parse records of received message, qclass error: [{}]", peer, ce);
                                    }
                                }
                            }
                            RequestParseError::RecordCount(expected, actual) => {
                                error!("[{}] failed to parse records of received message, record count mismatch: [Expected:{}] [Actual:{}]", peer, expected, actual);
                            }
                        }
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