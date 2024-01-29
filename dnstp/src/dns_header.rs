pub enum Direction {
    Request, Response
}

pub enum Opcode {
    Query = 0,
    RQuery = 1,
    Status = 2,
    Reserved = 3
}

pub enum ResponseCode {
    NoError = 0,
    FormatSpecError = 1,
    ServerFailure = 2,
    NameError = 3,
    RequestTypeUnsupported = 4,
    NotExecuted = 5
}

pub struct DNSHeader {
    pub id: u16,
    pub direction: Direction,
    pub opcode: Opcode,
    pub authoritative: bool,
    pub truncation: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub response: ResponseCode,
    pub question_count: u16,
    pub answer_record_count: u16,
    pub authority_record_count: u16,
    pub additional_record_count: u16,
}