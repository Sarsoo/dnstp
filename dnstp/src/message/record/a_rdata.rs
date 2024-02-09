use std::fmt::{Debug, Formatter};
use std::net::Ipv4Addr;
use crate::message::record::RData;

pub struct ARdata {
    pub rdata: Ipv4Addr
}

impl Debug for ARdata {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("A")
            .field("IP", &self.rdata)
            .finish()
    }
}

impl RData for ARdata {
    fn to_bytes(&self) -> Vec<u8> {
        self.rdata.octets().to_vec()
    }
}

impl ARdata {
    pub fn from(rdata: Ipv4Addr) -> ARdata
    {
        ARdata {
            rdata
        }
    }
}