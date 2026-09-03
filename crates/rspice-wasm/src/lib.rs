//! WebAssembly wrapper for the RSpice simulation core.
//!
//! The crate keeps the browser-facing API intentionally thin: it exposes
//! serializable snapshots that mirror stable simulator concepts while delegating
//! all numerical work to `rspice-core`.

mod deck_result_document;
mod result_document;
mod stb_result_document;

pub use deck_result_document::{
    DECK_RESULT_SCHEMA, DECK_RESULT_VERSION, DeckAxisAssignment, DeckAxisDescriptor, DeckAxisValue,
    DeckCoordinateDescriptor, DeckDataBinding, DeckFftBinWindow, DeckFftHarmonicWindow,
    DeckFftMetadata, DeckFftMetricsMetadata, DeckFftSummary, DeckPlannedAnalysisDescriptor,
    DeckResultDocument, DeckResultMetadata, DeckResultSummary,
};
pub use result_document::{
    ANALOG_RESULT_SCHEMA, ANALOG_RESULT_VERSION, AnalogAnalysisKind, AnalogResultDocument,
    AnalogResultMetadata, AnalogResultWindow, AnalogSignalKind, AnalysisIdentity, AxisDescriptor,
    AxisSeries, AxisWindow, ComplexSample, DeviceStateDescriptor, DeviceStateSeries,
    SignalDescriptor, SignalOwner, SignalSeries, SignalUnit, SignalValueType, SignalValues,
    SignalWindow, SignalWindowValues,
};
pub use stb_result_document::{
    STB_RESULT_SCHEMA, STB_RESULT_VERSION, StbAnalysisIdentity, StbAnalysisKind, StbBodeSeries,
    StbBodeWindow, StbComplexSample, StbComplexWindow, StbDocumentError, StbMarginDescriptors,
    StbMarginUnits, StbMargins, StbNyquistSeries, StbNyquistWindow, StbPrimarySeries,
    StbPrimaryWindow, StbResultDocument, StbResultMetadata, StbResultWindow, StbSeriesDescriptor,
    StbUnit, StbValueType,
};

use rspice_core::{
    AbortSignal, Engine, Netlist, NoAbort, ResourceKind, ResourceLimitError, ResourceLimits,
    SimulationConfig,
};
use rspice_core::{
    analysis::{StbConfig, StbSweepType},
    engine::{
        TransientFftHarmonic, TransientFftMetrics, TransientFftResult, TransientResult,
        TransientResultCompressed,
    },
    netlist::{FftFormat, FftOutput, FftWindow, XyceFftMode},
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, prelude::*};

type WasmResult<T> = Result<T, String>;
type DetailedWasmResult<T> = Result<T, Box<WasmError>>;

const MEBIBYTE: usize = 1024 * 1024;
const MAX_TIMEOUT_MILLISECONDS: u32 = 86_400_000;
const DEFAULT_MAX_TRANSFER_VALUES: usize = 262_144;

fn browser_resource_limits() -> ResourceLimits {
    let mut limits = ResourceLimits::default();
    limits.max_netlist_bytes = 8 * MEBIBYTE;
    limits.max_netlist_lines = 250_000;
    limits.max_expanded_source_bytes = 16 * MEBIBYTE;
    limits.max_dependency_source_bytes = 16 * MEBIBYTE;
    limits.max_external_data_bytes = 16 * MEBIBYTE;
    limits.max_external_data_values = 2_000_000;
    limits.max_shared_cache_bytes = 64 * MEBIBYTE;
    limits.max_include_depth = 16;
    limits.max_hierarchy_depth = 64;
    limits.max_flattened_elements = 20_000;
    limits.max_circuit_nodes = 2_000;
    limits.max_matrix_unknowns = 2_000;
    limits.max_analysis_points = 200_000;
    limits.max_result_values = 2_000_000;
    limits.max_parallel_workers = 1;
    limits.max_batch_runs = 1_000;
    limits
}

/// Browser-facing resource policy. JavaScript field names use camelCase.
///
/// Partial objects inherit every omitted field from the browser-safe defaults.
/// Unknown fields are rejected so a misspelled security control never appears
/// to have been applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase", deny_unknown_fields)]
pub struct WasmResourceLimits {
    pub max_netlist_bytes: usize,
    pub max_netlist_lines: usize,
    pub max_expanded_source_bytes: usize,
    pub max_dependency_source_bytes: usize,
    pub max_external_data_bytes: usize,
    pub max_external_data_values: usize,
    pub max_shared_cache_bytes: usize,
    pub max_include_depth: usize,
    pub max_hierarchy_depth: usize,
    pub max_flattened_elements: usize,
    pub max_circuit_nodes: usize,
    pub max_matrix_unknowns: usize,
    pub max_analysis_points: usize,
    pub max_result_values: usize,
    pub max_parallel_workers: usize,
    pub max_batch_runs: usize,
}

impl WasmResourceLimits {
    fn from_core(limits: ResourceLimits) -> Self {
        Self {
            max_netlist_bytes: limits.max_netlist_bytes,
            max_netlist_lines: limits.max_netlist_lines,
            max_expanded_source_bytes: limits.max_expanded_source_bytes,
            max_dependency_source_bytes: limits.max_dependency_source_bytes,
            max_external_data_bytes: limits.max_external_data_bytes,
            max_external_data_values: limits.max_external_data_values,
            max_shared_cache_bytes: limits.max_shared_cache_bytes,
            max_include_depth: limits.max_include_depth,
            max_hierarchy_depth: limits.max_hierarchy_depth,
            max_flattened_elements: limits.max_flattened_elements,
            max_circuit_nodes: limits.max_circuit_nodes,
            max_matrix_unknowns: limits.max_matrix_unknowns,
            max_analysis_points: limits.max_analysis_points,
            max_result_values: limits.max_result_values,
            max_parallel_workers: limits.max_parallel_workers,
            max_batch_runs: limits.max_batch_runs,
        }
    }

    fn to_core(&self) -> ResourceLimits {
        let mut limits = ResourceLimits::default();
        limits.max_netlist_bytes = self.max_netlist_bytes;
        limits.max_netlist_lines = self.max_netlist_lines;
        limits.max_expanded_source_bytes = self.max_expanded_source_bytes;
        limits.max_dependency_source_bytes = self.max_dependency_source_bytes;
        limits.max_external_data_bytes = self.max_external_data_bytes;
        limits.max_external_data_values = self.max_external_data_values;
        limits.max_shared_cache_bytes = self.max_shared_cache_bytes;
        limits.max_include_depth = self.max_include_depth;
        limits.max_hierarchy_depth = self.max_hierarchy_depth;
        limits.max_flattened_elements = self.max_flattened_elements;
        limits.max_circuit_nodes = self.max_circuit_nodes;
        limits.max_matrix_unknowns = self.max_matrix_unknowns;
        limits.max_analysis_points = self.max_analysis_points;
        limits.max_result_values = self.max_result_values;
        limits.max_parallel_workers = self.max_parallel_workers;
        limits.max_batch_runs = self.max_batch_runs;
        limits
    }
}

impl Default for WasmResourceLimits {
    fn default() -> Self {
        Self::from_core(browser_resource_limits())
    }
}

/// Extensible options object accepted by every JavaScript export.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase", deny_unknown_fields)]
pub struct WasmExecutionOptions {
    pub resource_limits: WasmResourceLimits,
}

/// Frequency-grid convention for direct scalar STB execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WasmStbSweep {
    Linear,
    Decade,
    Octave,
}

impl WasmStbSweep {
    const fn to_core(self) -> StbSweepType {
        match self {
            Self::Linear => StbSweepType::Linear,
            Self::Decade => StbSweepType::Decade,
            Self::Octave => StbSweepType::Octave,
        }
    }

    fn parse(value: &str) -> DetailedWasmResult<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "lin" | "linear" => Ok(Self::Linear),
            "dec" | "decade" => Ok(Self::Decade),
            "oct" | "octave" => Ok(Self::Octave),
            _ => Err(Box::new(WasmError::invalid_argument(format!(
                "STB sweep must be 'linear', 'decade', or 'octave', got {value:?}"
            )))),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug)]
struct ExecutionDeadline(Option<std::time::Instant>);

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
struct ExecutionDeadline(Option<f64>);

impl ExecutionDeadline {
    fn new(timeout_milliseconds: Option<u32>) -> DetailedWasmResult<Self> {
        if let Some(timeout) = timeout_milliseconds
            && timeout > MAX_TIMEOUT_MILLISECONDS
        {
            return Err(Box::new(WasmError::invalid_argument(format!(
                "timeoutMilliseconds must not exceed {MAX_TIMEOUT_MILLISECONDS}, got {timeout}"
            ))));
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(Self(timeout_milliseconds.map(|timeout| {
                std::time::Instant::now() + std::time::Duration::from_millis(u64::from(timeout))
            })))
        }
        #[cfg(target_arch = "wasm32")]
        {
            let deadline = timeout_milliseconds
                .map(|timeout| monotonic_now_milliseconds().map(|now| now + f64::from(timeout)))
                .transpose()?;
            Ok(Self(deadline))
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn expired(&self) -> bool {
        self.0
            .is_some_and(|deadline| std::time::Instant::now() >= deadline)
    }

    #[cfg(target_arch = "wasm32")]
    fn expired(&self) -> bool {
        self.0.is_some_and(|deadline| {
            monotonic_now_milliseconds().map_or(true, |now| now >= deadline)
        })
    }
}

#[cfg(target_arch = "wasm32")]
fn monotonic_now_milliseconds() -> DetailedWasmResult<f64> {
    let global = js_sys::global();
    let performance =
        js_sys::Reflect::get(&global, &JsValue::from_str("performance")).map_err(|_| {
            Box::new(WasmError::invalid_argument(
                "timeoutMilliseconds requires a host performance clock".to_string(),
            ))
        })?;
    let now = js_sys::Reflect::get(&performance, &JsValue::from_str("now"))
        .ok()
        .and_then(|value| value.dyn_into::<js_sys::Function>().ok())
        .ok_or_else(|| {
            Box::new(WasmError::invalid_argument(
                "timeoutMilliseconds requires performance.now()".to_string(),
            ))
        })?;
    let value = now
        .call0(&performance)
        .ok()
        .and_then(|value| value.as_f64())
        .filter(|value| value.is_finite())
        .ok_or_else(|| {
            Box::new(WasmError::invalid_argument(
                "performance.now() did not return a finite timestamp".to_string(),
            ))
        })?;
    Ok(value)
}

/// Compose one frontend cancellation source with the per-call deadline.
/// Every browser analysis passes this object to an abort-aware core entrypoint.
struct ConfiguredAbort<'a> {
    external: &'a dyn AbortSignal,
    deadline: ExecutionDeadline,
}

impl<'a> ConfiguredAbort<'a> {
    fn new(
        timeout_milliseconds: Option<u32>,
        external: &'a dyn AbortSignal,
    ) -> DetailedWasmResult<Self> {
        Ok(Self {
            external,
            deadline: ExecutionDeadline::new(timeout_milliseconds)?,
        })
    }
}

impl AbortSignal for ConfiguredAbort<'_> {
    fn is_aborted(&self) -> bool {
        self.external.is_aborted() || self.deadline.expired()
    }
}

/// Browser-facing transient compression policy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase", deny_unknown_fields)]
pub struct WasmCompressionOptions {
    /// Absolute interpolation error in each channel's native units.
    pub absolute_tolerance: f64,
    /// Relative interpolation error as a fraction of the actual value.
    pub relative_tolerance: f64,
    /// Maximum retained time-axis gap. Zero disables the gap ceiling.
    pub maximum_interval: f64,
    /// Set false to preserve every accepted point while retaining explicit
    /// compression provenance.
    pub enabled: bool,
}

impl Default for WasmCompressionOptions {
    fn default() -> Self {
        let defaults = rspice_core::engine::CompressionConfig::default();
        Self {
            absolute_tolerance: defaults.abs_tol,
            relative_tolerance: defaults.rel_tol,
            maximum_interval: defaults.min_interval,
            enabled: defaults.enabled,
        }
    }
}

impl WasmCompressionOptions {
    fn to_core(&self) -> DetailedWasmResult<rspice_core::engine::CompressionConfig> {
        for (name, value) in [
            ("absoluteTolerance", self.absolute_tolerance),
            ("relativeTolerance", self.relative_tolerance),
            ("maximumInterval", self.maximum_interval),
        ] {
            if !value.is_finite() || value < 0.0 {
                return Err(Box::new(WasmError::invalid_argument(format!(
                    "transient compression {name} must be finite and non-negative, got {value}"
                ))));
            }
        }
        Ok(rspice_core::engine::CompressionConfig {
            abs_tol: self.absolute_tolerance,
            rel_tol: self.relative_tolerance,
            enabled: self.enabled,
            min_interval: self.maximum_interval,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetlistSummary {
    pub title: String,
    pub element_count: usize,
    pub analysis_count: usize,
    pub model_count: usize,
    pub subcircuit_count: usize,
    pub parameter_count: usize,
    pub diagnostics: Vec<WasmDiagnostic>,
    #[serde(default)]
    pub startup_diagnostics: Vec<WasmStartupDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmDiagnostic {
    pub line: usize,
    pub severity: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmSourceLocation {
    pub source: Option<String>,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmStartupDirectiveScope {
    pub kind: String,
    pub qualified_definition: Option<String>,
    pub qualified_instances: Vec<String>,
}

/// Stable structured representation of a non-fatal `.IC`/`.NODESET`
/// semantic diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmStartupDiagnostic {
    pub code: String,
    pub stage: String,
    pub directive: String,
    pub origins: Vec<WasmSourceLocation>,
    pub scopes: Vec<WasmStartupDirectiveScope>,
    pub canonical_nodes: Vec<String>,
}

/// Stable structured error exposed by the browser bindings.
///
/// The legacy human-readable message remains available verbatim. Consumers
/// that need reliable diagnostics should branch on `kind` and `category`
/// instead of parsing that message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmError {
    pub message: String,
    /// Cross-interface stable error code. `kind` remains as a compatibility
    /// alias for existing browser consumers.
    pub code: String,
    pub kind: String,
    pub category: String,
    pub retryable: bool,
    pub primary_source: Option<String>,
    pub primary_line: Option<usize>,
    #[serde(default)]
    pub related_source: Option<String>,
    #[serde(default)]
    pub related_line: Option<usize>,
    #[serde(default)]
    pub first_startup_kind: Option<String>,
    #[serde(default)]
    pub conflicting_startup_kind: Option<String>,
    #[serde(default)]
    pub iterations: Option<usize>,
    #[serde(default)]
    pub resource: Option<String>,
    #[serde(default)]
    pub requested: Option<usize>,
    #[serde(default)]
    pub limit: Option<usize>,
    #[serde(default)]
    pub subcircuit_name: Option<String>,
    #[serde(default)]
    pub canonical_subcircuit_name: Option<String>,
    #[serde(default)]
    pub instance_name: Option<String>,
    #[serde(default)]
    pub canonical_instance_name: Option<String>,
    #[serde(default)]
    pub qualified_instance_name: Option<String>,
    #[serde(default)]
    pub parameter_name: Option<String>,
    #[serde(default)]
    pub canonical_parameter_name: Option<String>,
    #[serde(default)]
    pub expression: Option<String>,
    #[serde(default)]
    pub output_directive: Option<String>,
    #[serde(default)]
    pub operator_name: Option<String>,
    #[serde(default)]
    pub function_name: Option<String>,
    #[serde(default)]
    pub identifier_name: Option<String>,
    #[serde(default)]
    pub missing_dependency: Option<String>,
    #[serde(default)]
    pub reason: Option<String>,
    pub unresolved_output_symbols: Vec<WasmUnresolvedOutputSymbol>,
}

/// One unresolved output symbol, preserved in the core validator's exact
/// diagnostic order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmUnresolvedOutputSymbol {
    pub directive: String,
    pub source: Option<String>,
    pub line: usize,
    pub operator: String,
    pub symbol: String,
    pub symbol_kind: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct JsWasmErrorDetails<'a> {
    message: &'a str,
    code: &'a str,
    kind: &'a str,
    category: &'a str,
    retryable: bool,
    primary_source: Option<&'a str>,
    primary_line: Option<usize>,
    related_source: Option<&'a str>,
    related_line: Option<usize>,
    first_startup_kind: Option<&'a str>,
    conflicting_startup_kind: Option<&'a str>,
    iterations: Option<usize>,
    resource: Option<&'a str>,
    requested: Option<usize>,
    limit: Option<usize>,
    subcircuit_name: Option<&'a str>,
    canonical_subcircuit_name: Option<&'a str>,
    instance_name: Option<&'a str>,
    canonical_instance_name: Option<&'a str>,
    qualified_instance_name: Option<&'a str>,
    parameter_name: Option<&'a str>,
    canonical_parameter_name: Option<&'a str>,
    expression: Option<&'a str>,
    output_directive: Option<&'a str>,
    operator_name: Option<&'a str>,
    function_name: Option<&'a str>,
    identifier_name: Option<&'a str>,
    missing_dependency: Option<&'a str>,
    reason: Option<&'a str>,
    unresolved_output_symbols: Vec<JsUnresolvedOutputSymbol<'a>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct JsUnresolvedOutputSymbol<'a> {
    directive: &'a str,
    source: Option<&'a str>,
    line: usize,
    operator: &'a str,
    symbol: &'a str,
    symbol_kind: &'a str,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DcOperatingPoint {
    pub node_names: Vec<String>,
    pub node_voltages: Vec<f64>,
    pub branch_names: Vec<String>,
    pub branch_currents: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexSeries {
    pub real: Vec<f64>,
    pub imag: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AcPointSnapshot {
    pub frequency: f64,
    pub node_names: Vec<String>,
    pub branch_names: Vec<String>,
    pub voltages: ComplexSeries,
    pub currents: ComplexSeries,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientSnapshot {
    pub time: Vec<f64>,
    /// Exact accepted integration intervals aligned with `time`.
    pub step_sizes: Vec<f64>,
    /// Core node count, retained explicitly so schema drift cannot hide an
    /// incomplete name or waveform inventory.
    pub num_nodes: usize,
    pub node_names: Vec<String>,
    /// Node waveforms in core node order. A projected-out waveform is `None`
    /// (`null` in JavaScript), while a retained zero-point waveform is an
    /// explicitly present empty typed array.
    pub voltages: Vec<Option<Vec<f64>>>,
    /// Branch identities in the same stable order as `branch_currents`.
    pub branch_names: Vec<String>,
    /// Branch-current waveforms in core branch order. `None` means the known
    /// branch was deliberately projected out of the result.
    pub branch_currents: Vec<Option<Vec<f64>>>,
    /// Requested device operating-point channels in core discovery order.
    pub device_op_traces: Vec<TransientDeviceOpSnapshot>,
    /// Typed non-solution device-store channels in core topology order.
    pub store_traces: Vec<TransientStoreSnapshot>,
    /// Source-authored transient FFT results in declaration order.
    pub fft_results: Vec<TransientFftSnapshot>,
    /// Compression provenance. Full accepted-grid results use `None`; a
    /// compressed result reports its original and retained point counts.
    pub compression: Option<TransientCompressionSnapshot>,
}

/// One requested device operating-point history.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientDeviceOpSnapshot {
    pub device_name: String,
    pub parameter: String,
    pub values: Vec<f64>,
}

/// One typed, non-solution device-store history.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientStoreSnapshot {
    pub name: String,
    pub values: Vec<f64>,
}

/// Provenance for a compressed transient result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientCompressionSnapshot {
    /// Version of the compression evidence contract.
    pub schema_version: u32,
    /// Stable compression-algorithm identifier.
    pub algorithm: String,
    /// Sample domain over which reconstruction error was measured.
    pub sample_domain: String,
    /// Whether decimation was enabled.
    pub enabled: bool,
    /// Applied absolute tolerance in each signal's native unit.
    pub absolute_tolerance: f64,
    /// Applied relative tolerance.
    pub relative_tolerance: f64,
    /// Applied maximum interval between retained samples.
    pub maximum_retained_interval: f64,
    pub input_points: usize,
    pub retained_points: usize,
    pub compression_ratio: f64,
    /// Worst final-grid reconstruction error, selected by tolerance use.
    pub worst_observed: Option<TransientCompressionErrorSnapshot>,
}

/// Browser-facing final-grid compression-error evidence.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientCompressionErrorSnapshot {
    pub signal_kind: String,
    pub canonical_name: String,
    pub input_sample_index: usize,
    pub time: f64,
    pub actual_value: f64,
    pub absolute_error: f64,
    pub relative_error: Option<f64>,
    pub allowed_tolerance: f64,
    pub tolerance_utilization: f64,
}

/// Columnar FFT bins. The JavaScript export materializes every field as a
/// typed array while the Rust API retains ordinary owned vectors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientFftBinsSnapshot {
    pub indices: Vec<usize>,
    pub frequencies: Vec<f64>,
    pub real: Vec<f64>,
    pub imaginary: Vec<f64>,
    pub magnitudes: Vec<f64>,
    pub phase_degrees: Vec<f64>,
}

/// Columnar magnitude-ranked harmonic report. Ordering is the exact ordering
/// produced by the core (descending magnitude, then source bin).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientFftHarmonicsSnapshot {
    pub ranks: Vec<usize>,
    pub bins: Vec<usize>,
    pub frequencies: Vec<f64>,
    pub magnitudes: Vec<f64>,
    pub magnitudes_db: Vec<f64>,
    pub phase_degrees: Vec<f64>,
}

/// Optional Xyce-compatible FFT figures emitted when `FFTOUT=1`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientFftMetricsSnapshot {
    pub fundamental_magnitude: f64,
    pub thd_ratio: f64,
    pub thd_db: f64,
    pub sndr_db: f64,
    pub enob_bits: f64,
    pub snr_db: f64,
    pub sfdr_db: f64,
    pub sfdr_spur_bin: Option<usize>,
    pub sfdr_spur_frequency: Option<f64>,
    pub largest_harmonics: TransientFftHarmonicsSnapshot,
}

/// Complete browser representation of one core `TransientFftResult`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransientFftSnapshot {
    /// Stable source-order identity for this transient post-process result.
    pub analysis_id: String,
    /// Stable identity of the direct transient result consumed by this FFT.
    pub parent_analysis_id: String,
    /// One-based source-order ordinal among FFT directives.
    pub ordinal: usize,
    /// `probe` or `expression`, allowing consumers to interpret `source_text`
    /// without parsing it heuristically.
    pub source_kind: String,
    /// Canonical probe spelling or the expression body retained by the parser.
    pub source_text: String,
    /// Display spelling of the authored source; expression bodies include
    /// their braces here.
    pub authored_output: String,
    /// Resolved scalar result-column spelling.
    pub output_name: String,
    pub physical_type: String,
    /// Effective unit of Cartesian coefficients, magnitudes, and
    /// magnitude-like metrics. Normalized spectra use `1` while retaining
    /// `physical_type`; an unnormalized parameter has no known unit.
    pub value_unit: Option<String>,
    pub start_time: f64,
    pub stop_time: f64,
    pub sample_interval: f64,
    pub point_count: usize,
    pub accurate_sampling: bool,
    pub format: String,
    pub mode: String,
    pub window: String,
    pub window_name: String,
    pub alpha: f64,
    pub coherent_gain: f64,
    pub frequency_resolution: f64,
    pub fundamental_bin: usize,
    pub minimum_metric_bin: usize,
    pub maximum_metric_bin: usize,
    pub bins: TransientFftBinsSnapshot,
    /// `null` in JavaScript when `FFTOUT` was not requested.
    pub metrics: Option<TransientFftMetricsSnapshot>,
}

/// A versioned analog result retained in WebAssembly memory.
///
/// JavaScript reads the descriptor-only metadata once, then calls
/// `readWindow(start, count)` to transfer a bounded slice of every aligned
/// numeric column as typed arrays. This avoids serializing a second full copy
/// of a large result into ordinary JavaScript arrays.
#[derive(Debug)]
#[wasm_bindgen]
pub struct WasmAnalogResultHandle {
    document: AnalogResultDocument,
    maximum_window_values: usize,
}

/// Versioned results from a complete authored analog deck.
///
/// The handle retains every coordinate-local result in WebAssembly memory.
/// Only descriptors and caller-bounded numeric windows cross into JavaScript.
#[derive(Debug)]
#[wasm_bindgen]
pub struct WasmDeckResultHandle {
    document: DeckResultDocument,
    maximum_window_values: usize,
}

/// Versioned STB result retained in WebAssembly memory.
///
/// Metadata contains the six scalar margins and all descriptors. Large
/// primary, Bode, and optional Nyquist columns cross the boundary only through
/// bounded typed-array windows.
#[derive(Debug)]
#[wasm_bindgen]
pub struct WasmStbResultHandle {
    document: StbResultDocument,
    maximum_window_values: usize,
}

fn stb_metadata_error(error: StbDocumentError) -> Box<WasmError> {
    match error {
        StbDocumentError::Aborted => Box::new(WasmError::from_simulation_error(
            rspice_core::engine::SimulationError::Aborted,
        )),
        StbDocumentError::Invalid(message) => Box::new(WasmError::new(
            message,
            "invalid_result_document",
            "result_validation",
        )),
        StbDocumentError::Allocation(message) => Box::new(WasmError::new(
            message,
            "result_allocation_failed",
            "result_transfer",
        )),
    }
}

fn stb_window_error(error: StbDocumentError) -> Box<WasmError> {
    match error {
        StbDocumentError::Aborted => Box::new(WasmError::from_simulation_error(
            rspice_core::engine::SimulationError::Aborted,
        )),
        StbDocumentError::Invalid(message) => Box::new(WasmError::new(
            message,
            "invalid_result_window",
            "result_transfer",
        )),
        StbDocumentError::Allocation(message) => Box::new(WasmError::new(
            message,
            "result_allocation_failed",
            "result_transfer",
        )),
    }
}

