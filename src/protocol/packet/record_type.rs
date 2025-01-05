#[derive(Debug)]
#[repr(u16)]
pub enum RecordType {
    A = 1,
    NS,
    MD,
    MF,
    CNAME,
    SOA,
    MB,
    MG,
    MR,
    NULL,
    WKS,
    PTR,
    HINFO,
    MINFO,
    MX,
    TXT,
    AAAA = 28,
    HTTPS = 65,
    AXFR = 252,
    MAILB,
    MAILA,
    BROADCAST,
}

impl From<u16> for RecordType {
    fn from(value: u16) -> Self {
        match value {
            1 => RecordType::A,
            2 => RecordType::NS,
            3 => RecordType::MD,
            4 => RecordType::MF,
            5 => RecordType::CNAME,
            6 => RecordType::SOA,
            7 => RecordType::MB,
            8 => RecordType::MG,
            9 => RecordType::MR,
            10 => RecordType::NULL,
            11 => RecordType::WKS,
            12 => RecordType::PTR,
            13 => RecordType::HINFO,
            14 => RecordType::MINFO,
            15 => RecordType::MX,
            16 => RecordType::TXT,
            28 => RecordType::AAAA,
            65 => RecordType::HTTPS,
            252 => RecordType::AXFR,
            253 => RecordType::MAILB,
            254 => RecordType::MAILA,
            255 => RecordType::BROADCAST,
            _ => RecordType::NULL,
        }
    }
}

impl Into<u16> for RecordType {
    fn into(self) -> u16 {
        self as u16
    }
}
