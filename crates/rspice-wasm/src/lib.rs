//! WebAssembly wrapper for the RSpice simulation core.
//!
//! The crate keeps the browser-facing API intentionally thin: it exposes
//! serializable snapshots that mirror stable simulator concepts while delegating
//! all numerical work to `rspice-core`.

use rspice_core::{
    Engine, Netlist, ResourceKind, ResourceLimitError, ResourceLimits, SimulationConfig,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

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
    pub kind: String,
    pub category: String,
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
    kind: &'a str,
    category: &'a str,
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
    pub node_names: Vec<String>,
    pub voltages: Vec<Vec<f64>>,
}

impl WasmError {
    fn new(message: String, kind: &str, category: &str) -> Self {
        Self {
            message,
            kind: kind.to_string(),
            category: category.to_string(),
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
        use rspice_core::engine::{SimulationConfigError, SimulationError};

        let message = error.to_string();
        match error {
            SimulationError::Configuration(SimulationConfigError::ResourceLimit(error))
            | SimulationError::ResourceLimit(error) => Self::resource_limit(message, error),
            SimulationError::Configuration(_) => {
                Self::new(message, "invalid_configuration", "configuration")
            }
            SimulationError::Circuit(_) => Self::new(message, "circuit_error", "simulation"),
            SimulationError::Solver(_) => Self::new(message, "solver_error", "solver"),
            SimulationError::Netlist(_) => Self::new(message, "netlist_error", "netlist"),
            SimulationError::ConvergenceFailed(iterations) => {
                let mut structured = Self::new(message, "convergence_error", "convergence");
                structured.iterations = Some(iterations);
                structured
            }
            SimulationError::Aborted => Self::new(message, "aborted", "cancellation"),
        }
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
                    kind: "undefined_output_symbols".to_string(),
                    category: "output_symbol_validation".to_string(),
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
                    unresolved_output_symbols,
                }
            }
            rspice_core::netlist::ParseError::StartupDirectiveConflict(error) => Self {
                message,
                kind: "conflicting_startup_directives".to_string(),
                category: "startup_directive_validation".to_string(),
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
                unresolved_output_symbols: Vec::new(),
            },
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

fn serialize_to_js<T: Serialize>(value: &T) -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(value)
        .map_err(|err| JsValue::from_str(&format!("serialization failed: {err}")))
}

fn wasm_error_to_js(error: WasmError) -> JsValue {
    let js_error = js_sys::Error::new(&error.message);
    js_error.set_name("RSpiceError");
    let object: &JsValue = js_error.as_ref();

    let details = JsWasmErrorDetails {
        message: &error.message,
        kind: &error.kind,
        category: &error.category,
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
            "kind",
            "category",
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
pub fn run_transient_analysis_with_options_detailed(
    source: &str,
    tstop: f64,
    max_step: f64,
    options: &WasmExecutionOptions,
) -> DetailedWasmResult<TransientSnapshot> {
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
    let resource_limits = options.resource_limits.to_core();
    let estimated_points = (tstop / max_step).ceil() as usize;
    let estimated_points = estimated_points.saturating_add(1);
    if estimated_points > resource_limits.max_analysis_points {
        return Err(resource_limit_error(
            ResourceKind::AnalysisPoints,
            estimated_points,
            resource_limits.max_analysis_points,
        ));
    }

    let netlist = parse_netlist_detailed(source, resource_limits)?;
    let result = engine_with_resource_limits(resource_limits)?
        .run_tran(&netlist, tstop, max_step)
        .map_err(|error| Box::new(WasmError::from_simulation_error(error)))?;

    Ok(TransientSnapshot {
        time: result.time,
        node_names: result.node_names,
        voltages: result.voltages,
    })
}

/// Backward-compatible string-error transient API.
pub fn run_transient_analysis(
    source: &str,
    tstop: f64,
    max_step: f64,
) -> WasmResult<TransientSnapshot> {
    run_transient_analysis_detailed(source, tstop, max_step).map_err(|error| error.message)
}

#[wasm_bindgen(js_name = defaultResourceLimits)]
pub fn default_resource_limits_js() -> Result<JsValue, JsValue> {
    serialize_to_js(&WasmResourceLimits::default())
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
    serialize_to_js(&result)
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn browser_resource_defaults_are_stricter_than_desktop_defaults() {
        let browser = WasmResourceLimits::default();
        let desktop = ResourceLimits::default();

        assert_eq!(browser.max_netlist_bytes, 8 * MEBIBYTE);
        assert_eq!(browser.max_analysis_points, 200_000);
        assert_eq!(browser.max_result_values, 2_000_000);
        assert!(browser.max_netlist_bytes < desktop.max_netlist_bytes);
        assert!(browser.max_analysis_points < desktop.max_analysis_points);
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
        assert_eq!(parse_error.category, "resource_limit");
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
    fn ac_input_validation_rejects_non_finite_and_negative_frequencies() {
        let source = "valid\nV1 1 0 1\nR1 1 0 1k\n.END\n";
        for frequencies in [[1.0, f64::NAN], [1.0, -1.0]] {
            let error = run_ac_analysis_detailed(source, &frequencies)
                .expect_err("invalid explicit frequency must fail at the boundary");
            assert_eq!(error.kind, "invalid_argument");
            assert_eq!(error.category, "input_validation");
        }
    }
}
