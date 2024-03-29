//! Structures making up the DNS workflow including messages and headers
pub mod header;
pub mod question;
pub mod message;
pub mod record;

pub use question::{DNSQuestion, QClass, QType, QuestionParseError, questions_to_bytes, questions_from_bytes};
pub use record::{ResourceRecord, RawRData, RData, ARdata, AAAARdata, TXTRdata, RecordParseError, records_to_bytes, records_from_bytes};
pub use header::{DNSHeader, Direction, Opcode, ResponseCode, HEADER_SIZE};
pub use message::DNSMessage;