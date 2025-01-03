use super::{
    header::PacketHeader, packet::Packet, question::Question, resource_record::ResourceRecord,
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

    pub fn with_qr(mut self, qr: u16) -> Self {
        self.header.flags |= qr << 15;
        self
    }

    pub fn with_opcode(mut self, opcode: u16) -> Self {
        self.header.flags |= opcode << 11;
        self
    }

    pub fn with_aa(mut self, aa: u16) -> Self {
        self.header.flags |= aa << 10;
        self
    }

    pub fn with_tc(mut self, tc: u16) -> Self {
        self.header.flags |= tc << 9;
        self
    }

    pub fn with_rd(mut self, rd: u16) -> Self {
        self.header.flags |= rd << 8;
        self
    }

    pub fn with_ra(mut self, ra: u16) -> Self {
        self.header.flags |= ra << 7;
        self
    }

    pub fn with_rcode(mut self, rcode: u16) -> Self {
        self.header.flags |= rcode;
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

    pub fn build(self) -> Packet {
        Packet {
            header: self.header,
            questions: self.questions,
            answers: self.answers,
            authorities: self.authorities,
            additionals: self.additionals,
        }
    }
}
