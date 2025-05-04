use hyper::{
    header::{HeaderName, HeaderValue},
    Body,
    Client,
    Request,
    Response,
    Server,
    StatusCode, // Use specific status code
    Uri,
};
use hyper::service::{make_service_fn, service_fn};
use std::{collections::HashMap, convert::Infallible, net::SocketAddr, sync::Arc, str};
use serde::Serialize;

use crate::{config, wasm_engine};
#[derive(Serialize)]
struct WasmInput {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: String,
}


pub async fn start_proxy(config: Arc<config::Config>) {
    let make_svc = make_service_fn(move |_conn| {
        let cfg = config.clone();
        let config_inner = cfg.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_request(req, config_inner.clone())
            }))
        }
    });

    let host = std::env::var("RILOT_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("RILOT_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8080);

    let addr = SocketAddr::new(host.parse().expect("Invalid host"), port);

    println!("🚀 Rilot proxy starting at http://{}", addr);
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("❌ Server error: {}", e);
    }
}

fn simple_response(status: StatusCode, body: &'static str) -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .status(status)
        .header("Content-Type", "text/plain")
        .body(Body::from(body))
        .unwrap())
}

async fn handle_request(
    mut req: Request<Body>,
    config: Arc<config::Config>,
) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path().to_string();
    let method = req.method().clone();

    println!("➡️ Received request: {} {}", method, path);

    let matched_proxy = config.proxies.iter().find(|p| {
        match p.rule.r#type.as_str() {
            "exact" => path == p.rule.path,
            "contain" | _ => path.starts_with(&p.rule.path),
        }
    });

    let proxy_config = match matched_proxy {
        Some(p) => p,
        None => {
            println!("🚫 No matching proxy rule found for path: {}", path);
            return simple_response(StatusCode::NOT_FOUND, "Not Found: No matching proxy rule.");
        }
    };

    println!(
        "✅ Matched rule for '{}' to app '{}' ({})",
        path, proxy_config.app_name, proxy_config.app_uri
    );

    let mut target_uri_str = proxy_config.app_uri.clone(); // Base target

    let headers_map: HashMap<String, String> = req
        .headers()
        .iter()
        .filter_map(|(k, v)| {
            v.to_str().ok().map(|v_str| (k.as_str().to_string(), v_str.to_string()))
        })
        .collect();

    let body_bytes = match hyper::body::to_bytes(req.body_mut()).await {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("⚠️ Failed to read request body: {}", e);
            return simple_response(StatusCode::INTERNAL_SERVER_ERROR, "Error reading request body.");
        }
    };

    let body_str = String::from_utf8_lossy(&body_bytes).to_string();

    if let Some(wasm_file) = &proxy_config.override_file {
        println!("⚙️ Running Wasm override: {}", wasm_file);
        let wasm_input = WasmInput {
            method: method.to_string(),
            path: path.clone(),
            headers: headers_map.clone(),
            body: body_str.clone(),
        };

        let input_json = match serde_json::to_string(&wasm_input) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("⚠️ Failed to serialize input for Wasm: {}", e);
                return simple_response(StatusCode::INTERNAL_SERVER_ERROR, "Error preparing Wasm input.");
            }
        };

        match wasm_engine::run_modify_request(wasm_file, &input_json).await {
            Ok(out) => {
                println!("✅ Wasm execution successful. Output: {:?}", out);
                if let Some(new_target) = out.app_url {
                    println!("↪️ Overriding target URI to: {}", new_target);
                    target_uri_str = new_target;
                }

                for (k, v) in out.headers_to_update {
                    if let (Ok(name), Ok(value)) = (
                        HeaderName::from_bytes(k.as_bytes()),
                        HeaderValue::from_str(&v),
                    ) {
                        println!("Adding/Updating header: {} = {}", k, v);
                        req.headers_mut().insert(name, value);
                    } else {
                        eprintln!("⚠️ Invalid header from Wasm: {} = {}", k, v);
                    }
                }
                for k in out.headers_to_remove {
                    if let Ok(name) = HeaderName::from_bytes(k.as_bytes()) {
                        println!("Removing header: {}", k);
                        req.headers_mut().remove(name);
                    } else {
                        eprintln!("⚠️ Invalid header name to remove from Wasm: {}", k);
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Wasm execution failed: {}", e);
                return simple_response(StatusCode::INTERNAL_SERVER_ERROR, "Wasm override module failed.");
            }
        };
    }

    let final_path_and_query = match proxy_config.rewrite.as_str() {
        "strip" => {
            req.uri().path_and_query()
                .map(|pq| pq.as_str().strip_prefix(&proxy_config.rule.path).unwrap_or(pq.as_str()))
                .unwrap_or("")
        },
        _ => req.uri().path_and_query().map(|pq| pq.as_str()).unwrap_or(""),
    };

    let final_target_uri_str = format!(
        "{}{}",
        target_uri_str.trim_end_matches('/'),
        final_path_and_query
    );

    let final_uri = match Uri::try_from(&final_target_uri_str) {
        Ok(uri) => uri,
        Err(e) => {
            eprintln!("⚠️ Failed to construct final target URI '{}': {}", final_target_uri_str, e);
            return simple_response(StatusCode::INTERNAL_SERVER_ERROR, "Error constructing target URL.");
        }
    };

    println!("🚀 Forwarding request to: {}", final_uri);

    *req.uri_mut() = final_uri;
    *req.body_mut() = Body::from(body_bytes); // Use original bytes

    let client = Client::new();

    match client.request(req).await {
        Ok(backend_res) => {
            println!("✅ Received response from backend: {}", backend_res.status());
            Ok(backend_res)
        },
        Err(e) => {
            eprintln!("❌ Error forwarding request: {}", e);
            simple_response(StatusCode::BAD_GATEWAY, "Error connecting to upstream service.")
        }
    }
}