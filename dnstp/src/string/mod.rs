//! Utility functions for manipulating strings

#[cfg(test)]
mod tests;

use urlencoding::{decode, encode};
use crate::crypto::fatten_public_key;

pub fn encode_domain_name(name: &String) -> Vec<u8>
{
    let mut ret: Vec<u8> = Vec::with_capacity(name.len() + 3);

    for part in name.split(".")
    {
        let encoded_string = encode(part);
        let count = encoded_string.len();

        ret.push(count as u8);
        for x in encoded_string.bytes() {
            ret.push(x);
        };
    }

    ret.push(0);

    ret
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub enum DomainDecodeError {
    UTF8Parse,
    URLDecode
}

pub fn decode_domain_name(name: Vec<u8>) -> Result<String, DomainDecodeError>
{
    let mut full_domain: String = String::new();
    let mut current_query: Vec<u8> = Vec::with_capacity(10);
    let mut current_length: Option<u8> = None;
    let mut remaining_length: u8 = 0;

    for char in name {

        match current_length {
            None => {
                current_length = Some(char);
                remaining_length = char;
            }
            Some(_) => {
                if remaining_length == 0 {

                    match String::from_utf8(current_query.clone()) {
                        Ok(parsed_query) => {
                            match decode(parsed_query.as_str()) {
                                Ok(decoded_query) => {
                                    full_domain.push_str(&decoded_query.to_string());

                                    if char != 0 {
                                        full_domain.push('.');
                                    }

                                    current_query.clear();
                                    current_length = Some(char);
                                    remaining_length = char;
                                }
                                Err(_) => {
                                    return Err(DomainDecodeError::URLDecode);
                                }
                            }
                        }
                        Err(_) => {
                            return Err(DomainDecodeError::UTF8Parse);
                        }
                    }
                }
                else {
                    current_query.push(char);
                    remaining_length -= 1;
                }
            }
        }
    }

    Ok(full_domain)
}

pub fn strip_base_domain_from_key(public_key: &String) -> (String, String)
{
    let periods: Vec<_> = public_key.rmatch_indices(".").collect();

    if periods.len() >= 2 {
        (public_key[0 .. periods[1].0].to_string(),
         public_key[periods[1].0 .. ].to_string())
    }
    else if periods.len() == 1 {
        (public_key[0 .. periods[0].0].to_string(),
         public_key[periods[0].0 .. ].to_string())
    }
    else {
        (public_key.to_string(), String::new())
    }
}

pub fn append_base_domain_to_key(trimmed_key: String, base_domain: &String) -> String
{
    vec![trimmed_key, base_domain.to_string()].join(".")
}

/// Extract the client's public key from the DNS message, turn the hostname back into the full fat public key with --- BEGIN KEY --- headers and trailers
pub fn get_fattened_public_key(key_question: &String) -> (String, String)
{
    let public_key = key_question;
    let (trimmed_public_key, base_domain) = strip_base_domain_from_key(public_key);

    (fatten_public_key(&trimmed_public_key), base_domain)
}
