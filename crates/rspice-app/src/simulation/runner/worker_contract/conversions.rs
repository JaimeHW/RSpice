//! Converting between the worker's mirror types and the core result model.
//!
//! Every conversion here is total in one direction and checked in the other:
//! going out to the worker cannot fail, but coming back must reject a payload
//! whose enum tags or lengths do not correspond to anything the core model
//! admits.  That is why these live together — the pair for each type has to be
//! read as one round trip.

use super::*;

#[cfg(test)]
pub(super) fn noise_summary_payload_bytes(summary: &WorkerNoiseSummary) -> usize {
    sum_payload_bytes([
        summary
            .rows
            .iter()
            .map(|_| f64_payload_bytes(2))
            .fold(0usize, |total, bytes| total.saturating_add(bytes)),
        f64_payload_bytes(3),
        summary
            .conversion
            .as_ref()
            .map_or(0, |_| f64_payload_bytes(1).saturating_add(12)),
        summary.noise_figure.as_ref().map_or(0, |figure| {
            f64_payload_bytes(
                3usize
                    .saturating_add(figure.frequencies.len())
                    .saturating_add(figure.decibels.len()),
            )
        }),
    ])
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerMonteCarloVariable {
    /// Confidence in the mean, with estimator and successful-trial population.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean_confidence: Option<crate::state::MonteCarloMeanConfidence>,
    pub name: String,
    pub samples: Vec<f64>,
    pub mean: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub histogram: Vec<usize>,
    pub bin_edges: Vec<f64>,
}

#[cfg(test)]
impl WorkerMonteCarloVariable {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        sum_payload_bytes([
            f64_payload_bytes(4usize.saturating_add(self.samples.len())),
            if self.mean_confidence.is_some() {
                48
            } else {
                0
            },
            usize_payload_bytes(self.histogram.len()),
            f64_payload_bytes(self.bin_edges.len()),
        ])
    }
}

impl From<MonteCarloVariableResult> for WorkerMonteCarloVariable {
    fn from(value: MonteCarloVariableResult) -> Self {
        Self {
            mean_confidence: value.mean_confidence,
            name: value.name,
            samples: value.samples,
            mean: value.mean,
            std_dev: value.std_dev,
            min: value.min,
            max: value.max,
            histogram: value.histogram,
            bin_edges: value.bin_edges,
        }
    }
}

impl From<WorkerMonteCarloVariable> for MonteCarloVariableResult {
    fn from(value: WorkerMonteCarloVariable) -> Self {
        Self {
            mean_confidence: value.mean_confidence,
            name: value.name,
            samples: value.samples,
            mean: value.mean,
            std_dev: value.std_dev,
            min: value.min,
            max: value.max,
            histogram: value.histogram,
            bin_edges: value.bin_edges,
        }
    }
}

#[cfg(test)]
fn soa_evaluation_payload_bytes(value: &WorkerSoAEvaluation) -> usize {
    f64_payload_bytes(if value.derating.is_some() { 6 } else { 3 })
        .saturating_add(f64_payload_bytes(value.envelope.as_ref().map_or(0, |e| {
            1 + usize::from(e.curve.pulse_width_s.is_some())
                + e.curve.voltages_v.len()
                + e.curve.dc_currents_a.as_ref().map_or(0, Vec::len)
                + e.curve
                    .pulses
                    .iter()
                    .map(|p| 1 + p.currents_a.len())
                    .sum::<usize>()
        })))
        .saturating_add(f64_payload_bytes(if value.duration.is_some() {
            6
        } else {
            0
        }))
        .saturating_add(f64_payload_bytes(if value.thresholds.is_default() {
            0
        } else {
            usize::from(value.thresholds.warning_fraction.is_some())
                + usize::from(value.thresholds.critical_fraction.is_some())
        }))
        .saturating_add(std::mem::size_of::<u64>())
}

#[cfg(test)]
fn soa_violation_payload_bytes(_value: &WorkerSoAViolation) -> usize {
    f64_payload_bytes(3)
}

#[cfg(test)]
fn waveform_payload_bytes(waveform: &WorkerWaveform) -> usize {
    sum_payload_bytes([
        f64_payload_bytes(waveform.x_values.len()),
        f64_payload_bytes(waveform.y_values.len()),
        waveform
            .y_imag
            .as_ref()
            .map_or(0, |values| f64_payload_bytes(values.len())),
    ])
}

#[cfg(test)]
fn measurement_payload_bytes(measurement: &WorkerMeasurement) -> usize {
    f64_payload_bytes(
        usize::from(measurement.value.is_some())
            + usize::from(measurement.raw_value.is_some())
            + usize::from(measurement.expected.is_some())
            + usize::from(measurement.tolerance.is_some())
            + usize::from(measurement.failure_limit.is_some())
            + usize::from(measurement.event_axis.is_some()),
    )
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerDeviceOpReport {
    pub entries: Vec<WorkerDeviceOpEntry>,
}

#[cfg(test)]
impl WorkerDeviceOpReport {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        self.entries
            .iter()
            .map(WorkerDeviceOpEntry::estimated_numeric_payload_bytes)
            .fold(0usize, |total, bytes| total.saturating_add(bytes))
    }
}

