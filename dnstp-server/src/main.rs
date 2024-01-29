use clap::Parser;
use std::{thread};

use log::info;
use simplelog::*;
use std::fs::File;
use std::net::SocketAddr;

use dnstplib::dns_socket::DNSSocket;
use dnstplib::request_processor::RequestProcesor;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Addresses to bind server to
    #[arg(short, long)]
    address: Vec<String>,
}

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("dnstp.log").unwrap()),
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
    socket.run_tx();

    let mut processor = RequestProcesor::new();
    processor.run(socket.get_tx_message_channel().expect("couldn't get message transmitting channel"));

    socket.run_rx(processor.get_message_channel().expect("couldn't get message processing channel"));

    thread::park();
}
