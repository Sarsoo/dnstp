use std::net::Ipv4Addr;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use log::{error, info};
use crate::config::DomainConfig;

use crate::message::{QuestionParseError, DNSResponse};
use crate::net::{NetworkMessage, NetworkMessagePtr};
use crate::request_parser::{HeaderParseError, parse_request, RequestParseError};

pub struct RequestProcesor {
    message_channel: Option<Sender<NetworkMessagePtr>>,
    domain_config: DomainConfig
}

impl RequestProcesor {
    pub fn new(domain_config: DomainConfig) -> RequestProcesor {
        RequestProcesor {
            message_channel: None,
            domain_config
        }
    }

    pub fn run(&mut self, sending_channel: Sender<NetworkMessagePtr>)
    {
        let (tx, rx): (Sender<NetworkMessagePtr>, Receiver<NetworkMessagePtr>) = mpsc::channel();
        self.message_channel = Some(tx);

        let mut base_domain_equality = self.domain_config.base_domain.clone();
        base_domain_equality.insert_str(0, ".");
        let base_domain_len = base_domain_equality.len() + 1;

        thread::spawn(move || {

            for m in rx
            {
                let peer = m.peer.clone();

                match parse_request(*m) {
                    Ok(r) => {
                        info!("received dns message: {:?}", r);

                        if r.questions.iter().any(|q| q.qname.ends_with(&base_domain_equality))
                        {

                        }
                        else {
                            let mut response = DNSResponse::a_from_request(&r, |q| Ipv4Addr::from([127, 0, 0, 1]));

                            sending_channel.send(Box::from(
                                NetworkMessage {
                                    buffer: Box::from(response.to_bytes()),
                                    peer: response.peer
                                }
                            ));
                        }
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
            }

            info!("message processing thread finishing")
        });
    }

    pub fn get_message_channel(&mut self) -> Option<Sender<NetworkMessagePtr>>
    {
        self.message_channel.clone()
    }
}