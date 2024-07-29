use crate::string::encode_domain_name;
// use super::*;
use super::encryption::*;

#[test]
fn encryption()
{
    let (_private, public) = get_key_request_with_base_domain(String::from("sarsoo.xyz"));

    let _encoded = encode_domain_name(&public);
    // let decoded = decode_domain_name();

    assert_eq!(1, 1);
}