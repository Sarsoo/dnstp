//! Network layer for binding and processing UDP traffic

pub mod socket;
pub mod raw_request;

pub use raw_request::{NetworkMessage, NetworkMessagePtr};
pub use socket::DNSSocket;