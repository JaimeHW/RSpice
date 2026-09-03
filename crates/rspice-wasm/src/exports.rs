//! The `#[wasm_bindgen]` export shims.
//!
//! Every export decodes the shared execution request, installs the
//! cancellation control, composes the deadline, and calls the corresponding
//! Rust entry point. No export contains analysis logic.

use wasm_bindgen::prelude::*;

use crate::abort::{ActiveSharedCancellationGuard, ConfiguredAbort, JsSharedAbortSignal};
use crate::errors::wasm_error_to_js;
use crate::handles::{WasmAnalogResultHandle, WasmDeckResultHandle, WasmStbResultHandle};
use crate::js_interop::{
    compression_options_from_js, execution_request_from_js, serialize_to_js,
    serialize_transient_to_js,
};
use crate::options::{WasmResourceLimits, WasmStbSweep};
use crate::runners::deck::run_authored_deck_document_with_options_and_abort_detailed;
use crate::runners::direct::{
    health_check_with_options_and_abort_detailed, run_ac_analysis_with_options_and_abort_detailed,
    run_ac_document_with_options_and_abort_detailed,
    run_dc_operating_point_with_options_and_abort_detailed,
    run_dc_sweep_document_with_options_and_abort_detailed,
    run_noise_document_with_options_and_abort_detailed,
    run_operating_point_document_with_options_and_abort_detailed,
    run_stb_document_with_options_and_abort_detailed,
    run_transient_analysis_compressed_with_options_and_abort_detailed,
    run_transient_analysis_with_options_and_abort_detailed,
    run_transient_document_with_options_and_abort_detailed,
    summarize_netlist_with_options_and_abort_detailed,
};

#[wasm_bindgen(js_name = defaultResourceLimits)]
pub fn default_resource_limits_js() -> Result<JsValue, JsValue> {
    serialize_to_js(&WasmResourceLimits::default())
}

#[wasm_bindgen(js_name = healthCheck)]
pub fn health_check_js(options: JsValue) -> Result<JsValue, JsValue> {
    let request = execution_request_from_js(options).map_err(|error| wasm_error_to_js(*error))?;
    let cancellation_enabled = request.cancellation.is_some();
    let _guard = ActiveSharedCancellationGuard::install(request.cancellation)
        .map_err(|error| wasm_error_to_js(*error))?;
    let shared_abort = JsSharedAbortSignal {
        enabled: cancellation_enabled,
    };
    let abort = ConfiguredAbort::new(request.timeout_milliseconds, &shared_abort)
        .map_err(|error| wasm_error_to_js(*error))?;
    let report = health_check_with_options_and_abort_detailed(&request.options, &abort)
        .map_err(|error| wasm_error_to_js(*error))?;
    serialize_to_js(&report)
}

#[wasm_bindgen(js_name = summarizeNetlist)]
pub fn summarize_netlist_js(source: &str, options: JsValue) -> Result<JsValue, JsValue> {
    let request = execution_request_from_js(options).map_err(|error| wasm_error_to_js(*error))?;
    let cancellation_enabled = request.cancellation.is_some();
    let _guard = ActiveSharedCancellationGuard::install(request.cancellation)
        .map_err(|error| wasm_error_to_js(*error))?;
    let shared_abort = JsSharedAbortSignal {
        enabled: cancellation_enabled,
    };
    let abort = ConfiguredAbort::new(request.timeout_milliseconds, &shared_abort)
        .map_err(|error| wasm_error_to_js(*error))?;
    let summary =
        summarize_netlist_with_options_and_abort_detailed(source, &request.options, &abort)
            .map_err(|error| wasm_error_to_js(*error))?;
    serialize_to_js(&summary)
}

#[wasm_bindgen(js_name = runDcOperatingPoint)]
pub fn run_dc_operating_point_js(source: &str, options: JsValue) -> Result<JsValue, JsValue> {
    let request = execution_request_from_js(options).map_err(|error| wasm_error_to_js(*error))?;
    let cancellation_enabled = request.cancellation.is_some();
    let _guard = ActiveSharedCancellationGuard::install(request.cancellation)
        .map_err(|error| wasm_error_to_js(*error))?;
    let shared_abort = JsSharedAbortSignal {
        enabled: cancellation_enabled,
    };
    let abort = ConfiguredAbort::new(request.timeout_milliseconds, &shared_abort)
        .map_err(|error| wasm_error_to_js(*error))?;
    let result =
        run_dc_operating_point_with_options_and_abort_detailed(source, &request.options, &abort)
            .map_err(|error| wasm_error_to_js(*error))?;
    serialize_to_js(&result)
}

