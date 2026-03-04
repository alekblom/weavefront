use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_db_path")]
    pub db_path: String,
    pub admin_password: String,
    #[serde(default)]
    pub pinata_jwt: Option<String>,
    #[serde(default)]
    pub ipfs_api_url: Option<String>,
    #[serde(default)]
    pub arweave_gateway_url: Option<String>,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    3100
}

fn default_db_path() -> String {
    "weavefront.db".to_string()
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            host: std::env::var("WEAVEFRONT_HOST").unwrap_or_else(|_| default_host()),
            db_path: std::env::var("WEAVEFRONT_DB_PATH").unwrap_or_else(|_| default_db_path()),
            port: std::env::var("WEAVEFRONT_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or_else(default_port),
            admin_password: std::env::var("ADMIN_PASSWORD")
                .expect("ADMIN_PASSWORD must be set in environment"),
            pinata_jwt: std::env::var("PINATA_JWT").ok(),
            ipfs_api_url: std::env::var("IPFS_API_URL").ok(),
            arweave_gateway_url: std::env::var("ARWEAVE_GATEWAY_URL").ok(),
        }
    }

    pub fn listen_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
