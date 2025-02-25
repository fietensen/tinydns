use crate::resolver::Resolver;

pub struct ServerConfig {
    udp_port: u16,
    listen_addr: String,
    resolver: Resolver,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            udp_port: 53,
            listen_addr: "127.0.0.1".to_string(),
            resolver: Resolver::default(),
        }
    }
}

impl ServerConfig {
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

    pub fn udp_port(&self) -> u16 {
        self.udp_port
    }

    pub fn listen_addr(&self) -> &str {
        &self.listen_addr
    }

    pub fn resolver(&self) -> &Resolver {
        &self.resolver
    }
}
