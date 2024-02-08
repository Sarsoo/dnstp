use crate::message::question::{DNSQuestion, QClass, QType, questions_from_bytes};

use super::*;

#[test]
fn one_answer_back_and_forth() {
    let q = ResourceRecord {
        // name_offset: "google.com".to_string(),
        name_offset: 12,
        answer_type: QType::A,
        class: QClass::Internet,
        ttl: 0,
        rd_length: 1,
        r_data: Box::from(RawRData::from(vec![1]))
    };

    let mut q_bytes = q.to_bytes();
    q_bytes.append(&mut vec![0, 0, 0, 0, 0, 0]);

    let (q_reconstructed, q_remaining) = records_from_bytes(q_bytes, 1).unwrap();

    assert_eq!(q.name_offset, q_reconstructed[0].name_offset);
    assert_eq!(q.answer_type, q_reconstructed[0].answer_type);
    assert_eq!(q.class, q_reconstructed[0].class);
    assert_eq!(q.ttl, q_reconstructed[0].ttl);
    assert_eq!(q.rd_length, q_reconstructed[0].rd_length);
    assert_eq!(q.r_data.to_bytes(), q_reconstructed[0].r_data.to_bytes());
}

#[test]
fn two_answers_back_and_forth() {
    let q = ResourceRecord {
        // name_offset: "google.com".to_string(),
        name_offset: 12,
        answer_type: QType::A,
        class: QClass::Internet,
        ttl: 0,
        rd_length: 1,
        r_data: Box::from(RawRData::from(vec![1]))
    };

    let q_2 = ResourceRecord {
        // name_offset: "google.com".to_string(),
        name_offset: 12,
        answer_type: QType::AAAA,
        class: QClass::Internet,
        ttl: 0,
        rd_length: 3,
        r_data: Box::from(RawRData::from(vec![1, 2, 3]))
    };

    let mut q_bytes = q.to_bytes();
    q_bytes.append(&mut q_2.to_bytes());
    q_bytes.append(&mut vec![0, 0, 0, 0, 0, 0]);

    let (q_reconstructed, q_remaining) = records_from_bytes(q_bytes, 2).unwrap();

    assert_eq!(q.name_offset, q_reconstructed[0].name_offset);
    assert_eq!(q.answer_type, q_reconstructed[0].answer_type);
    assert_eq!(q.class, q_reconstructed[0].class);
    assert_eq!(q.ttl, q_reconstructed[0].ttl);
    assert_eq!(q.rd_length, q_reconstructed[0].rd_length);
    assert_eq!(q.r_data.to_bytes(), q_reconstructed[0].r_data.to_bytes());

    assert_eq!(q_2.name_offset, q_reconstructed[1].name_offset);
    assert_eq!(q_2.answer_type, q_reconstructed[1].answer_type);
    assert_eq!(q_2.class, q_reconstructed[1].class);
    assert_eq!(q_2.ttl, q_reconstructed[1].ttl);
    assert_eq!(q_2.rd_length, q_reconstructed[1].rd_length);
    assert_eq!(q_2.r_data.to_bytes(), q_reconstructed[1].r_data.to_bytes());
}