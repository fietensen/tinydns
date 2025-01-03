use super::{
    flags::HeaderFlags, header::PacketHeader, packet::Packet, question::Question,
    resource_record::ResourceRecord,
};

pub struct PacketBuilder {
    header: PacketHeader,
    questions: Vec<Question>,
    answers: Vec<ResourceRecord>,
    authorities: Vec<ResourceRecord>,
    additionals: Vec<ResourceRecord>,
}

impl PacketBuilder {
    pub fn new() -> Self {
        PacketBuilder {
            header: PacketHeader::default(),
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            additionals: Vec::new(),
        }
    }

    pub fn with_id(mut self, id: u16) -> Self {
        self.header.id = id;
        self
    }

    pub fn with_flags(mut self, flags: HeaderFlags) -> Self {
        self.header.flags = flags.serialize();
        self
    }

    pub fn with_qentry(mut self, qentry: Question) -> Self {
        self.questions.push(qentry);
        self
    }

    pub fn with_aentry(mut self, aentry: ResourceRecord) -> Self {
        self.answers.push(aentry);
        self
    }

    pub fn with_authentry(mut self, authentry: ResourceRecord) -> Self {
        self.authorities.push(authentry);
        self
    }

    pub fn with_addentry(mut self, addentry: ResourceRecord) -> Self {
        self.additionals.push(addentry);
        self
    }

    pub fn build(mut self) -> Packet {
        self.header.qdcount = self.questions.len() as u16;
        self.header.ancount = self.answers.len() as u16;
        self.header.nscount = self.authorities.len() as u16;
        self.header.arcount = self.additionals.len() as u16;

        Packet {
            header: self.header,
            questions: self.questions,
            answers: self.answers,
            authorities: self.authorities,
            additionals: self.additionals,
        }
    }
}
