use std::default;

use super::record_type::RecordType;

#[derive(Debug)]
pub struct Question {
    name: String,
    qtype: u16,
    qclass: u16,

    size: usize,
}

impl Default for Question {
    fn default() -> Self {
        Self {
            name: String::new(),
            qtype: RecordType::A as u16,
            qclass: 1,
            size: 5,
        }
    }
}

impl Question {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self.size = 5 + self.name.len();
        self
    }

    pub fn with_qtype(mut self, qtype: u16) -> Self {
        self.qtype = qtype;
        self
    }

    pub fn with_qclass(mut self, qclass: u16) -> Self {
        self.qclass = qclass;
        self
    }

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

    pub fn deserialize(buffer: &[u8]) -> Result<Question, Box<dyn std::error::Error>> {
        let mut offset = 0;
        let mut name = String::new();
        loop {
            let len = buffer[offset] as usize;
            if len == 0 {
                break;
            }
            if !name.is_empty() {
                name.push('.');
            }
            name.push_str(std::str::from_utf8(&buffer[offset + 1..offset + 1 + len])?);
            offset += len + 1;
        }
        offset += 1;
        let qtype = u16::from_be_bytes([buffer[offset], buffer[offset + 1]]);
        offset += 2;
        let qclass = u16::from_be_bytes([buffer[offset], buffer[offset + 1]]);
        let size = offset + 2;

        Ok(Question {
            name,
            qtype,
            qclass,
            size,
        })
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