impl From<rspice_core::circuit::DeviceOpReport> for WorkerDeviceOpReport {
    fn from(value: rspice_core::circuit::DeviceOpReport) -> Self {
        Self {
            entries: value
                .entries
                .into_iter()
                .map(WorkerDeviceOpEntry::from)
                .collect(),
        }
    }
}

impl From<WorkerDeviceOpReport> for rspice_core::circuit::DeviceOpReport {
    fn from(value: WorkerDeviceOpReport) -> Self {
        Self {
            entries: value
                .entries
                .into_iter()
                .map(rspice_core::circuit::DeviceOpEntry::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerDeviceOpEntry {
    pub name: String,
    pub device_kind: String,
    pub region: Option<String>,
    pub params: Vec<WorkerNamedValue>,
}

#[cfg(test)]
impl WorkerDeviceOpEntry {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        self.params
            .iter()
            .map(WorkerNamedValue::estimated_numeric_payload_bytes)
            .fold(0usize, |total, bytes| total.saturating_add(bytes))
    }
}

impl From<rspice_core::circuit::DeviceOpEntry> for WorkerDeviceOpEntry {
    fn from(value: rspice_core::circuit::DeviceOpEntry) -> Self {
        Self {
            name: value.name,
            device_kind: value.device_kind.to_string(),
            region: value.region.map(str::to_string),
            params: value
                .params
                .into_iter()
                .map(|(name, value)| WorkerNamedValue {
                    name: name.to_string(),
                    value,
                })
                .collect(),
        }
    }
}

impl From<WorkerDeviceOpEntry> for rspice_core::circuit::DeviceOpEntry {
    fn from(value: WorkerDeviceOpEntry) -> Self {
        Self {
            name: value.name,
            device_kind: intern_static_label(value.device_kind),
            region: value.region.map(intern_static_label),
            params: value
                .params
                .into_iter()
                .map(|param| (intern_static_label(param.name), param.value))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerNamedValue {
    pub name: String,
    pub value: f64,
}

#[cfg(test)]
impl WorkerNamedValue {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        f64_payload_bytes(1)
    }
}

#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
pub(crate) fn worker_response_from_request(request: WorkerRequest) -> WorkerResponse {
    worker_response_from_request_with_progress(request, None)
}

pub(super) fn worker_response_from_request_with_progress(
    request: WorkerRequest,
    progress_observer: Option<super::super::ProgressObserver>,
) -> WorkerResponse {
    let id = request.id;
    let (request, input) = request.into_runner_parts();
    let progress = Arc::new(Mutex::new(SimulationProgress::default()));
    let abort_flag = Arc::new(AtomicBool::new(false));

    WorkerResponse::from_result_for_transfer(
        id,
        super::super::run_simulation_thread_with_progress_observer(
            request,
            input,
            progress,
            abort_flag,
            super::super::RunStreams {
                progress_observer,
                ..Default::default()
            },
        ),
    )
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
thread_local! {
    static ACTIVE_WORKER_PROGRESS_ID: std::cell::Cell<Option<u64>> = const { std::cell::Cell::new(None) };
    static PENDING_WASM_JIT_REQUEST: std::cell::RefCell<Option<(u32, WorkerRequest)>> = const { std::cell::RefCell::new(None) };
    static NEXT_WASM_JIT_DISPATCH_TOKEN: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub(super) fn emit_worker_progress_snapshot(progress: &SimulationProgress) {
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::JsValue;

    let id = ACTIVE_WORKER_PROGRESS_ID.with(|active| active.get());
    let Some(id) = id else {
        return;
    };

    let snapshot = WorkerProgressSnapshot::from_status(id, &progress.status, progress.elapsed);
    let message = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &message,
        &JsValue::from_str("type"),
        &JsValue::from_str("progress"),
    );
    let _ = js_sys::Reflect::set(
        &message,
        &JsValue::from_str("id"),
        &JsValue::from_f64(id as f64),
    );
    if let Ok(snapshot) = serde_wasm_bindgen::to_value(&snapshot) {
        let _ = js_sys::Reflect::set(&message, &JsValue::from_str("progress"), &snapshot);
    }

    let global = js_sys::global();
    let Ok(post_message) = js_sys::Reflect::get(&global, &JsValue::from_str("postMessage"))
        .and_then(|value| value.dyn_into::<js_sys::Function>())
    else {
        return;
    };
    let _ = post_message.call1(&global, &JsValue::from(message));
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub(super) fn emit_worker_transient_sample(sample: &super::super::TransientSampleDelta) {
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::JsValue;

    let id = ACTIVE_WORKER_PROGRESS_ID.with(|active| active.get());
    let Some(id) = id else {
        return;
    };
    let message = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &message,
        &JsValue::from_str("type"),
        &JsValue::from_str("transientSample"),
    );
    let _ = js_sys::Reflect::set(
        &message,
        &JsValue::from_str("id"),
        &JsValue::from_f64(id as f64),
    );
    if let Ok(sample) = serde_wasm_bindgen::to_value(sample) {
        let _ = js_sys::Reflect::set(&message, &JsValue::from_str("sample"), &sample);
    } else {
        return;
    }

    let global = js_sys::global();
    let Ok(post_message) = js_sys::Reflect::get(&global, &JsValue::from_str("postMessage"))
        .and_then(|value| value.dyn_into::<js_sys::Function>())
    else {
        return;
    };
    let _ = post_message.call1(&global, &JsValue::from(message));
}

/// Deliver the portable checkpoint as transferred bytes before terminal success.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
fn emit_worker_monte_carlo_checkpoint(bytes: &[u8]) -> Result<(), SimulationError> {
    use wasm_bindgen::{JsCast as _, JsValue};
    let fail = |message: String| {
        SimulationError::InvalidConfig(format!(
            "Could not deliver Monte Carlo checkpoint: {message}"
        ))
    };
    super::super::monte_carlo_checkpoint::validate_checkpoint_bytes_size(bytes.len())
        .map_err(&fail)?;
    let id = ACTIVE_WORKER_PROGRESS_ID
        .with(|active| active.get())
        .ok_or_else(|| fail("no active worker request".into()))?;
    let message = js_sys::Object::new();
    let view = js_sys::Uint8Array::new_with_length(bytes.len() as u32);
    view.copy_from(bytes);
    for (key, value) in [
        ("type", JsValue::from_str("monteCarloCheckpoint")),
        ("id", JsValue::from_f64(id as f64)),
        ("checkpoint", JsValue::from(view.clone())),
    ] {
        if !js_sys::Reflect::set(&message, &JsValue::from_str(key), &value)
            .map_err(|error| fail(format!("{error:?}")))?
        {
            return Err(fail(format!("could not set {key}")));
        }
    }
    let transfer = js_sys::Array::new();
    transfer.push(&view.buffer());
    let global = js_sys::global();
    let post = js_sys::Reflect::get(&global, &JsValue::from_str("postMessage"))
        .and_then(|value| value.dyn_into::<js_sys::Function>())
        .map_err(|error| fail(format!("{error:?}")))?;
    post.call2(&global, &message, &transfer)
        .map_err(|error| fail(format!("{error:?}")))?;
    Ok(())
}

/// Post one line the engine logged back to the UI instance.
///
/// The worker is its own wasm instance, so there is no queue on this side of
/// the boundary for a run to write: the same shape as
/// [`emit_worker_transient_sample`], one message per line, keyed by the active
/// request so a superseded run's lines cannot land in a newer run's Console.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub(super) fn emit_worker_engine_log(line: &crate::diagnostics::engine_log::EngineLogLine) {
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::JsValue;

    let id = ACTIVE_WORKER_PROGRESS_ID.with(|active| active.get());
    let Some(id) = id else {
        return;
    };
    let message = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &message,
        &JsValue::from_str("type"),
        &JsValue::from_str("engineLog"),
    );
    let _ = js_sys::Reflect::set(
        &message,
        &JsValue::from_str("id"),
        &JsValue::from_f64(id as f64),
    );
    if let Ok(line) = serde_wasm_bindgen::to_value(line) {
        let _ = js_sys::Reflect::set(&message, &JsValue::from_str("line"), &line);
    } else {
        return;
    }

    let global = js_sys::global();
    let Ok(post_message) = js_sys::Reflect::get(&global, &JsValue::from_str("postMessage"))
        .and_then(|value| value.dyn_into::<js_sys::Function>())
    else {
        return;
    };
    let _ = post_message.call1(&global, &JsValue::from(message));
}

/// The worker image's whole logger: it feeds the run's sink and nothing else.
///
/// This image has no terminal and paints nothing, so there is no second reader
/// to filter for — where the desktop's `StudioLogger` wraps `env_logger`, this
/// is the sink half alone.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
struct WorkerEngineLogger;

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
impl log::Log for WorkerEngineLogger {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        crate::diagnostics::engine_log::admits(metadata)
    }

    fn log(&self, record: &log::Record<'_>) {
        crate::diagnostics::engine_log::offer(record);
    }

    fn flush(&self) {}
}

/// Install [`WorkerEngineLogger`] on this worker, once.
///
/// Called at the head of a request rather than from a bootstrap export: the
/// worker script's startup sequence is a published contract with the page, and
/// a logger the first run installs needs no place in it. `set_boxed_logger`
/// succeeds once per instance, and the worker instance outlives many requests,
/// so the guard is a `OnceLock` rather than a per-run install.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
fn install_worker_engine_logger() {
    static INSTALLED: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    INSTALLED.get_or_init(|| {
        if log::set_boxed_logger(Box::new(WorkerEngineLogger)).is_ok() {
            // No stderr half at all on this image, so the level the process
            // admits is exactly what the run's sink asks for.
            crate::diagnostics::engine_log::note_stderr_level(log::LevelFilter::Off);
        }
    });
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub(crate) fn run_worker_request_value(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let request = worker_request_from_value(value)?;
    run_decoded_worker_request(request)
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
fn run_decoded_worker_request(
    request: WorkerRequest,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let id = request.id;
    ACTIVE_WORKER_PROGRESS_ID.with(|active| active.set(Some(id)));
    let stream_transient_samples = request.stream_transient_samples;
    let (request, input) = request.into_runner_parts();
    install_worker_engine_logger();
    let verbose = super::super::request_asked_for_verbose(&request);
    let progress = Arc::new(Mutex::new(SimulationProgress::default()));
    let abort_flag = Arc::new(AtomicBool::new(false));
    let response = WorkerResponse::from_result_for_transfer(
        id,
        super::super::run_simulation_thread_with_progress_observer(
            request,
            input,
            progress,
            abort_flag,
            super::super::RunStreams {
                progress_observer: Some(emit_worker_progress_snapshot),
                checkpoint_observer: Some(Arc::new(emit_worker_monte_carlo_checkpoint)),
                transient_sample_observer: stream_transient_samples
                    .then_some(emit_worker_transient_sample),
                engine_log: Some(crate::diagnostics::engine_log::RunLogSink::observed(
                    emit_worker_engine_log,
                    verbose,
                )),
                ..Default::default()
            },
        ),
    );
    ACTIVE_WORKER_PROGRESS_ID.with(|active| active.set(None));
    worker_response_transport_value(response)
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct WasmJitRequestPreparation {
    dispatch_token: u32,
    artifacts: Vec<crate::simulation::veriloga::WasmJitWorkerArtifact>,
    errors: Vec<String>,
}

/// Compile every sealed Verilog-A runtime required by a simulation request
/// before the synchronous solver begins. The JavaScript worker installs these
/// modules into its persistent, capability-limited instance cache.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub(crate) fn prepare_wasm_jit_request_value(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    if PENDING_WASM_JIT_REQUEST.with(|pending| pending.borrow().is_some()) {
        return Err(wasm_bindgen::JsValue::from_str(
            "a prepared browser simulation request is already pending",
        ));
    }
    let request = worker_request_from_value(value)?;
    let dispatch_token = NEXT_WASM_JIT_DISPATCH_TOKEN.with(|next| {
        let token = next.get().wrapping_add(1).max(1);
        next.set(token);
        token
    });
    let mut preparation = WasmJitRequestPreparation {
        dispatch_token,
        artifacts: Vec::with_capacity(request.project_veriloga_runtimes.device_runtimes().len()),
        errors: Vec::new(),
    };
    for runtime in request.project_veriloga_runtimes.device_runtimes() {
        match runtime.compile_wasm_jit_artifact() {
            Ok(artifact) => preparation.artifacts.push(artifact),
            Err(error) => preparation.errors.push(format!(
                "Verilog-A runtime '{}' could not qualify for the browser JIT: {error}",
                runtime.netlist_alias()
            )),
        }
    }
    let value = serde_wasm_bindgen::to_value(&preparation)
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))?;
    PENDING_WASM_JIT_REQUEST.with(|pending| {
        *pending.borrow_mut() = Some((dispatch_token, request));
    });
    Ok(value)
}

/// Consume exactly the request decoded by `prepare_wasm_jit_request_value`.
/// This avoids a second copy of every transferred numerical dependency.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub(crate) fn run_prepared_wasm_jit_request_value(
    dispatch_token: u32,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let request = PENDING_WASM_JIT_REQUEST.with(|pending| {
        let mut pending = pending.borrow_mut();
        let Some((expected_token, _)) = pending.as_ref() else {
            return Err(wasm_bindgen::JsValue::from_str(
                "no prepared browser simulation request is pending",
            ));
        };
        if dispatch_token == 0 || dispatch_token != *expected_token {
            return Err(wasm_bindgen::JsValue::from_str(
                "stale browser simulation dispatch token",
            ));
        }
        Ok(pending
            .take()
            .expect("validated prepared request must remain present")
            .1)
    })?;
    run_decoded_worker_request(request)
}

/// Discard a prepared request only when the caller presents its exact token.
///
/// This closes the one failure path between request decoding and synchronous
/// dispatch without allowing a stale JavaScript caller to cancel newer work.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub(crate) fn cancel_prepared_wasm_jit_request_value(
    dispatch_token: u32,
) -> Result<(), wasm_bindgen::JsValue> {
    PENDING_WASM_JIT_REQUEST.with(|pending| {
        let mut pending = pending.borrow_mut();
        let Some((expected_token, _)) = pending.as_ref() else {
            return Err(wasm_bindgen::JsValue::from_str(
                "no prepared browser simulation request is pending",
            ));
        };
        if dispatch_token == 0 || dispatch_token != *expected_token {
            return Err(wasm_bindgen::JsValue::from_str(
                "stale browser simulation dispatch token",
            ));
        }
        pending.take();
        Ok(())
    })
}

