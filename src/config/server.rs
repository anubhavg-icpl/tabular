use std::env;

pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("SERVER_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080);

        ServerConfig { host, port }
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn display_info(&self) {
        println!("╔═══════════════════════════════════════════╗");
        println!("║      🚀 Tabular Dashboard Server          ║");
        println!("╠═══════════════════════════════════════════╣");
        println!("║ 📊 Server: http://{}:{:<18} ║", self.host, self.port);
        println!("║ 📁 Endpoints:                             ║");
        println!("║    • /          → Dashboard               ║");
        println!("║    • /dashboard → Data view               ║");
        println!("║    • /datasets  → Available datasets      ║");
        println!("╚═══════════════════════════════════════════╝");
    }
}
