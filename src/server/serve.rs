use std::net::SocketAddr;

use crate::{
    protocol::packet::{
        flags::{Flags, HeaderFlags, OpCode, ResponseCode},
        Packet, PacketBuilder, PacketHeader,
    },
    server::handle_packet::handle_packet,
};

use super::ServerConfig;

pub async fn send_packet(
    server: &tokio::net::UdpSocket,
    client: SocketAddr,
    packet: Packet,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = packet.serialize();

    match response {
        Ok(data) => {
            if let Err(e) = server.send_to(&data, client).await {
                log::error!("Failed to send response: {}", e);
            }
        }
        Err(e) => log::error!("Failed to serialize response packet: {}", e),
    }

    Ok(())
}

pub async fn serve_udp<'a>(config: &ServerConfig<'a>) -> Result<(), Box<dyn std::error::Error>> {

    let server =
        tokio::net::UdpSocket::bind(format!("{}:{}", config.listen_addr(), config.udp_port()))
            .await?;

    loop {
        let mut buf: [u8; 512] = [0; 512];

        if let Ok((size, client)) = server.recv_from(&mut buf).await {
            let data = &buf[..size];
            log::trace!("Received {} bytes from {}:{}", size, client.ip(), client.port());

            // not even a query id was sent that could
            // be used to return a meaningful error
            if data.len() < 2 {
                continue;
            }

            let packet_deserialized = Packet::deserialize(data);

            // couldn't parse received packet
            if !packet_deserialized.is_ok() {
                log::warn!("Received malformed packet. Trying to reconstruct and answer.");
                if let Ok(header) = PacketHeader::deserialize(data) {
                    // try and preserve header
                    let header_flags: HeaderFlags = header.flags.into();
                    let _ = send_packet(
                        &server,
                        client,
                        PacketBuilder::new()
                            .with_flags(
                                HeaderFlags::new()
                                    .with_opcode(header_flags.0)
                                    .with_rcode(ResponseCode::FormatError)
                                    .with_flag(Flags::QR)
                                    .with_flag(Flags::RA),
                            )
                            .with_id(header.id)
                            .build(),
                    )
                    .await;
                } else {
                    // fallback to only query ID
                    let query_id = u16::from_be_bytes([data[0], data[1]]);
                    let _ = send_packet(
                        &server,
                        client,
                        PacketBuilder::new()
                            .with_flags(
                                HeaderFlags::new()
                                    .with_opcode(OpCode::Query)
                                    .with_rcode(ResponseCode::FormatError)
                                    .with_flag(Flags::QR)
                                    .with_flag(Flags::RA),
                            )
                            .with_id(query_id)
                            .build(),
                    )
                    .await;
                }
                continue;
            }
            let packet_deserialized = packet_deserialized.unwrap();

            if let Ok(response_packet) = handle_packet(packet_deserialized.clone(), config).await {
                let _ = send_packet(&server, client, response_packet).await;
            } else {
                // construct fail-answer if no question could be answered
                let _ = send_packet(
                    &server,
                    client,
                    PacketBuilder::new()
                        .with_flags(
                            HeaderFlags::new()
                                .with_opcode(HeaderFlags::from(packet_deserialized.header.flags).0)
                                .with_rcode(ResponseCode::NameError)
                                .with_flag(Flags::QR)
                                .with_flag(Flags::RA),
                        )
                        .with_id(packet_deserialized.header.id)
                        .with_qentries(packet_deserialized.questions)
                        .build(),
                )
                .await;
            }
        }
    }
}
