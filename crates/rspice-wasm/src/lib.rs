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

    fn divider_netlist() -> &'static str {
        "Resistive divider\n\
         V1 in 0 DC 10 AC 1\n\
         R1 in out 1k\n\
         R2 out 0 1k\n\
         .end"
    }

    fn rc_lowpass_netlist() -> &'static str {
        "RC lowpass\n\
         V1 in 0 AC 1\n\
         R1 in out 1k\n\
         C1 out 0 1u\n\
         .end"
    }

    fn rc_step_netlist() -> &'static str {
        "RC transient\n\
         V1 in 0 PULSE(0 1 100u 1n 1n 10m 20m)\n\
         R1 in out 1k\n\
         C1 out 0 1u IC=0\n\
         .end"
    }

    fn node_voltage(result: &DcOperatingPoint, name: &str) -> f64 {
        let index = result
            .node_names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(name))
            .expect("missing node");
        result.node_voltages[index]
    }

    #[test]
    fn test_summarize_netlist_reports_structural_counts() {
        let source = "Summary test\n\
                      .param gain=2\n\
                      V1 in 0 DC 1\n\
                      R1 in out 1k\n\
                      .ac dec 10 1 1e3\n\
                      .end";

        let summary = summarize_netlist(source).expect("summary");
        assert_eq!(summary.title, "Summary test");
        assert_eq!(summary.element_count, 2);
        assert_eq!(summary.analysis_count, 1);
        assert_eq!(summary.parameter_count, 1);
    }

    #[test]
    fn test_run_dc_operating_point_matches_divider_solution() {
        let op = run_dc_operating_point(divider_netlist()).expect("dc op");
        assert!((node_voltage(&op, "in") - 10.0).abs() < 1e-9);
        assert!((node_voltage(&op, "out") - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_run_ac_analysis_returns_expected_lowpass_gain_shape() {
        let fc = 1.0 / (2.0 * std::f64::consts::PI * 1e3 * 1e-6);
        let sweep =
            run_ac_analysis(rc_lowpass_netlist(), &[1.0, fc, 10_000.0]).expect("ac analysis");
        assert_eq!(sweep.len(), 3);

        let gain = |point: &AcPointSnapshot| {
            let out_idx = point
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .expect("out node");
            let re = point.voltages.real[out_idx];
            let im = point.voltages.imag[out_idx];
            (re * re + im * im).sqrt()
        };

        let low = gain(&sweep[0]);
        let mid = gain(&sweep[1]);
        let high = gain(&sweep[2]);

        assert!(low > 0.99, "low-frequency gain should be near unity: {low}");
        assert!(
            (mid - std::f64::consts::FRAC_1_SQRT_2).abs() < 0.05,
            "cutoff gain should be near -3 dB: {mid}"
        );
        assert!(
            high < 0.05,
            "high-frequency gain should roll off strongly: {high}"
        );
    }

    #[test]
    fn test_run_ac_analysis_rejects_empty_frequency_list() {
        let error = run_ac_analysis(rc_lowpass_netlist(), &[]).expect_err("empty frequency list");
        assert!(error.contains("at least one frequency"));
    }

    #[test]
    fn test_run_transient_analysis_tracks_rc_step_response() {
        let transient =
            run_transient_analysis(rc_step_netlist(), 5e-3, 5e-5).expect("transient analysis");

        assert!(!transient.time.is_empty());
        assert_eq!(transient.node_names.len(), transient.voltages.len());
        assert!(transient.time.windows(2).all(|pair| pair[1] >= pair[0]));

        let out_idx = transient
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .expect("out node");
        let out = &transient.voltages[out_idx];
        let initial = out.first().copied().unwrap_or_default();
        let final_value = out.last().copied().unwrap_or_default();
        assert!(
            initial.abs() < 1e-3,
            "output should start near 0 V: {initial}"
        );
        assert!(
            (final_value - 0.992).abs() < 0.03,
            "output should settle near the analytic RC step value: {final_value}"
        );
    }

    #[test]
    fn test_run_transient_analysis_rejects_invalid_arguments() {
        let err = run_transient_analysis(rc_step_netlist(), -1.0, 1e-6).expect_err("bad tstop");
        assert!(err.contains("stop time"));

        let err = run_transient_analysis(rc_step_netlist(), 1e-3, 0.0).expect_err("bad step");
        assert!(err.contains("maximum step"));
    }
}
