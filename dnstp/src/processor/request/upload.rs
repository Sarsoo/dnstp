use std::net::SocketAddr;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use aes_gcm_siv::aead::consts::U12;
use aes_gcm_siv::aead::generic_array::GenericArray;
use aes_gcm_siv::Nonce;
use base64::DecodeError;
use log::{info, error};
use base64::prelude::*;

// use std::fs::OpenOptions;
// use std::io::prelude::*;

use crate::session::clients::Clients;
use crate::crypto::decrypt;
use crate::message::DNSMessage;
use crate::net::NetworkMessagePtr;
use crate::processor::RequestProcesor;

struct UploadValue {
    pub key: Option<String>,
    pub value: String,
    pub nonce: String
}

impl UploadValue {
    pub fn from(r: &DNSMessage) -> Option<UploadValue> {
        if r.questions.len() == 3 {
            return Some(UploadValue {
                key: None,
                value: r.questions[1].qname.clone(),
                nonce: r.questions[2].qname.clone()
            });
        }
        else if r.questions.len() == 4 {
            return Some(UploadValue {
                key: Some(r.questions[1].qname.clone()),
                value: r.questions[2].qname.clone(),
                nonce: r.questions[3].qname.clone()
            });
        }

        None
    }

    pub fn get_decoded_nonce(&self) -> Result<Vec<u8>, DecodeError>
    {
        BASE64_STANDARD.decode(&self.nonce)
    }

    pub fn get_decoded_encrypted_key(&self) -> Option<Vec<u8>>
    {
        if let Some(key) = &self.key {
            if let Ok(decode) = BASE64_STANDARD.decode(key) {
                return Some(decode);
            }
        }

        None
    }

    pub fn get_decoded_encrypted_value(&self) -> Result<Vec<u8>, DecodeError>
    {
        BASE64_STANDARD.decode(&self.value)
    }
}

impl RequestProcesor {
    pub fn handle_upload_request(r: DNSMessage, _sending_channel: &Sender<NetworkMessagePtr>, clients: &Arc<Mutex<Clients>>, peer: SocketAddr)
    {
        info!("[{}] received upload request", peer);
        let client_id = &r.questions[0].qname;

        if let Err(_) = clients.lock().unwrap().bump_last_seen(client_id) {
            error!("[{}] failed to bump last seen time", peer);
        }

        if let Some(value_context) = UploadValue::from(&r) {

            match (value_context.get_decoded_encrypted_value(), value_context.get_decoded_nonce()) {
                (Ok(encrypted_value), Ok(nonce_value)) => {
                    let nonce = Nonce::from_slice(nonce_value.as_slice());

                    let mut clients = clients.lock().unwrap();
                    let shared_key = clients.get_shared_key(client_id).unwrap();
                    let decrypted = decrypt(shared_key, nonce, &encrypted_value).unwrap();
                    let decrypted_string = String::from_utf8(decrypted).unwrap();

                    match value_context.get_decoded_encrypted_key() {
                        Some(encrypted_key) => {

                            let decrypted_key = decrypt(shared_key, nonce, &encrypted_key).unwrap();
                            let decrypted_key_string = String::from_utf8(decrypted_key).unwrap();

                            info!("[{}] decrypted [{}]:[{}] from peer", peer, decrypted_key_string.as_str(), decrypted_string.as_str());
                        }
                        None => {
                            info!("[{}] decrypted [{}] from peer", peer, decrypted_string.as_str());
                        }
                    }

                }
                (Err(e), _) => {
                    error!("[{}] failed to decode encrypted value from peer: {}", peer, e);
                }
                (_, Err(e)) => {
                    error!("[{}] failed to decode nonce from peer: {}", peer, e);
                }
            }
        }
    }
}