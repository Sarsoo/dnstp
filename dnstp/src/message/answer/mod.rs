mod raw_rdata;
pub use raw_rdata::RawRData;

mod ip_address;
pub use ip_address::IpRData;

#[cfg(test)]
mod tests;

use std::fmt::{Debug, Display};
use crate::byte::{four_byte_split, two_byte_split};
use crate::message::question::{DNSQuestion, QClass, QType, QuestionParseError};
use crate::string::encode_domain_name;


pub trait RData: Debug {
    fn to_bytes(&self) -> Vec<u8>;
}

#[derive(Debug)]
pub struct DNSAnswer {
    pub name: String,
    pub answer_type: QType,
    pub class: QClass,
    pub ttl: u32,
    pub rd_length: u16,
    pub r_data: Box<dyn RData>
}

impl DNSAnswer {

    pub fn to_bytes(&self) -> Vec<u8>
    {
        let mut ret = encode_domain_name(&self.name);

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

    pub fn from_query(query: &DNSQuestion, data: Box<dyn RData>, ttl: Option<u32>) -> DNSAnswer
    {
        DNSAnswer {
            name: query.qname.clone(),
            answer_type: query.qtype,
            class: query.qclass,
            ttl: ttl.unwrap_or(0),
            rd_length: data.to_bytes().len() as u16,
            r_data: data
        }
    }
}

pub fn answers_to_bytes(answers: &Vec<DNSAnswer>) -> Vec<u8>
{
    let mut ret = Vec::with_capacity(20);

    for a in answers
    {
        ret.append(&mut a.to_bytes());
    }

    ret
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum AnswerParseError {
    ShortLength(usize),
    QTypeParse(u8),
    QClassParse(u8)
}

pub fn answers_from_bytes(bytes: Vec<u8>, total_answers: u16) -> Result<(i32, Vec<DNSAnswer>), AnswerParseError>
{
    Ok((0, vec![]))
}