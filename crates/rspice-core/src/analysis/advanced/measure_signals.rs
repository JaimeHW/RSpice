//! Shared .MEAS evaluation harness over simulation results.
//!
//! Builds the signal-name table the [`MeasureEngine`](super::MeasureEngine)
//! expects and evaluates a netlist's measurement statements against
//! transient or DC-sweep data. Both the CLI and the Python bindings consume
//! this module so measurement semantics cannot drift between frontends.
//!
//! Signal naming: voltages are reachable as `V(name)`, `name`, `V(id)`, and
//! `id` in any case; branch currents as `I(name)`; the analysis axis as
//! `TIME` (the swept value plays that role for DC sweeps, so
//! `FIND TIME WHEN V(out)=...` addresses the sweep variable).

use std::collections::HashMap;

use super::measure::{MeasureEngine, MeasureResult, MeasureStatement};
use crate::Value;
use crate::analysis::AcResult;
use crate::engine::TransientResult;
use crate::netlist::Netlist;
use crate::solver::SimulationResult;

/// Insert `key` plus its lower/upper-case spellings.
fn insert_case_variants<'a>(
    signals: &mut HashMap<String, &'a [Value]>,
    key: &str,
    waveform: &'a [Value],
) {
    if key.is_empty() {
        return;
    }
    signals.insert(key.to_string(), waveform);
    let lower = key.to_ascii_lowercase();
    if lower != key {
        signals.insert(lower, waveform);
    }
    let upper = key.to_ascii_uppercase();
    if upper != key {
        signals.insert(upper, waveform);
    }
}

/// Insert `P(raw)` in every case combination of prefix and name.
fn insert_wrapped_variants<'a>(
    signals: &mut HashMap<String, &'a [Value]>,
    prefix: char,
    raw: &str,
    waveform: &'a [Value],
) {
    if raw.is_empty() {
        return;
    }
    let upper_prefix = prefix.to_ascii_uppercase();
    let lower_prefix = prefix.to_ascii_lowercase();
    for key in [
        format!("{upper_prefix}({raw})"),
        format!("{upper_prefix}({})", raw.to_ascii_lowercase()),
        format!("{upper_prefix}({})", raw.to_ascii_uppercase()),
        format!("{lower_prefix}({raw})"),
        format!("{lower_prefix}({})", raw.to_ascii_lowercase()),
        format!("{lower_prefix}({})", raw.to_ascii_uppercase()),
    ] {
        signals.insert(key, waveform);
    }
}

/// Build the measurement signal table for a transient result.
pub fn transient_signal_map(result: &TransientResult) -> HashMap<String, &[Value]> {
    let mut signals: HashMap<String, &[Value]> = HashMap::new();

    // The time axis itself, so `FIND TIME WHEN V(out)=...` works.
    insert_case_variants(&mut signals, "Time", result.time.as_slice());

    for (index, waveform) in result.voltages.iter().enumerate() {
        let fallback = (index + 1).to_string();
        let raw = result
            .node_names
            .get(index)
            .filter(|name| !name.is_empty())
            .cloned()
            .unwrap_or_else(|| fallback.clone());

        insert_wrapped_variants(&mut signals, 'V', &raw, waveform.as_slice());
        insert_case_variants(&mut signals, &raw, waveform.as_slice());
        if raw != fallback {
            insert_wrapped_variants(&mut signals, 'V', &fallback, waveform.as_slice());
            insert_case_variants(&mut signals, &fallback, waveform.as_slice());
        }
    }

    for (index, waveform) in result.branch_currents.iter().enumerate() {
        if let Some(name) = result.branch_names.get(index).filter(|n| !n.is_empty()) {
            insert_wrapped_variants(&mut signals, 'I', name, waveform.as_slice());
        }
    }

    signals
}

/// Owned per-signal series extracted from a DC sweep, from which a
/// measurement signal table can be borrowed.
pub struct DcSweepSeries {
    axis: Vec<Value>,
    /// (raw name, prefix, series)
    storage: Vec<(String, char, Vec<Value>)>,
}

