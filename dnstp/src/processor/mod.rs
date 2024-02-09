//! Business-logic-like structures for processing parsed messages

pub mod request;
pub mod response;

use std::net::SocketAddr;
use log::error;
pub use request::RequestProcesor;
pub use response::ResponseProcesor;
use crate::message::{QuestionParseError, RecordParseError};
use crate::message_parser::{HeaderParseError, MessageParseError};

pub fn print_error(e: MessageParseError, peer: &SocketAddr)
{
    match e {
        MessageParseError::HeaderParse(he) => {
            match he {
                HeaderParseError::OpcodeParse(oe) => {
                    error!("[{}] failed to parse opcode from received message: [{}]", peer, oe);
                }
                HeaderParseError::ResponseCodeParse(rce) => {
                    error!("[{}] failed to parse response code error from received message: [{}]", peer, rce);
                }
            }
        }
        MessageParseError::QuesionsParse(qe) => {
            match qe {
                QuestionParseError::ShortLength(sl) => {
                    error!("[{}] failed to parse questions of received message, too short: [{} bytes]", peer, sl);
                }
                QuestionParseError::QTypeParse(te) => {
                    error!("[{}] failed to parse questions of received message, qtype error: [{}]", peer, te);
                }
                QuestionParseError::QClassParse(ce) => {
                    error!("[{}] failed to parse questions of received message, qclass error: [{}]", peer, ce);
                }
            }
        }
        MessageParseError::RecordParse(rp) => {
            match rp {
                RecordParseError::ShortLength(sl) => {
                    error!("[{}] failed to parse records of received message, too short: [{} bytes]", peer, sl);
                }
                RecordParseError::QTypeParse(te) => {
                    error!("[{}] failed to parse records of received message, qtype error: [{}]", peer, te);
                }
                RecordParseError::QClassParse(ce) => {
                    error!("[{}] failed to parse records of received message, qclass error: [{}]", peer, ce);
                }
            }
        }
        MessageParseError::RecordCount(expected, actual) => {
            error!("[{}] failed to parse records of received message, record count mismatch: [Expected:{}] [Actual:{}]", peer, expected, actual);
        }
    }
}