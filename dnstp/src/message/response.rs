use std::net::{Ipv4Addr, SocketAddr};
use crate::message::{Direction, DNSHeader, DNSRequest, ResponseCode, answers_to_bytes, ARdata, DNSAnswer, DNSQuestion, questions_to_bytes};

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

    pub fn a_from_request(request: &DNSRequest, ip: impl Fn(&DNSQuestion) -> Ipv4Addr) -> DNSResponse
    {
        let mut response = DNSResponse{
            header: request.header.clone(),
            questions: request.questions.clone(),
            answers: vec![],
            peer: request.peer
        };

        response.answers = request.questions
            .iter()
            .map(|x|
                DNSAnswer::from_query(x,
                                      12,
                                      Box::from(ARdata::from(ip(x))),
                                      None))
            .collect();

        response.header.direction = Direction::Response;
        response.header.response = ResponseCode::NoError;
        response.header.answer_record_count = response.answers.len() as u16;
        response.header.authority_record_count = 0;
        response.header.additional_record_count = 0;

        if response.header.recursion_desired {
            response.header.recursion_available = true;
        }

        response
    }
}