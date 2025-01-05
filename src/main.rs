mod protocol;
mod resolver;
mod server;

#[tokio::main]
async fn main() {
    let dns_resolver =
        resolver::Resolver::default().with_fallback_server(("8.8.8.8".to_string(), 53));

    let config = server::ServerConfig::default().with_resolver(dns_resolver);

    server::serve::serve_udp(&config).await.unwrap();
}
