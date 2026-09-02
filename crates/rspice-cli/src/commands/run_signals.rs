//! Turn engine results into named export signals.
//!
//! One place decides how a node or branch becomes a column: the rawfile name
//! (`v(out)`, `i(v1)`) and the display name, the differential `V(a,b)` series
//! a `.SAVE`/`--save` request synthesizes, digital XSPICE states as numeric
//! values, and the `SaveSet` filter that drops everything not requested.
//! Every analysis exporter goes through here so a signal is named identically
//! whichever analysis produced it.

use rspice_core::{
    Value, analysis::AcResult, engine::TransientResult, netlist::SaveSet, netlist::SaveSignal,
    solver::SimulationResult, xspice::DigitalValue,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

/// Describe one already-projected scalar result without inspecting its
/// numeric values.  STEP preflight unions these descriptors across every
/// coordinate before any artifact is committed, so absent signals become
/// explicit validity bits instead of zeroes or first-coordinate columns.
pub(crate) fn scalar_signal_schema(
    signals: &[ScalarSignal],
) -> Result<rspice_core::execution::SignalSchema, rspice_core::execution::SignalSchemaError> {
    use rspice_core::execution::{
        SignalDescriptor, SignalKind as ExecutionSignalKind, SignalOwner, SignalShape, SignalUnit,
        SignalValueType,
    };

    let descriptors = signals
        .iter()
        .map(|signal| {
            let (kind, unit, owner) = match signal.kind {
                SignalKind::Voltage => (
                    ExecutionSignalKind::Voltage,
                    SignalUnit::Volt,
                    SignalOwner::Node(signal.raw_name.clone()),
                ),
                SignalKind::Current => (
                    ExecutionSignalKind::Current,
                    SignalUnit::Ampere,
                    SignalOwner::Branch(signal.raw_name.clone()),
                ),
                SignalKind::Digital => (
                    ExecutionSignalKind::Digital,
                    SignalUnit::Logic,
                    SignalOwner::Node(signal.raw_name.clone()),
                ),
                SignalKind::Scalar => scalar_signal_identity(signal),
            };
            let value_type = if kind == ExecutionSignalKind::Digital {
                SignalValueType::Logic
            } else {
                SignalValueType::Real
            };
            SignalDescriptor::new(
                signal.display_name.clone(),
                signal.display_name.clone(),
                kind,
                unit,
                value_type,
                SignalShape::Scalar,
                owner,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    rspice_core::execution::SignalSchema::new(descriptors)
}

fn scalar_signal_identity(
    signal: &ScalarSignal,
) -> (
    rspice_core::execution::SignalKind,
    rspice_core::execution::SignalUnit,
    rspice_core::execution::SignalOwner,
) {
    use rspice_core::execution::{SignalKind, SignalOwner, SignalUnit};

    let device = signal
        .raw_name
        .strip_prefix('@')
        .and_then(|name| name.split_once('['))
        .map(|(device, _)| device.trim())
        .filter(|device| !device.is_empty());
    match device {
        Some(device) => (
            SignalKind::DeviceObservable,
            SignalUnit::Dimensionless,
            SignalOwner::Device(device.to_string()),
        ),
        None => (
            SignalKind::Scalar,
            SignalUnit::Dimensionless,
            SignalOwner::Analysis,
        ),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DeviceParamRequest {
    device: String,
    param: String,
    authored_symbol: String,
}

fn requested_device_params(saves: &SaveSet) -> Vec<DeviceParamRequest> {
    let mut seen = std::collections::BTreeSet::new();
    let mut requests = Vec::new();
    for signal in &saves.signals {
        let SaveSignal::DeviceParam { device, param } = signal else {
            continue;
        };
        let identity = (device.to_ascii_lowercase(), param.to_ascii_lowercase());
        if seen.insert(identity) {
            requests.push(DeviceParamRequest {
                device: device.clone(),
                param: param.clone(),
                authored_symbol: format!("@{device}[{param}]"),
            });
        }
    }
    requests
}

fn requested_signal_unavailable(
    request: &DeviceParamRequest,
    analysis: &str,
    coordinate: Option<String>,
) -> rspice_core::SimulationError {
    rspice_core::SimulationError::requested_signal_unavailable(
        request.authored_symbol.clone(),
        analysis,
        coordinate,
    )
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

fn authored_save_symbol(signal: &SaveSignal) -> Option<String> {
    match signal {
        SaveSignal::All => None,
        SaveSignal::Voltage(node) => Some(format!("V({node})")),
        SaveSignal::VoltageDiff(positive, negative) => Some(format!("V({positive},{negative})")),
        SaveSignal::Current(device) => Some(format!("I({device})")),
        SaveSignal::DeviceParam { device, param } => Some(format!("@{device}[{param}]")),
        SaveSignal::Raw(raw) => Some(raw.clone()),
    }
}

fn single_save_set(signal: &SaveSignal) -> SaveSet {
    SaveSet {
        signals: vec![signal.clone()],
    }
}

/// Apply scalar output selection and prove that every independently authored
/// request resolved to at least one data column.
pub(crate) fn apply_save_set_checked(
    signals: Vec<ScalarSignal>,
    saves: &SaveSet,
    analysis: &str,
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    if saves.keeps_everything()
        && saves
            .signals
            .iter()
            .all(|signal| matches!(signal, SaveSignal::All))
    {
        return Ok(signals);
    }

    let expanded = with_differential_voltage_signals(signals, saves);
    for saved in &saves.signals {
        let Some(authored) = authored_save_symbol(saved) else {
            continue;
        };
        let selection = single_save_set(saved);
        if !expanded
            .iter()
            .any(|signal| signal_is_selected(signal, &selection))
        {
            return Err(rspice_core::SimulationError::requested_signal_unavailable(
                authored, analysis, None,
            ));
        }
    }
    Ok(expanded
        .into_iter()
        .filter(|signal| signal_is_selected(signal, saves))
        .collect())
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

/// Restrict a complex result while refusing device probes the result did not
/// materialize.
///
/// AC-family result types currently carry node voltages and branch currents.
/// Callers must use this checked entry point so an authored
/// `.SAVE @device[param]` cannot turn into a successful frequency-only file.
/// The presence check deliberately accepts a future complex device-observable
/// signal with the exact qualified display name, so extending a core result
/// registry will automatically enable projection rather than require another
/// frontend special case.
pub(crate) fn apply_save_set_complex_checked(
    signals: Vec<ComplexSignal>,
    saves: &SaveSet,
    analysis: &str,
) -> Result<Vec<ComplexSignal>, rspice_core::SimulationError> {
    if saves.keeps_everything()
        && saves
            .signals
            .iter()
            .all(|signal| matches!(signal, SaveSignal::All))
    {
        return Ok(signals);
    }

    let expanded = with_differential_complex_signals(signals, saves);
    for saved in &saves.signals {
        let Some(authored) = authored_save_symbol(saved) else {
            continue;
        };
        let selection = single_save_set(saved);
        if !expanded
            .iter()
            .any(|signal| complex_signal_is_selected(signal, &selection))
        {
            return Err(rspice_core::SimulationError::requested_signal_unavailable(
                authored, analysis, None,
            ));
        }
    }
    Ok(expanded
        .into_iter()
        .filter(|signal| complex_signal_is_selected(signal, saves))
        .collect())
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
    result
        .node_voltages
        .iter()
        .copied()
        .enumerate()
        .skip(1)
        .map(|(node_id, value)| {
            let raw_name = result.node_names.get(node_id).map_or_else(
                || node_id.to_string(),
                |name| voltage_raw_name(name, node_id),
            );
            voltage_signal(raw_name, vec![value])
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

fn device_param_signal(request: &DeviceParamRequest, values: Vec<Value>) -> ScalarSignal {
    ScalarSignal {
        display_name: request.authored_symbol.clone(),
        raw_name: request.authored_symbol.clone(),
        kind: SignalKind::Scalar,
        values,
    }
}

fn dc_device_param_value(result: &SimulationResult, request: &DeviceParamRequest) -> Option<Value> {
    [
        request.authored_symbol.clone(),
        format!("{}:{}", request.device, request.param),
        format!("N({}:{})", request.device, request.param),
    ]
    .into_iter()
    .find_map(|candidate| result.try_dc_observable_named(&candidate))
}

fn dc_operating_point_device_param_signals(
    result: &SimulationResult,
    saves: &SaveSet,
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    requested_device_params(saves)
        .into_iter()
        .map(|request| {
            let value = dc_device_param_value(result, &request).ok_or_else(|| {
                requested_signal_unavailable(&request, "DC OP", Some("operating point".to_string()))
            })?;
            Ok(device_param_signal(&request, vec![value]))
        })
        .collect()
}

fn dc_sweep_device_param_signals(
    results: &[(Value, SimulationResult)],
    saves: &SaveSet,
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    requested_device_params(saves)
        .into_iter()
        .map(|request| {
            let mut values = Vec::with_capacity(results.len());
            if results.is_empty() {
                return Err(requested_signal_unavailable(
                    &request,
                    "DC",
                    Some("empty sweep result".to_string()),
                ));
            }
            for (point_index, (scale, result)) in results.iter().enumerate() {
                let coordinate = format!(
                    "sweep point {} ({scale:.16e})",
                    point_index.saturating_add(1)
                );
                let value = dc_device_param_value(result, &request).ok_or_else(|| {
                    requested_signal_unavailable(&request, "DC", Some(coordinate))
                })?;
                values.push(value);
            }
            Ok(device_param_signal(&request, values))
        })
        .collect()
}

fn transient_device_param_signals(
    result: &TransientResult,
    saves: &SaveSet,
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    requested_device_params(saves)
        .into_iter()
        .map(|request| {
            let values = result
                .try_device_op_waveform_named(&request.device, &request.param)
                .ok_or_else(|| requested_signal_unavailable(&request, "TRAN", None))?;
            if values.len() != result.time.len() {
                return Err(rspice_core::SimulationError::Circuit(format!(
                    "TRAN result schema is malformed for requested signal '{}': {} values for {} time points",
                    request.authored_symbol,
                    values.len(),
                    result.time.len()
                )));
            }
            Ok(device_param_signal(&request, values.to_vec()))
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SignalIdentity {
    kind: SignalKind,
    canonical_raw_name: String,
}

impl SignalIdentity {
    fn from_scalar(signal: &ScalarSignal) -> Self {
        Self {
            kind: signal.kind,
            canonical_raw_name: signal.raw_name.to_ascii_lowercase(),
        }
    }

    fn from_complex(signal: &ComplexSignal) -> Self {
        Self {
            kind: signal.kind,
            canonical_raw_name: signal.raw_name.to_ascii_lowercase(),
        }
    }

    fn display_name(&self) -> String {
        match self.kind {
            SignalKind::Voltage => format!("V({})", self.canonical_raw_name),
            SignalKind::Current => format!("I({})", self.canonical_raw_name),
            SignalKind::Digital => format!("D({})", self.canonical_raw_name),
            SignalKind::Scalar => self.canonical_raw_name.clone(),
        }
    }
}

fn validate_dc_point_shape(
    result: &SimulationResult,
    point: &str,
) -> Result<(), rspice_core::SimulationError> {
    if result.node_names.len() != result.node_voltages.len() {
        return Err(rspice_core::SimulationError::Circuit(format!(
            "DC result schema is malformed at {point}: {} node names for {} voltages",
            result.node_names.len(),
            result.node_voltages.len()
        )));
    }
    if result.branch_names.len() != result.branch_currents.len() {
        return Err(rspice_core::SimulationError::Circuit(format!(
            "DC result schema is malformed at {point}: {} branch names for {} currents",
            result.branch_names.len(),
            result.branch_currents.len()
        )));
    }
    for (index, name) in result.node_names.iter().enumerate() {
        if canonical_name_is_empty(name, 'V') {
            return Err(rspice_core::SimulationError::Circuit(format!(
                "DC result schema has an empty node name at index {index} at {point}"
            )));
        }
    }
    for (index, name) in result.branch_names.iter().enumerate() {
        if canonical_name_is_empty(name, 'I') {
            return Err(rspice_core::SimulationError::Circuit(format!(
                "DC result schema has an empty branch name at index {index} at {point}"
            )));
        }
    }
    Ok(())
}

fn canonical_name_is_empty(name: &str, prefix: char) -> bool {
    let trimmed = name.trim();
    unwrap_signal_name(trimmed, prefix)
        .unwrap_or(trimmed)
        .trim()
        .is_empty()
}

pub(crate) fn checked_dc_operating_point_signals(
    result: &SimulationResult,
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    dc_point_signals(result, "operating point")
}

pub(crate) fn dc_operating_point_export_signals(
    result: &SimulationResult,
    saves: &SaveSet,
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    let mut signals = checked_dc_operating_point_signals(result)?;
    signals.extend(dc_operating_point_device_param_signals(result, saves)?);
    apply_save_set_checked(signals, saves, "DC OP")
}

fn dc_point_signals(
    result: &SimulationResult,
    point: &str,
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    validate_dc_point_shape(result, point)?;
    Ok(dc_operating_point_signals(result))
}

fn scalar_point_index(
    signals: Vec<ScalarSignal>,
    point: &str,
) -> Result<std::collections::BTreeMap<SignalIdentity, ScalarSignal>, rspice_core::SimulationError>
{
    let mut indexed = std::collections::BTreeMap::new();
    for signal in signals {
        let identity = SignalIdentity::from_scalar(&signal);
        if indexed.insert(identity.clone(), signal).is_some() {
            return Err(rspice_core::SimulationError::Circuit(format!(
                "DC result schema contains duplicate signal '{}' at {point}",
                identity.display_name()
            )));
        }
    }
    Ok(indexed)
}

fn schema_difference(
    expected: impl Iterator<Item = SignalIdentity>,
    actual: impl Iterator<Item = SignalIdentity>,
) -> (Vec<String>, Vec<String>) {
    let expected = expected.collect::<std::collections::BTreeSet<_>>();
    let actual = actual.collect::<std::collections::BTreeSet<_>>();
    let missing = expected
        .difference(&actual)
        .map(SignalIdentity::display_name)
        .collect();
    let unexpected = actual
        .difference(&expected)
        .map(SignalIdentity::display_name)
        .collect();
    (missing, unexpected)
}

pub(crate) fn dc_sweep_signals(
    results: &[(Value, SimulationResult)],
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    let Some((first_scale, first_result)) = results.first() else {
        return Ok(Vec::new());
    };

    let first_point = format!("sweep point 1 ({first_scale:.16e})");
    let first_signals = dc_point_signals(first_result, &first_point)?;
    let expected_identities = first_signals
        .iter()
        .map(SignalIdentity::from_scalar)
        .collect::<Vec<_>>();
    let mut aggregated = first_signals
        .into_iter()
        .map(|mut signal| {
            signal.values.clear();
            signal
        })
        .collect::<Vec<_>>();

    for (point_index, (scale, result)) in results.iter().enumerate() {
        let point = format!("sweep point {} ({scale:.16e})", point_index + 1);
        let mut actual = scalar_point_index(dc_point_signals(result, &point)?, &point)?;
        let (missing, unexpected) =
            schema_difference(expected_identities.iter().cloned(), actual.keys().cloned());
        if !missing.is_empty() || !unexpected.is_empty() {
            return Err(rspice_core::SimulationError::Circuit(format!(
                "DC result schema changes at {point}: missing [{}]; unexpected [{}]",
                missing.join(", "),
                unexpected.join(", ")
            )));
        }

        for (signal, identity) in aggregated.iter_mut().zip(&expected_identities) {
            let point_signal = actual.remove(identity).ok_or_else(|| {
                rspice_core::SimulationError::Circuit(format!(
                    "DC result schema lost signal '{}' while aggregating {point}",
                    identity.display_name()
                ))
            })?;
            let value = point_signal.values.first().copied().ok_or_else(|| {
                rspice_core::SimulationError::Circuit(format!(
                    "DC result signal '{}' has no scalar value at {point}",
                    identity.display_name()
                ))
            })?;
            if point_signal.values.len() != 1 {
                return Err(rspice_core::SimulationError::Circuit(format!(
                    "DC result signal '{}' has {} values at {point}; expected one",
                    identity.display_name(),
                    point_signal.values.len()
                )));
            }
            signal.values.push(value);
        }
    }

    Ok(aggregated)
}

pub(crate) fn dc_sweep_voltage_signals(
    results: &[(Value, SimulationResult)],
) -> Result<Vec<ScalarSignal>, rspice_core::SimulationError> {
    Ok(dc_sweep_signals(results)?
        .into_iter()
        .filter(|signal| signal.kind == SignalKind::Voltage)
        .collect())
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
        let mut signals = dc_sweep_signals(results)?;
        signals.extend(dc_sweep_device_param_signals(results, &netlist.saves)?);
        return apply_save_set_checked(signals, &netlist.saves, "DC");
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
        let mut signals = transient_signals(result);
        signals.extend(transient_device_param_signals(result, &netlist.saves)?);
        return apply_save_set_checked(signals, &netlist.saves, "TRAN");
    }
    rspice_core::analysis::evaluate_tran_output_requests_with_abort(netlist, result, limits, abort)
        .map(projected_output_signals)
}

fn ac_point_signals(
    result: &AcResult,
    point: &str,
) -> Result<Vec<ComplexSignal>, rspice_core::SimulationError> {
    if result.node_names.len() != result.voltages.len() {
        return Err(rspice_core::SimulationError::Circuit(format!(
            "AC result schema is malformed at {point}: {} node names for {} voltages",
            result.node_names.len(),
            result.voltages.len()
        )));
    }
    if result.branch_names.len() != result.currents.len() {
        return Err(rspice_core::SimulationError::Circuit(format!(
            "AC result schema is malformed at {point}: {} branch names for {} currents",
            result.branch_names.len(),
            result.currents.len()
        )));
    }
    for (index, name) in result.node_names.iter().enumerate() {
        if canonical_name_is_empty(name, 'V') {
            return Err(rspice_core::SimulationError::Circuit(format!(
                "AC result schema has an empty node name at index {index} at {point}"
            )));
        }
    }
    for (index, name) in result.branch_names.iter().enumerate() {
        if canonical_name_is_empty(name, 'I') {
            return Err(rspice_core::SimulationError::Circuit(format!(
                "AC result schema has an empty branch name at index {index} at {point}"
            )));
        }
    }

    let mut signals = Vec::with_capacity(result.voltages.len() + result.currents.len());
    for (node_idx, value) in result.voltages.iter().copied().enumerate() {
        let raw_name = voltage_raw_name(&result.node_names[node_idx], node_idx + 1);
        signals.push(ComplexSignal {
            display_name: format!("V({raw_name})"),
            raw_name,
            kind: SignalKind::Voltage,
            real: vec![value.re],
            imag: vec![value.im],
        });
    }
    for (branch_idx, value) in result.currents.iter().copied().enumerate() {
        let raw_name = current_raw_name(&result.branch_names[branch_idx], branch_idx + 1);
        signals.push(ComplexSignal {
            display_name: format!("I({raw_name})"),
            raw_name,
            kind: SignalKind::Current,
            real: vec![value.re],
            imag: vec![value.im],
        });
    }
    Ok(signals)
}

fn complex_point_index(
    signals: Vec<ComplexSignal>,
    point: &str,
) -> Result<std::collections::BTreeMap<SignalIdentity, ComplexSignal>, rspice_core::SimulationError>
{
    let mut indexed = std::collections::BTreeMap::new();
    for signal in signals {
        let identity = SignalIdentity::from_complex(&signal);
        if indexed.insert(identity.clone(), signal).is_some() {
            return Err(rspice_core::SimulationError::Circuit(format!(
                "AC result schema contains duplicate signal '{}' at {point}",
                identity.display_name()
            )));
        }
    }
    Ok(indexed)
}

pub(crate) fn ac_signals(
    results: &[AcResult],
) -> Result<Vec<ComplexSignal>, rspice_core::SimulationError> {
    let Some(first_result) = results.first() else {
        return Ok(Vec::new());
    };
    let first_point = format!("frequency point 1 ({:.16e} Hz)", first_result.frequency);
    let first_signals = ac_point_signals(first_result, &first_point)?;
    let expected_identities = first_signals
        .iter()
        .map(SignalIdentity::from_complex)
        .collect::<Vec<_>>();
    let mut aggregated = first_signals
        .into_iter()
        .map(|mut signal| {
            signal.real.clear();
            signal.imag.clear();
            signal
        })
        .collect::<Vec<_>>();

    for (point_index, result) in results.iter().enumerate() {
        let point = format!(
            "frequency point {} ({:.16e} Hz)",
            point_index + 1,
            result.frequency
        );
        let mut actual = complex_point_index(ac_point_signals(result, &point)?, &point)?;
        let (missing, unexpected) =
            schema_difference(expected_identities.iter().cloned(), actual.keys().cloned());
        if !missing.is_empty() || !unexpected.is_empty() {
            return Err(rspice_core::SimulationError::Circuit(format!(
                "AC result schema changes at {point}: missing [{}]; unexpected [{}]",
                missing.join(", "),
                unexpected.join(", ")
            )));
        }

        for (signal, identity) in aggregated.iter_mut().zip(&expected_identities) {
            let point_signal = actual.remove(identity).ok_or_else(|| {
                rspice_core::SimulationError::Circuit(format!(
                    "AC result schema lost signal '{}' while aggregating {point}",
                    identity.display_name()
                ))
            })?;
            if point_signal.real.len() != 1 || point_signal.imag.len() != 1 {
                return Err(rspice_core::SimulationError::Circuit(format!(
                    "AC result signal '{}' is not scalar at {point}",
                    identity.display_name()
                )));
            }
            signal.real.push(point_signal.real[0]);
            signal.imag.push(point_signal.imag[0]);
        }
    }

    Ok(aggregated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::Complex64;

    fn dc_result(nodes: &[(&str, Value)], branches: &[(&str, Value)]) -> SimulationResult {
        let mut result = SimulationResult::new(nodes.len(), branches.len());
        result.node_names = std::iter::once("0".to_string())
            .chain(nodes.iter().map(|(name, _)| (*name).to_string()))
            .collect();
        result.node_voltages = std::iter::once(0.0)
            .chain(nodes.iter().map(|(_, value)| *value))
            .collect();
        result.branch_names = branches
            .iter()
            .map(|(name, _)| (*name).to_string())
            .collect();
        result.branch_currents = branches.iter().map(|(_, value)| *value).collect();
        result
    }

    fn ac_result(
        frequency: Value,
        nodes: &[(&str, Complex64)],
        branches: &[(&str, Complex64)],
    ) -> AcResult {
        AcResult {
            frequency,
            node_names: nodes.iter().map(|(name, _)| (*name).to_string()).collect(),
            branch_names: branches
                .iter()
                .map(|(name, _)| (*name).to_string())
                .collect(),
            voltages: nodes.iter().map(|(_, value)| *value).collect(),
            currents: branches.iter().map(|(_, value)| *value).collect(),
        }
    }

    fn scalar_values<'a>(signals: &'a [ScalarSignal], name: &str) -> &'a [Value] {
        signals
            .iter()
            .find(|signal| signal.display_name.eq_ignore_ascii_case(name))
            .unwrap_or_else(|| panic!("missing scalar signal {name}"))
            .values
            .as_slice()
    }

    fn complex_values<'a>(signals: &'a [ComplexSignal], name: &str) -> (&'a [Value], &'a [Value]) {
        let signal = signals
            .iter()
            .find(|signal| signal.display_name.eq_ignore_ascii_case(name))
            .unwrap_or_else(|| panic!("missing complex signal {name}"));
        (&signal.real, &signal.imag)
    }

    fn device_save(device: &str, param: &str) -> SaveSet {
        SaveSet {
            signals: vec![SaveSignal::DeviceParam {
                device: device.to_string(),
                param: param.to_string(),
            }],
        }
    }

    #[test]
    fn dc_device_save_materializes_authored_qualified_name() {
        let mut result = dc_result(&[("out", 1.0)], &[]);
        result.dc_observables.push(("D1:ID".to_string(), 2.5e-3));
        let saves = device_save("D1", "Id");

        let op =
            dc_operating_point_export_signals(&result, &saves).expect("DC observable is available");
        assert_eq!(op.len(), 1);
        assert_eq!(op[0].display_name, "@D1[Id]");
        assert_eq!(op[0].values, [2.5e-3]);

        let mut netlist = rspice_core::Netlist::parse(
            "device projection test\nV1 out 0 1\nR1 out 0 1k\n.DC V1 0 1 1\n.END\n",
        )
        .expect("test netlist parses");
        netlist.saves = saves;
        let sweep = dc_export_signals(
            &netlist,
            &[(0.0, result.clone()), (1.0, result)],
            rspice_core::ResourceLimits::default(),
            &rspice_core::NoAbort,
        )
        .expect("DC sweep observable is available");
        assert_eq!(sweep.len(), 1);
        assert_eq!(sweep[0].display_name, "@D1[Id]");
        assert_eq!(sweep[0].values, [2.5e-3, 2.5e-3]);
    }

    #[test]
    fn checked_complex_save_returns_typed_authored_symbol_error() {
        let error = apply_save_set_complex_checked(Vec::new(), &device_save("Mdriver", "Id"), "AC")
            .expect_err("AC result has no device-observable registry");
        assert_eq!(
            error.descriptor().code,
            rspice_core::SimulationErrorCode::RequestedSignalUnavailable
        );
        let rspice_core::SimulationError::RequestedSignalUnavailable(detail) = error else {
            panic!("missing typed unavailable-signal error");
        };
        assert_eq!(detail.signal, "@Mdriver[Id]");
        assert_eq!(detail.analysis, "AC");

        let missing_voltage = SaveSet {
            signals: vec![SaveSignal::Voltage("MissingNode".to_string())],
        };
        let error = apply_save_set_complex_checked(Vec::new(), &missing_voltage, "AC")
            .expect_err("missing AC voltage cannot become a frequency-only result");
        let rspice_core::SimulationError::RequestedSignalUnavailable(detail) = error else {
            panic!("missing typed unavailable-voltage error");
        };
        assert_eq!(detail.signal, "V(MissingNode)");
    }

    #[test]
    fn checked_scalar_save_validates_each_request_and_supports_wildcards() {
        let signals = vec![voltage_signal("out".to_string(), vec![1.0])];
        let wildcard = SaveSet {
            signals: vec![SaveSignal::Voltage("o*".to_string())],
        };
        let projected = apply_save_set_checked(signals.clone(), &wildcard, "DC OP")
            .expect("wildcard resolves one voltage");
        assert_eq!(projected.len(), 1);
        assert_eq!(projected[0].display_name, "V(out)");

        let missing = SaveSet {
            signals: vec![SaveSignal::Current("AuthoredCase".to_string())],
        };
        let error = apply_save_set_checked(signals, &missing, "DC OP")
            .expect_err("missing current must not produce an empty result");
        let rspice_core::SimulationError::RequestedSignalUnavailable(detail) = error else {
            panic!("missing typed unavailable-current error");
        };
        assert_eq!(detail.signal, "I(AuthoredCase)");
    }

    #[test]
    fn dc_sweep_aggregation_tracks_signal_identity_across_storage_reordering() {
        let results = vec![
            (
                1.0,
                dc_result(&[("a", 1.0), ("b", 2.0)], &[("V1", 3.0), ("L1", 4.0)]),
            ),
            (
                2.0,
                dc_result(&[("b", 20.0), ("a", 10.0)], &[("L1", 40.0), ("V1", 30.0)]),
            ),
        ];

        let signals = dc_sweep_signals(&results).expect("same named schema aggregates");
        assert_eq!(scalar_values(&signals, "V(a)"), [1.0, 10.0]);
        assert_eq!(scalar_values(&signals, "V(b)"), [2.0, 20.0]);
        assert_eq!(scalar_values(&signals, "I(V1)"), [3.0, 30.0]);
        assert_eq!(scalar_values(&signals, "I(L1)"), [4.0, 40.0]);
    }

    #[test]
    fn dc_sweep_topology_change_fails_instead_of_swapping_or_zero_filling() {
        let changed_node = vec![
            (1.0, dc_result(&[("a", 1.0)], &[("V1", 2.0)])),
            (2.0, dc_result(&[("b", 3.0)], &[("V1", 4.0)])),
        ];
        let error = dc_sweep_signals(&changed_node).expect_err("renamed topology must fail closed");
        let message = error.to_string();
        assert!(message.contains("missing [V(a)]"), "{message}");
        assert!(message.contains("unexpected [V(b)]"), "{message}");

        let missing_branch = vec![
            (1.0, dc_result(&[("a", 1.0)], &[("V1", 2.0)])),
            (2.0, dc_result(&[("a", 3.0)], &[])),
        ];
        let error = dc_sweep_signals(&missing_branch)
            .expect_err("missing branch must not become a zero current");
        let message = error.to_string();
        assert!(message.contains("missing [I(v1)]"), "{message}");
    }

    #[test]
    fn malformed_dc_result_shape_is_reported_before_indexing() {
        let mut malformed = dc_result(&[("a", 1.0)], &[]);
        malformed.node_names.clear();
        let error = dc_sweep_signals(&[(1.0, malformed)]).expect_err("shape mismatch must fail");
        assert!(error.to_string().contains("0 node names for 2 voltages"));

        let mut unnamed = dc_result(&[("a", 1.0)], &[("V1", 2.0)]);
        unnamed.branch_names[0] = " I( ) ".to_string();
        let error = dc_sweep_signals(&[(1.0, unnamed)])
            .expect_err("empty canonical names must not fall back to ordinals");
        assert!(error.to_string().contains("empty branch name"));
    }

    #[test]
    fn ac_aggregation_tracks_names_and_refuses_schema_changes() {
        let first = ac_result(
            1.0,
            &[
                ("a", Complex64::new(1.0, 2.0)),
                ("b", Complex64::new(3.0, 4.0)),
            ],
            &[("V1", Complex64::new(5.0, 6.0))],
        );
        let reordered = ac_result(
            2.0,
            &[
                ("b", Complex64::new(30.0, 40.0)),
                ("a", Complex64::new(10.0, 20.0)),
            ],
            &[("V1", Complex64::new(50.0, 60.0))],
        );
        let signals = ac_signals(&[first.clone(), reordered]).expect("same AC schema aggregates");
        assert_eq!(
            complex_values(&signals, "V(a)"),
            (&[1.0, 10.0][..], &[2.0, 20.0][..])
        );
        assert_eq!(
            complex_values(&signals, "V(b)"),
            (&[3.0, 30.0][..], &[4.0, 40.0][..])
        );
        assert_eq!(
            complex_values(&signals, "I(V1)"),
            (&[5.0, 50.0][..], &[6.0, 60.0][..])
        );

        let changed = ac_result(
            2.0,
            &[
                ("c", Complex64::new(7.0, 8.0)),
                ("b", Complex64::new(9.0, 10.0)),
            ],
            &[("V1", Complex64::new(11.0, 12.0))],
        );
        let error =
            ac_signals(&[first.clone(), changed]).expect_err("changed AC schema must fail closed");
        let message = error.to_string();
        assert!(message.contains("missing [V(a)]"), "{message}");
        assert!(message.contains("unexpected [V(c)]"), "{message}");

        let reordered_branches = ac_result(
            2.0,
            &[
                ("a", Complex64::new(7.0, 8.0)),
                ("b", Complex64::new(9.0, 10.0)),
            ],
            &[
                ("L1", Complex64::new(30.0, 40.0)),
                ("V1", Complex64::new(50.0, 60.0)),
            ],
        );
        let first_with_branches = ac_result(
            1.0,
            &[
                ("a", Complex64::new(1.0, 2.0)),
                ("b", Complex64::new(3.0, 4.0)),
            ],
            &[
                ("V1", Complex64::new(5.0, 6.0)),
                ("L1", Complex64::new(7.0, 8.0)),
            ],
        );
        let signals = ac_signals(&[first_with_branches, reordered_branches])
            .expect("named AC branch currents aggregate after reordering");
        assert_eq!(
            complex_values(&signals, "I(V1)"),
            (&[5.0, 50.0][..], &[6.0, 60.0][..])
        );
        assert_eq!(
            complex_values(&signals, "I(L1)"),
            (&[7.0, 30.0][..], &[8.0, 40.0][..])
        );

        let mut missing_branch = first.clone();
        missing_branch.branch_names.clear();
        missing_branch.currents.clear();
        let error = ac_signals(&[first.clone(), missing_branch])
            .expect_err("missing AC current must not become a zero");
        assert!(error.to_string().contains("missing [I(v1)]"));

        let mut unnamed = first;
        unnamed.node_names[0] = "V( )".to_string();
        let error = ac_signals(&[unnamed])
            .expect_err("empty canonical AC names must not fall back to ordinals");
        assert!(error.to_string().contains("empty node name"));
    }
}
