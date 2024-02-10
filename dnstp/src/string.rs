//! Utility functions for manipulating strings

use urlencoding::encode;

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