impl WasmStbResultHandle {
    fn new_with_abort(
        document: StbResultDocument,
        resource_limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> DetailedWasmResult<Self> {
        match document.validate_with_abort(abort) {
            Ok(()) => {}
            Err(stb_result_document::StbDocumentError::Aborted) => {
                return Err(Box::new(WasmError::from_simulation_error(
                    rspice_core::engine::SimulationError::Aborted,
                )));
            }
            Err(stb_result_document::StbDocumentError::Invalid(message)) => {
                return Err(Box::new(WasmError::new(
                    message,
                    "invalid_result_document",
                    "result_validation",
                )));
            }
            Err(stb_result_document::StbDocumentError::Allocation(message)) => {
                return Err(Box::new(WasmError::new(
                    message,
                    "result_allocation_failed",
                    "result_validation",
                )));
            }
        }
        let retained_values = document.retained_numeric_value_count().map_err(|message| {
            Box::new(WasmError::new(
                message,
                "invalid_result_document",
                "result_validation",
            ))
        })?;
        if retained_values > resource_limits.max_result_values {
            return Err(resource_limit_error(
                ResourceKind::ResultValues,
                retained_values,
                resource_limits.max_result_values,
            ));
        }
        Ok(Self {
            document,
            maximum_window_values: resource_limits
                .max_result_values
                .min(DEFAULT_MAX_TRANSFER_VALUES),
        })
    }

    /// Access the canonical Rust document without crossing the JS boundary.
    pub fn document(&self) -> &StbResultDocument {
        &self.document
    }

    fn metadata_snapshot(&self) -> DetailedWasmResult<StbResultMetadata> {
        self.document
            .metadata(self.maximum_window_values)
            .map_err(stb_metadata_error)
    }

    fn window_snapshot(&self, start: usize, count: usize) -> DetailedWasmResult<StbResultWindow> {
        self.document
            .window(start, count, self.maximum_window_values)
            .map_err(stb_window_error)
    }
}

#[wasm_bindgen]
impl WasmStbResultHandle {
    #[wasm_bindgen(getter, js_name = pointCount)]
    pub fn point_count(&self) -> usize {
        self.document.point_count
    }

    #[wasm_bindgen(getter, js_name = analysisId)]
    pub fn analysis_id(&self) -> String {
        self.document.analysis.id.clone()
    }

    /// Return STB descriptors, units, margins, status, and transfer ceiling
    /// without copying any per-frequency sample column.
    #[wasm_bindgen(js_name = metadata)]
    pub fn metadata_js(&self) -> Result<JsValue, JsValue> {
        let metadata = self
            .metadata_snapshot()
            .map_err(|error| wasm_error_to_js(*error))?;
        serialize_to_js(&metadata).map_err(|_| {
            wasm_error_to_js(WasmError::new(
                "failed to serialize STB result metadata".to_owned(),
                "result_serialization_failed",
                "result_transfer",
            ))
        })
    }

    /// Transfer one bounded half-open frequency range as typed arrays.
    #[wasm_bindgen(js_name = readWindow)]
    pub fn read_window_js(&self, start: usize, count: usize) -> Result<JsValue, JsValue> {
        let window = self
            .window_snapshot(start, count)
            .map_err(|error| wasm_error_to_js(*error))?;
        serialize_stb_result_window_to_js(&window).map_err(|_| {
            wasm_error_to_js(WasmError::new(
                "failed to serialize STB result window".to_owned(),
                "result_serialization_failed",
                "result_transfer",
            ))
        })
    }
}

impl WasmAnalogResultHandle {
    fn new(
        document: AnalogResultDocument,
        resource_limits: ResourceLimits,
    ) -> DetailedWasmResult<Self> {
        document.validate().map_err(|message| {
            Box::new(WasmError::new(
                message,
                "invalid_result_document",
                "result_validation",
            ))
        })?;
        let retained_values = document.retained_numeric_value_count();
        if retained_values > resource_limits.max_result_values {
            return Err(resource_limit_error(
                ResourceKind::ResultValues,
                retained_values,
                resource_limits.max_result_values,
            ));
        }
        Ok(Self {
            document,
            maximum_window_values: resource_limits
                .max_result_values
                .min(DEFAULT_MAX_TRANSFER_VALUES),
        })
    }

    /// Access the canonical Rust document without crossing the JS boundary.
    pub fn document(&self) -> &AnalogResultDocument {
        &self.document
    }

    fn metadata_snapshot(&self) -> AnalogResultMetadata {
        self.document.metadata(self.maximum_window_values)
    }

    fn window_snapshot(
        &self,
        start: usize,
        count: usize,
    ) -> DetailedWasmResult<AnalogResultWindow> {
        self.document
            .window(start, count, self.maximum_window_values)
            .map_err(|message| {
                Box::new(WasmError::new(
                    message,
                    "invalid_result_window",
                    "result_transfer",
                ))
            })
    }
}

impl WasmDeckResultHandle {
    fn new_with_abort(
        document: DeckResultDocument,
        resource_limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> DetailedWasmResult<Self> {
        document.validate_with_abort(abort).map_err(|message| {
            if abort.is_aborted() {
                aborted_error()
            } else {
                Box::new(WasmError::new(
                    message,
                    "invalid_deck_result_document",
                    "result_validation",
                ))
            }
        })?;
        let retained_values = document
            .retained_numeric_value_count_with_abort(abort)
            .map_err(|message| {
                if abort.is_aborted() {
                    aborted_error()
                } else {
                    Box::new(WasmError::new(
                        message,
                        "invalid_deck_result_document",
                        "result_validation",
                    ))
                }
            })?;
        if retained_values > resource_limits.max_result_values {
            return Err(resource_limit_error(
                ResourceKind::ResultValues,
                retained_values,
                resource_limits.max_result_values,
            ));
        }
        Ok(Self {
            document,
            maximum_window_values: resource_limits
                .max_result_values
                .min(DEFAULT_MAX_TRANSFER_VALUES),
        })
    }

    /// Access the canonical Rust document without crossing the JS boundary.
    pub fn document(&self) -> &DeckResultDocument {
        &self.document
    }
}

#[wasm_bindgen]
impl WasmDeckResultHandle {
    #[wasm_bindgen(getter, js_name = coordinateCount)]
    pub fn coordinate_count(&self) -> usize {
        self.document.coordinates.len()
    }

    #[wasm_bindgen(getter, js_name = resultCount)]
    pub fn result_count(&self) -> usize {
        self.document.results.len()
    }

    #[wasm_bindgen(getter, js_name = fftResultCount)]
    pub fn fft_result_count(&self) -> usize {
        self.document.fft_results.len()
    }

    /// Return aggregate axes, coordinates, stable namespaces, and compact
    /// result summaries without copying any numeric result column.
    #[wasm_bindgen(js_name = metadata)]
    pub fn metadata_js(&self) -> Result<JsValue, JsValue> {
        let metadata = self
            .document
            .metadata(self.maximum_window_values)
            .map_err(|message| {
                wasm_error_to_js(WasmError::new(
                    message,
                    "invalid_deck_result_document",
                    "result_validation",
                ))
            })?;
        serialize_to_js(&metadata).map_err(|_| {
            wasm_error_to_js(WasmError::new(
                "failed to serialize deck result metadata".to_owned(),
                "result_serialization_failed",
                "result_transfer",
            ))
        })
    }

    /// Return the coordinate-local schema for one analog result.
    #[wasm_bindgen(js_name = resultMetadata)]
    pub fn result_metadata_js(&self, result_index: usize) -> Result<JsValue, JsValue> {
        let metadata = self
            .document
            .result_metadata(result_index, self.maximum_window_values)
            .map_err(|message| {
                wasm_error_to_js(WasmError::new(
                    message,
                    "invalid_result_index",
                    "result_transfer",
                ))
            })?;
        serialize_to_js(&metadata).map_err(|_| {
            wasm_error_to_js(WasmError::new(
                "failed to serialize coordinate-local result metadata".to_owned(),
                "result_serialization_failed",
                "result_transfer",
            ))
        })
    }

    /// Transfer one bounded half-open window from one coordinate-local analog
    /// result as typed numeric and validity arrays.
    #[wasm_bindgen(js_name = readWindow)]
    pub fn read_window_js(
        &self,
        result_index: usize,
        start: usize,
        count: usize,
    ) -> Result<JsValue, JsValue> {
        let window = self
            .document
            .result_window(result_index, start, count, self.maximum_window_values)
            .map_err(|message| {
                wasm_error_to_js(WasmError::new(
                    message,
                    "invalid_result_window",
                    "result_transfer",
                ))
            })?;
        serialize_result_window_to_js(&window).map_err(|_| {
            wasm_error_to_js(WasmError::new(
                "failed to serialize coordinate-local result window".to_owned(),
                "result_serialization_failed",
                "result_transfer",
            ))
        })
    }

    /// Return complete scalar FFT configuration and metrics without copying
    /// bin or harmonic numeric columns.
    #[wasm_bindgen(js_name = fftMetadata)]
    pub fn fft_metadata_js(&self, fft_index: usize) -> Result<JsValue, JsValue> {
        let metadata = self
            .document
            .fft_metadata(fft_index, self.maximum_window_values)
            .map_err(|message| {
                wasm_error_to_js(WasmError::new(
                    message,
                    "invalid_fft_result_index",
                    "result_transfer",
                ))
            })?;
        serialize_to_js(&metadata).map_err(|_| {
            wasm_error_to_js(WasmError::new(
                "failed to serialize deck FFT metadata".to_owned(),
                "result_serialization_failed",
                "result_transfer",
            ))
        })
    }

    /// Transfer one bounded half-open FFT-bin window as typed arrays.
    #[wasm_bindgen(js_name = readFftBins)]
    pub fn read_fft_bins_js(
        &self,
        fft_index: usize,
        start: usize,
        count: usize,
    ) -> Result<JsValue, JsValue> {
        let window = self
            .document
            .fft_bin_window(fft_index, start, count, self.maximum_window_values)
            .map_err(|message| {
                wasm_error_to_js(WasmError::new(
                    message,
                    "invalid_fft_result_window",
                    "result_transfer",
                ))
            })?;
        serialize_deck_fft_bin_window_to_js(&window).map_err(|_| {
            wasm_error_to_js(WasmError::new(
                "failed to serialize deck FFT bin window".to_owned(),
                "result_serialization_failed",
                "result_transfer",
            ))
        })
    }

    /// Transfer one bounded half-open magnitude-ranked harmonic window.
    #[wasm_bindgen(js_name = readFftHarmonics)]
    pub fn read_fft_harmonics_js(
        &self,
        fft_index: usize,
        start: usize,
        count: usize,
    ) -> Result<JsValue, JsValue> {
        let window = self
            .document
            .fft_harmonic_window(fft_index, start, count, self.maximum_window_values)
            .map_err(|message| {
                wasm_error_to_js(WasmError::new(
                    message,
                    "invalid_fft_result_window",
                    "result_transfer",
                ))
            })?;
        serialize_deck_fft_harmonic_window_to_js(&window).map_err(|_| {
            wasm_error_to_js(WasmError::new(
                "failed to serialize deck FFT harmonic window".to_owned(),
                "result_serialization_failed",
                "result_transfer",
            ))
        })
    }
}

#[wasm_bindgen]
impl WasmAnalogResultHandle {
    #[wasm_bindgen(getter, js_name = pointCount)]
    pub fn point_count(&self) -> usize {
        self.document.point_count
    }

    #[wasm_bindgen(getter, js_name = analysisId)]
    pub fn analysis_id(&self) -> String {
        self.document.analysis.id.clone()
    }

    /// Return descriptors, units, identity, explicit coordinate absence, and
    /// the transfer ceiling without copying result samples.
    #[wasm_bindgen(js_name = metadata)]
    pub fn metadata_js(&self) -> Result<JsValue, JsValue> {
        serialize_to_js(&self.metadata_snapshot())
    }

    /// Transfer one bounded, half-open point range as typed numeric and
    /// validity arrays. Missing samples carry validity zero; their numeric
    /// slots are placeholders and must not be interpreted.
    #[wasm_bindgen(js_name = readWindow)]
    pub fn read_window_js(&self, start: usize, count: usize) -> Result<JsValue, JsValue> {
        let window = self
            .window_snapshot(start, count)
            .map_err(|error| wasm_error_to_js(*error))?;
        serialize_result_window_to_js(&window)
    }
}

/// Browser-facing parser-to-solver readiness result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WasmHealthReport {
    pub status: String,
    pub ready: bool,
    pub duration_seconds: f64,
    pub element_count: usize,
    pub node_count: usize,
    pub branch_count: usize,
    pub output_voltage: f64,
}

impl WasmError {
    fn new(message: String, kind: &str, category: &str) -> Self {
        Self {
            message,
            code: kind.to_string(),
            kind: kind.to_string(),
            category: category.to_string(),
            retryable: false,
            primary_source: None,
            primary_line: None,
            related_source: None,
            related_line: None,
            first_startup_kind: None,
            conflicting_startup_kind: None,
            iterations: None,
            resource: None,
            requested: None,
            limit: None,
            subcircuit_name: None,
            canonical_subcircuit_name: None,
            instance_name: None,
            canonical_instance_name: None,
            qualified_instance_name: None,
            parameter_name: None,
            canonical_parameter_name: None,
            expression: None,
            output_directive: None,
            operator_name: None,
            function_name: None,
            identifier_name: None,
            missing_dependency: None,
            reason: None,
            unresolved_output_symbols: Vec::new(),
        }
    }

    fn invalid_argument(message: String) -> Self {
        Self::new(message, "invalid_argument", "input_validation")
    }

    fn unsupported_cancellation(mechanism: String) -> Self {
        let mut error = Self::new(
            format!("unsupported cancellation mechanism '{mechanism}'; expected 'sharedInt32'"),
            "unsupported_cancellation",
            "cancellation",
        );
        error.reason = Some(mechanism);
        error
    }

    fn resource_limit(message: String, error: ResourceLimitError) -> Self {
        let mut structured = Self::new(message, "resource_limit", "resource_limit");
        structured.resource = Some(error.resource.as_str().to_string());
        structured.requested = Some(error.requested);
        structured.limit = Some(error.limit);
        structured
    }

    fn from_simulation_error(error: rspice_core::engine::SimulationError) -> Self {
        let descriptor = error.descriptor();
        let message = error.to_string();
        let mut structured = if let Some(resource) = descriptor.resource_limit {
            Self::resource_limit(message, resource)
        } else {
            Self::new(
                message,
                descriptor.code.as_str(),
                descriptor.category.as_str(),
            )
        };
        structured.code = descriptor.code.as_str().to_string();
        structured.kind = structured.code.clone();
        structured.category = descriptor.category.as_str().to_string();
        structured.retryable = descriptor.retryable;
        structured.iterations = descriptor.iterations;
        if let rspice_core::engine::SimulationError::BehavioralReference(error) = &error {
            structured.instance_name = Some(error.owner_name.clone());
            structured.canonical_instance_name = Some(error.canonical_owner_name.clone());
            structured.missing_dependency = Some(error.canonical_dependency_name.clone());
            structured.reason = Some(error.reason.as_str().to_string());
        }
        structured
    }

    fn from_parse_error(error: rspice_core::netlist::ParseError) -> Self {
        let message = error.to_string();
        match error {
            rspice_core::netlist::ParseError::ResourceLimit(error) => {
                Self::resource_limit(message, error)
            }
            rspice_core::netlist::ParseError::OutputSymbolValidation(error) => {
                let unresolved_output_symbols = error
                    .unresolved
                    .iter()
                    .map(|item| WasmUnresolvedOutputSymbol {
                        directive: output_directive_name(item.directive).to_string(),
                        source: source_path(&item.origin),
                        line: item.origin.line,
                        operator: item.operator.clone(),
                        symbol: item.symbol.clone(),
                        symbol_kind: output_symbol_kind_name(item.kind).to_string(),
                    })
                    .collect::<Vec<_>>();
                let primary = error.unresolved.first().map(|item| &item.origin);

                Self {
                    message,
                    code: "undefined_output_symbols".to_string(),
                    kind: "undefined_output_symbols".to_string(),
                    category: "output_symbol_validation".to_string(),
                    retryable: false,
                    primary_source: primary.and_then(source_path),
                    primary_line: primary.map(|origin| origin.line),
                    related_source: None,
                    related_line: None,
                    first_startup_kind: None,
                    conflicting_startup_kind: None,
                    iterations: None,
                    resource: None,
                    requested: None,
                    limit: None,
                    subcircuit_name: None,
                    canonical_subcircuit_name: None,
                    instance_name: None,
                    canonical_instance_name: None,
                    qualified_instance_name: None,
                    parameter_name: None,
                    canonical_parameter_name: None,
                    expression: None,
                    output_directive: None,
                    operator_name: None,
                    function_name: None,
                    identifier_name: None,
                    missing_dependency: None,
                    reason: None,
                    unresolved_output_symbols,
                }
            }
            rspice_core::netlist::ParseError::OutputExpressionValidation(error) => {
                use rspice_core::netlist::OutputExpressionIssue;

                let (kind, operator_name, function_name, identifier_name) = match &error.issue {
                    OutputExpressionIssue::UnknownFunction { function } => (
                        "unknown_output_function",
                        None,
                        Some(function.clone()),
                        None,
                    ),
                    OutputExpressionIssue::UnresolvedIdentifier { identifier } => (
                        "unresolved_output_identifier",
                        None,
                        None,
                        Some(identifier.clone()),
                    ),
                    OutputExpressionIssue::InvalidAccessor { operator, .. } => (
                        "invalid_output_accessor",
                        Some(operator.clone()),
                        None,
                        None,
                    ),
                    OutputExpressionIssue::UnresolvedDeviceParameter { .. } => {
                        ("unresolved_output_device_parameter", None, None, None)
                    }
                    OutputExpressionIssue::Syntax { .. } => {
                        ("invalid_output_expression_syntax", None, None, None)
                    }
                };
                let mut structured = Self::new(message, kind, "output_expression_validation");
                structured.primary_source = source_path(&error.origin);
                structured.primary_line = Some(error.origin.line);
                structured.expression = Some(error.expression);
                structured.output_directive = Some(error.directive.to_string());
                structured.operator_name = operator_name;
                structured.function_name = function_name;
                structured.identifier_name = identifier_name;
                if let OutputExpressionIssue::UnresolvedDeviceParameter { device, parameter } =
                    &error.issue
                {
                    structured.instance_name = Some(device.clone());
                    structured.parameter_name = Some(parameter.clone());
                }
                structured.reason = Some(error.issue.reason());
                structured
            }
            rspice_core::netlist::ParseError::StartupDirectiveConflict(error) => Self {
                message,
                code: "conflicting_startup_directives".to_string(),
                kind: "conflicting_startup_directives".to_string(),
                category: "startup_directive_validation".to_string(),
                retryable: false,
                primary_source: source_path(&error.first),
                primary_line: Some(error.first.line),
                related_source: source_path(&error.conflicting),
                related_line: Some(error.conflicting.line),
                first_startup_kind: Some(startup_directive_kind_name(error.first_kind).to_string()),
                conflicting_startup_kind: Some(
                    startup_directive_kind_name(error.conflicting_kind).to_string(),
                ),
                iterations: None,
                resource: None,
                requested: None,
                limit: None,
                subcircuit_name: None,
                canonical_subcircuit_name: None,
                instance_name: None,
                canonical_instance_name: None,
                qualified_instance_name: None,
                parameter_name: None,
                canonical_parameter_name: None,
                expression: None,
                output_directive: None,
                operator_name: None,
                function_name: None,
                identifier_name: None,
                missing_dependency: None,
                reason: None,
                unresolved_output_symbols: Vec::new(),
            },
            rspice_core::netlist::ParseError::UnresolvedSubcircuitParameter(error) => {
                let mut structured = Self::new(
                    message,
                    "unresolved_subcircuit_parameter",
                    "subcircuit_parameter_resolution",
                );
                structured.subcircuit_name = Some(error.subcircuit_name);
                structured.canonical_subcircuit_name = Some(error.canonical_subcircuit_name);
                structured.instance_name = Some(error.instance_name);
                structured.canonical_instance_name = Some(error.canonical_instance_name);
                structured.qualified_instance_name = Some(error.qualified_instance_name);
                structured.parameter_name = Some(error.parameter_name);
                structured.canonical_parameter_name = Some(error.canonical_parameter_name);
                structured.expression = Some(error.expression);
                structured.missing_dependency = error.missing_dependency;
                structured.reason = Some(error.reason);
                structured
            }
            rspice_core::netlist::ParseError::UndefinedSubcircuit(error) => {
                let mut structured =
                    Self::new(message, "undefined_subcircuit", "subcircuit_resolution");
                structured.subcircuit_name = Some(error.subcircuit_name);
                structured.canonical_subcircuit_name = Some(error.canonical_subcircuit_name);
                structured.instance_name = Some(error.instance_name);
                structured.canonical_instance_name = Some(error.canonical_instance_name);
                structured.qualified_instance_name = Some(error.qualified_instance_name);
                structured
            }
            rspice_core::netlist::ParseError::MissingDeviceModel(error) => {
                let mut structured =
                    Self::new(message, "missing_device_model", "device_model_resolution");
                structured.primary_line = Some(error.line);
                structured.instance_name = Some(error.device_name);
                structured.canonical_instance_name = Some(error.canonical_device_name);
                structured.reason = Some(error.device_type);
                structured
            }
            _ => Self::new(message, "parse_error", "netlist_parse"),
        }
    }
}

fn source_path(location: &rspice_core::netlist::NetlistSourceLocation) -> Option<String> {
    location
        .path
        .as_ref()
        .map(|path| path.to_string_lossy().into_owned())
}

fn startup_directive_kind_name(kind: rspice_core::netlist::StartupDirectiveKind) -> &'static str {
    match kind {
        rspice_core::netlist::StartupDirectiveKind::Ic => "ic",
        rspice_core::netlist::StartupDirectiveKind::NodeSet => "nodeset",
    }
}

fn output_directive_name(kind: rspice_core::netlist::OutputDirectiveKind) -> &'static str {
    use rspice_core::netlist::OutputDirectiveKind;
    match kind {
        OutputDirectiveKind::Save => "save",
        OutputDirectiveKind::Probe => "probe",
        OutputDirectiveKind::Print => "print",
        OutputDirectiveKind::Plot => "plot",
        OutputDirectiveKind::Measure => "measure",
        OutputDirectiveKind::Four => "four",
        OutputDirectiveKind::Fft => "fft",
    }
}

fn output_symbol_kind_name(kind: rspice_core::netlist::OutputSymbolKind) -> &'static str {
    match kind {
        rspice_core::netlist::OutputSymbolKind::Node => "node",
        rspice_core::netlist::OutputSymbolKind::Device => "device",
    }
}

