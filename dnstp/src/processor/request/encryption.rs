use std::net::Ipv4Addr;
use p256::ecdh::EphemeralSecret;
use crate::clients::Client;
use crate::crypto::{asym_to_sym_key, fatten_public_key, get_random_asym_pair, get_shared_asym_secret, trim_public_key};
use crate::message::{ARdata, DNSMessage, QClass, QType, ResourceRecord};
use crate::message::record::CnameRdata;
use crate::string::{append_base_domain_to_key, encode_domain_name, strip_base_domain_from_key};

/// Result of a client's handshake request including server key pair and prepared response
pub struct KeySwapContext {
    /// New client structure to track derived shared secret and last seen time
    pub new_client: Client,
    /// Response message to send to the client with the server's public key
    pub response: DNSMessage,
    /// Public key of the server's key pair
    pub server_public: String,
    /// Public key extracted from the client's request
    pub client_public: String
}

/// Generate a random asymmetric key pair, append the dnstp base domain to the secret
pub fn get_key_request_with_base_domain(base_domain: String) -> (EphemeralSecret, String)
{
    let (private, public) = get_random_asym_pair();

    (private, append_base_domain_to_key(trim_public_key(&public), &base_domain))
}

/// Extract the client's public key from the DNS message, turn the hostname back into the full fat public key with --- BEGIN KEY --- headers and trailers
pub fn get_fattened_public_key(key_question: &String) -> (String, String)
{
    let public_key = key_question;
    let (trimmed_public_key, base_domain) = strip_base_domain_from_key(public_key);

    (fatten_public_key(&trimmed_public_key), base_domain)
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub enum KeyDecodeError {
    QuestionCount(usize),
    FirstQuestionNotA(QType),
    SecondQuestionNotA(QType),
    SharedSecretDerivation,
}

/// Take a client's handshake request, process the crypto and prepare a response
///
/// Includes generating a server key pair, using the public key in the response, deriving the shared secret.
pub fn decode_key_request(message: &DNSMessage) -> Result<KeySwapContext, KeyDecodeError>
{
    if message.questions.len() == 2 {

        if message.questions[0].qtype != QType::A
        {
            return Err(KeyDecodeError::FirstQuestionNotA(message.questions[0].qtype));
        }

        let key_question = &message.questions[1];

        if key_question.qtype != QType::A
        {
            return Err(KeyDecodeError::SecondQuestionNotA(key_question.qtype));
        }

        // key is transmitted wihout --- BEGIN KEY -- header and trailer bits and with '.' instead of new lines
        let (fattened_public_key, base_domain) = get_fattened_public_key(&key_question.qname);
        // generate the servers public/private key pair
        let (server_private, server_public) = get_random_asym_pair();

        // do the Diffie-Hellman shared secret derivation for the server side symmetric encryption
        match get_shared_asym_secret(&server_private, &fattened_public_key) {
            Ok(secret) => {

                let sym_key = asym_to_sym_key(&secret);
                let new_client = Client::new(sym_key);
                let mut response = message.empty_resp_from_request();

                // return an empty null response for the key hostname (static.BLANK.TLD) question, not that important
                let first_record = ResourceRecord {
                    name_offset: 12,
                    answer_type: QType::A,
                    class: QClass::Internet,
                    ttl: 0,
                    rd_length: 4,
                    r_data: Box::new(ARdata::from(Ipv4Addr::from([127,0,0,1])))
                };

                let server_public_domain = append_base_domain_to_key(
                    trim_public_key(&server_public),
                    &base_domain
                );
                // for the public key question that the client sent, respond with the server's public key
                let second_record = ResourceRecord {
                    name_offset: 12 + (&message.questions[0]).to_bytes().len() as u16,
                    answer_type: QType::CNAME,
                    class: QClass::Internet,
                    ttl: 0,
                    rd_length: encode_domain_name(&server_public_domain).len() as u16,
                    r_data: Box::new(
                        CnameRdata::from(
                            server_public_domain
                        )
                    )
                };

                response.header.answer_record_count = 2;
                response.answer_records = vec![
                    first_record, second_record
                ];

                return Ok(KeySwapContext {
                    new_client,
                    response,
                    server_public,
                    client_public: key_question.qname.to_string()
                });
            }
            Err(_) => {
                return Err(KeyDecodeError::SharedSecretDerivation);
            }
        }
    }
    else
    {
        return Err(KeyDecodeError::QuestionCount(message.questions.len()));
    }
}
