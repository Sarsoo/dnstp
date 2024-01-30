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