//! The simulation worker contract.
//!
//! Everything crossing the boundary between the application and the engine
//! worker: the request and result messages, their transport encoding, and
//! the state each side must hold for a request to be replayable. The two
//! sides run in different threads natively and different contexts in the
//! browser, so this is a serialized contract rather than a shared type.

mod analysis;
mod conversions;
mod transport;

pub(crate) use conversions::*;
pub(crate) use transport::*;

pub(crate) use analysis::*;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, atomic::AtomicBool};

use serde::{Deserialize, Serialize};

use super::{NetlistInput, SimulationError, SimulationRequest, SpecExecutionOptions};
use crate::services::safety::{
    SoAEvaluation, SoAParameter, SoARuleVerdict, SoAViolation, ViolationSeverity,
};
use crate::simulation::config::{
    AcAnalysisConfig, AcSweepType, AnalysisConfig, DcSweepConfig, NoiseAnalysisConfig,
    NoiseContributionDetail, NoiseIntegrationMode, NoiseSweepType, PoleZeroConfig, PzAnalysisType,
    SensitivityConfig, TransientAnalysisConfig,
};
use crate::simulation::multi_run::{
    AnalysisSpec, EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve,
    FrequencySweep, HbToneSpec, OptimizationAlgorithm, OptimizationGoal, OptimizationVariable,
    PssMethod, SpPort, TfAccuracy, TfNormalization,
};
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
    #[serde(default)]
    pub project_veriloga_runtimes: crate::simulation::veriloga::PreparedVerilogARuntimeSet,
    #[serde(default)]
    pub(in crate::simulation) dependencies:
        crate::simulation::execution::ResolvedExecutionDependencies,
    #[serde(default)]
    pub(in crate::simulation) environment: Option<super::AnalysisExecutionEnvironment>,
    #[serde(default)]
    pub(in crate::simulation) stream_transient_samples: bool,
}

#[cfg(any(target_arch = "wasm32", test))]
pub(crate) const WORKER_REQUEST_TRANSPORT_PROTOCOL: u8 = 8;

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

const fn worker_default_pss_stabilization_cycles() -> usize {
    20
}

const fn worker_default_pss_shooting_points() -> usize {
    512
}

const fn worker_default_true() -> bool {
    true
}

const fn worker_default_noise_temperature() -> f64 {
    rspice_core::constants::TEMP_REFERENCE
}