fn parse_netlist_detailed(
    source: &str,
    resource_limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> DetailedWasmResult<Netlist> {
    Netlist::parse_validated_with_options_and_abort(
        source,
        rspice_core::netlist::NetlistParseOptions {
            resource_limits,
            ..rspice_core::netlist::NetlistParseOptions::default()
        },
        abort,
    )
    .map_err(|error| match error {
        rspice_core::netlist::ParseWithAbortError::Aborted => Box::new(
            WasmError::from_simulation_error(rspice_core::engine::SimulationError::Aborted),
        ),
        rspice_core::netlist::ParseWithAbortError::Parse(error) => {
            Box::new(WasmError::from_parse_error(error))
        }
    })
}

fn engine_with_resource_limits(resource_limits: ResourceLimits) -> DetailedWasmResult<Engine> {
    let config = SimulationConfig {
        resource_limits,
        ..SimulationConfig::default()
    };
    Engine::try_new(config).map_err(|error| {
        Box::new(WasmError::from_simulation_error(
            rspice_core::engine::SimulationError::Configuration(error),
        ))
    })
}

fn resource_limit_error(resource: ResourceKind, requested: usize, limit: usize) -> Box<WasmError> {
    let error = ResourceLimitError {
        resource,
        requested,
        limit,
    };
    Box::new(WasmError::resource_limit(error.to_string(), error))
}

fn aborted_error() -> Box<WasmError> {
    Box::new(WasmError::from_simulation_error(
        rspice_core::engine::SimulationError::Aborted,
    ))
}

fn ensure_not_aborted(abort: &dyn AbortSignal) -> DetailedWasmResult<()> {
    if abort.is_aborted() {
        Err(aborted_error())
    } else {
        Ok(())
    }
}

#[derive(Clone)]
struct JsSharedCancellationControl {
    view: js_sys::Int32Array,
    index: u32,
}

struct JsExecutionRequest {
    options: WasmExecutionOptions,
    timeout_milliseconds: Option<u32>,
    cancellation: Option<JsSharedCancellationControl>,
}

thread_local! {
    static ACTIVE_SHARED_CANCELLATION: std::cell::RefCell<Option<JsSharedCancellationControl>> =
        const { std::cell::RefCell::new(None) };
}

/// The signal itself owns no JavaScript handle and therefore satisfies the
/// core's Send + Sync contract without unsafe code. The browser build is
/// deliberately single-threaded; the per-agent control view lives in TLS.
struct JsSharedAbortSignal {
    enabled: bool,
}

impl AbortSignal for JsSharedAbortSignal {
    fn is_aborted(&self) -> bool {
        if !self.enabled {
            return false;
        }
        ACTIVE_SHARED_CANCELLATION.with(|active| {
            active
                .borrow()
                .as_ref()
                .is_none_or(|control| js_sys::Atomics::load(&control.view, control.index) != Ok(0))
        })
    }
}

struct ActiveSharedCancellationGuard {
    installed: bool,
}

impl ActiveSharedCancellationGuard {
    fn install(control: Option<JsSharedCancellationControl>) -> DetailedWasmResult<Self> {
        let installed = control.is_some();
        ACTIVE_SHARED_CANCELLATION.with(|active| {
            let mut active = active.borrow_mut();
            if active.is_some() {
                return Err(Box::new(WasmError::invalid_argument(
                    "nested WASM execution is not supported".to_string(),
                )));
            }
            *active = control;
            Ok(Self { installed })
        })
    }
}

impl Drop for ActiveSharedCancellationGuard {
    fn drop(&mut self) {
        if self.installed {
            ACTIVE_SHARED_CANCELLATION.with(|active| {
                *active.borrow_mut() = None;
            });
        }
    }
}

fn js_object_keys(value: &JsValue) -> Vec<String> {
    js_sys::Object::keys(value.unchecked_ref::<js_sys::Object>())
        .iter()
        .filter_map(|key| key.as_string())
        .collect()
}

fn optional_js_property(object: &JsValue, name: &str) -> DetailedWasmResult<Option<JsValue>> {
    let value = js_sys::Reflect::get(object, &JsValue::from_str(name)).map_err(|_| {
        Box::new(WasmError::invalid_argument(format!(
            "could not read execution option '{name}'"
        )))
    })?;
    // A missing JavaScript property reads as `undefined`. Preserve an
    // explicitly authored `null` so the field's type validator rejects it
    // instead of silently converting malformed input into a default.
    Ok((!value.is_undefined()).then_some(value))
}

fn shared_cancellation_from_js(value: JsValue) -> DetailedWasmResult<JsSharedCancellationControl> {
    if !value.is_object() || value.is_array() {
        return Err(Box::new(WasmError::invalid_argument(
            "cancellation must be an object".to_string(),
        )));
    }
    for key in js_object_keys(&value) {
        if !matches!(key.as_str(), "mechanism" | "view" | "index") {
            return Err(Box::new(WasmError::invalid_argument(format!(
                "unknown cancellation option '{key}'"
            ))));
        }
    }

    let mechanism = optional_js_property(&value, "mechanism")?
        .and_then(|value| value.as_string())
        .ok_or_else(|| {
            Box::new(WasmError::invalid_argument(
                "cancellation.mechanism must be the string 'sharedInt32'".to_string(),
            ))
        })?;
    if mechanism != "sharedInt32" {
        return Err(Box::new(WasmError::unsupported_cancellation(mechanism)));
    }

    let view = optional_js_property(&value, "view")?
        .and_then(|value| value.dyn_into::<js_sys::Int32Array>().ok())
        .ok_or_else(|| {
            Box::new(WasmError::invalid_argument(
                "cancellation.view must be an Int32Array over SharedArrayBuffer".to_string(),
            ))
        })?;
    let buffer = js_sys::Reflect::get(&view, &JsValue::from_str("buffer")).map_err(|_| {
        Box::new(WasmError::invalid_argument(
            "could not inspect cancellation.view.buffer".to_string(),
        ))
    })?;
    if !buffer.is_instance_of::<js_sys::SharedArrayBuffer>() {
        return Err(Box::new(WasmError::invalid_argument(
            "cancellation.view must use SharedArrayBuffer, not ArrayBuffer".to_string(),
        )));
    }

    let index = match optional_js_property(&value, "index")? {
        None => 0,
        Some(value) => {
            let number = value.as_f64().filter(|number| {
                number.is_finite()
                    && *number >= 0.0
                    && number.fract() == 0.0
                    && *number <= f64::from(u32::MAX)
            });
            number.ok_or_else(|| {
                Box::new(WasmError::invalid_argument(
                    "cancellation.index must be a non-negative integer".to_string(),
                ))
            })? as u32
        }
    };
    if index >= view.length() {
        return Err(Box::new(WasmError::invalid_argument(format!(
            "cancellation.index {index} is outside a view of length {}",
            view.length()
        ))));
    }
    js_sys::Atomics::load(&view, index).map_err(|_| {
        Box::new(WasmError::invalid_argument(
            "cancellation.view does not support Atomics.load".to_string(),
        ))
    })?;

    Ok(JsSharedCancellationControl { view, index })
}

fn execution_request_from_js(value: JsValue) -> DetailedWasmResult<JsExecutionRequest> {
    if value.is_undefined() || value.is_null() {
        return Ok(JsExecutionRequest {
            options: WasmExecutionOptions::default(),
            timeout_milliseconds: None,
            cancellation: None,
        });
    }
    if !value.is_object() || value.is_array() {
        return Err(Box::new(WasmError::invalid_argument(
            "execution options must be an object".to_string(),
        )));
    }
    for key in js_object_keys(&value) {
        if !matches!(
            key.as_str(),
            "resourceLimits" | "timeoutMilliseconds" | "cancellation"
        ) {
            return Err(Box::new(WasmError::invalid_argument(format!(
                "unknown execution option '{key}'"
            ))));
        }
    }

    let serializable = js_sys::Object::new();
    for name in ["resourceLimits"] {
        if let Some(field) = optional_js_property(&value, name)? {
            js_sys::Reflect::set(&serializable, &JsValue::from_str(name), &field).map_err(
                |_| {
                    Box::new(WasmError::invalid_argument(format!(
                        "could not decode execution option '{name}'"
                    )))
                },
            )?;
        }
    }
    let options: WasmExecutionOptions = serde_wasm_bindgen::from_value(serializable.into())
        .map_err(|error| {
            Box::new(WasmError::invalid_argument(format!(
                "invalid execution options: {error}"
            )))
        })?;
    let timeout_milliseconds = optional_js_property(&value, "timeoutMilliseconds")?
        .map(|value| {
            let number = value.as_f64().filter(|number| {
                number.is_finite()
                    && *number >= 0.0
                    && number.fract() == 0.0
                    && *number <= f64::from(MAX_TIMEOUT_MILLISECONDS)
            });
            number.map(|number| number as u32).ok_or_else(|| {
                Box::new(WasmError::invalid_argument(format!(
                    "timeoutMilliseconds must be an integer from 0 through {MAX_TIMEOUT_MILLISECONDS}"
                )))
            })
        })
        .transpose()?;
    let cancellation = optional_js_property(&value, "cancellation")?
        .map(shared_cancellation_from_js)
        .transpose()?;
    Ok(JsExecutionRequest {
        options,
        timeout_milliseconds,
        cancellation,
    })
}

fn compression_options_from_js(value: JsValue) -> DetailedWasmResult<WasmCompressionOptions> {
    if value.is_undefined() || value.is_null() {
        return Ok(WasmCompressionOptions::default());
    }
    serde_wasm_bindgen::from_value(value).map_err(|error| {
        Box::new(WasmError::invalid_argument(format!(
            "invalid transient compression options: {error}"
        )))
    })
}

fn serialize_to_js<T: Serialize>(value: &T) -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(value)
        .map_err(|err| JsValue::from_str(&format!("serialization failed: {err}")))
}

fn js_property(object: &JsValue, name: &str) -> Result<JsValue, JsValue> {
    js_sys::Reflect::get(object, &JsValue::from_str(name)).map_err(|_| {
        JsValue::from_str(&format!(
            "serialization failed: result property `{name}` is unavailable"
        ))
    })
}

fn set_float64_array(object: &JsValue, name: &str, values: &[f64]) -> Result<(), JsValue> {
    let values = js_sys::Float64Array::from(values);
    js_sys::Reflect::set(object, &JsValue::from_str(name), values.as_ref())
        .map(|_| ())
        .map_err(|_| {
            JsValue::from_str(&format!(
                "serialization failed: cannot publish result typed array `{name}`"
            ))
        })
}

fn set_float64_array_entry(
    array: &js_sys::Array,
    index: usize,
    values: &[f64],
    name: &str,
) -> Result<(), JsValue> {
    let index = u32::try_from(index).map_err(|_| {
        JsValue::from_str(&format!(
            "serialization failed: result `{name}` index exceeds JavaScript array bounds"
        ))
    })?;
    let values = js_sys::Float64Array::from(values);
    array.set(index, values.into());
    Ok(())
}

fn js_array_property(object: &JsValue, name: &str) -> Result<js_sys::Array, JsValue> {
    js_property(object, name)?
        .dyn_into::<js_sys::Array>()
        .map_err(|_| {
            JsValue::from_str(&format!(
                "serialization failed: result property `{name}` is not an array"
            ))
        })
}

fn publish_optional_waveforms_as_typed_arrays(
    object: &JsValue,
    name: &str,
    waveforms: &[Option<Vec<f64>>],
) -> Result<(), JsValue> {
    let serialized = js_array_property(object, name)?;
    for (index, waveform) in waveforms.iter().enumerate() {
        if let Some(values) = waveform {
            set_float64_array_entry(&serialized, index, values, name)?;
        }
    }
    Ok(())
}

fn publish_trace_values_as_typed_arrays<T>(
    object: &JsValue,
    name: &str,
    traces: &[T],
    values: impl Fn(&T) -> &[f64],
) -> Result<(), JsValue> {
    let serialized = js_array_property(object, name)?;
    for (index, trace) in traces.iter().enumerate() {
        let js_trace = serialized.get(u32::try_from(index).map_err(|_| {
            JsValue::from_str(&format!(
                "serialization failed: result `{name}` index exceeds JavaScript array bounds"
            ))
        })?);
        set_float64_array(&js_trace, "values", values(trace))?;
    }
    Ok(())
}

fn set_uint32_array(object: &JsValue, name: &str, values: &[usize]) -> Result<(), JsValue> {
    let values = values
        .iter()
        .copied()
        .map(u32::try_from)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| {
            JsValue::from_str(&format!(
                "serialization failed: transient FFT index `{name}` exceeds Uint32Array"
            ))
        })?;
    let values = js_sys::Uint32Array::from(values.as_slice());
    js_sys::Reflect::set(object, &JsValue::from_str(name), values.as_ref())
        .map(|_| ())
        .map_err(|_| {
            JsValue::from_str(&format!(
                "serialization failed: cannot publish transient FFT typed array `{name}`"
            ))
        })
}

fn set_uint8_array(object: &JsValue, name: &str, values: &[u8]) -> Result<(), JsValue> {
    let values = js_sys::Uint8Array::from(values);
    js_sys::Reflect::set(object, &JsValue::from_str(name), values.as_ref())
        .map(|_| ())
        .map_err(|_| {
            JsValue::from_str(&format!(
                "serialization failed: cannot publish typed validity array `{name}`"
            ))
        })
}

fn publish_fft_bins_as_typed_arrays(
    object: &JsValue,
    bins: &TransientFftBinsSnapshot,
) -> Result<(), JsValue> {
    set_uint32_array(object, "indices", &bins.indices)?;
    set_float64_array(object, "frequencies", &bins.frequencies)?;
    set_float64_array(object, "real", &bins.real)?;
    set_float64_array(object, "imaginary", &bins.imaginary)?;
    set_float64_array(object, "magnitudes", &bins.magnitudes)?;
    set_float64_array(object, "phase_degrees", &bins.phase_degrees)
}

fn publish_fft_harmonics_as_typed_arrays(
    object: &JsValue,
    harmonics: &TransientFftHarmonicsSnapshot,
) -> Result<(), JsValue> {
    set_uint32_array(object, "ranks", &harmonics.ranks)?;
    set_uint32_array(object, "bins", &harmonics.bins)?;
    set_float64_array(object, "frequencies", &harmonics.frequencies)?;
    set_float64_array(object, "magnitudes", &harmonics.magnitudes)?;
    set_float64_array(object, "magnitudes_db", &harmonics.magnitudes_db)?;
    set_float64_array(object, "phase_degrees", &harmonics.phase_degrees)
}

/// Serialize transient analog and FFT numeric columns as compact,
/// interoperable JavaScript typed arrays. Optional projected waveforms,
/// compression provenance, and FFT fields are deliberately encoded as `null`,
/// not omitted or `undefined`, so consumers can distinguish absence explicitly.
fn serialize_transient_to_js(snapshot: &TransientSnapshot) -> Result<JsValue, JsValue> {
    let serializer = serde_wasm_bindgen::Serializer::new().serialize_missing_as_null(true);
    let serialized = snapshot
        .serialize(&serializer)
        .map_err(|error| JsValue::from_str(&format!("serialization failed: {error}")))?;
    set_float64_array(&serialized, "time", &snapshot.time)?;
    set_float64_array(&serialized, "step_sizes", &snapshot.step_sizes)?;
    publish_optional_waveforms_as_typed_arrays(&serialized, "voltages", &snapshot.voltages)?;
    publish_optional_waveforms_as_typed_arrays(
        &serialized,
        "branch_currents",
        &snapshot.branch_currents,
    )?;
    publish_trace_values_as_typed_arrays(
        &serialized,
        "device_op_traces",
        &snapshot.device_op_traces,
        |trace| &trace.values,
    )?;
    publish_trace_values_as_typed_arrays(
        &serialized,
        "store_traces",
        &snapshot.store_traces,
        |trace| &trace.values,
    )?;

    let fft_results = js_array_property(&serialized, "fft_results")?;

    for (index, fft) in snapshot.fft_results.iter().enumerate() {
        let js_fft = fft_results.get(index as u32);
        let js_bins = js_property(&js_fft, "bins")?;
        publish_fft_bins_as_typed_arrays(&js_bins, &fft.bins)?;

        if let Some(metrics) = &fft.metrics {
            let js_metrics = js_property(&js_fft, "metrics")?;
            let js_harmonics = js_property(&js_metrics, "largest_harmonics")?;
            publish_fft_harmonics_as_typed_arrays(&js_harmonics, &metrics.largest_harmonics)?;
        }
    }

    Ok(serialized)
}

/// Serialize only a bounded analog-result window, replacing every numeric
/// Serde array with its compact JavaScript typed-array representation.
fn serialize_result_window_to_js(window: &AnalogResultWindow) -> Result<JsValue, JsValue> {
    let serializer = serde_wasm_bindgen::Serializer::new().serialize_missing_as_null(true);
    let serialized = window
        .serialize(&serializer)
        .map_err(|error| JsValue::from_str(&format!("serialization failed: {error}")))?;

    let axes = js_array_property(&serialized, "axes")?;
    for (index, axis) in window.axes.iter().enumerate() {
        let object = axes.get(u32::try_from(index).map_err(|_| {
            JsValue::from_str("serialization failed: result axis index exceeds JavaScript bounds")
        })?);
        set_float64_array(&object, "values", &axis.values)?;
    }

    let signals = js_array_property(&serialized, "signals")?;
    for (index, signal) in window.signals.iter().enumerate() {
        let object = signals.get(u32::try_from(index).map_err(|_| {
            JsValue::from_str("serialization failed: result signal index exceeds JavaScript bounds")
        })?);
        let values = js_property(&object, "values")?;
        match &signal.values {
            SignalWindowValues::Real {
                values: samples,
                validity,
            } => {
                set_float64_array(&values, "values", samples)?;
                set_uint8_array(&values, "validity", validity)?;
            }
            SignalWindowValues::Complex {
                real,
                imaginary,
                validity,
            } => {
                set_float64_array(&values, "real", real)?;
                set_float64_array(&values, "imaginary", imaginary)?;
                set_uint8_array(&values, "validity", validity)?;
            }
        }
    }
    Ok(serialized)
}

fn serialize_deck_fft_bin_window_to_js(window: &DeckFftBinWindow) -> Result<JsValue, JsValue> {
    let serialized = serialize_to_js(window)?;
    set_uint32_array(&serialized, "indices", &window.indices)?;
    set_float64_array(&serialized, "frequencies", &window.frequencies)?;
    set_float64_array(&serialized, "real", &window.real)?;
    set_float64_array(&serialized, "imaginary", &window.imaginary)?;
    set_float64_array(&serialized, "magnitudes", &window.magnitudes)?;
    set_float64_array(&serialized, "phaseDegrees", &window.phase_degrees)?;
    Ok(serialized)
}

fn serialize_deck_fft_harmonic_window_to_js(
    window: &DeckFftHarmonicWindow,
) -> Result<JsValue, JsValue> {
    let serialized = serialize_to_js(window)?;
    set_uint32_array(&serialized, "ranks", &window.ranks)?;
    set_uint32_array(&serialized, "bins", &window.bins)?;
    set_float64_array(&serialized, "frequencies", &window.frequencies)?;
    set_float64_array(&serialized, "magnitudes", &window.magnitudes)?;
    set_float64_array(&serialized, "magnitudesDb", &window.magnitudes_db)?;
    set_float64_array(&serialized, "phaseDegrees", &window.phase_degrees)?;
    Ok(serialized)
}

/// Serialize one bounded STB window, replacing every retained per-frequency
/// numeric column with a `Float64Array` while leaving optional Nyquist absence
/// explicit as `null`.
fn serialize_stb_result_window_to_js(window: &StbResultWindow) -> Result<JsValue, JsValue> {
    let serializer = serde_wasm_bindgen::Serializer::new().serialize_missing_as_null(true);
    let serialized = window
        .serialize(&serializer)
        .map_err(|error| JsValue::from_str(&format!("serialization failed: {error}")))?;

    let primary = js_property(&serialized, "primary")?;
    set_float64_array(&primary, "frequencies", &window.primary.frequencies)?;
    let primary_loop_gain = js_property(&primary, "loopGain")?;
    set_float64_array(&primary_loop_gain, "real", &window.primary.loop_gain.real)?;
    set_float64_array(
        &primary_loop_gain,
        "imaginary",
        &window.primary.loop_gain.imaginary,
    )?;

    let bode = js_property(&serialized, "bode")?;
    set_float64_array(&bode, "frequencies", &window.bode.frequencies)?;
    set_float64_array(&bode, "magnitudes", &window.bode.magnitudes)?;
    set_float64_array(&bode, "magnitudesDb", &window.bode.magnitudes_db)?;
    set_float64_array(&bode, "phaseDegrees", &window.bode.phase_degrees)?;
    let bode_loop_gain = js_property(&bode, "loopGain")?;
    set_float64_array(&bode_loop_gain, "real", &window.bode.loop_gain.real)?;
    set_float64_array(
        &bode_loop_gain,
        "imaginary",
        &window.bode.loop_gain.imaginary,
    )?;

    if let Some(nyquist) = &window.nyquist {
        let js_nyquist = js_property(&serialized, "nyquist")?;
        set_float64_array(&js_nyquist, "real", &nyquist.real)?;
        set_float64_array(&js_nyquist, "imaginary", &nyquist.imaginary)?;
        set_float64_array(&js_nyquist, "frequencies", &nyquist.frequencies)?;
    }
    Ok(serialized)
}

fn wasm_error_to_js(error: WasmError) -> JsValue {
    let js_error = js_sys::Error::new(&error.message);
    js_error.set_name("RSpiceError");
    let object: &JsValue = js_error.as_ref();

    let details = JsWasmErrorDetails {
        message: &error.message,
        code: &error.code,
        kind: &error.kind,
        category: &error.category,
        retryable: error.retryable,
        primary_source: error.primary_source.as_deref(),
        primary_line: error.primary_line,
        related_source: error.related_source.as_deref(),
        related_line: error.related_line,
        first_startup_kind: error.first_startup_kind.as_deref(),
        conflicting_startup_kind: error.conflicting_startup_kind.as_deref(),
        iterations: error.iterations,
        resource: error.resource.as_deref(),
        requested: error.requested,
        limit: error.limit,
        subcircuit_name: error.subcircuit_name.as_deref(),
        canonical_subcircuit_name: error.canonical_subcircuit_name.as_deref(),
        instance_name: error.instance_name.as_deref(),
        canonical_instance_name: error.canonical_instance_name.as_deref(),
        qualified_instance_name: error.qualified_instance_name.as_deref(),
        parameter_name: error.parameter_name.as_deref(),
        canonical_parameter_name: error.canonical_parameter_name.as_deref(),
        expression: error.expression.as_deref(),
        output_directive: error.output_directive.as_deref(),
        operator_name: error.operator_name.as_deref(),
        function_name: error.function_name.as_deref(),
        identifier_name: error.identifier_name.as_deref(),
        missing_dependency: error.missing_dependency.as_deref(),
        reason: error.reason.as_deref(),
        unresolved_output_symbols: error
            .unresolved_output_symbols
            .iter()
            .map(|item| JsUnresolvedOutputSymbol {
                directive: &item.directive,
                source: item.source.as_deref(),
                line: item.line,
                operator: &item.operator,
                symbol: &item.symbol,
                symbol_kind: &item.symbol_kind,
            })
            .collect(),
    };
    if let Ok(details) = serde_wasm_bindgen::to_value(&details) {
        for field in [
            "code",
            "kind",
            "category",
            "retryable",
            "primarySource",
            "primaryLine",
            "relatedSource",
            "relatedLine",
            "firstStartupKind",
            "conflictingStartupKind",
            "iterations",
            "resource",
            "requested",
            "limit",
            "subcircuitName",
            "canonicalSubcircuitName",
            "instanceName",
            "canonicalInstanceName",
            "qualifiedInstanceName",
            "parameterName",
            "canonicalParameterName",
            "expression",
            "outputDirective",
            "operatorName",
            "functionName",
            "identifierName",
            "missingDependency",
            "reason",
            "unresolvedOutputSymbols",
        ] {
            let key = JsValue::from_str(field);
            if let Ok(value) = js_sys::Reflect::get(&details, &key) {
                let _ = js_sys::Reflect::set(object, &key, &value);
            }
        }
        let _ = js_sys::Reflect::set(object, &JsValue::from_str("details"), &details);
    }

    js_error.into()
}

fn diagnostic_summary(diagnostic: &rspice_core::netlist::ParseDiagnostic) -> WasmDiagnostic {
    WasmDiagnostic {
        line: diagnostic.line,
        severity: match diagnostic.severity {
            rspice_core::netlist::DiagnosticSeverity::Warning => "warning".to_string(),
        },
        code: diagnostic.code.clone(),
        message: diagnostic.message.clone(),
    }
}

fn startup_diagnostic_summary(
    diagnostic: &rspice_core::netlist::StartupDiagnostic,
) -> WasmStartupDiagnostic {
    use rspice_core::netlist::{StartupDiagnosticStage, StartupDirectiveScope};

    WasmStartupDiagnostic {
        code: diagnostic.code.as_str().to_string(),
        stage: match diagnostic.stage {
            StartupDiagnosticStage::Parse => "parse",
            StartupDiagnosticStage::StartupTopology => "startup_topology",
        }
        .to_string(),
        directive: startup_directive_kind_name(diagnostic.kind).to_string(),
        origins: diagnostic
            .origins
            .iter()
            .map(|origin| WasmSourceLocation {
                source: source_path(origin),
                line: origin.line,
            })
            .collect(),
        scopes: diagnostic
            .scopes
            .iter()
            .map(|scope| match scope {
                StartupDirectiveScope::TopLevel => WasmStartupDirectiveScope {
                    kind: "top_level".to_string(),
                    qualified_definition: None,
                    qualified_instances: Vec::new(),
                },
                StartupDirectiveScope::Subcircuit {
                    qualified_definition,
                    qualified_instances,
                } => WasmStartupDirectiveScope {
                    kind: "subcircuit".to_string(),
                    qualified_definition: Some(qualified_definition.clone()),
                    qualified_instances: qualified_instances.clone(),
                },
            })
            .collect(),
        canonical_nodes: diagnostic.canonical_nodes.clone(),
    }
}

fn complex_series_from_slice(values: &[rspice_core::Complex64]) -> ComplexSeries {
    ComplexSeries {
        real: values.iter().map(|value| value.re).collect(),
        imag: values.iter().map(|value| value.im).collect(),
    }
}

fn fft_output_identity(output: &FftOutput) -> (&'static str, &str, String) {
    match output {
        FftOutput::Probe(probe) => ("probe", probe, probe.clone()),
        FftOutput::Expression(expression) => {
            ("expression", expression, format!("{{{expression}}}"))
        }
    }
}

const fn fft_format_name(format: FftFormat) -> &'static str {
    match format {
        FftFormat::Normalized => "normalized",
        FftFormat::Unnormalized => "unnormalized",
    }
}

const fn fft_mode_name(mode: XyceFftMode) -> &'static str {
    match mode {
        XyceFftMode::HspiceCompatible => "hspice_compatible",
        XyceFftMode::SpectreCompatible => "spectre_compatible",
    }
}

const fn fft_window_name(window: FftWindow) -> &'static str {
    match window {
        FftWindow::Rectangular => "rectangular",
        FftWindow::Bartlett => "bartlett",
        FftWindow::BartlettHann => "bartlett_hann",
        FftWindow::Hamming => "hamming",
        FftWindow::Hann => "hann",
        FftWindow::Blackman67Db => "blackman_67db",
        FftWindow::Blackman => "blackman",
        FftWindow::BlackmanHarris => "blackman_harris",
        FftWindow::Nuttall => "nuttall",
        FftWindow::HalfCycleSine => "half_cycle_sine",
        FftWindow::HalfCycleSine3 => "half_cycle_sine_3",
        FftWindow::HalfCycleSine6 => "half_cycle_sine_6",
        FftWindow::Cosine2 => "cosine_2",
        FftWindow::Cosine4 => "cosine_4",
    }
}

fn fft_harmonics_snapshot(harmonics: &[TransientFftHarmonic]) -> TransientFftHarmonicsSnapshot {
    TransientFftHarmonicsSnapshot {
        ranks: harmonics.iter().map(|harmonic| harmonic.rank).collect(),
        bins: harmonics.iter().map(|harmonic| harmonic.bin).collect(),
        frequencies: harmonics
            .iter()
            .map(|harmonic| harmonic.frequency)
            .collect(),
        magnitudes: harmonics
            .iter()
            .map(|harmonic| harmonic.magnitude)
            .collect(),
        magnitudes_db: harmonics
            .iter()
            .map(|harmonic| harmonic.magnitude_db)
            .collect(),
        phase_degrees: harmonics
            .iter()
            .map(|harmonic| harmonic.phase_degrees)
            .collect(),
    }
}

fn fft_metrics_snapshot(metrics: &TransientFftMetrics) -> TransientFftMetricsSnapshot {
    TransientFftMetricsSnapshot {
        fundamental_magnitude: metrics.fundamental_magnitude,
        thd_ratio: metrics.thd_ratio,
        thd_db: metrics.thd_db,
        sndr_db: metrics.sndr_db,
        enob_bits: metrics.enob_bits,
        snr_db: metrics.snr_db,
        sfdr_db: metrics.sfdr_db,
        sfdr_spur_bin: metrics.sfdr_spur_bin,
        sfdr_spur_frequency: metrics.sfdr_spur_frequency,
        largest_harmonics: fft_harmonics_snapshot(&metrics.largest_harmonics),
    }
}

fn fft_value_unit(physical_type: &str, format: FftFormat) -> Result<Option<&'static str>, String> {
    let physical_unit = match physical_type {
        "voltage" => Some("V"),
        "current" => Some("A"),
        "parameter" => None,
        other => {
            return Err(format!("unsupported transient FFT physical type '{other}'"));
        }
    };
    Ok(match format {
        FftFormat::Normalized => Some("1"),
        FftFormat::Unnormalized => physical_unit,
    })
}

fn fft_snapshot(
    result: &TransientFftResult,
    ordinal: usize,
    parent_analysis_id: &str,
) -> Result<TransientFftSnapshot, String> {
    let (source_kind, source_text, authored_output) = fft_output_identity(&result.output);
    let value_unit = fft_value_unit(result.physical_type, result.format)?;
    Ok(TransientFftSnapshot {
        analysis_id: format!("fft-{ordinal:03}"),
        parent_analysis_id: parent_analysis_id.to_owned(),
        ordinal,
        source_kind: source_kind.to_string(),
        source_text: source_text.to_string(),
        authored_output,
        output_name: result.output_name.clone(),
        physical_type: result.physical_type.to_string(),
        value_unit: value_unit.map(str::to_string),
        start_time: result.start_time,
        stop_time: result.stop_time,
        sample_interval: result.sample_interval,
        point_count: result.point_count,
        accurate_sampling: result.accurate_sampling,
        format: fft_format_name(result.format).to_string(),
        mode: fft_mode_name(result.mode).to_string(),
        window: fft_window_name(result.window).to_string(),
        window_name: result.window_name.clone(),
        alpha: result.alpha,
        coherent_gain: result.coherent_gain,
        frequency_resolution: result.frequency_resolution,
        fundamental_bin: result.fundamental_bin,
        minimum_metric_bin: result.minimum_metric_bin,
        maximum_metric_bin: result.maximum_metric_bin,
        bins: TransientFftBinsSnapshot {
            indices: result.bins.iter().map(|bin| bin.index).collect(),
            frequencies: result.bins.iter().map(|bin| bin.frequency).collect(),
            real: result.bins.iter().map(|bin| bin.real).collect(),
            imaginary: result.bins.iter().map(|bin| bin.imaginary).collect(),
            magnitudes: result.bins.iter().map(|bin| bin.magnitude).collect(),
            phase_degrees: result.bins.iter().map(|bin| bin.phase_degrees).collect(),
        },
        metrics: result.metrics.as_ref().map(fft_metrics_snapshot),
    })
}

