#[derive(Copy, Clone)]
pub enum OpCode {
    Query = 0,
    IQuery = 1,
    Status = 2,
}
#[derive(Copy, Clone)]
pub enum ResponseCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5,
}

pub enum Flags {
    QR = 1 << 15,
    AA = 1 << 10,
    TC = 1 << 9,
    RD = 1 << 8,
    RA = 1 << 7,
}

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
