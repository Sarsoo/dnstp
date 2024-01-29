use std::ops::Sub;
use urlencoding::{encode, decode};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
enum QType {
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
    type Error = ();

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
            _ => Err(()),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
enum QClass {
    Internet = 1,
    Chaos = 3,
    Hesiod = 4,
}

impl TryFrom<u8> for QClass {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == QClass::Internet as u8 => Ok(QClass::Internet),
            x if x == QClass::Chaos as u8 => Ok(QClass::Chaos),
            x if x == QClass::Hesiod as u8 => Ok(QClass::Hesiod),
            _ => Err(()),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct DNSQuestion {
    qname: String,
    qtype: QType,
    qclass: QClass
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
        let mut ret: Vec<u8> = Vec::with_capacity(self.qname.len() + 2 + 3);

        for part in  self.qname.split(".")
        {
            let encoded_string = encode(part);
            let count = encoded_string.len();

            ret.push(count as u8);
            for x in encoded_string.bytes() {
                ret.push(x);
            };
        }

        ret.push(0);

        ret.push(self.qtype as u8);
        ret.push(self.qclass as u8);

        ret
    }
}

pub fn questions_from_bytes(bytes: Vec<u8>, total_questions: u8) -> Result<Vec<DNSQuestion>, ()>
{
    if (bytes.len() < 4)
    {
        return Err(());
    }

    let mut questions: Vec<DNSQuestion> = Vec::with_capacity(total_questions as usize);
    let mut current_query: Option<Vec<u8>> = None;

    let mut current_length: Option<u8> = None;
    let mut remaining_length: Box<u8> = Box::from(0);
    let mut current_qtype: Option<u8> = None;
    let mut current_qclass: Option<u8> = None;
    let mut trailers_reached = false;

    for byte in bytes {
        match current_length {
            None => { // next question, init lengths
                current_length = Some(byte);
                remaining_length = Box::from(byte);
                current_query = Some(Vec::with_capacity(10));
            }
            Some(_) => {
                if byte == 0 {
                    trailers_reached = true;
                    continue
                }

                if *remaining_length == 0 && !trailers_reached {
                    current_query.as_mut().unwrap().push('.' as u8);
                    current_length = Some(byte);
                    remaining_length = Box::from(byte);
                }
                else if trailers_reached { // trailer fields
                    match current_qtype {
                        None => {
                            current_qtype = Some(byte);
                        }
                        Some(qtype_b) => {
                            match current_qclass {
                                None => {
                                    current_qclass = Some(byte);
                                }
                                Some(qclass_b) => {

                                    match (qtype_b.try_into(), qclass_b.try_into()) {
                                        (Ok(qtype), Ok(qclass)) => {
                                            questions.push(DNSQuestion {
                                                qname: String::from_utf8(current_query.unwrap()).unwrap(),
                                                qtype,
                                                qclass
                                            });

                                            current_length = Some(byte);
                                            remaining_length = Box::from(byte);
                                            current_query = Some(Vec::with_capacity(10));
                                            current_qtype = None;
                                            current_qclass = None;
                                            trailers_reached = false;
                                        }
                                        _ => {
                                            return Err(());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                else
                {
                    current_query.as_mut().unwrap().push(byte);
                    *remaining_length  = remaining_length.sub(1);
                }
            }
        }
    }

    match (current_qtype, current_qclass) {
        (Some(qtype), Some(qclass)) => {
            match (qtype.try_into(), qclass.try_into()) {
                (Ok(qtype), Ok(qclass)) => {
                    questions.push(DNSQuestion {
                        qname: String::from_utf8(current_query.unwrap()).unwrap(),
                        qtype,
                        qclass
                    });
                }
                _ => {
                    return Err(());
                }
            }
        }
        _ => {
            return Err(());
        }
    }

    Ok(questions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_question_back_and_forth() {
        let q = DNSQuestion {
            qname: "google.com".to_string(),
            qclass: QClass::Internet,
            qtype: QType::A
        };

        let q_bytes = q.to_bytes();

        let q_reconstructed = questions_from_bytes(q_bytes, 1).unwrap();

        assert_eq!(q.qname, q_reconstructed[0].qname);
        assert_eq!(q.qclass, q_reconstructed[0].qclass);
        assert_eq!(q.qtype, q_reconstructed[0].qtype);
    }

    #[test]
    fn two_questions_back_and_forth() {
        let q = DNSQuestion {
            qname: "google.com".to_string(),
            qclass: QClass::Internet,
            qtype: QType::A
        };

        let q2 = DNSQuestion {
            qname: "duck.com".to_string(),
            qclass: QClass::Internet,
            qtype: QType::AAAA
        };

        let mut q_bytes = q.to_bytes();
        let mut q2_bytes = q2.to_bytes();

        q_bytes.append(&mut q2_bytes);

        let q_reconstructed = questions_from_bytes(q_bytes, 2).unwrap();

        assert_eq!(q.qname, q_reconstructed[0].qname);
        assert_eq!(q.qclass, q_reconstructed[0].qclass);
        assert_eq!(q.qtype, q_reconstructed[0].qtype);

        assert_eq!(q2.qname, q_reconstructed[1].qname);
        assert_eq!(q2.qclass, q_reconstructed[1].qclass);
        assert_eq!(q2.qtype, q_reconstructed[1].qtype);
    }

    #[test]
    fn three_questions_back_and_forth() {
        let q = DNSQuestion {
            qname: "google.com".to_string(),
            qclass: QClass::Internet,
            qtype: QType::A
        };

        let q2 = DNSQuestion {
            qname: "duck.com".to_string(),
            qclass: QClass::Internet,
            qtype: QType::AAAA
        };

        let q3 = DNSQuestion {
            qname: "facebook.com".to_string(),
            qclass: QClass::Hesiod,
            qtype: QType::CNAME
        };

        let mut q_bytes = q.to_bytes();
        let mut q2_bytes = q2.to_bytes();
        let mut q3_bytes = q3.to_bytes();

        q_bytes.append(&mut q2_bytes);
        q_bytes.append(&mut q3_bytes);

        let q_reconstructed = questions_from_bytes(q_bytes, 2).unwrap();

        assert_eq!(q.qname, q_reconstructed[0].qname);
        assert_eq!(q.qclass, q_reconstructed[0].qclass);
        assert_eq!(q.qtype, q_reconstructed[0].qtype);

        assert_eq!(q2.qname, q_reconstructed[1].qname);
        assert_eq!(q2.qclass, q_reconstructed[1].qclass);
        assert_eq!(q2.qtype, q_reconstructed[1].qtype);

        assert_eq!(q3.qname, q_reconstructed[2].qname);
        assert_eq!(q3.qclass, q_reconstructed[2].qclass);
        assert_eq!(q3.qtype, q_reconstructed[2].qtype);
    }
}