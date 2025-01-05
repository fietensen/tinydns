use crate::protocol::{
    answer::AnswerEntry,
    packet::{
        flags::{Flags, HeaderFlags, ResponseCode},
        Packet, PacketBuilder, Question,
    },
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

    pub async fn resolve(&self, questions: Vec<Question>) -> Option<AnswerEntry> {
        None
    }
}
