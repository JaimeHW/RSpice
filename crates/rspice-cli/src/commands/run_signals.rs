use std::collections::HashMap;

use rspice_core::{
    Complex64, Value, analysis::AcResult, engine::TransientResult, netlist::SaveSet,
    solver::SimulationResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SignalKind {
    Voltage,
    Current,
}

impl SignalKind {
    pub(crate) fn raw_variable_type(self) -> &'static str {
        match self {
            Self::Voltage => "voltage",
            Self::Current => "current",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ScalarSignal {
    pub(crate) display_name: String,
    pub(crate) raw_name: String,
    pub(crate) kind: SignalKind,
    pub(crate) values: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ComplexSignal {
    pub(crate) display_name: String,
    pub(crate) raw_name: String,
    pub(crate) kind: SignalKind,
    pub(crate) real: Vec<Value>,
    pub(crate) imag: Vec<Value>,
}

fn unwrap_signal_name(name: &str, prefix: char) -> Option<&str> {
    let trimmed = name.trim();
    let mut chars = trimmed.chars();
    let first = chars.next()?;
    if !first.eq_ignore_ascii_case(&prefix) || chars.next()? != '(' || !trimmed.ends_with(')') {
        return None;
    }
    Some(&trimmed[2..trimmed.len() - 1])
}

fn canonical_signal_name(name: &str, fallback_index: usize, prefix: char) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return fallback_index.to_string();
    }

    let candidate = unwrap_signal_name(trimmed, prefix)
        .unwrap_or(trimmed)
        .trim();
    if candidate.is_empty() {
        fallback_index.to_string()
    } else {
        candidate.to_string()
    }
}

pub(crate) fn voltage_raw_name(name: &str, fallback_index: usize) -> String {
    canonical_signal_name(name, fallback_index, 'V')
}

pub(crate) fn voltage_display_name(name: &str, fallback_index: usize) -> String {
    format!("V({})", voltage_raw_name(name, fallback_index))
}

pub(crate) fn current_raw_name(name: &str, fallback_index: usize) -> String {
    canonical_signal_name(name, fallback_index, 'I')
}

pub(crate) fn current_display_name(name: &str, fallback_index: usize) -> String {
    format!("I({})", current_raw_name(name, fallback_index))
}

/// Restrict scalar signals to a netlist's `.save`/`.probe`/`.print` selection.
///
/// An empty selection (or one containing `all`) keeps every signal. Matching
/// runs against the display name (`V(out)` / `I(v1)`), which follows raw-file
/// conventions.
pub(crate) fn apply_save_set(signals: Vec<ScalarSignal>, saves: &SaveSet) -> Vec<ScalarSignal> {
    if saves.keeps_everything() {
        return signals;
    }
    signals
        .into_iter()
        .filter(|signal| saves.selects(&signal.display_name))
        .collect()
}

/// Restrict complex (AC) signals to a netlist's output selection.
pub(crate) fn apply_save_set_complex(
    signals: Vec<ComplexSignal>,
    saves: &SaveSet,
) -> Vec<ComplexSignal> {
    if saves.keeps_everything() {
        return signals;
    }
    signals
        .into_iter()
        .filter(|signal| saves.selects(&signal.display_name))
        .collect()
}

fn voltage_signal(raw_name: String, values: Vec<Value>) -> ScalarSignal {
    ScalarSignal {
        display_name: format!("V({raw_name})"),
        raw_name,
        kind: SignalKind::Voltage,
        values,
    }
}

fn current_signal(raw_name: String, values: Vec<Value>) -> ScalarSignal {
    ScalarSignal {
        display_name: current_display_name(&raw_name, 1),
        raw_name,
        kind: SignalKind::Current,
        values,
    }
}

pub(crate) fn transient_voltage_signals(result: &TransientResult) -> Vec<ScalarSignal> {
    result
        .voltages
        .iter()
        .enumerate()
        .map(|(index, waveform)| {
            let raw_name = result.node_names.get(index).map_or_else(
                || (index + 1).to_string(),
                |name| voltage_raw_name(name, index + 1),
            );
            voltage_signal(raw_name, waveform.clone())
        })
        .collect()
}

pub(crate) fn dc_operating_point_voltage_signals(result: &SimulationResult) -> Vec<ScalarSignal> {
    (1..result.node_voltages.len())
        .map(|node_id| {
            let raw_name = result.node_names.get(node_id).map_or_else(
                || node_id.to_string(),
                |name| voltage_raw_name(name, node_id),
            );
            voltage_signal(raw_name, vec![result.voltage(node_id)])
        })
        .collect()
}

pub(crate) fn dc_operating_point_current_signals(result: &SimulationResult) -> Vec<ScalarSignal> {
    result
        .branch_currents
        .iter()
        .enumerate()
        .map(|(index, current)| {
            let raw_name = result.branch_names.get(index).map_or_else(
                || (index + 1).to_string(),
                |name| current_raw_name(name, index + 1),
            );
            current_signal(raw_name, vec![*current])
        })
        .collect()
}

pub(crate) fn dc_operating_point_signals(result: &SimulationResult) -> Vec<ScalarSignal> {
    let mut signals = dc_operating_point_voltage_signals(result);
    signals.extend(dc_operating_point_current_signals(result));
    signals
}

pub(crate) fn dc_sweep_voltage_signals(results: &[(Value, SimulationResult)]) -> Vec<ScalarSignal> {
    let Some((_, first_result)) = results.first() else {
        return Vec::new();
    };

    (1..first_result.node_voltages.len())
        .map(|node_id| {
            let raw_name = first_result.node_names.get(node_id).map_or_else(
                || node_id.to_string(),
                |name| voltage_raw_name(name, node_id),
            );
            let values = results
                .iter()
                .map(|(_, result)| result.voltage(node_id))
                .collect();
            voltage_signal(raw_name, values)
        })
        .collect()
}

fn split_complex(values: impl Iterator<Item = Complex64>) -> (Vec<Value>, Vec<Value>) {
    let mut real = Vec::new();
    let mut imag = Vec::new();
    for value in values {
        real.push(value.re);
        imag.push(value.im);
    }
    (real, imag)
}

pub(crate) fn ac_signals(results: &[AcResult]) -> Vec<ComplexSignal> {
    let Some(first_result) = results.first() else {
        return Vec::new();
    };

    let mut signals = Vec::new();

    for node_idx in 0..first_result.voltages.len() {
        let raw_name = first_result.node_names.get(node_idx).map_or_else(
            || (node_idx + 1).to_string(),
            |name| voltage_raw_name(name, node_idx + 1),
        );
        let (real, imag) = split_complex(
            results
                .iter()
                .map(|result| result.voltages.get(node_idx).copied().unwrap_or_default()),
        );
        signals.push(ComplexSignal {
            display_name: format!("V({raw_name})"),
            raw_name,
            kind: SignalKind::Voltage,
            real,
            imag,
        });
    }

    for branch_idx in 0..first_result.currents.len() {
        let raw_name = first_result.branch_names.get(branch_idx).map_or_else(
            || (branch_idx + 1).to_string(),
            |name| current_raw_name(name, branch_idx + 1),
        );
        let (real, imag) = split_complex(
            results
                .iter()
                .map(|result| result.currents.get(branch_idx).copied().unwrap_or_default()),
        );
        signals.push(ComplexSignal {
            display_name: format!("I({raw_name})"),
            raw_name,
            kind: SignalKind::Current,
            real,
            imag,
        });
    }

    signals
}

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

fn insert_wrapped_voltage_variants<'a>(
    signals: &mut HashMap<String, &'a [Value]>,
    raw_name: &str,
    waveform: &'a [Value],
) {
    if raw_name.is_empty() {
        return;
    }

    for key in [
        format!("V({raw_name})"),
        format!("V({})", raw_name.to_ascii_lowercase()),
        format!("V({})", raw_name.to_ascii_uppercase()),
        format!("v({raw_name})"),
        format!("v({})", raw_name.to_ascii_lowercase()),
        format!("v({})", raw_name.to_ascii_uppercase()),
    ] {
        signals.insert(key, waveform);
    }
}

pub(crate) fn insert_transient_measurement_aliases<'a>(
    signals: &mut HashMap<String, &'a [Value]>,
    result: &'a TransientResult,
) {
    for (index, waveform) in result.voltages.iter().enumerate() {
        let fallback_index = index + 1;
        let raw_name = result.node_names.get(index).map_or_else(
            || fallback_index.to_string(),
            |name| voltage_raw_name(name, fallback_index),
        );
        let display_name = format!("V({raw_name})");
        let numeric_display = format!("V({fallback_index})");
        let numeric_raw = fallback_index.to_string();

        insert_wrapped_voltage_variants(signals, &raw_name, waveform.as_slice());
        insert_case_variants(signals, &raw_name, waveform.as_slice());

        if numeric_display != display_name {
            insert_wrapped_voltage_variants(signals, &numeric_raw, waveform.as_slice());
        }
        if numeric_raw != raw_name {
            insert_case_variants(signals, &numeric_raw, waveform.as_slice());
        }
    }
}
