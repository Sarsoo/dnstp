use std::net::{Ipv4Addr, SocketAddr};
use crate::message::{DNSQuestion, DNSHeader, questions_to_bytes, Direction, ResponseCode, QType, QClass, ResourceRecord, records_to_bytes, ARdata, TXTRdata, RData};
use crate::RequestError;

pub const MESSAGE_SIZE: usize = 512;

/// A DNS message which can be used as either a request or response based on its direction and composition
#[derive(Debug)]
pub struct DNSMessage {
    /// Status/request codes, counts for other collections
    pub header: DNSHeader,
    /// Hostname queries, should be the same in both requests and responses
    pub questions: Vec<DNSQuestion>,
    /// Responses for [`DNSMessage::questions`], has similar structure with varying data field based on query type
    pub answer_records: Vec<ResourceRecord>,
    pub authority_records: Vec<ResourceRecord>,
    pub additional_records: Vec<ResourceRecord>,
    /// IP and socket address of the client which sent this message/client to send message to
    pub peer: SocketAddr
}

impl DNSMessage {

    /// Transform a message into a network transmissable byte sequence
    pub fn to_bytes(& self) -> Vec<u8>
    {
        let mut header_bytes = self.header.to_bytes().to_vec();
        let mut body_bytes = questions_to_bytes(&self.questions);
        let mut answer_bytes = records_to_bytes(&self.answer_records);
        let mut authority_bytes = records_to_bytes(&self.authority_records);
        let mut additional_bytes = records_to_bytes(&self.additional_records);

        header_bytes.append(&mut body_bytes);
        header_bytes.append(&mut answer_bytes);
        header_bytes.append(&mut authority_bytes);
        header_bytes.append(&mut additional_bytes);

        return header_bytes
    }

    /// Helper function for getting a DNS request for the IPv4 of a single hostname
    pub fn req_from_hostname(peer: SocketAddr, id: u16, hostname: String) -> DNSMessage
    {
        DNSMessage {
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

    /// Helper function to get a DNS request for the IPv4s of multiple hostnames
    pub fn reqs_from_hostnames(peer: SocketAddr, id: u16, hostnames: Vec<String>) -> DNSMessage
    {
        DNSMessage {
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

    pub fn a_resp_from_request(&self, ip: impl Fn(&DNSQuestion) -> Ipv4Addr) -> DNSMessage
    {
        let mut response = DNSMessage{
            header: self.header.clone(),
            questions: self.questions.clone(),
            answer_records: vec![],
            authority_records: vec![],
            additional_records: vec![],
            peer: self.peer
        };

        response.answer_records = self.questions
            .iter()
            .map(|x|
                ResourceRecord::from_query(x,
                                           12,
                                           Box::new(ARdata::from(ip(x))),
                                           None))
            .collect();

        response.header.direction = Direction::Response;
        response.header.response = ResponseCode::NoError;
        response.header.answer_record_count = response.answer_records.len() as u16;
        response.header.authority_record_count = 0;
        response.header.additional_record_count = 0;

        if response.header.recursion_desired {
            response.header.recursion_available = true;
        }

        response
    }

    pub fn dumb_resp_from_request(&self) -> DNSMessage
    {
        let mut response = DNSMessage{
            header: self.header.clone(),
            questions: self.questions.clone(),
            answer_records: vec![],
            authority_records: vec![],
            additional_records: vec![],
            peer: self.peer
        };

        response.header.direction = Direction::Response;
        response.header.response = ResponseCode::NotImplemented;
        response.header.answer_record_count = 0;
        response.header.authority_record_count = 0;
        response.header.additional_record_count = 0;

        response
    }

    pub fn empty_resp_from_request(&self) -> DNSMessage
    {
        let mut response = DNSMessage{
            header: self.header.clone(),
            questions: self.questions.clone(),
            answer_records: vec![],
            authority_records: vec![],
            additional_records: vec![],
            peer: self.peer
        };

        response.header.direction = Direction::Response;
        response.header.response = ResponseCode::NoError;
        response.header.answer_record_count = 0;
        response.header.authority_record_count = 0;
        response.header.additional_record_count = 0;

        if response.header.recursion_desired {
            response.header.recursion_available = true;
        }

        response
    }

    pub fn protocol_error_from_request(&self, _error_code: RequestError) -> DNSMessage
    {
        let txt = Box::new(TXTRdata::from(String::new()));

        let mut response = DNSMessage{
            header: self.header.clone(),
            questions: self.questions.clone(),
            answer_records: vec![ResourceRecord {
                name_offset: 12,
                answer_type: QType::TXT,
                class: QClass::Internet,
                ttl: 0,
                rd_length: txt.to_bytes().len() as u16,
                r_data: txt
            }],
            authority_records: vec![],
            additional_records: vec![],
            peer: self.peer
        };

        response.header.direction = Direction::Response;
        response.header.response = ResponseCode::ServerFailure;
        response.header.answer_record_count = 1;
        response.header.authority_record_count = 0;
        response.header.additional_record_count = 0;

        response
    }
}