//! Browser worker host for simulation, compilation, hardcopy, and source imports.

#[cfg(target_arch = "wasm32")]
mod hardcopy;
#[cfg(target_arch = "wasm32")]
mod jit;
#[cfg(target_arch = "wasm32")]
mod model_import;

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

#[cfg(target_arch = "wasm32")]
pub fn run_rspice_ui_pdk_import_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    rspice_simulation::pdk::import::run_pdk_import_worker_request_value(value)
}
