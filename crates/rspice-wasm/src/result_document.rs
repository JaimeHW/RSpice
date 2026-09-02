//! Versioned, loss-aware analog result documents for browser consumers.
//!
//! The document is retained in WebAssembly memory. JavaScript first reads the
//! small metadata object, then requests bounded point windows whose numeric
//! columns are published as typed arrays by `lib.rs`. The canonical Rust/Serde
//! representation uses `Option` samples, so missing data is never encoded as
//! a plausible zero.

use std::collections::{BTreeMap, HashSet};

use rspice_core::{
    analysis::{NoiseResult, NoiseSourceType},
    circuit::DeviceOpReport,
    engine::DcSweepPointResult,
    solver::SimulationResult,
};
use serde::{Deserialize, Serialize};

use crate::{AcPointSnapshot, TransientSnapshot};

pub const ANALOG_RESULT_SCHEMA: &str = "rspice-analog-result";
pub const ANALOG_RESULT_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AnalogResultDocument {
    pub schema: String,
    pub schema_version: u32,
    pub analysis: AnalysisIdentity,
    /// Shared-deck STEP/TEMP coordinates are not accepted by this API yet.
    /// Keeping the field explicit prevents scalar calls from being confused
    /// with an implicit first coordinate.
    pub coordinate_id: Option<String>,
    pub point_count: usize,
    pub axes: Vec<AxisSeries>,
    pub signals: Vec<SignalSeries>,
    pub device_states: Vec<DeviceStateSeries>,
}

