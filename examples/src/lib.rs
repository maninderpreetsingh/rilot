use serde::Serialize;

#[derive(Serialize)]
struct WasmResponse {
    app_url: Option<String>,
    headers_to_update: std::collections::HashMap<String, String>,
    headers_to_remove: Vec<String>,
}

#[no_mangle]
pub extern "C" fn modify_request(ptr: *mut u8, len: usize) -> i32 {
    unsafe {
        let input = std::slice::from_raw_parts(ptr, len);
        let input_str = std::str::from_utf8(input).unwrap();

        let request_json: serde_json::Value = serde_json::from_str(input_str).unwrap();

        let path = request_json["path"].as_str().unwrap_or("/");
        let method = request_json["method"].as_str().unwrap_or("GET");

        let mut headers = std::collections::HashMap::new();
        headers.insert("X-Debug-Path".to_string(), path.to_string());
        headers.insert("X-Debug-Method".to_string(), method.to_string());
        headers.insert("X-Via-Rilot".to_string(), "5".to_string());

        let response = WasmResponse {
            app_url: None, // change to any backend dynamically if needed else default based on config.json
            headers_to_update: headers,
            headers_to_remove: vec![],
        };

        let output_json = serde_json::to_string(&response).unwrap();

        let output_bytes = output_json.as_bytes();
        let out_ptr = 512;

        // First clear memory (optional, to avoid trailing garbage)
        let out_slice_full = std::slice::from_raw_parts_mut(ptr.offset(out_ptr as isize), 4096);
        for byte in out_slice_full.iter_mut() {
            *byte = 0;
        }

        // Now copy real output
        let out_slice = std::slice::from_raw_parts_mut(ptr.offset(out_ptr as isize), output_bytes.len());
        out_slice.copy_from_slice(output_bytes);

        out_ptr
    }
}
