use std::net::Ipv4Addr;
use p256::ecdh::EphemeralSecret;
use crate::crypto::{get_random_asym_pair, trim_public_key};
use crate::message::DNSMessage;

pub fn get_key_response(request: DNSMessage) -> DNSMessage
{
    DNSMessage::a_resp_from_request(&request, |_| Ipv4Addr::from([127, 0, 0, 1]))
}

pub fn get_key_request_with_base_domain(base_domain: String) -> (EphemeralSecret, String)
{
    let (private, public) = get_random_asym_pair();

    (private, vec![trim_public_key(&public), base_domain].join("."))
}
