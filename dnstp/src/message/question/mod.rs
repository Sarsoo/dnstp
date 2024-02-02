#[cfg(test)]
mod tests;

use urlencoding::decode;
use crate::string::encode_domain_name;

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
    SRV = 33
}

impl TryFrom<u8> for QType {
    type Error = u8;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == QType::A as u8 => Ok(QType::A),
            x if x == QType::NS as u8 => Ok(QType::NS),
            x if x == QType::CNAME as u8 => Ok(QType::CNAME),
            x if x == QType::SOA as u8 => Ok(QType::SOA),
            x if x == QType::WKS as u8 => Ok(QType::WKS),
            x if x == QType::PTR as u8 => Ok(QType::PTR),
            x if x == QType::HINFO as u8 => Ok(QType::HINFO),
            x if x == QType::MINFO as u8 => Ok(QType::MINFO),
            x if x == QType::MX as u8 => Ok(QType::MX),
            x if x == QType::TXT as u8 => Ok(QType::TXT),
            x if x == QType::RP as u8 => Ok(QType::RP),
            x if x == QType::AAAA as u8 => Ok(QType::AAAA),
            x if x == QType::SRV as u8 => Ok(QType::SRV),
            _ => Err(v),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub enum QClass {
    Internet = 1,
    Chaos = 3,
    Hesiod = 4,
}

impl TryFrom<u8> for QClass {
    type Error = u8;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == QClass::Internet as u8 => Ok(QClass::Internet),
            x if x == QClass::Chaos as u8 => Ok(QClass::Chaos),
            x if x == QClass::Hesiod as u8 => Ok(QClass::Hesiod),
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

        ret.push(self.qtype as u8);
        ret.push(self.qclass as u8);

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
    QTypeParse(u8),
    QClassParse(u8)
}

pub fn questions_from_bytes(bytes: Vec<u8>, total_questions: u16) -> Result<(i32, Vec<DNSQuestion>), QuestionParseError>
{
    if bytes.len() < 4
    {
        return Err(QuestionParseError::ShortLength(bytes.len()));
    }

    let mut questions: Vec<DNSQuestion> = Vec::with_capacity(total_questions as usize);
    let mut current_query: Vec<u8> = Vec::with_capacity(10);

    let mut current_length: Option<u8> = None;
    let mut remaining_length: u8 = 0;
    let mut current_qtype: Option<u8> = None;
    let mut trailers_reached = false;

    let mut byte_counter  = 0;

    for byte in bytes {
        byte_counter += 1;
        match current_length {
            None => { // next question, init lengths
                current_length = Some(byte);
                remaining_length = byte;
                current_query.clear();
            }
            Some(_) => {
                if byte == 0 {
                    trailers_reached = true;
                    continue
                }

                if remaining_length == 0 && !trailers_reached {
                    current_query.push('.' as u8);
                    current_length = Some(byte);
                    remaining_length = byte;
                }
                else if trailers_reached { // trailer fields
                    match current_qtype {
                        None => {
                            current_qtype = Some(byte);
                        }
                        Some(qtype_b) => {
                            match (qtype_b.try_into(), byte.try_into()) {
                                (Ok(qtype), Ok(qclass)) => {
                                    questions.push(DNSQuestion {
                                        qname: decode(String::from_utf8(current_query.clone()).unwrap().as_str()).unwrap().to_string(),
                                        qtype,
                                        qclass
                                    });

                                    if questions.len() == total_questions as usize {
                                        break
                                    }

                                    current_length = None;
                                    remaining_length = byte;
                                    current_query.clear();
                                    current_qtype = None;
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
                    }
                }
                else {
                    current_query.push(byte);
                    remaining_length -= 1;
                }
            }
        }
    }

    Ok((byte_counter, questions))
}