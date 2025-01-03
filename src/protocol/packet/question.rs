#[derive(Default)]
pub struct Question {
    pub name: String,
    pub qtype: u16,
    pub qclass: u16,
}

impl Question {
    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        for part in self.name.split('.') {
            buf.push(part.len() as u8);
            buf.extend(part.as_bytes());
        }
        buf.push(0);
        buf.extend(&self.qtype.to_be_bytes());
        buf.extend(&self.qclass.to_be_bytes());
        Ok(buf)
    }
}
