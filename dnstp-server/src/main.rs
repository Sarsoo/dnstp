use clap::Parser;
use std::str;

use log::{error, info, warn};
use simplelog::*;
use std::fs::File;
use std::net::{SocketAddr, UdpSocket};

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

    match UdpSocket::bind(&addresses[..]) {
        Ok(s) => {
            loop {
                let mut buf = [0; 512];
                let res = s.recv_from(&mut buf);

                match res {
                    Ok(r) => {
                        let res_str = str::from_utf8(&buf).unwrap();
                        info!("{}", res_str);
                    }
                    Err(_) => {}
                }
            }
        }
        Err(e) => error!("couldn't bind to address {}", e)
    }

}
