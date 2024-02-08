#[cfg(test)]
mod tests;

use urlencoding::decode;
use crate::byte::{push_split_bytes, two_byte_combine};
use crate::string::encode_domain_name;

#[repr(u16)]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub enum QType {
    A = 1,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    WKS = 11,
    PTR = 12,
    HINFO = 13,
    MINFO = 14,
    MX = 15,
    TXT = 16,
    RP = 17,
    AAAA = 28,
    SRV = 33,
    OPT = 41,
    ANY = 255,
}

impl TryFrom<u16> for QType {
    type Error = u16;

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == QType::A as u16 => Ok(QType::A),
            x if x == QType::NS as u16 => Ok(QType::NS),
            x if x == QType::CNAME as u16 => Ok(QType::CNAME),
            x if x == QType::SOA as u16 => Ok(QType::SOA),
            x if x == QType::WKS as u16 => Ok(QType::WKS),
            x if x == QType::PTR as u16 => Ok(QType::PTR),
            x if x == QType::HINFO as u16 => Ok(QType::HINFO),
            x if x == QType::MINFO as u16 => Ok(QType::MINFO),
            x if x == QType::MX as u16 => Ok(QType::MX),
            x if x == QType::TXT as u16 => Ok(QType::TXT),
            x if x == QType::RP as u16 => Ok(QType::RP),
            x if x == QType::AAAA as u16 => Ok(QType::AAAA),
            x if x == QType::SRV as u16 => Ok(QType::SRV),
            x if x == QType::ANY as u16 => Ok(QType::ANY),
            _ => Err(v),
        }
    }
}

#[repr(u16)]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub enum QClass {
    Internet = 1,
    Chaos = 3,
    Hesiod = 4,
}

impl TryFrom<u16> for QClass {
    type Error = u16;

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == QClass::Internet as u16 => Ok(QClass::Internet),
            x if x == QClass::Chaos as u16 => Ok(QClass::Chaos),
            x if x == QClass::Hesiod as u16 => Ok(QClass::Hesiod),
            _ => Err(v),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
pub struct DNSQuestion {
    pub qname: String,
    pub qtype: QType,
    pub qclass: QClass
}

impl DNSQuestion {
    pub fn new(qname: String, qtype: QType, qclass: QClass) -> DNSQuestion
    {
        DNSQuestion {
            qname,
            qtype,
            qclass
        }
    }

    pub fn to_bytes(&self) -> Vec<u8>
    {
        let mut ret = encode_domain_name(&self.qname);

        push_split_bytes(&mut ret, self.qtype as u16);
        push_split_bytes(&mut ret, self.qclass as u16);

        ret
    }
}

pub fn questions_to_bytes(questions: &Vec<DNSQuestion>) -> Vec<u8>
{
    let mut ret = Vec::with_capacity(20);

    for q in questions
    {
        ret.append(&mut q.to_bytes());
    }

    ret
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum QuestionParseError {
    ShortLength(usize),
    QTypeParse(u16),
    QClassParse(u16)
}

pub fn questions_from_bytes(bytes: Vec<u8>, total_questions: u16) -> Result<(Vec<DNSQuestion>, Vec<u8>), QuestionParseError>
{
    if bytes.len() < 4
    {
        return Err(QuestionParseError::ShortLength(bytes.len()));
    }

    let mut questions: Vec<DNSQuestion> = Vec::with_capacity(total_questions as usize);
    let mut remaining = vec![];

    let mut current_query: Vec<u8> = Vec::with_capacity(10);

    let mut current_length: Option<u8> = None;
    let mut remaining_length: u8 = 0;
    let mut current_qtype: (Option<u8>, Option<u8>) = (None, None);
    let mut current_qclass: (Option<u8>, Option<u8>) = (None, None);
    let mut trailers_reached = false;

    for byte in bytes {
        if questions.len() == total_questions as usize {
            remaining.push(byte);
            continue;
        }

        match current_length {
            None => { // next question, init lengths
                current_length = Some(byte);
                remaining_length = byte;
                current_query.clear();
            }
            Some(_) => {
                if byte == 0 && !trailers_reached {
                    trailers_reached = true;
                    continue
                }

                if remaining_length == 0 && !trailers_reached {
                    current_query.push('.' as u8);
                    current_length = Some(byte);
                    remaining_length = byte;
                }
                else if trailers_reached { // trailer fields
                    match (current_qtype, current_qclass) {
                        ((None, _), (_, _)) => {
                            current_qtype.0 = Some(byte);
                        },
                        ((_, None), (_, _)) => {
                            current_qtype.1 = Some(byte);
                        },
                        ((_, _), (None, _)) => {
                            current_qclass.0 = Some(byte);
                        }
                        ((Some(qtype_1), Some(qtype_2)), (Some(qclass_1), None)) => {
                            match (two_byte_combine(qtype_1, qtype_2).try_into(),
                                   two_byte_combine(qclass_1, byte).try_into()) {
                                (Ok(qtype), Ok(qclass)) => {
                                    questions.push(DNSQuestion {
                                        qname: decode(String::from_utf8(current_query.clone()).unwrap().as_str()).unwrap().to_string(),
                                        qtype,
                                        qclass
                                    });

                                    current_length = None;
                                    remaining_length = byte;
                                    current_query.clear();
                                    current_qtype = (None, None);
                                    current_qclass = (None, None);
                                    trailers_reached = false;
                                }
                                (Err(qtype_e), _) => {
                                    return Err(QuestionParseError::QTypeParse(qtype_e));
                                }
                                (_, Err(qclass_e)) => {
                                    return Err(QuestionParseError::QClassParse(qclass_e));
                                }
                            }
                        }
                        _ => {}
                    }
                }
                else {
                    current_query.push(byte);
                    remaining_length -= 1;
                }
            }
        }
    }

    Ok((questions, remaining))
}