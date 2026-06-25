//! WebAssembly wrapper for the RSpice simulation core.
//!
//! The crate keeps the browser-facing API intentionally thin: it exposes
//! serializable snapshots that mirror stable simulator concepts while delegating
//! all numerical work to `rspice-core`.

use rspice_core::{Engine, Netlist};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

type WasmResult<T> = Result<T, String>;

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

fn parse_netlist(source: &str) -> WasmResult<Netlist> {
    Netlist::parse(source).map_err(|err| err.to_string())
}

fn serialize_to_js<T: Serialize>(value: &T) -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(value)
        .map_err(|err| JsValue::from_str(&format!("serialization failed: {err}")))
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

pub fn summarize_netlist(source: &str) -> WasmResult<NetlistSummary> {
    let netlist = parse_netlist(source)?;
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

pub fn run_dc_operating_point(source: &str) -> WasmResult<DcOperatingPoint> {
    let netlist = parse_netlist(source)?;
    let result = Engine::default()
        .run_dc_op(&netlist)
        .map_err(|err| err.to_string())?;
    Ok(DcOperatingPoint {
        node_names: result.node_names,
        node_voltages: result.node_voltages,
        branch_names: result.branch_names,
        branch_currents: result.branch_currents,
    })
}

pub fn run_ac_analysis(source: &str, frequencies: &[f64]) -> WasmResult<Vec<AcPointSnapshot>> {
    if frequencies.is_empty() {
        return Err("AC analysis requires at least one frequency".to_string());
    }

    let netlist = parse_netlist(source)?;
    let results = Engine::default()
        .run_ac(&netlist, frequencies)
        .map_err(|err| err.to_string())?;

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

pub fn run_transient_analysis(
    source: &str,
    tstop: f64,
    max_step: f64,
) -> WasmResult<TransientSnapshot> {
    if !tstop.is_finite() || tstop <= 0.0 {
        return Err(format!(
            "Transient stop time must be positive and finite, got {tstop}"
        ));
    }
    if !max_step.is_finite() || max_step <= 0.0 {
        return Err(format!(
            "Transient maximum step must be positive and finite, got {max_step}"
        ));
    }

    let netlist = parse_netlist(source)?;
    let result = Engine::default()
        .run_tran(&netlist, tstop, max_step)
        .map_err(|err| err.to_string())?;

    Ok(TransientSnapshot {
        time: result.time,
        node_names: result.node_names,
        voltages: result.voltages,
    })
}

#[wasm_bindgen(js_name = summarizeNetlist)]
pub fn summarize_netlist_js(source: &str) -> Result<JsValue, JsValue> {
    let summary = summarize_netlist(source).map_err(|err| JsValue::from_str(&err))?;
    serialize_to_js(&summary)
}

#[wasm_bindgen(js_name = runDcOperatingPoint)]
pub fn run_dc_operating_point_js(source: &str) -> Result<JsValue, JsValue> {
    let result = run_dc_operating_point(source).map_err(|err| JsValue::from_str(&err))?;
    serialize_to_js(&result)
}

#[wasm_bindgen(js_name = runAcAnalysis)]
pub fn run_ac_analysis_js(source: &str, frequencies: Vec<f64>) -> Result<JsValue, JsValue> {
    let result = run_ac_analysis(source, &frequencies).map_err(|err| JsValue::from_str(&err))?;
    serialize_to_js(&result)
}

#[wasm_bindgen(js_name = runTransientAnalysis)]
pub fn run_transient_analysis_js(
    source: &str,
    tstop: f64,
    max_step: f64,
) -> Result<JsValue, JsValue> {
    let result =
        run_transient_analysis(source, tstop, max_step).map_err(|err| JsValue::from_str(&err))?;
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
