use std::net::{SocketAddr, UdpSocket};
use clap::Parser;
use log::error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

}

fn main() {
    let args = Args::parse();

    let address = SocketAddr::from(([127, 0, 0, 1], 0));

    match UdpSocket::bind(&address) {
        Ok(s) => {
            loop {

                let send_buf = "hello world";
                let send_res = s.send_to(send_buf.as_bytes(), "127.0.0.1:5000");

            }
        }
        Err(e) => error!("couldn't bind to address {}", e)
    }
}
