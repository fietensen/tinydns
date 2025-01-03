use super::{header::PacketHeader, question::Question, resource_record::ResourceRecord};

#[derive(Debug)]
pub struct Packet {
    pub header: PacketHeader,
    pub questions: Vec<Question>,
    pub answers: Vec<ResourceRecord>,
    pub authorities: Vec<ResourceRecord>,
    pub additionals: Vec<ResourceRecord>,
}

impl Packet {
    pub fn serialize(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut buffer = Vec::new();
        buffer.extend(self.header.serialize()?);
        for question in &self.questions {
            buffer.extend(question.serialize()?);
        }
        for answer in &self.answers {
            buffer.extend(answer.serialize()?);
        }
        for authority in &self.authorities {
            buffer.extend(authority.serialize()?);
        }
        for additional in &self.additionals {
            buffer.extend(additional.serialize()?);
        }
        Ok(buffer)
    }

    pub fn deserialize(buffer: &[u8]) -> Result<Packet, Box<dyn std::error::Error>> {
        let mut offset = 0;
        let header = PacketHeader::deserialize(&buffer[offset..])?;
        offset += std::mem::size_of::<PacketHeader>();

        let mut questions = Vec::new();
        for _ in 0..header.qdcount {
            let question = Question::deserialize(&buffer[offset..])?;
            offset += question.size();
            questions.push(question);
        }

        let mut answers = Vec::new();
        for _ in 0..header.ancount {
            let answer = ResourceRecord::deserialize(&buffer[offset..])?;
            offset += answer.size();
            answers.push(answer);
        }

        let mut authorities = Vec::new();
        for _ in 0..header.nscount {
            let authority = ResourceRecord::deserialize(&buffer[offset..])?;
            offset += authority.size();
            authorities.push(authority);
        }

        let mut additionals = Vec::new();
        for _ in 0..header.arcount {
            let additional = ResourceRecord::deserialize(&buffer[offset..])?;
            offset += additional.size();
            additionals.push(additional);
        }

        Ok(Packet {
            header,
            questions,
            answers,
            authorities,
            additionals,
        })
    }
}
