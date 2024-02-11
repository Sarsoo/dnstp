//! # Client Side
//!

mod test;
mod upload;
mod download;

use std::fs::OpenOptions;
use clap::{Parser, Subcommand};
use log::{LevelFilter};
use simplelog::*;

use crate::download::download;
use crate::test::send_test_requests;
use crate::upload::upload;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Send test requests on loop to the server
    Test {
        #[clap(flatten)]
        net_options: NetSettings
    },
    /// Upload data to the remote server
    Upload {
        #[clap(flatten)]
        net_options: NetSettings,
        #[arg(short, long)]
        value: String
    },
    /// Download a payload from the remote server
    Download {
        #[clap(flatten)]
        net_options: NetSettings
    }
}

#[derive(Parser, Debug)]
struct NetSettings {
    /// Server address to send requests to
    #[arg(short, long)]
    address: String,
    /// Base domain server is operating on
    #[arg(long)]
    base_domain: String,
    /// Sub-domain to handle key handling when requested
    #[arg(short, long, default_value = "static")]
    key_endpoint: String,
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

    match args.command {
        Command::Test { net_options } => {
            send_test_requests(net_options);
        }
        Command::Upload { net_options, value } => {
            upload(net_options, value);
        }
        Command::Download { net_options } => {
            download(net_options);
        }
    }
}
