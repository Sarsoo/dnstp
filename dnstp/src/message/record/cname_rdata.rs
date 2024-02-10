use std::fmt::{Debug, Formatter};
use crate::message::record::RData;
use crate::string::encode_domain_name;

pub struct CnameRdata {
    pub rdata: String
}

impl Debug for CnameRdata {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CNAME")
            .field("Host", &self.rdata)
            .finish()
    }
}

impl RData for CnameRdata {
    fn to_bytes(&self) -> Vec<u8> {
        encode_domain_name(&self.rdata)
    }
}

impl CnameRdata {
    pub fn from(rdata: String) -> CnameRdata
    {
        CnameRdata {
            rdata
        }
    }
}