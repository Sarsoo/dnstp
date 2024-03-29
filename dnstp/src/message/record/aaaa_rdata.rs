use std::fmt::{Debug, Formatter};
use std::net::Ipv6Addr;
use crate::message::record::RData;

pub struct AAAARdata {
    pub rdata: Ipv6Addr
}

impl Debug for AAAARdata {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AAAA")
            .field("IP", &self.rdata)
            .finish()
    }
}

impl RData for AAAARdata {
    fn to_bytes(&self) -> Vec<u8> {
        self.rdata.octets().to_vec()
    }
}

impl AAAARdata {
    pub fn from(rdata: Ipv6Addr) -> AAAARdata
    {
        AAAARdata {
            rdata
        }
    }
}