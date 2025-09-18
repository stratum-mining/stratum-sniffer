use serde::Deserialize;
use std::fs;
use std::net::SocketAddr;
use std::path::Path;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server_addr: SocketAddr,
    pub listen_addr: SocketAddr,
    pub sv2: bool,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let contents = fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Failed to read config file: {}", e))?;
        let config: Self = toml::from_str(&contents)
            .map_err(|e| anyhow::anyhow!("Failed to parse config file: {}", e))?;
        Ok(config)
    }
}
