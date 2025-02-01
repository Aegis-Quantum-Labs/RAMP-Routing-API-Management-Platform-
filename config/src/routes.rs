use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RouteConfig {
    pub path: String,
    pub target: String,
    #[serde(default)]
    pub methods: Vec<String>,
    #[serde(default)]
    pub rate_limit: Option<u32>,
    #[serde(default)]
    pub auth_required: bool,
    #[serde(default)]
    pub validation_schema: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GatewayConfig {
    pub listen_address: String,
    pub jwt_secret: String,
    #[serde(default = "default_rate_limit")]
    pub default_rate_limit: u32,
    pub routes: Vec<RouteConfig>,
}

fn default_rate_limit() -> u32 {
    100
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileReadError(String),
    #[error("Failed to parse config: {0}")]
    ParseError(#[from] serde_yaml::Error),
}

pub fn load_config<P: AsRef<Path>>(path: P) -> Result<GatewayConfig, ConfigError> {
    let content = std::fs::read_to_string(&path)
        .map_err(|e| ConfigError::FileReadError(e.to_string()))?;
    serde_yaml::from_str(&content).map_err(Into::into)
}