use crate::{nameserver::Nameserver, resolver::Resolver};

pub struct ServerConfig<'a> {
    udp_port: u16,
    listen_addr: String,
    resolver: Resolver,
    nameserver: Option<Nameserver<'a>>
}

impl<'a> Default for ServerConfig<'a> {
    fn default() -> Self {
        ServerConfig {
            udp_port: 53,
            listen_addr: "127.0.0.1".to_string(),
            resolver: Resolver::default(),
            nameserver: None,
        }
    }
}

impl<'a> ServerConfig<'a> {
    pub fn with_udp_port(mut self, port: u16) -> Self {
        self.udp_port = port;
        self
    }

    pub fn with_listen_addr(mut self, addr: String) -> Self {
        self.listen_addr = addr;
        self
    }

    pub fn with_resolver(mut self, resolver: Resolver) -> Self {
        self.resolver = resolver;
        self
    }

    pub fn with_nameserver(mut self, nameserver: Nameserver<'a>) -> Self {
        self.nameserver = Some(nameserver);
        self
    }

    pub fn udp_port(&self) -> u16 {
        self.udp_port
    }

    pub fn listen_addr(&self) -> &str {
        &self.listen_addr
    }

    pub fn resolver(&self) -> &Resolver {
        &self.resolver
    }

    pub fn nameserver(&self) -> Option<&Nameserver<'a>> {
        self.nameserver.as_ref()
    }
}
