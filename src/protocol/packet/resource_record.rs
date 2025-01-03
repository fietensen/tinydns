#[derive(Debug, PartialEq, Clone)]
pub struct ResourceRecord {
    name: String,
    rtype: u16,
    rclass: u16,
    ttl: u32,
    rdlength: u16,
    rdata: Vec<u8>,
    size: usize,
}

impl Default for ResourceRecord {
    fn default() -> Self {
        Self {
            name: String::new(),
            rtype: 1,
            rclass: 1,
            ttl: 0,
            rdlength: 0,
            rdata: Vec::new(),
            size: 12,
        }
    }
}

impl ResourceRecord {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self.size = 12 + self.name.len();
        self
    }

    pub fn with_rtype(mut self, rtype: u16) -> Self {
        self.rtype = rtype;
        self
    }

    pub fn with_rclass(mut self, rclass: u16) -> Self {
        self.rclass = rclass;
        self
    }

    pub fn with_ttl(mut self, ttl: u32) -> Self {
        self.ttl = ttl;
        self
    }

    pub fn with_rdlength(mut self, rdlength: u16) -> Self {
        self.rdlength = rdlength;
        self
    }

    pub fn with_rdata(mut self, rdata: Vec<u8>) -> Self {
        self.rdata = rdata;
        self.size = 12 + self.name.len() + self.rdata.len();
        self
    }

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

    pub fn deserialize(
        buffer: &[u8],
        offset: &mut usize,
    ) -> Result<ResourceRecord, Box<dyn std::error::Error>> {
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
        let rtype = u16::from_be_bytes([buffer[*offset], buffer[*offset + 1]]);
        *offset += 2;
        let rclass = u16::from_be_bytes([buffer[*offset], buffer[*offset + 1]]);
        *offset += 2;
        let ttl = u32::from_be_bytes([
            buffer[*offset],
            buffer[*offset + 1],
            buffer[*offset + 2],
            buffer[*offset + 3],
        ]);
        *offset += 4;
        let rdlength = u16::from_be_bytes([buffer[*offset], buffer[*offset + 1]]);
        *offset += 2;
        let rdata = buffer[*offset..*offset + rdlength as usize].to_vec();
        *offset += rdlength as usize;

        let size = 12 + name.len() + rdata.len();

        Ok(ResourceRecord {
            name,
            rtype,
            rclass,
            ttl,
            rdlength,
            rdata,
            size,
        })
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
