use std::net::Ipv4Addr;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use log::{error, info};
use crate::clients::Clients;
use crate::config::DomainConfig;

use crate::message::DNSMessage;
use crate::net::{NetworkMessage, NetworkMessagePtr};
use crate::message_parser::parse_message;
use crate::processor::print_error;
use crate::processor::request::encryption::{decode_key_request, KeyDecodeError};

pub mod encryption;

#[cfg(test)]
mod tests;

pub struct RequestProcesor {
    message_channel: Option<Sender<NetworkMessagePtr>>,
    domain_config: DomainConfig,
    encryption_endpoint: String,

    clients: Arc<Mutex<Clients>>
}

impl RequestProcesor {
    pub fn new(domain_config: DomainConfig) -> RequestProcesor {

        let fq_key_endpoint = domain_config.get_fq_key_endpoint();
        RequestProcesor {
            message_channel: None,
            domain_config,
            encryption_endpoint: fq_key_endpoint,
            clients: Arc::new(Mutex::new(Clients::new()))
        }
    }

    pub fn run(&mut self, sending_channel: Sender<NetworkMessagePtr>)
    {
        let (tx, rx): (Sender<NetworkMessagePtr>, Receiver<NetworkMessagePtr>) = mpsc::channel();
        self.message_channel = Some(tx);

        let mut base_domain_equality = self.domain_config.base_domain.clone();
        base_domain_equality.insert_str(0, ".");

        let fq_key_endpoint = self.encryption_endpoint.clone();
        let clients = self.clients.clone();

        thread::spawn(move || {

            let clients = clients;

            for m in rx
            {
                let peer = m.peer.clone();

                match parse_message(*m) {
                    Ok(r) => {
                        info!("received dns message: {:?}", r);

                        if r.questions.iter().any(|q| q.qname.ends_with(&base_domain_equality))
                        {
                            if r.questions[0].qname.eq_ignore_ascii_case(&fq_key_endpoint)
                            {
                                info!("[{}] received encryption key request", peer);

                                match decode_key_request(r)
                                {
                                    Ok(context) => {

                                        clients.lock().unwrap().add(context.client_public, context.new_client);

                                        sending_channel.send(Box::new(
                                            NetworkMessage {
                                                buffer: Box::new(context.response.to_bytes()),
                                                peer: context.response.peer
                                            }
                                        ));
                                    }
                                    Err(e) => {
                                        match e {
                                            KeyDecodeError::QuestionCount(qc) => {
                                                error!("[{}] failed to parse public key, wrong question count [{}]", peer, qc);
                                            }
                                            KeyDecodeError::FirstQuestionNotA(qtype) => {
                                                error!("[{}] failed to parse public key, first question wasn't an A request [{}]", peer, qtype);
                                            }
                                            KeyDecodeError::SecondQuestionNotA(qtype) => {
                                                error!("[{}] failed to parse public key, second question wasn't an A request [{}]", peer, qtype);
                                            }
                                            KeyDecodeError::SharedSecretDerivation => {
                                                error!("[{}] failed to parse public key, failed to derived shared secret", peer);
                                            }
                                        }
                                    }
                                }
                            }
                            else
                            {
                                let response = DNSMessage::a_resp_from_request(&r, |_| Ipv4Addr::from([127, 0, 0, 1]));

                                sending_channel.send(Box::new(
                                    NetworkMessage {
                                        buffer: Box::new(response.to_bytes()),
                                        peer: response.peer
                                    }
                                ));
                            }
                        }
                        else {
                            let response = DNSMessage::a_resp_from_request(&r, |_| Ipv4Addr::from([127, 0, 0, 1]));

                            sending_channel.send(Box::new(
                                NetworkMessage {
                                    buffer: Box::new(response.to_bytes()),
                                    peer: response.peer
                                }
                            ));
                        }
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
