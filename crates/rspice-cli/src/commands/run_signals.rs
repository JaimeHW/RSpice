use rspice_core::{
    Complex64, Value, analysis::AcResult, engine::TransientResult, netlist::SaveSet,
    netlist::SaveSignal, solver::SimulationResult, xspice::DigitalValue,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SignalKind {
    Voltage,
    Current,
    Digital,
}

impl SignalKind {
    pub(crate) fn raw_variable_type(self) -> &'static str {
        match self {
            Self::Voltage => "voltage",
            Self::Current => "current",
            Self::Digital => "digital",
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
    with_differential_voltage_signals(signals, saves)
        .into_iter()
        .filter(|signal| signal_is_selected(signal, saves))
        .collect()
}

fn signal_is_selected(signal: &ScalarSignal, saves: &SaveSet) -> bool {
    saves.selects(&signal.display_name)
        || (signal.kind == SignalKind::Digital && saves.selects_raw_name(&signal.raw_name))
}

/// Restrict complex (AC) signals to a netlist's output selection.
pub(crate) fn apply_save_set_complex(
    signals: Vec<ComplexSignal>,
    saves: &SaveSet,
) -> Vec<ComplexSignal> {
    if saves.keeps_everything() {
        return signals;
    }
    with_differential_complex_signals(signals, saves)
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

fn digital_signal(raw_name: String, values: Vec<Value>) -> ScalarSignal {
    ScalarSignal {
        display_name: format!("D({raw_name})"),
        raw_name,
        kind: SignalKind::Digital,
        values,
    }
}

fn digital_value_numeric(value: DigitalValue) -> Value {
    match value.to_bool() {
        Some(false) => 0.0,
        Some(true) => 1.0,
        None => 0.5,
    }
}

fn is_ground_node(name: &str) -> bool {
    matches!(name.trim().to_ascii_lowercase().as_str(), "0" | "gnd")
}

fn requested_voltage_diffs(saves: &SaveSet) -> impl Iterator<Item = (&str, &str)> {
    saves.signals.iter().filter_map(|signal| match signal {
        SaveSignal::VoltageDiff(a, b) => Some((a.as_str(), b.as_str())),
        _ => None,
    })
}

fn scalar_voltage_values(
    signals: &[ScalarSignal],
    node: &str,
    sample_count: usize,
) -> Option<Vec<Value>> {
    if is_ground_node(node) {
        return Some(vec![0.0; sample_count]);
    }

    let target = voltage_raw_name(node, 0);
    signals
        .iter()
        .find(|signal| {
            signal.kind == SignalKind::Voltage && signal.raw_name.eq_ignore_ascii_case(&target)
        })
        .map(|signal| signal.values.clone())
}

fn complex_voltage_values(
    signals: &[ComplexSignal],
    node: &str,
    sample_count: usize,
) -> Option<(Vec<Value>, Vec<Value>)> {
    if is_ground_node(node) {
        return Some((vec![0.0; sample_count], vec![0.0; sample_count]));
    }

    let target = voltage_raw_name(node, 0);
    signals
        .iter()
        .find(|signal| {
            signal.kind == SignalKind::Voltage && signal.raw_name.eq_ignore_ascii_case(&target)
        })
        .map(|signal| (signal.real.clone(), signal.imag.clone()))
}

fn with_differential_voltage_signals(
    mut signals: Vec<ScalarSignal>,
    saves: &SaveSet,
) -> Vec<ScalarSignal> {
    let sample_count = signals
        .iter()
        .find(|signal| signal.kind == SignalKind::Voltage)
        .map_or(0, |signal| signal.values.len());

    for (positive, negative) in requested_voltage_diffs(saves) {
        let raw_name = format!(
            "{},{}",
            voltage_raw_name(positive, 0),
            voltage_raw_name(negative, 0)
        );
        let display_name = format!("V({raw_name})");
        if signals
            .iter()
            .any(|signal| signal.display_name.eq_ignore_ascii_case(&display_name))
        {
            continue;
        }

        let Some(pos_values) = scalar_voltage_values(&signals, positive, sample_count) else {
            continue;
        };
        let Some(neg_values) = scalar_voltage_values(&signals, negative, sample_count) else {
            continue;
        };
        if pos_values.len() != neg_values.len() {
            continue;
        }

        let values = pos_values
            .into_iter()
            .zip(neg_values)
            .map(|(pos, neg)| pos - neg)
            .collect();
        signals.push(ScalarSignal {
            display_name,
            raw_name,
            kind: SignalKind::Voltage,
            values,
        });
    }

    signals
}

fn with_differential_complex_signals(
    mut signals: Vec<ComplexSignal>,
    saves: &SaveSet,
) -> Vec<ComplexSignal> {
    let sample_count = signals
        .iter()
        .find(|signal| signal.kind == SignalKind::Voltage)
        .map_or(0, |signal| signal.real.len());

    for (positive, negative) in requested_voltage_diffs(saves) {
        let raw_name = format!(
            "{},{}",
            voltage_raw_name(positive, 0),
            voltage_raw_name(negative, 0)
        );
        let display_name = format!("V({raw_name})");
        if signals
            .iter()
            .any(|signal| signal.display_name.eq_ignore_ascii_case(&display_name))
        {
            continue;
        }

        let Some((pos_real, pos_imag)) = complex_voltage_values(&signals, positive, sample_count)
        else {
            continue;
        };
        let Some((neg_real, neg_imag)) = complex_voltage_values(&signals, negative, sample_count)
        else {
            continue;
        };
        if pos_real.len() != neg_real.len() || pos_imag.len() != neg_imag.len() {
            continue;
        }

        let real = pos_real
            .into_iter()
            .zip(neg_real)
            .map(|(pos, neg)| pos - neg)
            .collect();
        let imag = pos_imag
            .into_iter()
            .zip(neg_imag)
            .map(|(pos, neg)| pos - neg)
            .collect();
        signals.push(ComplexSignal {
            display_name,
            raw_name,
            kind: SignalKind::Voltage,
            real,
            imag,
        });
    }

    signals
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

pub(crate) fn transient_current_signals(result: &TransientResult) -> Vec<ScalarSignal> {
    result
        .branch_currents
        .iter()
        .enumerate()
        .map(|(index, waveform)| {
            let raw_name = result.branch_names.get(index).map_or_else(
                || (index + 1).to_string(),
                |name| current_raw_name(name, index + 1),
            );
            current_signal(raw_name, waveform.clone())
        })
        .collect()
}

pub(crate) fn transient_digital_signals(result: &TransientResult) -> Vec<ScalarSignal> {
    const TIME_EPSILON: Value = 1.0e-18;

    result
        .digital_traces
        .iter()
        .map(|trace| {
            let mut values = Vec::with_capacity(result.time.len());
            let mut point_index = 0;
            let mut current = DigitalValue::default();
            for &time in &result.time {
                while let Some(point) = trace.points.get(point_index) {
                    if point.time <= time + TIME_EPSILON {
                        current = point.value;
                        point_index += 1;
                    } else {
                        break;
                    }
                }
                values.push(digital_value_numeric(current));
            }
            digital_signal(trace.node_name.clone(), values)
        })
        .collect()
}

pub(crate) fn transient_signals(result: &TransientResult) -> Vec<ScalarSignal> {
    let mut signals = transient_voltage_signals(result);
    signals.extend(transient_current_signals(result));
    signals.extend(transient_digital_signals(result));
    signals
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

pub(crate) fn dc_sweep_current_signals(results: &[(Value, SimulationResult)]) -> Vec<ScalarSignal> {
    let Some((_, first_result)) = results.first() else {
        return Vec::new();
    };

    first_result
        .branch_currents
        .iter()
        .enumerate()
        .map(|(branch_idx, _)| {
            let raw_name = first_result.branch_names.get(branch_idx).map_or_else(
                || (branch_idx + 1).to_string(),
                |name| current_raw_name(name, branch_idx + 1),
            );
            let values = results
                .iter()
                .map(|(_, result)| {
                    result
                        .branch_currents
                        .get(branch_idx)
                        .copied()
                        .unwrap_or_default()
                })
                .collect();
            current_signal(raw_name, values)
        })
        .collect()
}

pub(crate) fn dc_sweep_signals(results: &[(Value, SimulationResult)]) -> Vec<ScalarSignal> {
    let mut signals = dc_sweep_voltage_signals(results);
    signals.extend(dc_sweep_current_signals(results));
    signals
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
