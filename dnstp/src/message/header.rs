use std::convert::TryFrom;
use crate::byte::apply_split_bytes;

use crate::message::Direction::Response;

/// Size in bytes for a DNS message
pub const HEADER_SIZE: usize = 12;

/// Flag for whether the message represents a request or a response
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    Request = 0,
    Response = 1
}

/// Operation code for describing the purpose of the message
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub enum Opcode {
    Query = 0,
    RQuery = 1,
    Status = 2,
    Reserved = 3
}

impl TryFrom<u16> for Opcode {
    type Error = u16;

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == Opcode::Query as u16 => Ok(Opcode::Query),
            x if x == Opcode::RQuery as u16 => Ok(Opcode::RQuery),
            x if x == Opcode::Status as u16 => Ok(Opcode::Status),
            x if x == Opcode::Reserved as u16 => Ok(Opcode::Reserved),
            _ => Err(v),
        }
    }
}

/// What is the status of the request or response, what was the nature of the error, if encountered
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub enum ResponseCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5,
    YXDomain = 6,
    YXRRSet = 7,
    NXRRSet = 8,
    NotAuth = 9,
    NotZone = 10
}

impl TryFrom<u16> for ResponseCode {
    type Error = u16;

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == ResponseCode::NoError as u16 => Ok(ResponseCode::NoError),
            x if x == ResponseCode::FormatError as u16 => Ok(ResponseCode::FormatError),
            x if x == ResponseCode::ServerFailure as u16 => Ok(ResponseCode::ServerFailure),
            x if x == ResponseCode::NameError as u16 => Ok(ResponseCode::NameError),
            x if x == ResponseCode::NotImplemented as u16 => Ok(ResponseCode::NotImplemented),
            x if x == ResponseCode::Refused as u16 => Ok(ResponseCode::Refused),
            x if x == ResponseCode::YXDomain as u16 => Ok(ResponseCode::YXDomain),
            x if x == ResponseCode::YXRRSet as u16 => Ok(ResponseCode::YXRRSet),
            x if x == ResponseCode::NXRRSet as u16 => Ok(ResponseCode::NXRRSet),
            x if x == ResponseCode::NotAuth as u16 => Ok(ResponseCode::NotAuth),
            x if x == ResponseCode::NotZone as u16 => Ok(ResponseCode::NotZone),
            _ => Err(v),
        }
    }
}

/// Represents a header including flag fields and record counts
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
pub struct DNSHeader {
    /// Random ID for associating responses with requests
    pub id: u16,
    /// Is the message a request or the associated response
    pub direction: Direction,
    /// What is the message function, e.g query, reverse DNS query
    pub opcode: Opcode,
    pub authoritative: bool,
    pub truncation: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub valid_zeroes: bool,
    /// Status of the request or response
    pub response: ResponseCode,
    /// Number of questions being made, should be the same in both the request and response
    pub question_count: u16,
    pub answer_record_count: u16,
    pub authority_record_count: u16,
    pub additional_record_count: u16,
}

impl DNSHeader {
    /// Serialise a header memory structure back into bytes for putting on the wire
    pub fn to_bytes(&self) -> [u8; 12]
    {
        let mut header_bytes: [u8; 12] = [0; 12];

        apply_split_bytes(&mut header_bytes, self.id, crate::message::message_parser::ID_START);

        let mut flags: u16 = 0;

        if self.direction == Response {
            flags |= 0b1 << crate::message::message_parser::DIRECTION_SHIFT;
        }

        flags |= (self.opcode as u16) << crate::message::message_parser::OPCODE_SHIFT;

        flags |= (self.authoritative as u16) << crate::message::message_parser::AUTHORITATIVE_SHIFT;
        flags |= (self.truncation as u16) << crate::message::message_parser::TRUNCATION_SHIFT;
        flags |= (self.recursion_desired as u16) << crate::message::message_parser::RECURSION_DESIRED_SHIFT;
        flags |= (self.recursion_available as u16) << crate::message::message_parser::RECURSION_AVAILABLE_SHIFT;

        flags |= self.response as u16;

        apply_split_bytes(&mut header_bytes, flags, crate::message::message_parser::FLAGS_START);

        apply_split_bytes(&mut header_bytes, self.question_count, crate::message::message_parser::QUESTION_COUNT_START);
        apply_split_bytes(&mut header_bytes, self.answer_record_count, crate::message::message_parser::ANSWER_RECORD_COUNT_START);
        apply_split_bytes(&mut header_bytes, self.authority_record_count, crate::message::message_parser::AUTHORITY_RECORD_COUNT_START);
        apply_split_bytes(&mut header_bytes, self.additional_record_count, crate::message::message_parser::ADDITIONAL_RECORD_COUNT_START);

        header_bytes
    }

    pub fn new_request(id: u16, questions: Option<u16>) -> DNSHeader
    {
        DNSHeader {
            id,
            direction: Direction::Request,
            opcode: Opcode::Query,
            authoritative: false,
            truncation: false,
            recursion_desired: true,
            recursion_available: false,
            valid_zeroes: true,
            response: ResponseCode::NoError,
            question_count: match questions {
                None => 1,
                Some(v) => v
            },
            answer_record_count: 0,
            authority_record_count: 0,
            additional_record_count: 0
        }
    }
}