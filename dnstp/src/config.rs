//! Bundling config for the server and client

#[derive(Clone)]
pub struct DomainConfig {
    pub base_domain: String,
    pub key_endpoint: String,
}

impl DomainConfig {
    pub fn get_fq_key_endpoint(&self) -> String
    {
        vec![self.key_endpoint.clone(), self.base_domain.clone()].join(".")
    }
}