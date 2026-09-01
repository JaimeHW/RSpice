//! WebAssembly wrapper for the RSpice simulation core.
//!
//! The crate keeps the browser-facing API intentionally thin: it exposes
//! serializable snapshots that mirror stable simulator concepts while delegating
//! all numerical work to `rspice-core`.

use rspice_core::{
    Engine, Netlist, ResourceKind, ResourceLimitError, ResourceLimits, SimulationConfig,
};
use rspice_core::{
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
    pub input_points: usize,
    pub retained_points: usize,
    pub compression_ratio: f64,
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
) -> DetailedWasmResult<Netlist> {
    Netlist::parse_validated_with_options(
        source,
        rspice_core::netlist::NetlistParseOptions {
            resource_limits,
            ..rspice_core::netlist::NetlistParseOptions::default()
        },
    )
    .map_err(|error| Box::new(WasmError::from_parse_error(error)))
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

fn execution_options_from_js(value: JsValue) -> DetailedWasmResult<WasmExecutionOptions> {
    if value.is_undefined() || value.is_null() {
        return Ok(WasmExecutionOptions::default());
    }
    serde_wasm_bindgen::from_value(value).map_err(|error| {
        Box::new(WasmError::invalid_argument(format!(
            "invalid execution options: {error}"
        )))
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
            "serialization failed: transient property `{name}` is unavailable"
        ))
    })
}

