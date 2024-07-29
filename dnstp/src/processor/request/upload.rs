use std::net::SocketAddr;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use aes_gcm_siv::Nonce;
use log::{info, error};
use base64::prelude::*;

// use std::fs::OpenOptions;
// use std::io::prelude::*;

use crate::session::clients::Clients;
use crate::crypto::decrypt;
use crate::message::DNSMessage;
use crate::net::NetworkMessagePtr;
use crate::processor::RequestProcesor;

impl RequestProcesor {
    pub fn handle_upload_request(r: DNSMessage, _sending_channel: &Sender<NetworkMessagePtr>, clients: &Arc<Mutex<Clients>>, peer: SocketAddr)
    {
        info!("[{}] received upload request", peer);
        let client_id = &r.questions[0].qname;

        if let Err(_) = clients.lock().unwrap().bump_last_seen(client_id) {
            error!("[{}] failed to bump last seen time", peer);
        }

        let encrypted_value = BASE64_STANDARD.decode(r.questions[1].qname.clone());
        let nonce_value = BASE64_STANDARD.decode(r.questions[2].qname.clone());

        match (encrypted_value, nonce_value) {
            (Ok(encrypted_value), Ok(nonce_value)) => {
                let nonce = Nonce::from_slice(nonce_value.as_slice());
                let decrypted = decrypt(clients.lock().unwrap().get_shared_key(client_id).unwrap(), nonce, &encrypted_value).unwrap();
                let decrypted_string = String::from_utf8(decrypted).unwrap();

                info!("[{}] decrypted [{}] from peer", peer, decrypted_string.as_str());

                // let mut file = OpenOptions::new()
                //     .read(true)
                //     .write(true)
                //     .append(true)
                //     .create(true)
                //     .open(client_id)
                //     .unwrap();
                // 
                // if let Err(e) = file.write(decrypted_string.as_bytes()) {
                //     error!("[{}] couldn't write to file: {}", peer, e);
                // }
                // if let Err(e) = file.write("\n".as_bytes()) {
                //     error!("[{}] couldn't write to file: {}", peer, e);
                // }
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