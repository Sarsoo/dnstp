//! Bundling config for the server and client

#[derive(Clone)]
pub struct DomainConfig {
    pub base_domain: String,
    pub key_endpoint: String,
}