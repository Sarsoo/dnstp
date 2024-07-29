//! Structures making up the DNS workflow including messages and headers
pub mod header;
pub mod question;
pub mod message;
pub mod record;
pub mod message_parser;

pub use question::{DNSQuestion, QClass, QType, QuestionParseError, questions_from_bytes, questions_to_bytes};
pub use record::{AAAARdata, ARdata, RawRData, RData, RecordParseError, records_from_bytes, records_to_bytes, ResourceRecord, TXTRdata};
pub use header::{Direction, DNSHeader, HEADER_SIZE, Opcode, ResponseCode};
pub use message::DNSMessage;
pub use message_parser::*;