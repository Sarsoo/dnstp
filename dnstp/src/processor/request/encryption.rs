use std::net::Ipv4Addr;
use p256::ecdh::EphemeralSecret;
use crate::clients::Client;
use crate::crypto::{asym_to_sym_key, fatten_public_key, get_random_asym_pair, get_shared_asym_secret, trim_public_key};
use crate::message::{ARdata, DNSMessage, DNSQuestion, QClass, QType, ResourceRecord};
use crate::message::record::CnameRdata;
use crate::string::{append_base_domain_to_key, strip_base_domain_from_key};

pub struct KeySwapContext {
    pub new_client: Client,
    pub response: DNSMessage,
    pub server_public: String,
    pub client_public: String
}

pub fn get_key_request_with_base_domain(base_domain: String) -> (EphemeralSecret, String)
{
    let (private, public) = get_random_asym_pair();

    (private, append_base_domain_to_key(trim_public_key(&public), &base_domain))
}

pub fn get_fattened_public_key(key_question: &DNSQuestion) -> (String, String)
{
    let public_key = &key_question.qname;
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

pub fn decode_key_request(message: DNSMessage) -> Result<KeySwapContext, KeyDecodeError>
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

        let (fattened_public_key, base_domain) = get_fattened_public_key(&key_question);
        let (server_private, server_public) = get_random_asym_pair();

        let shared_secret = get_shared_asym_secret(server_private, fattened_public_key);

        match shared_secret {
            Ok(secret) => {

                let sym_key = asym_to_sym_key(&secret);
                let new_client = Client::new(sym_key);
                let mut response = message.empty_resp_from_request();

                let first_record = ResourceRecord {
                    name_offset: 12,
                    answer_type: QType::A,
                    class: QClass::Internet,
                    ttl: 0,
                    rd_length: 4,
                    r_data: Box::new(ARdata::from(Ipv4Addr::from([127,0,0,1])))
                };

                let second_record = ResourceRecord {
                    name_offset: 12 + (&message.questions[0]).to_bytes().len() as u16,
                    answer_type: QType::CNAME,
                    class: QClass::Internet,
                    ttl: 0,
                    rd_length: 4,
                    r_data: Box::new(
                        CnameRdata::from(
                            append_base_domain_to_key(
                                trim_public_key(&server_public),
                                &base_domain
                            )
                        )
                    )
                };

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