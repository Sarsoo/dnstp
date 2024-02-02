use std::fmt::{Debug, Formatter};
use std::net::{IpAddr, Ipv4Addr};
use crate::message::answer::RData;

pub struct IpRData {
    pub rdata: Ipv4Addr
}

impl Debug for IpRData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IP")
            .field("data", &self.rdata)
            .finish()
    }
}

impl RData for IpRData {
    fn to_bytes(&self) -> Vec<u8> {
        return self.rdata.octets().to_vec();
    }
}

impl IpRData {
    pub fn from(rdata: Ipv4Addr) -> IpRData
    {
        IpRData {
            rdata
        }
    }
}