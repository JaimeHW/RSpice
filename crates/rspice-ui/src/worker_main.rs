//! Dedicated browser worker WebAssembly entry image.
//!
//! Keeping these exports out of the interactive eframe image lets whole-
//! program optimization remove solver, compiler, and publication execution
//! paths from the UI instance. The worker likewise does not retain the GUI
//! entrypoint.

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = rspiceUiWasmJitProbeModule)]
pub fn rspice_ui_wasm_jit_probe_module() -> Result<Vec<u8>, wasm_bindgen::JsValue> {
    rspice_ui::rspice_ui_wasm_jit_probe_module().map_err(Into::into)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = rspiceUiWasmJitAbiVersion)]
pub fn rspice_ui_wasm_jit_abi_version() -> u32 {
    rspice_ui::rspice_ui_wasm_jit_abi_version()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = rspiceUiWasmJitEmitterVersion)]
pub fn rspice_ui_wasm_jit_emitter_version() -> u32 {
    rspice_ui::rspice_ui_wasm_jit_emitter_version()
}

// Capability exports bound directly into every generated model module.
//
// These are `no_mangle` and not `wasm_bindgen`: the worker builds each
// generated module's import object from `wasmExports.rspice_ui_wasm_jit_*`, so
// the symbols have to exist as raw WebAssembly exports under exactly these
// names. A `wasm_bindgen` export is reached only through the generated
// JavaScript, which would put a JS frame -- and, for the `i64` argument below,
// BigInt marshalling -- between a model's `exp()` and its implementation, on a
// path that runs thousands of times per device evaluation.

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
#[allow(clippy::too_many_arguments)]
pub extern "C" fn rspice_ui_wasm_jit_eval_op_v1(
    frame_offset: u32,
    opcode: i32,
    aux0: i32,
    aux1: i32,
    aux2: i64,
    operand0: f64,
    operand1: f64,
    operand2: f64,
    operand3: f64,
    operand4: f64,
) -> f64 {
    rspice_ui::rspice_ui_wasm_jit_eval_op_v1(
        frame_offset,
        opcode,
        aux0,
        aux1,
        aux2,
        operand0,
        operand1,
        operand2,
        operand3,
        operand4,
    )
}

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn rspice_ui_wasm_jit_math1_v1(opcode: i32, value: f64) -> f64 {
    rspice_ui::rspice_ui_wasm_jit_math1_v1(opcode, value)
}

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn rspice_ui_wasm_jit_math2_v1(opcode: i32, left: f64, right: f64) -> f64 {
    rspice_ui::rspice_ui_wasm_jit_math2_v1(opcode, left, right)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = prepareRspiceUiWasmJitProbe)]
pub fn prepare_rspice_ui_wasm_jit_probe() -> Result<u32, wasm_bindgen::JsValue> {
    rspice_ui::prepare_rspice_ui_wasm_jit_probe().map_err(Into::into)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = finishRspiceUiWasmJitProbe)]
pub fn finish_rspice_ui_wasm_jit_probe(
    frame_offset: u32,
    status: i32,
) -> Result<f64, wasm_bindgen::JsValue> {
    rspice_ui::finish_rspice_ui_wasm_jit_probe(frame_offset, status).map_err(Into::into)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = rspiceUiWasmJitSolverProbeArtifact)]
pub fn rspice_ui_wasm_jit_solver_probe_artifact()
-> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    rspice_ui::rspice_ui_wasm_jit_solver_probe_artifact()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = rspiceUiWasmJitRunSolverProbe)]
pub fn rspice_ui_wasm_jit_run_solver_probe() -> Result<f64, wasm_bindgen::JsValue> {
    rspice_ui::rspice_ui_wasm_jit_run_solver_probe().map_err(Into::into)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = rspiceUiWasmJitKernelProbeArtifact)]
pub fn rspice_ui_wasm_jit_kernel_probe_artifact()
-> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    rspice_ui::rspice_ui_wasm_jit_kernel_probe_artifact()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = rspiceUiWasmJitRunKernelProbe)]
pub fn rspice_ui_wasm_jit_run_kernel_probe() -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue>
{
    rspice_ui::rspice_ui_wasm_jit_run_kernel_probe().map_err(Into::into)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = runRspiceUiWorkerRequest)]
pub fn run_rspice_ui_worker_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    rspice_ui::run_rspice_ui_worker_request(value)
}

#[wasm_bindgen::prelude::wasm_bindgen(js_name = prepareRspiceUiWasmJitRequest)]
pub fn prepare_rspice_ui_wasm_jit_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    rspice_ui::prepare_rspice_ui_wasm_jit_request(value)
}

#[wasm_bindgen::prelude::wasm_bindgen(js_name = installRspiceUiWasmJitDispatcher)]
pub fn install_rspice_ui_wasm_jit_dispatcher(dispatcher: js_sys::Function) {
    rspice_ui::install_rspice_ui_wasm_jit_dispatcher(dispatcher);
}

#[wasm_bindgen::prelude::wasm_bindgen(js_name = runPreparedRspiceUiWasmJitRequest)]
pub fn run_prepared_rspice_ui_wasm_jit_request(
    dispatch_token: u32,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    rspice_ui::run_prepared_rspice_ui_wasm_jit_request(dispatch_token)
}

#[wasm_bindgen::prelude::wasm_bindgen(js_name = cancelPreparedRspiceUiWasmJitRequest)]
pub fn cancel_prepared_rspice_ui_wasm_jit_request(
    dispatch_token: u32,
) -> Result<(), wasm_bindgen::JsValue> {
    rspice_ui::cancel_prepared_rspice_ui_wasm_jit_request(dispatch_token)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = runRspiceUiVerilogACompileRequest)]
pub fn run_rspice_ui_veriloga_compile_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    rspice_ui::run_rspice_ui_veriloga_compile_request(value)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = runRspiceUiHardcopyRequest)]
pub fn run_rspice_ui_hardcopy_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    rspice_ui::run_rspice_ui_hardcopy_request(value)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = runRspiceUiModelImportRequest)]
pub fn run_rspice_ui_model_import_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    rspice_ui::run_rspice_ui_model_import_request(value)
}

fn main() {}
