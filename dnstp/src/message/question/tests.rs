use super::*;

#[test]
fn one_question_back_and_forth() {
    let q = DNSQuestion {
        qname: "google.com".to_string(),
        qclass: QClass::Internet,
        qtype: QType::A
    };

    let mut q_bytes = q.to_bytes();
    q_bytes.append(&mut vec![0, 0, 0, 0, 0, 0]);

    let (q_read, q_reconstructed) = questions_from_bytes(q_bytes, 1).unwrap();

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

    let (q_read, q_reconstructed) = questions_from_bytes(q_bytes, 2).unwrap();

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

    let (q_read, q_reconstructed) = questions_from_bytes(q_bytes, 3).unwrap();

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