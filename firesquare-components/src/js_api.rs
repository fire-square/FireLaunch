use wasm_bindgen::prelude::*;

/// This is a wrapper around the Tauri APIs.
#[wasm_bindgen]
extern "C" {
	/// Invoke a Tauri command.
	#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
	pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// This is a wrapper around the standard browser APIs.
#[wasm_bindgen]
extern "C" {
	/// Log a message to the browser console.
	#[wasm_bindgen(js_namespace = console)]
	pub fn log(s: &str);
}
