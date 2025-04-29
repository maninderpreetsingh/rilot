# Rilot

⚡ Fast, lightweight, and pluggable reverse proxy with WebAssembly (WASM) overrides.
Built with ❤️ in Rust for microservices, frontend multi-zone architectures, and blazing edge performance.

---

## ✨ Features

- 🚀 Hot-reload WebAssembly overrides (no restart needed)
- 🛡️ Minimal memory proxy with Hyper + Tokio
- 🔥 Per-path dynamic routing (`contain` / `exact` match)
- 🔄 Seamless header manipulation without backend code changes
- 📝 Fully customizable with simple `config.json`
- ⚡ Ultra-fast cold start and live updates
- 🛠️ Built-in Docker support(coming soon)
- 🔒 MIT Licensed (no liability, use at your own risk)


---

## 🛠️ How it Works

Rilot acts as a **frontdoor proxy**,
Routing based on URL paths,
Injecting WebAssembly (WASM) modules dynamically to modify behavior without server restart.

```plaintext
[User Request]
     ↓
[Rilot Proxy] ──(optional WASM logic)──> [App]
```

✅ Simple.
✅ Flexible.
✅ Powerful.

---

## 📦 Installation

### Using NPX (coming soon)

```bash
npx rilot
```

or install globally (coming soon):

```bash
npm install -g rilot
```

### Manual (Cargo)

```bash
git clone https://github.com/maninderpreetsingh/rilot.git
cd rilot
cargo build --release
```

---

## 🚀 Quick Start Example

1. Create a folder `my_app/`

```plaintext
my_app/
 ├── config.json
 ├── Dockerfile (optional) -> if you want to deploy docker container
 └── runtime/override_sample.wasm (optional) -> build wasm with (AssemblyScript / Rust / )
```

2. Example `config.json`:

```json
{
    "proxies": [
        {
            "app_name": "App 1",
            "app_uri": "http://127.0.0.1:5502",
            "override_file": "/path/to/override.wasm",
            "rule": {
                "path": "/",
                "type": "exact"
            }
        },
        {
            "app_name": "App 2",
            "app_uri": "http://127.0.0.1:5501/",
            "rule": {
                "path": "/app2",
                "type": "contain"
            }
        }
    ]
}
```

3. Run Rilot:

```bash
cargo run ./my_app/config.json
```

✅ Your proxy server will start at `http://127.0.0.1:8080`!

---

## ⚙️ Configuration Explained

- `app_name`: Friendly name for your service
- `app_uri`: Target backend URL
- `override_file`: Optional WebAssembly module to override headers / routing
- `rule.path`: URL path to match
- `rule.type`: `"exact"` or `"contain"`

✅ No complicated config — simple and powerful.

---

## 🔥 Live Hot-Reload of Overrides

Every request dynamically loads the `.wasm` file!
✅ No server restart needed
✅ Modify your override logic live
✅ Instant effect on next request

---

## License

This project is licensed under the MIT License.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.

IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY.


## 🙏 Acknowledgements

- Built with ❤️ in Rust

- Inspired by production-grade proxies like Cloudflare Workers, Vercel Edge Runtime and Fastly compute Edge

- Powered by Hyper, Tokio, and Wasmtime