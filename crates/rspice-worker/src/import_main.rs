//! The browser's import worker WebAssembly entry image: the page starts it when an import starts,
//! and it reaches only the reader of picked result files.

/// Import the picked `file`, named `name`, under the page's `policy` (JSON): see
/// [`rspice_worker::import_result_file`].
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = importRspiceResultFile)]
pub async fn import_rspice_result_file(
    file: web_sys::Blob,
    name: String,
    policy: String,
    store: js_sys::Function,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    rspice_worker::import_result_file(file, name, policy, store).await
}

fn main() {}
