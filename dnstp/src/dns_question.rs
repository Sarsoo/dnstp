use url::form_urlencoded;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
enum QType {
    A = 1,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    WKS = 11,
    PTR = 12,
    HINFO = 13,
    MINFO = 14,
    MX = 15,
    TXT = 16,
    RP = 17,
    AAAA = 28,
    SRV = 33
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
enum QClass {
    Internet = 1,
    Chaos = 3,
    Hesiod = 4,
}

struct DNSQuestion {
    qname: String,
    qtype: QType,
    qclass: QClass
}

impl DNSQuestion {
    pub fn new(qname: String, qtype: QType, qclass: QClass) -> DNSQuestion
    {
        DNSQuestion {
            qname,
            qtype,
            qclass
        }
    }

    pub fn to_bytes(&self) -> Vec<u8>
    {
        let mut ret: Vec<u8> = vec!();

        for part in  self.qname.split(".")
        {
            let encoded_string: String = form_urlencoded::byte_serialize(part.as_bytes()).collect();
            let count = encoded_string.len();

            ret.push(count as u8);
            ret.reserve(count);
            for x in encoded_string.into_bytes() {
                ret.push(x);
            };
        }

        ret.push(0);

        ret.push(self.qtype as u8);
        ret.push(self.qclass as u8);

        ret
    }
}