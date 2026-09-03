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

/// Tests that exercise the real JavaScript boundary.
///
/// These run under `wasm-bindgen-test` on Node. Everything they assert is a
/// property of the boundary itself -- typed-array columns, explicit `null`,
/// the thrown error's shape, and the shared control word -- which is exactly
/// what a native test cannot observe.
#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;
    use crate::js_interop::{js_array_property, js_property};

    const DECK: &str = "browser boundary deck\n\
V1 in 0 PULSE(0 1 0 1n 1n 20n 40n) AC 1\n\
R1 in out 1k\n\
C1 out 0 1p\n\
.TRAN 1n 20n\n\
.END\n";

    /// An options object carrying only a `sharedInt32` control word.
    fn shared_cancellation_options(cancelled: bool) -> JsValue {
        let buffer = js_sys::SharedArrayBuffer::new(4);
        let view = js_sys::Int32Array::new(buffer.as_ref());
        js_sys::Atomics::store(&view, 0, i32::from(cancelled))
            .expect("Node supports Atomics.store on SharedArrayBuffer");

        let cancellation = js_sys::Object::new();
        js_sys::Reflect::set(
            &cancellation,
            &JsValue::from_str("mechanism"),
            &JsValue::from_str("sharedInt32"),
        )
        .expect("set cancellation mechanism");
        js_sys::Reflect::set(&cancellation, &JsValue::from_str("view"), &view)
            .expect("set cancellation view");

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &JsValue::from_str("cancellation"), &cancellation)
            .expect("set cancellation policy");
        options.into()
    }

    fn error_field(error: &JsValue, name: &str) -> Option<String> {
        js_property(error, name)
            .expect("RSpiceError exposes its structured fields")
            .as_string()
    }

    /// Window columns cross the boundary as typed arrays, and every signal
    /// carries a `Uint8Array` validity mask beside its numbers.
    #[wasm_bindgen_test]
    fn window_columns_cross_the_boundary_as_typed_arrays() {
        let handle = run_authored_deck_document_js(DECK, JsValue::UNDEFINED)
            .expect("the authored deck runs through the JavaScript export");
        let window = handle
            .read_window_js(0, 0, 4)
            .expect("a four-point window is inside the transient");

        let axes = js_array_property(&window, "axes").expect("the window publishes its axes");
        let axis = axes.get(0);
        assert!(
            js_property(&axis, "values")
                .expect("an axis publishes its coordinates")
                .is_instance_of::<js_sys::Float64Array>(),
            "axis coordinates must be a Float64Array"
        );

        let signals =
            js_array_property(&window, "signals").expect("the window publishes its signals");
        let values =
            js_property(&signals.get(0), "values").expect("a signal publishes its samples");
        assert!(
            js_property(&values, "validity")
                .expect("a signal publishes its validity mask")
                .is_instance_of::<js_sys::Uint8Array>(),
            "the validity mask must be a Uint8Array"
        );
        let representation = js_property(&values, "representation")
            .expect("a signal declares its representation")
            .as_string();
        match representation.as_deref() {
            Some("real") => assert!(
                js_property(&values, "values")
                    .expect("a real signal publishes its samples")
                    .is_instance_of::<js_sys::Float64Array>()
            ),
            Some("complex") => {
                for column in ["real", "imaginary"] {
                    assert!(
                        js_property(&values, column)
                            .expect("a complex signal publishes both columns")
                            .is_instance_of::<js_sys::Float64Array>()
                    );
                }
            }
            other => panic!("unexpected sample representation {other:?}"),
        }
    }

    /// Optional metadata is published as explicit `null`, never omitted, so a
    /// consumer can tell absence from a field this build forgot to write.
    #[wasm_bindgen_test]
    fn absent_metadata_is_explicit_null() {
        let handle = run_authored_deck_document_js(DECK, JsValue::UNDEFINED)
            .expect("the authored deck runs through the JavaScript export");
        let metadata = handle
            .result_metadata_js(0)
            .expect("result metadata crosses the boundary");
        assert!(
            js_property(&metadata, "parentAnalysis")
                .expect("the metadata declares its parent analysis")
                .is_null(),
            "a transient has no parent analysis and must say so explicitly"
        );
        let payload = js_property(&metadata, "payload").expect("the metadata declares its payload");
        assert!(
            js_property(&payload, "compression")
                .expect("the payload declares its compression certificate")
                .is_null(),
            "an uncompressed run must publish a null certificate, not omit it"
        );
    }

    /// A control word that is already set cancels the export before it
    /// publishes anything, and the thrown error is the documented one.
    #[wasm_bindgen_test]
    fn a_pre_set_control_word_cancels_the_export() {
        let error = run_authored_deck_document_js(DECK, shared_cancellation_options(true))
            .expect_err("a pre-set control word must cancel the run");
        assert_eq!(error_field(&error, "code").as_deref(), Some("aborted"));
        assert_eq!(
            error_field(&error, "category").as_deref(),
            Some("cancellation")
        );
        assert_eq!(
            js_property(&error, "retryable")
                .expect("RSpiceError declares its retry policy")
                .as_bool(),
            Some(true)
        );
    }

    /// A clear control word does not cancel anything.
    #[wasm_bindgen_test]
    fn a_clear_control_word_runs_to_completion() {
        let handle = run_authored_deck_document_js(DECK, shared_cancellation_options(false))
            .expect("a clear control word must not cancel the run");
        assert!(handle.result_count() > 0);
    }

    /// Cancellation controls this build cannot honour are rejected before any
    /// work, rather than accepted and then ignored.
    #[wasm_bindgen_test]
    fn unhonourable_cancellation_controls_are_rejected() {
        let abort_signal = js_sys::Object::new();
        js_sys::Reflect::set(
            &abort_signal,
            &JsValue::from_str("mechanism"),
            &JsValue::from_str("abortSignal"),
        )
        .expect("set cancellation mechanism");
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &JsValue::from_str("cancellation"), &abort_signal)
            .expect("set cancellation policy");
        let error = run_authored_deck_document_js(DECK, options.into())
            .expect_err("a DOM AbortSignal cannot interrupt a synchronous call");
        assert_eq!(
            error_field(&error, "code").as_deref(),
            Some("unsupported_cancellation")
        );

        // An ordinary ArrayBuffer is not shared memory, so the caller could
        // never signal through it.
        let view = js_sys::Int32Array::new(&js_sys::ArrayBuffer::new(4).into());
        let cancellation = js_sys::Object::new();
        js_sys::Reflect::set(
            &cancellation,
            &JsValue::from_str("mechanism"),
            &JsValue::from_str("sharedInt32"),
        )
        .expect("set cancellation mechanism");
        js_sys::Reflect::set(&cancellation, &JsValue::from_str("view"), &view)
            .expect("set cancellation view");
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &JsValue::from_str("cancellation"), &cancellation)
            .expect("set cancellation policy");
        let error = run_authored_deck_document_js(DECK, options.into())
            .expect_err("an ordinary ArrayBuffer must be rejected");
        assert_eq!(
            error_field(&error, "code").as_deref(),
            Some("invalid_argument")
        );
    }

    /// A refused family throws the typed refusal, naming the card and the
    /// missing core API, and publishes no handle.
    #[wasm_bindgen_test]
    fn a_refused_family_throws_the_typed_refusal() {
        let source = "browser refusal deck\n\
V1 in 0 AC 1\n\
R1 in out 1k\n\
.SP DEC 3 1k 100k\n\
.END\n";
        let error = run_authored_deck_document_js(source, JsValue::UNDEFINED)
            .expect_err("an unroutable family must be refused");
        assert_eq!(
            error_field(&error, "code").as_deref(),
            Some("unsupported_deck_analysis")
        );
        assert_eq!(
            error_field(&error, "category").as_deref(),
            Some("unsupported_feature")
        );
        let message = error_field(&error, "message").unwrap_or_default();
        assert!(
            message.contains(".SP") && message.contains("SParameterResult"),
            "the refusal names the card and the missing core API: {message}"
        );
    }
}
