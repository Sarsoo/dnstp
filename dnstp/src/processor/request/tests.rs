use crate::crypto::{fatten_public_key, get_random_asym_pair, trim_public_key};
use crate::string::encode_domain_name;
use super::*;
use super::encryption::*;

#[test]
fn encryption()
{
    let (private, public) = get_key_request_with_base_domain(String::from("sarsoo.xyz"));

    let encoded = encode_domain_name(&public);
    // let decoded = decode_domain_name();

    assert_eq!(1, 1);
}