//! Functions for constructing internal DNS structures from network message bytes

use crate::byte;
use crate::message::{DNSMessage, Direction, DNSHeader, Opcode, ResponseCode, QuestionParseError, questions_from_bytes, records_from_bytes, RecordParseError};
use crate::net::NetworkMessage;
use crate::message_parser::RequestParseError::{HeaderParse, QuesionsParse};

pub const ID_START: usize = 0;
pub const FLAGS_START: usize = 2;
pub const DIRECTION_SHIFT: usize = 15;
pub const OPCODE_SHIFT: usize = 11;
pub const AUTHORITATIVE_SHIFT: usize = 10;
pub const TRUNCATION_SHIFT: usize = 9;
pub const RECURSION_DESIRED_SHIFT: usize = 8;
pub const RECURSION_AVAILABLE_SHIFT: usize = 7;
pub const ZEROES_SHIFT: usize = 4;
pub const QUESTION_COUNT_START: usize = 4;
pub const ANSWER_RECORD_COUNT_START: usize = 6;
pub const AUTHORITY_RECORD_COUNT_START: usize = 8;
pub const ADDITIONAL_RECORD_COUNT_START: usize = 10;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum HeaderParseError {
    OpcodeParse(u16),
    ResponseCodeParse(u16),
}

pub fn parse_header(header: &[u8; 12]) -> Result<DNSHeader, HeaderParseError>
{
    let id = byte::two_byte_extraction(header, ID_START);

    let flags = byte::two_byte_extraction(header, FLAGS_START);
    let direction = if flags & (0b1 << DIRECTION_SHIFT) == 0 {Direction::Request} else { Direction::Response };

    let opcode: Result<Opcode, u16> = ((flags & (0b1111 << OPCODE_SHIFT)) >> OPCODE_SHIFT).try_into();
    if let Err(e) = opcode {
        return Err(HeaderParseError::OpcodeParse(e));
    }

    let authoritative = (flags & (0b1 << AUTHORITATIVE_SHIFT)) != 0;
    let truncation = (flags & (0b1 << TRUNCATION_SHIFT)) != 0;
    let recursion_desired = (flags & (0b1 << RECURSION_DESIRED_SHIFT)) != 0;
    let recursion_available = (flags & (0b1 << RECURSION_AVAILABLE_SHIFT)) != 0;

    let zeroes = (flags & (0b111 << ZEROES_SHIFT)) == 0;

    let response: Result<ResponseCode, u16> = (flags & 0b1111).try_into();
    if let Err(e) = response
    {
        return Err(HeaderParseError::ResponseCodeParse(e));
    }

    let question_count = byte::two_byte_extraction(header, QUESTION_COUNT_START);
    let answer_record_count = byte::two_byte_extraction(header, ANSWER_RECORD_COUNT_START);
    let authority_record_count = byte::two_byte_extraction(header, AUTHORITY_RECORD_COUNT_START);
    let additional_record_count = byte::two_byte_extraction(header, ADDITIONAL_RECORD_COUNT_START);

    Ok(DNSHeader {
        id,

        direction,
        opcode: opcode.unwrap(),
        authoritative,
        truncation,
        recursion_desired,
        recursion_available,

        valid_zeroes: zeroes,

        response: response.unwrap(),

        question_count,
        answer_record_count,
        authority_record_count,
        additional_record_count
    })
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum RequestParseError {
    HeaderParse(HeaderParseError),
    QuesionsParse(QuestionParseError),
    RecordParse(RecordParseError),
    RecordCount(u16, usize),
}

pub fn parse_message(msg: NetworkMessage) -> Result<DNSMessage, RequestParseError>
{
    let header = parse_header(msg.buffer[0..12].try_into().unwrap());

    match header {
        Ok(header) => {
            let mut trimmed = msg.buffer.to_vec();
            trimmed.drain(0 .. 12);

            match questions_from_bytes(trimmed, header.question_count)
            {
                Ok((questions, remaining)) => {
                    if remaining.len() > 0 {

                        // can't handle EDNS records at the moment
                        // let total_records = header.answer_record_count + header.authority_record_count + header.additional_record_count;
                        let total_records = header.answer_record_count + header.authority_record_count;

                        match records_from_bytes(remaining, total_records){
                            Ok((mut answers, _)) => {

                                if answers.len() != total_records as usize {
                                    return Err(RequestParseError::RecordCount(total_records, answers.len()));
                                }
                                else {
                                    let answer_records = answers.drain(0 .. (header.answer_record_count as usize)).collect();
                                    let authority_records = answers.drain(0 .. (header.authority_record_count as usize)).collect();

                                    return Ok(DNSMessage {
                                        header,
                                        questions,
                                        peer: msg.peer,
                                        answer_records,
                                        authority_records,
                                        additional_records: answers
                                    });
                                }
                            }
                            Err(e) => return Err(RequestParseError::RecordParse(e))
                        }
                    }
                    else {
                        return Ok(DNSMessage {
                            header,
                            questions,
                            peer: msg.peer,
                            answer_records: vec![],
                            authority_records: vec![],
                            additional_records: vec![]
                        });
                    }
                }
                Err(e) => return Err(QuesionsParse(e))
            }
        },
        Err(e) => return Err(HeaderParse(e))
    }
}

#[cfg(test)]
mod tests {
    use crate::byte::{two_byte_extraction, two_byte_split};
    use super::*;

    #[test]
    fn two_byte_extraction_test() {
        let buffer: [u8; 12] = core::array::from_fn(|i| (i + 1) as u8);

        let value = two_byte_extraction(&buffer, 0);
        assert_eq!(value, 258);

        let value = two_byte_extraction(&buffer, 2);
        assert_eq!(value, 772);

    }

    #[test]
    fn two_byte_split_test() {
        let (val1, val2) = two_byte_split(258);
        assert_eq!(val1, 1);
        assert_eq!(val2, 2);

        let (val1, val2) = two_byte_split(772);
        assert_eq!(val1, 3);
        assert_eq!(val2, 4);
    }

    #[test]
    fn both_ways_test() {
        
        let header = DNSHeader {
            id: 100,
            direction: Direction::Response,
            opcode: Opcode::Query,
            authoritative: true,
            truncation: false,
            recursion_desired: true,
            recursion_available: false,
            valid_zeroes: true,
            response: ResponseCode::NoError,

            question_count: 1,
            answer_record_count: 2,
            authority_record_count: 3,
            additional_record_count: 4
        };

        let parsed_bytes = header.to_bytes();

        let header_again = parse_header(&parsed_bytes).unwrap();

        assert_eq!(header.id, header_again.id);
        assert_eq!(header.direction, header_again.direction);
        assert_eq!(header.opcode, header_again.opcode);
        assert_eq!(header.authoritative, header_again.authoritative);
        assert_eq!(header.truncation, header_again.truncation);
        assert_eq!(header.recursion_desired, header_again.recursion_desired);
        assert_eq!(header.recursion_available, header_again.recursion_available);
        assert_eq!(header.valid_zeroes, header_again.valid_zeroes);
        assert_eq!(header.response, header_again.response);
        assert_eq!(header.question_count, header_again.question_count);
        assert_eq!(header.answer_record_count, header_again.answer_record_count);
        assert_eq!(header.authority_record_count, header_again.authority_record_count);
        assert_eq!(header.additional_record_count, header_again.additional_record_count);
    }
}