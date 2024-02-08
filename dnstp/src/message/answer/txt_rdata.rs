use std::fmt::{Debug, Formatter};
use crate::message::answer::RData;

pub struct TXTRdata {
    pub rdata: String
}

impl Debug for TXTRdata {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TXT")
            .field("data", &self.rdata)
            .finish()
    }
}

impl RData for TXTRdata {
    fn to_bytes(&self) -> Vec<u8> {
        self.rdata.clone().into_bytes()
    }
}

impl TXTRdata {
    pub fn from(rdata: String) -> TXTRdata
    {
        TXTRdata {
            rdata
        }
    }
}