use super::*;

macro_rules! assert_questions_eq {
    ($left:expr, $right:expr) => {
        assert_eq!($left.qname, $right.qname);
        assert_eq!($left.qclass, $right.qclass);
        assert_eq!($left.qtype, $right.qtype);
    };
}

#[test]
fn one_question_back_and_forth() {
    let q = DNSQuestion {
        qname: "google.com".to_string(),
        qclass: QClass::Internet,
        qtype: QType::A
    };

    let mut q_bytes = q.to_bytes();
    q_bytes.append(&mut vec![0, 0, 0, 0, 0, 0]);

    let (q_reconstructed, q_remaining) = questions_from_bytes(q_bytes, 1).unwrap();

    assert_questions_eq!(q, q_reconstructed[0]);
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

    let (q_reconstructed, q_remaining) = questions_from_bytes(q_bytes, 2).unwrap();

    assert_questions_eq!(q, q_reconstructed[0]);
    assert_questions_eq!(q2, q_reconstructed[1]);
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

    let (q_reconstructed, q_remaining) = questions_from_bytes(q_bytes, 3).unwrap();

    assert_questions_eq!(q, q_reconstructed[0]);
    assert_questions_eq!(q2, q_reconstructed[1]);
    assert_questions_eq!(q3, q_reconstructed[2]);
}