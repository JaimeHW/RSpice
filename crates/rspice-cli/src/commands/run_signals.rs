//! Turn engine results into named export signals.
//!
//! One place decides how a node or branch becomes a column: the rawfile name
//! (`v(out)`, `i(v1)`) and the display name, the differential `V(a,b)` series
//! a `.SAVE`/`--save` request synthesizes, digital XSPICE states as numeric
//! values, and the `SaveSet` filter that drops everything not requested.
//! Every analysis exporter goes through here so a signal is named identically
//! whichever analysis produced it.

use rspice_core::{
    Complex64, Value, analysis::AcResult, engine::TransientResult, netlist::SaveSet,
    netlist::SaveSignal, solver::SimulationResult, xspice::DigitalValue,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SignalKind {
    Voltage,
    Current,
    Digital,
    Scalar,
}

impl SignalKind {
    pub(crate) fn raw_variable_type(self) -> &'static str {
        match self {
            Self::Voltage => "voltage",
            Self::Current => "current",
            Self::Digital => "digital",
            Self::Scalar => "parameter",
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
        .filter(|signal| complex_signal_is_selected(signal, saves))
        .collect()
}

fn complex_signal_is_selected(signal: &ComplexSignal, saves: &SaveSet) -> bool {
    if saves.selects(&signal.display_name) {
        return true;
    }

    saves.signals.iter().any(|saved| {
        let SaveSignal::Raw(authored) = saved else {
            return false;
        };
        let compact = authored
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect::<String>();
        let Some((operator, argument)) = compact
            .split_once('(')
            .and_then(|(operator, rest)| rest.strip_suffix(')').map(|arg| (operator, arg)))
        else {
            return false;
        };
        let operator = operator.to_ascii_uppercase();
        let compatible_kind = match signal.kind {
            SignalKind::Voltage => {
                matches!(operator.as_str(), "V" | "VR" | "VI" | "VM" | "VP" | "VDB")
            }
            SignalKind::Current => {
                matches!(operator.as_str(), "I" | "IR" | "II" | "IM" | "IP" | "IDB")
            }
            SignalKind::Digital | SignalKind::Scalar => false,
        };
        compatible_kind && argument.eq_ignore_ascii_case(&signal.raw_name)
    })
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
    // Parsed selections and CLI overrides are normalized through the
    // netlist's GroundPolicy before any export path reaches this module.
    name.trim() == "0"
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
        .filter(|(_, waveform)| result.time.is_empty() || !waveform.is_empty())
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
        .filter(|(_, waveform)| result.time.is_empty() || !waveform.is_empty())
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

fn authored_print_requests(
    netlist: &rspice_core::Netlist,
    analysis: rspice_core::netlist::OutputAnalysisKind,
) -> Vec<&rspice_core::netlist::OutputRequest> {
    netlist
        .output_requests
        .iter()
        .filter(|request| {
            request.directive == rspice_core::netlist::OutputDirectiveKind::Print
                && request.analysis.is_none_or(|kind| kind == analysis)
        })
        .collect()
}

fn projected_output_signals(
    projected: Vec<(String, &'static str, Vec<Value>)>,
) -> Vec<ScalarSignal> {
    projected
        .into_iter()
        .map(|(name, physical_type, values)| ScalarSignal {
            display_name: name.clone(),
            raw_name: name,
            kind: match physical_type {
                "voltage" => SignalKind::Voltage,
                "current" => SignalKind::Current,
                "parameter" => SignalKind::Scalar,
                unexpected => {
                    unreachable!("core returned unsupported physical output type '{unexpected}'")
                }
            },
            values,
        })
        .collect()
}

pub(crate) fn dc_export_signals(
    netlist: &rspice_core::Netlist,
    results: &[(Value, SimulationResult)],
    limits: rspice_core::ResourceLimits,
    abort: &dyn rspice_core::AbortSignal,
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    let requests = authored_print_requests(netlist, rspice_core::netlist::OutputAnalysisKind::Dc);
    if requests.is_empty()
        || requests
            .iter()
            .flat_map(|request| &request.operands)
            .any(|operand| {
                matches!(
                    rspice_core::netlist::parse_save_probe(operand),
                    Some(rspice_core::netlist::SaveSignal::All)
                )
            })
    {
        return Ok(apply_save_set(dc_sweep_signals(results), &netlist.saves));
    }
    rspice_core::analysis::evaluate_dc_output_requests_with_abort(netlist, results, limits, abort)
        .map(projected_output_signals)
}

pub(crate) fn transient_export_signals(
    netlist: &rspice_core::Netlist,
    result: &TransientResult,
    limits: rspice_core::ResourceLimits,
    abort: &dyn rspice_core::AbortSignal,
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    let requests = authored_print_requests(netlist, rspice_core::netlist::OutputAnalysisKind::Tran);
    if requests.is_empty()
        || requests
            .iter()
            .flat_map(|request| &request.operands)
            .any(|operand| {
                matches!(
                    rspice_core::netlist::parse_save_probe(operand),
                    Some(rspice_core::netlist::SaveSignal::All)
                )
            })
    {
        return Ok(apply_save_set(transient_signals(result), &netlist.saves));
    }
    rspice_core::analysis::evaluate_tran_output_requests_with_abort(netlist, result, limits, abort)
        .map(projected_output_signals)
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
