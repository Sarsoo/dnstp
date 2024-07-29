use aes_gcm_siv::Aes256GcmSiv;
use p256::ecdh::EphemeralSecret;
use crate::crypto::{get_random_asym_pair, trim_public_key};
use crate::string::append_base_domain_to_key;

/// Represents the server from the perspective of a client
pub struct ClientCryptoContext {
    pub shared_key: Option<Aes256GcmSiv>,
    pub client_private: EphemeralSecret,
    pub client_public: String,
    pub server_public: Option<String>
}

impl ClientCryptoContext {
    pub fn new() -> Self {
        let (client_private, client_public) = get_random_asym_pair();

        Self {
            shared_key: None,
            client_private,
            client_public,
            server_public: None
        }
    }

    pub fn is_complete(&self) -> bool
    {
        self.server_public.is_some() && self.shared_key.is_some()
    }

    pub fn get_public_key_domain(&self, base_domain: &String) -> String
    {
        append_base_domain_to_key(
            trim_public_key(&self.client_public),
            base_domain
        )
    }
}

