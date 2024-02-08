use std::net::SocketAddr;
use crate::message::{DNSQuestion, DNSHeader, questions_to_bytes, Direction, Opcode, ResponseCode, QType, QClass, ResourceRecord};

#[derive(Debug)]
pub struct DNSRequest {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>,
    pub answer_records: Vec<ResourceRecord>,
    pub authority_records: Vec<ResourceRecord>,
    pub additional_records: Vec<ResourceRecord>,
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

    pub fn from_hostname(peer: SocketAddr, id: u16, hostname: String) -> DNSRequest
    {
        DNSRequest {
            header: DNSHeader::new_request(id, None),
            questions: vec![
                DNSQuestion {
                    qname: hostname,
                    qtype: QType::A,
                    qclass: QClass::Internet
                }
            ],
            answer_records: vec![],
            authority_records: vec![],
            additional_records: vec![],
            peer
        }
    }

    pub fn from_hostnames(peer: SocketAddr, id: u16, hostnames: Vec<String>) -> DNSRequest
    {
        DNSRequest {
            header: DNSHeader::new_request(id, Some(hostnames.len() as u16)),
            questions: hostnames
                .into_iter()
                .map(|n|
                    DNSQuestion {
                        qname: n,
                        qclass: QClass::Internet,
                        qtype: QType::A
                    })
                .collect(),
            answer_records: vec![],
            authority_records: vec![],
            additional_records: vec![],
            peer
        }
    }
}