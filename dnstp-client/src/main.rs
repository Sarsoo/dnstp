use std::fs::File;
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;
use clap::Parser;
use log::{info, LevelFilter};
use rand::RngCore;
use simplelog::*;
use dnstplib::message::header::{Direction, DNSHeader, Opcode, ResponseCode};
use dnstplib::message::question::{DNSQuestion, QClass, QType};
use dnstplib::message::request::DNSRequest;
use dnstplib::net::socket::DNSSocket;
use dnstplib::net::raw_request::NetworkMessage;
use dnstplib::processor::ResponseProcesor;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Addresses to send requests
    #[arg(short, long)]
    address: String,
}

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("dnstp.log").unwrap()),
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

    let mut rng = rand::thread_rng();
    loop {

        info!("sending...");

        let message = DNSRequest {
            header: DNSHeader {
                id: rng.next_u32() as u16,
                direction: Direction::Request,
                opcode: Opcode::Query,
                authoritative: false,
                truncation: false,
                recursion_desired: true,
                recursion_available: false,
                valid_zeroes: true,
                response: ResponseCode::NoError,
                question_count: 1,
                answer_record_count: 0,
                authority_record_count: 0,
                additional_record_count: 0
            },
            questions: vec![
                DNSQuestion {
                    qname: "duck.com".to_string(),
                    qtype: QType::A,
                    qclass: QClass::Internet
                }
            ],
            additional_records: vec![],
            peer: address
        };

        let bytes = message.to_bytes();

        tx_channel.send(Box::from(NetworkMessage {
            buffer: Box::from(bytes),
            peer: args.address.parse().unwrap()
        }));

        thread::sleep(Duration::from_secs(1));
    }
}
