use crate::dns_header::{Direction, DNSHeader, Opcode, ResponseCode};
use crate::dns_header::Direction::Response;

fn two_byte_extraction(buffer: &[u8], idx: usize) -> u16
{
    ((buffer[idx] as u16) << 8) | buffer[idx + 1] as u16
}

fn two_byte_split(num: u16) -> (u8, u8)
{
    ((num >> 8) as u8, (num & 0b0000000011111111) as u8)
}

const ID_START: usize = 0;
const FLAGS_START: usize = 2;
const DIRECTION_SHIFT: usize = 15;
const OPCODE_SHIFT: usize = 11;
const AUTHORITATIVE_SHIFT: usize = 10;
const TRUNCATION_SHIFT: usize = 9;
const RECURSION_DESIRED_SHIFT: usize = 8;
const RECURSION_AVAILABLE_SHIFT: usize = 7;
const ZEROES_SHIFT: usize = 4;
const QUESTION_COUNT_START: usize = 4;
const ANSWER_RECORD_COUNT_START: usize = 6;
const AUTHORITY_RECORD_COUNT_START: usize = 8;
const ADDITIONAL_RECORD_COUNT_START: usize = 10;

pub fn parse_header(header: &[u8; 12]) -> Result<DNSHeader, ()>
{
    let id = two_byte_extraction(header, ID_START);

    let flags = two_byte_extraction(header, FLAGS_START);
    let direction = if flags & (0b1 << DIRECTION_SHIFT) == 0 {Direction::Request} else { Direction::Response };

    let opcode: Result<Opcode, ()> = ((flags & (0b1111 << OPCODE_SHIFT)) >> OPCODE_SHIFT).try_into();
    if let Err(e) = opcode {
        return Err(e);
    }

    let authoritative = (flags & (0b1 << AUTHORITATIVE_SHIFT)) != 0;
    let truncation = (flags & (0b1 << TRUNCATION_SHIFT)) != 0;
    let recursion_desired = (flags & (0b1 << RECURSION_DESIRED_SHIFT)) != 0;
    let recursion_available = (flags & (0b1 << RECURSION_AVAILABLE_SHIFT)) != 0;

    let zeroes = (flags & (0b111 << ZEROES_SHIFT)) == 0;

    let response: Result<ResponseCode, ()> = (flags & 0b1111).try_into();
    if let Err(e) = response
    {
        return Err(e);
    }

    let question_count = two_byte_extraction(header, QUESTION_COUNT_START);
    let answer_record_count = two_byte_extraction(header, ANSWER_RECORD_COUNT_START);
    let authority_record_count = two_byte_extraction(header, AUTHORITY_RECORD_COUNT_START);
    let additional_record_count = two_byte_extraction(header, ADDITIONAL_RECORD_COUNT_START);

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

fn apply_split_bytes(buffer: &mut [u8], value: u16, index: usize)
{
    let val = two_byte_split(value);
    buffer[index] = val.0;
    buffer[index + 1] = val.1;
}

pub fn parse_header_to_bytes(header: &DNSHeader) -> [u8; 12]
{
    let mut header_bytes: [u8; 12] = [0; 12];

    apply_split_bytes(&mut header_bytes, header.id, ID_START);

    let mut flags: u16 = 0;

    if header.direction == Response {
        flags |= 0b1 << DIRECTION_SHIFT;
    }

    flags |= (header.opcode as u16) << OPCODE_SHIFT;

    flags |= (header.authoritative as u16) << AUTHORITATIVE_SHIFT;
    flags |= (header.truncation as u16) << TRUNCATION_SHIFT;
    flags |= (header.recursion_desired as u16) << RECURSION_DESIRED_SHIFT;
    flags |= (header.recursion_available as u16) << RECURSION_AVAILABLE_SHIFT;

    flags |= header.response as u16;

    apply_split_bytes(&mut header_bytes, flags, FLAGS_START);

    apply_split_bytes(&mut header_bytes, header.question_count, QUESTION_COUNT_START);
    apply_split_bytes(&mut header_bytes, header.answer_record_count, ANSWER_RECORD_COUNT_START);
    apply_split_bytes(&mut header_bytes, header.authority_record_count, AUTHORITY_RECORD_COUNT_START);
    apply_split_bytes(&mut header_bytes, header.additional_record_count, ADDITIONAL_RECORD_COUNT_START);

    header_bytes
}

#[cfg(test)]
mod tests {
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

        let parsed_bytes = parse_header_to_bytes(&header);

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