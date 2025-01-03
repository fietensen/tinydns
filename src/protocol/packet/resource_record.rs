#[derive(Default)]
pub struct ResourceRecord {
    pub name: String,
    pub rtype: u16,
    pub rclass: u16,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>,
}

impl ResourceRecord {
    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        for part in self.name.split('.') {
            buf.push(part.len() as u8);
            buf.extend(part.as_bytes());
        }
        buf.push(0);
        buf.extend(&self.rtype.to_be_bytes());
        buf.extend(&self.rclass.to_be_bytes());
        buf.extend(&self.ttl.to_be_bytes());
        buf.extend(&self.rdlength.to_be_bytes());
        buf.extend(&self.rdata);
        Ok(buf)
    }
}
