use std::net::SocketAddr;

pub type NetworkMessagePtr = Box<NetworkMessage>;

pub struct NetworkMessage {
    pub buffer: Box<Vec<u8>>,
    pub peer: SocketAddr
}