#[cfg(target_arch = "wasm32")]
pub(super) fn worker_request_from_value(
    value: wasm_bindgen::JsValue,
) -> Result<WorkerRequest, wasm_bindgen::JsValue> {
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::JsValue;

    let protocol = js_sys::Reflect::get(&value, &JsValue::from_str("protocolVersion"))
        .map_err(worker_request_js_error)?
        .as_f64()
        .and_then(|value| {
            (value.fract() == 0.0 && (0.0..=f64::from(u8::MAX)).contains(&value))
                .then_some(value as u8)
        })
        .ok_or_else(|| {
            JsValue::from_str("worker request transport protocolVersion must be an unsigned byte")
        })?;

    let request = js_sys::Reflect::get(&value, &JsValue::from_str("request"))
        .map_err(worker_request_js_error)?;
    let request = serde_wasm_bindgen::from_value::<WorkerRequestTransportMetadata>(request)
        .map_err(|error| JsValue::from_str(&error.to_string()))?;

    let buffers = js_sys::Reflect::get(&value, &JsValue::from_str("buffers"))
        .map_err(worker_request_js_error)?
        .dyn_into::<js_sys::Array>()
        .map_err(|_| JsValue::from_str("worker request transport buffers must be an array"))?;
    let buffer_count = buffers.length() as usize;
    if buffer_count > MAX_WORKER_TRANSFER_BUFFERS {
        return Err(JsValue::from_str(&format!(
            "worker request contains {buffer_count} transfer buffers, exceeding the {MAX_WORKER_TRANSFER_BUFFERS}-buffer limit"
        )));
    }
    let mut numeric_values = 0usize;
    for index in 0..buffers.length() {
        let view = buffers
            .get(index)
            .dyn_into::<js_sys::Float64Array>()
            .map_err(|_| {
                JsValue::from_str(&format!(
                    "worker request transport buffer {index} is not a Float64Array"
                ))
            })?;
        numeric_values = checked_worker_request_numeric_total(
            numeric_values,
            index as usize,
            view.length() as usize,
        )
        .map_err(|error| JsValue::from_str(&error))?;
    }

    let byte_buffers = js_sys::Reflect::get(&value, &JsValue::from_str("byteBuffers"))
        .map_err(worker_request_js_error)?
        .dyn_into::<js_sys::Array>()
        .map_err(|_| JsValue::from_str("worker request byteBuffers must be an array"))?;
    if byte_buffers.length() > 1 {
        return Err(JsValue::from_str(
            "worker request carries too many checkpoint buffers",
        ));
    }
    let mut byte_lengths = Vec::new();
    for index in 0..byte_buffers.length() {
        let view = byte_buffers
            .get(index)
            .dyn_into::<js_sys::Uint8Array>()
            .map_err(|_| JsValue::from_str("worker request checkpoint must be a Uint8Array"))?;
        byte_lengths.push(view.length() as usize);
    }
    validate_worker_request_checkpoint_lengths(numeric_values, &byte_lengths)
        .map_err(|error| JsValue::from_str(&error))?;
    let mut decoded_bytes = Vec::new();
    for index in 0..byte_buffers.length() {
        let view = byte_buffers
            .get(index)
            .dyn_into::<js_sys::Uint8Array>()
            .map_err(|_| JsValue::from_str("worker request checkpoint must be a Uint8Array"))?;
        let mut bytes = vec![0; view.length() as usize];
        view.copy_to(&mut bytes);
        decoded_bytes.push(bytes);
    }

    let mut decoded_buffers = Vec::with_capacity(buffer_count);
    for index in 0..buffers.length() {
        let view = buffers
            .get(index)
            .dyn_into::<js_sys::Float64Array>()
            .map_err(|_| {
                JsValue::from_str(&format!(
                    "worker request transport buffer {index} is not a Float64Array"
                ))
            })?;
        let mut values = vec![0.0; view.length() as usize];
        view.copy_to(&mut values);
        decoded_buffers.push(values);
    }

    WorkerRequestTransport {
        protocol,
        request,
        buffers: decoded_buffers,
        byte_buffers: decoded_bytes,
    }
    .into_request()
    .map_err(|error| JsValue::from_str(&error))
}

