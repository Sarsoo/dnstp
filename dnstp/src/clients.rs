//! Structures for managing the state of connected clients from the perspective of the server

use std::collections::HashMap;
use std::time::SystemTime;
use aes_gcm_siv::Aes256GcmSiv;

/// A single client including when they connected and their shared cryptographic key
pub struct Client {
    pub first_seen: SystemTime,
    pub last_seen: SystemTime,
    pub shared_key: Aes256GcmSiv
}

impl Client {

    /// Create a new client as
    pub fn new(shared_key: Aes256GcmSiv) -> Client
    {
        let time = SystemTime::now();

        Client {
            first_seen: time,
            last_seen: time,
            shared_key
        }
    }

    pub fn bump_last_seen(&mut self)
    {
        self.last_seen = SystemTime::now();
    }
}

/// Container for managing connected clients and their keys
pub struct Clients {
    client_map: HashMap<String, Client>
}

impl Clients {

    /// Create a new collection of clients
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

    /// Add a newly connected client to the collection of connections. Index the client by public key.
    pub fn add(&mut self, client_id: String, client:Client)
    {
        self.client_map.insert(client_id, client);
    }

    pub fn client_is_connected(&self, client_id: &String) -> bool
    {
        self.client_map.contains_key(client_id)
    }

    pub fn bump_last_seen(&mut self, client_id: &String) -> Result<(), ()>
    {
        match self.client_map.get_mut(client_id)
        {
            None => Err(()),
            Some(client) => {
                client.bump_last_seen();
                Ok(())
            }
        }
    }
}