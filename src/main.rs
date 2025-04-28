use std::sync::Arc;
use std::env;

mod config;
mod proxy;
mod wasm_engine;

#[tokio::main]
async fn main() {
    /*Config file is needed */
    let args: Vec<String> = env::args().collect();
    let config_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "./config.json".to_string()
    };

    println!("🛠️ Loading config from: {}", config_path);

    let cfg = config::load_config(&config_path);
    let config = Arc::new(cfg);

    proxy::start_proxy(config).await;
}
