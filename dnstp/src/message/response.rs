use std::net::SocketAddr;
use crate::message::answer::DNSAnswer;
use crate::message::header::DNSHeader;
use crate::message::question::{DNSQuestion, questions_to_bytes};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct DNSResponse {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>,
    pub answers: Vec<DNSAnswer>,
    pub peer: SocketAddr
}

impl DNSResponse {

    pub fn to_bytes(& self) -> Vec<u8>
    {
        let mut header_bytes = self.header.to_bytes().to_vec();
        let mut body_bytes = questions_to_bytes(&self.questions);

        header_bytes.append(&mut body_bytes);

        return header_bytes
    }
}