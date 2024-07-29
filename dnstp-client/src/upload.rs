use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use log::info;
use rand::rngs::OsRng;
use dnstplib::session::{ClientCryptoContext, generate_client_handshake_message, generate_string_encryption_message};
use dnstplib::{DomainConfig, send_message};
use dnstplib::net::DNSSocket;
use dnstplib::processor::ResponseProcesor;
use crate::NetSettings;

pub fn upload(net_settings: NetSettings, values: Vec<String>)
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

    let message = generate_client_handshake_message(&mut OsRng, &domain_config, crypto_context.clone(), &net_settings.address);

    send_message(message, &tx_channel);

    while !crypto_context.lock().unwrap().is_complete() {
        info!("waiting for crypto completion...");

        thread::sleep(Duration::from_millis(100));
    }

    info!("crypto complete, sending data");

    for v in values {

        info!("sending [{}]", v);
        
        if let Ok(encryption_message) = generate_string_encryption_message(
            v,
            &mut OsRng,
            &domain_config,
            crypto_context.clone(),
            &net_settings.address
        ) {
            send_message(encryption_message, &tx_channel);
        }
    }
}