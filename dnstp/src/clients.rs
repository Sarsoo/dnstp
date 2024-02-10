use std::collections::HashMap;
use std::time::SystemTime;
use aes_gcm_siv::Aes256GcmSiv;

pub struct Client {
    pub first_seen: SystemTime,
    pub shared_key: Aes256GcmSiv
}

impl Client {

    pub fn new(shared_key: Aes256GcmSiv) -> Client
    {
        Client {
            first_seen: SystemTime::now(),
            shared_key
        }
    }
}

pub struct Clients {
    client_map: HashMap<String, Client>
}

impl Clients {

    pub fn new() -> Clients
    {
        Clients {
            client_map: HashMap::new()
        }
    }

    // pub fn add_from(&mut self, client_id: String, shared_key: Aes256GcmSiv)
    // {
    //     self.client_map.insert(client_id, Client::new(shared_key));
    // }

    pub fn add(&mut self, client_id: String, client:Client)
    {
        self.client_map.insert(client_id, client);
    }
}