use super::*;

#[test]
fn test_encode()
{
    let payload = "google.com";

    let encoded = encode_domain_name(&payload.to_string());

    assert_eq!(encoded.len(), "google".len() + "com".len() + 1 + 1 + 1);
    assert_eq!(encoded[0], "google".len() as u8);
    // assert_eq!(encoded["google".len()], "com".len() as u8);
}

#[test]
fn test_encode_decode()
{
    let payload = "google.com";

    let encoded = encode_domain_name(&payload.to_string());
    let decoded = decode_domain_name(encoded);

    assert_eq!(payload, decoded);
}

#[test]
fn test_encode_decode_two()
{
    let payload = "sub.domain.com";

    let encoded = encode_domain_name(&payload.to_string());
    let decoded = decode_domain_name(encoded);

    assert_eq!(payload, decoded);
}