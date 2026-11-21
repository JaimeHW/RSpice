//! Browser worker host for simulation, model qualification, compilation,
//! hardcopy, source imports, and this browser's results stores; and the
//! import worker's reader of picked result files, which only its own entry
//! image reaches.

#[cfg(target_arch = "wasm32")]
mod hardcopy;
#[cfg(target_arch = "wasm32")]
mod import;
#[cfg(target_arch = "wasm32")]
mod jit;
#[cfg(target_arch = "wasm32")]
mod model_import;
#[cfg(target_arch = "wasm32")]
mod model_qualification;
#[cfg(target_arch = "wasm32")]
mod results_store;

#[cfg(target_arch = "wasm32")]
pub use import::import_result_file;
#[cfg(target_arch = "wasm32")]
pub use jit::*;

#[cfg(target_arch = "wasm32")]
pub fn run_rspice_ui_worker_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    rspice_simulation::runner::run_worker_request_value(value)
}

#[cfg(target_arch = "wasm32")]
pub fn prepare_rspice_ui_wasm_jit_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    rspice_simulation::runner::prepare_wasm_jit_request_value(value)
}

#[cfg(target_arch = "wasm32")]
pub fn run_prepared_rspice_ui_wasm_jit_request(
    dispatch_token: u32,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    rspice_simulation::runner::run_prepared_wasm_jit_request_value(dispatch_token)
}

#[cfg(target_arch = "wasm32")]
pub fn cancel_prepared_rspice_ui_wasm_jit_request(
    dispatch_token: u32,
) -> Result<(), wasm_bindgen::JsValue> {
    rspice_simulation::runner::cancel_prepared_wasm_jit_request_value(dispatch_token)
}

#[cfg(target_arch = "wasm32")]
pub fn run_rspice_ui_veriloga_compile_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    rspice_simulation::project_veriloga::compile_service::run_worker_request_value(
        value,
        rspice_simulation::project_veriloga::worker::WorkerCompileTarget::WasmJit,
    )
}

#[cfg(target_arch = "wasm32")]
pub fn run_rspice_ui_hardcopy_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    hardcopy::run_worker_request_value(value)
}

#[cfg(target_arch = "wasm32")]
pub fn run_rspice_ui_model_import_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    model_import::run_worker_request_value(value)
}

/// Answer a page's request of this browser's results stores: its JSON and the numeric buffers
/// beside it, answered with `{ response, buffers }`.
#[cfg(target_arch = "wasm32")]
pub async fn run_rspice_ui_results_store_request(
    request: String,
    buffers: js_sys::Array,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    results_store::run_request(request, buffers).await
}

#[cfg(target_arch = "wasm32")]
pub fn run_rspice_ui_model_qualification_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    model_qualification::run_worker_request_value(value)
}
