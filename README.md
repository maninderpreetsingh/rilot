Rilot

🚀 Rilot is a fast, lightweight, Rust-based reverse proxy with optional dynamic WebAssembly (WASM) overrides.Built for multi frontends and backend microservices — designed for speed, flexibility, and simplicity.

---

## ✨ Features

- ⚡ High-performance proxy built with [Hyper](https://hyper.rs/) and [Tokio](https://tokio.rs/)
- 🔥 Dynamic path-based routing (exact or contain match rules)
- 🔧 Optional WebAssembly (WASM) override per app route (inject custom header logic, app URL switching)
- 🛡️ Minimal memory footprint
- 📝 Fully customizable with simple `config.json`
- 📦 Docker-ready(coming soon)
- 🔒 Licensed under MIT (no liability, use at your own risk)

---

## 📦 Installation

git clone https://github.com/yourusername/rilot.git
cd rilot
cargo build --release


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