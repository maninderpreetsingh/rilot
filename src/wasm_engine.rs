use serde::Deserialize;
use std::collections::HashMap;
use std::sync::RwLock;
use once_cell::sync::Lazy;
use wasmtime::{Engine, Linker, Memory, Module, Store, TypedFunc};

#[derive(Debug, Deserialize)]
pub struct WasmOutput {
    pub app_url: Option<String>,
    pub headers_to_update: Option<HashMap<String, String>>,
    pub headers_to_remove: Option<Vec<String>>,
}

// Static shared engine and cache
static WASM_ENGINE: Lazy<Engine> = Lazy::new(|| Engine::default());
static WASM_CACHE: Lazy<RwLock<HashMap<String, Module>>> = Lazy::new(|| RwLock::new(HashMap::new()));

pub fn run_modify_request(wasm_path: &str, input: &str) -> Result<WasmOutput, String> {
    let mode = std::env::var("RILOT_MODE").unwrap_or_else(|_| "development".to_string());

    let module = if mode == "production" {
        let mut cache = WASM_CACHE.write().unwrap();
        if let Some(m) = cache.get(wasm_path) {
            m.clone()
        } else {
            let m = Module::from_file(&*WASM_ENGINE, wasm_path).map_err(|e| e.to_string())?;
            cache.insert(wasm_path.to_string(), m.clone());
            m
        }
    } else {
        Module::from_file(&*WASM_ENGINE, wasm_path).map_err(|e| e.to_string())?
    };

    let mut store = Store::new(&*WASM_ENGINE, ());
    let mut linker = Linker::new(&*WASM_ENGINE);

    let instance = linker.instantiate(&mut store, &module).map_err(|e| e.to_string())?;

    let func = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "modify_request")
        .map_err(|e| e.to_string())?;

    let memory = instance
        .get_memory(&mut store, "memory")
        .ok_or_else(|| "Memory export not found".to_string())?;

    let input_bytes = input.as_bytes();
    let ptr = 0;
    memory
        .write(&mut store, ptr, input_bytes)
        .map_err(|e| e.to_string())?;

    let result_ptr = func
        .call(&mut store, (ptr as i32, input_bytes.len() as i32))
        .map_err(|e| e.to_string())?;

    let mut buffer = vec![0u8; 4096];
    memory
        .read(&mut store, result_ptr as usize, &mut buffer)
        .map_err(|e| e.to_string())?;

    let nul_pos = buffer.iter().position(|&b| b == 0).unwrap_or(buffer.len());
    let output = String::from_utf8_lossy(&buffer[..nul_pos]).to_string();

    serde_json::from_str(&output).map_err(|e| e.to_string())
}
