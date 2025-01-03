use bincode::Options;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

use super::flags::HeaderFlags;

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
        let serialize_options = bincode::DefaultOptions::new()
            .with_fixint_encoding()
            .with_big_endian();

        Ok(serialize_options.serialize(&self)?)
    }

    pub fn deserialize(buffer: &[u8]) -> Result<PacketHeader, Box<dyn std::error::Error>> {
        let deserialize_options = bincode::DefaultOptions::new()
            .with_fixint_encoding()
            .allow_trailing_bytes()
            .with_big_endian();

        Ok(deserialize_options.deserialize(buffer)?)
    }
}

impl Debug for PacketHeader {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "PacketHeader {{ id: {}, flags: {:?}, qdcount: {}, ancount: {}, nscount: {}, arcount: {} }}",
            self.id, HeaderFlags::from(self.flags), self.qdcount, self.ancount, self.nscount, self.arcount
        )
    }
}
