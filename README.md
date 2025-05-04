# Rilot - A Configurable Reverse Proxy with Wasm Overrides

⚡ Fast, lightweight, and pluggable reverse proxy with WebAssembly (WASM) overrides. Built with ❤️ in Rust for microservices, frontend multi-zone architectures, and blazing edge performance.

## Core Features

* **Configurable Routing:** Define backends and path-based routing rules (`contain` / `exact` match) in `config.json`.
* **Wasm Overrides:** Specify a Wasm component (`.wasm`) per rule to execute custom logic.
* **Dynamic Modification:** Wasm modules can alter target URLs, modify request/response headers, and make external HTTP(S) calls.
* **WASI & Component Model:** Uses WASI Preview 2 and the Component Model for host-guest interaction (currently via piped stdio).
* **Performance:** Built on Tokio/Hyper.
* **Conditional Wasm Loading:**
    * **Development Mode (default):** Wasm modules are reloaded on each request for live updates ("hot-reloading").
    * **Production Mode (`RILOT_ENV=production`):** Compiled Wasm components are cached after first use for improved performance.

## Configuration (`config.json`)

Define proxy rules and optional Wasm overrides:

```json
{
  "proxies": [
    {
      "app_name": "My API Service",
      "app_uri": "http://backend-service:8080",
      "override_file": "/path/to/your/override.wasm", // Optional Wasm component
      "rewrite": "strip", // Optional: "none" or "strip"
      "rule": {
        "path": "/api/",
        "type": "contain" // "contain" or "exact"
      }
    },
    {
      "app_name": "Static Files",
      "app_uri": "http://static-server:80/",
      "override_file": null, // No override
      "rewrite": "none",
      "rule": {
        "path": "/static/",
        "type": "contain"
      }
    }
  ]
}```


## Running
### Development (Wasm recompiled)
- RUST_LOG=debug ./target/debug/rilot config.json

### Production (Wasm cached)
- RILOT_ENV=production RUST_LOG=info ./target/release/rilot config.json

### Use default ./config.json if path omitted
### ./target/release/rilot


- Set RILOT_ENV=production to enable Wasm caching.
- Set RUST_LOG (e.g., debug, info) for logging level.
- Set RILOT_HOST / RILOT_PORT to change listen address (defaults 127.0.0.1:8080).

## Custom Overrides
Use the examples directory as a template. Create a Rust library project, define your WIT interface, configure Cargo.toml, implement the logic in lib.rs, and build using `cargo component


---

## License

This project is licensed under the MIT License.


## 🙏 Acknowledgements

- Built with ❤️ in Rust

- Inspired by production-grade proxies like Cloudflare Workers, Vercel Edge Runtime and Fastly compute Edge

- Powered by Hyper, Tokio, and Wasmtime