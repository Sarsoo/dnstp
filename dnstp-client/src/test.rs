use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use log::info;
use rand::RngCore;
use dnstplib::client_crypto_context::ClientCryptoContext;
use dnstplib::DomainConfig;
use dnstplib::message::DNSMessage;
use dnstplib::net::{DNSSocket, NetworkMessage};
use dnstplib::processor::ResponseProcesor;
use crate::NetSettings;

pub fn send_test_requests(args: NetSettings)
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
        base_domain: args.base_domain,
        key_endpoint: args.key_endpoint
    };

    let domain = domain_config.get_fq_key_endpoint();

    let mut rng = rand::thread_rng();
    loop {
        info!("sending...");

        let message = DNSMessage::req_from_hostname(address, rng.next_u32() as u16, domain.clone());

        let bytes = message.to_bytes();

        tx_channel.send(Box::new(NetworkMessage {
            buffer: Box::new(bytes),
            peer: args.address.parse().unwrap()
        }));

        thread::sleep(Duration::from_secs(1));
    }
}