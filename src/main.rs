use std::sync::Arc;
use std::env;
mod config;
mod proxy;
mod wasm_engine;
mod logger;
mod error;

use error::{Result, RilotError};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize custom logger
    logger::init();

    let args: Vec<String> = env::args().collect();
    let config_path = args.get(1).map_or("./config.json", |p| p.as_str());

    log::info!("🛠️ Loading configuration from: {}", config_path);

    let cfg = config::load_config(config_path)
        .map_err(|e| RilotError::ConfigError(format!("Failed to load config: {}", e)))?;
    log::info!("✅ Configuration loaded successfully.");

    if cfg.proxies.is_empty() {
        log::warn!("⚠️ No proxy rules defined in the configuration.");
    }

    let config_arc = Arc::new(cfg);

    log::info!("🚀 Starting proxy server...");
    proxy::start_proxy(config_arc).await
        .map_err(|e| RilotError::ProxyError(format!("Proxy server error: {}", e)))?;

    log::info!("👋 Proxy server shut down.");
    Ok(())
}