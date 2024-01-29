use std::net::SocketAddr;
use crate::dns_header::DNSHeader;
use crate::dns_question::DNSQuestion;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct DNSRequest {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>,
    pub peer: SocketAddr
}