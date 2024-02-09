//! # Client Side
//!

use std::fs::OpenOptions;
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;
use clap::Parser;
use log::{info, LevelFilter};
use rand::RngCore;
use simplelog::*;
use dnstplib::DomainConfig;

use dnstplib::message::DNSMessage;
use dnstplib::net::{DNSSocket, NetworkMessage};
use dnstplib::processor::ResponseProcesor;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Addresses to send requests
    #[arg(short, long)]
    address: String,
    /// Base domain to operate on
    #[arg(long)]
    base_domain: String,
    /// Sub-domain to handle key handling when requested
    #[arg(long, default_value = "static")]
    key_endpoint: String
}

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), OpenOptions::new()
                .read(true)
                .write(true)
                .append(true)
                .create(true)
                .open("dnstp.log").unwrap()),
        ]
    ).unwrap();

    let args = Args::parse();

    let address = SocketAddr::from(([127, 0, 0, 1], 0));

    let mut socket = DNSSocket::new(vec!(address));
    socket.bind();
    socket.run_tx();

    let tx_channel = socket.get_tx_message_channel().unwrap();

    let mut processor = ResponseProcesor::new();
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
