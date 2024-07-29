//! # Server Side
//! DNS server component for processing requests and replying with DNS records.
//!
//! The aim is to have clients exfil to this server and to allow pulling down data from the server.

use clap::Parser;
use std::{thread};

use log::info;
use simplelog::*;
use std::fs::OpenOptions;
use std::net::SocketAddr;
use dnstplib::DomainConfig;

use dnstplib::net::DNSSocket;
use dnstplib::processor::RequestProcesor;

/// Command-line arguments for configuring the server
#[derive(Parser, Debug)]
#[command(name = "DNSTPd")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Addresses to bind server to
    #[arg(short, long)]
    address: Vec<String>,
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

    info!("======================");
    info!("       dnstp");
    info!("======================");

    for a in args.address.iter() {
        info!("Binding to {}", a)
    }

    let addresses: Vec<SocketAddr> = args.address
        .iter()
        .map(|x| x.parse().expect("Couldn't parse address"))
        .collect();

    let mut socket = DNSSocket::new(addresses);
    socket.bind();
    socket.run_tx();

    let mut processor = RequestProcesor::new(DomainConfig {
        base_domain: args.base_domain,
        key_endpoint: args.key_endpoint
    });
    processor.run(socket.get_tx_message_channel().expect("couldn't get message transmitting channel"));

    socket.run_rx(processor.get_message_channel().expect("couldn't get message processing channel"));

    thread::park();
}
