use crate::{
    protocol::{
        answer::AnswerEntry,
        packet::{
            flags::{Flags, HeaderFlags, OpCode, ResponseCode},
            Packet, PacketBuilder, Question,
        },
    }, server::ServerConfig
};

#[derive(Default)]
pub struct Resolver {
    fallback_servers: Vec<(String, u16)>,
}

impl Resolver {
    pub fn with_fallback_server(mut self, server: (String, u16)) -> Self {
        self.fallback_servers.push(server);
        self
    }
}

impl Resolver {
    /*
        Queries fallback dns for answers
        TODO: Handle multiple/truncated packets
    */
    async fn query_fallback(
        &self,
        packet: Packet,
        fallback: (String, u16),
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let connection = tokio::net::UdpSocket::bind("0.0.0.0:0").await?;
        connection.connect(fallback).await?;

        connection.send(&packet.serialize()?).await?;

        let mut buffer = [0; 512];
        let size = connection.recv(&mut buffer).await?;
        let response = &buffer[..size];

        let proxied_packet = Packet::deserialize(response)?;
        let flags: HeaderFlags = proxied_packet.header.flags.into();

        let packet_builder = PacketBuilder::from_packet(proxied_packet.clone())
            .with_flags(flags.without_flag(Flags::AA));

        Ok(packet_builder.build().serialize()?)
    }

    /*
        Intended for questions that should be delegated to fallback dns
    */
    pub async fn resolve_recursive(&self, questions: Vec<Question>) -> Vec<AnswerEntry> {
        let query_packet = PacketBuilder::new()
            .with_flags(
                HeaderFlags::new()
                    .with_opcode(OpCode::Query)
                    .with_rcode(ResponseCode::NoError)
                    .with_flag(Flags::RD),
            )
            // TODO: How should this be generated?
            .with_id(0x0001)
            .with_qentries(questions)
            .build();

        for fallback in self.fallback_servers.clone() {
            let response = self.query_fallback(query_packet.clone(), fallback).await;
            if !response.is_ok() {
                continue;
            }

            // parse received packet
            let packet = Packet::deserialize(&response.unwrap());
            if !packet.is_ok() {
                continue;
            }

            // it worked! return answers
            // TODO: improve the way these answers are constructed
            // (my inner monk won't let me sleep tonight for writing something this ugly/hacky)
            let packet = packet.unwrap();
            let mut answers = Vec::new();
            for answer in packet.answers {
                let mut answer_entry = AnswerEntry::default();
                answer_entry.resource = Some(answer);
                answers.push(answer_entry);
            }

            for authority in packet.authorities {
                let mut auth_entry = AnswerEntry::default();
                auth_entry.authority = Some(authority);
                answers.push(auth_entry)
            }

            let mut additionals = AnswerEntry::default();
            additionals.additional = packet.additionals;
            answers.push(additionals);

            return answers;
        }

        Vec::new()
    }

    /*
        Tries to retrieve zone authority
    */
    pub async fn get_zoneauthority<'a>(&self, zone: String, config: &ServerConfig<'a>) -> AnswerEntry {
        // TODO: resolve zone authority
        AnswerEntry::default()
    }
}
