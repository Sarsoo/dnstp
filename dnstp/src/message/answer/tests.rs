use crate::message::question::{DNSQuestion, QClass, QType, questions_from_bytes};

use super::*;

#[test]
#[ignore]
fn one_answer_back_and_forth() {
    let q = ResourceRecord {
        // name_offset: "google.com".to_string(),
        name_offset: 12,
        answer_type: QType::A,
        class: QClass::Internet,
        ttl: 0,
        rd_length: 0,
        r_data: Box::from(RawRData::from(vec![]))
    };

    let mut q_bytes = q.to_bytes();
    q_bytes.append(&mut vec![0, 0, 0, 0, 0, 0]);

    let (q_read, q_reconstructed) = answers_from_bytes(q_bytes, 0).unwrap();

    assert_eq!(q.name_offset, q_reconstructed[0].name_offset);
    assert_eq!(q.answer_type, q_reconstructed[0].answer_type);
    assert_eq!(q.class, q_reconstructed[0].class);
    assert_eq!(q.ttl, q_reconstructed[0].ttl);
    assert_eq!(q.rd_length, q_reconstructed[0].rd_length);
}