//! WebAssembly wrapper for the RSpice simulation core.
//!
//! The crate keeps the browser-facing API intentionally thin: it exposes
//! serializable snapshots that mirror stable simulator concepts while delegating
//! all numerical work to `rspice-core`.

use rspice_core::{Engine, Netlist};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

type WasmResult<T> = Result<T, String>;
type DetailedWasmResult<T> = Result<T, Box<WasmError>>;

const MAX_TRANSIENT_POINTS: f64 = 200_000.0;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetlistSummary {
    pub title: String,
    pub element_count: usize,
    pub analysis_count: usize,
    pub model_count: usize,
    pub subcircuit_count: usize,
    pub parameter_count: usize,
    pub diagnostics: Vec<WasmDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmDiagnostic {
    pub line: usize,
    pub severity: String,
    pub code: String,
    pub message: String,
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
            unresolved_output_symbols: Vec::new(),
        }
    }

    fn invalid_argument(message: String) -> Self {
        Self::new(message, "invalid_argument", "input_validation")
    }

    fn simulation(message: String) -> Self {
        Self::new(message, "simulation_error", "simulation")
    }

    fn from_parse_error(error: rspice_core::netlist::ParseError) -> Self {
        let message = error.to_string();
        let rspice_core::netlist::ParseError::OutputSymbolValidation(error) = error else {
            return Self::new(message, "parse_error", "netlist_parse");
        };

        let unresolved_output_symbols = error
            .unresolved
            .iter()
            .map(|item| WasmUnresolvedOutputSymbol {
                directive: output_directive_name(item.directive).to_string(),
                source: item
                    .origin
                    .path
                    .as_ref()
                    .map(|path| path.to_string_lossy().into_owned()),
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
            primary_source: primary.and_then(|origin| {
                origin
                    .path
                    .as_ref()
                    .map(|path| path.to_string_lossy().into_owned())
            }),
            primary_line: primary.map(|origin| origin.line),
            unresolved_output_symbols,
        }
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

fn parse_netlist_detailed(source: &str) -> DetailedWasmResult<Netlist> {
    Netlist::parse_validated(source).map_err(|error| Box::new(WasmError::from_parse_error(error)))
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

fn complex_series_from_slice(values: &[rspice_core::Complex64]) -> ComplexSeries {
    ComplexSeries {
        real: values.iter().map(|value| value.re).collect(),
        imag: values.iter().map(|value| value.im).collect(),
    }
}

/// Summarize and semantically validate a netlist, returning typed diagnostics.
pub fn summarize_netlist_detailed(source: &str) -> DetailedWasmResult<NetlistSummary> {
    let netlist = parse_netlist_detailed(source)?;
    Ok(NetlistSummary {
        title: netlist.title,
        element_count: netlist.elements.len(),
        analysis_count: netlist.analyses.len(),
        model_count: netlist.models.len(),
        subcircuit_count: netlist.subcircuits.len(),
        parameter_count: netlist.params.all_params().len(),
        diagnostics: netlist.diagnostics.iter().map(diagnostic_summary).collect(),
    })
}

/// Backward-compatible string-error summary API.
pub fn summarize_netlist(source: &str) -> WasmResult<NetlistSummary> {
    summarize_netlist_detailed(source).map_err(|error| error.message)
}

/// Run an operating point after strict semantic validation.
pub fn run_dc_operating_point_detailed(source: &str) -> DetailedWasmResult<DcOperatingPoint> {
    let netlist = parse_netlist_detailed(source)?;
    let result = Engine::default()
        .run_dc_op(&netlist)
        .map_err(|error| Box::new(WasmError::simulation(error.to_string())))?;
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
    if frequencies.is_empty() {
        return Err(Box::new(WasmError::invalid_argument(
            "AC analysis requires at least one frequency".to_string(),
        )));
    }

    let netlist = parse_netlist_detailed(source)?;
    let results = Engine::default()
        .run_ac(&netlist, frequencies)
        .map_err(|error| Box::new(WasmError::simulation(error.to_string())))?;

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
    let estimated_points = (tstop / max_step).ceil() + 1.0;
    if !estimated_points.is_finite() || estimated_points > MAX_TRANSIENT_POINTS {
        return Err(Box::new(WasmError::invalid_argument(format!(
            "Transient request would generate more than {:.0} points; increase max_step or reduce tstop",
            MAX_TRANSIENT_POINTS
        ))));
    }

    let netlist = parse_netlist_detailed(source)?;
    let result = Engine::default()
        .run_tran(&netlist, tstop, max_step)
        .map_err(|error| Box::new(WasmError::simulation(error.to_string())))?;

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

#[wasm_bindgen(js_name = summarizeNetlist)]
pub fn summarize_netlist_js(source: &str) -> Result<JsValue, JsValue> {
    let summary = summarize_netlist_detailed(source).map_err(|error| wasm_error_to_js(*error))?;
    serialize_to_js(&summary)
}

#[wasm_bindgen(js_name = runDcOperatingPoint)]
pub fn run_dc_operating_point_js(source: &str) -> Result<JsValue, JsValue> {
    let result =
        run_dc_operating_point_detailed(source).map_err(|error| wasm_error_to_js(*error))?;
    serialize_to_js(&result)
}

#[wasm_bindgen(js_name = runAcAnalysis)]
pub fn run_ac_analysis_js(source: &str, frequencies: Vec<f64>) -> Result<JsValue, JsValue> {
    let result =
        run_ac_analysis_detailed(source, &frequencies).map_err(|error| wasm_error_to_js(*error))?;
    serialize_to_js(&result)
}

#[wasm_bindgen(js_name = runTransientAnalysis)]
pub fn run_transient_analysis_js(
    source: &str,
    tstop: f64,
    max_step: f64,
) -> Result<JsValue, JsValue> {
    let result = run_transient_analysis_detailed(source, tstop, max_step)
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
}
