//! The simulation worker contract.
//!
//! Everything crossing the boundary between the application and the engine
//! worker: the request and result messages, their transport encoding, and
//! the state each side must hold for a request to be replayable. The two
//! sides run in different threads natively and different contexts in the
//! browser, so this is a serialized contract rather than a shared type.

mod analysis;
mod analysis_spec;
mod conversions;
mod qpac;
mod qpnoise;
mod reliability;
use reliability::validate_worker_reliability_result;
mod qpxf;
use qpnoise::validate_worker_qpnoise_result;
use qpxf::validate_worker_qpxf_result;
mod qpss;
mod recorded_fft;
use qpac::validate_worker_qpac_result;
use qpss::validate_worker_qpss_result;
mod transport;

pub(crate) use conversions::*;
pub(crate) use recorded_fft::WorkerRecordedFftSpectrum;
pub(crate) use transport::*;

pub(crate) use analysis::*;
pub(crate) use analysis_spec::*;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, atomic::AtomicBool};

use serde::{Deserialize, Serialize};

use super::{
    NetlistInput, ResultSchemaMismatch, SimulationError, SimulationRequest, SpecExecutionOptions,
};
use crate::services::safety::{
    SoAEvaluation, SoAParameter, SoARuleVerdict, SoAViolation, ViolationSeverity,
};
use crate::simulation::config::{
    AcAnalysisConfig, AcSweepType, AnalysisConfig, DcSweepConfig, NoiseAnalysisConfig,
    NoiseContributionDetail, NoiseIntegrationMode, NoiseSweepType, PoleZeroConfig, PzAnalysisType,
    SensitivityConfig, SensitivitySweep, TransientAnalysisConfig,
};
use crate::simulation::multi_run::{AnalysisSpec, FrequencySweep, TfAccuracy, TfNormalization};
use crate::simulation::reliability_engine::{ParamShift, ReliabilityResult, StressMetrics};
use crate::simulation::results::{
    DcOpResult, DigitalEventPoint, EventNodeHistory, MonteCarloVariableResult, RealEventPoint,
    SimulationResult, TransferFunctionQuantity, TransferFunctionScalar, TransientEventHistory,
    WaveformData,
};
use crate::simulation::status::{SimulationProgress, SimulationStatus};
use crate::state::{NoiseContributorRow, NoiseSummary};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerRequest {
    pub id: u64,
    pub request: WorkerSimulationRequest,
    pub netlist: String,
    pub source_path: Option<String>,
    pub project_veriloga_runtimes: crate::simulation::veriloga::PreparedVerilogARuntimeSet,
    #[serde(default)]
    pub(in crate::simulation) dependencies:
        crate::simulation::execution::ResolvedExecutionDependencies,
    #[serde(default)]
    pub(in crate::simulation) environment: Option<super::AnalysisExecutionEnvironment>,
    #[serde(default)]
    pub(in crate::simulation) stream_transient_samples: bool,
}

/// 11: a transient-trajectory dependency carries the spectra its solve
///     recorded, and `AnalysisSpec::Fft` is a request a worker can be given.
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) const WORKER_REQUEST_TRANSPORT_PROTOCOL: u8 = 11;

/// Browser-worker request split into compact metadata and transferable
/// floating-point buffers. The embedded request deliberately carries empty
/// dependencies; authenticated dependency metadata is encoded separately so
/// its numerical payload never expands into per-sample JavaScript objects.
#[cfg(any(target_arch = "wasm32", test))]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct WorkerRequestTransport {
    pub protocol: u8,
    pub request: WorkerRequestTransportMetadata,
    pub buffers: Vec<Vec<f64>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerRequestTransportMetadata {
    pub request: WorkerRequest,
    pub dependency_metadata: String,
    /// Number of leading buffers owned by `dependency_metadata`. Any
    /// remaining buffer is reserved for the detached OP previous-state MNA
    /// vector below.
    pub dependency_buffer_count: usize,
    #[serde(default)]
    pub op_previous_state: Option<WorkerOpPreviousStateTransport>,
}

/// Authenticated scalar half of a retained OP initial guess. The numerical
/// MNA state is always a single transferable Float64 buffer; accepting an
/// inline representation here would silently reintroduce the browser JSON
/// expansion this transport exists to prevent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WorkerOpPreviousStateTransport {
    source_content_digest: crate::product::ContentDigest,
    producer_snapshot_digest: crate::product::ContentDigest,
    producer_result_digest: crate::product::ContentDigest,
    node_names: Vec<String>,
    branch_names: Vec<String>,
    solution: WorkerF64Series,
    solution_digest: crate::product::ContentDigest,
}

#[cfg(any(target_arch = "wasm32", test))]
impl WorkerRequestTransport {
    #[cfg(test)]
    pub(crate) fn from_request(mut request: WorkerRequest) -> Result<Self, String> {
        let dependencies = std::mem::take(&mut request.dependencies);
        let (dependency_metadata, mut buffers) = dependencies
            .encode_transfer()
            .map_err(|error| error.to_string())?;
        let dependency_buffer_count = buffers.len();
        let (op_previous_state, op_buffers) = take_worker_request_op_previous_state(&mut request)?;
        buffers.extend(op_buffers);
        validate_worker_request_transfer_buffers(&buffers)?;
        Ok(Self {
            protocol: WORKER_REQUEST_TRANSPORT_PROTOCOL,
            request: WorkerRequestTransportMetadata {
                request,
                dependency_metadata,
                dependency_buffer_count,
                op_previous_state,
            },
            buffers,
        })
    }

    pub(crate) fn into_request(self) -> Result<WorkerRequest, String> {
        if self.protocol != WORKER_REQUEST_TRANSPORT_PROTOCOL {
            return Err(format!(
                "unsupported worker request transport protocol {}",
                self.protocol
            ));
        }
        let WorkerRequestTransportMetadata {
            mut request,
            dependency_metadata,
            dependency_buffer_count,
            op_previous_state,
        } = self.request;
        request.project_veriloga_runtimes.validate()?;
        if request.dependencies != Default::default() {
            return Err("worker request metadata carries duplicate inline dependencies".to_owned());
        }
        reject_inline_worker_request_op_previous_state(&request)?;
        validate_worker_request_transfer_buffers(&self.buffers)?;
        if dependency_buffer_count > self.buffers.len() {
            return Err(format!(
                "worker request declares {dependency_buffer_count} dependency buffers but carries only {} total buffers",
                self.buffers.len()
            ));
        }
        let mut dependency_buffers = self.buffers;
        let op_buffers = dependency_buffers.split_off(dependency_buffer_count);
        request.dependencies =
            crate::simulation::execution::ResolvedExecutionDependencies::decode_transfer(
                &dependency_metadata,
                dependency_buffers,
            )
            .map_err(|error| error.to_string())?;
        restore_worker_request_op_previous_state(&mut request, op_previous_state, &op_buffers)?;
        Ok(request)
    }
}

#[cfg(any(target_arch = "wasm32", test))]
fn worker_request_op_config_mut(
    request: &mut WorkerRequest,
) -> Option<&mut crate::simulation::dialog::OpConfig> {
    match &mut request.request {
        WorkerSimulationRequest::Config(config) => match config.as_mut() {
            WorkerAnalysisConfig::DcOp(config) => Some(config),
            _ => None,
        },
        WorkerSimulationRequest::Spec { spec, .. } => match spec.as_mut() {
            WorkerAnalysisSpec::DcOp(config) => Some(config),
            _ => None,
        },
    }
}

#[cfg(any(target_arch = "wasm32", test))]
fn worker_request_op_config(
    request: &WorkerRequest,
) -> Option<&crate::simulation::dialog::OpConfig> {
    match &request.request {
        WorkerSimulationRequest::Config(config) => match config.as_ref() {
            WorkerAnalysisConfig::DcOp(config) => Some(config),
            _ => None,
        },
        WorkerSimulationRequest::Spec { spec, .. } => match spec.as_ref() {
            WorkerAnalysisSpec::DcOp(config) => Some(config),
            _ => None,
        },
    }
}

#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn take_worker_request_op_previous_state(
    request: &mut WorkerRequest,
) -> Result<(Option<WorkerOpPreviousStateTransport>, Vec<Vec<f64>>), String> {
    let Some(config) = worker_request_op_config_mut(request) else {
        return Ok((None, Vec::new()));
    };
    config.validate_for_execution()?;
    let Some(previous_state) = config.previous_state.take() else {
        return Ok((None, Vec::new()));
    };
    let mut buffers = Vec::with_capacity(1);
    let transport =
        WorkerOpPreviousStateTransport::from_previous_state(previous_state, &mut buffers)?;
    Ok((Some(transport), buffers))
}

#[cfg(any(target_arch = "wasm32", test))]
fn reject_inline_worker_request_op_previous_state(request: &WorkerRequest) -> Result<(), String> {
    if worker_request_op_config(request).is_some_and(|config| config.previous_state.is_some()) {
        return Err(
            "worker request metadata carries a duplicate inline OP previous-state solution"
                .to_owned(),
        );
    }
    Ok(())
}

