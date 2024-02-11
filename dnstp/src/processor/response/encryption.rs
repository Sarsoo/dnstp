use std::sync::{Arc, Mutex};
use crate::client_crypto_context::ClientCryptoContext;
use crate::crypto::{asym_to_sym_key, get_shared_asym_secret};
use crate::message::DNSMessage;
use crate::processor::request::encryption::get_fattened_public_key;
use crate::string::decode_domain_name;

pub fn decode_key_response(message: &DNSMessage, client_crypto_context: Arc<Mutex<ClientCryptoContext>>)
{
    if message.answer_records.len() == 2 {
        // if message.questions[0].qtype != QType::A
        // {
        //     return Err(KeyDecodeError::FirstQuestionNotA(message.questions[0].qtype));
        // }

        let key_answer = &message.answer_records[1];

        // if key_answer.answer_type != QType::A
        // {
        //     return Err(KeyDecodeError::SecondQuestionNotA(key_answer.answer_type));
        // }

        let data_string = decode_domain_name(key_answer.r_data.to_bytes());
        // key is transmitted wihout --- BEGIN KEY -- header and trailer bits and with '.' instead of new lines
        let (fattened_public_key, _) = get_fattened_public_key(&data_string);

        let mut context = client_crypto_context.lock().unwrap();

        match get_shared_asym_secret(&context.client_private, &fattened_public_key)
        {
            Ok(k) => {
                context.server_public = Some(fattened_public_key);
                context.shared_key = Some(asym_to_sym_key(&k));
            }
            Err(_) => {}
        }
    }
}