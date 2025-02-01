use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RouteConfig {
    pub path: String,
    pub target: String,
    pub methods: Vec<String>,
    pub rate_limit: Option<u32>,
    pub auth_required: bool,
    #[serde(default)]
    pub validation_schema: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GatewayConfig {
    pub listen_address: String,
    pub jwt_secret: String,
    pub default_rate_limit: u32,
    pub routes: Vec<RouteConfig>,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Parse error: {0}")]
    ParseError(serde_yaml::Error),
}

pub fn load_config<P: AsRef<Path>>(path: P) -> Result<GatewayConfig, ConfigError> {
    let content = fs::read_to_string(&path).map_err(|_| ConfigError::FileNotFound(path.as_ref().to_string_lossy().into()))?;
    serde_yaml::from_str(&content).map_err(ConfigError::ParseError)
}