#[cfg(target_arch = "wasm32")]
pub(super) fn worker_request_js_error(error: wasm_bindgen::JsValue) -> wasm_bindgen::JsValue {
    wasm_bindgen::JsValue::from_str(&worker_js_error(error).to_string())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn worker_response_transport_value(
    response: WorkerResponse,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    use wasm_bindgen::JsValue;

    let transport = WorkerResponseTransport::from_response(response)
        .map_err(|error| JsValue::from_str(&error))?;
    let message = js_sys::Object::new();
    js_sys::Reflect::set(
        &message,
        &JsValue::from_str("protocolVersion"),
        &JsValue::from_f64(f64::from(transport.protocol)),
    )?;
    // Result identities and Monte Carlo seeds use the complete u64 range.
    // Structured clone carries BigInt exactly; the default JS-number encoder
    // rejects otherwise valid results above 2^53 - 1.
    let response = transport
        .response
        .serialize(
            &serde_wasm_bindgen::Serializer::new().serialize_large_number_types_as_bigints(true),
        )
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    js_sys::Reflect::set(&message, &JsValue::from_str("response"), &response)?;

    let buffers = js_sys::Array::new();
    for values in transport.buffers {
        let view = js_sys::Float64Array::new_with_length(values.len() as u32);
        view.copy_from(&values);
        buffers.push(&view);
    }
    js_sys::Reflect::set(&message, &JsValue::from_str("buffers"), &buffers)?;

    Ok(JsValue::from(message))
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn worker_response_from_value(
    value: wasm_bindgen::JsValue,
) -> Result<WorkerResponse, SimulationError> {
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::JsValue;

    let protocol = js_sys::Reflect::get(&value, &JsValue::from_str("protocolVersion"))
        .ok()
        .and_then(|value| value.as_f64())
        .map(|value| value as u8);

    if protocol != Some(WORKER_RESPONSE_TRANSPORT_PROTOCOL) {
        let response = serde_wasm_bindgen::from_value::<WorkerResponse>(value)
            .map_err(|error| SimulationError::InvalidConfig(error.to_string()))?;
        super::transport::validate_worker_response_before_transport(&response)
            .map_err(SimulationError::InvalidConfig)?;
        return Ok(response);
    }

    let response =
        js_sys::Reflect::get(&value, &JsValue::from_str("response")).map_err(worker_js_error)?;
    let response = serde_wasm_bindgen::from_value::<WorkerResponseTransportMetadata>(response)
        .map_err(|error| SimulationError::InvalidConfig(error.to_string()))?;

    let buffers = js_sys::Reflect::get(&value, &JsValue::from_str("buffers"))
        .map_err(worker_js_error)?
        .dyn_into::<js_sys::Array>()
        .map_err(|_| {
            SimulationError::InvalidConfig(
                "worker response transport buffers must be an array".to_string(),
            )
        })?;

    let buffer_count = buffers.length() as usize;
    if buffer_count > MAX_WORKER_TRANSFER_BUFFERS {
        return Err(SimulationError::InvalidConfig(format!(
            "worker response contains {buffer_count} transfer buffers, exceeding the {MAX_WORKER_TRANSFER_BUFFERS}-buffer limit"
        )));
    }
    let mut numeric_values = 0usize;
    for index in 0..buffers.length() {
        let view = buffers
            .get(index)
            .dyn_into::<js_sys::Float64Array>()
            .map_err(|_| {
                SimulationError::InvalidConfig(format!(
                    "worker response transport buffer {index} is not a Float64Array"
                ))
            })?;
        numeric_values = numeric_values
            .checked_add(view.length() as usize)
            .ok_or_else(|| {
                SimulationError::InvalidConfig(
                    "worker response numeric size overflows this platform".to_owned(),
                )
            })?;
        if numeric_values > MAX_WORKER_F64_VALUES {
            return Err(SimulationError::InvalidConfig(format!(
                "worker response contains more than {MAX_WORKER_F64_VALUES} numerical values"
            )));
        }
    }

    let mut decoded_buffers = Vec::with_capacity(buffer_count);
    for index in 0..buffers.length() {
        let view = buffers
            .get(index)
            .dyn_into::<js_sys::Float64Array>()
            .map_err(|_| {
                SimulationError::InvalidConfig(format!(
                    "worker response transport buffer {index} is not a Float64Array"
                ))
            })?;
        let mut values = vec![0.0; view.length() as usize];
        view.copy_to(&mut values);
        decoded_buffers.push(values);
    }

    WorkerResponseTransport {
        protocol: WORKER_RESPONSE_TRANSPORT_PROTOCOL,
        response,
        buffers: decoded_buffers,
    }
    .into_response()
    .map_err(SimulationError::InvalidConfig)
}

#[cfg(target_arch = "wasm32")]
pub(super) fn worker_js_error(error: wasm_bindgen::JsValue) -> SimulationError {
    use wasm_bindgen::JsValue;

    let message = error
        .as_string()
        .or_else(|| {
            js_sys::Reflect::get(&error, &JsValue::from_str("message"))
                .ok()
                .and_then(|message| message.as_string())
        })
        .unwrap_or_else(|| "unknown JavaScript error".to_string());
    SimulationError::InvalidConfig(message)
}

#[cfg(test)]
pub(super) fn sum_payload_bytes(bytes: impl IntoIterator<Item = usize>) -> usize {
    bytes
        .into_iter()
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
pub(super) fn f64_payload_bytes(len: usize) -> usize {
    len.saturating_mul(std::mem::size_of::<f64>())
}

#[cfg(test)]
pub(super) fn pss_operating_point_payload_bytes(
    operating_point: &rspice_core::engine::PssOperatingPoint,
) -> usize {
    let analysis = operating_point.analysis();
    let values = analysis
        .result
        .time
        .len()
        .saturating_add(
            analysis
                .result
                .waveforms
                .iter()
                .chain(&analysis.result.branch_waveforms)
                .map(|waveform| waveform.values.len())
                .sum::<usize>(),
        )
        .saturating_add(analysis.monodromy.iter().map(Vec::len).sum::<usize>())
        .saturating_add(analysis.result.floquet_multipliers.len().saturating_mul(2))
        .saturating_add(analysis.floquet_multipliers.len().saturating_mul(2))
        .saturating_add(operating_point.shooting_state().len());
    f64_payload_bytes(values)
}

#[cfg(test)]
pub(super) fn usize_payload_bytes(len: usize) -> usize {
    len.saturating_mul(std::mem::size_of::<usize>())
}

#[cfg(test)]
pub(super) fn complex_pair_payload_bytes(len: usize) -> usize {
    len.saturating_mul(2)
        .saturating_mul(std::mem::size_of::<f64>())
}

#[cfg(test)]
pub(super) fn event_history_payload_bytes(events: &WorkerEventHistory) -> usize {
    let digital = events
        .digital
        .iter()
        .map(|trace| f64_payload_bytes(trace.points.len()).saturating_add(trace.points.len()))
        .fold(0usize, |total, bytes| total.saturating_add(bytes));
    let real = events
        .real
        .iter()
        .map(|trace| f64_payload_bytes(trace.points.len().saturating_mul(2)))
        .fold(digital, |total, bytes| total.saturating_add(bytes));
    // A bus contributes its two declared indices and nothing else: the
    // members are names, which this budget does not count for a trace either,
    // and there is no value in a declaration to count.
    let impulses = events.current_impulses.as_ref().map_or(0, |history| {
        history
            .traces
            .iter()
            .fold(2 * size_of::<f64>() + 1, |total, trace| {
                total
                    .saturating_add(1)
                    .saturating_add(f64_payload_bytes(trace.points.len().saturating_mul(2)))
            })
    });
    real.saturating_add(events.buses.len().saturating_mul(2 * size_of::<i64>()))
        .saturating_add(impulses)
}

#[cfg(test)]
pub(super) fn waveforms_payload_bytes(waveforms: &[WorkerWaveform]) -> usize {
    waveforms
        .iter()
        .map(waveform_payload_bytes)
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
pub(super) fn measurements_payload_bytes(measurements: &[WorkerMeasurement]) -> usize {
    measurements
        .iter()
        .map(measurement_payload_bytes)
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
pub(super) fn vec_map_payload_bytes(values_by_name: &HashMap<String, Vec<f64>>) -> usize {
    values_by_name
        .values()
        .map(|values| f64_payload_bytes(values.len()))
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
pub(super) fn soa_violations_payload_bytes(violations: &[WorkerSoAViolation]) -> usize {
    violations
        .iter()
        .map(soa_violation_payload_bytes)
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
pub(super) fn soa_evaluations_payload_bytes(evaluations: &[WorkerSoAEvaluation]) -> usize {
    evaluations
        .iter()
        .map(soa_evaluation_payload_bytes)
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

pub(super) fn worker_waveforms(waveforms: HashMap<String, WaveformData>) -> Vec<WorkerWaveform> {
    let mut waveforms: Vec<_> = waveforms.into_values().map(WorkerWaveform::from).collect();
    waveforms.sort_by(|left, right| left.name.cmp(&right.name));
    waveforms
}

pub(super) fn pss_display_projection(
    operating_point: &rspice_core::engine::PssOperatingPoint,
    reporting_times: &[f64],
) -> Result<rspice_core::analysis::transient::TransientOutputProjection, String> {
    let result = &operating_point.analysis().result;
    let times = if reporting_times.is_empty() {
        &result.time
    } else {
        reporting_times
    };
    let channels = result
        .node_names
        .iter()
        .filter(|name| name.as_str() != "0" && !name.eq_ignore_ascii_case("gnd"))
        .count()
        .saturating_add(result.branch_names.len());
    // Reconstruction allocates an x and y series for each displayed channel.
    let values = times
        .len()
        .saturating_mul(channels.saturating_mul(2).saturating_add(1));
    if values > MAX_WORKER_F64_VALUES {
        return Err(format!(
            "PSS display requires {values} numerical values, exceeding the {MAX_WORKER_F64_VALUES}-value limit"
        ));
    }
    rspice_core::analysis::transient::TransientOutputProjection::interpolate_times(
        &result.time,
        times,
        MAX_WORKER_F64_VALUES,
    )
}

pub(super) fn validate_pss_display_contract(
    time: &[f64],
    waveforms: &HashMap<String, WaveformData>,
    operating_point: &rspice_core::engine::PssOperatingPoint,
) -> Result<(), SimulationError> {
    let result = &operating_point.analysis().result;
    if time.is_empty() {
        return Err(SimulationError::InvalidConfig(
            "PSS display time axis is empty".into(),
        ));
    }
    let projection =
        pss_display_projection(operating_point, time).map_err(SimulationError::InvalidConfig)?;
    let expected_count = result
        .node_names
        .iter()
        .filter(|name| name.as_str() != "0" && !name.eq_ignore_ascii_case("gnd"))
        .count()
        + result.branch_names.len();
    if waveforms.len() != expected_count {
        return Err(SimulationError::InvalidConfig(format!(
            "PSS display contains {} waveforms, but its retained orbit requires {expected_count}",
            waveforms.len()
        )));
    }
    for (name, periodic, prefix, unit) in result
        .node_names
        .iter()
        .zip(&result.waveforms)
        .filter(|(name, _)| name.as_str() != "0" && !name.eq_ignore_ascii_case("gnd"))
        .map(|(name, waveform)| (name, waveform, "V", "V"))
        .chain(
            result
                .branch_names
                .iter()
                .zip(&result.branch_waveforms)
                .map(|(name, waveform)| (name, waveform, "I", "A")),
        )
    {
        let display_name = format!("{prefix}({name})");
        let display = waveforms.get(&display_name).ok_or_else(|| {
            SimulationError::InvalidConfig(format!(
                "PSS display is missing retained-orbit waveform '{display_name}'"
            ))
        })?;
        let expected = projection
            .project(&periodic.values)
            .map_err(SimulationError::InvalidConfig)?;
        if display.name != display_name
            || display.x_values.as_slice() != time
            || display.y_values != expected
            || display.y_unit != unit
            || display.is_complex
            || display.y_imag.is_some()
        {
            return Err(SimulationError::InvalidConfig(format!(
                "PSS display waveform '{display_name}' does not match the reporting projection of its retained numerical orbit"
            )));
        }
    }
    Ok(())
}

pub(super) fn simulation_result_from_worker_pss(
    measurements: Vec<WorkerMeasurement>,
    operating_point: rspice_core::engine::PssOperatingPoint,
    reporting_times: Vec<f64>,
) -> SimulationResult {
    let projection = pss_display_projection(&operating_point, &reporting_times)
        .expect("PSS reporting grid is validated at worker ingress");
    let result = &operating_point.analysis().result;
    let time = projection.times().to_vec();
    let mut waveforms =
        HashMap::with_capacity(result.waveforms.len() + result.branch_waveforms.len());
    for (name, periodic, prefix, unit) in result
        .node_names
        .iter()
        .zip(&result.waveforms)
        .filter(|(name, _)| name.as_str() != "0" && !name.eq_ignore_ascii_case("gnd"))
        .map(|(name, waveform)| (name, waveform, "V", "V"))
        .chain(
            result
                .branch_names
                .iter()
                .zip(&result.branch_waveforms)
                .map(|(name, waveform)| (name, waveform, "I", "A")),
        )
    {
        let display_name = format!("{prefix}({name})");
        waveforms.insert(
            display_name.clone(),
            WaveformData::new_time_domain_in_unit(
                display_name,
                time.clone(),
                projection
                    .project(&periodic.values)
                    .expect("retained PSS waveform lengths are authenticated"),
                unit,
            ),
        );
    }
    SimulationResult::Transient {
        spectra: Vec::new(),
        time,
        waveforms,
        measurements: measure_results(measurements),
        periodic_state: Some(std::sync::Arc::new(operating_point)),
        convergence: Default::default(),
        events: TransientEventHistory::default(),
    }
}

pub(super) fn waveform_map(waveforms: Vec<WorkerWaveform>) -> HashMap<String, WaveformData> {
    waveforms
        .into_iter()
        .map(|waveform| {
            let name = waveform.name.clone();
            (name, WaveformData::from(waveform))
        })
        .collect()
}

pub(super) fn worker_measurements(
    measurements: Vec<rspice_core::MeasureResult>,
) -> Vec<WorkerMeasurement> {
    measurements
        .into_iter()
        .map(WorkerMeasurement::from)
        .collect()
}

pub(super) fn measure_results(
    measurements: Vec<WorkerMeasurement>,
) -> Vec<rspice_core::MeasureResult> {
    measurements
        .into_iter()
        .map(rspice_core::MeasureResult::from)
        .collect()
}

pub(super) fn intern_static_label(value: String) -> &'static str {
    known_static_label(&value).unwrap_or("unknown")
}

/// Labels a worker response may carry, interned back to the `&'static str`
/// the host build uses.
///
/// The engine owns the vocabulary, so it is asked rather than restated.
/// `unknown` is accepted because [`intern_static_label`] produces it, and a
/// report that already crossed the boundary once has to survive crossing it
/// again unchanged. A noise mechanism does not come through here: the summary
/// carries it as owned text the whole way, so nothing interns it.
pub(super) fn known_static_label(value: &str) -> Option<&'static str> {
    rspice_core::circuit::resolve_op_label(value)
        .or_else(|| (value == "unknown").then_some("unknown"))
}
