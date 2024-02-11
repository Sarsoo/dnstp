use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use dnstplib::crypto::{asym_to_sym_key, decrypt, encrypt, generate_aes_nonce, get_random_asym_pair, get_shared_asym_secret, trim_public_key};
use dnstplib::message::{DNSHeader, DNSMessage, DNSQuestion};
use dnstplib::message::QClass::Internet;
use dnstplib::message::QType::A;
use dnstplib::processor::request::encryption::decode_key_request;
use dnstplib::string::append_base_domain_to_key;
#[test]
fn test_key_swap()
{
    ////////////
    // CLIENT
    ////////////

    // generate pair
    let (client_private, client_public) = get_random_asym_pair();

    // generate public key submission domain
    let serialised_client_public = append_base_domain_to_key(
        trim_public_key(&client_public),
        &"sarsoo.xyz".to_string()
    );

    let message = DNSMessage {
        header: DNSHeader::new_request(1, Some(1)),
        questions: vec![
            DNSQuestion {
                qname: "static.sarsoo.xyz".to_string(),
                qtype: A,
                qclass: Internet
            },
            DNSQuestion {
                qname: serialised_client_public,
                qtype: A,
                qclass: Internet
            }
        ],
        answer_records: vec![],
        authority_records: vec![],
        additional_records: vec![],
        peer: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::from([127,0,0,1]), 5000)),
    };

    /////////////////
    //   SERVER
    /////////////////

    let question_count = message.questions.len();
    // handle message "received by client"
    let resp = decode_key_request(&message).unwrap();

    assert_eq!(question_count, resp.response.questions.len());
    assert_eq!(question_count, resp.response.answer_records.len());

    ////////////
    // CLIENT
    ////////////

    // client has received message from above and constructs shared secret
    let shared_secret_client = asym_to_sym_key(&get_shared_asym_secret(&client_private, &resp.server_public).unwrap());

    ///////////////////////////////
    // TEST ENCRYPTION/DECRYPTION
    ///////////////////////////////

    let nonce = generate_aes_nonce();
    let payload = "hello world!".to_string();

    // CLIENT encrypts something
    let encrypted = encrypt(&shared_secret_client, &nonce, &payload.clone().into_bytes()).unwrap();

    // SERVER decrypts it
    let decrypted = decrypt(&resp.new_client.shared_key, &nonce, &encrypted).unwrap();

    let decrypted_payload = String::from_utf8(decrypted).unwrap();

    // is it the same?
    assert_eq!(payload, decrypted_payload);
}