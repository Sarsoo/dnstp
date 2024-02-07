mod raw_rdata;
pub use raw_rdata::RawRData;

mod a_rdata;
pub use a_rdata::ARdata;

mod aaaa_rdata;
pub use aaaa_rdata::AAAARdata;


#[cfg(test)]
mod tests;

use std::fmt::Debug;
use std::fmt::Display;
use crate::byte::{four_byte_split, two_byte_split};
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
        let (name_1, name_2) = two_byte_split(self.name_offset | (0b11 << 14));
        let mut ret = vec![name_1, name_2];

        let type_split = two_byte_split(self.answer_type as u16);
        ret.push(type_split.0);
        ret.push(type_split.1);

        let class_split = two_byte_split(self.class as u16);
        ret.push(class_split.0);
        ret.push(class_split.1);

        let (ttl_1, ttl_2, ttl_3, ttl_4) = four_byte_split(self.ttl);
        ret.push(ttl_1);
        ret.push(ttl_2);
        ret.push(ttl_3);
        ret.push(ttl_4);

        let rd_length_split = two_byte_split(self.rd_length);
        ret.push(rd_length_split.0);
        ret.push(rd_length_split.1);

        ret.append(&mut self.r_data.to_bytes());

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
    QTypeParse(u8),
    QClassParse(u8)
}

pub fn answers_from_bytes(bytes: Vec<u8>, total_answers: u16) -> Result<(i32, Vec<ResourceRecord>), RecordParseError>
{
    Ok((0, vec![]))
}