#[wasm_bindgen(js_name = runAcAnalysis)]
pub fn run_ac_analysis_js(
    source: &str,
    frequencies: Vec<f64>,
    options: JsValue,
) -> Result<JsValue, JsValue> {
    let request = execution_request_from_js(options).map_err(|error| wasm_error_to_js(*error))?;
    let cancellation_enabled = request.cancellation.is_some();
    let _guard = ActiveSharedCancellationGuard::install(request.cancellation)
        .map_err(|error| wasm_error_to_js(*error))?;
    let shared_abort = JsSharedAbortSignal {
        enabled: cancellation_enabled,
    };
    let abort = ConfiguredAbort::new(request.timeout_milliseconds, &shared_abort)
        .map_err(|error| wasm_error_to_js(*error))?;
    let result = run_ac_analysis_with_options_and_abort_detailed(
        source,
        &frequencies,
        &request.options,
        &abort,
    )
    .map_err(|error| wasm_error_to_js(*error))?;
    serialize_to_js(&result)
}

#[wasm_bindgen(js_name = runTransientAnalysis)]
pub fn run_transient_analysis_js(
    source: &str,
    tstop: f64,
    max_step: f64,
    options: JsValue,
) -> Result<JsValue, JsValue> {
    let request = execution_request_from_js(options).map_err(|error| wasm_error_to_js(*error))?;
    let cancellation_enabled = request.cancellation.is_some();
    let _guard = ActiveSharedCancellationGuard::install(request.cancellation)
        .map_err(|error| wasm_error_to_js(*error))?;
    let shared_abort = JsSharedAbortSignal {
        enabled: cancellation_enabled,
    };
    let abort = ConfiguredAbort::new(request.timeout_milliseconds, &shared_abort)
        .map_err(|error| wasm_error_to_js(*error))?;
    let result = run_transient_analysis_with_options_and_abort_detailed(
        source,
        tstop,
        max_step,
        &request.options,
        &abort,
    )
    .map_err(|error| wasm_error_to_js(*error))?;
    serialize_transient_to_js(&result)
}

#[wasm_bindgen(js_name = runTransientAnalysisCompressed)]
pub fn run_transient_analysis_compressed_js(
    source: &str,
    tstop: f64,
    max_step: f64,
    compression: JsValue,
    options: JsValue,
) -> Result<JsValue, JsValue> {
    let compression =
        compression_options_from_js(compression).map_err(|error| wasm_error_to_js(*error))?;
    let request = execution_request_from_js(options).map_err(|error| wasm_error_to_js(*error))?;
    let cancellation_enabled = request.cancellation.is_some();
    let _guard = ActiveSharedCancellationGuard::install(request.cancellation)
        .map_err(|error| wasm_error_to_js(*error))?;
    let shared_abort = JsSharedAbortSignal {
        enabled: cancellation_enabled,
    };
    let abort = ConfiguredAbort::new(request.timeout_milliseconds, &shared_abort)
        .map_err(|error| wasm_error_to_js(*error))?;
    let result = run_transient_analysis_compressed_with_options_and_abort_detailed(
        source,
        tstop,
        max_step,
        &compression,
        &request.options,
        &abort,
    )
    .map_err(|error| wasm_error_to_js(*error))?;
    serialize_transient_to_js(&result)
}

#[wasm_bindgen(js_name = runOperatingPointDocument)]
pub fn run_operating_point_document_js(
    source: &str,
    ordinal: usize,
    options: JsValue,
) -> Result<WasmAnalogResultHandle, JsValue> {
    let request = execution_request_from_js(options).map_err(|error| wasm_error_to_js(*error))?;
    let cancellation_enabled = request.cancellation.is_some();
    let _guard = ActiveSharedCancellationGuard::install(request.cancellation)
        .map_err(|error| wasm_error_to_js(*error))?;
    let shared_abort = JsSharedAbortSignal {
        enabled: cancellation_enabled,
    };
    let abort = ConfiguredAbort::new(request.timeout_milliseconds, &shared_abort)
        .map_err(|error| wasm_error_to_js(*error))?;
    let document = run_operating_point_document_with_options_and_abort_detailed(
        source,
        ordinal,
        &request.options,
        &abort,
    )
    .map_err(|error| wasm_error_to_js(*error))?;
    WasmAnalogResultHandle::new(document, request.options.resource_limits.to_core())
        .map_err(|error| wasm_error_to_js(*error))
}

