use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

#[derive(Serialize, Deserialize, Default)]
pub struct PacketHeader {
    pub id: u16,
    pub flags: u16,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl PacketHeader {
    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(bincode::serialize(&self)?)
    }
}

impl Debug for PacketHeader {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "PacketHeader")
    }
}
