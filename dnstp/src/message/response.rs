use std::net::SocketAddr;
use crate::message::answer::{answers_to_bytes, DNSAnswer};
use crate::message::header::DNSHeader;
use crate::message::question::{DNSQuestion, questions_to_bytes};

#[derive(Debug)]
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
        let mut answer_bytes = answers_to_bytes(&self.answers);

        header_bytes.append(&mut body_bytes);
        header_bytes.append(&mut answer_bytes);

        return header_bytes
    }
}