fn worker_default_noise_reference_node() -> String {
    "0".to_owned()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerAnalysisSpec {
    #[serde(rename = "DcOp")]
    LegacyDcOp,
    #[serde(rename = "DcOpConfigured")]
    DcOp(crate::simulation::dialog::OpConfig),
    DcSweep {
        source_name: String,
        start: f64,
        stop: f64,
        step: f64,
        source2: Option<String>,
        start2: Option<f64>,
        stop2: Option<f64>,
        step2: Option<f64>,
        /// Sweep out and back as one continued solve. Defaulted on read so a
        /// worker message from an older build is understood as the one-way
        /// sweep it described.
        #[serde(default)]
        hysteresis: bool,
    },
    Transient {
        stop_time: f64,
        step_time: f64,
        start_time: f64,
        max_timestep: Option<f64>,
        uic: bool,
    },
    Ac {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: WorkerSweepType,
    },
    AcData {
        table_name: String,
        frequencies: Vec<f64>,
    },
    Noise {
        output_node: String,
        #[serde(default = "worker_default_noise_reference_node")]
        reference_node: String,
        #[serde(default)]
        input_source: String,
        start_freq: f64,
        stop_freq: f64,
        points_per_decade: usize,
        #[serde(default)]
        sweep: NoiseSweepType,
        #[serde(default)]
        explicit_frequencies: Option<Vec<f64>>,
        #[serde(default)]
        data_table_name: Option<String>,
        #[serde(default)]
        contribution_detail: NoiseContributionDetail,
        #[serde(default)]
        integration_mode: NoiseIntegrationMode,
        temperature: f64,
    },
    Sensitivity {
        output_var: String,
        ac_mode: bool,
        frequency: Option<f64>,
    },
    PoleZero {
        input_node: String,
        input_ref: String,
        output_node: String,
        output_ref: String,
        transfer_type: String,
        analysis_type: String,
    },
    Tf {
        input_source: String,
        output_expression: String,
        transfer_gain: bool,
        input_resistance: bool,
        output_resistance: bool,
        normalization: TfNormalization,
        accuracy: TfAccuracy,
    },
    Pac,
    Pxf,
    Pnoise,
    Pstb,
    Parametric,
    Corner,
    MonteCarlo {
        #[serde(default)]
        variation_source: crate::simulation::dialog::McVariationSource,
    },
    Reliability {
        target_years: Vec<f64>,
        enable_hci: bool,
        enable_nbti: bool,
        enable_em: bool,
        min_stress_voltage: f64,
    },
    Optimization {
        variables: Vec<OptimizationVariable>,
        objective_node: String,
        objective_ref: String,
        goal: OptimizationGoal,
        target: Option<f64>,
        algorithm: OptimizationAlgorithm,
        max_iterations: usize,
        cost_tolerance: f64,
        fd_step: f64,
        initial_step: f64,
        min_step: f64,
    },
    Soa {
        stop_time: f64,
        step_time: f64,
        check_vgs_max: bool,
        max_vgs: f64,
        check_vds_max: bool,
        max_vds: f64,
        check_vbe_max: bool,
        max_vbe: f64,
        check_vce_max: bool,
        max_vce: f64,
    },
    Stb {
        probe_node: String,
        start_freq: f64,
        stop_freq: f64,
        sweep: WorkerSweepType,
        points_per_decade: usize,
        #[serde(default = "worker_default_true")]
        compute_nyquist: bool,
    },
    SParameter {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: WorkerSweepType,
        z0: f64,
        ports: Vec<SpPort>,
    },
    Disto {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: WorkerSweepType,
        f2_over_f1: Option<f64>,
    },
    Pss {
        #[serde(default)]
        method: PssMethod,
        fundamental_freq: f64,
        /// A request that named no tone restores as one that named no tone;
        /// no reader can supply a source name the design does not carry.
        #[serde(default)]
        tone_sources: Vec<String>,
        #[serde(default = "worker_default_pss_stabilization_cycles")]
        tstab_periods: usize,
        #[serde(default = "worker_default_pss_shooting_points")]
        points_per_period: usize,
        #[serde(alias = "period_tolerance")]
        tolerance: f64,
        #[serde(default)]
        oscillator_mode: bool,
        #[serde(default)]
        oscillator_node: Option<String>,
        num_harmonics: usize,
    },
    HarmonicBalance {
        tones: Vec<HbToneSpec>,
        reltol: f64,
        abstol: f64,
        max_iterations: usize,
        damping: f64,
        oversample: usize,
        #[serde(default)]
        collocation_points: Option<usize>,
        max_mixing_order: usize,
        use_krylov: bool,
        gmres_restart: usize,
        source_stepping: bool,
        verbose: bool,
    },
    Envelope {
        fundamental_freq: f64,
        #[serde(default)]
        additional_carrier_tones: Vec<f64>,
        stop_time: f64,
        num_harmonics: usize,
        #[serde(default, alias = "max_step")]
        envelope_step: Option<f64>,
        #[serde(default)]
        modulation_sources: Vec<String>,
        #[serde(default)]
        initial_periodic_solve: EnvelopeInitialPeriodicSolve,
        #[serde(default)]
        adaptive_mode: EnvelopeAdaptiveMode,
        #[serde(default)]
        extraction_path: EnvelopeExtractionPath,
    },
    Fourier {
        fundamental_freq: f64,
        num_harmonics: usize,
        output_node: String,
        output_ref: String,
        start_time: f64,
        stop_time: f64,
        #[serde(default = "worker_default_true")]
        compute_thd: bool,
        #[serde(default)]
        normalize: bool,
    },
    /// Canonical complex analysis carried verbatim when a dedicated wire
    /// mirror would merely duplicate the domain shape. The dispatcher remains
    /// responsible for capability validation after lossless reconstruction.
    #[serde(alias = "ManifestPreview")]
    CanonicalSpec(AnalysisSpec),
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
            WorkerOutcome::Success(result) => Ok(SimulationResult::from(*result)),
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
    SolverError(String),
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
            SimulationError::SolverError(message) => Self::SolverError(message),
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
            WorkerSimulationError::SolverError(message) => Self::SolverError(message),
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
    },
    Transient {
        time: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
        measurements: Vec<WorkerMeasurement>,
        #[serde(default)]
        events: WorkerEventHistory,
    },
    /// PSS numerical evidence is transported once. Display waveforms are
    /// deterministically reconstructed from this retained orbit by the
    /// receiver instead of duplicating every sample across the worker edge.
    Pss {
        measurements: Vec<WorkerMeasurement>,
        operating_point: rspice_core::engine::PssOperatingPoint,
    },
    Ac {
        frequencies: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
        measurements: Vec<WorkerMeasurement>,
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
    Sensitivity {
        output: String,
        ac_mode: bool,
        frequency_hz: Option<f64>,
        sensitivities: HashMap<String, f64>,
        normalized: HashMap<String, f64>,
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
        time: Vec<f64>,
        waveforms: Vec<WorkerWaveform>,
        violations: Vec<WorkerSoAViolation>,
        evaluations: Vec<WorkerSoAEvaluation>,
    },
    MeasurementsOnly {
        measurements: HashMap<String, f64>,
    },
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
                ..
            } => sum_payload_bytes([
                f64_payload_bytes(sweep_values.len()),
                waveforms_payload_bytes(waveforms),
                measurements_payload_bytes(measurements),
            ]),
            WorkerSimulationResult::Transient {
                time,
                waveforms,
                measurements,
                events,
            } => sum_payload_bytes([
                f64_payload_bytes(time.len()),
                waveforms_payload_bytes(waveforms),
                measurements_payload_bytes(measurements),
                event_history_payload_bytes(events),
            ]),
            WorkerSimulationResult::Pss {
                measurements,
                operating_point,
            } => sum_payload_bytes([
                measurements_payload_bytes(measurements),
                pss_operating_point_payload_bytes(operating_point),
            ]),
            WorkerSimulationResult::Ac {
                frequencies,
                waveforms,
                measurements,
            } => sum_payload_bytes([
                f64_payload_bytes(frequencies.len()),
                waveforms_payload_bytes(waveforms),
                measurements_payload_bytes(measurements),
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
            WorkerSimulationResult::Sensitivity {
                frequency_hz,
                sensitivities,
                normalized,
                ..
            } => sum_payload_bytes([
                f64_payload_bytes(usize::from(frequency_hz.is_some())),
                f64_payload_bytes(sensitivities.len()),
                f64_payload_bytes(normalized.len()),
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
            WorkerSimulationResult::MonteCarlo { variables, .. } => variables
                .iter()
                .map(WorkerMonteCarloVariable::estimated_numeric_payload_bytes)
                .fold(0usize, |total, bytes| total.saturating_add(bytes)),
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
                time,
                waveforms,
                violations,
                evaluations,
            } => sum_payload_bytes([
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

const WORKER_RESPONSE_TRANSPORT_PROTOCOL: u8 = 10;

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
                sweep_var,
                sweep_values,
                waveforms,
                measurements,
            } => Ok(Self::DcSweep {
                sweep_var,
                sweep_values,
                waveforms: worker_waveforms(waveforms),
                measurements: worker_measurements(measurements),
            }),
            SimulationResult::Transient {
                time,
                waveforms,
                measurements,
                periodic_state,
                // Convergence metrics do not cross the worker boundary yet:
                // `WorkerSimulationResult` is a serde wire format and
                // `ConvergenceQuality` is not serializable. The browser build
                // therefore reports no convergence warnings. Bound explicitly
                // rather than swallowed by `..` so adding a field to this
                // result forces a decision here.
                convergence: _,
                events,
            } => match periodic_state {
                Some(operating_point) => {
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
                    events: events.into(),
                }),
            },
            SimulationResult::Ac {
                frequencies,
                waveforms,
                measurements,
            } => Ok(Self::Ac {
                frequencies,
                waveforms: worker_waveforms(waveforms),
                measurements: worker_measurements(measurements),
            }),
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
            SimulationResult::Sensitivity {
                output,
                ac_mode,
                frequency_hz,
                sensitivities,
                normalized,
            } => Ok(Self::Sensitivity {
                output,
                ac_mode,
                frequency_hz,
                sensitivities,
                normalized,
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
                time,
                waveforms,
                violations,
                evaluations,
            } => Ok(Self::Soa {
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
                sweep_var,
                sweep_values,
                waveforms,
                measurements,
            } => Self::DcSweep {
                sweep_var,
                sweep_values,
                waveforms: waveform_map(waveforms),
                measurements: measure_results(measurements),
            },
            WorkerSimulationResult::Transient {
                time,
                waveforms,
                measurements,
                events,
            } => Self::Transient {
                time,
                waveforms: waveform_map(waveforms),
                measurements: measure_results(measurements),
                periodic_state: None,
                // See the outbound conversion: not carried over the wire.
                convergence: Default::default(),
                events: events.into(),
            },
            WorkerSimulationResult::Pss {
                measurements,
                operating_point,
            } => simulation_result_from_worker_pss(measurements, operating_point),
            WorkerSimulationResult::Ac {
                frequencies,
                waveforms,
                measurements,
            } => Self::Ac {
                frequencies,
                waveforms: waveform_map(waveforms),
                measurements: measure_results(measurements),
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
            WorkerSimulationResult::Sensitivity {
                output,
                ac_mode,
                frequency_hz,
                sensitivities,
                normalized,
            } => Self::Sensitivity {
                output,
                ac_mode,
                frequency_hz,
                sensitivities,
                normalized,
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
                time,
                waveforms,
                violations,
                evaluations,
            } => Self::Soa {
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