impl DcSweepSeries {
    /// Collect node-voltage and branch-current series across the sweep.
    /// Returns `None` for an empty sweep.
    pub fn from_sweep(sweep: &[(Value, SimulationResult)]) -> Option<Self> {
        let (_, first) = sweep.first()?;
        let axis: Vec<Value> = sweep.iter().map(|(v, _)| *v).collect();

        let mut storage: Vec<(String, char, Vec<Value>)> = Vec::new();
        for node in 1..first.node_voltages.len() {
            let series: Vec<Value> = sweep
                .iter()
                .map(|(_, r)| r.node_voltages.get(node).copied().unwrap_or(0.0))
                .collect();
            let fallback = node.to_string();
            let raw = first
                .node_names
                .get(node)
                .filter(|name| !name.is_empty())
                .cloned()
                .unwrap_or_else(|| fallback.clone());
            storage.push((raw.clone(), 'V', series.clone()));
            if raw != fallback {
                storage.push((fallback, 'V', series));
            }
        }
        for (branch, name) in first.branch_names.iter().enumerate() {
            if name.is_empty() {
                continue;
            }
            let series: Vec<Value> = sweep
                .iter()
                .map(|(_, r)| r.branch_currents.get(branch).copied().unwrap_or(0.0))
                .collect();
            storage.push((name.clone(), 'I', series));
        }

        Some(Self { axis, storage })
    }

    /// The swept values, used as the measurement abscissa.
    pub fn axis(&self) -> &[Value] {
        &self.axis
    }

    /// Borrowed signal table over the collected series.
    pub fn signal_map(&self) -> HashMap<String, &[Value]> {
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        // The sweep axis, so `FIND TIME WHEN ...` addresses the swept value.
        insert_case_variants(&mut signals, "Time", self.axis.as_slice());
        for (raw, prefix, series) in &self.storage {
            insert_wrapped_variants(&mut signals, *prefix, raw, series.as_slice());
            if *prefix == 'V' {
                insert_case_variants(&mut signals, raw, series.as_slice());
            }
        }
        signals
    }
}

/// Owned per-signal series derived from an AC sweep, from which a
/// measurement signal table can be borrowed.
///
/// AC quantities are complex; measurements address the standard derived
/// real series. For a node `x`: `V(x)`/`VM(x)` magnitude, `VDB(x)`
/// 20·log10 magnitude, `VP(x)` phase in degrees, `VR(x)`/`VI(x)` real and
/// imaginary parts; branch currents use the `I` prefix the same way. The
/// frequency axis is reachable as `TIME` (the measurement abscissa),
/// `FREQUENCY`, and `FREQ`.
pub struct AcSweepSeries {
    axis: Vec<Value>,
    /// (full signal key, series)
    storage: Vec<(String, Vec<Value>)>,
}

impl AcSweepSeries {
    /// Collect the derived real series across the sweep. Returns `None`
    /// for an empty sweep.
    pub fn from_sweep(sweep: &[AcResult]) -> Option<Self> {
        let first = sweep.first()?;
        let axis: Vec<Value> = sweep.iter().map(|point| point.frequency).collect();

        let mut storage: Vec<(String, Vec<Value>)> = Vec::new();
        let mut push_complex_series =
            |prefix: char, raw: &str, values: Vec<crate::Complex64>| {
                let magnitude: Vec<Value> = values.iter().map(|c| c.norm()).collect();
                let db: Vec<Value> = magnitude
                    .iter()
                    .map(|m| {
                        if *m > 1e-30 {
                            20.0 * m.log10()
                        } else {
                            -600.0
                        }
                    })
                    .collect();
                let phase_deg: Vec<Value> =
                    values.iter().map(|c| c.arg().to_degrees()).collect();
                let real: Vec<Value> = values.iter().map(|c| c.re).collect();
                let imag: Vec<Value> = values.iter().map(|c| c.im).collect();

                storage.push((format!("{prefix}({raw})"), magnitude.clone()));
                storage.push((format!("{prefix}M({raw})"), magnitude));
                storage.push((format!("{prefix}DB({raw})"), db));
                storage.push((format!("{prefix}P({raw})"), phase_deg));
                storage.push((format!("{prefix}R({raw})"), real));
                storage.push((format!("{prefix}I({raw})"), imag));
            };

        for (index, name) in first.node_names.iter().enumerate() {
            let raw = if name.is_empty() {
                (index + 1).to_string()
            } else {
                name.clone()
            };
            let values: Vec<crate::Complex64> = sweep
                .iter()
                .map(|point| point.voltages.get(index).copied().unwrap_or_default())
                .collect();
            push_complex_series('V', &raw, values);
        }
        for (index, name) in first.branch_names.iter().enumerate() {
            if name.is_empty() {
                continue;
            }
            let values: Vec<crate::Complex64> = sweep
                .iter()
                .map(|point| point.currents.get(index).copied().unwrap_or_default())
                .collect();
            push_complex_series('I', name, values);
        }

        Some(Self { axis, storage })
    }

    /// The sweep frequencies, used as the measurement abscissa.
    pub fn axis(&self) -> &[Value] {
        &self.axis
    }