#[wasm_bindgen(js_name = runDcSweepDocument)]
#[allow(clippy::too_many_arguments)]
pub fn run_dc_sweep_document_js(
    source: &str,
    source_name: &str,
    start: f64,
    stop: f64,
    step: f64,
    ordinal: usize,
    options: JsValue,
) -> Result<WasmAnalogResultHandle, JsValue> {
    let request = execution_request_from_js(options).map_err(|error| wasm_error_to_js(*error))?;
    let cancellation_enabled = request.cancellation.is_some();
    let _guard = ActiveSharedCancellationGuard::install(request.cancellation)
        .map_err(|error| wasm_error_to_js(*error))?;
    let shared_abort = JsSharedAbortSignal {
        enabled: cancellation_enabled,
    };
    let abort = ConfiguredAbort::new(request.timeout_milliseconds, &shared_abort)
        .map_err(|error| wasm_error_to_js(*error))?;
    let document = run_dc_sweep_document_with_options_and_abort_detailed(
        source,
        source_name,
        start,
        stop,
        step,
        ordinal,
        &request.options,
        &abort,
    )
    .map_err(|error| wasm_error_to_js(*error))?;
    WasmAnalogResultHandle::new(document, request.options.resource_limits.to_core())
        .map_err(|error| wasm_error_to_js(*error))
}

#[wasm_bindgen(js_name = runAcAnalysisDocument)]
pub fn run_ac_document_js(
    source: &str,
    frequencies: Vec<f64>,
    ordinal: usize,
    options: JsValue,
) -> Result<WasmAnalogResultHandle, JsValue> {
    let request = execution_request_from_js(options).map_err(|error| wasm_error_to_js(*error))?;
    let cancellation_enabled = request.cancellation.is_some();
    let _guard = ActiveSharedCancellationGuard::install(request.cancellation)
        .map_err(|error| wasm_error_to_js(*error))?;
    let shared_abort = JsSharedAbortSignal {
        enabled: cancellation_enabled,
    };
    let abort = ConfiguredAbort::new(request.timeout_milliseconds, &shared_abort)
        .map_err(|error| wasm_error_to_js(*error))?;
    let document = run_ac_document_with_options_and_abort_detailed(
        source,
        &frequencies,
        ordinal,
        &request.options,
        &abort,
    )
    .map_err(|error| wasm_error_to_js(*error))?;
    WasmAnalogResultHandle::new(document, request.options.resource_limits.to_core())
        .map_err(|error| wasm_error_to_js(*error))
}

#[wasm_bindgen(js_name = runTransientAnalysisDocument)]
pub fn run_transient_document_js(
    source: &str,
    tstop: f64,
    max_step: f64,
    ordinal: usize,
    options: JsValue,
) -> Result<WasmAnalogResultHandle, JsValue> {
    let request = execution_request_from_js(options).map_err(|error| wasm_error_to_js(*error))?;
    let cancellation_enabled = request.cancellation.is_some();
    let _guard = ActiveSharedCancellationGuard::install(request.cancellation)
        .map_err(|error| wasm_error_to_js(*error))?;
    let shared_abort = JsSharedAbortSignal {
        enabled: cancellation_enabled,
    };
    let abort = ConfiguredAbort::new(request.timeout_milliseconds, &shared_abort)
        .map_err(|error| wasm_error_to_js(*error))?;
    let document = run_transient_document_with_options_and_abort_detailed(
        source,
        tstop,
        max_step,
        ordinal,
        &request.options,
        &abort,
    )
    .map_err(|error| wasm_error_to_js(*error))?;
    WasmAnalogResultHandle::new(document, request.options.resource_limits.to_core())
        .map_err(|error| wasm_error_to_js(*error))
}

