mod raw_rdata;
pub use raw_rdata::RawRData;

mod a_rdata;
pub use a_rdata::ARdata;

mod aaaa_rdata;
pub use aaaa_rdata::AAAARdata;

mod txt_rdata;
pub use txt_rdata::TXTRdata;


#[cfg(test)]
mod tests;

use std::fmt::Debug;
use std::fmt::Display;
use crate::byte::{four_byte_combine, four_byte_split, push_split_bytes, two_byte_combine};
use crate::message::question::{DNSQuestion, QClass, QType};

pub trait RData: Debug {
    fn to_bytes(&self) -> Vec<u8>;
}

#[derive(Debug)]
pub struct ResourceRecord {
    pub name_offset: u16,
    pub answer_type: QType,
    pub class: QClass,
    pub ttl: u32,
    pub rd_length: u16,
    pub r_data: Box<dyn RData>
}

impl ResourceRecord {

    pub fn to_bytes(&self) -> Vec<u8>
    {
        let mut data_bytes = self.r_data.to_bytes();
        let mut ret = Vec::with_capacity(2 + 2 + 2 + 4 + 2 + data_bytes.len());

        push_split_bytes(&mut ret, self.name_offset | (0b11 << 14));
        push_split_bytes(&mut ret, self.answer_type as u16);
        push_split_bytes(&mut ret, self.class as u16);

        let (ttl_1, ttl_2, ttl_3, ttl_4) = four_byte_split(self.ttl);
        ret.push(ttl_1);
        ret.push(ttl_2);
        ret.push(ttl_3);
        ret.push(ttl_4);

        push_split_bytes(&mut ret, self.rd_length);

        ret.append(&mut data_bytes);

        return ret
    }

    pub fn from_query(query: &DNSQuestion, name_offset: u16, data: Box<dyn RData>, ttl: Option<u32>) -> ResourceRecord
    {
        ResourceRecord {
            name_offset,
            answer_type: query.qtype,
            class: query.qclass,
            ttl: ttl.unwrap_or(0),
            rd_length: data.to_bytes().len() as u16,
            r_data: data
        }
    }
}

pub fn records_to_bytes(answers: &Vec<ResourceRecord>) -> Vec<u8>
{
    let mut ret = Vec::with_capacity(20);

    for a in answers
    {
        ret.append(&mut a.to_bytes());
    }

    ret
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum RecordParseError {
    ShortLength(usize),
    QTypeParse(u16),
    QClassParse(u16)
}

pub fn records_from_bytes(bytes: Vec<u8>, total_answers: u16) -> Result<(Vec<ResourceRecord>, Vec<u8>), RecordParseError>
{
    let mut ret = Vec::with_capacity(total_answers as usize);
    let mut remaining = vec![];

    let mut name_offset = (None, None);
    let mut qtype = (None, None);
    let mut qclass = (None, None);
    let mut ttl = (None, None, None, None);
    let mut data_length = (None, None);
    let mut data_length_combined = None;
    let mut data_length_remaining: u16 = 0;
    let mut data = Vec::with_capacity(8);

    for byte in bytes {
        if ret.len() == total_answers as usize {
            remaining.push(byte);
            continue;
        }

        if data_length_remaining != 0 || data_length_combined == Some(0) {

            if data_length_remaining != 0 {
                data.push(byte);
                data_length_remaining -= 1;
            }

            if data_length_remaining == 0 {

                match (two_byte_combine(qtype.0.unwrap(), qtype.1.unwrap()).try_into(),
                       two_byte_combine(qclass.0.unwrap(), qclass.1.unwrap()).try_into())
                {
                    (Ok(qtype_formed), Ok(qclass_formed)) => {
                        ret.push(ResourceRecord {
                            name_offset: two_byte_combine(name_offset.0.unwrap(), name_offset.1.unwrap()) & 0b11111111111111,
                            answer_type: qtype_formed,
                            class: qclass_formed,
                            ttl: four_byte_combine(ttl.0.unwrap(), ttl.1.unwrap(), ttl.2.unwrap(), ttl.3.unwrap()),
                            rd_length: data_length_combined.unwrap(),
                            r_data: Box::from(RawRData::from(data.clone()))
                        });

                        name_offset = (None, None);
                        qtype = (None, None);
                        qclass = (None, None);
                        ttl = (None, None, None, None);
                        data_length = (None, None);
                        data_length_combined = None;
                        data.clear();


                    }
                    (Err(qtype_e), _) => {
                        return Err(RecordParseError::QTypeParse(qtype_e));
                    }
                    (_, Err(qclass_e)) => {
                        return Err(RecordParseError::QClassParse(qclass_e));
                    }
                }
            }
        }
        else {
            match (name_offset, qtype, qclass, ttl, data_length) {
                ((None, _), // NAME OFFSET
                    _, _, _, _) => {
                    name_offset.0 = Some(byte);
                }
                ((Some(_), None), // NAME OFFSET
                    _, _, _, _) => {
                    name_offset.1 = Some(byte);
                }
                ((Some(_), Some(_)), // QTYPE
                    (None, _),
                    _, _, _) => {
                    qtype.0 = Some(byte);
                }
                ((Some(_), Some(_)), // QTYPE
                    (Some(_), None),
                    _, _, _) => {
                    qtype.1 = Some(byte);
                }
                ((Some(_), Some(_)), // QCLASS
                    (Some(_), Some(_)),
                    (None, _),
                    _, _) => {
                    qclass.0 = Some(byte);
                }
                ((Some(_), Some(_)), // QCLASS
                    (Some(_), Some(_)),
                    (Some(_), None),
                    _, _) => {
                    qclass.1 = Some(byte);
                }
                ((Some(_), Some(_)), // TTL
                    (Some(_), Some(_)),
                    (Some(_), Some(_)),
                    (None, _, _, _),
                    _) => {
                    ttl.0 = Some(byte);
                }
                ((Some(_), Some(_)), // TTL
                    (Some(_), Some(_)),
                    (Some(_), Some(_)),
                    (Some(_), None, _, _),
                    _) => {
                    ttl.1 = Some(byte);
                }
                ((Some(_), Some(_)), // TTL
                    (Some(_), Some(_)),
                    (Some(_), Some(_)),
                    (Some(_), Some(_), None, _),
                    _) => {
                    ttl.2 = Some(byte);
                }
                ((Some(_), Some(_)), // TTL
                    (Some(_), Some(_)),
                    (Some(_), Some(_)),
                    (Some(_), Some(_), Some(_), None),
                    _) => {
                    ttl.3 = Some(byte);
                }
                ((Some(_), Some(_)), // DATA LENGTH
                    (Some(_), Some(_)),
                    (Some(_), Some(_)),
                    (Some(_), Some(_), Some(_), Some(_)),
                    (None, _)) => {
                    data_length.0 = Some(byte);
                }
                ((Some(_), Some(_)), // DATA LENGTH
                    (Some(_), Some(_)),
                    (Some(_), Some(_)),
                    (Some(_), Some(_), Some(_), Some(_)),
                    (Some(data_length_1), None)) => {
                    data_length.1 = Some(byte);

                    data_length_combined = Some(two_byte_combine(data_length_1, byte));
                    data_length_remaining = data_length_combined.unwrap();
                }
                ((Some(_), Some(_)),
                    (Some(_), Some(_)),
                    (Some(_), Some(_)),
                    (Some(_), Some(_), Some(_), Some(_)),
                    (Some(_), Some(_))) => {


                }
            }
        }

    }

    Ok((ret, remaining))
}