pub mod socket;
pub mod raw_request;

pub use raw_request::{NetworkMessage, NetworkMessagePtr};
pub use socket::DNSSocket;