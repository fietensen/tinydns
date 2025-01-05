use crate::protocol::packet::Packet;

use super::ServerConfig;

pub async fn serve_udp(config: &ServerConfig) -> Result<(), Box<dyn std::error::Error>> {
    let server =
        tokio::net::UdpSocket::bind(format!("{}:{}", config.listen_addr(), config.udp_port()))
            .await?;

    loop {
        let mut buf: [u8; 512] = [0; 512];

        if let Ok((size, client)) = server.recv_from(&mut buf).await {
            let data = &buf[..size];
            println!("Received {} bytes", size);

            let packet_deserialized = Packet::deserialize(data);
            if !packet_deserialized.is_ok() {
                println!("Failed to deserialize packet");
                continue;
            }

            let response = config
                .resolver()
                .resolve(packet_deserialized.unwrap())
                .await?;

            if let Err(e) = server.send_to(&response, client).await {
                println!("Failed to send response: {}", e);
            }
        }
    }
}