fn set_float64_array(object: &JsValue, name: &str, values: &[f64]) -> Result<(), JsValue> {
    let values = js_sys::Float64Array::from(values);
    js_sys::Reflect::set(object, &JsValue::from_str(name), values.as_ref())
        .map(|_| ())
        .map_err(|_| {
            JsValue::from_str(&format!(
                "serialization failed: cannot publish transient typed array `{name}`"
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
            "serialization failed: transient `{name}` index exceeds JavaScript array bounds"
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
                "serialization failed: transient property `{name}` is not an array"
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
                "serialization failed: transient `{name}` index exceeds JavaScript array bounds"
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

fn fft_snapshot(result: &TransientFftResult) -> TransientFftSnapshot {
    let (source_kind, source_text, authored_output) = fft_output_identity(&result.output);
    TransientFftSnapshot {
        source_kind: source_kind.to_string(),
        source_text: source_text.to_string(),
        authored_output,
        output_name: result.output_name.clone(),
        physical_type: result.physical_type.to_string(),
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
    }
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
    let fft_results = result.fft_results.iter().map(fft_snapshot).collect();
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
    let fft_results = result.fft_results.iter().map(fft_snapshot).collect();
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
            input_points: result.input_points,
            retained_points: point_count,
            compression_ratio: result.compression_ratio,
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
    let netlist = parse_netlist_detailed(source, options.resource_limits.to_core())?;
    let startup_diagnostics = netlist
        .startup_diagnostics()
        .iter()
        .map(startup_diagnostic_summary)
        .collect();
    Ok(NetlistSummary {
        title: netlist.title,
        element_count: netlist.elements.len(),
        analysis_count: netlist.analyses.len(),
        model_count: netlist.models.len(),
        subcircuit_count: netlist.subcircuits.len(),
        parameter_count: netlist.params.all_params().len(),
        diagnostics: netlist.diagnostics.iter().map(diagnostic_summary).collect(),
        startup_diagnostics,
    })
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
    let resource_limits = options.resource_limits.to_core();
    let netlist = parse_netlist_detailed(source, resource_limits)?;
    let result = engine_with_resource_limits(resource_limits)?
        .run_dc_op(&netlist)
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
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

    let netlist = parse_netlist_detailed(source, resource_limits)?;
    let results = engine_with_resource_limits(resource_limits)?
        .run_ac(&netlist, frequencies)
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;

    Ok(results
        .into_iter()
        .map(|point| AcPointSnapshot {
            frequency: point.frequency,
            node_names: point.node_names,
            branch_names: point.branch_names,
            voltages: complex_series_from_slice(&point.voltages),
            currents: complex_series_from_slice(&point.currents),
        })
        .collect())
}

/// Backward-compatible string-error AC API.
pub fn run_ac_analysis(source: &str, frequencies: &[f64]) -> WasmResult<Vec<AcPointSnapshot>> {
    run_ac_analysis_detailed(source, frequencies).map_err(|error| error.message)
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
    let resource_limits = options.resource_limits.to_core();
    validate_transient_request(tstop, max_step, resource_limits)?;

    let netlist = parse_netlist_detailed(source, resource_limits)?;
    let result = engine_with_resource_limits(resource_limits)?
        .run_tran(&netlist, tstop, max_step)
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;

    transient_snapshot_from_result(result).map_err(|message| {
        Box::new(WasmError::new(
            message,
            "invalid_transient_result",
            "result_validation",
        ))
    })
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
    let resource_limits = options.resource_limits.to_core();
    validate_transient_request(tstop, max_step, resource_limits)?;
    let compression = compression.to_core()?;
    let netlist = parse_netlist_detailed(source, resource_limits)?;
    let result = engine_with_resource_limits(resource_limits)?
        .run_tran_compressed(&netlist, tstop, max_step, compression)
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;
    transient_snapshot_from_compressed_result(result).map_err(|message| {
        Box::new(WasmError::new(
            message,
            "invalid_transient_result",
            "result_validation",
        ))
    })
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

/// Exercise the configured browser parser-to-solver path without I/O.
pub fn health_check_with_options_detailed(
    options: &WasmExecutionOptions,
) -> DetailedWasmResult<WasmHealthReport> {
    let report = engine_with_resource_limits(options.resource_limits.to_core())?
        .health_check()
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
    let options = execution_options_from_js(options).map_err(|error| wasm_error_to_js(*error))?;
    let report =
        health_check_with_options_detailed(&options).map_err(|error| wasm_error_to_js(*error))?;
    serialize_to_js(&report)
}

#[wasm_bindgen(js_name = summarizeNetlist)]
pub fn summarize_netlist_js(source: &str, options: JsValue) -> Result<JsValue, JsValue> {
    let options = execution_options_from_js(options).map_err(|error| wasm_error_to_js(*error))?;
    let summary = summarize_netlist_with_options_detailed(source, &options)
        .map_err(|error| wasm_error_to_js(*error))?;
    serialize_to_js(&summary)
}

#[wasm_bindgen(js_name = runDcOperatingPoint)]
pub fn run_dc_operating_point_js(source: &str, options: JsValue) -> Result<JsValue, JsValue> {
    let options = execution_options_from_js(options).map_err(|error| wasm_error_to_js(*error))?;
    let result = run_dc_operating_point_with_options_detailed(source, &options)
        .map_err(|error| wasm_error_to_js(*error))?;
    serialize_to_js(&result)
}

#[wasm_bindgen(js_name = runAcAnalysis)]
pub fn run_ac_analysis_js(
    source: &str,
    frequencies: Vec<f64>,
    options: JsValue,
) -> Result<JsValue, JsValue> {
    let options = execution_options_from_js(options).map_err(|error| wasm_error_to_js(*error))?;
    let result = run_ac_analysis_with_options_detailed(source, &frequencies, &options)
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
    let options = execution_options_from_js(options).map_err(|error| wasm_error_to_js(*error))?;
    let result = run_transient_analysis_with_options_detailed(source, tstop, max_step, &options)
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
    let options = execution_options_from_js(options).map_err(|error| wasm_error_to_js(*error))?;
    let result = run_transient_analysis_compressed_with_options_detailed(
        source,
        tstop,
        max_step,
        &compression,
        &options,
    )
    .map_err(|error| wasm_error_to_js(*error))?;
    serialize_transient_to_js(&result)
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

    const FFT_PARITY_DECK: &str = "browser transient FFT parity\n\
        V1 out 0 SIN(0 1 1k)\n\
        R1 out 0 1k\n\
        .options fft fft_mode=1 fft_accurate=0 fftout=1\n\
        .tran 1u 1m\n\
        .fft v(out) np=128 format=unorm window=hann freq=1k fmin=1k fmax=10k\n\
        .fft {2*v(out)} np=64 format=norm window=rect\n\
        .end\n";

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
        assert_eq!(wasm.output_name, core.output_name);
        assert_eq!(wasm.physical_type, core.physical_type);
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
        assert_eq!(wasm.fft_results[0].output_name, "V(OUT)");
        assert_eq!(wasm.fft_results[1].output_name, "{2*v(out)}");
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
                input_points: 6,
                retained_points: 3,
                compression_ratio: 2.0,
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
                input_points: compressed_core.input_points,
                retained_points: compressed_core.time.len(),
                compression_ratio: compressed_core.compression_ratio,
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
            "source_kind",
            "source_text",
            "authored_output",
            "output_name",
            "physical_type",
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
        const COMPRESSION_FIELDS: &[&str] =
            &["input_points", "retained_points", "compression_ratio"];

        let (_, snapshot) = fft_parity_fixture();
        let encoded = serde_json::to_value(&snapshot).expect("serialize transient FFT DTO");
        assert_object_fields(&encoded, TRANSIENT_FIELDS);
        let first = &encoded["fft_results"][0];
        assert_object_fields(first, FFT_FIELDS);
        assert_object_fields(&first["bins"], BIN_FIELDS);
        assert_object_fields(&first["metrics"], METRIC_FIELDS);
        assert_object_fields(&first["metrics"]["largest_harmonics"], HARMONIC_FIELDS);

        let decoded: TransientSnapshot =
            serde_json::from_value(encoded).expect("deserialize transient FFT DTO");
        assert_eq!(decoded, snapshot);

        let mut without_metrics = snapshot;
        without_metrics.fft_results[0].metrics = None;
        let encoded = serde_json::to_value(without_metrics).expect("serialize absent metrics");
        assert!(encoded["fft_results"][0]["metrics"].is_null());

        let analog =
            transient_snapshot_from_compressed_result(synthetic_compressed_analog_result())
                .expect("compressed analog DTO adapts");
        let encoded = serde_json::to_value(&analog).expect("serialize complete analog DTO");
        assert_object_fields(&encoded["device_op_traces"][0], DEVICE_OP_FIELDS);
        assert_object_fields(&encoded["store_traces"][0], STORE_FIELDS);
        assert_object_fields(&encoded["compression"], COMPRESSION_FIELDS);
        assert!(encoded["voltages"][1].is_null());
        assert!(encoded["branch_currents"][1].is_null());
        let decoded: TransientSnapshot =
            serde_json::from_value(encoded).expect("deserialize complete analog DTO");
        assert_eq!(decoded, analog);
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
}
