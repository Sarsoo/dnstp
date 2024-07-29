use std::sync::{Arc, Mutex};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use rand_core::{OsRng, RngCore};
use crate::crypto::{encrypt, generate_aes_nonce};
use crate::DomainConfig;
use crate::message::{Direction, DNSHeader, DNSMessage, DNSQuestion, Opcode, QClass, QType, ResponseCode};
use crate::session::ClientCryptoContext;

pub fn generate_client_handshake_message(rand: &mut OsRng, domain_config: &DomainConfig, crypto_context: Arc<Mutex<ClientCryptoContext>>, peer: &String) -> DNSMessage {
    get_client_handshake_message(
        rand.next_u32() as u16,
        domain_config.get_fq_key_endpoint(),
        crypto_context.lock().unwrap().get_public_key_domain(&domain_config.base_domain),
        peer
    )
}

pub fn get_client_handshake_message(msg_id: u16, key_domain: String, public_key_domain: String, peer: &String) -> DNSMessage {
    DNSMessage {
        header: DNSHeader {
            id: msg_id,
            direction: Direction::Request,
            opcode: Opcode::Query,
            authoritative: false,
            truncation: false,
            recursion_desired: false,
            recursion_available: false,
            valid_zeroes: true,
            response: ResponseCode::NoError,
            question_count: 2,
            answer_record_count: 0,
            authority_record_count: 0,
            additional_record_count: 0,
        },
        questions: vec![
            DNSQuestion {
                qname: key_domain,
                qtype: QType::A,
                qclass: QClass::Internet,
            },
            DNSQuestion {
                qname: public_key_domain,
                qtype: QType::A,
                qclass: QClass::Internet,
            }
        ],
        answer_records: vec![],
        authority_records: vec![],
        additional_records: vec![],
        peer: peer.parse().unwrap(),
    }
}

pub fn generate_string_encryption_message(value: String, rand: &mut OsRng, domain_config: &DomainConfig, crypto_context: Arc<Mutex<ClientCryptoContext>>, peer: &String) -> Result<DNSMessage, ()> {

    let nonce = generate_aes_nonce();
    let encrypted = encrypt(&crypto_context.lock().unwrap().shared_key.clone().unwrap(), &nonce, &value.clone().into_bytes());

    if let Ok(e) = encrypted {
        let encrypted_string = BASE64_STANDARD.encode(e);
        let nonce_string = BASE64_STANDARD.encode(nonce);
        
        return Ok(get_string_encryption_message(
            rand.next_u32() as u16,
            crypto_context.lock().unwrap().get_public_key_domain(&domain_config.base_domain),
            encrypted_string,
            nonce_string,
            peer
        ))
    }
    
    Err(())
}

pub fn get_string_encryption_message(msg_id: u16, public_key_domain: String, encrypted_string: String, nonce_string: String, peer: &String) -> DNSMessage {
    DNSMessage {
        header: DNSHeader {
            id: msg_id,
            direction: Direction::Request,
            opcode: Opcode::Query,
            authoritative: false,
            truncation: false,
            recursion_desired: false,
            recursion_available: false,
            valid_zeroes: true,
            response: ResponseCode::NoError,
            question_count: 3,
            answer_record_count: 0,
            authority_record_count: 0,
            additional_record_count: 0,
        },
        questions: vec![
            DNSQuestion {
                qname: public_key_domain,
                qtype: QType::A,
                qclass: QClass::Internet,
            },
            DNSQuestion {
                qname: encrypted_string,
                qtype: QType::A,
                qclass: QClass::Internet,
            },
            DNSQuestion {
                qname: nonce_string,
                qtype: QType::A,
                qclass: QClass::Internet,
            }
        ],
        answer_records: vec![],
        authority_records: vec![],
        additional_records: vec![],
        peer: peer.parse().unwrap(),
    }
}