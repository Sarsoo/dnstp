use std::fmt::{Debug, Formatter};
use crate::message::record::RData;

pub struct RawRData {
    pub rdata: Vec<u8>
}

impl Debug for RawRData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RawRData")
            .field("data", &self.rdata)
            .finish()
    }
}

impl RData for RawRData {
    fn to_bytes(&self) -> Vec<u8> {
        self.rdata.clone()
    }
}

impl RawRData {
    pub fn from(rdata: Vec<u8>) -> RawRData
    {
        RawRData {
            rdata
        }
    }
}