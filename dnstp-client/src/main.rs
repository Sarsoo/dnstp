use std::fs::File;
use std::net::{SocketAddr, UdpSocket};
use std::thread;
use std::time::Duration;
use clap::Parser;
use log::{error, info, LevelFilter};
use simplelog::*;
use dnstplib::dns_socket::DNSSocket;
use dnstplib::raw_request::NetworkMessage;
use dnstplib::request_processor::RequestProcesor;
use dnstplib::response_processor::ResponseProcesor;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

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

    loop {

        info!("sending...");

        let mut send_buf = [0; 512];
        send_buf[0] = 'a' as u8;
        send_buf[1] = 'b' as u8;
        send_buf[2] = 'c' as u8;
        send_buf[3] = 'd' as u8;

        tx_channel.send(Box::from(NetworkMessage {
            buffer: Box::from(send_buf),
            peer: "127.0.0.1:5000".parse().unwrap()
        }));

        thread::sleep(Duration::from_secs(1));
    }
}
