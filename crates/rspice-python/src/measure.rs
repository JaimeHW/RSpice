//! .MEAS evaluation against simulation results.
//!
//! Builds the signal-name table the core `MeasureEngine` expects and runs the
//! netlist's measurement statements against transient or DC-sweep data.
//!
//! Signal keys mirror the CLI's conventions (`V(out)`, `v(out)`, raw node
//! names, numeric node ids) and additionally expose branch currents as
//! `I(name)` so current-based measurements work from Python.

use std::collections::HashMap;

use rspice_core::engine::TransientResult;
use rspice_core::solver::SimulationResult;
use rspice_core::{MeasureEngine, Netlist};

use crate::results::PyMeasurement;

/// Insert `key` plus its lower/upper-case spellings.
fn insert_case_variants<'a>(
    signals: &mut HashMap<String, &'a [f64]>,
    key: &str,
    waveform: &'a [f64],
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
    signals: &mut HashMap<String, &'a [f64]>,
    prefix: char,
    raw: &str,
    waveform: &'a [f64],
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

/// Build the signal table for a transient result.
///
/// Voltages are reachable as `V(name)`, `name`, `V(id)`, and `id`; branch
/// currents as `I(name)`.
fn transient_signal_map(result: &TransientResult) -> HashMap<String, &[f64]> {
    let mut signals: HashMap<String, &[f64]> = HashMap::new();

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

/// Evaluate the netlist's transient .MEAS statements against a result.
pub(crate) fn evaluate_tran_measurements(
    netlist: &Netlist,
    result: &TransientResult,
) -> Vec<PyMeasurement> {
    let statements: Vec<_> = netlist
        .measurements
        .iter()
        .filter(|m| m.analysis.eq_ignore_ascii_case("TRAN"))
        .cloned()
        .collect();
    if statements.is_empty() {
        return Vec::new();
    }

    let signals = transient_signal_map(result);
    let mut engine = MeasureEngine::new();
    for statement in &statements {
        engine.add(statement.clone());
    }
    engine
        .evaluate(&result.time, &signals)
        .iter()
        .map(|r| PyMeasurement::from_core(r, "TRAN"))
        .collect()
}

/// Evaluate the netlist's DC .MEAS statements against a sweep.
///
/// The sweep axis plays the role of "time"; signals are node voltages and
/// branch currents as series across the sweep points.
pub(crate) fn evaluate_dc_measurements(
    netlist: &Netlist,
    sweep: &[(f64, SimulationResult)],
) -> Vec<PyMeasurement> {
    let statements: Vec<_> = netlist
        .measurements
        .iter()
        .filter(|m| m.analysis.eq_ignore_ascii_case("DC"))
        .cloned()
        .collect();
    if statements.is_empty() {
        return Vec::new();
    }
    let Some((_, first)) = sweep.first() else {
        return statements
            .iter()
            .map(|m| PyMeasurement::unevaluated(&m.name, "DC", "DC sweep produced no points"))
            .collect();
    };

    let axis: Vec<f64> = sweep.iter().map(|(v, _)| *v).collect();

    // Owned series storage: (raw name, prefix, series).
    let mut series_storage: Vec<(String, char, Vec<f64>)> = Vec::new();
    for node in 1..first.node_voltages.len() {
        let series: Vec<f64> = sweep
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
        series_storage.push((raw.clone(), 'V', series.clone()));
        if raw != fallback {
            series_storage.push((fallback, 'V', series));
        }
    }
    for (branch, name) in first.branch_names.iter().enumerate() {
        if name.is_empty() {
            continue;
        }
        let series: Vec<f64> = sweep
            .iter()
            .map(|(_, r)| r.branch_currents.get(branch).copied().unwrap_or(0.0))
            .collect();
        series_storage.push((name.clone(), 'I', series));
    }

    let mut signals: HashMap<String, &[f64]> = HashMap::new();
    // The sweep axis, so `FIND TIME WHEN ...` addresses the swept value.
    insert_case_variants(&mut signals, "Time", axis.as_slice());
    for (raw, prefix, series) in &series_storage {
        insert_wrapped_variants(&mut signals, *prefix, raw, series.as_slice());
        if *prefix == 'V' {
            insert_case_variants(&mut signals, raw, series.as_slice());
        }
    }

    let mut engine = MeasureEngine::new();
    for statement in &statements {
        engine.add(statement.clone());
    }
    engine
        .evaluate(&axis, &signals)
        .iter()
        .map(|r| PyMeasurement::from_core(r, "DC"))
        .collect()
}

/// Produce explicit not-evaluated entries for measurements whose analysis
/// did not run, so CI fails loudly instead of silently skipping checks.
pub(crate) fn unevaluated_measurements(
    netlist: &Netlist,
    analysis: &str,
    reason: &str,
) -> Vec<PyMeasurement> {
    netlist
        .measurements
        .iter()
        .filter(|m| m.analysis.eq_ignore_ascii_case(analysis))
        .map(|m| PyMeasurement::unevaluated(&m.name, analysis, reason))
        .collect()
}
