use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use log::info;
use rand::RngCore;
use rand::rngs::OsRng;
use dnstplib::client_crypto_context::ClientCryptoContext;
use dnstplib::{DomainConfig, send_message};
use dnstplib::message::{Direction, DNSHeader, DNSMessage, DNSQuestion, Opcode, QClass, QType, ResponseCode};
use dnstplib::net::DNSSocket;
use dnstplib::processor::ResponseProcesor;
use crate::NetSettings;

pub fn upload(net_settings: NetSettings, value: String)
{
    let address = SocketAddr::from(([127, 0, 0, 1], 0));

    let mut socket = DNSSocket::new(vec!(address));
    socket.bind();
    socket.run_tx();

    let tx_channel = socket.get_tx_message_channel().unwrap();

    let crypto_context = Arc::new(Mutex::new(ClientCryptoContext::new()));
    let mut processor = ResponseProcesor::new(crypto_context.clone());
    processor.run();

    socket.run_rx(processor.get_message_channel().expect("couldn't get message processing channel"));

    let domain_config = DomainConfig {
        base_domain: net_settings.base_domain,
        key_endpoint: net_settings.key_endpoint
    };

    info!("sending handshake...");

    let message = DNSMessage {
        header: DNSHeader {
            id: OsRng.next_u32() as u16,
            direction: Direction::Request,
            opcode: Opcode::Query,
            authoritative: false,
            truncation: false,
            recursion_desired: false,
            recursion_available: false,
            valid_zeroes: true,
            response: ResponseCode::NoError,
            question_count: 2,
            answer_record_count: 0,
            authority_record_count: 0,
            additional_record_count: 0,
        },
        questions: vec![
            DNSQuestion {
                qname: domain_config.get_fq_key_endpoint(),
                qtype: QType::A,
                qclass: QClass::Internet,
            },
            DNSQuestion {
                qname: crypto_context.lock().unwrap().get_public_key_domain(&domain_config.base_domain),
                qtype: QType::A,
                qclass: QClass::Internet,
            }
        ],
        answer_records: vec![],
        authority_records: vec![],
        additional_records: vec![],
        peer: net_settings.address.parse().unwrap(),
    };

    send_message(message, &tx_channel);

    while !crypto_context.lock().unwrap().is_complete() {
        info!("waiting for crypto completion...");

        thread::sleep(Duration::from_millis(100));
    }

    info!("crypto complete, sending data");
}