//! Converting between the worker's mirror types and the core result model.
//!
//! Every conversion here is total in one direction and checked in the other:
//! going out to the worker cannot fail, but coming back must reject a payload
//! whose enum tags or lengths do not correspond to anything the core model
//! admits.  That is why these live together — the pair for each type has to be
//! read as one round trip.

use super::*;

impl From<TransferFunctionQuantity> for WorkerTransferFunctionQuantity {
    fn from(value: TransferFunctionQuantity) -> Self {
        match value {
            TransferFunctionQuantity::Voltage => Self::Voltage,
            TransferFunctionQuantity::Current => Self::Current,
        }
    }
}

impl From<WorkerTransferFunctionQuantity> for TransferFunctionQuantity {
    fn from(value: WorkerTransferFunctionQuantity) -> Self {
        match value {
            WorkerTransferFunctionQuantity::Voltage => Self::Voltage,
            WorkerTransferFunctionQuantity::Current => Self::Current,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerTransferFunctionScalar {
    Finite(f64),
    PositiveInfinity,
    NegativeInfinity,
}

impl From<TransferFunctionScalar> for WorkerTransferFunctionScalar {
    fn from(value: TransferFunctionScalar) -> Self {
        match value {
            TransferFunctionScalar::Finite(value) => Self::Finite(value),
            TransferFunctionScalar::PositiveInfinity => Self::PositiveInfinity,
            TransferFunctionScalar::NegativeInfinity => Self::NegativeInfinity,
        }
    }
}

impl From<WorkerTransferFunctionScalar> for TransferFunctionScalar {
    fn from(value: WorkerTransferFunctionScalar) -> Self {
        match value {
            WorkerTransferFunctionScalar::Finite(value) => Self::Finite(value),
            WorkerTransferFunctionScalar::PositiveInfinity => Self::PositiveInfinity,
            WorkerTransferFunctionScalar::NegativeInfinity => Self::NegativeInfinity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerNoiseSummary {
    pub rows: Vec<WorkerNoiseContributorRow>,
    #[serde(default)]
    pub total_rms: Option<f64>,
    #[serde(default)]
    pub input_rms: Option<f64>,
    pub band: (f64, f64),
}

#[cfg(test)]
impl WorkerNoiseSummary {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        sum_payload_bytes([
            self.rows
                .iter()
                .map(WorkerNoiseContributorRow::estimated_numeric_payload_bytes)
                .fold(0usize, |total, bytes| total.saturating_add(bytes)),
            f64_payload_bytes(3),
        ])
    }
}

impl From<NoiseSummary> for WorkerNoiseSummary {
    fn from(value: NoiseSummary) -> Self {
        Self {
            rows: value
                .rows
                .into_iter()
                .map(WorkerNoiseContributorRow::from)
                .collect(),
            total_rms: value.total_rms,
            input_rms: value.input_rms,
            band: value.band,
        }
    }
}

impl From<WorkerNoiseSummary> for NoiseSummary {
    fn from(value: WorkerNoiseSummary) -> Self {
        Self {
            rows: value
                .rows
                .into_iter()
                .map(NoiseContributorRow::from)
                .collect(),
            total_rms: value.total_rms,
            input_rms: value.input_rms,
            band: value.band,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerNoiseContributorRow {
    pub device: String,
    pub mechanism: String,
    pub power: f64,
    pub share_pct: f64,
}

#[cfg(test)]
impl WorkerNoiseContributorRow {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        f64_payload_bytes(2)
    }
}

impl From<NoiseContributorRow> for WorkerNoiseContributorRow {
    fn from(value: NoiseContributorRow) -> Self {
        Self {
            device: value.device,
            mechanism: value.mechanism.to_string(),
            power: value.power,
            share_pct: value.share_pct,
        }
    }
}

impl From<WorkerNoiseContributorRow> for NoiseContributorRow {
    fn from(value: WorkerNoiseContributorRow) -> Self {
        Self {
            device: value.device,
            mechanism: value.mechanism,
            power: value.power,
            share_pct: value.share_pct,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerMonteCarloVariable {
    pub name: String,
    pub samples: Vec<f64>,
    pub mean: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub histogram: Vec<usize>,
    pub bin_edges: Vec<f64>,
}

#[cfg(test)]
impl WorkerMonteCarloVariable {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        sum_payload_bytes([
            f64_payload_bytes(4usize.saturating_add(self.samples.len())),
            usize_payload_bytes(self.histogram.len()),
            f64_payload_bytes(self.bin_edges.len()),
        ])
    }
}

impl From<MonteCarloVariableResult> for WorkerMonteCarloVariable {
    fn from(value: MonteCarloVariableResult) -> Self {
        Self {
            name: value.name,
            samples: value.samples,
            mean: value.mean,
            std_dev: value.std_dev,
            min: value.min,
            max: value.max,
            histogram: value.histogram,
            bin_edges: value.bin_edges,
        }
    }
}

impl From<WorkerMonteCarloVariable> for MonteCarloVariableResult {
    fn from(value: WorkerMonteCarloVariable) -> Self {
        Self {
            name: value.name,
            samples: value.samples,
            mean: value.mean,
            std_dev: value.std_dev,
            min: value.min,
            max: value.max,
            histogram: value.histogram,
            bin_edges: value.bin_edges,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerReliabilityResult {
    pub device_id: String,
    pub stress: WorkerStressMetrics,
    pub shifts: HashMap<String, WorkerParamShift>,
}

#[cfg(test)]
impl WorkerReliabilityResult {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        sum_payload_bytes([
            self.stress.estimated_numeric_payload_bytes(),
            self.shifts
                .values()
                .map(WorkerParamShift::estimated_numeric_payload_bytes)
                .fold(0usize, |total, bytes| total.saturating_add(bytes)),
        ])
    }
}

impl From<ReliabilityResult> for WorkerReliabilityResult {
    fn from(value: ReliabilityResult) -> Self {
        Self {
            device_id: value.device_id,
            stress: WorkerStressMetrics::from(value.stress),
            shifts: value
                .shifts
                .into_iter()
                .map(|(label, shift)| (label, WorkerParamShift::from(shift)))
                .collect(),
        }
    }
}

impl From<WorkerReliabilityResult> for ReliabilityResult {
    fn from(value: WorkerReliabilityResult) -> Self {
        Self {
            device_id: value.device_id,
            stress: StressMetrics::from(value.stress),
            shifts: value
                .shifts
                .into_iter()
                .map(|(label, shift)| (label, ParamShift::from(shift)))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerStressMetrics {
    pub avg_vgs_stress: f64,
    pub avg_vds_stress: f64,
    pub avg_temp: f64,
    pub duration: f64,
}

#[cfg(test)]
impl WorkerStressMetrics {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        f64_payload_bytes(4)
    }
}

impl From<StressMetrics> for WorkerStressMetrics {
    fn from(value: StressMetrics) -> Self {
        Self {
            avg_vgs_stress: value.avg_vgs_stress,
            avg_vds_stress: value.avg_vds_stress,
            avg_temp: value.avg_temp,
            duration: value.duration,
        }
    }
}

impl From<WorkerStressMetrics> for StressMetrics {
    fn from(value: WorkerStressMetrics) -> Self {
        Self {
            avg_vgs_stress: value.avg_vgs_stress,
            avg_vds_stress: value.avg_vds_stress,
            avg_temp: value.avg_temp,
            duration: value.duration,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerParamShift {
    pub vth_shift: f64,
    pub mobility_shift: f64,
    pub rds_shift: f64,
}

#[cfg(test)]
impl WorkerParamShift {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        f64_payload_bytes(3)
    }
}

impl From<ParamShift> for WorkerParamShift {
    fn from(value: ParamShift) -> Self {
        Self {
            vth_shift: value.vth_shift,
            mobility_shift: value.mobility_shift,
            rds_shift: value.rds_shift,
        }
    }
}

impl From<WorkerParamShift> for ParamShift {
    fn from(value: WorkerParamShift) -> Self {
        Self {
            vth_shift: value.vth_shift,
            mobility_shift: value.mobility_shift,
            rds_shift: value.rds_shift,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerSoAEvaluation {
    pub device_id: String,
    pub parameter: WorkerSoAParameter,
    pub limit_value: f64,
    pub worst_actual_value: f64,
    pub worst_time: f64,
    pub sample_count: u64,
    pub unit: String,
    pub description: String,
    pub verdict: WorkerSoARuleVerdict,
}

#[cfg(test)]
impl WorkerSoAEvaluation {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        f64_payload_bytes(3).saturating_add(std::mem::size_of::<u64>())
    }
}

impl From<SoAEvaluation> for WorkerSoAEvaluation {
    fn from(value: SoAEvaluation) -> Self {
        Self {
            device_id: value.device_id,
            parameter: WorkerSoAParameter::from(value.parameter),
            limit_value: value.limit_value,
            worst_actual_value: value.worst_actual_value,
            worst_time: value.worst_time,
            sample_count: value.sample_count,
            unit: value.unit,
            description: value.description,
            verdict: WorkerSoARuleVerdict::from(value.verdict),
        }
    }
}

impl From<WorkerSoAEvaluation> for SoAEvaluation {
    fn from(value: WorkerSoAEvaluation) -> Self {
        Self {
            device_id: value.device_id,
            parameter: SoAParameter::from(value.parameter),
            limit_value: value.limit_value,
            worst_actual_value: value.worst_actual_value,
            worst_time: value.worst_time,
            sample_count: value.sample_count,
            unit: value.unit,
            description: value.description,
            verdict: SoARuleVerdict::from(value.verdict),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum WorkerSoARuleVerdict {
    Pass,
    Warning,
    Violation,
    Critical,
}

impl From<SoARuleVerdict> for WorkerSoARuleVerdict {
    fn from(value: SoARuleVerdict) -> Self {
        match value {
            SoARuleVerdict::Pass => Self::Pass,
            SoARuleVerdict::Warning => Self::Warning,
            SoARuleVerdict::Violation => Self::Violation,
            SoARuleVerdict::Critical => Self::Critical,
        }
    }
}

impl From<WorkerSoARuleVerdict> for SoARuleVerdict {
    fn from(value: WorkerSoARuleVerdict) -> Self {
        match value {
            WorkerSoARuleVerdict::Pass => Self::Pass,
            WorkerSoARuleVerdict::Warning => Self::Warning,
            WorkerSoARuleVerdict::Violation => Self::Violation,
            WorkerSoARuleVerdict::Critical => Self::Critical,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerSoAViolation {
    pub device_id: String,
    pub parameter: WorkerSoAParameter,
    pub limit_value: f64,
    pub actual_value: f64,
    pub time: f64,
    pub severity: WorkerViolationSeverity,
}

#[cfg(test)]
impl WorkerSoAViolation {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        f64_payload_bytes(3)
    }
}

impl From<SoAViolation> for WorkerSoAViolation {
    fn from(value: SoAViolation) -> Self {
        Self {
            device_id: value.device_id,
            parameter: WorkerSoAParameter::from(value.parameter),
            limit_value: value.limit_value,
            actual_value: value.actual_value,
            time: value.time,
            severity: WorkerViolationSeverity::from(value.severity),
        }
    }
}

impl From<WorkerSoAViolation> for SoAViolation {
    fn from(value: WorkerSoAViolation) -> Self {
        Self {
            device_id: value.device_id,
            parameter: SoAParameter::from(value.parameter),
            limit_value: value.limit_value,
            actual_value: value.actual_value,
            time: value.time,
            severity: ViolationSeverity::from(value.severity),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum WorkerSoAParameter {
    Vgs,
    Vds,
    Vgd,
    Vbe,
    Vce,
    Vbc,
    Id,
    Ic,
    Pdiss,
    Temp,
}

impl From<SoAParameter> for WorkerSoAParameter {
    fn from(value: SoAParameter) -> Self {
        match value {
            SoAParameter::Vgs => Self::Vgs,
            SoAParameter::Vds => Self::Vds,
            SoAParameter::Vgd => Self::Vgd,
            SoAParameter::Vbe => Self::Vbe,
            SoAParameter::Vce => Self::Vce,
            SoAParameter::Vbc => Self::Vbc,
            SoAParameter::Id => Self::Id,
            SoAParameter::Ic => Self::Ic,
            SoAParameter::Pdiss => Self::Pdiss,
            SoAParameter::Temp => Self::Temp,
        }
    }
}

impl From<WorkerSoAParameter> for SoAParameter {
    fn from(value: WorkerSoAParameter) -> Self {
        match value {
            WorkerSoAParameter::Vgs => Self::Vgs,
            WorkerSoAParameter::Vds => Self::Vds,
            WorkerSoAParameter::Vgd => Self::Vgd,
            WorkerSoAParameter::Vbe => Self::Vbe,
            WorkerSoAParameter::Vce => Self::Vce,
            WorkerSoAParameter::Vbc => Self::Vbc,
            WorkerSoAParameter::Id => Self::Id,
            WorkerSoAParameter::Ic => Self::Ic,
            WorkerSoAParameter::Pdiss => Self::Pdiss,
            WorkerSoAParameter::Temp => Self::Temp,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum WorkerViolationSeverity {
    Warning,
    Violation,
    Critical,
}

impl From<ViolationSeverity> for WorkerViolationSeverity {
    fn from(value: ViolationSeverity) -> Self {
        match value {
            ViolationSeverity::Warning => Self::Warning,
            ViolationSeverity::Violation => Self::Violation,
            ViolationSeverity::Critical => Self::Critical,
        }
    }
}

impl From<WorkerViolationSeverity> for ViolationSeverity {
    fn from(value: WorkerViolationSeverity) -> Self {
        match value {
            WorkerViolationSeverity::Warning => Self::Warning,
            WorkerViolationSeverity::Violation => Self::Violation,
            WorkerViolationSeverity::Critical => Self::Critical,
        }
    }
}

/// One committed digital event, as it crosses the worker edge.
///
/// Points are transported whole rather than as parallel time/value arrays:
/// an event history is short enough that the buffer split buys nothing, and
/// paired arrays can arrive with different lengths.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerDigitalEventPoint {
    pub time_s: f64,
    pub value_code: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerRealEventPoint {
    pub time_s: f64,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerDigitalEventTrace {
    pub node_name: String,
    pub points: Vec<WorkerDigitalEventPoint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerRealEventTrace {
    pub node_name: String,
    pub points: Vec<WorkerRealEventPoint>,
}

/// Every event node a transient run committed, on the wire.
///
/// Both fields default so a worker built before event transport still
/// deserializes — it simply reports no events, which is the truth for it.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerEventHistory {
    #[serde(default)]
    pub digital: Vec<WorkerDigitalEventTrace>,
    #[serde(default)]
    pub real: Vec<WorkerRealEventTrace>,
}

impl From<TransientEventHistory> for WorkerEventHistory {
    fn from(value: TransientEventHistory) -> Self {
        Self {
            digital: value
                .digital
                .into_iter()
                .map(|trace| WorkerDigitalEventTrace {
                    node_name: trace.node_name,
                    points: trace
                        .points
                        .into_iter()
                        .map(|point| WorkerDigitalEventPoint {
                            time_s: point.time_s,
                            value_code: point.value_code,
                        })
                        .collect(),
                })
                .collect(),
            real: value
                .real
                .into_iter()
                .map(|trace| WorkerRealEventTrace {
                    node_name: trace.node_name,
                    points: trace
                        .points
                        .into_iter()
                        .map(|point| WorkerRealEventPoint {
                            time_s: point.time_s,
                            value: point.value,
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}

impl From<WorkerEventHistory> for TransientEventHistory {
    fn from(value: WorkerEventHistory) -> Self {
        Self {
            digital: value
                .digital
                .into_iter()
                .map(|trace| EventNodeHistory {
                    node_name: trace.node_name,
                    points: trace
                        .points
                        .into_iter()
                        .map(|point| DigitalEventPoint {
                            time_s: point.time_s,
                            value_code: point.value_code,
                        })
                        .collect(),
                })
                .collect(),
            real: value
                .real
                .into_iter()
                .map(|trace| EventNodeHistory {
                    node_name: trace.node_name,
                    points: trace
                        .points
                        .into_iter()
                        .map(|point| RealEventPoint {
                            time_s: point.time_s,
                            value: point.value,
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerWaveform {
    pub name: String,
    pub x_values: Vec<f64>,
    pub y_values: Vec<f64>,
    pub y_unit: String,
    pub is_complex: bool,
    pub y_imag: Option<Vec<f64>>,
}

#[cfg(test)]
impl WorkerWaveform {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        sum_payload_bytes([
            f64_payload_bytes(self.x_values.len()),
            f64_payload_bytes(self.y_values.len()),
            self.y_imag
                .as_ref()
                .map_or(0, |values| f64_payload_bytes(values.len())),
        ])
    }
}

impl From<WaveformData> for WorkerWaveform {
    fn from(value: WaveformData) -> Self {
        Self {
            name: value.name,
            x_values: value.x_values,
            y_values: value.y_values,
            y_unit: value.y_unit,
            is_complex: value.is_complex,
            y_imag: value.y_imag,
        }
    }
}

impl From<WorkerWaveform> for WaveformData {
    fn from(value: WorkerWaveform) -> Self {
        Self {
            name: value.name,
            x_values: value.x_values,
            y_values: value.y_values,
            y_unit: value.y_unit,
            is_complex: value.is_complex,
            y_imag: value.y_imag,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerMeasurement {
    pub name: String,
    pub value: Option<f64>,
    pub error: Option<String>,
    pub passed: bool,
    pub expected: Option<f64>,
    pub tolerance: Option<f64>,
    pub event_axis: Option<f64>,
}

#[cfg(test)]
impl WorkerMeasurement {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        f64_payload_bytes(
            usize::from(self.value.is_some())
                + usize::from(self.expected.is_some())
                + usize::from(self.tolerance.is_some())
                + usize::from(self.event_axis.is_some()),
        )
    }
}

impl From<rspice_core::MeasureResult> for WorkerMeasurement {
    fn from(value: rspice_core::MeasureResult) -> Self {
        Self {
            name: value.name,
            value: value.value,
            error: value.error,
            passed: value.passed,
            expected: value.expected,
            tolerance: value.tolerance,
            event_axis: value.event_axis,
        }
    }
}

impl From<WorkerMeasurement> for rspice_core::MeasureResult {
    fn from(value: WorkerMeasurement) -> Self {
        Self {
            name: value.name,
            value: value.value,
            error: value.error,
            passed: value.passed,
            expected: value.expected,
            tolerance: value.tolerance,
            event_axis: value.event_axis,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerDeviceOpReport {
    pub entries: Vec<WorkerDeviceOpEntry>,
}

#[cfg(test)]
impl WorkerDeviceOpReport {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        self.entries
            .iter()
            .map(WorkerDeviceOpEntry::estimated_numeric_payload_bytes)
            .fold(0usize, |total, bytes| total.saturating_add(bytes))
    }
}

impl From<rspice_core::circuit::DeviceOpReport> for WorkerDeviceOpReport {
    fn from(value: rspice_core::circuit::DeviceOpReport) -> Self {
        Self {
            entries: value
                .entries
                .into_iter()
                .map(WorkerDeviceOpEntry::from)
                .collect(),
        }
    }
}

impl From<WorkerDeviceOpReport> for rspice_core::circuit::DeviceOpReport {
    fn from(value: WorkerDeviceOpReport) -> Self {
        Self {
            entries: value
                .entries
                .into_iter()
                .map(rspice_core::circuit::DeviceOpEntry::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerDeviceOpEntry {
    pub name: String,
    pub device_kind: String,
    pub region: Option<String>,
    pub params: Vec<WorkerNamedValue>,
}

#[cfg(test)]
impl WorkerDeviceOpEntry {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        self.params
            .iter()
            .map(WorkerNamedValue::estimated_numeric_payload_bytes)
            .fold(0usize, |total, bytes| total.saturating_add(bytes))
    }
}

impl From<rspice_core::circuit::DeviceOpEntry> for WorkerDeviceOpEntry {
    fn from(value: rspice_core::circuit::DeviceOpEntry) -> Self {
        Self {
            name: value.name,
            device_kind: value.device_kind.to_string(),
            region: value.region.map(str::to_string),
            params: value
                .params
                .into_iter()
                .map(|(name, value)| WorkerNamedValue {
                    name: name.to_string(),
                    value,
                })
                .collect(),
        }
    }
}

impl From<WorkerDeviceOpEntry> for rspice_core::circuit::DeviceOpEntry {
    fn from(value: WorkerDeviceOpEntry) -> Self {
        Self {
            name: value.name,
            device_kind: intern_static_label(value.device_kind),
            region: value.region.map(intern_static_label),
            params: value
                .params
                .into_iter()
                .map(|param| (intern_static_label(param.name), param.value))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerNamedValue {
    pub name: String,
    pub value: f64,
}

#[cfg(test)]
impl WorkerNamedValue {
    pub(super) fn estimated_numeric_payload_bytes(&self) -> usize {
        f64_payload_bytes(1)
    }
}

#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
pub(crate) fn worker_response_from_request(request: WorkerRequest) -> WorkerResponse {
    worker_response_from_request_with_progress(request, None)
}

pub(super) fn worker_response_from_request_with_progress(
    request: WorkerRequest,
    progress_observer: Option<super::super::ProgressObserver>,
) -> WorkerResponse {
    let id = request.id;
    let (request, input) = request.into_runner_parts();
    let progress = Arc::new(Mutex::new(SimulationProgress::default()));
    let abort_flag = Arc::new(AtomicBool::new(false));

    WorkerResponse::from_result_for_transfer(
        id,
        super::super::run_simulation_thread_with_progress_observer(
            request,
            input,
            progress,
            abort_flag,
            progress_observer,
        ),
    )
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static ACTIVE_WORKER_PROGRESS_ID: std::cell::Cell<Option<u64>> = const { std::cell::Cell::new(None) };
    static PENDING_WASM_JIT_REQUEST: std::cell::RefCell<Option<(u32, WorkerRequest)>> = const { std::cell::RefCell::new(None) };
    static NEXT_WASM_JIT_DISPATCH_TOKEN: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

#[cfg(target_arch = "wasm32")]
pub(super) fn emit_worker_progress_snapshot(progress: &SimulationProgress) {
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::JsValue;

    let id = ACTIVE_WORKER_PROGRESS_ID.with(|active| active.get());
    let Some(id) = id else {
        return;
    };

    let snapshot = WorkerProgressSnapshot::from_progress(id, progress);
    let message = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &message,
        &JsValue::from_str("type"),
        &JsValue::from_str("progress"),
    );
    let _ = js_sys::Reflect::set(
        &message,
        &JsValue::from_str("id"),
        &JsValue::from_f64(id as f64),
    );
    if let Ok(snapshot) = serde_wasm_bindgen::to_value(&snapshot) {
        let _ = js_sys::Reflect::set(&message, &JsValue::from_str("progress"), &snapshot);
    }

    let global = js_sys::global();
    let Ok(post_message) = js_sys::Reflect::get(&global, &JsValue::from_str("postMessage"))
        .and_then(|value| value.dyn_into::<js_sys::Function>())
    else {
        return;
    };
    let _ = post_message.call1(&global, &JsValue::from(message));
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn run_worker_request_value(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let request = worker_request_from_value(value)?;
    run_decoded_worker_request(request)
}

#[cfg(target_arch = "wasm32")]
fn run_decoded_worker_request(
    request: WorkerRequest,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let id = request.id;
    ACTIVE_WORKER_PROGRESS_ID.with(|active| active.set(Some(id)));
    let response =
        worker_response_from_request_with_progress(request, Some(emit_worker_progress_snapshot));
    ACTIVE_WORKER_PROGRESS_ID.with(|active| active.set(None));
    worker_response_transport_value(response)
}

#[cfg(target_arch = "wasm32")]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct WasmJitRequestPreparation {
    dispatch_token: u32,
    artifacts: Vec<crate::simulation::veriloga::WasmJitWorkerArtifact>,
    errors: Vec<String>,
}

/// Compile every sealed Verilog-A runtime required by a simulation request
/// before the synchronous solver begins. The JavaScript worker installs these
/// modules into its persistent, capability-limited instance cache.
#[cfg(target_arch = "wasm32")]
pub(crate) fn prepare_wasm_jit_request_value(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    if PENDING_WASM_JIT_REQUEST.with(|pending| pending.borrow().is_some()) {
        return Err(wasm_bindgen::JsValue::from_str(
            "a prepared browser simulation request is already pending",
        ));
    }
    let request = worker_request_from_value(value)?;
    let dispatch_token = NEXT_WASM_JIT_DISPATCH_TOKEN.with(|next| {
        let token = next.get().wrapping_add(1).max(1);
        next.set(token);
        token
    });
    let mut preparation = WasmJitRequestPreparation {
        dispatch_token,
        artifacts: Vec::with_capacity(request.project_veriloga_runtimes.iter().len()),
        errors: Vec::new(),
    };
    for runtime in request.project_veriloga_runtimes.iter() {
        match runtime.compile_wasm_jit_artifact() {
            Ok(artifact) => preparation.artifacts.push(artifact),
            Err(error) => preparation.errors.push(format!(
                "Verilog-A runtime '{}' could not qualify for the browser JIT: {error}",
                runtime.netlist_alias()
            )),
        }
    }
    let value = serde_wasm_bindgen::to_value(&preparation)
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))?;
    PENDING_WASM_JIT_REQUEST.with(|pending| {
        *pending.borrow_mut() = Some((dispatch_token, request));
    });
    Ok(value)
}

/// Consume exactly the request decoded by `prepare_wasm_jit_request_value`.
/// This avoids a second copy of every transferred numerical dependency.
#[cfg(target_arch = "wasm32")]
pub(crate) fn run_prepared_wasm_jit_request_value(
    dispatch_token: u32,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let request = PENDING_WASM_JIT_REQUEST.with(|pending| {
        let mut pending = pending.borrow_mut();
        let Some((expected_token, _)) = pending.as_ref() else {
            return Err(wasm_bindgen::JsValue::from_str(
                "no prepared browser simulation request is pending",
            ));
        };
        if dispatch_token == 0 || dispatch_token != *expected_token {
            return Err(wasm_bindgen::JsValue::from_str(
                "stale browser simulation dispatch token",
            ));
        }
        Ok(pending
            .take()
            .expect("validated prepared request must remain present")
            .1)
    })?;
    run_decoded_worker_request(request)
}

/// Discard a prepared request only when the caller presents its exact token.
///
/// This closes the one failure path between request decoding and synchronous
/// dispatch without allowing a stale JavaScript caller to cancel newer work.
#[cfg(target_arch = "wasm32")]
pub(crate) fn cancel_prepared_wasm_jit_request_value(
    dispatch_token: u32,
) -> Result<(), wasm_bindgen::JsValue> {
    PENDING_WASM_JIT_REQUEST.with(|pending| {
        let mut pending = pending.borrow_mut();
        let Some((expected_token, _)) = pending.as_ref() else {
            return Err(wasm_bindgen::JsValue::from_str(
                "no prepared browser simulation request is pending",
            ));
        };
        if dispatch_token == 0 || dispatch_token != *expected_token {
            return Err(wasm_bindgen::JsValue::from_str(
                "stale browser simulation dispatch token",
            ));
        }
        pending.take();
        Ok(())
    })
}

#[cfg(target_arch = "wasm32")]
pub(super) fn worker_request_from_value(
    value: wasm_bindgen::JsValue,
) -> Result<WorkerRequest, wasm_bindgen::JsValue> {
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::JsValue;

    let protocol = js_sys::Reflect::get(&value, &JsValue::from_str("protocolVersion"))
        .map_err(worker_request_js_error)?
        .as_f64()
        .and_then(|value| {
            (value.fract() == 0.0 && (0.0..=f64::from(u8::MAX)).contains(&value))
                .then_some(value as u8)
        })
        .ok_or_else(|| {
            JsValue::from_str("worker request transport protocolVersion must be an unsigned byte")
        })?;

    let request = js_sys::Reflect::get(&value, &JsValue::from_str("request"))
        .map_err(worker_request_js_error)?;
    let request = serde_wasm_bindgen::from_value::<WorkerRequestTransportMetadata>(request)
        .map_err(|error| JsValue::from_str(&error.to_string()))?;

    let buffers = js_sys::Reflect::get(&value, &JsValue::from_str("buffers"))
        .map_err(worker_request_js_error)?
        .dyn_into::<js_sys::Array>()
        .map_err(|_| JsValue::from_str("worker request transport buffers must be an array"))?;
    let buffer_count = buffers.length() as usize;
    if buffer_count > MAX_WORKER_TRANSFER_BUFFERS {
        return Err(JsValue::from_str(&format!(
            "worker request contains {buffer_count} transfer buffers, exceeding the {MAX_WORKER_TRANSFER_BUFFERS}-buffer limit"
        )));
    }
    let mut numeric_values = 0usize;
    for index in 0..buffers.length() {
        let view = buffers
            .get(index)
            .dyn_into::<js_sys::Float64Array>()
            .map_err(|_| {
                JsValue::from_str(&format!(
                    "worker request transport buffer {index} is not a Float64Array"
                ))
            })?;
        numeric_values = checked_worker_request_numeric_total(
            numeric_values,
            index as usize,
            view.length() as usize,
        )
        .map_err(|error| JsValue::from_str(&error))?;
    }

    let mut decoded_buffers = Vec::with_capacity(buffer_count);
    for index in 0..buffers.length() {
        let view = buffers
            .get(index)
            .dyn_into::<js_sys::Float64Array>()
            .map_err(|_| {
                JsValue::from_str(&format!(
                    "worker request transport buffer {index} is not a Float64Array"
                ))
            })?;
        let mut values = vec![0.0; view.length() as usize];
        view.copy_to(&mut values);
        decoded_buffers.push(values);
    }

    WorkerRequestTransport {
        protocol,
        request,
        buffers: decoded_buffers,
    }
    .into_request()
    .map_err(|error| JsValue::from_str(&error))
}

#[cfg(target_arch = "wasm32")]
pub(super) fn worker_request_js_error(error: wasm_bindgen::JsValue) -> wasm_bindgen::JsValue {
    wasm_bindgen::JsValue::from_str(&worker_js_error(error).to_string())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn worker_response_transport_value(
    response: WorkerResponse,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    use wasm_bindgen::JsValue;

    let transport = WorkerResponseTransport::from_response(response)
        .map_err(|error| JsValue::from_str(&error))?;
    let message = js_sys::Object::new();
    js_sys::Reflect::set(
        &message,
        &JsValue::from_str("protocolVersion"),
        &JsValue::from_f64(f64::from(transport.protocol)),
    )?;
    let response = serde_wasm_bindgen::to_value(&transport.response)
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    js_sys::Reflect::set(&message, &JsValue::from_str("response"), &response)?;

    let buffers = js_sys::Array::new();
    for values in transport.buffers {
        let view = js_sys::Float64Array::new_with_length(values.len() as u32);
        view.copy_from(&values);
        buffers.push(&view);
    }
    js_sys::Reflect::set(&message, &JsValue::from_str("buffers"), &buffers)?;

    Ok(JsValue::from(message))
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn worker_response_from_value(
    value: wasm_bindgen::JsValue,
) -> Result<WorkerResponse, SimulationError> {
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::JsValue;

    let protocol = js_sys::Reflect::get(&value, &JsValue::from_str("protocolVersion"))
        .ok()
        .and_then(|value| value.as_f64())
        .map(|value| value as u8);

    if protocol != Some(WORKER_RESPONSE_TRANSPORT_PROTOCOL) {
        return serde_wasm_bindgen::from_value::<WorkerResponse>(value)
            .map_err(|error| SimulationError::InvalidConfig(error.to_string()));
    }

    let response =
        js_sys::Reflect::get(&value, &JsValue::from_str("response")).map_err(worker_js_error)?;
    let response = serde_wasm_bindgen::from_value::<WorkerResponseTransportMetadata>(response)
        .map_err(|error| SimulationError::InvalidConfig(error.to_string()))?;

    let buffers = js_sys::Reflect::get(&value, &JsValue::from_str("buffers"))
        .map_err(worker_js_error)?
        .dyn_into::<js_sys::Array>()
        .map_err(|_| {
            SimulationError::InvalidConfig(
                "worker response transport buffers must be an array".to_string(),
            )
        })?;

    let buffer_count = buffers.length() as usize;
    if buffer_count > MAX_WORKER_TRANSFER_BUFFERS {
        return Err(SimulationError::InvalidConfig(format!(
            "worker response contains {buffer_count} transfer buffers, exceeding the {MAX_WORKER_TRANSFER_BUFFERS}-buffer limit"
        )));
    }
    let mut numeric_values = 0usize;
    for index in 0..buffers.length() {
        let view = buffers
            .get(index)
            .dyn_into::<js_sys::Float64Array>()
            .map_err(|_| {
                SimulationError::InvalidConfig(format!(
                    "worker response transport buffer {index} is not a Float64Array"
                ))
            })?;
        numeric_values = numeric_values
            .checked_add(view.length() as usize)
            .ok_or_else(|| {
                SimulationError::InvalidConfig(
                    "worker response numeric size overflows this platform".to_owned(),
                )
            })?;
        if numeric_values > MAX_WORKER_F64_VALUES {
            return Err(SimulationError::InvalidConfig(format!(
                "worker response contains more than {MAX_WORKER_F64_VALUES} numerical values"
            )));
        }
    }

    let mut decoded_buffers = Vec::with_capacity(buffer_count);
    for index in 0..buffers.length() {
        let view = buffers
            .get(index)
            .dyn_into::<js_sys::Float64Array>()
            .map_err(|_| {
                SimulationError::InvalidConfig(format!(
                    "worker response transport buffer {index} is not a Float64Array"
                ))
            })?;
        let mut values = vec![0.0; view.length() as usize];
        view.copy_to(&mut values);
        decoded_buffers.push(values);
    }

    WorkerResponseTransport {
        protocol: WORKER_RESPONSE_TRANSPORT_PROTOCOL,
        response,
        buffers: decoded_buffers,
    }
    .into_response()
    .map_err(SimulationError::InvalidConfig)
}

#[cfg(target_arch = "wasm32")]
pub(super) fn worker_js_error(error: wasm_bindgen::JsValue) -> SimulationError {
    use wasm_bindgen::JsValue;

    let message = error
        .as_string()
        .or_else(|| {
            js_sys::Reflect::get(&error, &JsValue::from_str("message"))
                .ok()
                .and_then(|message| message.as_string())
        })
        .unwrap_or_else(|| "unknown JavaScript error".to_string());
    SimulationError::InvalidConfig(message)
}

#[cfg(test)]
pub(super) fn sum_payload_bytes(bytes: impl IntoIterator<Item = usize>) -> usize {
    bytes
        .into_iter()
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
pub(super) fn f64_payload_bytes(len: usize) -> usize {
    len.saturating_mul(std::mem::size_of::<f64>())
}

#[cfg(test)]
pub(super) fn pss_operating_point_payload_bytes(
    operating_point: &rspice_core::engine::PssOperatingPoint,
) -> usize {
    let analysis = operating_point.analysis();
    let values = analysis
        .result
        .time
        .len()
        .saturating_add(
            analysis
                .result
                .waveforms
                .iter()
                .map(|waveform| waveform.values.len())
                .sum::<usize>(),
        )
        .saturating_add(analysis.monodromy.iter().map(Vec::len).sum::<usize>())
        .saturating_add(analysis.result.floquet_multipliers.len().saturating_mul(2))
        .saturating_add(analysis.floquet_multipliers.len().saturating_mul(2))
        .saturating_add(operating_point.shooting_state().len());
    f64_payload_bytes(values)
}

#[cfg(test)]
pub(super) fn usize_payload_bytes(len: usize) -> usize {
    len.saturating_mul(std::mem::size_of::<usize>())
}

#[cfg(test)]
pub(super) fn complex_pair_payload_bytes(len: usize) -> usize {
    len.saturating_mul(2)
        .saturating_mul(std::mem::size_of::<f64>())
}

#[cfg(test)]
pub(super) fn event_history_payload_bytes(events: &WorkerEventHistory) -> usize {
    let digital = events
        .digital
        .iter()
        .map(|trace| f64_payload_bytes(trace.points.len()).saturating_add(trace.points.len()))
        .fold(0usize, |total, bytes| total.saturating_add(bytes));
    events
        .real
        .iter()
        .map(|trace| f64_payload_bytes(trace.points.len().saturating_mul(2)))
        .fold(digital, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
pub(super) fn waveforms_payload_bytes(waveforms: &[WorkerWaveform]) -> usize {
    waveforms
        .iter()
        .map(WorkerWaveform::estimated_numeric_payload_bytes)
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
pub(super) fn measurements_payload_bytes(measurements: &[WorkerMeasurement]) -> usize {
    measurements
        .iter()
        .map(WorkerMeasurement::estimated_numeric_payload_bytes)
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
pub(super) fn vec_map_payload_bytes(values_by_name: &HashMap<String, Vec<f64>>) -> usize {
    values_by_name
        .values()
        .map(|values| f64_payload_bytes(values.len()))
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
pub(super) fn reliability_results_payload_bytes(results: &[WorkerReliabilityResult]) -> usize {
    results
        .iter()
        .map(WorkerReliabilityResult::estimated_numeric_payload_bytes)
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
pub(super) fn soa_violations_payload_bytes(violations: &[WorkerSoAViolation]) -> usize {
    violations
        .iter()
        .map(WorkerSoAViolation::estimated_numeric_payload_bytes)
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

#[cfg(test)]
pub(super) fn soa_evaluations_payload_bytes(evaluations: &[WorkerSoAEvaluation]) -> usize {
    evaluations
        .iter()
        .map(WorkerSoAEvaluation::estimated_numeric_payload_bytes)
        .fold(0usize, |total, bytes| total.saturating_add(bytes))
}

pub(super) fn worker_waveforms(waveforms: HashMap<String, WaveformData>) -> Vec<WorkerWaveform> {
    let mut waveforms: Vec<_> = waveforms.into_values().map(WorkerWaveform::from).collect();
    waveforms.sort_by(|left, right| left.name.cmp(&right.name));
    waveforms
}

pub(super) fn validate_pss_display_contract(
    time: &[f64],
    waveforms: &HashMap<String, WaveformData>,
    operating_point: &rspice_core::engine::PssOperatingPoint,
) -> Result<(), SimulationError> {
    let result = &operating_point.analysis().result;
    if time != result.time.as_slice() {
        return Err(SimulationError::InvalidConfig(
            "PSS display time axis does not match its retained numerical orbit".to_owned(),
        ));
    }
    let expected_count = result
        .node_names
        .iter()
        .filter(|name| name.as_str() != "0" && !name.eq_ignore_ascii_case("gnd"))
        .count();
    if waveforms.len() != expected_count {
        return Err(SimulationError::InvalidConfig(format!(
            "PSS display contains {} waveforms, but its retained orbit requires {expected_count}",
            waveforms.len()
        )));
    }
    for (name, periodic) in result.node_names.iter().zip(&result.waveforms) {
        if name == "0" || name.eq_ignore_ascii_case("gnd") {
            continue;
        }
        let display_name = format!("V({name})");
        let display = waveforms.get(&display_name).ok_or_else(|| {
            SimulationError::InvalidConfig(format!(
                "PSS display is missing retained-orbit waveform '{display_name}'"
            ))
        })?;
        if display.name != display_name
            || display.x_values.as_slice() != result.time.as_slice()
            || display.y_values.as_slice() != periodic.values.as_slice()
            || display.y_unit != "V"
            || display.is_complex
            || display.y_imag.is_some()
        {
            return Err(SimulationError::InvalidConfig(format!(
                "PSS display waveform '{display_name}' does not exactly match its retained numerical orbit"
            )));
        }
    }
    Ok(())
}

pub(super) fn simulation_result_from_worker_pss(
    measurements: Vec<WorkerMeasurement>,
    operating_point: rspice_core::engine::PssOperatingPoint,
) -> SimulationResult {
    let result = &operating_point.analysis().result;
    let time = result.time.clone();
    let mut waveforms = HashMap::with_capacity(result.waveforms.len());
    for (name, periodic) in result.node_names.iter().zip(&result.waveforms) {
        if name == "0" || name.eq_ignore_ascii_case("gnd") {
            continue;
        }
        let display_name = format!("V({name})");
        waveforms.insert(
            display_name.clone(),
            WaveformData::new_time_domain(display_name, time.clone(), periodic.values.clone()),
        );
    }
    SimulationResult::Transient {
        time,
        waveforms,
        measurements: measure_results(measurements),
        periodic_state: Some(std::sync::Arc::new(operating_point)),
        convergence: Default::default(),
        events: TransientEventHistory::default(),
    }
}

pub(super) fn waveform_map(waveforms: Vec<WorkerWaveform>) -> HashMap<String, WaveformData> {
    waveforms
        .into_iter()
        .map(|waveform| {
            let name = waveform.name.clone();
            (name, WaveformData::from(waveform))
        })
        .collect()
}

pub(super) fn worker_measurements(
    measurements: Vec<rspice_core::MeasureResult>,
) -> Vec<WorkerMeasurement> {
    measurements
        .into_iter()
        .map(WorkerMeasurement::from)
        .collect()
}

pub(super) fn measure_results(
    measurements: Vec<WorkerMeasurement>,
) -> Vec<rspice_core::MeasureResult> {
    measurements
        .into_iter()
        .map(rspice_core::MeasureResult::from)
        .collect()
}

pub(super) fn intern_static_label(value: String) -> &'static str {
    known_static_label(&value).unwrap_or("unknown")
}

/// Labels a worker response may carry, interned back to the `&'static str`
/// the host build uses.
///
/// The engine owns the vocabulary, so it is asked rather than restated.
/// `unknown` is accepted because [`intern_static_label`] produces it, and a
/// report that already crossed the boundary once has to survive crossing it
/// again unchanged. A noise mechanism does not come through here: the summary
/// carries it as owned text the whole way, so nothing interns it.
pub(super) fn known_static_label(value: &str) -> Option<&'static str> {
    rspice_core::circuit::resolve_op_label(value)
        .or_else(|| (value == "unknown").then_some("unknown"))
}
