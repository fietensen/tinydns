use super::{header::PacketHeader, question::Question, resource_record::ResourceRecord};

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
}