#[wasm_bindgen(js_name = runNoiseAnalysisDocument)]
#[allow(clippy::too_many_arguments)]
pub fn run_noise_document_js(
    source: &str,
    output_node: &str,
    reference_node: Option<String>,
    input_source: &str,
    frequencies: Vec<f64>,
    ordinal: usize,
    options: JsValue,
) -> Result<WasmAnalogResultHandle, JsValue> {
    let request = execution_request_from_js(options).map_err(|error| wasm_error_to_js(*error))?;
    let cancellation_enabled = request.cancellation.is_some();
    let _guard = ActiveSharedCancellationGuard::install(request.cancellation)
        .map_err(|error| wasm_error_to_js(*error))?;
    let shared_abort = JsSharedAbortSignal {
        enabled: cancellation_enabled,
    };
    let abort = ConfiguredAbort::new(request.timeout_milliseconds, &shared_abort)
        .map_err(|error| wasm_error_to_js(*error))?;
    let document = run_noise_document_with_options_and_abort_detailed(
        source,
        output_node,
        reference_node.as_deref(),
        input_source,
        &frequencies,
        ordinal,
        &request.options,
        &abort,
    )
    .map_err(|error| wasm_error_to_js(*error))?;
    WasmAnalogResultHandle::new(document, request.options.resource_limits.to_core())
        .map_err(|error| wasm_error_to_js(*error))
}

#[wasm_bindgen(js_name = runStbAnalysisDocument)]
#[allow(clippy::too_many_arguments)]
pub fn run_stb_document_js(
    source: &str,
    probe: &str,
    sweep: &str,
    points: usize,
    start_frequency: f64,
    stop_frequency: f64,
    compute_nyquist: bool,
    ordinal: usize,
    options: JsValue,
) -> Result<WasmStbResultHandle, JsValue> {
    let request = execution_request_from_js(options).map_err(|error| wasm_error_to_js(*error))?;
    let sweep = WasmStbSweep::parse(sweep).map_err(|error| wasm_error_to_js(*error))?;
    let cancellation_enabled = request.cancellation.is_some();
    let _guard = ActiveSharedCancellationGuard::install(request.cancellation)
        .map_err(|error| wasm_error_to_js(*error))?;
    let shared_abort = JsSharedAbortSignal {
        enabled: cancellation_enabled,
    };
    let abort = ConfiguredAbort::new(request.timeout_milliseconds, &shared_abort)
        .map_err(|error| wasm_error_to_js(*error))?;
    let document = run_stb_document_with_options_and_abort_detailed(
        source,
        probe,
        sweep,
        points,
        start_frequency,
        stop_frequency,
        compute_nyquist,
        ordinal,
        &request.options,
        &abort,
    )
    .map_err(|error| wasm_error_to_js(*error))?;
    WasmStbResultHandle::new_with_abort(document, request.options.resource_limits.to_core(), &abort)
        .map_err(|error| wasm_error_to_js(*error))
}

/// Execute a complete authored analog deck, including canonical STEP/TEMP
/// axes, and retain its coordinate-local results behind bounded windows.
#[wasm_bindgen(js_name = runAuthoredDeckDocument)]
pub fn run_authored_deck_document_js(
    source: &str,
    options: JsValue,
) -> Result<WasmDeckResultHandle, JsValue> {
    let request = execution_request_from_js(options).map_err(|error| wasm_error_to_js(*error))?;
    let cancellation_enabled = request.cancellation.is_some();
    let _guard = ActiveSharedCancellationGuard::install(request.cancellation)
        .map_err(|error| wasm_error_to_js(*error))?;
    let shared_abort = JsSharedAbortSignal {
        enabled: cancellation_enabled,
    };
    let abort = ConfiguredAbort::new(request.timeout_milliseconds, &shared_abort)
        .map_err(|error| wasm_error_to_js(*error))?;
    let document = run_authored_deck_document_with_options_and_abort_detailed(
        source,
        &request.options,
        &abort,
    )
    .map_err(|error| wasm_error_to_js(*error))?;
    WasmDeckResultHandle::new_with_abort(
        document,
        request.options.resource_limits.to_core(),
        &abort,
    )
    .map_err(|error| wasm_error_to_js(*error))
}
