use std::net::SocketAddr;
use crate::message::header::DNSHeader;
use crate::message::question::{DNSQuestion, questions_to_bytes};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
pub struct DNSRequest {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>,
    pub additional_records: Vec<DNSQuestion>,
    pub peer: SocketAddr
}

impl DNSRequest {

    pub fn to_bytes(& self) -> Vec<u8>
    {
        let mut header_bytes = self.header.to_bytes().to_vec();
        let mut body_bytes = questions_to_bytes(&self.questions);

        header_bytes.append(&mut body_bytes);

        return header_bytes
    }
}