impl AnalogResultDocument {
    fn new(kind: AnalogAnalysisKind, request_kind: &str, ordinal: usize) -> Self {
        Self {
            schema: ANALOG_RESULT_SCHEMA.to_owned(),
            schema_version: ANALOG_RESULT_VERSION,
            analysis: AnalysisIdentity {
                id: format!("{}-{ordinal:03}", kind.id_prefix()),
                kind,
                request_kind: request_kind.to_owned(),
                ordinal,
            },
            coordinate_id: None,
            point_count: 0,
            axes: Vec::new(),
            signals: Vec::new(),
            device_states: Vec::new(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema != ANALOG_RESULT_SCHEMA {
            return Err(format!("unexpected analog result schema {:?}", self.schema));
        }
        if self.schema_version != ANALOG_RESULT_VERSION {
            return Err(format!(
                "analog result version {} is unsupported (current version is {})",
                self.schema_version, ANALOG_RESULT_VERSION
            ));
        }
        let ordinary_id = format!(
            "{}-{:03}",
            self.analysis.kind.id_prefix(),
            self.analysis.ordinal
        );
        let implicit_op_id = format!("implicit-op-{:03}", self.analysis.ordinal);
        if self.analysis.ordinal == 0
            || (self.analysis.id != ordinary_id
                && !(self.analysis.kind == AnalogAnalysisKind::OperatingPoint
                    && self.analysis.id == implicit_op_id))
        {
            return Err("analysis identity does not match its kind and ordinal".to_owned());
        }
        if self.analysis.request_kind.trim().is_empty() || self.point_count == 0 {
            return Err("analysis request kind and a nonzero point count are required".to_owned());
        }
        let expected_axes = match self.analysis.kind {
            AnalogAnalysisKind::OperatingPoint => 0,
            AnalogAnalysisKind::DcSweep
            | AnalogAnalysisKind::AcSmallSignal
            | AnalogAnalysisKind::Transient
            | AnalogAnalysisKind::Noise => 1,
        };
        if self.axes.len() != expected_axes {
            return Err("axis count does not match the analysis kind".to_owned());
        }

        let mut identities = HashSet::new();
        for axis in &self.axes {
            if axis.name.trim().is_empty()
                || axis.values.len() != self.point_count
                || axis.values.iter().any(|value| !value.is_finite())
                || !identities.insert(format!("axis:{}", axis.name.to_ascii_lowercase()))
            {
                return Err("axis identity, shape, or value is invalid".to_owned());
            }
        }
        for signal in &self.signals {
            if signal.canonical_name.trim().is_empty()
                || signal.display_name.trim().is_empty()
                || !identities.insert(format!(
                    "signal:{}",
                    signal.canonical_name.to_ascii_lowercase()
                ))
                || signal.values.len() != self.point_count
                || !signal.values.is_finite()
            {
                return Err("signal identity, shape, or value is invalid".to_owned());
            }
            match signal.kind {
                AnalogSignalKind::Voltage if signal.unit != Some(SignalUnit::Volt) => {
                    return Err("voltage signals must use volts".to_owned());
                }
                AnalogSignalKind::BranchCurrent if signal.unit != Some(SignalUnit::Ampere) => {
                    return Err("branch-current signals must use amperes".to_owned());
                }
                _ => {}
            }
        }
        for state in &self.device_states {
            if state.device_name.trim().is_empty()
                || state.regions.len() != self.point_count
                || !identities.insert(format!("state:{}", state.device_name.to_ascii_lowercase()))
            {
                return Err("device-state identity or shape is invalid".to_owned());
            }
        }
        Ok(())
    }

    pub fn metadata(&self, maximum_window_values: usize) -> AnalogResultMetadata {
        AnalogResultMetadata {
            schema: self.schema.clone(),
            schema_version: self.schema_version,
            analysis: self.analysis.clone(),
            coordinate_id: self.coordinate_id.clone(),
            point_count: self.point_count,
            axes: self
                .axes
                .iter()
                .map(|axis| AxisDescriptor {
                    name: axis.name.clone(),
                    unit: axis.unit,
                })
                .collect(),
            signals: self
                .signals
                .iter()
                .map(|signal| SignalDescriptor {
                    canonical_name: signal.canonical_name.clone(),
                    display_name: signal.display_name.clone(),
                    kind: signal.kind,
                    owner: signal.owner.clone(),
                    unit: signal.unit,
                    value_type: signal.values.value_type(),
                    has_any_valid_samples: signal.values.has_any_valid_samples(),
                })
                .collect(),
            device_states: self
                .device_states
                .iter()
                .map(|state| DeviceStateDescriptor {
                    device_name: state.device_name.clone(),
                    device_kind: state.device_kind.clone(),
                })
                .collect(),
            maximum_window_values,
        }
    }

    /// Numeric slots retained by the document, including explicit missing
    /// slots and both components of complex samples.
    pub fn retained_numeric_value_count(&self) -> usize {
        self.axes
            .iter()
            .map(|axis| axis.values.len())
            .chain(self.signals.iter().map(|signal| {
                signal
                    .values
                    .len()
                    .saturating_mul(signal.values.numeric_columns())
            }))
            .fold(0usize, usize::saturating_add)
    }

    pub fn window(
        &self,
        start: usize,
        count: usize,
        maximum_values: usize,
    ) -> Result<AnalogResultWindow, String> {
        // `AnalogResultDocument` is a public Rust DTO. Callers can deserialize
        // or construct one without going through `WasmAnalogResultHandle`, so
        // a window request must not assume that `validate()` already proved
        // every retained column has `point_count` entries. Check the O(column)
        // shape invariants before any slice operation; otherwise a malformed
        // document can turn an ordinary API error into a bounds panic.
        if self
            .axes
            .iter()
            .any(|axis| axis.values.len() != self.point_count)
            || self
                .signals
                .iter()
                .any(|signal| signal.values.len() != self.point_count)
        {
            return Err("result document columns do not match point_count".to_owned());
        }
        let end = start
            .checked_add(count)
            .ok_or_else(|| "result window range overflows usize".to_owned())?;
        if count == 0 || start >= self.point_count || end > self.point_count {
            return Err(format!(
                "result window [{start}, {end}) is outside 0..{}",
                self.point_count
            ));
        }
        let values_per_point = self.axes.len().saturating_add(
            self.signals
                .iter()
                .map(|signal| signal.values.numeric_columns() + 1)
                .sum::<usize>(),
        );
        let requested_values = values_per_point.saturating_mul(count);
        if requested_values > maximum_values {
            return Err(format!(
                "result window requires {requested_values} numeric/validity values but the transfer limit is {maximum_values}"
            ));
        }
        Ok(AnalogResultWindow {
            schema_version: self.schema_version,
            analysis_id: self.analysis.id.clone(),
            coordinate_id: self.coordinate_id.clone(),
            start,
            end,
            point_count: self.point_count,
            axes: self
                .axes
                .iter()
                .map(|axis| AxisWindow {
                    name: axis.name.clone(),
                    values: axis.values[start..end].to_vec(),
                })
                .collect(),
            signals: self
                .signals
                .iter()
                .map(|signal| SignalWindow {
                    canonical_name: signal.canonical_name.clone(),
                    values: signal.values.window(start, end),
                })
                .collect(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AnalysisIdentity {
    pub id: String,
    pub kind: AnalogAnalysisKind,
    pub request_kind: String,
    pub ordinal: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalogAnalysisKind {
    OperatingPoint,
    DcSweep,
    AcSmallSignal,
    Transient,
    Noise,
}

impl AnalogAnalysisKind {
    pub const ALL: [Self; 5] = [
        Self::OperatingPoint,
        Self::DcSweep,
        Self::AcSmallSignal,
        Self::Transient,
        Self::Noise,
    ];

    const fn id_prefix(self) -> &'static str {
        match self {
            Self::OperatingPoint => "op",
            Self::DcSweep => "dc",
            Self::AcSmallSignal => "ac",
            Self::Transient => "tran",
            Self::Noise => "noise",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AxisSeries {
    pub name: String,
    pub unit: Option<SignalUnit>,
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SignalSeries {
    pub canonical_name: String,
    pub display_name: String,
    pub kind: AnalogSignalKind,
    pub owner: SignalOwner,
    pub unit: Option<SignalUnit>,
    pub values: SignalValues,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalogSignalKind {
    Voltage,
    BranchCurrent,
    DeviceObservable,
    Scalar,
}

impl AnalogSignalKind {
    pub const ALL: [Self; 4] = [
        Self::Voltage,
        Self::BranchCurrent,
        Self::DeviceObservable,
        Self::Scalar,
    ];
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SignalOwner {
    Node {
        name: String,
    },
    Branch {
        name: String,
    },
    Device {
        device: Option<String>,
        parameter: Option<String>,
        device_kind: Option<String>,
    },
    Analysis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalUnit {
    Volt,
    Ampere,
    Second,
    Hertz,
    VoltSquaredPerHertz,
    Dimensionless,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "representation", rename_all = "snake_case")]
pub enum SignalValues {
    Real { samples: Vec<Option<f64>> },
    Complex { samples: Vec<Option<ComplexSample>> },
}

impl SignalValues {
    fn len(&self) -> usize {
        match self {
            Self::Real { samples } => samples.len(),
            Self::Complex { samples } => samples.len(),
        }
    }

    fn is_finite(&self) -> bool {
        match self {
            Self::Real { samples } => samples.iter().flatten().all(|value| value.is_finite()),
            Self::Complex { samples } => samples
                .iter()
                .flatten()
                .all(|value| value.real.is_finite() && value.imaginary.is_finite()),
        }
    }

    const fn value_type(&self) -> SignalValueType {
        match self {
            Self::Real { .. } => SignalValueType::Real,
            Self::Complex { .. } => SignalValueType::Complex,
        }
    }

    fn has_any_valid_samples(&self) -> bool {
        match self {
            Self::Real { samples } => samples.iter().any(Option::is_some),
            Self::Complex { samples } => samples.iter().any(Option::is_some),
        }
    }

    const fn numeric_columns(&self) -> usize {
        match self {
            Self::Real { .. } => 1,
            Self::Complex { .. } => 2,
        }
    }

    fn window(&self, start: usize, end: usize) -> SignalWindowValues {
        match self {
            Self::Real { samples } => SignalWindowValues::Real {
                values: samples[start..end]
                    .iter()
                    .map(|value| value.unwrap_or(0.0))
                    .collect(),
                validity: samples[start..end]
                    .iter()
                    .map(|value| u8::from(value.is_some()))
                    .collect(),
            },
            Self::Complex { samples } => SignalWindowValues::Complex {
                real: samples[start..end]
                    .iter()
                    .map(|value| value.map_or(0.0, |value| value.real))
                    .collect(),
                imaginary: samples[start..end]
                    .iter()
                    .map(|value| value.map_or(0.0, |value| value.imaginary))
                    .collect(),
                validity: samples[start..end]
                    .iter()
                    .map(|value| u8::from(value.is_some()))
                    .collect(),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ComplexSample {
    pub real: f64,
    pub imaginary: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DeviceStateSeries {
    pub device_name: String,
    pub device_kind: Option<String>,
    pub regions: Vec<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalogResultMetadata {
    pub schema: String,
    pub schema_version: u32,
    pub analysis: AnalysisIdentity,
    pub coordinate_id: Option<String>,
    pub point_count: usize,
    pub axes: Vec<AxisDescriptor>,
    pub signals: Vec<SignalDescriptor>,
    pub device_states: Vec<DeviceStateDescriptor>,
    pub maximum_window_values: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisDescriptor {
    pub name: String,
    pub unit: Option<SignalUnit>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalDescriptor {
    pub canonical_name: String,
    pub display_name: String,
    pub kind: AnalogSignalKind,
    pub owner: SignalOwner,
    pub unit: Option<SignalUnit>,
    pub value_type: SignalValueType,
    pub has_any_valid_samples: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalValueType {
    Real,
    Complex,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceStateDescriptor {
    pub device_name: String,
    pub device_kind: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalogResultWindow {
    pub schema_version: u32,
    pub analysis_id: String,
    pub coordinate_id: Option<String>,
    pub start: usize,
    pub end: usize,
    pub point_count: usize,
    pub axes: Vec<AxisWindow>,
    pub signals: Vec<SignalWindow>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AxisWindow {
    pub name: String,
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalWindow {
    pub canonical_name: String,
    pub values: SignalWindowValues,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "representation", rename_all = "snake_case")]
pub enum SignalWindowValues {
    Real {
        values: Vec<f64>,
        validity: Vec<u8>,
    },
    Complex {
        real: Vec<f64>,
        imaginary: Vec<f64>,
        validity: Vec<u8>,
    },
}

fn real_signal(
    canonical_name: String,
    display_name: String,
    kind: AnalogSignalKind,
    owner: SignalOwner,
    unit: Option<SignalUnit>,
    samples: Vec<Option<f64>>,
) -> SignalSeries {
    SignalSeries {
        canonical_name,
        display_name,
        kind,
        owner,
        unit,
        values: SignalValues::Real { samples },
    }
}

fn complex_signal(
    canonical_name: String,
    display_name: String,
    kind: AnalogSignalKind,
    owner: SignalOwner,
    unit: Option<SignalUnit>,
    samples: Vec<Option<ComplexSample>>,
) -> SignalSeries {
    SignalSeries {
        canonical_name,
        display_name,
        kind,
        owner,
        unit,
        values: SignalValues::Complex { samples },
    }
}

fn signal_owner_for_observable(name: &str) -> SignalOwner {
    let device = name
        .split_once(':')
        .map(|(device, _)| device)
        .or_else(|| {
            name.strip_prefix("I(")
                .or_else(|| name.strip_prefix("P("))
                .and_then(|tail| tail.strip_suffix(')'))
        })
        .filter(|device| !device.is_empty())
        .map(str::to_owned);
    SignalOwner::Device {
        device,
        parameter: Some(name.to_owned()),
        device_kind: None,
    }
}

fn append_solution_signals(document: &mut AnalogResultDocument, result: &SimulationResult) {
    for (name, value) in result.node_names.iter().zip(&result.node_voltages) {
        document.signals.push(real_signal(
            format!("v({})", name.to_ascii_lowercase()),
            format!("V({name})"),
            AnalogSignalKind::Voltage,
            SignalOwner::Node { name: name.clone() },
            Some(SignalUnit::Volt),
            vec![Some(*value)],
        ));
    }
    for (name, value) in result.branch_names.iter().zip(&result.branch_currents) {
        document.signals.push(real_signal(
            format!("i({})", name.to_ascii_lowercase()),
            format!("I({name})"),
            AnalogSignalKind::BranchCurrent,
            SignalOwner::Branch { name: name.clone() },
            Some(SignalUnit::Ampere),
            vec![Some(*value)],
        ));
    }
    let mut existing = document
        .signals
        .iter()
        .map(|signal| signal.canonical_name.to_ascii_lowercase())
        .collect::<HashSet<_>>();
    for (name, value) in &result.dc_observables {
        if !existing.insert(name.to_ascii_lowercase()) {
            continue;
        }
        document.signals.push(real_signal(
            name.to_ascii_lowercase(),
            name.clone(),
            AnalogSignalKind::DeviceObservable,
            signal_owner_for_observable(name),
            None,
            vec![Some(*value)],
        ));
    }
}

fn validate_simulation_result(result: &SimulationResult, context: &str) -> Result<(), String> {
    if result.node_names.len() != result.node_voltages.len() {
        return Err(format!(
            "{context} result has {} node names but {} node voltages",
            result.node_names.len(),
            result.node_voltages.len()
        ));
    }
    if result.branch_names.len() != result.branch_currents.len() {
        return Err(format!(
            "{context} result has {} branch names but {} branch currents",
            result.branch_names.len(),
            result.branch_currents.len()
        ));
    }
    if result
        .node_voltages
        .iter()
        .chain(&result.branch_currents)
        .chain(result.dc_observables.iter().map(|(_, value)| value))
        .any(|value| !value.is_finite())
    {
        return Err(format!("{context} result contains a non-finite value"));
    }
    if result.node_names.iter().any(|name| name.trim().is_empty())
        || result
            .branch_names
            .iter()
            .any(|name| name.trim().is_empty())
        || result
            .dc_observables
            .iter()
            .any(|(name, _)| name.trim().is_empty())
    {
        return Err(format!("{context} result contains an empty signal name"));
    }
    Ok(())
}

fn device_state_series(report: &DeviceOpReport, point_count: usize) -> Vec<DeviceStateSeries> {
    report
        .entries
        .iter()
        .map(|entry| DeviceStateSeries {
            device_name: entry.name.clone(),
            device_kind: Some(entry.device_kind.to_owned()),
            regions: vec![entry.region.map(str::to_owned); point_count],
        })
        .collect()
}

pub fn operating_point_document(
    result: SimulationResult,
    report: DeviceOpReport,
    ordinal: usize,
) -> Result<AnalogResultDocument, String> {
    validate_simulation_result(&result, "operating-point")?;
    let mut document = AnalogResultDocument::new(
        AnalogAnalysisKind::OperatingPoint,
        "operating_point",
        ordinal,
    );
    document.point_count = 1;
    append_solution_signals(&mut document, &result);
    document.device_states = device_state_series(&report, 1);
    document.validate()?;
    Ok(document)
}

pub fn ac_document(
    points: Vec<AcPointSnapshot>,
    ordinal: usize,
) -> Result<AnalogResultDocument, String> {
    let first = points
        .first()
        .ok_or_else(|| "AC result has no points".to_owned())?;
    let point_count = points.len();
    if first.node_names.iter().any(|name| name.trim().is_empty())
        || first.branch_names.iter().any(|name| name.trim().is_empty())
    {
        return Err("AC result contains an empty signal name".to_owned());
    }
    for point in &points {
        if point.node_names != first.node_names
            || point.branch_names != first.branch_names
            || point.voltages.real.len() != first.node_names.len()
            || point.voltages.imag.len() != first.node_names.len()
            || point.currents.real.len() != first.branch_names.len()
            || point.currents.imag.len() != first.branch_names.len()
        {
            return Err("AC result schema changes across frequencies".to_owned());
        }
    }
    let mut document = AnalogResultDocument::new(AnalogAnalysisKind::AcSmallSignal, "ac", ordinal);
    document.point_count = point_count;
    document.axes.push(AxisSeries {
        name: "frequency".to_owned(),
        unit: Some(SignalUnit::Hertz),
        values: points.iter().map(|point| point.frequency).collect(),
    });
    for (index, name) in first.node_names.iter().enumerate() {
        document.signals.push(complex_signal(
            format!("v({})", name.to_ascii_lowercase()),
            format!("V({name})"),
            AnalogSignalKind::Voltage,
            SignalOwner::Node { name: name.clone() },
            Some(SignalUnit::Volt),
            points
                .iter()
                .map(|point| {
                    Some(ComplexSample {
                        real: point.voltages.real[index],
                        imaginary: point.voltages.imag[index],
                    })
                })
                .collect(),
        ));
    }
    for (index, name) in first.branch_names.iter().enumerate() {
        document.signals.push(complex_signal(
            format!("i({})", name.to_ascii_lowercase()),
            format!("I({name})"),
            AnalogSignalKind::BranchCurrent,
            SignalOwner::Branch { name: name.clone() },
            Some(SignalUnit::Ampere),
            points
                .iter()
                .map(|point| {
                    Some(ComplexSample {
                        real: point.currents.real[index],
                        imaginary: point.currents.imag[index],
                    })
                })
                .collect(),
        ));
    }
    document.validate()?;
    Ok(document)
}

pub fn transient_document(
    result: TransientSnapshot,
    ordinal: usize,
) -> Result<AnalogResultDocument, String> {
    let point_count = result.time.len();
    if point_count == 0 || result.step_sizes.len() != point_count {
        return Err(format!(
            "transient result has {point_count} time points but {} step sizes",
            result.step_sizes.len()
        ));
    }
    if result.num_nodes != result.node_names.len()
        || result.node_names.len() != result.voltages.len()
    {
        return Err(format!(
            "transient result declares {} nodes but has {} names and {} voltage channels",
            result.num_nodes,
            result.node_names.len(),
            result.voltages.len()
        ));
    }
    if result.branch_names.len() != result.branch_currents.len() {
        return Err(format!(
            "transient result has {} branch names but {} current channels",
            result.branch_names.len(),
            result.branch_currents.len()
        ));
    }
    if result.node_names.iter().any(|name| name.trim().is_empty())
        || result
            .branch_names
            .iter()
            .any(|name| name.trim().is_empty())
        || result
            .device_op_traces
            .iter()
            .any(|trace| trace.device_name.trim().is_empty() || trace.parameter.trim().is_empty())
        || result
            .store_traces
            .iter()
            .any(|trace| trace.name.trim().is_empty())
    {
        return Err("transient result contains an empty signal identity".to_owned());
    }
    for (kind, name, values) in result
        .node_names
        .iter()
        .zip(&result.voltages)
        .map(|(name, values)| ("voltage", name.as_str(), values.as_deref()))
        .chain(
            result
                .branch_names
                .iter()
                .zip(&result.branch_currents)
                .map(|(name, values)| ("branch-current", name.as_str(), values.as_deref())),
        )
    {
        if values.is_some_and(|values| values.len() != point_count) {
            return Err(format!(
                "transient {kind} channel '{name}' does not match the time grid"
            ));
        }
    }
    for (kind, name, values) in result
        .device_op_traces
        .iter()
        .map(|trace| {
            (
                "device operating-point",
                trace.parameter.as_str(),
                trace.values.as_slice(),
            )
        })
        .chain(
            result
                .store_traces
                .iter()
                .map(|trace| ("device store", trace.name.as_str(), trace.values.as_slice())),
        )
    {
        if values.len() != point_count {
            return Err(format!(
                "transient {kind} channel '{name}' does not match the time grid"
            ));
        }
    }
    if !result.fft_results.is_empty() {
        return Err(
            "transient result contains FFT post-results; use the complete transient FFT DTO"
                .to_owned(),
        );
    }
    if result.compression.is_some() {
        return Err(
            "compressed transient provenance is not representable by this result-document version"
                .to_owned(),
        );
    }
    let mut document =
        AnalogResultDocument::new(AnalogAnalysisKind::Transient, "transient", ordinal);
    document.point_count = point_count;
    document.axes.push(AxisSeries {
        name: "time".to_owned(),
        unit: Some(SignalUnit::Second),
        values: result.time,
    });
    document.signals.push(real_signal(
        "integration_step".to_owned(),
        "integration step".to_owned(),
        AnalogSignalKind::Scalar,
        SignalOwner::Analysis,
        Some(SignalUnit::Second),
        result.step_sizes.into_iter().map(Some).collect(),
    ));
    for (name, values) in result.node_names.into_iter().zip(result.voltages) {
        document.signals.push(real_signal(
            format!("v({})", name.to_ascii_lowercase()),
            format!("V({name})"),
            AnalogSignalKind::Voltage,
            SignalOwner::Node { name },
            Some(SignalUnit::Volt),
            values.map_or_else(
                || vec![None; point_count],
                |values| values.into_iter().map(Some).collect(),
            ),
        ));
    }
    for (name, values) in result.branch_names.into_iter().zip(result.branch_currents) {
        document.signals.push(real_signal(
            format!("i({})", name.to_ascii_lowercase()),
            format!("I({name})"),
            AnalogSignalKind::BranchCurrent,
            SignalOwner::Branch { name },
            Some(SignalUnit::Ampere),
            values.map_or_else(
                || vec![None; point_count],
                |values| values.into_iter().map(Some).collect(),
            ),
        ));
    }
    for trace in result.device_op_traces {
        document.signals.push(real_signal(
            format!(
                "{}:{}",
                trace.device_name.to_ascii_lowercase(),
                trace.parameter.to_ascii_lowercase()
            ),
            format!("{}:{}", trace.device_name, trace.parameter),
            AnalogSignalKind::DeviceObservable,
            SignalOwner::Device {
                device: Some(trace.device_name),
                parameter: Some(trace.parameter),
                device_kind: None,
            },
            None,
            trace.values.into_iter().map(Some).collect(),
        ));
    }
    for trace in result.store_traces {
        document.signals.push(real_signal(
            trace.name.to_ascii_lowercase(),
            trace.name.clone(),
            AnalogSignalKind::DeviceObservable,
            signal_owner_for_observable(&trace.name),
            None,
            trace.values.into_iter().map(Some).collect(),
        ));
    }
    document.validate()?;
    Ok(document)
}

#[derive(Clone)]
struct RealSignalBuilder {
    display_name: String,
    kind: AnalogSignalKind,
    owner: SignalOwner,
    unit: Option<SignalUnit>,
    samples: Vec<Option<f64>>,
}

#[allow(clippy::too_many_arguments)]
fn insert_real_sample(
    builders: &mut BTreeMap<String, RealSignalBuilder>,
    point_index: usize,
    point_count: usize,
    canonical_name: String,
    display_name: String,
    kind: AnalogSignalKind,
    owner: SignalOwner,
    unit: Option<SignalUnit>,
    value: f64,
) {
    let builder = builders
        .entry(canonical_name)
        .or_insert_with(|| RealSignalBuilder {
            display_name,
            kind,
            owner,
            unit,
            samples: vec![None; point_count],
        });
    builder.samples[point_index] = Some(value);
}

#[allow(clippy::too_many_arguments)]
fn accumulate_real_sample(
    builders: &mut BTreeMap<String, RealSignalBuilder>,
    point_index: usize,
    point_count: usize,
    canonical_name: String,
    display_name: String,
    kind: AnalogSignalKind,
    owner: SignalOwner,
    unit: Option<SignalUnit>,
    value: f64,
) {
    let builder = builders
        .entry(canonical_name)
        .or_insert_with(|| RealSignalBuilder {
            display_name,
            kind,
            owner,
            unit,
            samples: vec![None; point_count],
        });
    let sample = builder.samples[point_index].get_or_insert(0.0);
    *sample += value;
}

pub fn dc_sweep_document(
    source_name: &str,
    points: Vec<DcSweepPointResult>,
    ordinal: usize,
) -> Result<AnalogResultDocument, String> {
    if points.is_empty() {
        return Err("DC sweep result has no points".to_owned());
    }
    let point_count = points.len();
    let mut builders = BTreeMap::<String, RealSignalBuilder>::new();
    let mut states = BTreeMap::<String, DeviceStateSeries>::new();
    for (point_index, point) in points.iter().enumerate() {
        let result = &point.result;
        validate_simulation_result(result, &format!("DC sweep point {point_index}"))?;
        for (name, value) in result.node_names.iter().zip(&result.node_voltages) {
            insert_real_sample(
                &mut builders,
                point_index,
                point_count,
                format!("v({})", name.to_ascii_lowercase()),
                format!("V({name})"),
                AnalogSignalKind::Voltage,
                SignalOwner::Node { name: name.clone() },
                Some(SignalUnit::Volt),
                *value,
            );
        }
        for (name, value) in result.branch_names.iter().zip(&result.branch_currents) {
            insert_real_sample(
                &mut builders,
                point_index,
                point_count,
                format!("i({})", name.to_ascii_lowercase()),
                format!("I({name})"),
                AnalogSignalKind::BranchCurrent,
                SignalOwner::Branch { name: name.clone() },
                Some(SignalUnit::Ampere),
                *value,
            );
        }
        for (name, value) in &result.dc_observables {
            insert_real_sample(
                &mut builders,
                point_index,
                point_count,
                name.to_ascii_lowercase(),
                name.clone(),
                AnalogSignalKind::DeviceObservable,
                signal_owner_for_observable(name),
                None,
                *value,
            );
        }
        for entry in &point.device_op_report.entries {
            let state = states
                .entry(entry.name.to_ascii_lowercase())
                .or_insert_with(|| DeviceStateSeries {
                    device_name: entry.name.clone(),
                    device_kind: Some(entry.device_kind.to_owned()),
                    regions: vec![None; point_count],
                });
            state.regions[point_index] = entry.region.map(str::to_owned);
        }
    }
    let mut document = AnalogResultDocument::new(AnalogAnalysisKind::DcSweep, "dc", ordinal);
    document.point_count = point_count;
    document.axes.push(AxisSeries {
        name: source_name.to_owned(),
        unit: None,
        values: points.iter().map(|point| point.sweep_value).collect(),
    });
    document.signals = builders
        .into_iter()
        .map(|(canonical_name, builder)| {
            real_signal(
                canonical_name,
                builder.display_name,
                builder.kind,
                builder.owner,
                builder.unit,
                builder.samples,
            )
        })
        .collect();
    document.device_states = states.into_values().collect();
    document.validate()?;
    Ok(document)
}

fn noise_type_name(kind: NoiseSourceType) -> &'static str {
    match kind {
        NoiseSourceType::Thermal => "thermal",
        NoiseSourceType::Shot => "shot",
        NoiseSourceType::Flicker => "flicker",
        NoiseSourceType::Burst => "burst",
        NoiseSourceType::White => "white",
        NoiseSourceType::Table => "table",
        NoiseSourceType::Bsim4Flicker => "bsim4_flicker",
        NoiseSourceType::Bsim3Flicker => "bsim3_flicker",
        NoiseSourceType::Bsim4CorrelatedThermal => "bsim4_correlated_thermal",
    }
}

pub fn noise_document(
    points: Vec<NoiseResult>,
    ordinal: usize,
) -> Result<AnalogResultDocument, String> {
    let first = points
        .first()
        .ok_or_else(|| "noise result has no points".to_owned())?;
    let point_count = points.len();
    if first.node_names.iter().any(|name| name.trim().is_empty())
        || first.branch_names.iter().any(|name| name.trim().is_empty())
    {
        return Err("noise result contains an empty signal name".to_owned());
    }
    for point in &points {
        if point.node_names != first.node_names
            || point.branch_names != first.branch_names
            || point.voltages.len() != first.node_names.len()
            || point.currents.len() != first.branch_names.len()
        {
            return Err("noise result schema changes across frequencies".to_owned());
        }
    }
    let mut document = AnalogResultDocument::new(AnalogAnalysisKind::Noise, "noise", ordinal);
    document.point_count = point_count;
    document.axes.push(AxisSeries {
        name: "frequency".to_owned(),
        unit: Some(SignalUnit::Hertz),
        values: points.iter().map(|point| point.frequency).collect(),
    });
    for (index, name) in first.node_names.iter().enumerate() {
        document.signals.push(complex_signal(
            format!("v({})", name.to_ascii_lowercase()),
            format!("V({name})"),
            AnalogSignalKind::Voltage,
            SignalOwner::Node { name: name.clone() },
            Some(SignalUnit::Volt),
            points
                .iter()
                .map(|point| {
                    let value = point.voltages[index];
                    Some(ComplexSample {
                        real: value.re,
                        imaginary: value.im,
                    })
                })
                .collect(),
        ));
    }
    for (index, name) in first.branch_names.iter().enumerate() {
        document.signals.push(complex_signal(
            format!("i({})", name.to_ascii_lowercase()),
            format!("I({name})"),
            AnalogSignalKind::BranchCurrent,
            SignalOwner::Branch { name: name.clone() },
            Some(SignalUnit::Ampere),
            points
                .iter()
                .map(|point| {
                    let value = point.currents[index];
                    Some(ComplexSample {
                        real: value.re,
                        imaginary: value.im,
                    })
                })
                .collect(),
        ));
    }
    for (canonical, display, unit, values) in [
        (
            "output_noise_density",
            "output noise density",
            Some(SignalUnit::VoltSquaredPerHertz),
            points
                .iter()
                .map(|point| Some(point.output_noise_density))
                .collect(),
        ),
        (
            "input_referred_density",
            "input-referred noise density",
            Some(SignalUnit::VoltSquaredPerHertz),
            points
                .iter()
                .map(|point| Some(point.input_referred_density))
                .collect(),
        ),
        (
            "input_gain_squared",
            "input gain squared",
            Some(SignalUnit::Dimensionless),
            points
                .iter()
                .map(|point| Some(point.input_gain_squared))
                .collect(),
        ),
    ] {
        document.signals.push(real_signal(
            canonical.to_owned(),
            display.to_owned(),
            AnalogSignalKind::Scalar,
            SignalOwner::Analysis,
            unit,
            values,
        ));
    }

    let mut contribution_builders = BTreeMap::<String, RealSignalBuilder>::new();
    for (point_index, point) in points.iter().enumerate() {
        for contribution in &point.contributions {
            let mechanism = contribution
                .identity
                .mechanism
                .as_deref()
                .unwrap_or("total");
            let base = format!(
                "noise({},{},{})",
                contribution.identity.device.to_ascii_lowercase(),
                mechanism.to_ascii_lowercase(),
                noise_type_name(contribution.noise_type)
            );
            for (suffix, display_suffix, unit, value) in [
                (
                    "output",
                    "output",
                    Some(SignalUnit::VoltSquaredPerHertz),
                    contribution.output_contribution,
                ),
                (
                    "input",
                    "input",
                    Some(SignalUnit::VoltSquaredPerHertz),
                    contribution.input_contribution,
                ),
                (
                    "percentage",
                    "percentage",
                    Some(SignalUnit::Dimensionless),
                    contribution.percentage,
                ),
            ] {
                accumulate_real_sample(
                    &mut contribution_builders,
                    point_index,
                    point_count,
                    format!("{base}:{suffix}"),
                    format!(
                        "{}:{} {} {}",
                        contribution.identity.device,
                        mechanism,
                        noise_type_name(contribution.noise_type),
                        display_suffix
                    ),
                    AnalogSignalKind::DeviceObservable,
                    SignalOwner::Device {
                        device: Some(contribution.identity.device.clone()),
                        parameter: Some(format!(
                            "{}:{}:{suffix}",
                            mechanism,
                            noise_type_name(contribution.noise_type)
                        )),
                        device_kind: None,
                    },
                    unit,
                    value,
                );
            }
        }
    }
    document
        .signals
        .extend(contribution_builders.into_iter().map(|(name, builder)| {
            real_signal(
                name,
                builder.display_name,
                builder.kind,
                builder.owner,
                builder.unit,
                builder.samples,
            )
        }));
    document.validate()?;
    Ok(document)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_version_round_trips_complex_values_and_missingness() {
        let document = AnalogResultDocument {
            schema: ANALOG_RESULT_SCHEMA.to_owned(),
            schema_version: ANALOG_RESULT_VERSION,
            analysis: AnalysisIdentity {
                id: "ac-002".to_owned(),
                kind: AnalogAnalysisKind::AcSmallSignal,
                request_kind: "ac".to_owned(),
                ordinal: 2,
            },
            coordinate_id: None,
            point_count: 2,
            axes: vec![AxisSeries {
                name: "frequency".to_owned(),
                unit: Some(SignalUnit::Hertz),
                values: vec![1.0, 10.0],
            }],
            signals: vec![complex_signal(
                "v(out)".to_owned(),
                "V(out)".to_owned(),
                AnalogSignalKind::Voltage,
                SignalOwner::Node {
                    name: "out".to_owned(),
                },
                Some(SignalUnit::Volt),
                vec![
                    Some(ComplexSample {
                        real: 1.0,
                        imaginary: -0.25,
                    }),
                    None,
                ],
            )],
            device_states: Vec::new(),
        };
        document.validate().unwrap();
        let encoded = serde_json::to_string(&document).unwrap();
        let decoded: AnalogResultDocument = serde_json::from_str(&encoded).unwrap();
        decoded.validate().unwrap();
        assert_eq!(decoded, document);

        let window = decoded.window(0, 2, 32).unwrap();
        let SignalWindowValues::Complex {
            real,
            imaginary,
            validity,
        } = &window.signals[0].values
        else {
            panic!("expected complex window");
        };
        assert_eq!(real, &[1.0, 0.0]);
        assert_eq!(imaginary, &[-0.25, 0.0]);
        assert_eq!(validity, &[1, 0]);
    }

    #[test]
    fn forward_versions_and_oversized_windows_fail_closed() {
        let mut document = AnalogResultDocument::new(AnalogAnalysisKind::OperatingPoint, "op", 1);
        document.point_count = 1;
        document.signals.push(real_signal(
            "v(0)".to_owned(),
            "V(0)".to_owned(),
            AnalogSignalKind::Voltage,
            SignalOwner::Node {
                name: "0".to_owned(),
            },
            Some(SignalUnit::Volt),
            vec![Some(0.0)],
        ));
        document.validate().unwrap();
        assert!(document.window(0, 1, 1).is_err());
        document.schema_version += 1;
        let encoded = serde_json::to_string(&document).unwrap();
        let decoded: AnalogResultDocument = serde_json::from_str(&encoded).unwrap();
        assert!(decoded.validate().unwrap_err().contains("unsupported"));
    }

    #[test]
    fn malformed_document_window_fails_before_slicing_short_columns() {
        let mut document = AnalogResultDocument::new(AnalogAnalysisKind::AcSmallSignal, "ac", 1);
        document.point_count = 2;
        document.axes.push(AxisSeries {
            name: "frequency".to_owned(),
            unit: Some(SignalUnit::Hertz),
            values: vec![1.0],
        });
        document.signals.push(complex_signal(
            "v(out)".to_owned(),
            "V(out)".to_owned(),
            AnalogSignalKind::Voltage,
            SignalOwner::Node {
                name: "out".to_owned(),
            },
            Some(SignalUnit::Volt),
            vec![Some(ComplexSample {
                real: 1.0,
                imaginary: 0.0,
            })],
        ));

        assert_eq!(
            document.window(0, 2, 64),
            Err("result document columns do not match point_count".to_owned())
        );
    }

    #[test]
    fn every_analysis_and_signal_enum_variant_has_a_stable_wire_tag() {
        let analysis_tags = AnalogAnalysisKind::ALL
            .iter()
            .map(|kind| serde_json::to_value(kind).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            analysis_tags,
            [
                "operating_point",
                "dc_sweep",
                "ac_small_signal",
                "transient",
                "noise",
            ]
            .map(serde_json::Value::from)
        );
        let signal_tags = AnalogSignalKind::ALL
            .iter()
            .map(|kind| serde_json::to_value(kind).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            signal_tags,
            ["voltage", "branch_current", "device_observable", "scalar",]
                .map(serde_json::Value::from)
        );
    }

    #[test]
    fn malformed_core_inventories_fail_before_zip_can_truncate_them() {
        let mut op = SimulationResult::new(1, 0);
        op.node_names.pop();
        assert!(
            operating_point_document(op.clone(), DeviceOpReport::default(), 1)
                .unwrap_err()
                .contains("node names")
        );
        let dc = DcSweepPointResult {
            sweep_value: 0.0,
            result: op,
            device_op_report: DeviceOpReport::default(),
        };
        assert!(
            dc_sweep_document("V1", vec![dc], 1)
                .unwrap_err()
                .contains("node names")
        );

        let malformed_ac = AcPointSnapshot {
            frequency: 1.0,
            node_names: vec!["out".to_owned()],
            branch_names: Vec::new(),
            voltages: crate::ComplexSeries {
                real: Vec::new(),
                imag: Vec::new(),
            },
            currents: crate::ComplexSeries {
                real: Vec::new(),
                imag: Vec::new(),
            },
        };
        assert!(ac_document(vec![malformed_ac], 1).is_err());

        let malformed_tran = TransientSnapshot {
            time: vec![0.0, 1.0],
            step_sizes: vec![0.0],
            num_nodes: 1,
            node_names: vec!["out".to_owned()],
            voltages: vec![Some(vec![0.0, 1.0])],
            branch_names: Vec::new(),
            branch_currents: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
            compression: None,
        };
        assert!(
            transient_document(malformed_tran, 1)
                .unwrap_err()
                .contains("step sizes")
        );

        let malformed_noise = NoiseResult {
            frequency: 1.0,
            node_names: vec!["out".to_owned()],
            branch_names: Vec::new(),
            voltages: Vec::new(),
            currents: Vec::new(),
            output_noise_density: 0.0,
            input_referred_density: 0.0,
            input_gain_squared: 1.0,
            contribution_catalog: Vec::new(),
            mechanisms_unavailable: Vec::new(),
            contributions: Vec::new(),
        };
        assert!(noise_document(vec![malformed_noise], 1).is_err());
    }
}
