use std::net::SocketAddr;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use log::{error, info};
use crate::clients::Clients;
use crate::config::DomainConfig;

use crate::message::{DNSMessage, QType};
use crate::net::{NetworkMessagePtr};
use crate::message_parser::parse_message;
use crate::processor::print_error;
use crate::processor::request::encryption::{decode_key_request, DecodeKeyRequestError};
use crate::{RequestError, send_message};

pub mod encryption;

#[cfg(test)]
mod tests;
mod upload;
mod download;

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

    /// Spin a thread to process parsed DNS requests and respond as appropriate
    pub fn run(&mut self, sending_channel: Sender<NetworkMessagePtr>)
    {
        let (tx, rx): (Sender<NetworkMessagePtr>, Receiver<NetworkMessagePtr>) = mpsc::channel();
        self.message_channel = Some(tx);

        let mut base_domain_equality = self.domain_config.base_domain.clone();
        base_domain_equality.insert_str(0, ".");

        let fq_key_endpoint = self.encryption_endpoint.clone();
        let clients = self.clients.clone();

        thread::spawn(move || {

            for m in rx
            {
                let peer = m.peer.clone();

                match parse_message(*m) {
                    Ok(r) => {
                        info!("received dns message: {:?}", r);

                        // If there is a question containing the protocol base domain, treat it as a dnstp request
                        // (handshake, upload, download) and handle as such
                        if r.questions.iter().any(|q| q.qname.ends_with(&base_domain_equality))
                        {
                            Self::handle_dnstp_request(r, &sending_channel, &clients, peer, &fq_key_endpoint);
                        }
                        else // otherwise it's for something else and reply with some dumb empty answer
                        {
                            send_message(DNSMessage::dumb_resp_from_request(&r), &sending_channel);
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

    /// The message is trying to do some dnstp, work out whether it's hello handshaking, uplaoding or downloading data and handoff the message to that workflow
    fn handle_dnstp_request(r: DNSMessage, sending_channel: &Sender<NetworkMessagePtr>, clients: &Arc<Mutex<Clients>>, peer: SocketAddr, fq_key_endpoint: &String)
    {
        // if the first question is for the key domain (static.BLANK.TLD) it's a handshake for swapping keys
        if r.questions[0].qname.eq_ignore_ascii_case(fq_key_endpoint)
        {
            Self::handle_encryption_handshake(r, sending_channel, clients, peer);
        }
            // if we're not handshaking then the client should be known to the server
        else if clients.lock().unwrap().client_is_connected(&r.questions[0].qname) {
            info!("[{}] received request from known client", peer);

            // for now lets deal with three questions, first one is the client id, second is the actual request, third is the nonce
            if r.questions.len() == 3
            {
                match r.questions[1].qtype {
                    QType::A => {
                        Self::handle_upload_request(r, sending_channel, clients, peer);
                    }
                    QType::CNAME => { // makes sense for a cname to return lots of text
                        Self::handle_download_request(r, sending_channel, clients, peer);
                    }
                    _ => {}
                }
            }
            else
            {
                Self::send_protocol_error(RequestError::WrongNumberOfQuestions, &r, &sending_channel);
            }
        }
            // otherwise return protocol error for trying to do something without handshaking
        else
        {
            Self::send_protocol_error(RequestError::NoHandshake, &r, &sending_channel);
        }
    }

    /// Process a hello message from a new client with a public key and send the response.
    ///
    /// Generate a key pair and repspond with the public key, generate the shared secret and store this in the connected clients.
    fn handle_encryption_handshake(r: DNSMessage, sending_channel: &Sender<NetworkMessagePtr>, clients: &Arc<Mutex<Clients>>, peer: SocketAddr)
    {
        info!("[{}] received encryption key request", peer);

        // crypto bulk happens in decode, result includes message to be responded with
        match decode_key_request(&r)
        {
            Ok(context) => {
                clients.lock().unwrap().add(context.client_public, context.new_client);

                send_message(context.response, &sending_channel);
            }
            Err(e) => {
                match e {
                    DecodeKeyRequestError::QuestionCount(qc) => {
                        error!("[{}] failed to parse public key, wrong question count [{}]", peer, qc);
                    }
                    DecodeKeyRequestError::FirstQuestionNotA(qtype) => {
                        error!("[{}] failed to parse public key, first question wasn't an A request [{}]", peer, qtype);
                    }
                    DecodeKeyRequestError::SecondQuestionNotA(qtype) => {
                        error!("[{}] failed to parse public key, second question wasn't an A request [{}]", peer, qtype);
                    }
                    DecodeKeyRequestError::SharedSecretDerivation => {
                        error!("[{}] failed to parse public key, failed to derived shared secret", peer);
                    }
                }

                Self::send_protocol_error(RequestError::CryptoFailure, &r, &sending_channel);
            }
        }
    }

    fn handle_download_request(r: DNSMessage, sending_channel: &Sender<NetworkMessagePtr>, clients: &Arc<Mutex<Clients>>, peer: SocketAddr)
    {
        info!("[{}] received download request", peer);
        let client_id = &r.questions[0].qname;
        clients.lock().unwrap().bump_last_seen(client_id);
    }

    fn handle_upload_request(r: DNSMessage, sending_channel: &Sender<NetworkMessagePtr>, clients: &Arc<Mutex<Clients>>, peer: SocketAddr)
    {
        info!("[{}] received upload request", peer);
        let client_id = &r.questions[0].qname;
        clients.lock().unwrap().bump_last_seen(client_id);
    }

    pub fn get_message_channel(&mut self) -> Option<Sender<NetworkMessagePtr>>
    {
        self.message_channel.clone()
    }

    pub fn send_protocol_error(error_code: RequestError, r: &DNSMessage, sending_channel: &Sender<NetworkMessagePtr>)
    {
        send_message(DNSMessage::protocol_error_from_request(&r, error_code), sending_channel);
    }
}
