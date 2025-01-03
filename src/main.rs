use protocol::resolver;

mod protocol;

#[tokio::main]
async fn main() {
    let dns_resolver =
        resolver::Resolver::default().with_fallback_server(("8.8.8.8".to_string(), 53));

    let config = protocol::ServerConfig::default().with_resolver(dns_resolver);

    protocol::serve::serve_udp(&config).await.unwrap();
}