#[cfg(any(target_arch = "wasm32", test))]
fn restore_worker_request_op_previous_state(
    request: &mut WorkerRequest,
    previous_state: Option<WorkerOpPreviousStateTransport>,
    buffers: &[Vec<f64>],
) -> Result<(), String> {
    let expected_buffers = usize::from(previous_state.is_some());
    if buffers.len() != expected_buffers {
        return Err(format!(
            "worker OP previous-state transfer carries {} buffers, expected {expected_buffers}",
            buffers.len()
        ));
    }
    let config = worker_request_op_config_mut(request);
    match (config, previous_state) {
        (Some(config), Some(previous_state)) => {
            config.previous_state = Some(previous_state.into_previous_state(buffers)?);
            config.validate_for_execution()?;
        }
        (Some(config), None) => config.validate_for_execution()?,
        (None, Some(_)) => {
            return Err(
                "worker request carries OP previous-state metadata for a non-OP analysis"
                    .to_owned(),
            );
        }
        (None, None) => {}
    }
    Ok(())
}

impl WorkerRequest {
    pub(crate) fn from_runner_parts(
        id: u64,
        request: &SimulationRequest,
        input: &NetlistInput,
    ) -> Result<Self, SimulationError> {
        Ok(Self {
            id,
            request: WorkerSimulationRequest::try_from(request)?,
            netlist: input.netlist.clone(),
            source_path: input
                .source_path
                .as_ref()
                .map(|path| path.to_string_lossy().into_owned()),
            project_veriloga_runtimes: input.project_veriloga_runtimes.clone(),
            dependencies: input.dependencies.clone(),
            environment: input.environment.clone(),
            stream_transient_samples: input.stream_transient_samples,
        })
    }

