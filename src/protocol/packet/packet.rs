use super::{header::PacketHeader, question::Question, resource_record::ResourceRecord};

#[derive(Debug, Clone, PartialEq)]
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
            let question = Question::deserialize(&buffer, &mut offset)?;
            questions.push(question);
        }

        let mut answers = Vec::new();
        for _ in 0..header.ancount {
            let answer = ResourceRecord::deserialize(&buffer, &mut offset)?;
            answers.push(answer);
        }

        let mut authorities = Vec::new();
        for _ in 0..header.nscount {
            let authority = ResourceRecord::deserialize(&buffer, &mut offset)?;
            authorities.push(authority);
        }

        let mut additionals = Vec::new();
        for _ in 0..header.arcount {
            let additional = ResourceRecord::deserialize(&buffer, &mut offset)?;
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

#[cfg(test)]
mod tests {
    use crate::protocol::packet::{record_type::RecordType, resource_record};

    use super::*;

    #[test]
    fn test_packet_serialize_deserialize() {
        let header = PacketHeader {
            id: 0x1234,
            flags: 0x0100,
            qdcount: 1,
            ancount: 1,
            nscount: 1,
            arcount: 1,
        };

        let question = Question::default()
            .with_name("example.com".to_string())
            .with_qtype(RecordType::A as u16)
            .with_qclass(1);

        let resource_record = resource_record::ResourceRecord::default()
            .with_name("example.com".to_string())
            .with_rtype(1)
            .with_rclass(1)
            .with_ttl(3600)
            .with_rdlength(4)
            .with_rdata(vec![192, 0, 2, 1]);

        let packet = Packet {
            header,
            questions: vec![question],
            answers: vec![resource_record.clone()],
            authorities: vec![resource_record.clone()],
            additionals: vec![resource_record.clone()],
        };

        let serialized = packet.serialize().expect("Failed to serialize packet");
        let deserialized = Packet::deserialize(&serialized).expect("Failed to deserialize packet");

        assert_eq!(packet.header, deserialized.header);
        assert_eq!(packet.questions, deserialized.questions);
        assert_eq!(packet.answers, deserialized.answers);
        assert_eq!(packet.authorities, deserialized.authorities);
        assert_eq!(packet.additionals, deserialized.additionals);
    }
}
