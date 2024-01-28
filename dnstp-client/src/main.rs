use std::fs::File;
use std::net::{SocketAddr, UdpSocket};
use std::thread;
use std::time::Duration;
use clap::Parser;
use log::{error, info, LevelFilter};
use simplelog::*;

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

    match UdpSocket::bind(&address) {
        Ok(s) => {
            loop {

                info!("sending...");

                let send_buf = "hello world";
                let send_res = s.send_to(send_buf.as_bytes(), "127.0.0.1:5000");

                thread::sleep(Duration::from_secs(1));
            }
        }
        Err(e) => error!("couldn't bind to address {}", e)
    }
}