    /// Borrowed signal table over the collected series.
    pub fn signal_map(&self) -> HashMap<String, &[Value]> {
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        insert_case_variants(&mut signals, "Time", self.axis.as_slice());
        insert_case_variants(&mut signals, "Frequency", self.axis.as_slice());
        insert_case_variants(&mut signals, "Freq", self.axis.as_slice());
        for (key, series) in &self.storage {
            insert_case_variants(&mut signals, key, series.as_slice());
        }
        signals
    }
}

/// Owned series derived from a noise sweep: output and input-referred
/// spectral densities addressable as `ONOISE`/`INOISE` (also with the
/// `_SPECTRUM` suffix, matching the exported column names), with the
/// frequency axis as `TIME`/`FREQUENCY`/`FREQ`.
pub struct NoiseSweepSeries {
    axis: Vec<Value>,
    onoise: Vec<Value>,
    inoise: Vec<Value>,
}

impl NoiseSweepSeries {
    /// Collect spectral-density series across the sweep. Returns `None`
    /// for an empty sweep.
    pub fn from_sweep(sweep: &[crate::analysis::NoiseResult]) -> Option<Self> {
        if sweep.is_empty() {
            return None;
        }
        Some(Self {
            axis: sweep.iter().map(|point| point.frequency).collect(),
            onoise: sweep.iter().map(|point| point.output_noise_rms()).collect(),
            inoise: sweep
                .iter()
                .map(|point| point.input_referred_rms())
                .collect(),
        })
    }

    /// The sweep frequencies, used as the measurement abscissa.
    pub fn axis(&self) -> &[Value] {
        &self.axis
    }

    /// Borrowed signal table over the collected series.
    pub fn signal_map(&self) -> HashMap<String, &[Value]> {
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        insert_case_variants(&mut signals, "Time", self.axis.as_slice());
        insert_case_variants(&mut signals, "Frequency", self.axis.as_slice());
        insert_case_variants(&mut signals, "Freq", self.axis.as_slice());
        for key in ["Onoise", "Onoise_Spectrum"] {
            insert_case_variants(&mut signals, key, self.onoise.as_slice());
        }
        for key in ["Inoise", "Inoise_Spectrum"] {
            insert_case_variants(&mut signals, key, self.inoise.as_slice());
        }
        signals
    }
}

/// Evaluate the netlist's NOISE .MEAS statements against a sweep.
///
/// Returns an empty vector when the netlist has no NOISE measurements; an
/// empty sweep fails every statement explicitly rather than skipping it.
pub fn evaluate_noise_measurements(
    netlist: &Netlist,
    sweep: &[crate::analysis::NoiseResult],
) -> Vec<MeasureResult> {
    let statements = measurements_for_analysis(netlist, "NOISE");
    if statements.is_empty() {
        return Vec::new();
    }
    let Some(series) = NoiseSweepSeries::from_sweep(sweep) else {
        return statements
            .iter()
            .map(|m| MeasureResult::failed(&m.name, "noise sweep produced no points"))
            .collect();
    };
    let signals = series.signal_map();
    evaluate_statements(&statements, series.axis(), &signals)
}

/// The netlist's measurement statements for one analysis kind
/// (`"TRAN"`, `"DC"`, `"AC"`, ...).
pub fn measurements_for_analysis<'a>(
    netlist: &'a Netlist,
    analysis: &str,
) -> Vec<&'a MeasureStatement> {
    netlist
        .measurements
        .iter()
        .filter(|m| m.analysis.eq_ignore_ascii_case(analysis))
        .collect()
}

fn evaluate_statements(
    statements: &[&MeasureStatement],
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
) -> Vec<MeasureResult> {
    let mut engine = MeasureEngine::new();
    for statement in statements {
        engine.add((*statement).clone());
    }
    engine.evaluate(axis, signals)
}

/// Evaluate the netlist's transient .MEAS statements against a result.
///
/// Returns an empty vector when the netlist has no transient measurements.
pub fn evaluate_tran_measurements(
    netlist: &Netlist,
    result: &TransientResult,
) -> Vec<MeasureResult> {
    let statements = measurements_for_analysis(netlist, "TRAN");
    if statements.is_empty() {
        return Vec::new();
    }
    let signals = transient_signal_map(result);
    evaluate_statements(&statements, &result.time, &signals)
}

