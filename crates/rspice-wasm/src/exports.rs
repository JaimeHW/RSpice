//! The `#[wasm_bindgen]` export shims.
//!
//! Every export opens one [`ExecutionScope`], which decodes the options
//! object, installs the cancellation control, and starts the deadline, then
//! calls the corresponding Rust entry point with that scope's policy and
//! abort source. No export contains analysis logic and none of them can
//! reach a non-abort core entrypoint.
//!
//! Every analysis export returns a [`WasmResultHandle`]. There is no export
//! that copies a whole result into ordinary JavaScript arrays.

use wasm_bindgen::prelude::*;

use crate::DetailedWasmResult;
use crate::abort::ExecutionScope;
use crate::errors::wasm_error_to_js;
use crate::handles::WasmResultHandle;
use crate::js_interop::serialize_to_js;
use crate::options::WasmResourceLimits;
use crate::runners::deck::{
    DeckExecution, run_authored_deck_document_with_options_and_abort_detailed,
};
use crate::runners::direct::{
    health_check_with_options_and_abort_detailed, run_ac_document_with_options_and_abort_detailed,
    run_dc_sweep_document_with_options_and_abort_detailed,
    run_noise_document_with_options_and_abort_detailed,
    run_operating_point_document_with_options_and_abort_detailed,
    run_transient_document_with_options_and_abort_detailed,
    summarize_netlist_with_options_and_abort_detailed,
};

/// Open the shared per-call scope, projecting any failure into `RSpiceError`.
fn scope(options: JsValue) -> Result<ExecutionScope, JsValue> {
    ExecutionScope::open(options).map_err(|error| wasm_error_to_js(*error))
}

/// Project a typed runner failure into the thrown `RSpiceError`.
fn thrown<T>(outcome: DetailedWasmResult<T>) -> Result<T, JsValue> {
    outcome.map_err(|error| wasm_error_to_js(*error))
}

/// Retain one execution's results behind the shared bounded handle.
fn retain(scope: &ExecutionScope, execution: DeckExecution) -> Result<WasmResultHandle, JsValue> {
    thrown(WasmResultHandle::new(
        &execution.plan,
        execution.coordinates,
        execution.results,
        scope.resource_limits(),
    ))
}

#[wasm_bindgen(js_name = defaultResourceLimits)]
pub fn default_resource_limits_js() -> Result<JsValue, JsValue> {
    serialize_to_js(&WasmResourceLimits::default())
}

#[wasm_bindgen(js_name = healthCheck)]
pub fn health_check_js(options: JsValue) -> Result<JsValue, JsValue> {
    let scope = scope(options)?;
    let report = thrown(health_check_with_options_and_abort_detailed(
        scope.options(),
        &scope.abort(),
    ))?;
    serialize_to_js(&report)
}

#[wasm_bindgen(js_name = summarizeNetlist)]
pub fn summarize_netlist_js(source: &str, options: JsValue) -> Result<JsValue, JsValue> {
    let scope = scope(options)?;
    let summary = thrown(summarize_netlist_with_options_and_abort_detailed(
        source,
        scope.options(),
        &scope.abort(),
    ))?;
    serialize_to_js(&summary)
}

#[wasm_bindgen(js_name = runOperatingPointDocument)]
pub fn run_operating_point_document_js(
    source: &str,
    options: JsValue,
) -> Result<WasmResultHandle, JsValue> {
    let scope = scope(options)?;
    let execution = thrown(
        run_operating_point_document_with_options_and_abort_detailed(
            source,
            scope.options(),
            &scope.abort(),
        ),
    )?;
    retain(&scope, execution)
}

#[wasm_bindgen(js_name = runDcSweepDocument)]
pub fn run_dc_sweep_document_js(
    source: &str,
    source_name: &str,
    start: f64,
    stop: f64,
    step: f64,
    options: JsValue,
) -> Result<WasmResultHandle, JsValue> {
    let scope = scope(options)?;
    let execution = thrown(run_dc_sweep_document_with_options_and_abort_detailed(
        source,
        source_name,
        start,
        stop,
        step,
        scope.options(),
        &scope.abort(),
    ))?;
    retain(&scope, execution)
}

#[wasm_bindgen(js_name = runAcAnalysisDocument)]
pub fn run_ac_document_js(
    source: &str,
    frequencies: Vec<f64>,
    options: JsValue,
) -> Result<WasmResultHandle, JsValue> {
    let scope = scope(options)?;
    let execution = thrown(run_ac_document_with_options_and_abort_detailed(
        source,
        &frequencies,
        scope.options(),
        &scope.abort(),
    ))?;
    retain(&scope, execution)
}

#[wasm_bindgen(js_name = runTransientAnalysisDocument)]
pub fn run_transient_document_js(
    source: &str,
    tstop: f64,
    max_step: f64,
    options: JsValue,
) -> Result<WasmResultHandle, JsValue> {
    let scope = scope(options)?;
    let execution = thrown(run_transient_document_with_options_and_abort_detailed(
        source,
        tstop,
        max_step,
        scope.options(),
        &scope.abort(),
    ))?;
    retain(&scope, execution)
}

#[wasm_bindgen(js_name = runNoiseAnalysisDocument)]
pub fn run_noise_document_js(
    source: &str,
    output_node: &str,
    reference_node: Option<String>,
    input_source: &str,
    frequencies: Vec<f64>,
    options: JsValue,
) -> Result<WasmResultHandle, JsValue> {
    let scope = scope(options)?;
    let execution = thrown(run_noise_document_with_options_and_abort_detailed(
        source,
        output_node,
        reference_node.as_deref(),
        input_source,
        &frequencies,
        scope.options(),
        &scope.abort(),
    ))?;
    retain(&scope, execution)
}

/// Execute a complete authored analog deck, including its canonical
/// DATA/STEP/TEMP axes, and retain every coordinate-local result behind
/// bounded windows.
#[wasm_bindgen(js_name = runAuthoredDeckDocument)]
pub fn run_authored_deck_document_js(
    source: &str,
    options: JsValue,
) -> Result<WasmResultHandle, JsValue> {
    let scope = scope(options)?;
    let execution = thrown(run_authored_deck_document_with_options_and_abort_detailed(
        source,
        scope.options(),
        &scope.abort(),
    ))?;
    retain(&scope, execution)
}
