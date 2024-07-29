use std::net::SocketAddr;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use log::{error, info};
use crate::session::clients::Clients;
use crate::message::DNSMessage;
use crate::net::NetworkMessagePtr;
use crate::processor::RequestProcesor;

impl RequestProcesor {
    pub fn handle_download_request(r: DNSMessage, _sending_channel: &Sender<NetworkMessagePtr>, clients: &Arc<Mutex<Clients>>, peer: SocketAddr)
    {
        info!("[{}] received download request", peer);
        let client_id = &r.questions[0].qname;

        if let Err(_) = clients.lock().unwrap().bump_last_seen(client_id) {
            error!("[{}] failed to bump last seen time", peer);
        }
    }
}