use serde::Deserialize;

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
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub proxies: Vec<ProxyConfig>,
}

fn default_rule_type() -> String {
    "contain".to_string()
}

use std::fs;

pub fn load_config(path: &str) -> Config {
    let data = fs::read_to_string(path).expect("Failed to read config.json");
    serde_json::from_str(&data).expect("Failed to parse config.json")
}
