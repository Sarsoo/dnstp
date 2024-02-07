//! Structures making up the DNS workflow including requests, responses and headers
pub mod header;
pub mod question;
pub mod request;
pub mod answer;
pub mod response;

pub use question::{DNSQuestion, QClass, QType, QuestionParseError, questions_to_bytes, questions_from_bytes};
pub use answer::{ResourceRecord, RawRData, RData, ARdata, AAAARdata, RecordParseError, records_to_bytes, answers_from_bytes};
pub use header::{DNSHeader, Direction, Opcode, ResponseCode, HEADER_SIZE};
pub use request::DNSRequest;
pub use response::DNSResponse;