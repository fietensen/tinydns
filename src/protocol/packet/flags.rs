use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug)]
#[repr(u16)]
pub enum OpCode {
    #[default]
    Query = 0,
    IQuery = 1,
    Status = 2,
    Unknown = u16::MAX,
}

impl OpCode {
    pub fn from_u16(value: u16) -> Self {
        match value {
            0 => OpCode::Query,
            1 => OpCode::IQuery,
            2 => OpCode::Status,
            _ => OpCode::Unknown,
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
#[repr(u16)]
pub enum ResponseCode {
    #[default]
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5,
    Unknown = u16::MAX,
}

impl ResponseCode {
    pub fn from_u16(value: u16) -> Self {
        match value {
            0 => ResponseCode::NoError,
            1 => ResponseCode::FormatError,
            2 => ResponseCode::ServerFailure,
            3 => ResponseCode::NameError,
            4 => ResponseCode::NotImplemented,
            5 => ResponseCode::Refused,
            _ => ResponseCode::Unknown,
        }
    }
}

#[repr(u16)]
#[derive(Default)]
pub enum Flags {
    QR = 1 << 15,
    AA = 1 << 10,
    TC = 1 << 9,
    RD = 1 << 8,
    RA = 1 << 7,

    // Not in use
    #[default]
    NULL = 0,
}

#[derive(Default, Debug)]
pub struct HeaderFlags(OpCode, u16, ResponseCode);

impl HeaderFlags {
    pub fn new() -> Self {
        HeaderFlags(OpCode::Query, 0, ResponseCode::NoError)
    }

    pub fn with_opcode(mut self, opcode: OpCode) -> Self {
        self.0 = opcode;
        self
    }

    pub fn with_rcode(mut self, rcode: ResponseCode) -> Self {
        self.2 = rcode;
        self
    }

    pub fn with_flag(mut self, flag: Flags) -> Self {
        self.1 |= flag as u16;
        self
    }

    pub fn serialize(&self) -> u16 {
        ((self.0 as u16) << 11) | self.1 | ((self.2 as u16) << 0)
    }
}

impl Serialize for HeaderFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(self.serialize())
    }
}

impl From<u16> for HeaderFlags {
    fn from(flags: u16) -> Self {
        HeaderFlags(
            OpCode::from_u16((flags >> 11) & 0b111),
            flags & 0b11111111111,
            ResponseCode::from_u16(flags & 0b1111),
        )
    }
}
