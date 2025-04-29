use wasmtime::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WasmOutput {
    pub app_url: Option<String>,
    pub headers_to_update: Option<std::collections::HashMap<String, String>>,
    pub headers_to_remove: Option<Vec<String>>,
}

pub fn run_modify_request(wasm_path: &str, input: &str) -> Result<WasmOutput, String> {
    let engine: Engine = Engine::default();
    let module: Module = Module::from_file(&engine, wasm_path).map_err(|e| e.to_string())?;
    let mut store: Store<()> = Store::new(&engine, ());
    let instance: Instance = Instance::new(&mut store, &module, &[]).map_err(|e| e.to_string())?;

    let func: TypedFunc<(i32, i32), i32> = instance.get_typed_func::<(i32, i32), i32>(&mut store, "modify_request")
        .map_err(|e| e.to_string())?;

    let memory: Memory = instance.get_memory(&mut store, "memory")
        .ok_or_else(|| "failed to find memory export".to_string())?;

    let input_bytes: &[u8] = input.as_bytes();
    let ptr: usize = 0;
    memory.write(&mut store, ptr, input_bytes).map_err(|e| e.to_string())?;

    let result_ptr: i32 = func.call(&mut store, (ptr as i32, input_bytes.len() as i32))
        .map_err(|e| e.to_string())?;

    let mut buffer: [u8; 4096] = [0u8; 4096];
    memory.read(&mut store, result_ptr as usize, &mut buffer).map_err(|e| e.to_string())?;
    let nul_pos = buffer.iter().position(|&b| b == 0).unwrap_or(buffer.len());

    let output: String = String::from_utf8_lossy(&buffer[..nul_pos]).to_string();
    serde_json::from_str(&output).map_err(|e| e.to_string())
}
