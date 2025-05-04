use std::sync::Arc;
use std::env;
mod config;
mod proxy;
mod wasm_engine;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args: Vec<String> = env::args().collect();
    let config_path = args.get(1).map_or("./config.json", |p| p.as_str());

    log::info!("🛠️ Loading configuration from: {}", config_path);

    let cfg = config::load_config(config_path);
    log::info!("✅ Configuration loaded successfully.");

    if cfg.proxies.is_empty() {
        log::warn!("⚠️ No proxy rules defined in the configuration.");
    }

    let config_arc = Arc::new(cfg);

    log::info!("🚀 Starting proxy server...");
    proxy::start_proxy(config_arc).await;

    log::info!("👋 Proxy server shut down.");
}