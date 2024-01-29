use std::net::SocketAddr;

pub type NetworkMessagePtr = Box<NetworkMessage>;

pub struct NetworkMessage {
    pub buffer: Box<[u8; 512]>,
    pub peer: SocketAddr
}