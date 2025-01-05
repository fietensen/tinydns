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

    pub fn from_packet(packet: Packet) -> Self {
        PacketBuilder {
            header: packet.header,
            questions: packet.questions,
            answers: packet.answers,
            authorities: packet.authorities,
            additionals: packet.additionals,
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

    pub fn with_qentries(mut self, qentries: Vec<Question>) -> Self {
        self.questions.extend(qentries);
        self
    }

    pub fn with_aentry(mut self, aentry: ResourceRecord) -> Self {
        self.answers.push(aentry);
        self
    }

    pub fn with_aentries(mut self, aentries: Vec<ResourceRecord>) -> Self {
        self.answers.extend(aentries);
        self
    }

    pub fn with_authentry(mut self, authentry: ResourceRecord) -> Self {
        self.authorities.push(authentry);
        self
    }

    pub fn with_authentries(mut self, authentries: Vec<ResourceRecord>) -> Self {
        self.answers.extend(authentries);
        self
    }

    pub fn with_addentry(mut self, addentry: ResourceRecord) -> Self {
        self.additionals.push(addentry);
        self
    }

    pub fn with_addentries(mut self, addentries: Vec<ResourceRecord>) -> Self {
        self.answers.extend(addentries);
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