    pub(crate) fn into_runner_parts(self) -> (SimulationRequest, NetlistInput) {
        (
            SimulationRequest::from(self.request),
            NetlistInput {
                netlist: self.netlist,
                source_path: self.source_path.map(PathBuf::from),
                project_veriloga_runtimes: self.project_veriloga_runtimes,
                dependencies: self.dependencies,
                environment: self.environment,
                stream_transient_samples: self.stream_transient_samples,
            },
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerSimulationRequest {
    /// Boxed to match `Spec`, whose two payloads already are.
    Config(Box<WorkerAnalysisConfig>),
    Spec {
        spec: Box<WorkerAnalysisSpec>,
        options: Box<WorkerSpecExecutionOptions>,
    },
}

/// The reference temperature a noise request defaults to.
///
/// Named by a `serde(default)` in two modules — the specification enum and
/// the noise run configuration — so it stays where both can see it.
const fn worker_default_noise_temperature() -> f64 {
    rspice_core::constants::TEMP_REFERENCE
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerResponse {
    pub id: u64,
    pub outcome: WorkerOutcome,
}

impl WorkerResponse {
    pub(crate) fn from_result_for_transfer(
        id: u64,
        result: Result<SimulationResult, SimulationError>,
    ) -> Self {
        let outcome = worker_transfer_outcome_from_result(result);
        Self { id, outcome }
    }

    pub(crate) fn into_result(self) -> Result<SimulationResult, SimulationError> {
        match self.outcome {
            WorkerOutcome::Success(result) => {
                validate_worker_pstb_result(&result).map_err(SimulationError::InvalidConfig)?;
                validate_worker_qpss_result(&result).map_err(SimulationError::InvalidConfig)?;
                validate_worker_qpac_result(&result).map_err(SimulationError::InvalidConfig)?;
                validate_worker_qpxf_result(&result).map_err(SimulationError::InvalidConfig)?;
                validate_worker_reliability_result(&result)
                    .map_err(SimulationError::InvalidConfig)?;
                validate_worker_qpnoise_result(&result).map_err(SimulationError::InvalidConfig)?;
                if let WorkerSimulationResult::Transient { events, .. } = result.as_ref()
                    && let Some(history) = &events.current_impulses
                {
                    history.validate().map_err(SimulationError::InvalidConfig)?;
                }
                Ok(SimulationResult::from(*result))
            }
            WorkerOutcome::Failure(error) => Err(SimulationError::from(error)),
        }
    }
}

pub(crate) fn validate_worker_response_id(
    outer_id: u64,
    response: &WorkerResponse,
) -> Result<(), SimulationError> {
    if response.id == outer_id {
        Ok(())
    } else {
        Err(SimulationError::InvalidConfig(format!(
            "simulation worker result id mismatch: outer id {outer_id}, response id {}",
            response.id
        )))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerOutcome {
    /// Boxed: a result is an order of magnitude larger than an error, and
    /// every outcome value carried the difference.
    Success(Box<WorkerSimulationResult>),
    Failure(WorkerSimulationError),
}

#[cfg(test)]
fn worker_outcome_from_result(
    result: Result<SimulationResult, SimulationError>,
    payload_limit_bytes: usize,
) -> WorkerOutcome {
    match result {
        Ok(result) => match WorkerSimulationResult::try_from(result) {
            Ok(result) => {
                let payload_bytes = result.estimated_numeric_payload_bytes();
                if payload_bytes > payload_limit_bytes {
                    WorkerOutcome::Failure(worker_payload_limit_error(
                        payload_bytes,
                        payload_limit_bytes,
                    ))
                } else {
                    WorkerOutcome::Success(Box::new(result))
                }
            }
            Err(error) => WorkerOutcome::Failure(WorkerSimulationError::from(error)),
        },
        Err(error) => WorkerOutcome::Failure(WorkerSimulationError::from(error)),
    }
}

fn worker_transfer_outcome_from_result(
    result: Result<SimulationResult, SimulationError>,
) -> WorkerOutcome {
    match result {
        Ok(result) => match WorkerSimulationResult::try_from(result) {
            Ok(result) => WorkerOutcome::Success(Box::new(result)),
            Err(error) => WorkerOutcome::Failure(WorkerSimulationError::from(error)),
        },
        Err(error) => WorkerOutcome::Failure(WorkerSimulationError::from(error)),
    }
}

#[cfg(test)]
fn worker_payload_limit_error(payload_bytes: usize, limit_bytes: usize) -> WorkerSimulationError {
    WorkerSimulationError::InvalidConfig(format!(
        "browser worker result numeric payload is {} and exceeds the current {} transport limit; reduce saved signals/points or use the native desktop runner for dense waveforms",
        crate::simulation::run_set::format_bytes(payload_bytes as u64),
        crate::simulation::run_set::format_bytes(limit_bytes as u64)
    ))
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerSimulationError {
    ParseError(String),
    BehavioralReference {
        owner_name: String,
        canonical_owner_name: String,
        dependency_name: String,
        canonical_dependency_name: String,
        reason: String,
    },
    CircuitError(String),
    /// A Verilog-A or mixed elaboration refusal, still typed on the far side
    /// of the browser worker boundary: a schematic on this side has the same
    /// right to mark the instance the engine named.
    Elaboration {
        instance: Option<String>,
        module: Option<String>,
        kind: String,
        location: Option<String>,
        message: String,
    },
    SolverError(String),
    RequestedSignalUnavailable {
        signal: String,
        analysis: String,
        coordinate: Option<String>,
    },
    ResultSchemaMismatch(Box<ResultSchemaMismatch>),
    ConvergenceFailed {
        iterations: usize,
        message: String,
    },
    /// A failure the engine attributed to named design objects. The objects
    /// cross the worker boundary with the error because a browser run's
    /// schematic is on this side of it and has the same right to mark them.
    Attributed {
        message: String,
        attribution: crate::state::ConvergenceAttribution,
    },
    Aborted,
    AlreadyRunning,
    ThreadPanic,
    InvalidConfig(String),
    UnsupportedOutcome(String),
    ResourceLimit {
        resource: String,
        requested: usize,
        limit: usize,
    },
}

impl From<SimulationError> for WorkerSimulationError {
    fn from(value: SimulationError) -> Self {
        match value {
            SimulationError::ParseError(message) => Self::ParseError(message),
            SimulationError::BehavioralReference {
                owner_name,
                canonical_owner_name,
                dependency_name,
                canonical_dependency_name,
                reason,
            } => Self::BehavioralReference {
                owner_name,
                canonical_owner_name,
                dependency_name,
                canonical_dependency_name,
                reason,
            },
            SimulationError::CircuitError(message) => Self::CircuitError(message),
            SimulationError::Elaboration {
                instance,
                module,
                kind,
                location,
                message,
            } => Self::Elaboration {
                instance,
                module,
                kind,
                location,
                message,
            },
            SimulationError::SolverError(message) => Self::SolverError(message),
            SimulationError::RequestedSignalUnavailable {
                signal,
                analysis,
                coordinate,
            } => Self::RequestedSignalUnavailable {
                signal,
                analysis,
                coordinate,
            },
            SimulationError::ResultSchemaMismatch(mismatch) => Self::ResultSchemaMismatch(mismatch),
            SimulationError::ConvergenceFailed {
                iterations,
                message,
            } => Self::ConvergenceFailed {
                iterations,
                message,
            },
            SimulationError::Attributed {
                message,
                attribution,
            } => Self::Attributed {
                message,
                attribution,
            },
            SimulationError::Aborted => Self::Aborted,
            SimulationError::AlreadyRunning => Self::AlreadyRunning,
            SimulationError::ThreadPanic => Self::ThreadPanic,
            SimulationError::InvalidConfig(message) => Self::InvalidConfig(message),
            SimulationError::UnsupportedOutcome(message) => Self::UnsupportedOutcome(message),
            SimulationError::ResourceLimit {
                resource,
                requested,
                limit,
            } => Self::ResourceLimit {
                resource,
                requested,
                limit,
            },
        }
    }
}

impl From<WorkerSimulationError> for SimulationError {
    fn from(value: WorkerSimulationError) -> Self {
        match value {
            WorkerSimulationError::ParseError(message) => Self::ParseError(message),
            WorkerSimulationError::BehavioralReference {
                owner_name,
                canonical_owner_name,
                dependency_name,
                canonical_dependency_name,
                reason,
            } => Self::BehavioralReference {
                owner_name,
                canonical_owner_name,
                dependency_name,
                canonical_dependency_name,
                reason,
            },
            WorkerSimulationError::CircuitError(message) => Self::CircuitError(message),
            WorkerSimulationError::Elaboration {
                instance,
                module,
                kind,
                location,
                message,
            } => Self::Elaboration {
                instance,
                module,
                kind,
                location,
                message,
            },
            WorkerSimulationError::SolverError(message) => Self::SolverError(message),
            WorkerSimulationError::RequestedSignalUnavailable {
                signal,
                analysis,
                coordinate,
            } => Self::RequestedSignalUnavailable {
                signal,
                analysis,
                coordinate,
            },
            WorkerSimulationError::ResultSchemaMismatch(mismatch) => {
                Self::ResultSchemaMismatch(mismatch)
            }
            WorkerSimulationError::ConvergenceFailed {
                iterations,
                message,
            } => Self::ConvergenceFailed {
                iterations,
                message,
            },
            WorkerSimulationError::Attributed {
                message,
                attribution,
            } => Self::Attributed {
                message,
                attribution,
            },
            WorkerSimulationError::Aborted => Self::Aborted,
            WorkerSimulationError::AlreadyRunning => Self::AlreadyRunning,
            WorkerSimulationError::ThreadPanic => Self::ThreadPanic,
            WorkerSimulationError::InvalidConfig(message) => Self::InvalidConfig(message),
            WorkerSimulationError::UnsupportedOutcome(message) => Self::UnsupportedOutcome(message),
            WorkerSimulationError::ResourceLimit {
                resource,
                requested,
                limit,
            } => Self::ResourceLimit {
                resource,
                requested,
                limit,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerProgressSnapshot {
    pub id: u64,
    pub status: WorkerProgressStatus,
    pub progress: Option<f32>,
    pub elapsed_ms: u64,
}

impl WorkerProgressSnapshot {
    pub(crate) fn from_progress(id: u64, progress: &SimulationProgress) -> Self {
        let elapsed_ms = progress.elapsed.as_millis().min(u128::from(u64::MAX)) as u64;
        Self {
            id,
            status: WorkerProgressStatus::from(&progress.status),
            progress: progress.status.progress(),
            elapsed_ms,
        }
    }

    pub(crate) fn apply_to(self, progress: &mut SimulationProgress) {
        progress.elapsed = std::time::Duration::from_millis(self.elapsed_ms);
        progress.update_status(SimulationStatus::from(self.status));
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerProgressStatus {
    Idle,
    Parsing,
    Building,
    DcOperatingPoint,
    DcSweep { source: String, progress: f32 },
    Transient { time: f64, stop_time: f64 },
    AcAnalysis { freq: f64, stop_freq: f64 },
    NoiseAnalysis { freq: f64, stop_freq: f64 },
    PoleZero,
    Sensitivity,
    PostProcessing,
    Completed,
    Aborted,
}

impl From<&SimulationStatus> for WorkerProgressStatus {
    fn from(value: &SimulationStatus) -> Self {
        match value {
            SimulationStatus::Idle => Self::Idle,
            SimulationStatus::Parsing => Self::Parsing,
            SimulationStatus::Building => Self::Building,
            SimulationStatus::DcOperatingPoint => Self::DcOperatingPoint,
            SimulationStatus::DcSweep { source, progress } => Self::DcSweep {
                source: source.clone(),
                progress: *progress,
            },
            SimulationStatus::Transient { time, stop_time } => Self::Transient {
                time: *time,
                stop_time: *stop_time,
            },
            SimulationStatus::AcAnalysis { freq, stop_freq } => Self::AcAnalysis {
                freq: *freq,
                stop_freq: *stop_freq,
            },
            SimulationStatus::NoiseAnalysis { freq, stop_freq } => Self::NoiseAnalysis {
                freq: *freq,
                stop_freq: *stop_freq,
            },
            SimulationStatus::PoleZero => Self::PoleZero,
            SimulationStatus::Sensitivity => Self::Sensitivity,
            SimulationStatus::PostProcessing => Self::PostProcessing,
            SimulationStatus::Completed { .. } => Self::Completed,
            SimulationStatus::Aborted { .. } => Self::Aborted,
        }
    }
}

impl From<WorkerProgressStatus> for SimulationStatus {
    fn from(value: WorkerProgressStatus) -> Self {
        match value {
            WorkerProgressStatus::Idle => Self::Idle,
            WorkerProgressStatus::Parsing => Self::Parsing,
            WorkerProgressStatus::Building => Self::Building,
            WorkerProgressStatus::DcOperatingPoint => Self::DcOperatingPoint,
            WorkerProgressStatus::DcSweep { source, progress } => {
                Self::DcSweep { source, progress }
            }
            WorkerProgressStatus::Transient { time, stop_time } => {
                Self::Transient { time, stop_time }
            }
            WorkerProgressStatus::AcAnalysis { freq, stop_freq } => {
                Self::AcAnalysis { freq, stop_freq }
            }
            WorkerProgressStatus::NoiseAnalysis { freq, stop_freq } => {
                Self::NoiseAnalysis { freq, stop_freq }
            }
            WorkerProgressStatus::PoleZero => Self::PoleZero,
            WorkerProgressStatus::Sensitivity => Self::Sensitivity,
            WorkerProgressStatus::PostProcessing => Self::PostProcessing,
            WorkerProgressStatus::Completed => Self::Completed {
                elapsed: std::time::Duration::ZERO,
            },
            WorkerProgressStatus::Aborted => Self::Aborted {
                elapsed: std::time::Duration::ZERO,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerSimulationResult {
    DcOp {
        configuration: crate::simulation::dialog::OpConfig,
        validated_startup_directives: usize,
        #[serde(default)]
        mna_node_names: Vec<String>,
        #[serde(default)]
        mna_branch_names: Vec<String>,
        #[serde(default)]
        mna_solution: Vec<f64>,
        node_voltages: HashMap<String, f64>,
        branch_currents: HashMap<String, f64>,
        device_report: Option<WorkerDeviceOpReport>,
    },
    DcSweep {
        sweep_var: String,
        sweep_values: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
        measurements: Vec<WorkerMeasurement>,
        evidence: Option<crate::state::DcSweepEvidence>,
    },
    Transient {
        time: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
        measurements: Vec<WorkerMeasurement>,
        convergence: Option<crate::state::TransientConvergenceEvidence>,
        #[serde(default)]
        events: WorkerEventHistory,
        /// Spectra the engine computed for the `.fft` cards this solve carried.
        /// Defaulted so a worker built before recorded FFT existed answers
        /// this contract with the truth: it carried no card.
        #[serde(default)]
        spectra: Vec<WorkerRecordedFftSpectrum>,
    },
    /// One recorded `.FFT` spectrum, selected from the transient that computed
    /// it. The task runs no solve, so no netlist and no time axis cross here.
    Fft {
        spectrum: WorkerRecordedFftSpectrum,
        convergence: Option<crate::state::TransientConvergenceEvidence>,
    },
    /// PSS numerical evidence is transported once. Display waveforms are
    /// deterministically reconstructed from this retained orbit by the
    /// receiver instead of duplicating every sample across the worker edge.
    Pss {
        measurements: Vec<WorkerMeasurement>,
        operating_point: rspice_core::engine::PssOperatingPoint,
    },
    Ac {
        convergence: Option<crate::state::TransientConvergenceEvidence>,
        frequencies: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
        measurements: Vec<WorkerMeasurement>,
        #[serde(default)]
        reference_impedances_ohm: Option<Vec<f64>>,
        #[serde(default)]
        noise_reference_temperature_kelvin: Option<f64>,
    },
    Pstb {
        period: f64,
        fundamental_frequency: f64,
        stability_threshold: f64,
        probe_instance: String,
        detect_subharmonics: bool,
        modes: Vec<WorkerPstbFloquetMode>,
        floquet_evidence: rspice_core::analysis::FloquetSpectrumEvidence,
        orbit_kind: rspice_core::analysis::FloquetOrbitKind,
        trivial_multiplier_index: Option<usize>,
        stability_verdict: rspice_core::analysis::FloquetStabilityVerdict,
        stability_classification: WorkerPstbStabilityClassification,
        min_stability_margin_db: Option<f64>,
        max_multiplier_magnitude: f64,
        num_unstable: usize,
        subharmonics: Vec<usize>,
        converged: bool,
        iterations: usize,
        mode_indices: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
    },
    Qpac {
        frequencies: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
        response: rspice_core::engine::QpacAnalysisResult,
    },
    Qpnoise {
        frequencies: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
        response: rspice_core::engine::QpnoiseAnalysisResult,
    },
    Qpxf {
        frequencies: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
        response: rspice_core::engine::QpxfAnalysisResult,
    },
    Qpss {
        frequencies: Vec<f64>,
        tuples: Vec<Vec<i32>>,
        waveforms: Vec<WorkerWaveform>,
        operating_point: rspice_core::engine::QpssOperatingPoint,
    },
    Hb {
        frequencies: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
        measurements: Vec<WorkerMeasurement>,
        operating_point: rspice_core::engine::HbOperatingPoint,
    },
    Noise {
        frequencies: Vec<f64>,
        output_noise: Vec<f64>,
        input_noise: Option<Vec<f64>>,
        contributors: HashMap<String, Vec<f64>>,
        #[serde(default)]
        summary: Option<WorkerNoiseSummary>,
        #[serde(default)]
        measurements: Vec<WorkerMeasurement>,
    },
    PoleZero {
        poles: Vec<(f64, f64)>,
        zeros: Vec<(f64, f64)>,
        pole_evidence: crate::state::PoleZeroRootSetEvidence,
        zero_evidence: crate::state::PoleZeroRootSetEvidence,
        #[serde(default)]
        gain: Option<f64>,
    },
    SensitivityStudy {
        evidence: crate::state::SensitivityStudyEvidence,
    },
    /// Linearized DC mismatch spread and its ranked contributors.
    ///
    /// No transport protocol bump: a bump records a change an older worker
    /// would answer *silently differently*, and an older worker cannot answer
    /// a DC mismatch request at all — it refuses the specification by name
    /// before dispatch. This variant adds no reading of any existing payload.
    DcMismatch {
        evidence: crate::state::DcMismatchEvidence,
    },
    TransferFunction {
        input_source: String,
        output_expression: String,
        input_quantity: WorkerTransferFunctionQuantity,
        output_quantity: WorkerTransferFunctionQuantity,
        input_unit: String,
        output_unit: String,
        normalization: TfNormalization,
        accuracy: TfAccuracy,
        gain: Option<WorkerTransferFunctionScalar>,
        input_resistance: Option<WorkerTransferFunctionScalar>,
        output_resistance: Option<WorkerTransferFunctionScalar>,
        nominal_input: Option<f64>,
        nominal_output: Option<f64>,
    },
    Parametric {
        target: String,
        sweep_values: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
        num_failures: usize,
        /// Per-point evidence. Defaulted so a worker built before families
        /// measured their members still answers this contract.
        #[serde(default)]
        member_measurements: Vec<crate::state::FamilyMemberMeasurements>,
    },
    Corner {
        x_values: Vec<f64>,
        x_label: String,
        x_unit: String,
        temperatures_c: Vec<f64>,
        corner_labels: Vec<String>,
        waveforms: Vec<WorkerWaveform>,
        num_failures: usize,
        #[serde(default)]
        member_measurements: Vec<crate::state::FamilyMemberMeasurements>,
    },
    MonteCarlo {
        seed: u64,
        runs_requested: usize,
        runs_completed: usize,
        num_failures: usize,
        all_converged: bool,
        variables: Vec<WorkerMonteCarloVariable>,
        /// Per-trial evidence. Without it a worker-executed Monte Carlo run
        /// answers a limit with nothing while the same deck run in-process
        /// answers it with a distribution, and the two disagree about one run.
        #[serde(default)]
        member_measurements: Vec<crate::state::FamilyMemberMeasurements>,
    },
    ReliabilityMission {
        years: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
        response: rspice_core::engine::ReliabilityRunResult,
    },
    Reliability {
        years: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
        device_results: Vec<WorkerReliabilityResult>,
    },
    Optimization {
        iterations: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
        best_cost: f64,
        best_variables: HashMap<String, f64>,
        converged: bool,
    },
    Soa {
        convergence: Option<crate::state::TransientConvergenceEvidence>,
        time: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
        violations: Vec<WorkerSoAViolation>,
        evaluations: Vec<WorkerSoAEvaluation>,
    },
    MeasurementsOnly {
        measurements: HashMap<String, f64>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerPstbFloquetMode {
    multiplier: (f64, f64),
    exponent: (f64, f64),
    probe_participation: f64,
    is_unstable: bool,
    is_trivial: bool,
    subharmonic_order: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum WorkerPstbStabilityClassification {
    Stable,
    UnstableReal,
    UnstableComplex,
    PeriodDoubling,
    NeimarkSacker,
    SaddleNode,
    Marginal,
    Indeterminate,
}

impl WorkerPstbStabilityClassification {
    fn try_from_core(
        value: rspice_core::analysis::pstb::StabilityType,
    ) -> Result<Self, SimulationError> {
        use rspice_core::analysis::pstb::StabilityType;
        match value {
            StabilityType::Stable => Ok(Self::Stable),
            StabilityType::UnstableReal => Ok(Self::UnstableReal),
            StabilityType::UnstableComplex => Ok(Self::UnstableComplex),
            StabilityType::PeriodDoubling => Ok(Self::PeriodDoubling),
            StabilityType::NeimarkSacker => Ok(Self::NeimarkSacker),
            StabilityType::SaddleNode => Ok(Self::SaddleNode),
            StabilityType::Marginal => Ok(Self::Marginal),
            StabilityType::Indeterminate => Ok(Self::Indeterminate),
            _ => Err(SimulationError::InvalidConfig(
                "PSTB returned an unsupported stability classification".to_owned(),
            )),
        }
    }

    fn into_core(self) -> rspice_core::analysis::pstb::StabilityType {
        use rspice_core::analysis::pstb::StabilityType;
        match self {
            Self::Stable => StabilityType::Stable,
            Self::UnstableReal => StabilityType::UnstableReal,
            Self::UnstableComplex => StabilityType::UnstableComplex,
            Self::PeriodDoubling => StabilityType::PeriodDoubling,
            Self::NeimarkSacker => StabilityType::NeimarkSacker,
            Self::SaddleNode => StabilityType::SaddleNode,
            Self::Marginal => StabilityType::Marginal,
            Self::Indeterminate => StabilityType::Indeterminate,
        }
    }
}

fn validate_worker_pstb_result(result: &WorkerSimulationResult) -> Result<(), String> {
    let WorkerSimulationResult::Pstb {
        period,
        fundamental_frequency,
        stability_threshold,
        probe_instance,
        detect_subharmonics,
        modes,
        floquet_evidence,
        orbit_kind,
        trivial_multiplier_index,
        stability_verdict,
        stability_classification,
        min_stability_margin_db,
        max_multiplier_magnitude,
        num_unstable,
        subharmonics,
        converged,
        mode_indices,
        waveforms,
        ..
    } = result
    else {
        return Ok(());
    };

    if !period.is_finite()
        || *period <= 0.0
        || !fundamental_frequency.is_finite()
        || *fundamental_frequency <= 0.0
        || *fundamental_frequency != 1.0 / *period
        || !stability_threshold.is_finite()
        || *stability_threshold < 1.0
        || probe_instance.is_empty()
        || probe_instance.trim() != probe_instance.as_str()
        || !*converged
    {
        return Err("PSTB period, frequency, or convergence metadata is invalid".to_owned());
    }

    let multipliers = modes
        .iter()
        .map(|mode| num_complex::Complex64::new(mode.multiplier.0, mode.multiplier.1))
        .collect::<Vec<_>>();
    if !floquet_evidence.is_consistent_with(&multipliers) {
        return Err("PSTB Floquet evidence is inconsistent with the complete spectrum".to_owned());
    }
    match floquet_evidence {
        rspice_core::analysis::FloquetSpectrumEvidence::NoDynamicModes
            if modes.is_empty()
                && *orbit_kind == rspice_core::analysis::FloquetOrbitKind::Driven
                && trivial_multiplier_index.is_none() => {}
        rspice_core::analysis::FloquetSpectrumEvidence::Qualified { certificate }
            if certificate.is_valid()
                && !modes.is_empty()
                && certificate.problem_order == modes.len() => {}
        _ => {
            return Err("PSTB result lacks a complete canonical Floquet certificate".to_owned());
        }
    }

    let expected_trivial = if *orbit_kind == rspice_core::analysis::FloquetOrbitKind::Autonomous {
        rspice_core::analysis::select_autonomous_phase_mode(&multipliers)
    } else {
        None
    };
    if *trivial_multiplier_index != expected_trivial {
        return Err("PSTB trivial phase-mode index is inconsistent with the spectrum".to_owned());
    }

    if modes.windows(2).any(|pair| {
        let left = num_complex::Complex64::new(pair[0].multiplier.0, pair[0].multiplier.1);
        let right = num_complex::Complex64::new(pair[1].multiplier.0, pair[1].multiplier.1);
        right
            .norm()
            .total_cmp(&left.norm())
            .then_with(|| left.re.total_cmp(&right.re))
            .then_with(|| left.im.total_cmp(&right.im))
            .is_gt()
    }) {
        return Err("PSTB complete spectrum is not in canonical sorted order".to_owned());
    }

    let mut expected_subharmonics = Vec::new();
    let mut expected_unstable = 0usize;
    for (index, mode) in modes.iter().enumerate() {
        let value = multipliers[index];
        let magnitude = value.norm();
        let expected_exponent = value.ln() / *period;
        if !value.re.is_finite()
            || !value.im.is_finite()
            || !mode.exponent.0.is_finite()
            || !mode.exponent.1.is_finite()
            || mode.exponent != (expected_exponent.re, expected_exponent.im)
            || !magnitude.is_finite()
            || magnitude <= 0.0
            || !mode.probe_participation.is_finite()
            || !(0.0..=1.0).contains(&mode.probe_participation)
            || mode.is_trivial != (*trivial_multiplier_index == Some(index))
            || (mode.is_trivial && mode.is_unstable)
        {
            return Err(format!("PSTB Floquet mode {} is invalid", index + 1));
        }
        let detected_subharmonic = (2..=8).find(|order| {
            let expected_angle = 2.0 * std::f64::consts::PI / *order as f64;
            (magnitude - 1.0).abs() <= 0.01 && (value.arg().abs() - expected_angle).abs() < 0.01
        });
        let expected_subharmonic = (*detect_subharmonics)
            .then_some(detected_subharmonic)
            .flatten();
        if mode.subharmonic_order != expected_subharmonic {
            return Err(format!(
                "PSTB Floquet mode {} has an inconsistent subharmonic order",
                index + 1
            ));
        }
        if let Some(order) = expected_subharmonic {
            expected_subharmonics.push(order);
        }
        let is_unstable = !mode.is_trivial && magnitude > *stability_threshold;
        if mode.is_unstable != is_unstable {
            return Err(format!(
                "PSTB Floquet mode {} has an inconsistent instability flag",
                index + 1
            ));
        }
        if is_unstable {
            expected_unstable += 1;
        }
    }

    if *num_unstable != expected_unstable || *subharmonics != expected_subharmonics {
        return Err("PSTB aggregate mode counts or subharmonics are inconsistent".to_owned());
    }
    let expected_maximum = multipliers.first().map_or(0.0, |value| value.norm());
    let expected_minimum_margin = modes
        .iter()
        .enumerate()
        .filter(|(index, _)| *trivial_multiplier_index != Some(*index))
        .map(|(index, _)| -20.0 * multipliers[index].norm().log10())
        .min_by(f64::total_cmp);
    if !max_multiplier_magnitude.is_finite()
        || *max_multiplier_magnitude != expected_maximum
        || min_stability_margin_db.is_some_and(|margin| !margin.is_finite())
        || *min_stability_margin_db != expected_minimum_margin
    {
        return Err("PSTB global magnitude or margin metadata is inconsistent".to_owned());
    }

    let expected_verdict = rspice_core::analysis::classify_floquet_stability(
        &multipliers,
        floquet_evidence,
        *orbit_kind,
        *trivial_multiplier_index,
        *stability_threshold - 1.0,
    );
    if *stability_verdict != expected_verdict {
        return Err("PSTB stability verdict is inconsistent with the complete spectrum".to_owned());
    }

    let expected_classification = match stability_verdict {
        rspice_core::analysis::FloquetStabilityVerdict::Stable => {
            WorkerPstbStabilityClassification::Stable
        }
        rspice_core::analysis::FloquetStabilityVerdict::Indeterminate => {
            WorkerPstbStabilityClassification::Indeterminate
        }
        rspice_core::analysis::FloquetStabilityVerdict::Unstable => {
            let dominant = modes.iter().find(|mode| mode.is_unstable).ok_or_else(|| {
                "PSTB unstable verdict has no unstable complete-spectrum mode".to_owned()
            })?;
            if dominant.multiplier.1.abs() > 0.01 {
                WorkerPstbStabilityClassification::UnstableComplex
            } else {
                WorkerPstbStabilityClassification::UnstableReal
            }
        }
        rspice_core::analysis::FloquetStabilityVerdict::Marginal => modes
            .iter()
            .enumerate()
            .filter(|(index, _)| *trivial_multiplier_index != Some(*index))
            .find_map(|(_, mode)| {
                let value = num_complex::Complex64::new(mode.multiplier.0, mode.multiplier.1);
                if (value + num_complex::Complex64::new(1.0, 0.0)).norm() < 0.01 {
                    Some(WorkerPstbStabilityClassification::PeriodDoubling)
                } else if (value - num_complex::Complex64::new(1.0, 0.0)).norm() < 0.01 {
                    Some(WorkerPstbStabilityClassification::SaddleNode)
                } else if (value.norm() - 1.0).abs() < 0.01 && value.im.abs() > 0.01 {
                    Some(WorkerPstbStabilityClassification::NeimarkSacker)
                } else {
                    None
                }
            })
            .unwrap_or(WorkerPstbStabilityClassification::Marginal),
        _ => return Err("PSTB stability verdict is unsupported".to_owned()),
    };
    if *stability_classification != expected_classification {
        return Err("PSTB rich stability classification is inconsistent".to_owned());
    }

    validate_worker_pstb_display(mode_indices, waveforms, modes, &multipliers)
}

fn validate_worker_pstb_display(
    mode_indices: &[f64],
    waveforms: &[WorkerWaveform],
    modes: &[WorkerPstbFloquetMode],
    multipliers: &[num_complex::Complex64],
) -> Result<(), String> {
    if mode_indices.len() > modes.len()
        || mode_indices
            .iter()
            .enumerate()
            .any(|(index, value)| *value != (index + 1) as f64)
    {
        return Err(
            "PSTB display mode axis is not a leading complete-spectrum projection".to_owned(),
        );
    }
    const DISPLAY: [(&str, &str); 6] = [
        ("Floquet |lambda|", ""),
        ("Floquet Phase (deg)", "deg"),
        ("Stability Margin (dB)", "dB"),
        ("Mode Damping (1/s)", "1/s"),
        ("Mode Frequency (Hz)", "Hz"),
        ("Probe Mode Participation", ""),
    ];
    if waveforms.len() != DISPLAY.len() {
        return Err("PSTB display does not contain the exact waveform projection".to_owned());
    }
    let waveforms = waveforms
        .iter()
        .map(|waveform| (waveform.name.as_str(), waveform))
        .collect::<HashMap<_, _>>();
    if waveforms.len() != DISPLAY.len() {
        return Err("PSTB display contains duplicate waveform names".to_owned());
    }
    for (name, unit) in DISPLAY {
        let waveform = waveforms
            .get(name)
            .ok_or_else(|| format!("PSTB display is missing waveform '{name}'"))?;
        if waveform.name != name
            || waveform.y_unit != unit
            || waveform.is_complex
            || waveform.y_imag.is_some()
            || waveform.x_values.as_slice() != mode_indices
            || waveform.y_values.len() != mode_indices.len()
            || waveform.y_values.iter().any(|value| !value.is_finite())
        {
            return Err(format!("PSTB display waveform '{name}' is malformed"));
        }
        for index in 0..mode_indices.len() {
            let expected = match name {
                "Floquet |lambda|" => multipliers[index].norm(),
                "Floquet Phase (deg)" => multipliers[index].arg() * 180.0 / std::f64::consts::PI,
                "Stability Margin (dB)" => -20.0 * multipliers[index].norm().log10(),
                "Mode Damping (1/s)" => -modes[index].exponent.0,
                "Mode Frequency (Hz)" => {
                    modes[index].exponent.1.abs() / (2.0 * std::f64::consts::PI)
                }
                "Probe Mode Participation" => modes[index].probe_participation,
                _ => unreachable!("validated PSTB display name"),
            };
            if waveform.y_values[index] != expected {
                return Err(format!(
                    "PSTB display waveform '{name}' is not a leading complete-spectrum projection"
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
impl WorkerSimulationResult {
    fn estimated_numeric_payload_bytes(&self) -> usize {
        match self {
            WorkerSimulationResult::DcOp {
                configuration: _,
                validated_startup_directives: _,
                mna_node_names: _,
                mna_branch_names: _,
                mna_solution,
                node_voltages,
                branch_currents,
                device_report,
            } => sum_payload_bytes([
                f64_payload_bytes(node_voltages.len()),
                f64_payload_bytes(branch_currents.len()),
                f64_payload_bytes(mna_solution.len()),
                device_report
                    .as_ref()
                    .map_or(0, WorkerDeviceOpReport::estimated_numeric_payload_bytes),
            ]),
            WorkerSimulationResult::DcSweep {
                sweep_values,
                waveforms,
                measurements,
                evidence,
                ..
            } => sum_payload_bytes([
                f64_payload_bytes(sweep_values.len()),
                waveforms_payload_bytes(waveforms),
                measurements_payload_bytes(measurements),
                f64_payload_bytes(
                    evidence
                        .as_ref()
                        .map_or(0, |evidence| match &evidence.family {
                            crate::state::DcSweepFamily::Nested { values, .. } => values.len(),
                            _ => 0,
                        }),
                ),
            ]),
            WorkerSimulationResult::Transient {
                convergence,
                time,
                waveforms,
                measurements,
                events,
                spectra,
            } => sum_payload_bytes([
                convergence.as_ref().map_or(0, |quality| {
                    f64_payload_bytes(quality.transfer_value_count())
                }),
                f64_payload_bytes(time.len()),
                waveforms_payload_bytes(waveforms),
                measurements_payload_bytes(measurements),
                event_history_payload_bytes(events),
                f64_payload_bytes(
                    spectra
                        .iter()
                        .map(WorkerRecordedFftSpectrum::numeric_value_count)
                        .sum(),
                ),
            ]),
            WorkerSimulationResult::Fft {
                spectrum,
                convergence,
            } => sum_payload_bytes([
                convergence.as_ref().map_or(0, |quality| {
                    f64_payload_bytes(quality.transfer_value_count())
                }),
                f64_payload_bytes(spectrum.numeric_value_count()),
            ]),
            WorkerSimulationResult::Pss {
                measurements,
                operating_point,
            } => sum_payload_bytes([
                measurements_payload_bytes(measurements),
                pss_operating_point_payload_bytes(operating_point),
            ]),
            WorkerSimulationResult::Ac {
                convergence,
                frequencies,
                waveforms,
                measurements,
                reference_impedances_ohm,
                noise_reference_temperature_kelvin,
            } => sum_payload_bytes([
                convergence.as_ref().map_or(0, |quality| {
                    f64_payload_bytes(quality.transfer_value_count())
                }),
                f64_payload_bytes(frequencies.len()),
                waveforms_payload_bytes(waveforms),
                measurements_payload_bytes(measurements),
                f64_payload_bytes(reference_impedances_ohm.as_ref().map_or(0, Vec::len)),
                f64_payload_bytes(usize::from(noise_reference_temperature_kelvin.is_some())),
            ]),
            WorkerSimulationResult::Pstb {
                modes,
                mode_indices,
                waveforms,
                ..
            } => sum_payload_bytes([
                f64_payload_bytes(modes.len().saturating_mul(5)),
                f64_payload_bytes(mode_indices.len()),
                waveforms_payload_bytes(waveforms),
                f64_payload_bytes(5),
            ]),
            WorkerSimulationResult::Qpnoise {
                frequencies,
                waveforms,
                response,
            } => sum_payload_bytes([
                f64_payload_bytes(frequencies.len()),
                waveforms_payload_bytes(waveforms),
                qpnoise::response_bytes(response),
            ]),
            WorkerSimulationResult::Qpxf {
                frequencies,
                waveforms,
                response,
            } => sum_payload_bytes([
                f64_payload_bytes(frequencies.len()),
                waveforms_payload_bytes(waveforms),
                qpxf::response_bytes(response),
            ]),
            WorkerSimulationResult::Qpac {
                frequencies,
                waveforms,
                response,
            } => sum_payload_bytes([
                f64_payload_bytes(frequencies.len()),
                waveforms_payload_bytes(waveforms),
                f64_payload_bytes(
                    response
                        .unit_solutions
                        .iter()
                        .flat_map(|s| &s.spectra)
                        .map(Vec::len)
                        .sum::<usize>()
                        .saturating_mul(2),
                ),
                f64_payload_bytes(frequencies.len().saturating_mul(8)),
                response
                    .metadata
                    .tuples
                    .iter()
                    .map(|t| t.len().saturating_mul(4))
                    .sum(),
            ]),
            WorkerSimulationResult::Qpss {
                frequencies,
                tuples,
                waveforms,
                operating_point,
            } => sum_payload_bytes([
                f64_payload_bytes(frequencies.len()),
                waveforms_payload_bytes(waveforms),
                tuples
                    .iter()
                    .map(|tuple| tuple.len().saturating_mul(4))
                    .sum(),
                f64_payload_bytes(
                    operating_point
                        .spectra()
                        .iter()
                        .map(Vec::len)
                        .sum::<usize>()
                        .saturating_mul(2),
                ),
            ]),
            WorkerSimulationResult::Hb {
                frequencies,
                waveforms,
                measurements,
                operating_point,
            } => sum_payload_bytes([
                f64_payload_bytes(frequencies.len()),
                waveforms_payload_bytes(waveforms),
                measurements_payload_bytes(measurements),
                f64_payload_bytes(
                    operating_point
                        .spectral_state()
                        .iter()
                        .chain(operating_point.mna_branch_spectral_state())
                        .map(Vec::len)
                        .sum::<usize>()
                        .saturating_mul(2),
                ),
            ]),
            WorkerSimulationResult::Noise {
                frequencies,
                output_noise,
                input_noise,
                contributors,
                summary,
                measurements,
            } => sum_payload_bytes([
                f64_payload_bytes(frequencies.len()),
                f64_payload_bytes(output_noise.len()),
                input_noise
                    .as_ref()
                    .map_or(0, |values| f64_payload_bytes(values.len())),
                vec_map_payload_bytes(contributors),
                summary
                    .as_ref()
                    .map_or(0, WorkerNoiseSummary::estimated_numeric_payload_bytes),
                measurements_payload_bytes(measurements),
            ]),
            WorkerSimulationResult::PoleZero { poles, zeros, .. } => sum_payload_bytes([
                complex_pair_payload_bytes(poles.len()),
                complex_pair_payload_bytes(zeros.len()),
                f64_payload_bytes(1),
            ]),
            WorkerSimulationResult::SensitivityStudy { evidence } => sum_payload_bytes([
                // The grid, and its nominal output as a complex pair.
                f64_payload_bytes(evidence.point_count().saturating_mul(3)),
                evidence
                    .rows
                    .iter()
                    .map(|row| {
                        row.raw
                            .len()
                            .saturating_add(row.normalized.len())
                            .saturating_add(row.phase.len())
                            .saturating_mul(std::mem::size_of::<
                                rspice_core::analysis::sensitivity::SensitivityValue<f64>,
                            >())
                    })
                    .fold(0_usize, usize::saturating_add),
            ]),
            WorkerSimulationResult::DcMismatch { evidence } => sum_payload_bytes([
                // Five sigmas, the nominal value and the multiplier.
                f64_payload_bytes(7),
                evidence
                    .contributors
                    .len()
                    .saturating_mul(std::mem::size_of::<
                        crate::state::DcMismatchContributorEvidence,
                    >()),
            ]),
            WorkerSimulationResult::TransferFunction {
                gain,
                input_resistance,
                output_resistance,
                nominal_input,
                nominal_output,
                ..
            } => f64_payload_bytes(
                [
                    gain.is_some(),
                    input_resistance.is_some(),
                    output_resistance.is_some(),
                ]
                .into_iter()
                .filter(|present| *present)
                .count()
                    + usize::from(nominal_input.is_some())
                    + usize::from(nominal_output.is_some()),
            ),
            WorkerSimulationResult::Parametric {
                sweep_values,
                waveforms,
                ..
            } => sum_payload_bytes([
                f64_payload_bytes(sweep_values.len()),
                waveforms_payload_bytes(waveforms),
            ]),
            WorkerSimulationResult::Corner {
                x_values,
                temperatures_c,
                waveforms,
                ..
            } => sum_payload_bytes([
                f64_payload_bytes(x_values.len()),
                f64_payload_bytes(temperatures_c.len()),
                waveforms_payload_bytes(waveforms),
            ]),
            WorkerSimulationResult::MonteCarlo {
                variables,
                member_measurements,
                ..
            } => {
                let observations = member_measurements.iter().fold(0usize, |total, member| {
                    total
                        .saturating_add(member.measurements.len())
                        .saturating_add(2)
                });
                variables
                    .iter()
                    .map(WorkerMonteCarloVariable::estimated_numeric_payload_bytes)
                    .fold(f64_payload_bytes(observations), |total, bytes| {
                        total.saturating_add(bytes)
                    })
            }
            WorkerSimulationResult::ReliabilityMission {
                years,
                waveforms,
                response,
            } => sum_payload_bytes([
                f64_payload_bytes(years.len()),
                waveforms_payload_bytes(waveforms),
                crate::state::AnalysisResultPayload::reliability_response_bytes(response),
            ]),
            WorkerSimulationResult::Reliability {
                years,
                waveforms,
                device_results,
            } => sum_payload_bytes([
                f64_payload_bytes(years.len()),
                waveforms_payload_bytes(waveforms),
                reliability_results_payload_bytes(device_results),
            ]),
            WorkerSimulationResult::Optimization {
                iterations,
                waveforms,
                best_variables,
                ..
            } => sum_payload_bytes([
                f64_payload_bytes(iterations.len()),
                waveforms_payload_bytes(waveforms),
                f64_payload_bytes(best_variables.len()),
                f64_payload_bytes(1),
            ]),
            WorkerSimulationResult::Soa {
                convergence,
                time,
                waveforms,
                violations,
                evaluations,
            } => sum_payload_bytes([
                convergence.as_ref().map_or(0, |quality| {
                    f64_payload_bytes(quality.transfer_value_count())
                }),
                f64_payload_bytes(time.len()),
                waveforms_payload_bytes(waveforms),
                soa_violations_payload_bytes(violations),
                soa_evaluations_payload_bytes(evaluations),
            ]),
            WorkerSimulationResult::MeasurementsOnly { measurements } => {
                f64_payload_bytes(measurements.len())
            }
        }
    }
}

/// 18: transient-source convergence evidence survives result transport.
/// 19: exact DC curve identities, coordinates and traversal survive transport.
/// 20: retained PSS orbits carry canonical MNA branch-current samples.
/// 21: transient results retain exact current impulse histories and coverage.
/// 22: live samples carry sequenced current-impulse suffixes and loss accounting.
/// 23: a transient carries the spectra of the `.fft` cards it evaluated, and a
///     recorded FFT result is its own response variant.
/// 24: a sensitivity result is one study — its filter, every frequency it
///     solved, and a raw, normalized and phase column per variable — in place
///     of two maps read at a single point.
/// Earlier workers silently omit numerical quality or current observations.
/// 25: Monte Carlo retains complete parameter-stream trial identities and failed observations.
const WORKER_RESPONSE_TRANSPORT_PROTOCOL: u8 = 25;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct WorkerResponseTransport {
    pub protocol: u8,
    pub response: WorkerResponseTransportMetadata,
    pub buffers: Vec<Vec<f64>>,
}

impl TryFrom<SimulationResult> for WorkerSimulationResult {
    type Error = SimulationError;

    fn try_from(value: SimulationResult) -> Result<Self, Self::Error> {
        match value {
            SimulationResult::DcOp(result) => Ok(Self::DcOp {
                configuration: result.configuration,
                validated_startup_directives: result.validated_startup_directives,
                mna_node_names: result.mna_node_names,
                mna_branch_names: result.mna_branch_names,
                mna_solution: result.mna_solution,
                node_voltages: result.node_voltages,
                branch_currents: result.branch_currents,
                device_report: result.device_report.map(WorkerDeviceOpReport::from),
            }),
            SimulationResult::DcSweep {
                evidence,
                sweep_var,
                sweep_values,
                waveforms,
                measurements,
            } => Ok(Self::DcSweep {
                evidence: evidence.map(std::sync::Arc::unwrap_or_clone),
                sweep_var,
                sweep_values,
                waveforms: worker_waveforms(waveforms),
                measurements: worker_measurements(measurements),
            }),
            SimulationResult::Fft {
                spectrum,
                convergence,
            } => Ok(Self::Fft {
                spectrum: WorkerRecordedFftSpectrum::from(spectrum.as_ref()),
                convergence: convergence.map(std::sync::Arc::unwrap_or_clone),
            }),
            SimulationResult::Transient {
                time,
                waveforms,
                measurements,
                periodic_state,
                convergence,
                events,
                spectra,
            } => match periodic_state {
                Some(operating_point) => {
                    if convergence.is_some() {
                        return Err(SimulationError::SolverError(
                            "PSS display results cannot claim a transient source time basis"
                                .to_owned(),
                        ));
                    }
                    validate_pss_display_contract(&time, &waveforms, &operating_point)?;
                    Ok(Self::Pss {
                        measurements: worker_measurements(measurements),
                        operating_point: std::sync::Arc::unwrap_or_clone(operating_point),
                    })
                }
                None => Ok(Self::Transient {
                    time,
                    waveforms: worker_waveforms(waveforms),
                    measurements: worker_measurements(measurements),
                    convergence: convergence.map(std::sync::Arc::unwrap_or_clone),
                    events: events.into(),
                    spectra: recorded_fft::worker_spectra(spectra),
                }),
            },
            SimulationResult::Ac {
                convergence,
                frequencies,
                waveforms,
                measurements,
                reference_impedances_ohm,
                noise_reference_temperature_kelvin,
            } => Ok(Self::Ac {
                convergence: convergence.map(std::sync::Arc::unwrap_or_clone),
                frequencies,
                waveforms: worker_waveforms(waveforms),
                measurements: worker_measurements(measurements),
                reference_impedances_ohm,
                noise_reference_temperature_kelvin,
            }),
            SimulationResult::Pstb {
                period,
                fundamental_frequency,
                stability_threshold,
                probe_instance,
                detect_subharmonics,
                modes,
                floquet_evidence,
                orbit_kind,
                trivial_multiplier_index,
                stability_verdict,
                stability_classification,
                min_stability_margin_db,
                max_multiplier_magnitude,
                num_unstable,
                subharmonics,
                converged,
                iterations,
                mode_indices,
                waveforms,
            } => {
                let result = Self::Pstb {
                    period,
                    fundamental_frequency,
                    stability_threshold,
                    probe_instance,
                    detect_subharmonics,
                    modes: modes
                        .into_iter()
                        .map(|mode| WorkerPstbFloquetMode {
                            multiplier: mode.multiplier,
                            exponent: mode.exponent,
                            probe_participation: mode.probe_participation,
                            is_unstable: mode.is_unstable,
                            is_trivial: mode.is_trivial,
                            subharmonic_order: mode.subharmonic_order,
                        })
                        .collect(),
                    floquet_evidence,
                    orbit_kind,
                    trivial_multiplier_index,
                    stability_verdict,
                    stability_classification: WorkerPstbStabilityClassification::try_from_core(
                        stability_classification,
                    )?,
                    min_stability_margin_db,
                    max_multiplier_magnitude,
                    num_unstable,
                    subharmonics,
                    converged,
                    iterations,
                    mode_indices,
                    waveforms: worker_waveforms(waveforms),
                };
                validate_worker_pstb_result(&result).map_err(SimulationError::InvalidConfig)?;
                validate_worker_qpss_result(&result).map_err(SimulationError::InvalidConfig)?;
                validate_worker_qpac_result(&result).map_err(SimulationError::InvalidConfig)?;
                validate_worker_qpxf_result(&result).map_err(SimulationError::InvalidConfig)?;
                Ok(result)
            }
            SimulationResult::Qpnoise {
                frequencies,
                waveforms,
                response,
            } => {
                let result = Self::Qpnoise {
                    frequencies,
                    waveforms: worker_waveforms(waveforms),
                    response: Arc::unwrap_or_clone(response),
                };
                validate_worker_qpnoise_result(&result).map_err(SimulationError::InvalidConfig)?;
                Ok(result)
            }
            SimulationResult::Qpxf {
                frequencies,
                waveforms,
                response,
            } => {
                let result = Self::Qpxf {
                    frequencies,
                    waveforms: worker_waveforms(waveforms),
                    response: Arc::unwrap_or_clone(response),
                };
                validate_worker_qpxf_result(&result).map_err(SimulationError::InvalidConfig)?;
                Ok(result)
            }
            SimulationResult::Qpac {
                frequencies,
                waveforms,
                response,
            } => {
                let result = Self::Qpac {
                    frequencies,
                    waveforms: worker_waveforms(waveforms),
                    response: Arc::unwrap_or_clone(response),
                };
                validate_worker_qpac_result(&result).map_err(SimulationError::InvalidConfig)?;
                validate_worker_qpxf_result(&result).map_err(SimulationError::InvalidConfig)?;
                Ok(result)
            }
            SimulationResult::Qpss {
                frequencies,
                tuples,
                waveforms,
                operating_point,
            } => {
                let result = Self::Qpss {
                    frequencies,
                    tuples,
                    waveforms: worker_waveforms(waveforms),
                    operating_point: Arc::unwrap_or_clone(operating_point),
                };
                validate_worker_qpss_result(&result).map_err(SimulationError::InvalidConfig)?;
                validate_worker_qpac_result(&result).map_err(SimulationError::InvalidConfig)?;
                validate_worker_qpxf_result(&result).map_err(SimulationError::InvalidConfig)?;
                Ok(result)
            }
            SimulationResult::HarmonicBalance {
                frequencies,
                waveforms,
                measurements,
                operating_point,
            } => Ok(Self::Hb {
                frequencies,
                waveforms: worker_waveforms(waveforms),
                measurements: worker_measurements(measurements),
                operating_point: std::sync::Arc::unwrap_or_clone(operating_point),
            }),
            SimulationResult::Noise {
                frequencies,
                output_noise,
                input_noise,
                contributors,
                summary,
                measurements,
            } => Ok(Self::Noise {
                frequencies,
                output_noise,
                input_noise,
                contributors,
                summary: summary.map(WorkerNoiseSummary::from),
                measurements: worker_measurements(measurements),
            }),
            SimulationResult::PoleZero {
                poles,
                zeros,
                pole_evidence,
                zero_evidence,
                gain,
            } => Ok(Self::PoleZero {
                poles,
                zeros,
                pole_evidence,
                zero_evidence,
                gain,
            }),
            SimulationResult::SensitivityStudy { evidence } => Ok(Self::SensitivityStudy {
                evidence: std::sync::Arc::unwrap_or_clone(evidence),
            }),
            SimulationResult::DcMismatch { evidence } => Ok(Self::DcMismatch {
                evidence: (*evidence).clone(),
            }),
            SimulationResult::TransferFunction {
                input_source,
                output_expression,
                input_quantity,
                output_quantity,
                input_unit,
                output_unit,
                normalization,
                accuracy,
                gain,
                input_resistance,
                output_resistance,
                nominal_input,
                nominal_output,
            } => Ok(Self::TransferFunction {
                input_source,
                output_expression,
                input_quantity: WorkerTransferFunctionQuantity::from(input_quantity),
                output_quantity: WorkerTransferFunctionQuantity::from(output_quantity),
                input_unit,
                output_unit,
                normalization,
                accuracy,
                gain: gain.map(WorkerTransferFunctionScalar::from),
                input_resistance: input_resistance.map(WorkerTransferFunctionScalar::from),
                output_resistance: output_resistance.map(WorkerTransferFunctionScalar::from),
                nominal_input,
                nominal_output,
            }),
            SimulationResult::Parametric {
                target,
                sweep_values,
                waveforms,
                num_failures,
                member_measurements,
            } => Ok(Self::Parametric {
                target,
                sweep_values,
                waveforms: worker_waveforms(waveforms),
                num_failures,
                member_measurements,
            }),
            SimulationResult::Corner {
                x_values,
                x_label,
                x_unit,
                temperatures_c,
                corner_labels,
                waveforms,
                num_failures,
                member_measurements,
            } => Ok(Self::Corner {
                x_values,
                x_label,
                x_unit,
                temperatures_c,
                corner_labels,
                waveforms: worker_waveforms(waveforms),
                num_failures,
                member_measurements,
            }),
            SimulationResult::MonteCarlo {
                seed,
                runs_requested,
                runs_completed,
                num_failures,
                all_converged,
                variables,
                member_measurements,
            } => Ok(Self::MonteCarlo {
                seed,
                runs_requested,
                runs_completed,
                num_failures,
                all_converged,
                variables: variables
                    .into_iter()
                    .map(WorkerMonteCarloVariable::from)
                    .collect(),
                member_measurements,
            }),
            SimulationResult::ReliabilityMission {
                years,
                waveforms,
                response,
            } => {
                let result = Self::ReliabilityMission {
                    years,
                    waveforms: worker_waveforms(waveforms),
                    response: Arc::unwrap_or_clone(response),
                };
                validate_worker_reliability_result(&result)
                    .map_err(SimulationError::InvalidConfig)?;
                Ok(result)
            }
            SimulationResult::Reliability {
                years,
                waveforms,
                device_results,
            } => Ok(Self::Reliability {
                years,
                waveforms: worker_waveforms(waveforms),
                device_results: device_results
                    .into_iter()
                    .map(WorkerReliabilityResult::from)
                    .collect(),
            }),
            SimulationResult::Optimization {
                iterations,
                waveforms,
                best_cost,
                best_variables,
                converged,
            } => Ok(Self::Optimization {
                iterations,
                waveforms: worker_waveforms(waveforms),
                best_cost,
                best_variables,
                converged,
            }),
            SimulationResult::Soa {
                convergence,
                time,
                waveforms,
                violations,
                evaluations,
            } => Ok(Self::Soa {
                convergence: convergence.map(std::sync::Arc::unwrap_or_clone),
                time,
                waveforms: worker_waveforms(waveforms),
                violations: violations
                    .into_iter()
                    .map(WorkerSoAViolation::from)
                    .collect(),
                evaluations: evaluations
                    .into_iter()
                    .map(WorkerSoAEvaluation::from)
                    .collect(),
            }),
            SimulationResult::MeasurementsOnly { measurements } => {
                Ok(Self::MeasurementsOnly { measurements })
            }
        }
    }
}

impl From<WorkerSimulationResult> for SimulationResult {
    fn from(value: WorkerSimulationResult) -> Self {
        match value {
            WorkerSimulationResult::DcOp {
                configuration,
                validated_startup_directives,
                mna_node_names,
                mna_branch_names,
                mna_solution,
                node_voltages,
                branch_currents,
                device_report,
            } => Self::DcOp(Box::new(DcOpResult {
                configuration,
                validated_startup_directives,
                mna_node_names,
                mna_branch_names,
                mna_solution,
                node_voltages,
                branch_currents,
                device_report: device_report.map(rspice_core::circuit::DeviceOpReport::from),
            })),
            WorkerSimulationResult::DcSweep {
                evidence,
                sweep_var,
                sweep_values,
                waveforms,
                measurements,
            } => Self::DcSweep {
                evidence: evidence.map(std::sync::Arc::new),
                sweep_var,
                sweep_values,
                waveforms: waveform_map(waveforms),
                measurements: measure_results(measurements),
            },
            WorkerSimulationResult::Transient {
                time,
                waveforms,
                measurements,
                convergence,
                events,
                spectra,
            } => Self::Transient {
                time,
                waveforms: waveform_map(waveforms),
                measurements: measure_results(measurements),
                periodic_state: None,
                convergence: convergence.map(std::sync::Arc::new),
                events: events.into(),
                spectra: recorded_fft::recorded_spectra(spectra),
            },
            WorkerSimulationResult::Fft {
                spectrum,
                convergence,
            } => Self::Fft {
                spectrum: std::sync::Arc::new(spectrum.into()),
                convergence: convergence.map(std::sync::Arc::new),
            },
            WorkerSimulationResult::Pss {
                measurements,
                operating_point,
            } => simulation_result_from_worker_pss(measurements, operating_point),
            WorkerSimulationResult::Ac {
                convergence,
                frequencies,
                waveforms,
                measurements,
                reference_impedances_ohm,
                noise_reference_temperature_kelvin,
            } => Self::Ac {
                convergence: convergence.map(std::sync::Arc::new),
                frequencies,
                waveforms: waveform_map(waveforms),
                measurements: measure_results(measurements),
                reference_impedances_ohm,
                noise_reference_temperature_kelvin,
            },
            WorkerSimulationResult::Pstb {
                period,
                fundamental_frequency,
                stability_threshold,
                probe_instance,
                detect_subharmonics,
                modes,
                floquet_evidence,
                orbit_kind,
                trivial_multiplier_index,
                stability_verdict,
                stability_classification,
                min_stability_margin_db,
                max_multiplier_magnitude,
                num_unstable,
                subharmonics,
                converged,
                iterations,
                mode_indices,
                waveforms,
            } => Self::Pstb {
                period,
                fundamental_frequency,
                stability_threshold,
                probe_instance,
                detect_subharmonics,
                modes: modes
                    .into_iter()
                    .map(|mode| crate::simulation::results::PstbFloquetMode {
                        multiplier: mode.multiplier,
                        exponent: mode.exponent,
                        probe_participation: mode.probe_participation,
                        is_unstable: mode.is_unstable,
                        is_trivial: mode.is_trivial,
                        subharmonic_order: mode.subharmonic_order,
                    })
                    .collect(),
                floquet_evidence,
                orbit_kind,
                trivial_multiplier_index,
                stability_verdict,
                stability_classification: stability_classification.into_core(),
                min_stability_margin_db,
                max_multiplier_magnitude,
                num_unstable,
                subharmonics,
                converged,
                iterations,
                mode_indices,
                waveforms: waveform_map(waveforms),
            },
            WorkerSimulationResult::Qpnoise {
                frequencies,
                waveforms,
                response,
            } => Self::Qpnoise {
                frequencies,
                waveforms: waveform_map(waveforms),
                response: Arc::new(response),
            },
            WorkerSimulationResult::Qpxf {
                frequencies,
                waveforms,
                response,
            } => Self::Qpxf {
                frequencies,
                waveforms: waveform_map(waveforms),
                response: Arc::new(response),
            },
            WorkerSimulationResult::Qpac {
                frequencies,
                waveforms,
                response,
            } => Self::Qpac {
                frequencies,
                waveforms: waveform_map(waveforms),
                response: Arc::new(response),
            },
            WorkerSimulationResult::Qpss {
                frequencies,
                tuples,
                waveforms,
                operating_point,
            } => Self::Qpss {
                frequencies,
                tuples,
                waveforms: waveform_map(waveforms),
                operating_point: Arc::new(operating_point),
            },
            WorkerSimulationResult::Hb {
                frequencies,
                waveforms,
                measurements,
                operating_point,
            } => Self::HarmonicBalance {
                frequencies,
                waveforms: waveform_map(waveforms),
                measurements: measure_results(measurements),
                operating_point: std::sync::Arc::new(operating_point),
            },
            WorkerSimulationResult::Noise {
                frequencies,
                output_noise,
                input_noise,
                contributors,
                summary,
                measurements,
            } => Self::Noise {
                frequencies,
                output_noise,
                input_noise,
                contributors,
                summary: summary.map(NoiseSummary::from),
                measurements: measure_results(measurements),
            },
            WorkerSimulationResult::PoleZero {
                poles,
                zeros,
                pole_evidence,
                zero_evidence,
                gain,
            } => Self::PoleZero {
                poles,
                zeros,
                pole_evidence,
                zero_evidence,
                gain,
            },
            WorkerSimulationResult::SensitivityStudy { evidence } => Self::SensitivityStudy {
                evidence: std::sync::Arc::new(evidence),
            },
            WorkerSimulationResult::DcMismatch { evidence } => Self::DcMismatch {
                evidence: std::sync::Arc::new(evidence),
            },
            WorkerSimulationResult::TransferFunction {
                input_source,
                output_expression,
                input_quantity,
                output_quantity,
                input_unit,
                output_unit,
                normalization,
                accuracy,
                gain,
                input_resistance,
                output_resistance,
                nominal_input,
                nominal_output,
            } => Self::TransferFunction {
                input_source,
                output_expression,
                input_quantity: TransferFunctionQuantity::from(input_quantity),
                output_quantity: TransferFunctionQuantity::from(output_quantity),
                input_unit,
                output_unit,
                normalization,
                accuracy,
                gain: gain.map(TransferFunctionScalar::from),
                input_resistance: input_resistance.map(TransferFunctionScalar::from),
                output_resistance: output_resistance.map(TransferFunctionScalar::from),
                nominal_input,
                nominal_output,
            },
            WorkerSimulationResult::Parametric {
                target,
                sweep_values,
                waveforms,
                num_failures,
                member_measurements,
            } => Self::Parametric {
                target,
                sweep_values,
                waveforms: waveform_map(waveforms),
                num_failures,
                member_measurements,
            },
            WorkerSimulationResult::Corner {
                x_values,
                x_label,
                x_unit,
                temperatures_c,
                corner_labels,
                waveforms,
                num_failures,
                member_measurements,
            } => Self::Corner {
                x_values,
                x_label,
                x_unit,
                temperatures_c,
                corner_labels,
                waveforms: waveform_map(waveforms),
                num_failures,
                member_measurements,
            },
            WorkerSimulationResult::MonteCarlo {
                seed,
                runs_requested,
                runs_completed,
                num_failures,
                all_converged,
                variables,
                member_measurements,
            } => Self::MonteCarlo {
                seed,
                runs_requested,
                runs_completed,
                num_failures,
                all_converged,
                variables: variables
                    .into_iter()
                    .map(MonteCarloVariableResult::from)
                    .collect(),
                member_measurements,
            },
            WorkerSimulationResult::ReliabilityMission {
                years,
                waveforms,
                response,
            } => Self::ReliabilityMission {
                years,
                waveforms: waveform_map(waveforms),
                response: Arc::new(response),
            },
            WorkerSimulationResult::Reliability {
                years,
                waveforms,
                device_results,
            } => Self::Reliability {
                years,
                waveforms: waveform_map(waveforms),
                device_results: device_results
                    .into_iter()
                    .map(ReliabilityResult::from)
                    .collect(),
            },
            WorkerSimulationResult::Optimization {
                iterations,
                waveforms,
                best_cost,
                best_variables,
                converged,
            } => Self::Optimization {
                iterations,
                waveforms: waveform_map(waveforms),
                best_cost,
                best_variables,
                converged,
            },
            WorkerSimulationResult::Soa {
                convergence,
                time,
                waveforms,
                violations,
                evaluations,
            } => Self::Soa {
                convergence: convergence.map(std::sync::Arc::new),
                time,
                waveforms: waveform_map(waveforms),
                violations: violations.into_iter().map(SoAViolation::from).collect(),
                evaluations: evaluations.into_iter().map(SoAEvaluation::from).collect(),
            },
            WorkerSimulationResult::MeasurementsOnly { measurements } => {
                Self::MeasurementsOnly { measurements }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum WorkerTransferFunctionQuantity {
    Voltage,
    Current,
}

#[cfg(test)]
mod tests;
