use std::convert::TryFrom;

pub const HEADER_SIZE: usize = 12;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    Request = 0,
    Response = 1
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub enum Opcode {
    Query = 0,
    RQuery = 1,
    Status = 2,
    Reserved = 3
}

impl TryFrom<u16> for Opcode {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == Opcode::Query as u16 => Ok(Opcode::Query),
            x if x == Opcode::RQuery as u16 => Ok(Opcode::RQuery),
            x if x == Opcode::Status as u16 => Ok(Opcode::Status),
            x if x == Opcode::Reserved as u16 => Ok(Opcode::Reserved),
            _ => Err(()),
        }
    }
}

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
    type Error = ();

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
            _ => Err(()),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct DNSHeader {
    pub id: u16,
    pub direction: Direction,
    pub opcode: Opcode,
    pub authoritative: bool,
    pub truncation: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub valid_zeroes: bool,
    pub response: ResponseCode,
    pub question_count: u16,
    pub answer_record_count: u16,
    pub authority_record_count: u16,
    pub additional_record_count: u16,
}