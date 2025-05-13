use serde::Deserialize;
use crate::error::{Result, RilotError};
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct ProxyRule {
    pub path: String,
    #[serde(rename = "type", default = "default_rule_type")]
    pub r#type: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProxyConfig {
    pub app_name: String, // for next version handling directly via name istead url
    pub app_uri: String,
    #[serde(default)]
    pub override_file: Option<String>,
    pub rule: ProxyRule,
    #[serde(default = "default_rewrite_mode")]
    pub rewrite: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub proxies: Vec<ProxyConfig>,
}

fn default_rule_type() -> String {
    "contain".to_string()
}

fn default_rewrite_mode() -> String {
    "none".to_string()
}

pub fn load_config(path: &str) -> Result<Config> {
    let data = fs::read_to_string(path)
        .map_err(|e| RilotError::ConfigError(format!("Failed to read config file: {}", e)))?;

    serde_json::from_str(&data)
        .map_err(|e| RilotError::ConfigError(format!("Failed to parse config file: {}", e)))
}
