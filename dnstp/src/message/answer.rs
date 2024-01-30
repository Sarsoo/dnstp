use crate::byte::{four_byte_split, two_byte_split};
use crate::message::question::{QClass, QType};
use crate::string::encode_domain_name;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct DNSAnswer {
    pub name: String,
    pub answer_type: QType,
    pub class: QClass,
    pub ttl: u32,
    pub rd_length: u16,
    pub r_data: Vec<u8>
}

impl DNSAnswer {

    pub fn to_bytes(& self) -> Vec<u8>
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

        ret.append(&mut self.r_data.clone());

        return ret
    }
}