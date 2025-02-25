use database::RecordEntity;
use nameserver::Nameserver;
use protocol::{packet::RecordType, util};

mod nameserver;
mod protocol;
mod resolver;
mod server;
mod database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // set up logging
    log4rs::init_file("config/log4rs.yml", Default::default())?;
    let db = database::Database::init("sqlite.db").await?;

    // create local nameserver
    let nameserver = Nameserver::new(&db);

    // create resolver
    let dns_resolver =
        resolver::Resolver::default().with_fallback_server(("8.8.8.8".to_string(), 53));

    let config = server::ServerConfig::default()
        .with_resolver(dns_resolver)
        .with_nameserver(nameserver);

    log::info!("Starting to serve UDP");
    if let Err(err) = server::serve::serve_udp(&config).await {
        log::error!("Server failed due to an unhandled exception: {}", err);
    } else {
        log::info!("Server exited naturally")
    }

    Ok(())
}