#[allow(clippy::too_many_arguments)]
fn validate_transient_analog_inventory(
    time: &[f64],
    step_sizes: &[f64],
    num_nodes: usize,
    node_names: &[String],
    voltages: &[Vec<f64>],
    branch_names: &[String],
    branch_currents: &[Vec<f64>],
    device_op_traces: &[rspice_core::engine::TransientDeviceOpTrace],
    store_traces: &[rspice_core::engine::TransientStoreTrace],
) -> Result<(), String> {
    let point_count = time.len();
    if step_sizes.len() != point_count {
        return Err(format!(
            "transient result has {} step sizes for {point_count} time points",
            step_sizes.len()
        ));
    }
    if num_nodes != node_names.len() || num_nodes != voltages.len() {
        return Err(format!(
            "transient result declares {num_nodes} nodes but has {} node names and {} voltage channels",
            node_names.len(),
            voltages.len()
        ));
    }
    if branch_names.len() != branch_currents.len() {
        return Err(format!(
            "transient result has {} branch names but {} branch-current channels",
            branch_names.len(),
            branch_currents.len()
        ));
    }
    if time
        .windows(2)
        .any(|window| !window[0].is_finite() || window[1] <= window[0])
        || time.last().is_some_and(|value| !value.is_finite())
    {
        return Err(
            "transient result time points must be finite and strictly increasing".to_string(),
        );
    }
    if step_sizes
        .iter()
        .any(|step| !step.is_finite() || *step < 0.0)
    {
        return Err("transient result step sizes must be finite and non-negative".to_string());
    }

    for (kind, name, values, may_be_projected_out) in
        voltages
            .iter()
            .enumerate()
            .map(|(index, values)| ("voltage", node_names[index].as_str(), values, true))
            .chain(branch_currents.iter().enumerate().map(|(index, values)| {
                ("branch-current", branch_names[index].as_str(), values, true)
            }))
            .chain(device_op_traces.iter().map(|trace| {
                (
                    "device operating-point",
                    trace.parameter.as_str(),
                    &trace.values,
                    false,
                )
            }))
            .chain(
                store_traces
                    .iter()
                    .map(|trace| ("device store", trace.name.as_str(), &trace.values, false)),
            )
    {
        if values.len() != point_count && !(may_be_projected_out && values.is_empty()) {
            return Err(format!(
                "transient {kind} channel '{name}' has {} values for {point_count} time points",
                values.len()
            ));
        }
    }
    Ok(())
}

fn solution_waveforms(waveforms: Vec<Vec<f64>>, point_count: usize) -> Vec<Option<Vec<f64>>> {
    waveforms
        .into_iter()
        .map(|waveform| {
            if waveform.is_empty() && point_count != 0 {
                None
            } else {
                Some(waveform)
            }
        })
        .collect()
}

fn device_op_snapshots(
    traces: Vec<rspice_core::engine::TransientDeviceOpTrace>,
) -> Vec<TransientDeviceOpSnapshot> {
    traces
        .into_iter()
        .map(|trace| TransientDeviceOpSnapshot {
            device_name: trace.device_name,
            parameter: trace.parameter,
            values: trace.values,
        })
        .collect()
}

fn store_snapshots(
    traces: Vec<rspice_core::engine::TransientStoreTrace>,
) -> Vec<TransientStoreSnapshot> {
    traces
        .into_iter()
        .map(|trace| TransientStoreSnapshot {
            name: trace.name,
            values: trace.values,
        })
        .collect()
}

/// Convert a complete core transient result into the loss-aware browser DTO.
/// Solution-channel vector order is preserved exactly; an empty projected-out
/// voltage or branch-current channel becomes typed `None`/JavaScript `null`.
pub fn transient_snapshot_from_result(
    result: TransientResult,
) -> Result<TransientSnapshot, String> {
    validate_transient_analog_inventory(
        &result.time,
        &result.step_sizes,
        result.num_nodes,
        &result.node_names,
        &result.voltages,
        &result.branch_names,
        &result.branch_currents,
        &result.device_op_traces,
        &result.store_traces,
    )?;
    let point_count = result.time.len();
    let fft_results = result
        .fft_results
        .iter()
        .enumerate()
        .map(|(index, result)| fft_snapshot(result, index + 1, "tran-001"))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(TransientSnapshot {
        time: result.time,
        step_sizes: result.step_sizes,
        num_nodes: result.num_nodes,
        node_names: result.node_names,
        voltages: solution_waveforms(result.voltages, point_count),
        branch_names: result.branch_names,
        branch_currents: solution_waveforms(result.branch_currents, point_count),
        device_op_traces: device_op_snapshots(result.device_op_traces),
        store_traces: store_snapshots(result.store_traces),
        fft_results,
        compression: None,
    })
}

/// Convert a validated compressed core transient into the same browser DTO.
/// Compression provenance is retained rather than inferred from the grid.
pub fn transient_snapshot_from_compressed_result(
    result: TransientResultCompressed,
) -> Result<TransientSnapshot, String> {
    result.validate()?;
    validate_transient_analog_inventory(
        &result.time,
        &result.step_sizes,
        result.num_nodes,
        &result.node_names,
        &result.voltages,
        &result.branch_names,
        &result.branch_currents,
        &result.device_op_traces,
        &result.store_traces,
    )?;
    let point_count = result.time.len();
    let fft_results = result
        .fft_results
        .iter()
        .enumerate()
        .map(|(index, result)| fft_snapshot(result, index + 1, "tran-001"))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(TransientSnapshot {
        time: result.time,
        step_sizes: result.step_sizes,
        num_nodes: result.num_nodes,
        node_names: result.node_names,
        voltages: solution_waveforms(result.voltages, point_count),
        branch_names: result.branch_names,
        branch_currents: solution_waveforms(result.branch_currents, point_count),
        device_op_traces: device_op_snapshots(result.device_op_traces),
        store_traces: store_snapshots(result.store_traces),
        fft_results,
        compression: Some(TransientCompressionSnapshot {
            schema_version: result.compression_report.schema_version,
            algorithm: result.compression_report.algorithm.as_str().to_string(),
            sample_domain: result.compression_report.sample_domain.as_str().to_string(),
            enabled: result.compression_report.applied_policy.enabled,
            absolute_tolerance: result.compression_report.applied_policy.absolute_tolerance,
            relative_tolerance: result.compression_report.applied_policy.relative_tolerance,
            maximum_retained_interval: result
                .compression_report
                .applied_policy
                .maximum_retained_interval,
            input_points: result.input_points,
            retained_points: point_count,
            compression_ratio: result.compression_ratio,
            worst_observed: result.compression_report.worst_observed.map(|observation| {
                TransientCompressionErrorSnapshot {
                    signal_kind: observation.signal.kind.as_str().to_string(),
                    canonical_name: observation.signal.canonical_name,
                    input_sample_index: observation.input_sample_index,
                    time: observation.time,
                    actual_value: observation.actual_value,
                    absolute_error: observation.absolute_error,
                    relative_error: observation.relative_error,
                    allowed_tolerance: observation.allowed_tolerance,
                    tolerance_utilization: observation.tolerance_utilization,
                }
            }),
        }),
    })
}

/// Summarize and semantically validate a netlist, returning typed diagnostics.
pub fn summarize_netlist_detailed(source: &str) -> DetailedWasmResult<NetlistSummary> {
    summarize_netlist_with_options_detailed(source, &WasmExecutionOptions::default())
}

/// Summarize a netlist under an explicit browser execution policy.
pub fn summarize_netlist_with_options_detailed(
    source: &str,
    options: &WasmExecutionOptions,
) -> DetailedWasmResult<NetlistSummary> {
    summarize_netlist_with_options_and_abort_detailed(source, options, &NoAbort)
}

