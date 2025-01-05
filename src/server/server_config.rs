use crate::resolver::Resolver;

pub struct ServerConfig {
    udp_port: u16,
    listen_addr: String,
    log_level: u8,
    log_file: Option<String>,
    resolver: Resolver,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            udp_port: 53,
            listen_addr: "127.0.0.1".to_string(),
            log_level: 3,
            log_file: None,
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

    pub fn with_log_level(mut self, level: u8) -> Self {
        self.log_level = level;
        self
    }

    pub fn with_log_file(mut self, file: String) -> Self {
        self.log_file = Some(file);
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

    pub fn log_level(&self) -> u8 {
        self.log_level
    }

    pub fn log_file(&self) -> Option<&String> {
        self.log_file.as_ref()
    }

    pub fn resolver(&self) -> &Resolver {
        &self.resolver
    }

    pub fn log_enabled(&self) -> bool {
        self.log_level > 0
    }

    pub fn log_file_enabled(&self) -> bool {
        self.log_file.is_some()
    }
}