/// Evaluate the netlist's DC .MEAS statements against a sweep.
///
/// Returns an empty vector when the netlist has no DC measurements; an empty
/// sweep fails every statement explicitly rather than skipping it.
pub fn evaluate_dc_measurements(
    netlist: &Netlist,
    sweep: &[(Value, SimulationResult)],
) -> Vec<MeasureResult> {
    let statements = measurements_for_analysis(netlist, "DC");
    if statements.is_empty() {
        return Vec::new();
    }
    let Some(series) = DcSweepSeries::from_sweep(sweep) else {
        return statements
            .iter()
            .map(|m| MeasureResult::failed(&m.name, "DC sweep produced no points"))
            .collect();
    };
    let signals = series.signal_map();
    evaluate_statements(&statements, series.axis(), &signals)
}

/// Evaluate the netlist's AC .MEAS statements against a sweep.
///
/// Returns an empty vector when the netlist has no AC measurements; an
/// empty sweep fails every statement explicitly rather than skipping it.
/// Signal naming follows [`AcSweepSeries`]: magnitudes under `V(x)`/`VM(x)`,
/// `VDB`/`VP` (degrees)/`VR`/`VI` variants, and the frequency axis as
/// `TIME`/`FREQUENCY`/`FREQ`.
pub fn evaluate_ac_measurements(netlist: &Netlist, sweep: &[AcResult]) -> Vec<MeasureResult> {
    let statements = measurements_for_analysis(netlist, "AC");
    if statements.is_empty() {
        return Vec::new();
    }
    let Some(series) = AcSweepSeries::from_sweep(sweep) else {
        return statements
            .iter()
            .map(|m| MeasureResult::failed(&m.name, "AC sweep produced no points"))
            .collect();
    };
    let signals = series.signal_map();
    evaluate_statements(&statements, series.axis(), &signals)
}

/// Explicit not-evaluated entries for measurements whose analysis did not
/// run (or is not supported by the caller), so automation fails loudly
/// instead of silently skipping checks.
pub fn unevaluated_measurements(
    netlist: &Netlist,
    analysis: &str,
    reason: &str,
) -> Vec<MeasureResult> {
    measurements_for_analysis(netlist, analysis)
        .iter()
        .map(|m| MeasureResult::failed(&m.name, reason))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tran_result() -> TransientResult {
        TransientResult {
            time: vec![0.0, 1.0, 2.0, 3.0],
            voltages: vec![vec![0.0, 1.0, 2.0, 3.0]],
            branch_currents: vec![vec![0.0, -1.0, -2.0, -3.0]],
            num_nodes: 1,
            node_names: vec!["out".to_string()],
            branch_names: vec!["v1".to_string()],
        }
    }

    #[test]
    fn transient_map_exposes_time_voltage_and_current() {
        let result = tran_result();
        let signals = transient_signal_map(&result);
        assert!(signals.contains_key("TIME"));
        assert!(signals.contains_key("V(out)"));
        assert!(signals.contains_key("v(OUT)"));
        assert!(signals.contains_key("I(v1)"));
        assert_eq!(signals["TIME"], result.time.as_slice());
    }

    #[test]
    fn ac_series_exposes_derived_real_quantities() {
        let point = |freq: f64, voltage: crate::Complex64| AcResult {
            frequency: freq,
            node_names: vec!["out".to_string()],
            branch_names: vec![],
            voltages: vec![voltage],
            currents: vec![],
        };
        let sweep = vec![
            point(1.0, crate::Complex64::new(1.0, 0.0)),
            point(10.0, crate::Complex64::new(0.0, -1.0)),
        ];

        let series = AcSweepSeries::from_sweep(&sweep).expect("non-empty sweep");
        assert_eq!(series.axis(), &[1.0, 10.0]);
        let signals = series.signal_map();
        assert_eq!(signals["V(out)"], &[1.0, 1.0][..], "magnitude");
        assert_eq!(signals["VDB(out)"], &[0.0, 0.0][..], "decibels");
        assert_eq!(signals["VR(out)"], &[1.0, 0.0][..], "real part");
        assert_eq!(signals["VI(out)"], &[0.0, -1.0][..], "imaginary part");
        assert_eq!(signals["VP(out)"][1], -90.0, "phase in degrees");
        assert!(signals.contains_key("FREQUENCY"));
    }

    #[test]
    fn dc_series_uses_sweep_as_axis() {
        let mut point = SimulationResult::new(1, 0);
        point.node_voltages = vec![0.0, 2.5];
        point.node_names = vec!["0".to_string(), "out".to_string()];
        let sweep = vec![(0.0, point.clone()), (5.0, point)];

        let series = DcSweepSeries::from_sweep(&sweep).expect("non-empty sweep");
        assert_eq!(series.axis(), &[0.0, 5.0]);
        let signals = series.signal_map();
        assert!(signals.contains_key("TIME"));
        assert_eq!(signals["V(out)"], &[2.5, 2.5][..]);
    }
}