/// Summarize a netlist while observing an explicit cooperative abort source.
pub fn summarize_netlist_with_options_and_abort_detailed(
    source: &str,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<NetlistSummary> {
    let netlist =
        parse_netlist_detailed(source, options.resource_limits.to_core(), external_abort)?;
    let startup_diagnostics = netlist
        .startup_diagnostics()
        .iter()
        .map(startup_diagnostic_summary)
        .collect();
    let summary = NetlistSummary {
        title: netlist.title,
        element_count: netlist.elements.len(),
        analysis_count: netlist.analyses.len(),
        model_count: netlist.models.len(),
        subcircuit_count: netlist.subcircuits.len(),
        parameter_count: netlist.params.all_params().len(),
        diagnostics: netlist.diagnostics.iter().map(diagnostic_summary).collect(),
        startup_diagnostics,
    };
    ensure_not_aborted(external_abort)?;
    Ok(summary)
}

/// Backward-compatible string-error summary API.
pub fn summarize_netlist(source: &str) -> WasmResult<NetlistSummary> {
    summarize_netlist_detailed(source).map_err(|error| error.message)
}

/// Run an operating point after strict semantic validation.
pub fn run_dc_operating_point_detailed(source: &str) -> DetailedWasmResult<DcOperatingPoint> {
    run_dc_operating_point_with_options_detailed(source, &WasmExecutionOptions::default())
}

/// Run an operating point under an explicit browser execution policy.
pub fn run_dc_operating_point_with_options_detailed(
    source: &str,
    options: &WasmExecutionOptions,
) -> DetailedWasmResult<DcOperatingPoint> {
    run_dc_operating_point_with_options_and_abort_detailed(source, options, &NoAbort)
}

/// Run an operating point under an explicit browser policy and cooperative
/// abort source.
pub fn run_dc_operating_point_with_options_and_abort_detailed(
    source: &str,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<DcOperatingPoint> {
    let resource_limits = options.resource_limits.to_core();
    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    let result = engine_with_resource_limits(resource_limits)?
        .run_dc_op_with_abort(&netlist, external_abort)
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
    ensure_not_aborted(external_abort)?;
    Ok(DcOperatingPoint {
        node_names: result.node_names,
        node_voltages: result.node_voltages,
        branch_names: result.branch_names,
        branch_currents: result.branch_currents,
    })
}

/// Backward-compatible string-error operating-point API.
pub fn run_dc_operating_point(source: &str) -> WasmResult<DcOperatingPoint> {
    run_dc_operating_point_detailed(source).map_err(|error| error.message)
}

fn validate_analysis_ordinal(ordinal: usize) -> DetailedWasmResult<()> {
    if ordinal == 0 {
        return Err(Box::new(WasmError::invalid_argument(
            "analysis ordinal must be one-based".to_owned(),
        )));
    }
    Ok(())
}

/// Run OP into the versioned, loss-aware analog document. Unlike the legacy
/// compatibility DTO, this retains engine-owned device observables and device
/// operating regions in addition to node voltages and branch currents.
pub fn run_operating_point_document_with_options_and_abort_detailed(
    source: &str,
    ordinal: usize,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<AnalogResultDocument> {
    validate_analysis_ordinal(ordinal)?;
    ensure_not_aborted(external_abort)?;
    let resource_limits = options.resource_limits.to_core();
    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    let (result, report) = engine_with_resource_limits(resource_limits)?
        .run_dc_op_with_report_and_abort(&netlist, external_abort)
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
    ensure_not_aborted(external_abort)?;
    result_document::operating_point_document(result, report, ordinal).map_err(|message| {
        Box::new(WasmError::new(
            message,
            "invalid_result_document",
            "result_validation",
        ))
    })
}

pub fn run_operating_point_document_detailed(
    source: &str,
) -> DetailedWasmResult<AnalogResultDocument> {
    run_operating_point_document_with_options_and_abort_detailed(
        source,
        1,
        &WasmExecutionOptions::default(),
        &NoAbort,
    )
}

/// Run a scalar-deck DC sweep into one typed document. The adapter unions
/// device observables across points and marks coordinate-local absence
/// explicitly instead of zero-filling it.
#[allow(clippy::too_many_arguments)]
pub fn run_dc_sweep_document_with_options_and_abort_detailed(
    source: &str,
    source_name: &str,
    start: f64,
    stop: f64,
    step: f64,
    ordinal: usize,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<AnalogResultDocument> {
    validate_analysis_ordinal(ordinal)?;
    ensure_not_aborted(external_abort)?;
    if source_name.trim().is_empty() {
        return Err(Box::new(WasmError::invalid_argument(
            "DC sweep source name must not be empty".to_owned(),
        )));
    }
    if !start.is_finite() || !stop.is_finite() || !step.is_finite() || step == 0.0 {
        return Err(Box::new(WasmError::invalid_argument(
            "DC sweep start/stop must be finite and step must be finite and nonzero".to_owned(),
        )));
    }
    let resource_limits = options.resource_limits.to_core();
    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    let points = engine_with_resource_limits(resource_limits)?
        .run_dc_sweep_with_report_and_abort(
            &netlist,
            source_name,
            start,
            stop,
            step,
            external_abort,
        )
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
    ensure_not_aborted(external_abort)?;
    result_document::dc_sweep_document(source_name, points, ordinal).map_err(|message| {
        Box::new(WasmError::new(
            message,
            "invalid_result_document",
            "result_validation",
        ))
    })
}

pub fn run_dc_sweep_document_detailed(
    source: &str,
    source_name: &str,
    start: f64,
    stop: f64,
    step: f64,
) -> DetailedWasmResult<AnalogResultDocument> {
    run_dc_sweep_document_with_options_and_abort_detailed(
        source,
        source_name,
        start,
        stop,
        step,
        1,
        &WasmExecutionOptions::default(),
        &NoAbort,
    )
}

/// Run AC analysis after strict semantic validation.
pub fn run_ac_analysis_detailed(
    source: &str,
    frequencies: &[f64],
) -> DetailedWasmResult<Vec<AcPointSnapshot>> {
    run_ac_analysis_with_options_detailed(source, frequencies, &WasmExecutionOptions::default())
}

/// Run AC analysis under an explicit browser execution policy.
pub fn run_ac_analysis_with_options_detailed(
    source: &str,
    frequencies: &[f64],
    options: &WasmExecutionOptions,
) -> DetailedWasmResult<Vec<AcPointSnapshot>> {
    run_ac_analysis_with_options_and_abort_detailed(source, frequencies, options, &NoAbort)
}

/// Run AC analysis under an explicit browser policy and cooperative abort
/// source.
pub fn run_ac_analysis_with_options_and_abort_detailed(
    source: &str,
    frequencies: &[f64],
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<Vec<AcPointSnapshot>> {
    ensure_not_aborted(external_abort)?;
    if frequencies.is_empty() {
        return Err(Box::new(WasmError::invalid_argument(
            "AC analysis requires at least one frequency".to_string(),
        )));
    }
    let resource_limits = options.resource_limits.to_core();
    if frequencies.len() > resource_limits.max_analysis_points {
        return Err(resource_limit_error(
            ResourceKind::AnalysisPoints,
            frequencies.len(),
            resource_limits.max_analysis_points,
        ));
    }
    if let Some((index, frequency)) = frequencies
        .iter()
        .copied()
        .enumerate()
        .find(|(_, frequency)| !frequency.is_finite() || *frequency < 0.0)
    {
        return Err(Box::new(WasmError::invalid_argument(format!(
            "AC frequency at index {index} must be finite and non-negative, got {frequency}"
        ))));
    }

    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    let results = engine_with_resource_limits(resource_limits)?
        .run_ac_with_abort(&netlist, frequencies, external_abort)
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;

    let snapshots = results
        .into_iter()
        .map(|point| AcPointSnapshot {
            frequency: point.frequency,
            node_names: point.node_names,
            branch_names: point.branch_names,
            voltages: complex_series_from_slice(&point.voltages),
            currents: complex_series_from_slice(&point.currents),
        })
        .collect();
    ensure_not_aborted(external_abort)?;
    Ok(snapshots)
}

/// Backward-compatible string-error AC API.
pub fn run_ac_analysis(source: &str, frequencies: &[f64]) -> WasmResult<Vec<AcPointSnapshot>> {
    run_ac_analysis_detailed(source, frequencies).map_err(|error| error.message)
}

/// Run AC into the common versioned analog document, preserving complex node
/// voltages and branch currents as aligned series.
pub fn run_ac_document_with_options_and_abort_detailed(
    source: &str,
    frequencies: &[f64],
    ordinal: usize,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<AnalogResultDocument> {
    validate_analysis_ordinal(ordinal)?;
    let points = run_ac_analysis_with_options_and_abort_detailed(
        source,
        frequencies,
        options,
        external_abort,
    )?;
    result_document::ac_document(points, ordinal).map_err(|message| {
        Box::new(WasmError::new(
            message,
            "invalid_result_document",
            "result_validation",
        ))
    })
}

pub fn run_ac_document_detailed(
    source: &str,
    frequencies: &[f64],
) -> DetailedWasmResult<AnalogResultDocument> {
    run_ac_document_with_options_and_abort_detailed(
        source,
        frequencies,
        1,
        &WasmExecutionOptions::default(),
        &NoAbort,
    )
}

/// Run transient analysis after strict semantic validation.
pub fn run_transient_analysis_detailed(
    source: &str,
    tstop: f64,
    max_step: f64,
) -> DetailedWasmResult<TransientSnapshot> {
    run_transient_analysis_with_options_detailed(
        source,
        tstop,
        max_step,
        &WasmExecutionOptions::default(),
    )
}

/// Run transient analysis under an explicit browser execution policy.
fn validate_transient_request(
    tstop: f64,
    max_step: f64,
    resource_limits: ResourceLimits,
) -> DetailedWasmResult<()> {
    if !tstop.is_finite() || tstop <= 0.0 {
        return Err(Box::new(WasmError::invalid_argument(format!(
            "Transient stop time must be positive and finite, got {tstop}"
        ))));
    }
    if !max_step.is_finite() || max_step <= 0.0 {
        return Err(Box::new(WasmError::invalid_argument(format!(
            "Transient maximum step must be positive and finite, got {max_step}"
        ))));
    }
    let estimated_points = (tstop / max_step).ceil() as usize;
    let estimated_points = estimated_points.saturating_add(1);
    if estimated_points > resource_limits.max_analysis_points {
        return Err(resource_limit_error(
            ResourceKind::AnalysisPoints,
            estimated_points,
            resource_limits.max_analysis_points,
        ));
    }
    Ok(())
}

pub fn run_transient_analysis_with_options_detailed(
    source: &str,
    tstop: f64,
    max_step: f64,
    options: &WasmExecutionOptions,
) -> DetailedWasmResult<TransientSnapshot> {
    run_transient_analysis_with_options_and_abort_detailed(
        source, tstop, max_step, options, &NoAbort,
    )
}

/// Run transient analysis under an explicit browser policy and cooperative
/// abort source.
pub fn run_transient_analysis_with_options_and_abort_detailed(
    source: &str,
    tstop: f64,
    max_step: f64,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<TransientSnapshot> {
    ensure_not_aborted(external_abort)?;
    let resource_limits = options.resource_limits.to_core();
    validate_transient_request(tstop, max_step, resource_limits)?;

    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    let result = engine_with_resource_limits(resource_limits)?
        .run_tran_with_abort(&netlist, tstop, max_step, external_abort)
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;

    let snapshot = transient_snapshot_from_result(result).map_err(|message| {
        Box::new(WasmError::new(
            message,
            "invalid_transient_result",
            "result_validation",
        ))
    })?;
    ensure_not_aborted(external_abort)?;
    Ok(snapshot)
}

/// Run transient analysis with bounded, multi-channel analog compression.
pub fn run_transient_analysis_compressed_detailed(
    source: &str,
    tstop: f64,
    max_step: f64,
    compression: &WasmCompressionOptions,
) -> DetailedWasmResult<TransientSnapshot> {
    run_transient_analysis_compressed_with_options_detailed(
        source,
        tstop,
        max_step,
        compression,
        &WasmExecutionOptions::default(),
    )
}

/// Run a compressed transient under explicit compression and browser resource
/// policies. The solver and authored output projection are identical to the
/// full-grid path; only the published analog history is decimated.
pub fn run_transient_analysis_compressed_with_options_detailed(
    source: &str,
    tstop: f64,
    max_step: f64,
    compression: &WasmCompressionOptions,
    options: &WasmExecutionOptions,
) -> DetailedWasmResult<TransientSnapshot> {
    run_transient_analysis_compressed_with_options_and_abort_detailed(
        source,
        tstop,
        max_step,
        compression,
        options,
        &NoAbort,
    )
}

/// Run compressed transient analysis under explicit browser policies and a
/// cooperative abort source. Both the solver and compression pass observe the
/// same signal through the core abort-aware entrypoint.
pub fn run_transient_analysis_compressed_with_options_and_abort_detailed(
    source: &str,
    tstop: f64,
    max_step: f64,
    compression: &WasmCompressionOptions,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<TransientSnapshot> {
    ensure_not_aborted(external_abort)?;
    let resource_limits = options.resource_limits.to_core();
    validate_transient_request(tstop, max_step, resource_limits)?;
    let compression = compression.to_core()?;
    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    let result = engine_with_resource_limits(resource_limits)?
        .run_tran_compressed_with_abort(&netlist, tstop, max_step, compression, external_abort)
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
    let snapshot = transient_snapshot_from_compressed_result(result).map_err(|message| {
        Box::new(WasmError::new(
            message,
            "invalid_transient_result",
            "result_validation",
        ))
    })?;
    ensure_not_aborted(external_abort)?;
    Ok(snapshot)
}

/// Backward-compatible string-error compressed transient API.
pub fn run_transient_analysis_compressed(
    source: &str,
    tstop: f64,
    max_step: f64,
    compression: &WasmCompressionOptions,
) -> WasmResult<TransientSnapshot> {
    run_transient_analysis_compressed_detailed(source, tstop, max_step, compression)
        .map_err(|error| error.message)
}

/// Backward-compatible string-error transient API.
pub fn run_transient_analysis(
    source: &str,
    tstop: f64,
    max_step: f64,
) -> WasmResult<TransientSnapshot> {
    run_transient_analysis_detailed(source, tstop, max_step).map_err(|error| error.message)
}

/// Run transient into the common result document. Projected-out solution
/// channels remain present with `None` samples, while device OP/store traces
/// retain explicit owners and unknown units.
#[allow(clippy::too_many_arguments)]
pub fn run_transient_document_with_options_and_abort_detailed(
    source: &str,
    tstop: f64,
    max_step: f64,
    ordinal: usize,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<AnalogResultDocument> {
    validate_analysis_ordinal(ordinal)?;
    let snapshot = run_transient_analysis_with_options_and_abort_detailed(
        source,
        tstop,
        max_step,
        options,
        external_abort,
    )?;
    result_document::transient_document(snapshot, ordinal).map_err(|message| {
        Box::new(WasmError::new(
            message,
            "invalid_result_document",
            "result_validation",
        ))
    })
}

pub fn run_transient_document_detailed(
    source: &str,
    tstop: f64,
    max_step: f64,
) -> DetailedWasmResult<AnalogResultDocument> {
    run_transient_document_with_options_and_abort_detailed(
        source,
        tstop,
        max_step,
        1,
        &WasmExecutionOptions::default(),
        &NoAbort,
    )
}

/// Run scalar-deck input-referred noise into the common typed result document.
/// It preserves complex small-signal voltages/currents, total densities, gain,
/// and sparse per-device contribution identities with explicit validity.
#[allow(clippy::too_many_arguments)]
pub fn run_noise_document_with_options_and_abort_detailed(
    source: &str,
    output_node: &str,
    reference_node: Option<&str>,
    input_source: &str,
    frequencies: &[f64],
    ordinal: usize,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<AnalogResultDocument> {
    validate_analysis_ordinal(ordinal)?;
    ensure_not_aborted(external_abort)?;
    if output_node.trim().is_empty() || input_source.trim().is_empty() {
        return Err(Box::new(WasmError::invalid_argument(
            "noise output node and input source must not be empty".to_owned(),
        )));
    }
    if frequencies.is_empty() {
        return Err(Box::new(WasmError::invalid_argument(
            "noise analysis requires at least one frequency".to_owned(),
        )));
    }
    let resource_limits = options.resource_limits.to_core();
    if frequencies.len() > resource_limits.max_analysis_points {
        return Err(resource_limit_error(
            ResourceKind::AnalysisPoints,
            frequencies.len(),
            resource_limits.max_analysis_points,
        ));
    }
    if let Some((index, frequency)) = frequencies
        .iter()
        .copied()
        .enumerate()
        .find(|(_, frequency)| !frequency.is_finite() || *frequency <= 0.0)
    {
        return Err(Box::new(WasmError::invalid_argument(format!(
            "noise frequency at index {index} must be finite and positive, got {frequency}"
        ))));
    }
    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    let engine = engine_with_resource_limits(resource_limits)?;
    let points = engine
        .run_noise_named_with_input_source_and_abort(
            &netlist,
            output_node,
            reference_node,
            input_source,
            frequencies,
            engine.config().temperature,
            external_abort,
        )
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
    ensure_not_aborted(external_abort)?;
    result_document::noise_document(points, ordinal).map_err(|message| {
        Box::new(WasmError::new(
            message,
            "invalid_result_document",
            "result_validation",
        ))
    })
}

pub fn run_noise_document_detailed(
    source: &str,
    output_node: &str,
    reference_node: Option<&str>,
    input_source: &str,
    frequencies: &[f64],
) -> DetailedWasmResult<AnalogResultDocument> {
    run_noise_document_with_options_and_abort_detailed(
        source,
        output_node,
        reference_node,
        input_source,
        frequencies,
        1,
        &WasmExecutionOptions::default(),
        &NoAbort,
    )
}

/// Run one scalar Tian loop-stability analysis into its lossless retained
/// result document. The direct request deliberately does not consume authored
/// STEP/TEMP axes.
#[allow(clippy::too_many_arguments)]
pub fn run_stb_document_with_options_and_abort_detailed(
    source: &str,
    probe: &str,
    sweep: WasmStbSweep,
    points: usize,
    start_frequency: f64,
    stop_frequency: f64,
    compute_nyquist: bool,
    ordinal: usize,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<StbResultDocument> {
    validate_analysis_ordinal(ordinal)?;
    ensure_not_aborted(external_abort)?;
    if probe.trim().is_empty() {
        return Err(Box::new(WasmError::invalid_argument(
            "STB probe name must not be empty".to_owned(),
        )));
    }
    let config = StbConfig::new()
        .with_sweep(start_frequency, stop_frequency, points)
        .with_sweep_type(sweep.to_core())
        .with_probe(probe)
        .with_nyquist(compute_nyquist);
    config.validate().map_err(|message| {
        Box::new(WasmError::invalid_argument(format!(
            "invalid STB request: {message}"
        )))
    })?;

    let resource_limits = options.resource_limits.to_core();
    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    let result = engine_with_resource_limits(resource_limits)?
        .run_stb_with_abort(&netlist, config, external_abort)
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
    ensure_not_aborted(external_abort)?;
    match stb_result_document::stb_document_with_abort(result, ordinal, external_abort) {
        Ok(document) => Ok(document),
        Err(stb_result_document::StbDocumentError::Aborted) => Err(Box::new(
            WasmError::from_simulation_error(rspice_core::engine::SimulationError::Aborted),
        )),
        Err(stb_result_document::StbDocumentError::Invalid(message)) => Err(Box::new(
            WasmError::new(message, "invalid_result_document", "result_validation"),
        )),
        Err(stb_result_document::StbDocumentError::Allocation(message)) => Err(Box::new(
            WasmError::new(message, "result_allocation_failed", "result_projection"),
        )),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn run_stb_document_detailed(
    source: &str,
    probe: &str,
    sweep: WasmStbSweep,
    points: usize,
    start_frequency: f64,
    stop_frequency: f64,
    compute_nyquist: bool,
) -> DetailedWasmResult<StbResultDocument> {
    run_stb_document_with_options_and_abort_detailed(
        source,
        probe,
        sweep,
        points,
        start_frequency,
        stop_frequency,
        compute_nyquist,
        1,
        &WasmExecutionOptions::default(),
        &NoAbort,
    )
}

/// Execute every supported physical analysis in an authored analog deck over
/// its canonical STEP/TEMP coordinate product. No request selection or
/// implicit scalar fallback is applied: an unsupported card rejects the whole
/// request before a result handle is published.
pub fn run_authored_deck_document_with_options_and_abort_detailed(
    source: &str,
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<DeckResultDocument> {
    use rspice_core::execution::{AnalysisKind, DeckPlan};
    ensure_not_aborted(external_abort)?;
    let resource_limits = options.resource_limits.to_core();
    let netlist = parse_netlist_detailed(source, resource_limits, external_abort)?;
    preflight_authored_deck(&netlist)?;
    let plan = DeckPlan::from_netlist_with_abort(&netlist, &resource_limits, external_abort)
        .map_err(deck_plan_wasm_error)?;
    for axis in plan.axes() {
        match axis.kind() {
            rspice_core::execution::AxisKind::Data
            | rspice_core::execution::AxisKind::Step
            | rspice_core::execution::AxisKind::Temperature => {}
            rspice_core::execution::AxisKind::Alter => {
                return Err(unsupported_deck_axis(
                    "ALTER axes must be expanded before browser deck execution".to_owned(),
                ));
            }
            _ => {
                return Err(unsupported_deck_axis(
                    "unknown canonical run-axis kind is not supported".to_owned(),
                ));
            }
        }
    }
    for planned in plan.analyses() {
        if !matches!(
            planned.id().kind(),
            AnalysisKind::ImplicitOp
                | AnalysisKind::Op
                | AnalysisKind::Dc
                | AnalysisKind::Ac
                | AnalysisKind::Tran
                | AnalysisKind::Noise
        ) {
            return Err(unsupported_deck_analysis(format!(
                "canonical analysis {} is not mapped by the browser deck API",
                planned.id()
            )));
        }
    }

    let planning_engine = engine_with_resource_limits(resource_limits)?;
    let materializer = planning_engine
        .prepare_deck_plan_materializer_with_abort(&netlist, &plan, external_abort)
        .map_err(materialized_run_wasm_error)?;
    let mut document = DeckResultDocument::new(&plan).map_err(deck_document_error)?;
    document
        .coordinates
        .try_reserve_exact(materializer.len())
        .map_err(|_| deck_allocation_error("deck coordinates"))?;
    let result_capacity = materializer
        .len()
        .checked_mul(plan.analyses().len())
        .ok_or_else(|| deck_allocation_error("coordinate-local analysis results"))?;
    document
        .results
        .try_reserve_exact(result_capacity)
        .map_err(|_| deck_allocation_error("coordinate-local analysis results"))?;
    let mut retained_values = 0usize;

    for run_index in 0..materializer.len() {
        ensure_not_aborted(external_abort)?;
        let materialized = materializer
            .materialize_run_with_abort(run_index, external_abort)
            .map_err(materialized_run_wasm_error)?;
        let (coordinate, coordinate_netlist, _topology, analyses) = materialized.into_parts();
        let coordinate_index = document
            .push_coordinate(&coordinate)
            .map_err(deck_document_error)?;
        if analyses.len() != plan.analyses().len() {
            return Err(deck_document_error(format!(
                "coordinate {coordinate_index} materialized {} analyses; expected {}",
                analyses.len(),
                plan.analyses().len()
            )));
        }
        let coordinate_config = rspice_core::resolve_simulation_config(
            planning_engine.config(),
            Some(&coordinate_netlist.options),
            &rspice_core::SimulationConfigOverrides::default(),
        );
        let coordinate_engine = Engine::try_new(coordinate_config).map_err(|error| {
            Box::new(WasmError::from_simulation_error(
                rspice_core::engine::SimulationError::Configuration(error),
            ))
        })?;

        for analysis in analyses {
            ensure_not_aborted(external_abort)?;
            let analysis_id = analysis.id();
            let ordinal = analysis_id.ordinal() as usize + 1;
            let (mut analog, fft_results) = match analysis.command() {
                None if analysis_id.kind() == AnalysisKind::ImplicitOp => {
                    execute_authored_operating_point(
                        &coordinate_engine,
                        &coordinate_netlist,
                        ordinal,
                        external_abort,
                    )?
                }
                None => {
                    return Err(deck_document_error(format!(
                        "canonical materializer omitted the authored command for {analysis_id}"
                    )));
                }
                Some(command) => execute_authored_analysis(
                    &coordinate_engine,
                    &coordinate_netlist,
                    command,
                    ordinal,
                    resource_limits,
                    external_abort,
                )?,
            };
            deck_result_document::set_execution_identity(&mut analog, &coordinate, analysis_id);
            let analog_values = analog.retained_numeric_value_count();
            let fft_values = fft_results.iter().try_fold(0usize, |total, fft| {
                total
                    .checked_add(
                        deck_result_document::fft_retained_numeric_value_count(fft)
                            .map_err(deck_document_error)?,
                    )
                    .ok_or_else(|| {
                        deck_document_error("deck retained-value count overflowed usize".to_owned())
                    })
            })?;
            let next_retained_values = retained_values
                .checked_add(analog_values)
                .and_then(|value| value.checked_add(fft_values))
                .ok_or_else(|| {
                    deck_document_error("deck retained-value count overflowed usize".to_owned())
                })?;
            if next_retained_values > resource_limits.max_result_values {
                return Err(resource_limit_error(
                    ResourceKind::ResultValues,
                    next_retained_values,
                    resource_limits.max_result_values,
                ));
            }
            retained_values = next_retained_values;
            let output_namespace = analysis.output_namespace().components().join("/");
            let checkpoint_namespace = analysis.checkpoint_namespace().components().join("/");
            let parent_result_index = document.results.len();
            document
                .results
                .push(deck_result_document::DeckAnalogResult {
                    coordinate_index,
                    analysis_instance_id: analysis_id.tag(),
                    output_namespace,
                    checkpoint_namespace,
                    document: analog,
                });
            if !fft_results.is_empty() {
                document
                    .fft_results
                    .try_reserve(fft_results.len())
                    .map_err(|_| deck_allocation_error("attached FFT results"))?;
            }
            for mut fft in fft_results {
                fft.parent_analysis_id = analysis_id.tag();
                let output_namespace = format!(
                    "{}/{}/{}",
                    coordinate.stable_tag(),
                    analysis_id,
                    fft.analysis_id
                );
                document
                    .fft_results
                    .push(deck_result_document::DeckFftResult {
                        coordinate_index,
                        parent_result_index,
                        output_namespace,
                        snapshot: fft,
                    });
            }
        }
    }
    ensure_not_aborted(external_abort)?;
    document
        .validate_with_abort(external_abort)
        .map_err(|message| {
            if external_abort.is_aborted() {
                aborted_error()
            } else {
                deck_document_error(message)
            }
        })?;
    Ok(document)
}

pub fn run_authored_deck_document_detailed(source: &str) -> DetailedWasmResult<DeckResultDocument> {
    run_authored_deck_document_with_options_and_abort_detailed(
        source,
        &WasmExecutionOptions::default(),
        &NoAbort,
    )
}

fn preflight_authored_deck(netlist: &Netlist) -> DetailedWasmResult<()> {
    use rspice_core::netlist::AnalysisCommand;

    if netlist.source_text.as_deref().is_some_and(|source| {
        source.lines().any(|line| {
            line.split_whitespace()
                .next()
                .is_some_and(|token| token.eq_ignore_ascii_case(".alter"))
        })
    }) {
        return Err(unsupported_deck_axis(
            "ALTER variants must be expanded before browser deck execution".to_owned(),
        ));
    }
    for command in &netlist.analyses {
        match command {
            AnalysisCommand::Op
            | AnalysisCommand::Dc { sweep2: None, .. }
            | AnalysisCommand::Ac { .. }
            | AnalysisCommand::Tran { start: None, .. }
            | AnalysisCommand::Tran {
                start: Some(0.0), ..
            }
            | AnalysisCommand::Noise { .. }
            | AnalysisCommand::Step(_)
            | AnalysisCommand::Temp { .. } => {}
            AnalysisCommand::Dc {
                sweep2: Some(_), ..
            } => {
                return Err(unsupported_deck_analysis(
                    "nested two-source DC sweeps are not represented by analog result schema v1"
                        .to_owned(),
                ));
            }
            AnalysisCommand::Tran { start: Some(_), .. } => {
                return Err(unsupported_deck_analysis(
                    "nonzero transient output start times are not represented by this browser deck executor"
                        .to_owned(),
                ));
            }
            AnalysisCommand::Four { .. } => {
                return Err(unsupported_deck_analysis(
                    "authored FOUR post-processing is not represented by the browser deck API"
                        .to_owned(),
                ));
            }
            other => {
                return Err(unsupported_deck_analysis(format!(
                    "authored analysis {other:?} is not mapped by the browser deck API"
                )));
            }
        }
    }
    Ok(())
}

fn execute_authored_operating_point(
    engine: &Engine,
    netlist: &Netlist,
    ordinal: usize,
    abort: &dyn AbortSignal,
) -> DetailedWasmResult<(AnalogResultDocument, Vec<TransientFftSnapshot>)> {
    let (result, report) = engine
        .run_dc_op_with_report_and_abort(netlist, abort)
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
    ensure_not_aborted(abort)?;
    let document = result_document::operating_point_document(result, report, ordinal)
        .map_err(deck_document_error)?;
    Ok((document, Vec::new()))
}

#[allow(clippy::too_many_arguments)]
fn execute_authored_analysis(
    engine: &Engine,
    netlist: &Netlist,
    command: &rspice_core::netlist::AnalysisCommand,
    ordinal: usize,
    resource_limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> DetailedWasmResult<(AnalogResultDocument, Vec<TransientFftSnapshot>)> {
    use rspice_core::netlist::{AnalysisCommand, DcSweepSpec};

    match command {
        AnalysisCommand::Op => execute_authored_operating_point(engine, netlist, ordinal, abort),
        AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            mode,
            sweep2: None,
        } => {
            let spec = DcSweepSpec {
                start: *start,
                stop: *stop,
                step: *step,
                mode: mode.clone(),
            };
            let points = engine
                .run_dc_sweep2_spec_with_report_and_abort(netlist, source, &spec, None, abort)
                .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
            ensure_not_aborted(abort)?;
            let document = result_document::dc_sweep_document(source, points, ordinal)
                .map_err(deck_document_error)?;
            Ok((document, Vec::new()))
        }
        AnalysisCommand::Ac {
            variation,
            points,
            start_freq,
            stop_freq,
        } => {
            let frequencies = authored_frequency_grid(
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                false,
                resource_limits,
                abort,
            )?;
            let results = engine
                .run_ac_with_abort(netlist, &frequencies, abort)
                .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
            let mut snapshots = Vec::new();
            snapshots
                .try_reserve_exact(results.len())
                .map_err(|_| deck_allocation_error("coordinate-local AC snapshots"))?;
            for (index, point) in results.into_iter().enumerate() {
                if index.is_multiple_of(64) {
                    ensure_not_aborted(abort)?;
                }
                snapshots.push(AcPointSnapshot {
                    frequency: point.frequency,
                    node_names: point.node_names,
                    branch_names: point.branch_names,
                    voltages: complex_series_from_slice(&point.voltages),
                    currents: complex_series_from_slice(&point.currents),
                });
            }
            ensure_not_aborted(abort)?;
            let document =
                result_document::ac_document(snapshots, ordinal).map_err(deck_document_error)?;
            Ok((document, Vec::new()))
        }
        AnalysisCommand::Tran {
            step,
            stop,
            start,
            max_step,
            uic,
        } if start.is_none_or(|start| start == 0.0) => {
            let ceiling = resolved_authored_tran_step(*step, *stop, *start, *max_step)?;
            validate_transient_request(*stop, ceiling, resource_limits)?;
            let result = engine
                .run_tran_with_startup_mode_and_abort(
                    netlist,
                    *stop,
                    ceiling,
                    rspice_core::engine::TransientStartupMode::from_uic(*uic),
                    abort,
                )
                .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
            let mut snapshot = transient_snapshot_from_result(result).map_err(|message| {
                Box::new(WasmError::new(
                    message,
                    "invalid_transient_result",
                    "result_validation",
                ))
            })?;
            ensure_not_aborted(abort)?;
            let fft_results = std::mem::take(&mut snapshot.fft_results);
            let document = result_document::transient_document(snapshot, ordinal)
                .map_err(deck_document_error)?;
            Ok((document, fft_results))
        }
        AnalysisCommand::Noise {
            output_node,
            reference_node,
            input_source,
            variation,
            points,
            start_freq,
            stop_freq,
        } => {
            let frequencies = authored_frequency_grid(
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                true,
                resource_limits,
                abort,
            )?;
            let points = engine
                .run_noise_named_with_input_source_and_abort(
                    netlist,
                    output_node,
                    reference_node.as_deref(),
                    input_source,
                    &frequencies,
                    engine.config().temperature,
                    abort,
                )
                .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
            ensure_not_aborted(abort)?;
            let document =
                result_document::noise_document(points, ordinal).map_err(deck_document_error)?;
            Ok((document, Vec::new()))
        }
        _ => Err(unsupported_deck_analysis(format!(
            "materialized analysis {command:?} is not mapped by the browser deck API"
        ))),
    }
}

fn authored_frequency_grid(
    variation: rspice_core::netlist::FreqVariation,
    points: usize,
    start: f64,
    stop: f64,
    strictly_positive: bool,
    resource_limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> DetailedWasmResult<Vec<f64>> {
    ensure_not_aborted(abort)?;
    if strictly_positive && (!start.is_finite() || start <= 0.0) {
        return Err(Box::new(WasmError::invalid_argument(format!(
            "authored noise start frequency must be positive and finite, got {start}"
        ))));
    }
    rspice_core::analysis::ac::try_ac_sweep_frequencies_bounded_with_abort(
        variation,
        points,
        start,
        stop,
        resource_limits.max_analysis_points,
        abort,
    )
    .map_err(|error| match error {
        rspice_core::analysis::FrequencyGridError::Aborted => aborted_error(),
        rspice_core::analysis::FrequencyGridError::LimitExceeded { requested, limit } => {
            resource_limit_error(ResourceKind::AnalysisPoints, requested, limit)
        }
        rspice_core::analysis::FrequencyGridError::Allocation { requested } => {
            Box::new(WasmError::new(
                format!("could not allocate the {requested}-point authored frequency grid"),
                "result_allocation_failed",
                "analysis_setup",
            ))
        }
        other => Box::new(WasmError::invalid_argument(format!(
            "invalid authored frequency sweep {variation:?} {points} from {start} to {stop} Hz: {other}"
        ))),
    })
}

fn resolved_authored_tran_step(
    step: f64,
    stop: f64,
    start: Option<f64>,
    explicit: Option<f64>,
) -> DetailedWasmResult<f64> {
    rspice_core::execution::resolve_transient_maximum_step(step, stop, start, explicit)
        .map_err(|error| Box::new(WasmError::invalid_argument(error.to_string())))
}

fn unsupported_deck_analysis(message: String) -> Box<WasmError> {
    Box::new(WasmError::new(
        message,
        "unsupported_deck_analysis",
        "unsupported_feature",
    ))
}

fn unsupported_deck_axis(message: String) -> Box<WasmError> {
    Box::new(WasmError::new(
        message,
        "unsupported_deck_axis",
        "unsupported_feature",
    ))
}

fn deck_document_error(message: String) -> Box<WasmError> {
    Box::new(WasmError::new(
        message,
        "invalid_deck_result_document",
        "result_validation",
    ))
}

fn deck_allocation_error(object: &'static str) -> Box<WasmError> {
    Box::new(WasmError::new(
        format!("could not allocate {object}"),
        "result_allocation_failed",
        "result_projection",
    ))
}

fn deck_plan_wasm_error(error: rspice_core::execution::DeckPlanError) -> Box<WasmError> {
    match error {
        rspice_core::execution::DeckPlanError::Aborted => aborted_error(),
        rspice_core::execution::DeckPlanError::ResourceLimit(error) => {
            Box::new(WasmError::resource_limit(error.to_string(), error))
        }
        other => Box::new(WasmError::new(
            other.to_string(),
            "invalid_deck_plan",
            "input_validation",
        )),
    }
}

fn materialized_run_wasm_error(
    error: rspice_core::execution::MaterializedRunError,
) -> Box<WasmError> {
    match error {
        rspice_core::execution::MaterializedRunError::Aborted => aborted_error(),
        rspice_core::execution::MaterializedRunError::DeckPlan(error) => {
            deck_plan_wasm_error(error)
        }
        rspice_core::execution::MaterializedRunError::Simulation(error) => {
            Box::new(WasmError::from_simulation_error(error))
        }
        other => Box::new(WasmError::new(
            other.to_string(),
            "deck_materialization_failed",
            "execution",
        )),
    }
}

/// Exercise the configured browser parser-to-solver path without I/O.
pub fn health_check_with_options_detailed(
    options: &WasmExecutionOptions,
) -> DetailedWasmResult<WasmHealthReport> {
    health_check_with_options_and_abort_detailed(options, &NoAbort)
}

/// Execute the readiness probe with the same deadline and cancellation
/// contract as analysis calls.
pub fn health_check_with_options_and_abort_detailed(
    options: &WasmExecutionOptions,
    external_abort: &dyn AbortSignal,
) -> DetailedWasmResult<WasmHealthReport> {
    let report = engine_with_resource_limits(options.resource_limits.to_core())?
        .health_check_with_abort(external_abort)
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
    Ok(WasmHealthReport {
        status: "ready".to_string(),
        ready: true,
        duration_seconds: report.elapsed.as_secs_f64(),
        element_count: report.element_count,
        node_count: report.node_count,
        branch_count: report.branch_count,
        output_voltage: report.output_voltage,
    })
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_directive_names_cover_the_browser_diagnostic_contract() {
        use rspice_core::netlist::OutputDirectiveKind;

        for (kind, expected) in [
            (OutputDirectiveKind::Save, "save"),
            (OutputDirectiveKind::Probe, "probe"),
            (OutputDirectiveKind::Print, "print"),
            (OutputDirectiveKind::Plot, "plot"),
            (OutputDirectiveKind::Measure, "measure"),
            (OutputDirectiveKind::Four, "four"),
            (OutputDirectiveKind::Fft, "fft"),
        ] {
            assert_eq!(output_directive_name(kind), expected);
        }
    }

    #[test]
    fn summary_includes_nonfatal_parser_diagnostics() {
        let summary = summarize_netlist(
            "diagnostic deck\n\
             V1 in 0 1\n\
             R1 in 0 1k\n\
             .options vendorcompat=1\n\
             .end\n",
        )
        .expect("deck parses with warning");

        assert_eq!(summary.diagnostics.len(), 1);
        assert_eq!(summary.diagnostics[0].line, 4);
        assert!(
            summary.diagnostics[0]
                .message
                .to_ascii_lowercase()
                .contains("vendorcompat")
        );
    }

    #[test]
    fn summary_exposes_structured_startup_diagnostics_additively() {
        let summary =
            summarize_netlist_detailed("startup diagnostic\nV1 in 0 1\n.IC V(MISSING)=1\n.END\n")
                .expect("an unknown startup node is a non-fatal semantic warning");

        assert!(
            summary
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "startup-undefined-node")
        );
        assert_eq!(summary.startup_diagnostics.len(), 1);
        let diagnostic = &summary.startup_diagnostics[0];
        assert_eq!(diagnostic.code, "startup-undefined-node");
        assert_eq!(diagnostic.stage, "startup_topology");
        assert_eq!(diagnostic.directive, "ic");
        assert_eq!(diagnostic.canonical_nodes, ["MISSING"]);
        assert_eq!(diagnostic.origins[0].line, 3);
        assert_eq!(diagnostic.scopes[0].kind, "top_level");
    }

    #[test]
    fn startup_conflict_error_preserves_both_modes_and_origins() {
        let error = WasmError::from_parse_error(
            rspice_core::netlist::ParseError::StartupDirectiveConflict(Box::new(
                rspice_core::netlist::StartupDirectiveConflictError {
                    first_kind: rspice_core::netlist::StartupDirectiveKind::Ic,
                    first: rspice_core::netlist::NetlistSourceLocation::in_file("deck.cir", 3),
                    conflicting_kind: rspice_core::netlist::StartupDirectiveKind::NodeSet,
                    conflicting: rspice_core::netlist::NetlistSourceLocation::in_file(
                        "included.cir",
                        4,
                    ),
                },
            )),
        );

        assert_eq!(error.kind, "conflicting_startup_directives");
        assert_eq!(error.category, "startup_directive_validation");
        assert_eq!(error.primary_source.as_deref(), Some("deck.cir"));
        assert_eq!(error.primary_line, Some(3));
        assert_eq!(error.related_source.as_deref(), Some("included.cir"));
        assert_eq!(error.related_line, Some(4));
        assert_eq!(error.first_startup_kind.as_deref(), Some("ic"));
        assert_eq!(error.conflicting_startup_kind.as_deref(), Some("nodeset"));
        assert!(error.unresolved_output_symbols.is_empty());
    }

    #[test]
    fn unresolved_subcircuit_parameter_error_preserves_typed_hierarchy_identity() {
        let error = WasmError::from_parse_error(
            rspice_core::netlist::ParseError::UnresolvedSubcircuitParameter(Box::new(
                rspice_core::netlist::UnresolvedSubcircuitParameterError {
                    subcircuit_name: "cell".into(),
                    canonical_subcircuit_name: "CELL".into(),
                    instance_name: "x1".into(),
                    canonical_instance_name: "X1".into(),
                    qualified_instance_name: "TOP.X1".into(),
                    parameter_name: "foo".into(),
                    canonical_parameter_name: "FOO".into(),
                    expression: "TIME + meh".into(),
                    missing_dependency: Some("MEH".into()),
                    reason: "Undefined parameter: MEH".into(),
                },
            )),
        );

        assert_eq!(error.kind, "unresolved_subcircuit_parameter");
        assert_eq!(error.category, "subcircuit_parameter_resolution");
        assert_eq!(error.subcircuit_name.as_deref(), Some("cell"));
        assert_eq!(error.qualified_instance_name.as_deref(), Some("TOP.X1"));
        assert_eq!(error.parameter_name.as_deref(), Some("foo"));
        assert_eq!(error.canonical_parameter_name.as_deref(), Some("FOO"));
        assert_eq!(error.expression.as_deref(), Some("TIME + meh"));
        assert_eq!(error.missing_dependency.as_deref(), Some("MEH"));
        assert_eq!(error.reason.as_deref(), Some("Undefined parameter: MEH"));
    }

    #[test]
    fn undefined_subcircuit_error_preserves_typed_hierarchy_identity() {
        let error =
            WasmError::from_parse_error(rspice_core::netlist::ParseError::UndefinedSubcircuit(
                Box::new(rspice_core::netlist::UndefinedSubcircuitError {
                    subcircuit_name: "missing".into(),
                    canonical_subcircuit_name: "MISSING".into(),
                    instance_name: "x1".into(),
                    canonical_instance_name: "X1".into(),
                    qualified_instance_name: "TOP.X1".into(),
                }),
            ));

        assert_eq!(error.kind, "undefined_subcircuit");
        assert_eq!(error.category, "subcircuit_resolution");
        assert_eq!(error.subcircuit_name.as_deref(), Some("missing"));
        assert_eq!(error.canonical_subcircuit_name.as_deref(), Some("MISSING"));
        assert_eq!(error.instance_name.as_deref(), Some("x1"));
        assert_eq!(error.canonical_instance_name.as_deref(), Some("X1"));
        assert_eq!(error.qualified_instance_name.as_deref(), Some("TOP.X1"));
    }

    #[test]
    fn missing_device_model_error_preserves_typed_device_identity() {
        let error =
            WasmError::from_parse_error(rspice_core::netlist::ParseError::MissingDeviceModel(
                Box::new(rspice_core::netlist::MissingDeviceModelError {
                    line: 4,
                    device_name: "d1".into(),
                    canonical_device_name: "D1".into(),
                    device_type: "DIODE".into(),
                }),
            ));

        assert_eq!(error.kind, "missing_device_model");
        assert_eq!(error.category, "device_model_resolution");
        assert_eq!(error.primary_line, Some(4));
        assert_eq!(error.instance_name.as_deref(), Some("d1"));
        assert_eq!(error.canonical_instance_name.as_deref(), Some("D1"));
        assert_eq!(error.reason.as_deref(), Some("DIODE"));
    }

    #[test]
    fn browser_resource_defaults_are_stricter_than_desktop_defaults() {
        let browser = WasmResourceLimits::default();
        let desktop = ResourceLimits::default();

        assert_eq!(browser.max_netlist_bytes, 8 * MEBIBYTE);
        assert_eq!(browser.max_analysis_points, 200_000);
        assert_eq!(browser.max_result_values, 2_000_000);
        assert_eq!(browser.max_parallel_workers, 1);
        assert!(browser.max_netlist_bytes < desktop.max_netlist_bytes);
        assert!(browser.max_analysis_points < desktop.max_analysis_points);
    }

    #[test]
    fn browser_health_probe_exercises_parser_and_solver() {
        let report = health_check_with_options_detailed(&WasmExecutionOptions::default())
            .expect("browser backend is ready");
        assert_eq!(report.status, "ready");
        assert!(report.ready);
        assert_eq!(report.element_count, 2);
        assert_eq!(report.node_count, 1);
        assert_eq!(report.branch_count, 1);
        assert!((report.output_voltage - 1.0).abs() <= 1.0e-12);
    }

    #[test]
    fn partial_options_inherit_defaults_and_reject_unknown_controls() {
        let options: WasmExecutionOptions = serde_json::from_value(serde_json::json!({
            "resourceLimits": {"maxAnalysisPoints": 17}
        }))
        .expect("partial browser policy deserializes");
        assert_eq!(options.resource_limits.max_analysis_points, 17);
        assert_eq!(
            options.resource_limits.max_netlist_bytes,
            WasmResourceLimits::default().max_netlist_bytes
        );

        assert!(
            serde_json::from_value::<WasmExecutionOptions>(serde_json::json!({
                "resourceLimits": {"maxAnalaysisPoints": 17}
            }))
            .is_err(),
            "misspelled resource controls must fail closed"
        );
    }

    #[test]
    fn parse_and_analysis_limits_publish_typed_resource_details() {
        let mut parse_options = WasmExecutionOptions::default();
        parse_options.resource_limits.max_netlist_bytes = 8;
        let parse_error = summarize_netlist_with_options_detailed(
            "bounded browser deck\nV1 1 0 1\n.END\n",
            &parse_options,
        )
        .expect_err("source must exceed the explicit browser byte ceiling");
        assert_eq!(parse_error.kind, "resource_limit");
        assert_eq!(parse_error.code, "resource_limit");
        assert_eq!(parse_error.category, "resource_limit");
        assert!(!parse_error.retryable);
        assert_eq!(parse_error.resource.as_deref(), Some("netlist_bytes"));
        assert_eq!(parse_error.limit, Some(8));

        let mut analysis_options = WasmExecutionOptions::default();
        analysis_options.resource_limits.max_analysis_points = 2;
        let analysis_error = run_ac_analysis_with_options_detailed(
            "valid\nV1 1 0 1\nR1 1 0 1k\n.END\n",
            &[1.0, 10.0, 100.0],
            &analysis_options,
        )
        .expect_err("frequency vector must exceed the explicit point ceiling");
        assert_eq!(analysis_error.kind, "resource_limit");
        assert_eq!(analysis_error.resource.as_deref(), Some("analysis_points"));
        assert_eq!(analysis_error.requested, Some(3));
        assert_eq!(analysis_error.limit, Some(2));
    }

    #[test]
    fn simulation_errors_share_core_codes_and_retry_policy() {
        let cancelled =
            WasmError::from_simulation_error(rspice_core::engine::SimulationError::Aborted);
        assert_eq!(cancelled.kind, "aborted");
        assert_eq!(cancelled.code, "aborted");
        assert_eq!(cancelled.category, "cancellation");
        assert!(cancelled.retryable);

        let convergence = WasmError::from_simulation_error(
            rspice_core::engine::SimulationError::ConvergenceFailed(37),
        );
        assert_eq!(convergence.code, "convergence_error");
        assert_eq!(convergence.iterations, Some(37));
        assert!(!convergence.retryable);

        let behavioral = WasmError::from_simulation_error(
            rspice_core::engine::SimulationError::BehavioralReference(Box::new(
                rspice_core::device::BehavioralReferenceError {
                    owner_name: "b2".to_string(),
                    canonical_owner_name: "B2".to_string(),
                    dependency_name: "b1".to_string(),
                    canonical_dependency_name: "B1".to_string(),
                    reason: rspice_core::device::BehavioralReferenceReason::LeadCurrentNotSolutionVariable,
                },
            )),
        );
        assert_eq!(behavioral.code, "behavioral_reference_error");
        assert_eq!(behavioral.instance_name.as_deref(), Some("b2"));
        assert_eq!(behavioral.canonical_instance_name.as_deref(), Some("B2"));
        assert_eq!(behavioral.missing_dependency.as_deref(), Some("B1"));
        assert_eq!(
            behavioral.reason.as_deref(),
            Some("lead_current_not_solution_variable")
        );
    }

    #[test]
    fn ac_input_validation_rejects_non_finite_and_negative_frequencies() {
        let source = "valid\nV1 1 0 1\nR1 1 0 1k\n.END\n";
        for frequencies in [[1.0, f64::NAN], [1.0, -1.0]] {
            let error = run_ac_analysis_detailed(source, &frequencies)
                .expect_err("invalid explicit frequency must fail at the boundary");
            assert_eq!(error.kind, "invalid_argument");
            assert_eq!(error.category, "input_validation");
        }
    }

    const CANCELLATION_DECK: &str = "browser cancellation\n\
        V1 out 0 PULSE(0 1 0 1n 1n 1u 2u)\n\
        R1 out 0 1k\n\
        .end\n";

    fn assert_cancelled(error: Box<WasmError>) {
        assert_eq!(error.code, "aborted");
        assert_eq!(error.kind, "aborted");
        assert_eq!(error.category, "cancellation");
        assert!(error.retryable);
    }

    #[test]
    fn every_browser_analysis_path_propagates_the_explicit_abort_source() {
        let options = WasmExecutionOptions::default();
        let abort = rspice_core::abort_signal::ImmediateAbort;

        assert_cancelled(
            run_dc_operating_point_with_options_and_abort_detailed(
                CANCELLATION_DECK,
                &options,
                &abort,
            )
            .expect_err("OP must observe the frontend abort source"),
        );
        assert_cancelled(
            run_ac_analysis_with_options_and_abort_detailed(
                CANCELLATION_DECK,
                &[1.0, 10.0],
                &options,
                &abort,
            )
            .expect_err("AC must observe the frontend abort source"),
        );
        assert_cancelled(
            run_transient_analysis_with_options_and_abort_detailed(
                CANCELLATION_DECK,
                10.0e-6,
                1.0e-9,
                &options,
                &abort,
            )
            .expect_err("TRAN must observe the frontend abort source"),
        );
        assert_cancelled(
            run_transient_analysis_compressed_with_options_and_abort_detailed(
                CANCELLATION_DECK,
                10.0e-6,
                1.0e-9,
                &WasmCompressionOptions::default(),
                &options,
                &abort,
            )
            .expect_err("compressed TRAN must observe the frontend abort source"),
        );
        assert_cancelled(
            run_operating_point_document_with_options_and_abort_detailed(
                CANCELLATION_DECK,
                1,
                &options,
                &abort,
            )
            .expect_err("typed OP must observe the frontend abort source"),
        );
        assert_cancelled(
            run_dc_sweep_document_with_options_and_abort_detailed(
                CANCELLATION_DECK,
                "V1",
                0.0,
                1.0,
                0.5,
                1,
                &options,
                &abort,
            )
            .expect_err("typed DC must observe the frontend abort source"),
        );
        assert_cancelled(
            run_ac_document_with_options_and_abort_detailed(
                CANCELLATION_DECK,
                &[1.0, 10.0],
                1,
                &options,
                &abort,
            )
            .expect_err("typed AC must observe the frontend abort source"),
        );
        assert_cancelled(
            run_transient_document_with_options_and_abort_detailed(
                CANCELLATION_DECK,
                10.0e-6,
                1.0e-9,
                1,
                &options,
                &abort,
            )
            .expect_err("typed TRAN must observe the frontend abort source"),
        );
        assert_cancelled(
            run_noise_document_with_options_and_abort_detailed(
                CANCELLATION_DECK,
                "out",
                None,
                "V1",
                &[1.0, 10.0],
                1,
                &options,
                &abort,
            )
            .expect_err("typed noise must observe the frontend abort source"),
        );
        assert_cancelled(
            run_stb_document_with_options_and_abort_detailed(
                CANCELLATION_DECK,
                "V1",
                WasmStbSweep::Linear,
                2,
                1.0,
                10.0,
                true,
                1,
                &options,
                &abort,
            )
            .expect_err("typed STB must observe the frontend abort source"),
        );
        assert_cancelled(
            run_authored_deck_document_with_options_and_abort_detailed(
                "authored cancellation\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n",
                &options,
                &abort,
            )
            .expect_err("authored deck execution must observe the frontend abort source"),
        );
    }

    const AUTHORED_STEP_TRAN_DECK: &str = "authored STEP transient\n\
        .param load=1k\n\
        V1 out 0 PULSE(0 1 0 1n 1n 1u 2u)\n\
        R1 out 0 {load}\n\
        .step param load list 1k 2k\n\
        .tran 200n 1u\n\
        .end\n";

    const AUTHORED_TEMP_AC_DECK: &str = "authored TEMP AC\n\
        V1 out 0 AC 1\n\
        R1 out 0 1k\n\
        .temp 0 27\n\
        .ac lin 2 1 10\n\
        .end\n";

    #[test]
    fn authored_step_and_temp_wrap_only_the_authored_physical_analysis() {
        let step = run_authored_deck_document_detailed(AUTHORED_STEP_TRAN_DECK)
            .expect("STEP/TRAN authored deck executes");
        assert_eq!(step.coordinates.len(), 2);
        assert_eq!(step.planned_analyses.len(), 1);
        assert_eq!(step.planned_analyses[0].analysis_instance_id, "tran-001");
        assert_eq!(step.results.len(), 2);
        assert!(
            step.results
                .iter()
                .all(|result| result.analysis_instance_id == "tran-001")
        );
        assert!(
            step.results
                .iter()
                .all(|result| result.document.analysis.kind == AnalogAnalysisKind::Transient)
        );

        let temperature = run_authored_deck_document_detailed(AUTHORED_TEMP_AC_DECK)
            .expect("TEMP/AC authored deck executes");
        assert_eq!(temperature.coordinates.len(), 2);
        assert_eq!(temperature.results.len(), 2);
        assert!(
            temperature
                .results
                .iter()
                .all(|result| result.analysis_instance_id == "ac-001")
        );
        assert!(
            temperature
                .results
                .iter()
                .all(|result| result.document.analysis.kind == AnalogAnalysisKind::AcSmallSignal)
        );
    }

    #[test]
    fn authored_data_backed_step_preserves_row_bindings_and_coordinates() {
        let deck = "authored DATA STEP\n\
            .param load=1k bias=1\n\
            V1 out 0 {bias}\n\
            R1 out 0 {load}\n\
            .data corners load bias\n\
            1k 1\n\
            2k 2\n\
            .enddata\n\
            .step data=corners\n\
            .op\n\
            .end\n";
        let document = run_authored_deck_document_detailed(deck)
            .expect("DATA-backed STEP executes through the canonical materializer");
        assert_eq!(document.axes.len(), 1);
        assert_eq!(document.axes[0].kind, "data");
        assert_eq!(document.axes[0].data_bindings, ["bias", "load"]);
        assert_eq!(document.coordinates.len(), 2);
        for (index, coordinate) in document.coordinates.iter().enumerate() {
            assert_eq!(coordinate.index, index);
            let DeckAxisValue::DataRow { bindings } = &coordinate.assignments[0].value else {
                panic!("DATA coordinate must retain named row bindings")
            };
            assert_eq!(
                bindings
                    .iter()
                    .map(|binding| binding.name.as_str())
                    .collect::<Vec<_>>(),
                ["bias", "load"]
            );
        }
    }

    #[test]
    fn authored_repeated_analyses_preserve_order_and_unique_instance_ids() {
        let deck = "authored repeated analyses\n\
            V1 out 0 DC 1 AC 1\n\
            R1 out 0 1k\n\
            .op\n\
            .ac lin 3 1 10\n\
            .ac lin 4 10 100\n\
            .end\n";
        let document =
            run_authored_deck_document_detailed(deck).expect("repeated authored analyses execute");
        assert_eq!(document.coordinates.len(), 1);
        assert_eq!(
            document
                .planned_analyses
                .iter()
                .map(|analysis| analysis.analysis_instance_id.as_str())
                .collect::<Vec<_>>(),
            ["op-001", "ac-001", "ac-002"]
        );
        assert_eq!(
            document
                .results
                .iter()
                .map(|result| result.analysis_instance_id.as_str())
                .collect::<Vec<_>>(),
            ["op-001", "ac-001", "ac-002"]
        );
        assert_eq!(document.results[1].document.point_count, 3);
        assert_eq!(document.results[2].document.point_count, 4);
        assert_ne!(
            document.results[1].output_namespace,
            document.results[2].output_namespace
        );
    }

    #[test]
    fn authored_multi_coordinate_multi_analysis_budget_is_cumulative() {
        let deck = "authored cumulative result budget\n\
            .param load=1k\n\
            V1 out 0 DC 1 AC 1\n\
            R1 out 0 {load}\n\
            .step param load list 1k 2k\n\
            .op\n\
            .ac lin 3 1 10\n\
            .end\n";
        let accepted = run_authored_deck_document_detailed(deck)
            .expect("cumulative-budget fixture executes under defaults");
        assert_eq!(accepted.coordinates.len(), 2);
        assert_eq!(accepted.results.len(), 4);
        let contributions = accepted
            .results
            .iter()
            .map(|result| result.document.retained_numeric_value_count())
            .collect::<Vec<_>>();
        let total = contributions.iter().sum::<usize>();
        assert!(contributions.iter().all(|value| *value < total - 1));

        let mut options = WasmExecutionOptions::default();
        options.resource_limits.max_result_values = total - 1;
        let error =
            run_authored_deck_document_with_options_and_abort_detailed(deck, &options, &NoAbort)
                .expect_err("aggregate result values above the shared ceiling must fail");
        assert_eq!(error.code, "resource_limit");
        assert_eq!(error.resource.as_deref(), Some("result_values"));
        assert_eq!(error.requested, Some(total));
        assert_eq!(error.limit, Some(total - 1));
    }

    #[test]
    fn conditional_coordinate_local_schemas_are_stable_by_coordinate_identity() {
        fn execute(values: &str) -> DeckResultDocument {
            run_authored_deck_document_detailed(&format!(
                "conditional topology\n\
                 .param sel=0\n\
                 V1 in 0 AC 1\n\
                 .step param sel list {values}\n\
                 .if (sel==0)\n\
                 R1 in 0 1k\n\
                 .else\n\
                 R1 in mid 1k\n\
                 R2 mid 0 1k\n\
                 .endif\n\
                 .ac lin 2 1 10\n\
                 .end\n"
            ))
            .expect("conditional authored deck executes")
        }

        let forward = execute("0 1");
        let reverse = execute("1 0");
        let schema = |document: &DeckResultDocument| {
            document
                .results
                .iter()
                .map(|result| {
                    (
                        result.document.coordinate_id.clone().unwrap(),
                        result
                            .document
                            .signals
                            .iter()
                            .map(|signal| signal.canonical_name.clone())
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<std::collections::BTreeMap<_, _>>()
        };
        assert_eq!(schema(&forward), schema(&reverse));
        assert_ne!(
            forward.results[0].document.signals.len(),
            forward.results[1].document.signals.len(),
            "conditional topology must retain coordinate-local schemas"
        );
    }

    #[test]
    fn authored_deck_rejects_unsupported_analysis_shapes() {
        let unsupported = run_authored_deck_document_detailed(
            "unsupported deck\nV1 out 0 1\nR1 out 0 1k\n.tf V(out) V1\n.end\n",
        )
        .expect_err("unmapped TF must fail closed");
        assert_eq!(unsupported.code, "unsupported_deck_analysis");

        let nested_dc = run_authored_deck_document_detailed(
            "nested DC\nV1 out 0 0\nV2 x 0 0\nR1 out 0 1k\n.dc V1 0 1 1 V2 0 1 1\n.end\n",
        )
        .expect_err("nested DC schema must fail closed");
        assert_eq!(nested_dc.code, "unsupported_deck_analysis");

        let alter = run_authored_deck_document_detailed(
            "ALTER deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.alter second\nR1 out 0 2k\n.end\n",
        )
        .expect_err("textual ALTER must fail before materialization");
        assert_eq!(alter.code, "unsupported_deck_axis");
    }

    #[test]
    fn authored_tran_default_and_explicit_max_step_contract_is_exact() {
        assert_eq!(
            resolved_authored_tran_step(1.0, 100.0, None, None).unwrap(),
            1.0
        );
        assert_eq!(
            resolved_authored_tran_step(10.0, 100.0, None, None).unwrap(),
            2.0
        );
        assert_eq!(
            resolved_authored_tran_step(10.0, 100.0, Some(90.0), None).unwrap(),
            0.2
        );
        assert_eq!(
            resolved_authored_tran_step(10.0, 100.0, Some(90.0), Some(0.25)).unwrap(),
            0.25
        );
        for error in [
            resolved_authored_tran_step(0.0, 100.0, None, None),
            resolved_authored_tran_step(1.0, 0.0, None, None),
            resolved_authored_tran_step(1.0, 100.0, Some(-1.0), None),
            resolved_authored_tran_step(1.0, 100.0, None, Some(0.0)),
            resolved_authored_tran_step(1.0, 100.0, None, Some(f64::NAN)),
        ] {
            assert_eq!(error.unwrap_err().code, "invalid_argument");
        }
    }

    const TYPED_DOCUMENT_DECK: &str = "browser typed analog document\n\
        V1 in 0 DC 0 AC 1 PULSE(0 1 0 1n 1n 1u 2u)\n\
        R1 in out 1k\n\
        R2 out 0 1k\n\
        .save V(out) I(V1)\n\
        .end\n";

    const STB_DOCUMENT_DECK: &str = "browser typed STB document\n\
        EAMP out 0 in 0 10\n\
        VPROBE out fb 0\n\
        RF fb in 10k\n\
        RIN in 0 1k\n\
        .end\n";

    #[test]
    fn scalar_stb_document_retains_primary_bode_nyquist_margins_and_units() {
        let document = run_stb_document_detailed(
            STB_DOCUMENT_DECK,
            "VPROBE",
            WasmStbSweep::Linear,
            4,
            10.0,
            1.0e3,
            true,
        )
        .expect("typed STB document executes");

        assert_eq!(document.schema, STB_RESULT_SCHEMA);
        assert_eq!(document.schema_version, STB_RESULT_VERSION);
        assert_eq!(document.analysis.id, "stb-001");
        assert_eq!(document.coordinate_id, None);
        assert_eq!(document.point_count, 4);
        assert_eq!(document.primary.frequencies.len(), 4);
        assert_eq!(document.primary.loop_gains.len(), 4);
        assert_eq!(document.bode.frequencies.len(), 4);
        assert_eq!(document.bode.loop_gains.len(), 4);
        assert_eq!(document.nyquist.as_ref().unwrap().real.len(), 4);
        assert_eq!(document.retained_numeric_value_count().unwrap(), 4 * 12 + 6);
        assert_eq!(document.margins.units.gain_margin_db, StbUnit::Decibel);
        assert_eq!(
            document.margins.units.phase_margin_frequency,
            StbUnit::Hertz
        );

        let core_netlist = Netlist::parse(STB_DOCUMENT_DECK).expect("parse core STB deck");
        let core_result = Engine::new(SimulationConfig::default())
            .run_stb(
                &core_netlist,
                StbConfig::new()
                    .with_sweep(10.0, 1.0e3, 4)
                    .with_sweep_type(StbSweepType::Linear)
                    .with_probe("VPROBE")
                    .with_nyquist(true),
            )
            .expect("core STB reference executes");
        assert_eq!(document.primary.frequencies, core_result.frequencies);
        for (mapped, core) in document
            .primary
            .loop_gains
            .iter()
            .zip(&core_result.loop_gains)
        {
            assert_eq!(mapped.real.to_bits(), core.re.to_bits());
            assert_eq!(mapped.imaginary.to_bits(), core.im.to_bits());
        }
        for (index, core) in core_result.result.bode_points.iter().enumerate() {
            assert_eq!(
                document.bode.frequencies[index].to_bits(),
                core.frequency.to_bits()
            );
            assert_eq!(
                document.bode.magnitudes[index].to_bits(),
                core.magnitude.to_bits()
            );
            assert_eq!(
                document.bode.magnitudes_db[index].to_bits(),
                core.magnitude_db.to_bits()
            );
            assert_eq!(
                document.bode.phase_degrees[index].to_bits(),
                core.phase_deg.to_bits()
            );
            assert_eq!(
                document.bode.loop_gains[index].real.to_bits(),
                core.loop_gain.re.to_bits()
            );
            assert_eq!(
                document.bode.loop_gains[index].imaginary.to_bits(),
                core.loop_gain.im.to_bits()
            );
        }
        let mapped_nyquist = document.nyquist.as_ref().expect("mapped Nyquist data");
        for (index, core) in core_result.result.nyquist_points.iter().enumerate() {
            assert_eq!(mapped_nyquist.real[index].to_bits(), core.real.to_bits());
            assert_eq!(
                mapped_nyquist.imaginary[index].to_bits(),
                core.imag.to_bits()
            );
            assert_eq!(
                mapped_nyquist.frequencies[index].to_bits(),
                core.frequency.to_bits()
            );
        }
        let core_margins = &core_result.result.margins;
        assert_eq!(
            document.margins.gain_margin_db.to_bits(),
            core_margins.gain_margin_db.to_bits()
        );
        assert_eq!(
            document.margins.gain_margin_frequency.to_bits(),
            core_margins.gain_margin_freq.to_bits()
        );
        assert_eq!(
            document.margins.phase_margin_degrees.to_bits(),
            core_margins.phase_margin_deg.to_bits()
        );
        assert_eq!(
            document.margins.phase_margin_frequency.to_bits(),
            core_margins.phase_margin_freq.to_bits()
        );
        assert_eq!(
            document.margins.dc_gain_db.to_bits(),
            core_margins.dc_gain_db.to_bits()
        );
        assert_eq!(
            document.margins.unity_gain_bandwidth.to_bits(),
            core_margins.unity_gain_bandwidth.to_bits()
        );
        assert_eq!(
            document.margins.conditionally_stable,
            core_margins.conditionally_stable
        );
        assert_eq!(document.margins.num_crossovers, core_margins.num_crossovers);
        assert_eq!(document.margins.is_stable, core_margins.is_stable());

        let metadata = document.metadata(128).expect("STB metadata projects");
        assert!(metadata.has_nyquist);
        assert_eq!(metadata.series.len(), 10);
        assert!(metadata.series.iter().any(|descriptor| {
            descriptor.group == "bode"
                && descriptor.name == "phase_degrees"
                && descriptor.unit == StbUnit::Degree
        }));

        let json = serde_json::to_string(&document).expect("STB document JSON serializes");
        let decoded: StbResultDocument =
            serde_json::from_str(&json).expect("STB document JSON deserializes");
        decoded.validate().expect("STB JSON round trip validates");
        assert_eq!(decoded.primary.frequencies, document.primary.frequencies);
        for (decoded, original) in decoded
            .primary
            .loop_gains
            .iter()
            .zip(&document.primary.loop_gains)
        {
            assert!((decoded.real - original.real).abs() <= f64::EPSILON);
            assert!((decoded.imaginary - original.imaginary).abs() <= f64::EPSILON);
        }
        assert_eq!(
            decoded.nyquist.as_ref().map(|series| series.real.len()),
            document.nyquist.as_ref().map(|series| series.real.len())
        );
    }

    #[test]
    fn scalar_stb_optional_nyquist_and_exact_resource_accounting_are_fail_closed() {
        let without_nyquist = run_stb_document_detailed(
            STB_DOCUMENT_DECK,
            "VPROBE",
            WasmStbSweep::Linear,
            4,
            10.0,
            1.0e3,
            false,
        )
        .expect("STB executes without Nyquist projection");
        assert!(without_nyquist.nyquist.is_none());
        assert_eq!(
            without_nyquist.retained_numeric_value_count().unwrap(),
            4 * 9 + 6
        );

        let mut options = WasmExecutionOptions::default();
        options.resource_limits.max_result_values = 4 * 12 + 6 - 1;
        let error = run_stb_document_with_options_and_abort_detailed(
            STB_DOCUMENT_DECK,
            "VPROBE",
            WasmStbSweep::Linear,
            4,
            10.0,
            1.0e3,
            true,
            1,
            &options,
            &NoAbort,
        )
        .expect_err("one value below exact retained STB accounting must fail");
        assert_eq!(error.code, "resource_limit");
        assert_eq!(error.category, "resource_limit");
        assert_eq!(error.resource.as_deref(), Some("result_values"));
        assert_eq!(error.requested, Some(4 * 12 + 6));
        assert_eq!(error.limit, Some(4 * 12 + 5));
    }

    #[test]
    fn retained_stb_handle_enforces_exact_bounded_window_columns() {
        let document = run_stb_document_detailed(
            STB_DOCUMENT_DECK,
            "VPROBE",
            WasmStbSweep::Linear,
            4,
            10.0,
            1.0e3,
            true,
        )
        .expect("typed STB document executes");
        let cancelled_document = document.clone();
        let mut handle =
            WasmStbResultHandle::new_with_abort(document, ResourceLimits::default(), &NoAbort)
                .expect("valid handle");
        handle.maximum_window_values = 24;
        assert_eq!(
            handle
                .metadata_snapshot()
                .expect("STB metadata projects")
                .maximum_window_values,
            24
        );
        assert!(handle.window_snapshot(0, 2).is_ok());
        let error = handle
            .window_snapshot(0, 3)
            .expect_err("36-value STB window exceeds a 24-value ceiling");
        assert_eq!(error.code, "invalid_result_window");
        assert_eq!(error.category, "result_transfer");

        let abort = rspice_core::abort_signal::CountingAbort::new(3);
        let error = WasmStbResultHandle::new_with_abort(
            cancelled_document,
            ResourceLimits::default(),
            &abort,
        )
        .expect_err("retained-handle validation must remain cancellable");
        assert_eq!(error.code, "aborted");
        assert_eq!(error.category, "cancellation");
    }

    #[test]
    fn stb_boundary_validation_uses_typed_argument_errors() {
        let error = run_stb_document_detailed(
            STB_DOCUMENT_DECK,
            "VPROBE",
            WasmStbSweep::Decade,
            0,
            10.0,
            1.0e3,
            true,
        )
        .expect_err("zero STB density must fail at the browser boundary");
        assert_eq!(error.code, "invalid_argument");
        assert_eq!(error.category, "input_validation");

        let error = WasmStbSweep::parse("logarithmic")
            .expect_err("unknown STB sweep spelling must fail closed");
        assert_eq!(error.code, "invalid_argument");

        for error in [
            stb_metadata_error(StbDocumentError::Allocation(
                "synthetic metadata allocation failure".to_owned(),
            )),
            stb_window_error(StbDocumentError::Allocation(
                "synthetic window allocation failure".to_owned(),
            )),
        ] {
            assert_eq!(error.code, "result_allocation_failed");
            assert_eq!(error.category, "result_transfer");
        }
    }

    #[test]
    fn typed_documents_cover_scalar_op_dc_ac_tran_and_noise_without_schema_loss() {
        let op = run_operating_point_document_detailed(TYPED_DOCUMENT_DECK)
            .expect("typed OP document executes");
        assert_eq!(op.analysis.id, "op-001");
        assert_eq!(op.coordinate_id, None);
        assert!(
            op.signals
                .iter()
                .any(|signal| signal.kind == AnalogSignalKind::BranchCurrent)
        );

        let dc = run_dc_sweep_document_detailed(TYPED_DOCUMENT_DECK, "V1", -1.0, 1.0, 1.0)
            .expect("typed DC document executes");
        assert_eq!(dc.point_count, 3);
        assert_eq!(dc.axes[0].values, [-1.0, 0.0, 1.0]);
        assert!(
            dc.signals
                .iter()
                .any(|signal| signal.kind == AnalogSignalKind::DeviceObservable)
        );

        let ac = run_ac_document_detailed(TYPED_DOCUMENT_DECK, &[1.0, 10.0])
            .expect("typed AC document executes");
        assert_eq!(ac.analysis.id, "ac-001");
        assert!(ac.signals.iter().any(|signal| {
            signal.kind == AnalogSignalKind::BranchCurrent
                && matches!(signal.values, SignalValues::Complex { .. })
        }));

        let tran = run_transient_document_detailed(TYPED_DOCUMENT_DECK, 2.0e-6, 20.0e-9)
            .expect("typed transient document executes");
        assert_eq!(tran.axes[0].unit, Some(SignalUnit::Second));
        assert!(tran.signals.iter().any(|signal| {
            signal.canonical_name == "i(v1)" && matches!(signal.values, SignalValues::Real { .. })
        }));

        let noise =
            run_noise_document_detailed(TYPED_DOCUMENT_DECK, "out", None, "V1", &[1.0, 10.0])
                .expect("typed noise document executes");
        assert_eq!(noise.analysis.id, "noise-001");
        assert!(noise.signals.iter().any(|signal| {
            signal.canonical_name == "output_noise_density"
                && signal.unit == Some(SignalUnit::VoltSquaredPerHertz)
        }));
        assert!(noise.signals.iter().any(|signal| {
            signal.kind == AnalogSignalKind::BranchCurrent
                && matches!(signal.values, SignalValues::Complex { .. })
        }));
    }

    #[test]
    fn retained_result_handle_enforces_bounded_windows_and_exposes_descriptors_only() {
        let document = run_ac_document_detailed(TYPED_DOCUMENT_DECK, &[1.0, 10.0, 100.0])
            .expect("typed AC document executes");
        let mut handle =
            WasmAnalogResultHandle::new(document, ResourceLimits::default()).expect("valid handle");
        handle.maximum_window_values = 20;
        let metadata = handle.metadata_snapshot();
        assert_eq!(metadata.point_count, 3);
        assert_eq!(metadata.coordinate_id, None);
        assert!(metadata.maximum_window_values <= 20);
        assert!(handle.window_snapshot(0, 1).is_ok());
        let error = handle
            .window_snapshot(0, 3)
            .expect_err("oversized transfer must fail closed");
        assert_eq!(error.code, "invalid_result_window");
        assert_eq!(error.category, "result_transfer");
    }

    #[test]
    fn zero_timeout_cancels_and_oversized_timeout_fails_before_work() {
        let abort = ConfiguredAbort::new(Some(0), &NoAbort)
            .expect("a zero deadline is a valid immediate-cancellation policy");
        assert!(abort.is_aborted());

        let error = match ConfiguredAbort::new(Some(MAX_TIMEOUT_MILLISECONDS + 1), &NoAbort) {
            Ok(_) => panic!("an implausibly large browser deadline must fail closed"),
            Err(error) => error,
        };
        assert_eq!(error.code, "invalid_argument");
        assert!(error.message.contains("timeoutMilliseconds"));
    }

    const FFT_PARITY_DECK: &str = "browser transient FFT parity\n\
        V1 out 0 SIN(0 1 1k)\n\
        R1 out 0 1k\n\
        .options fft fft_mode=1 fft_accurate=0 fftout=1\n\
        .tran 1u 1m\n\
        .fft v(out) np=128 format=unorm window=hann freq=1k fmin=1k fmax=10k\n\
        .fft {2*v(out)} np=64 format=norm window=rect\n\
        .end\n";

    #[test]
    fn authored_deck_attaches_complete_fft_results_to_the_exact_transient_parent() {
        let document = run_authored_deck_document_detailed(FFT_PARITY_DECK)
            .expect("authored FFT deck executes");
        assert_eq!(document.results.len(), 1);
        assert_eq!(document.results[0].analysis_instance_id, "tran-001");
        assert_eq!(document.fft_results.len(), 2);
        for (index, fft) in document.fft_results.iter().enumerate() {
            assert_eq!(fft.coordinate_index, 0);
            assert_eq!(fft.parent_result_index, 0);
            assert_eq!(fft.snapshot.parent_analysis_id, "tran-001");
            assert_eq!(fft.snapshot.analysis_id, format!("fft-{:03}", index + 1));
            assert!(
                fft.output_namespace
                    .ends_with(&format!("/tran-001/fft-{:03}", index + 1))
            );
        }

        let analog_values = document.results[0].document.retained_numeric_value_count();
        let fft_values = document
            .fft_results
            .iter()
            .map(|fft| {
                let bin_values = fft.snapshot.bins.indices.len() * 6;
                let (metric_values, harmonic_values) =
                    fft.snapshot.metrics.as_ref().map_or((0, 0), |metrics| {
                        (
                            7 + usize::from(metrics.sfdr_spur_bin.is_some())
                                + usize::from(metrics.sfdr_spur_frequency.is_some()),
                            metrics.largest_harmonics.ranks.len() * 6,
                        )
                    });
                bin_values + metric_values + harmonic_values
            })
            .sum::<usize>();
        assert_eq!(
            document.retained_numeric_value_count().unwrap(),
            analog_values + fft_values
        );
    }

    #[test]
    fn authored_deck_attaches_each_global_fft_to_each_repeated_transient_parent() {
        let document = run_authored_deck_document_detailed(
            "repeated transient FFT parents\n\
             V1 out 0 SIN(0 1 1k)\n\
             R1 out 0 1k\n\
             .tran 10u 1m\n\
             .tran 20u 1m\n\
             .fft V(out) NP=16 FORMAT=UNORM WINDOW=HANN\n\
             .end\n",
        )
        .expect("each authored transient receives the global FFT request");

        assert_eq!(document.results.len(), 2);
        assert_eq!(document.fft_results.len(), 2);
        for (parent_index, expected_parent) in ["tran-001", "tran-002"].iter().enumerate() {
            let result = &document.results[parent_index];
            let fft = &document.fft_results[parent_index];
            assert_eq!(result.analysis_instance_id, *expected_parent);
            assert_eq!(fft.parent_result_index, parent_index);
            assert_eq!(fft.snapshot.parent_analysis_id, *expected_parent);
            assert_eq!(fft.snapshot.analysis_id, "fft-001");
            assert!(
                fft.output_namespace
                    .ends_with(&format!("/{expected_parent}/fft-001"))
            );
        }
    }

    const ANALOG_PARITY_DECK: &str = "browser complete analog transient parity\n\
        VDD d 0 5\n\
        VG g 0 PULSE(0 3 100n 20n 20n 500n 1u)\n\
        M1 d g 0 0 NM W=10u L=1u\n\
        .model NM NMOS (LEVEL=1 VTO=1 KP=100u)\n\
        VMEM memory 0 0.2\n\
        .model MRM MEMRISTOR LEVEL=2 RON=50 ROFF=1k\n\
        YMEMRISTOR MR1 memory 0 MRM IVRELATION=1\n\
        .save V(d) I(VDD) @M1[gm] @M1[id]\n\
        .tran 20n 2u\n\
        .end\n";

    fn synthetic_analog_result() -> TransientResult {
        TransientResult {
            time: vec![0.0, 1.0, 2.0],
            step_sizes: vec![0.0, 1.0, 1.0],
            voltages: vec![vec![1.0, 2.0, 3.0], Vec::new()],
            branch_currents: vec![vec![4.0, 5.0, 6.0], Vec::new()],
            num_nodes: 2,
            node_names: vec!["first".into(), "projected-node".into()],
            branch_names: vec!["VFIRST".into(), "VPROJECTED".into()],
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: vec![
                rspice_core::engine::TransientDeviceOpTrace {
                    device_name: "M2".into(),
                    parameter: "gm".into(),
                    values: vec![7.0, 8.0, 9.0],
                },
                rspice_core::engine::TransientDeviceOpTrace {
                    device_name: "M1".into(),
                    parameter: "id".into(),
                    values: vec![10.0, 11.0, 12.0],
                },
            ],
            store_traces: vec![
                rspice_core::engine::TransientStoreTrace {
                    name: "YMEMRISTOR!SECOND:R".into(),
                    values: vec![13.0, 14.0, 15.0],
                },
                rspice_core::engine::TransientStoreTrace {
                    name: "YMEMRISTOR!FIRST:R".into(),
                    values: vec![16.0, 17.0, 18.0],
                },
            ],
            fft_results: Vec::new(),
        }
    }

    fn synthetic_compressed_analog_result() -> TransientResultCompressed {
        let result = synthetic_analog_result();
        let compression_config = rspice_core::engine::CompressionConfig::default();
        TransientResultCompressed {
            time: result.time,
            step_sizes: result.step_sizes,
            voltages: result.voltages,
            branch_currents: result.branch_currents,
            num_nodes: result.num_nodes,
            node_names: result.node_names,
            branch_names: result.branch_names,
            device_op_traces: result.device_op_traces,
            store_traces: result.store_traces,
            fft_results: result.fft_results,
            compression_ratio: 2.0,
            input_points: 6,
            compression_report: rspice_core::engine::TransientCompressionReport {
                schema_version: rspice_core::engine::TRANSIENT_COMPRESSION_REPORT_VERSION,
                algorithm:
                    rspice_core::engine::TransientCompressionAlgorithm::MultiChannelRdpLinearV1,
                sample_domain:
                    rspice_core::engine::TransientCompressionSampleDomain::AcceptedInputSamples,
                applied_policy: (&compression_config).into(),
                input_points: 6,
                retained_points: 3,
                worst_observed: Some(rspice_core::engine::TransientCompressionErrorObservation {
                    signal: rspice_core::engine::TransientCompressionSignal::new(
                        rspice_core::engine::TransientCompressionSignalKind::Voltage,
                        "v(first)",
                    )
                    .expect("synthetic compression signal is valid"),
                    input_sample_index: 1,
                    time: 0.5,
                    actual_value: 1.5,
                    absolute_error: 0.0,
                    relative_error: Some(0.0),
                    allowed_tolerance: compression_config.abs_tol
                        + compression_config.rel_tol * 1.5,
                    tolerance_utilization: 0.0,
                }),
            },
        }
    }

    fn fft_parity_fixture() -> (TransientResult, TransientSnapshot) {
        let netlist = Netlist::parse(FFT_PARITY_DECK).expect("FFT parity deck parses in core");
        let core = Engine::new(SimulationConfig::default())
            .run_tran(&netlist, 1.0e-3, 1.0e-6)
            .expect("FFT parity deck executes in core");
        let wasm = run_transient_analysis_detailed(FFT_PARITY_DECK, 1.0e-3, 1.0e-6)
            .expect("FFT parity deck executes through browser adapter");
        (core, wasm)
    }

    fn assert_harmonic_parity(core: &[TransientFftHarmonic], wasm: &TransientFftHarmonicsSnapshot) {
        assert_eq!(
            wasm.ranks,
            core.iter()
                .map(|harmonic| harmonic.rank)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            wasm.bins,
            core.iter().map(|harmonic| harmonic.bin).collect::<Vec<_>>()
        );
        assert_eq!(
            wasm.frequencies,
            core.iter()
                .map(|harmonic| harmonic.frequency)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            wasm.magnitudes,
            core.iter()
                .map(|harmonic| harmonic.magnitude)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            wasm.magnitudes_db,
            core.iter()
                .map(|harmonic| harmonic.magnitude_db)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            wasm.phase_degrees,
            core.iter()
                .map(|harmonic| harmonic.phase_degrees)
                .collect::<Vec<_>>()
        );
    }

    fn assert_fft_parity(core: &TransientFftResult, wasm: &TransientFftSnapshot) {
        match &core.output {
            FftOutput::Probe(probe) => {
                assert_eq!(wasm.source_kind, "probe");
                assert_eq!(&wasm.source_text, probe);
                assert_eq!(&wasm.authored_output, probe);
            }
            FftOutput::Expression(expression) => {
                assert_eq!(wasm.source_kind, "expression");
                assert_eq!(&wasm.source_text, expression);
                assert_eq!(wasm.authored_output, format!("{{{expression}}}"));
            }
        }
        assert!(wasm.analysis_id.starts_with("fft-"));
        assert_eq!(wasm.parent_analysis_id, "tran-001");
        assert!(wasm.ordinal > 0);
        assert_eq!(wasm.output_name, core.output_name);
        assert_eq!(wasm.physical_type, core.physical_type);
        assert_eq!(
            wasm.value_unit.as_deref(),
            fft_value_unit(core.physical_type, core.format).unwrap()
        );
        assert_eq!(wasm.start_time, core.start_time);
        assert_eq!(wasm.stop_time, core.stop_time);
        assert_eq!(wasm.sample_interval, core.sample_interval);
        assert_eq!(wasm.point_count, core.point_count);
        assert_eq!(wasm.accurate_sampling, core.accurate_sampling);
        assert_eq!(wasm.format, fft_format_name(core.format));
        assert_eq!(wasm.mode, fft_mode_name(core.mode));
        assert_eq!(wasm.window, fft_window_name(core.window));
        assert_eq!(wasm.window_name, core.window_name);
        assert_eq!(wasm.alpha, core.alpha);
        assert_eq!(wasm.coherent_gain, core.coherent_gain);
        assert_eq!(wasm.frequency_resolution, core.frequency_resolution);
        assert_eq!(wasm.fundamental_bin, core.fundamental_bin);
        assert_eq!(wasm.minimum_metric_bin, core.minimum_metric_bin);
        assert_eq!(wasm.maximum_metric_bin, core.maximum_metric_bin);
        assert_eq!(
            wasm.bins.indices,
            core.bins.iter().map(|bin| bin.index).collect::<Vec<_>>()
        );
        assert_eq!(
            wasm.bins.frequencies,
            core.bins
                .iter()
                .map(|bin| bin.frequency)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            wasm.bins.real,
            core.bins.iter().map(|bin| bin.real).collect::<Vec<_>>()
        );
        assert_eq!(
            wasm.bins.imaginary,
            core.bins
                .iter()
                .map(|bin| bin.imaginary)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            wasm.bins.magnitudes,
            core.bins
                .iter()
                .map(|bin| bin.magnitude)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            wasm.bins.phase_degrees,
            core.bins
                .iter()
                .map(|bin| bin.phase_degrees)
                .collect::<Vec<_>>()
        );

        match (&core.metrics, &wasm.metrics) {
            (Some(core), Some(wasm)) => {
                assert_eq!(wasm.fundamental_magnitude, core.fundamental_magnitude);
                assert_eq!(wasm.thd_ratio, core.thd_ratio);
                assert_eq!(wasm.thd_db, core.thd_db);
                assert_eq!(wasm.sndr_db, core.sndr_db);
                assert_eq!(wasm.enob_bits, core.enob_bits);
                assert_eq!(wasm.snr_db, core.snr_db);
                assert_eq!(wasm.sfdr_db, core.sfdr_db);
                assert_eq!(wasm.sfdr_spur_bin, core.sfdr_spur_bin);
                assert_eq!(wasm.sfdr_spur_frequency, core.sfdr_spur_frequency);
                assert_harmonic_parity(&core.largest_harmonics, &wasm.largest_harmonics);
            }
            (None, None) => {}
            _ => panic!("browser FFT metrics optionality differs from core"),
        }
    }

    fn assert_object_fields(value: &serde_json::Value, expected: &[&str]) {
        let object = value.as_object().expect("contract value must be an object");
        let mut actual = object.keys().map(String::as_str).collect::<Vec<_>>();
        actual.sort_unstable();
        let mut expected = expected.to_vec();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }

    #[test]
    fn transient_fft_adapter_preserves_core_values_and_source_order() {
        let (core, wasm) = fft_parity_fixture();
        assert_eq!(wasm.fft_results.len(), core.fft_results.len());
        for (core, wasm) in core.fft_results.iter().zip(&wasm.fft_results) {
            assert_fft_parity(core, wasm);
        }
        assert_eq!(wasm.fft_results[0].analysis_id, "fft-001");
        assert_eq!(wasm.fft_results[0].ordinal, 1);
        assert_eq!(wasm.fft_results[1].analysis_id, "fft-002");
        assert_eq!(wasm.fft_results[1].ordinal, 2);
        assert_eq!(wasm.fft_results[0].output_name, "V(OUT)");
        assert_eq!(wasm.fft_results[1].output_name, "{2*v(out)}");
    }

    #[test]
    fn transient_fft_value_units_cover_every_supported_quantity_and_format() {
        use FftFormat::{Normalized, Unnormalized};

        assert_eq!(fft_value_unit("voltage", Normalized).unwrap(), Some("1"));
        assert_eq!(fft_value_unit("current", Normalized).unwrap(), Some("1"));
        assert_eq!(fft_value_unit("parameter", Normalized).unwrap(), Some("1"));
        assert_eq!(fft_value_unit("voltage", Unnormalized).unwrap(), Some("V"));
        assert_eq!(fft_value_unit("current", Unnormalized).unwrap(), Some("A"));
        assert_eq!(fft_value_unit("parameter", Unnormalized).unwrap(), None);
        assert!(fft_value_unit("unsupported", Normalized).is_err());
    }

    #[test]
    fn transient_fft_snapshot_conversion_rejects_unsupported_physical_type() {
        let netlist = Netlist::parse(FFT_PARITY_DECK).expect("FFT parity deck parses in core");
        let mut core = Engine::new(SimulationConfig::default())
            .run_tran(&netlist, 1.0e-3, 1.0e-6)
            .expect("FFT parity deck executes in core");
        core.fft_results[0].physical_type = "unsupported";

        let error = transient_snapshot_from_result(core)
            .expect_err("unknown FFT physical types must fail snapshot conversion");
        assert!(error.contains("unsupported transient FFT physical type 'unsupported'"));
    }

    #[test]
    fn transient_analog_adapter_preserves_complete_inventory_order_and_missingness() {
        let full = transient_snapshot_from_result(synthetic_analog_result())
            .expect("valid full analog result adapts");
        assert_eq!(full.time, [0.0, 1.0, 2.0]);
        assert_eq!(full.step_sizes, [0.0, 1.0, 1.0]);
        assert_eq!(full.num_nodes, 2);
        assert_eq!(full.node_names, ["first", "projected-node"]);
        assert_eq!(full.voltages[0].as_deref(), Some(&[1.0, 2.0, 3.0][..]));
        assert_eq!(full.voltages[1], None);
        assert_eq!(full.branch_names, ["VFIRST", "VPROJECTED"]);
        assert_eq!(
            full.branch_currents[0].as_deref(),
            Some(&[4.0, 5.0, 6.0][..])
        );
        assert_eq!(full.branch_currents[1], None);
        assert_eq!(
            full.device_op_traces
                .iter()
                .map(|trace| (trace.device_name.as_str(), trace.parameter.as_str()))
                .collect::<Vec<_>>(),
            [("M2", "gm"), ("M1", "id")]
        );
        assert_eq!(
            full.store_traces
                .iter()
                .map(|trace| trace.name.as_str())
                .collect::<Vec<_>>(),
            ["YMEMRISTOR!SECOND:R", "YMEMRISTOR!FIRST:R"]
        );
        assert_eq!(full.compression, None);

        let compressed =
            transient_snapshot_from_compressed_result(synthetic_compressed_analog_result())
                .expect("valid compressed analog result adapts");
        assert_eq!(compressed.time, full.time);
        assert_eq!(compressed.step_sizes, full.step_sizes);
        assert_eq!(compressed.node_names, full.node_names);
        assert_eq!(compressed.voltages, full.voltages);
        assert_eq!(compressed.branch_names, full.branch_names);
        assert_eq!(compressed.branch_currents, full.branch_currents);
        assert_eq!(compressed.device_op_traces, full.device_op_traces);
        assert_eq!(compressed.store_traces, full.store_traces);
        assert_eq!(
            compressed.compression,
            Some(TransientCompressionSnapshot {
                schema_version: rspice_core::engine::TRANSIENT_COMPRESSION_REPORT_VERSION,
                algorithm: "multi-channel-rdp-linear-v1".to_string(),
                sample_domain: "accepted-input-samples".to_string(),
                enabled: true,
                absolute_tolerance: 1.0e-6,
                relative_tolerance: 1.0e-3,
                maximum_retained_interval: 0.0,
                input_points: 6,
                retained_points: 3,
                compression_ratio: 2.0,
                worst_observed: Some(TransientCompressionErrorSnapshot {
                    signal_kind: "voltage".to_string(),
                    canonical_name: "v(first)".to_string(),
                    input_sample_index: 1,
                    time: 0.5,
                    actual_value: 1.5,
                    absolute_error: 0.0,
                    relative_error: Some(0.0),
                    allowed_tolerance: 1.501e-3,
                    tolerance_utilization: 0.0,
                }),
            })
        );
    }

    #[test]
    fn transient_analog_adapter_matches_actual_core_execution_inventory() {
        let netlist = Netlist::parse(ANALOG_PARITY_DECK).expect("analog parity deck parses");
        let core = Engine::new(SimulationConfig::default())
            .run_tran(&netlist, 2.0e-6, 20.0e-9)
            .expect("analog parity deck executes in core");
        let wasm = run_transient_analysis_detailed(ANALOG_PARITY_DECK, 2.0e-6, 20.0e-9)
            .expect("analog parity deck executes through browser adapter");

        assert_eq!(wasm.time, core.time);
        assert_eq!(wasm.step_sizes, core.step_sizes);
        assert_eq!(wasm.num_nodes, core.num_nodes);
        assert_eq!(wasm.node_names, core.node_names);
        assert_eq!(wasm.branch_names, core.branch_names);
        for (adapted, source) in wasm.voltages.iter().zip(&core.voltages) {
            assert_eq!(
                adapted.as_deref(),
                (!source.is_empty()).then_some(source.as_slice())
            );
        }
        for (adapted, source) in wasm.branch_currents.iter().zip(&core.branch_currents) {
            assert_eq!(
                adapted.as_deref(),
                (!source.is_empty()).then_some(source.as_slice())
            );
        }
        assert_eq!(
            wasm.device_op_traces
                .iter()
                .map(|trace| (
                    trace.device_name.as_str(),
                    trace.parameter.as_str(),
                    &trace.values
                ))
                .collect::<Vec<_>>(),
            core.device_op_traces
                .iter()
                .map(|trace| (
                    trace.device_name.as_str(),
                    trace.parameter.as_str(),
                    &trace.values
                ))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            wasm.store_traces
                .iter()
                .map(|trace| (trace.name.as_str(), &trace.values))
                .collect::<Vec<_>>(),
            core.store_traces
                .iter()
                .map(|trace| (trace.name.as_str(), &trace.values))
                .collect::<Vec<_>>()
        );
        assert!(
            wasm.voltages.iter().any(Option::is_none),
            "authored .SAVE projection must remain explicit"
        );
        assert!(
            wasm.branch_currents.iter().any(Option::is_none),
            "projected-out branch currents must remain explicit"
        );
        assert!(
            wasm.device_op_traces
                .iter()
                .any(|trace| trace.device_name.eq_ignore_ascii_case("M1")
                    && trace.parameter.eq_ignore_ascii_case("gm")),
            "requested device operating-point trace is missing"
        );
        assert_eq!(
            wasm.store_traces
                .iter()
                .map(|trace| trace.name.as_str())
                .collect::<Vec<_>>(),
            ["YMEMRISTOR!MR1:R"]
        );
        assert_eq!(wasm.compression, None);

        let compression_options = WasmCompressionOptions {
            absolute_tolerance: 1.0e-8,
            relative_tolerance: 1.0e-4,
            enabled: true,
            maximum_interval: 100.0e-9,
        };
        let compressed_core = Engine::new(SimulationConfig::default())
            .run_tran_compressed(
                &netlist,
                2.0e-6,
                20.0e-9,
                compression_options
                    .to_core()
                    .expect("compression options are valid"),
            )
            .expect("analog parity deck executes through core compression");
        let compressed = transient_snapshot_from_compressed_result(compressed_core.clone())
            .expect("actual compressed core result adapts");
        let public_compressed = run_transient_analysis_compressed_detailed(
            ANALOG_PARITY_DECK,
            2.0e-6,
            20.0e-9,
            &compression_options,
        )
        .expect("compressed browser API executes");
        assert_eq!(public_compressed, compressed);
        assert_eq!(compressed.time, compressed_core.time);
        assert_eq!(compressed.step_sizes, compressed_core.step_sizes);
        assert_eq!(compressed.node_names, compressed_core.node_names);
        assert_eq!(compressed.branch_names, compressed_core.branch_names);
        assert_eq!(
            compressed.device_op_traces.len(),
            compressed_core.device_op_traces.len()
        );
        assert_eq!(
            compressed.store_traces.len(),
            compressed_core.store_traces.len()
        );
        assert_eq!(
            compressed.compression,
            Some(TransientCompressionSnapshot {
                schema_version: compressed_core.compression_report.schema_version,
                algorithm: compressed_core
                    .compression_report
                    .algorithm
                    .as_str()
                    .to_string(),
                sample_domain: compressed_core
                    .compression_report
                    .sample_domain
                    .as_str()
                    .to_string(),
                enabled: compressed_core.compression_report.applied_policy.enabled,
                absolute_tolerance: compressed_core
                    .compression_report
                    .applied_policy
                    .absolute_tolerance,
                relative_tolerance: compressed_core
                    .compression_report
                    .applied_policy
                    .relative_tolerance,
                maximum_retained_interval: compressed_core
                    .compression_report
                    .applied_policy
                    .maximum_retained_interval,
                input_points: compressed_core.input_points,
                retained_points: compressed_core.time.len(),
                compression_ratio: compressed_core.compression_ratio,
                worst_observed: compressed_core.compression_report.worst_observed.map(
                    |observation| TransientCompressionErrorSnapshot {
                        signal_kind: observation.signal.kind.as_str().to_string(),
                        canonical_name: observation.signal.canonical_name,
                        input_sample_index: observation.input_sample_index,
                        time: observation.time,
                        actual_value: observation.actual_value,
                        absolute_error: observation.absolute_error,
                        relative_error: observation.relative_error,
                        allowed_tolerance: observation.allowed_tolerance,
                        tolerance_utilization: observation.tolerance_utilization,
                    }
                ),
            })
        );
    }

    #[test]
    fn transient_compression_options_fail_closed() {
        for options in [
            WasmCompressionOptions {
                absolute_tolerance: -1.0,
                ..WasmCompressionOptions::default()
            },
            WasmCompressionOptions {
                relative_tolerance: f64::NAN,
                ..WasmCompressionOptions::default()
            },
            WasmCompressionOptions {
                maximum_interval: f64::INFINITY,
                ..WasmCompressionOptions::default()
            },
        ] {
            let error = options
                .to_core()
                .expect_err("invalid compression policy must be rejected");
            assert_eq!(error.kind, "invalid_argument");
            assert_eq!(error.category, "input_validation");
        }

        let unknown = serde_json::from_value::<WasmCompressionOptions>(serde_json::json!({
            "absoluteTolerance": 1.0e-6,
            "misspelledTolerance": 1.0e-3,
        }));
        assert!(
            unknown.is_err(),
            "unknown compression fields must fail closed"
        );
    }

    #[test]
    fn transient_fft_dto_round_trips_and_inventory_covers_every_field() {
        const TRANSIENT_FIELDS: &[&str] = &[
            "time",
            "step_sizes",
            "num_nodes",
            "node_names",
            "voltages",
            "branch_names",
            "branch_currents",
            "device_op_traces",
            "store_traces",
            "fft_results",
            "compression",
        ];
        const FFT_FIELDS: &[&str] = &[
            "analysis_id",
            "parent_analysis_id",
            "ordinal",
            "source_kind",
            "source_text",
            "authored_output",
            "output_name",
            "physical_type",
            "value_unit",
            "start_time",
            "stop_time",
            "sample_interval",
            "point_count",
            "accurate_sampling",
            "format",
            "mode",
            "window",
            "window_name",
            "alpha",
            "coherent_gain",
            "frequency_resolution",
            "fundamental_bin",
            "minimum_metric_bin",
            "maximum_metric_bin",
            "bins",
            "metrics",
        ];
        const BIN_FIELDS: &[&str] = &[
            "indices",
            "frequencies",
            "real",
            "imaginary",
            "magnitudes",
            "phase_degrees",
        ];
        const METRIC_FIELDS: &[&str] = &[
            "fundamental_magnitude",
            "thd_ratio",
            "thd_db",
            "sndr_db",
            "enob_bits",
            "snr_db",
            "sfdr_db",
            "sfdr_spur_bin",
            "sfdr_spur_frequency",
            "largest_harmonics",
        ];
        const HARMONIC_FIELDS: &[&str] = &[
            "ranks",
            "bins",
            "frequencies",
            "magnitudes",
            "magnitudes_db",
            "phase_degrees",
        ];
        const DEVICE_OP_FIELDS: &[&str] = &["device_name", "parameter", "values"];
        const STORE_FIELDS: &[&str] = &["name", "values"];
        const COMPRESSION_FIELDS: &[&str] = &[
            "schema_version",
            "algorithm",
            "sample_domain",
            "enabled",
            "absolute_tolerance",
            "relative_tolerance",
            "maximum_retained_interval",
            "input_points",
            "retained_points",
            "compression_ratio",
            "worst_observed",
        ];
        const COMPRESSION_ERROR_FIELDS: &[&str] = &[
            "signal_kind",
            "canonical_name",
            "input_sample_index",
            "time",
            "actual_value",
            "absolute_error",
            "relative_error",
            "allowed_tolerance",
            "tolerance_utilization",
        ];

        let (_, snapshot) = fft_parity_fixture();
        let encoded = serde_json::to_value(&snapshot).expect("serialize transient FFT DTO");
        assert_object_fields(&encoded, TRANSIENT_FIELDS);
        let first = &encoded["fft_results"][0];
        assert_object_fields(first, FFT_FIELDS);
        assert_eq!(first["physical_type"], "voltage");
        assert_eq!(first["value_unit"], "V");
        assert_eq!(encoded["fft_results"][1]["physical_type"], "parameter");
        assert_eq!(encoded["fft_results"][1]["value_unit"], "1");
        assert_object_fields(&first["bins"], BIN_FIELDS);
        assert_object_fields(&first["metrics"], METRIC_FIELDS);
        assert_object_fields(&first["metrics"]["largest_harmonics"], HARMONIC_FIELDS);

        let decoded: TransientSnapshot =
            serde_json::from_value(encoded).expect("deserialize transient FFT DTO");
        assert_eq!(decoded, snapshot);

        let mut without_metrics = snapshot.clone();
        without_metrics.fft_results[0].metrics = None;
        let encoded = serde_json::to_value(without_metrics).expect("serialize absent metrics");
        assert!(encoded["fft_results"][0]["metrics"].is_null());

        let mut unnormalized_parameter = snapshot;
        unnormalized_parameter.fft_results[1].format = "unnormalized".to_string();
        unnormalized_parameter.fft_results[1].value_unit = None;
        let encoded = serde_json::to_value(unnormalized_parameter)
            .expect("serialize unnormalized parameter FFT unit");
        assert!(encoded["fft_results"][1]["value_unit"].is_null());

        let analog =
            transient_snapshot_from_compressed_result(synthetic_compressed_analog_result())
                .expect("compressed analog DTO adapts");
        let encoded = serde_json::to_value(&analog).expect("serialize complete analog DTO");
        assert_object_fields(&encoded["device_op_traces"][0], DEVICE_OP_FIELDS);
        assert_object_fields(&encoded["store_traces"][0], STORE_FIELDS);
        assert_object_fields(&encoded["compression"], COMPRESSION_FIELDS);
        assert_object_fields(
            &encoded["compression"]["worst_observed"],
            COMPRESSION_ERROR_FIELDS,
        );
        assert!(encoded["voltages"][1].is_null());
        assert!(encoded["branch_currents"][1].is_null());
        let decoded: TransientSnapshot =
            serde_json::from_value(encoded).expect("deserialize complete analog DTO");
        assert_eq!(decoded, analog);
    }

    #[cfg(target_arch = "wasm32")]
    fn js_shared_cancellation_options(cancelled: bool) -> JsValue {
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

    #[cfg(target_arch = "wasm32")]
    fn assert_js_error_code(error: JsValue, expected: &str) {
        assert_eq!(
            js_property(&error, "code")
                .expect("RSpiceError has a code")
                .as_string()
                .as_deref(),
            Some(expected)
        );
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn node_shared_control_word_cancels_every_analysis_export() {
        let options = || js_shared_cancellation_options(true);

        assert_js_error_code(
            run_dc_operating_point_js(CANCELLATION_DECK, options())
                .expect_err("pre-set shared flag cancels OP"),
            "aborted",
        );
        assert_js_error_code(
            run_ac_analysis_js(CANCELLATION_DECK, vec![1.0, 10.0], options())
                .expect_err("pre-set shared flag cancels AC"),
            "aborted",
        );
        assert_js_error_code(
            run_transient_analysis_js(CANCELLATION_DECK, 10.0e-6, 1.0e-9, options())
                .expect_err("pre-set shared flag cancels TRAN"),
            "aborted",
        );
        assert_js_error_code(
            run_transient_analysis_compressed_js(
                CANCELLATION_DECK,
                10.0e-6,
                1.0e-9,
                JsValue::NULL,
                options(),
            )
            .expect_err("pre-set shared flag cancels compressed TRAN"),
            "aborted",
        );
        assert_js_error_code(
            run_operating_point_document_js(CANCELLATION_DECK, 1, options())
                .expect_err("pre-set shared flag cancels typed OP"),
            "aborted",
        );
        assert_js_error_code(
            run_dc_sweep_document_js(CANCELLATION_DECK, "V1", 0.0, 1.0, 0.5, 1, options())
                .expect_err("pre-set shared flag cancels typed DC"),
            "aborted",
        );
        assert_js_error_code(
            run_ac_document_js(CANCELLATION_DECK, vec![1.0, 10.0], 1, options())
                .expect_err("pre-set shared flag cancels typed AC"),
            "aborted",
        );
        assert_js_error_code(
            run_transient_document_js(CANCELLATION_DECK, 10.0e-6, 1.0e-9, 1, options())
                .expect_err("pre-set shared flag cancels typed TRAN"),
            "aborted",
        );
        assert_js_error_code(
            run_noise_document_js(
                CANCELLATION_DECK,
                "out",
                None,
                "V1",
                vec![1.0, 10.0],
                1,
                options(),
            )
            .expect_err("pre-set shared flag cancels typed noise"),
            "aborted",
        );
        assert_js_error_code(
            run_stb_document_js(
                CANCELLATION_DECK,
                "V1",
                "linear",
                2,
                1.0,
                10.0,
                true,
                1,
                options(),
            )
            .expect_err("pre-set shared flag cancels typed STB"),
            "aborted",
        );
        assert_js_error_code(
            run_authored_deck_document_js(
                "authored JS cancellation\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n",
                options(),
            )
            .expect_err("pre-set shared flag cancels authored deck execution"),
            "aborted",
        );
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn authored_deck_public_js_handle_preserves_axes_ids_and_typed_windows() {
        let handle = run_authored_deck_document_js(AUTHORED_TEMP_AC_DECK, JsValue::NULL)
            .expect("public authored deck export executes under Node");
        assert_eq!(handle.coordinate_count(), 2);
        assert_eq!(handle.result_count(), 2);
        assert_eq!(handle.fft_result_count(), 0);

        let metadata = handle.metadata_js().expect("deck metadata serializes");
        assert_eq!(
            js_property(&metadata, "schema")
                .expect("schema exists")
                .as_string()
                .as_deref(),
            Some(DECK_RESULT_SCHEMA)
        );
        let coordinates = js_array_property(&metadata, "coordinates")
            .expect("canonical coordinate descriptors exist");
        assert_eq!(coordinates.length(), 2);
        let results =
            js_array_property(&metadata, "results").expect("canonical result summaries exist");
        assert_eq!(results.length(), 2);
        assert_eq!(
            js_property(&results.get(0), "analysisInstanceId")
                .expect("stable analysis instance id exists")
                .as_string()
                .as_deref(),
            Some("ac-001")
        );
        assert_ne!(
            js_property(&coordinates.get(0), "id")
                .expect("first coordinate id exists")
                .as_string(),
            js_property(&coordinates.get(1), "id")
                .expect("second coordinate id exists")
                .as_string()
        );

        let result_metadata = handle
            .result_metadata_js(0)
            .expect("coordinate-local schema serializes");
        assert_eq!(
            js_property(&result_metadata, "coordinateId")
                .expect("result coordinate id exists")
                .as_string(),
            js_property(&coordinates.get(0), "id")
                .expect("coordinate id exists")
                .as_string()
        );
        let window = handle
            .read_window_js(0, 0, 1)
            .expect("bounded coordinate-local window transfers");
        let axes = js_array_property(&window, "axes").expect("result axes exist");
        assert!(
            js_property(&axes.get(0), "values")
                .expect("frequency values exist")
                .is_instance_of::<js_sys::Float64Array>()
        );
        let signals = js_array_property(&window, "signals").expect("result signals exist");
        let values = js_property(&signals.get(0), "values").expect("signal values exist");
        assert!(
            js_property(&values, "real")
                .expect("complex real values exist")
                .is_instance_of::<js_sys::Float64Array>()
        );
        assert!(
            js_property(&values, "validity")
                .expect("signal validity exists")
                .is_instance_of::<js_sys::Uint8Array>()
        );
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn authored_deck_node_regressions_cover_step_repetition_conditionals_and_fail_closed() {
        let step = run_authored_deck_document_js(AUTHORED_STEP_TRAN_DECK, JsValue::NULL)
            .expect("public STEP/TRAN deck executes");
        assert_eq!(step.coordinate_count(), 2);
        assert_eq!(step.result_count(), 2);
        assert!(
            step.document()
                .results
                .iter()
                .all(|result| result.analysis_instance_id == "tran-001")
        );

        let repeated = run_authored_deck_document_js(
            "Node repeated analyses\n\
             V1 out 0 DC 1 AC 1\n\
             R1 out 0 1k\n\
             .op\n\
             .ac lin 3 1 10\n\
             .ac lin 4 10 100\n\
             .end\n",
            JsValue::NULL,
        )
        .expect("public repeated-analysis deck executes");
        assert_eq!(
            repeated
                .document()
                .results
                .iter()
                .map(|result| result.analysis_instance_id.as_str())
                .collect::<Vec<_>>(),
            ["op-001", "ac-001", "ac-002"]
        );

        let conditional = |values: &str| {
            run_authored_deck_document_js(
                &format!(
                    "Node conditional topology\n\
                     .param sel=0\n\
                     V1 in 0 AC 1\n\
                     .step param sel list {values}\n\
                     .if (sel==0)\n\
                     R1 in 0 1k\n\
                     .else\n\
                     R1 in mid 1k\n\
                     R2 mid 0 1k\n\
                     .endif\n\
                     .ac lin 3 1 10\n\
                     .end\n"
                ),
                JsValue::NULL,
            )
            .expect("public conditional deck executes")
        };
        let forward = conditional("0 1");
        let reverse = conditional("1 0");
        let schemas = |handle: &WasmDeckResultHandle| {
            handle
                .document()
                .results
                .iter()
                .map(|result| {
                    (
                        result.document.coordinate_id.clone().unwrap(),
                        result
                            .document
                            .signals
                            .iter()
                            .map(|signal| signal.canonical_name.clone())
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<std::collections::BTreeMap<_, _>>()
        };
        assert_eq!(schemas(&forward), schemas(&reverse));

        assert_js_error_code(
            run_authored_deck_document_js(
                "Node unsupported\nV1 out 0 1\nR1 out 0 1k\n.tf V(out) V1\n.end\n",
                JsValue::NULL,
            )
            .expect_err("unmapped authored analysis must fail through the JS export"),
            "unsupported_deck_analysis",
        );
        assert_js_error_code(
            run_authored_deck_document_js(
                "Node malformed\nV1 out 0 1\nR1 out 0 1k!\n.end\n",
                JsValue::NULL,
            )
            .expect_err("malformed authored deck must fail through the JS export"),
            "parse_error",
        );
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn node_cancellation_options_fail_closed() {
        let unsupported = js_sys::Object::new();
        let unsupported_cancellation = js_sys::Object::new();
        js_sys::Reflect::set(
            &unsupported_cancellation,
            &JsValue::from_str("mechanism"),
            &JsValue::from_str("abortSignal"),
        )
        .expect("set unsupported mechanism");
        js_sys::Reflect::set(
            &unsupported,
            &JsValue::from_str("cancellation"),
            &unsupported_cancellation,
        )
        .expect("set unsupported cancellation object");
        assert_js_error_code(
            run_dc_operating_point_js(CANCELLATION_DECK, unsupported.into())
                .expect_err("DOM AbortSignal must not appear supported"),
            "unsupported_cancellation",
        );

        let ordinary = js_sys::Object::new();
        let ordinary_cancellation = js_sys::Object::new();
        js_sys::Reflect::set(
            &ordinary_cancellation,
            &JsValue::from_str("mechanism"),
            &JsValue::from_str("sharedInt32"),
        )
        .expect("set shared mechanism");
        let ordinary_view = js_sys::Int32Array::new_with_length(1);
        js_sys::Reflect::set(
            &ordinary_cancellation,
            &JsValue::from_str("view"),
            &ordinary_view,
        )
        .expect("set ordinary view");
        js_sys::Reflect::set(
            &ordinary,
            &JsValue::from_str("cancellation"),
            &ordinary_cancellation,
        )
        .expect("set ordinary cancellation object");
        assert_js_error_code(
            run_dc_operating_point_js(CANCELLATION_DECK, ordinary.into())
                .expect_err("ordinary ArrayBuffer must not masquerade as shared cancellation"),
            "invalid_argument",
        );

        for field in ["resourceLimits", "timeoutMilliseconds", "cancellation"] {
            let malformed = js_sys::Object::new();
            js_sys::Reflect::set(&malformed, &JsValue::from_str(field), &JsValue::NULL)
                .expect("set explicit null execution option");
            assert_js_error_code(
                run_dc_operating_point_js(CANCELLATION_DECK, malformed.into())
                    .expect_err("explicit null execution fields must not become defaults"),
                "invalid_argument",
            );
        }

        let timeout = js_sys::Object::new();
        js_sys::Reflect::set(
            &timeout,
            &JsValue::from_str("timeoutMilliseconds"),
            &JsValue::from_f64(0.0),
        )
        .expect("set zero timeout");
        assert_js_error_code(
            run_dc_operating_point_js(CANCELLATION_DECK, timeout.into())
                .expect_err("zero timeout requests immediate cancellation"),
            "aborted",
        );
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn transient_analog_js_contract_uses_typed_arrays_and_explicit_missingness() {
        let snapshot = transient_snapshot_from_result(synthetic_analog_result())
            .expect("full analog fixture adapts");
        let serialized =
            serialize_transient_to_js(&snapshot).expect("serialize complete analog DTO");

        for field in ["time", "step_sizes"] {
            assert!(
                js_property(&serialized, field)
                    .expect("top-level numeric column exists")
                    .is_instance_of::<js_sys::Float64Array>(),
                "{field} is not a Float64Array"
            );
        }

        let voltages =
            js_array_property(&serialized, "voltages").expect("voltage waveform collection exists");
        assert!(voltages.get(0).is_instance_of::<js_sys::Float64Array>());
        assert!(voltages.get(1).is_null());
        let currents = js_array_property(&serialized, "branch_currents")
            .expect("branch-current waveform collection exists");
        assert!(currents.get(0).is_instance_of::<js_sys::Float64Array>());
        assert!(currents.get(1).is_null());

        for collection in ["device_op_traces", "store_traces"] {
            let traces =
                js_array_property(&serialized, collection).expect("trace collection exists");
            assert!(
                js_property(&traces.get(0), "values")
                    .expect("trace values exist")
                    .is_instance_of::<js_sys::Float64Array>(),
                "{collection} values are not a Float64Array"
            );
        }
        assert!(
            js_property(&serialized, "compression")
                .expect("compression property exists")
                .is_null()
        );

        let decoded: TransientSnapshot = serde_wasm_bindgen::from_value(serialized)
            .expect("typed-array analog contract round-trips to its Rust DTO");
        assert_eq!(decoded, snapshot);

        let compressed =
            transient_snapshot_from_compressed_result(synthetic_compressed_analog_result())
                .expect("compressed analog fixture adapts");
        let serialized =
            serialize_transient_to_js(&compressed).expect("serialize compressed analog DTO");
        assert!(
            !js_property(&serialized, "compression")
                .expect("compression property exists")
                .is_null()
        );
        let decoded: TransientSnapshot = serde_wasm_bindgen::from_value(serialized)
            .expect("compressed analog contract round-trips to its Rust DTO");
        assert_eq!(decoded, compressed);

        let compression_options = serde_wasm_bindgen::to_value(&WasmCompressionOptions {
            absolute_tolerance: 1.0e-8,
            relative_tolerance: 1.0e-4,
            maximum_interval: 100.0e-9,
            enabled: true,
        })
        .expect("compression options serialize");
        let executed = run_transient_analysis_compressed_js(
            ANALOG_PARITY_DECK,
            2.0e-6,
            20.0e-9,
            compression_options,
            JsValue::NULL,
        )
        .expect("compressed analog API executes under wasm32");
        assert!(
            js_property(&executed, "time")
                .expect("executed time exists")
                .is_instance_of::<js_sys::Float64Array>()
        );
        assert!(
            !js_property(&executed, "compression")
                .expect("executed compression provenance exists")
                .is_null()
        );
        assert!(
            js_array_property(&executed, "device_op_traces")
                .expect("executed device operating-point traces exist")
                .length()
                >= 2
        );
        assert_eq!(
            js_array_property(&executed, "store_traces")
                .expect("executed typed store traces exist")
                .length(),
            1
        );
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn transient_fft_js_contract_uses_typed_numeric_columns_and_explicit_null() {
        let (_, mut snapshot) = fft_parity_fixture();
        snapshot.fft_results[1].metrics = None;
        let serialized = serialize_transient_to_js(&snapshot).expect("serialize browser FFT DTO");
        let fft_results = js_property(&serialized, "fft_results")
            .expect("FFT result collection exists")
            .dyn_into::<js_sys::Array>()
            .expect("FFT result collection is an array");
        let first = fft_results.get(0);
        let bins = js_property(&first, "bins").expect("FFT bin object exists");
        assert!(
            js_property(&bins, "indices")
                .expect("FFT indices exist")
                .is_instance_of::<js_sys::Uint32Array>()
        );
        for field in [
            "frequencies",
            "real",
            "imaginary",
            "magnitudes",
            "phase_degrees",
        ] {
            assert!(
                js_property(&bins, field)
                    .expect("FFT numeric column exists")
                    .is_instance_of::<js_sys::Float64Array>()
            );
        }

        let metrics = js_property(&first, "metrics").expect("FFT metrics property exists");
        let harmonics =
            js_property(&metrics, "largest_harmonics").expect("FFT ranked harmonic object exists");
        assert!(
            js_property(&harmonics, "ranks")
                .expect("FFT harmonic ranks exist")
                .is_instance_of::<js_sys::Uint32Array>()
        );
        assert!(
            js_property(&harmonics, "magnitudes")
                .expect("FFT harmonic magnitudes exist")
                .is_instance_of::<js_sys::Float64Array>()
        );

        let second = fft_results.get(1);
        assert!(
            js_property(&second, "metrics")
                .expect("FFT metrics property exists")
                .is_null()
        );

        let decoded: TransientSnapshot = serde_wasm_bindgen::from_value(serialized)
            .expect("typed-array FFT contract round-trips to its Rust DTO");
        assert_eq!(decoded, snapshot);
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn result_document_windows_use_typed_numeric_and_validity_columns() {
        let document = run_ac_document_detailed(TYPED_DOCUMENT_DECK, &[1.0, 10.0])
            .expect("typed AC document executes under wasm32");
        let window = document.window(0, 2, 128).expect("bounded window exists");
        let serialized =
            serialize_result_window_to_js(&window).expect("serialize typed result window");

        let axes = js_array_property(&serialized, "axes").expect("axis collection exists");
        assert!(
            js_property(&axes.get(0), "values")
                .expect("axis values exist")
                .is_instance_of::<js_sys::Float64Array>()
        );
        let signals = js_array_property(&serialized, "signals").expect("signal collection exists");
        let values = js_property(&signals.get(0), "values").expect("signal values exist");
        assert!(
            js_property(&values, "real")
                .expect("complex real values exist")
                .is_instance_of::<js_sys::Float64Array>()
        );
        assert!(
            js_property(&values, "imaginary")
                .expect("complex imaginary values exist")
                .is_instance_of::<js_sys::Float64Array>()
        );
        assert!(
            js_property(&values, "validity")
                .expect("validity values exist")
                .is_instance_of::<js_sys::Uint8Array>()
        );
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn stb_windows_use_typed_columns_and_explicit_optional_nyquist() {
        let document = run_stb_document_detailed(
            STB_DOCUMENT_DECK,
            "VPROBE",
            WasmStbSweep::Linear,
            4,
            10.0,
            1.0e3,
            true,
        )
        .expect("typed STB document executes under wasm32");
        let window = document
            .window(0, 2, 128)
            .expect("bounded STB window exists");
        let serialized =
            serialize_stb_result_window_to_js(&window).expect("serialize typed STB window");

        let primary = js_property(&serialized, "primary").expect("primary STB group exists");
        let primary_gain = js_property(&primary, "loopGain").expect("primary loop gain exists");
        for (object, fields) in [
            (&primary, &["frequencies"][..]),
            (&primary_gain, &["real", "imaginary"][..]),
        ] {
            for field in fields {
                assert!(
                    js_property(object, field)
                        .expect("primary STB numeric field exists")
                        .is_instance_of::<js_sys::Float64Array>()
                );
            }
        }
        let bode = js_property(&serialized, "bode").expect("Bode STB group exists");
        for field in ["frequencies", "magnitudes", "magnitudesDb", "phaseDegrees"] {
            assert!(
                js_property(&bode, field)
                    .expect("Bode numeric field exists")
                    .is_instance_of::<js_sys::Float64Array>()
            );
        }
        let nyquist = js_property(&serialized, "nyquist").expect("Nyquist group exists");
        for field in ["real", "imaginary", "frequencies"] {
            assert!(
                js_property(&nyquist, field)
                    .expect("Nyquist numeric field exists")
                    .is_instance_of::<js_sys::Float64Array>()
            );
        }

        let without_nyquist = run_stb_document_detailed(
            STB_DOCUMENT_DECK,
            "VPROBE",
            WasmStbSweep::Linear,
            2,
            10.0,
            100.0,
            false,
        )
        .expect("STB without Nyquist executes under wasm32");
        let serialized = serialize_stb_result_window_to_js(
            &without_nyquist
                .window(0, 2, 128)
                .expect("bounded non-Nyquist STB window exists"),
        )
        .expect("serialize non-Nyquist STB window");
        assert!(
            js_property(&serialized, "nyquist")
                .expect("Nyquist optionality is explicit")
                .is_null()
        );
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn public_stb_export_returns_structured_metadata_and_typed_windows() {
        let handle = run_stb_document_js(
            STB_DOCUMENT_DECK,
            "VPROBE",
            "linear",
            4,
            10.0,
            1.0e3,
            true,
            1,
            JsValue::UNDEFINED,
        )
        .expect("public STB export executes");

        assert_eq!(handle.analysis_id(), "stb-001");
        assert_eq!(handle.point_count(), 4);
        let metadata = handle.metadata_js().expect("public metadata serializes");
        assert_eq!(
            js_property(&metadata, "schema")
                .expect("metadata schema")
                .as_string()
                .as_deref(),
            Some(STB_RESULT_SCHEMA)
        );
        assert!(
            js_property(&metadata, "margins")
                .expect("metadata margins")
                .is_object()
        );

        let window = handle
            .read_window_js(0, 2)
            .expect("public bounded window serializes");
        let primary = js_property(&window, "primary").expect("public primary series");
        assert!(
            js_property(&primary, "frequencies")
                .expect("public primary frequencies")
                .is_instance_of::<js_sys::Float64Array>()
        );
        let nyquist = js_property(&window, "nyquist").expect("public Nyquist series");
        assert!(
            js_property(&nyquist, "real")
                .expect("public Nyquist real values")
                .is_instance_of::<js_sys::Float64Array>()
        );
    }
}
