//! # Common Functionality
//! The vast majority of functionality is in this library crate. The client and server executable crates are really just wiring up bits and pieces from this library.

mod byte;
pub mod processor;
pub mod message;
pub mod net;
pub mod string;
pub mod config;
pub mod crypto;
pub mod session;

use std::sync::mpsc::Sender;
use log::error;
pub use config::DomainConfig;
use crate::message::DNSMessage;
use crate::net::{NetworkMessage, NetworkMessagePtr};

#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum RequestError {
    /// Trying to perform an operation without having handshaked first
    NoHandshake,
    WrongNumberOfQuestions,
    CryptoFailure
}

pub fn send_message(response: DNSMessage, sending_channel: &Sender<NetworkMessagePtr>)
{
    match sending_channel.send(Box::new(
        NetworkMessage {
            buffer: Box::new(response.to_bytes()),
            peer: response.peer
        }
    )){
        Ok(_) => {}
        Err(e) => {
            error!("failed to pass a message to the network layer for delivery [{}]", e.to_string());
        }
    }
}