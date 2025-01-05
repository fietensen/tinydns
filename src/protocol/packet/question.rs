use super::record_type::RecordType;

#[derive(PartialEq, Clone)]
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
            size: 6,
        }
    }
}

impl Question {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self.size = 6 + self.name.len();
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

    pub fn deserialize(
        buffer: &[u8],
        offset: &mut usize,
    ) -> Result<Question, Box<dyn std::error::Error>> {
        let mut name = String::new();

        if buffer[*offset] & 0xc0 == 0xc0 {
            let pointer = u16::from_be_bytes([buffer[*offset], buffer[*offset + 1]]) & 0x3FFF;
            let mut loop_offset = pointer as usize;
            loop {
                let len = buffer[loop_offset] as usize;
                if len == 0 {
                    break;
                }
                if !name.is_empty() {
                    name.push('.');
                }
                name.push_str(std::str::from_utf8(
                    &buffer[loop_offset + 1..loop_offset + 1 + len],
                )?);
                loop_offset += len + 1;
            }

            *offset += 2;
        } else {
            loop {
                let len = buffer[*offset] as usize;
                if len == 0 {
                    *offset += 1;
                    break;
                }
                if !name.is_empty() {
                    name.push('.');
                }
                name.push_str(std::str::from_utf8(
                    &buffer[*offset + 1..*offset + 1 + len],
                )?);
                *offset += len + 1;
            }
        }

        let qtype = u16::from_be_bytes([buffer[*offset], buffer[*offset + 1]]);
        *offset += 2;
        let qclass = u16::from_be_bytes([buffer[*offset], buffer[*offset + 1]]);
        *offset += 2;
        let size = 6 + name.len();

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

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn qtype(&self) -> RecordType {
        RecordType::from(self.qtype)
    }

    pub fn qclass(&self) -> u16 {
        self.qclass
    }
}

impl std::fmt::Debug for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Question {{ name: {}, qtype: {:#?}, qclass: {} }}",
            self.name,
            RecordType::from(self.qtype),
            self.qclass
        )
    }
}
