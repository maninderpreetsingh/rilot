use hyper::{Body, Client, Request, Response, Server, Uri};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use crate::{config, wasm_engine};
use std::str;
use std::sync::Arc;
use serde::Serialize;

#[derive(Serialize)]
struct WasmInput {
    method: String,
    path: String,
    headers: std::collections::HashMap<String, String>,
    body: String,
}

pub async fn start_proxy(config: Arc<config::Config>) {
    let make_svc = make_service_fn(move |_conn| {
        let config = config.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| handle_request(req, config.clone())))
        }
    });

    let host = std::env::var("RILOT_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("RILOT_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8080);

    let addr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid host or port");

    let server = Server::bind(&addr).serve(make_svc);

    println!("🚀 Rilot proxy running at http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn handle_request(mut req: Request<Body>, config: Arc<config::Config>) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path().to_string();

    // Find matching proxy
    let matched_proxy = config.proxies.iter().find(|proxy| {
        match proxy.rule.r#type.as_str() {
            "exact" => path == proxy.rule.path,
            "contain" => path.starts_with(&proxy.rule.path),
            _ => false,
        }
    });

    if let Some(proxy) = matched_proxy {
        let mut target_backend = proxy.app_uri.clone();

        // Prepare wasm input
        let whole_body = hyper::body::to_bytes(req.body_mut()).await.unwrap_or_default();
        let body_str = match str::from_utf8(&whole_body) {
            Ok(v) => v,
            Err(_) => "",
        };

        let headers_map = req.headers()
            .iter()
            .filter_map(|(k, v)| {
                Some((k.as_str().to_string(), v.to_str().ok()?.to_string()))
            })
            .collect::<std::collections::HashMap<String, String>>();

        let wasm_input = WasmInput {
            method: req.method().to_string(),
            path: path.clone(),
            headers: headers_map,
            body: body_str.to_string(),
        };

        let wasm_input_json = serde_json::to_string(&wasm_input).unwrap();

        // Run wasm override if exists
        if let Some(ref override_file) = proxy.override_file {
            let wasm_output = match wasm_engine::run_modify_request(override_file, &wasm_input_json) {
                Ok(output) => output,
                Err(e) => {
                    println!("⚠️ WASM execution failed: {}", e);
                    wasm_engine::WasmOutput {
                        app_url: None,
                        headers_to_update: None,
                        headers_to_remove: None,
                    }
                }
            };

            // Override backend if needed
            if let Some(app_url) = wasm_output.app_url {
                target_backend = app_url;
            }

            // Update headers
            if let Some(header_map) = wasm_output.headers_to_update {
                for (key, value) in header_map.iter() {
                    if let (Ok(header_name), Ok(header_value)) = (
                        hyper::header::HeaderName::from_bytes(key.as_bytes()),
                        hyper::header::HeaderValue::from_str(value),
                    ) {
                        req.headers_mut().insert(header_name, header_value);
                    }
                }
            }

            // Remove headers
            if let Some(headers) = wasm_output.headers_to_remove {
                for key in headers {
                    if let Ok(header_name) = hyper::header::HeaderName::from_bytes(key.as_bytes()) {
                        req.headers_mut().remove(header_name);
                    }
                }
            }

            // TODO:: update body?
        }

        // Direct forwarding
        let method = req.method().clone();
        let headers = req.headers().clone();
        let body = req.into_body();

        let backend_uri = format!("{}{}", target_backend, path)
            .parse::<Uri>()
            .expect("Failed to build backend URI");

        let mut new_req = Request::builder()
            .method(method)
            .uri(backend_uri)
            .body(body)
            .expect("Failed to build new request");

        *new_req.headers_mut() = headers;

        let client = Client::new();
        let resp_result = client.request(new_req).await;

        match resp_result {
            Ok(mut backend_resp) => {
                let backend_body = hyper::body::to_bytes(backend_resp.body_mut()).await.unwrap_or_default();
                Ok(Response::builder()
                    .status(backend_resp.status())
                    .body(Body::from(backend_body))
                    .expect("Failed to build final response"))
            }
            Err(_) => Ok(Response::builder()
                .status(502)
                .body(Body::from("Bad Gateway"))
                .unwrap()),
        }
    } else {
        Ok(Response::builder()
            .status(404)
            .body(Body::from("Not Found"))
            .unwrap())
    }
}
