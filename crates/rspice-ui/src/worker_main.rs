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

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = rspiceUiWasmJitEvalOpV1)]
#[allow(clippy::too_many_arguments)]
pub fn rspice_ui_wasm_jit_eval_op_v1(
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

/// Raw capability exports bound directly into generated modules.
///
/// The worker passes `wasmExports.rspice_ui_wasm_jit_math1_v1` rather than the
/// generated JavaScript wrapper, so a model's `exp()` is a wasm-to-wasm import
/// call. The `js_name` wrappers exist only so the worker can assert the
/// symbols are present before it instantiates anything.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = rspiceUiWasmJitMath1V1)]
pub fn rspice_ui_wasm_jit_math1_v1(opcode: i32, value: f64) -> f64 {
    rspice_ui::rspice_ui_wasm_jit_math1_v1(opcode, value)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = rspiceUiWasmJitMath2V1)]
pub fn rspice_ui_wasm_jit_math2_v1(opcode: i32, left: f64, right: f64) -> f64 {
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

fn main() {}
