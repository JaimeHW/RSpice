//! Running a suite, and the rule that partial work never becomes evidence.
//!
//! A run walks every vector on every requested platform, reporting progress as
//! it goes.  Progress is observation only: cancellation or a failed vector
//! discards the in-progress run rather than publishing what completed, so
//! qualification evidence is always whole-suite or absent.  Nothing here
//! mutates a release — it only produces the outcomes that
//! [`super::promotion`] later gates on.

use super::*;

/// Observable suite progress. Emitting progress never publishes partial
/// qualification evidence; cancellation discards the in-progress run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualificationExecutionProgress {
    pub platform: QualificationPlatform,
    pub completed_vectors: usize,
    pub total_vectors: usize,
    pub current_vector_id: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum QualificationExecutionError {
    #[error(transparent)]
    InvalidContract(#[from] QualificationValidationError),
    #[error("qualification execution was cancelled; no partial platform run was published")]
    Cancelled,
    #[error("qualification execution session has already finished")]
    SessionFinished,
}

/// Result of one cooperative session step. An in-progress step never exposes
/// partial evidence. The terminal variant contains the first publishable,
/// fully validated platform run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QualificationExecutionStep {
    InProgress(QualificationExecutionProgress),
    Complete {
        progress: QualificationExecutionProgress,
        run: QualificationPlatformRun,
    },
}

/// Owned, frame-safe execution state. `step` executes at most one vector, so
/// callers can yield to egui or a browser event loop between vectors.
#[derive(Debug, Clone)]
pub struct QualificationExecutionSession {
    suite: QualificationSuite,
    source: ModelSourceEvidenceBinding,
    platform: QualificationPlatform,
    next_vector: usize,
    vector_outcomes: Vec<QualificationPlatformVectorOutcome>,
    cancelled: bool,
    finished: bool,
}

impl QualificationExecutionSession {
    pub fn try_new(
        suite: &QualificationSuite,
        source: &ModelSourceEvidenceBinding,
    ) -> Result<Self, QualificationExecutionError> {
        validate_execution_contract(suite, source)?;
        Ok(Self {
            suite: suite.clone(),
            source: source.clone(),
            platform: current_qualification_platform(),
            next_vector: 0,
            vector_outcomes: Vec::with_capacity(suite.vectors.len()),
            cancelled: false,
            finished: false,
        })
    }

    #[must_use]
    pub fn progress(&self) -> QualificationExecutionProgress {
        QualificationExecutionProgress {
            platform: self.platform,
            completed_vectors: self.next_vector,
            total_vectors: self.suite.vectors.len(),
            current_vector_id: self
                .suite
                .vectors
                .get(self.next_vector)
                .map(|value| value.id.clone()),
        }
    }

    /// Request cancellation before another vector begins. No partial run can
    /// be retrieved from a cancelled session.
    pub fn cancel(&mut self) {
        if !self.finished {
            self.cancelled = true;
        }
    }

    #[must_use]
    pub const fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    #[must_use]
    pub const fn is_finished(&self) -> bool {
        self.finished
    }

    /// Execute at most one vector and yield a progress or terminal record.
    pub fn step(
        &mut self,
        abort: &dyn rspice_core::AbortSignal,
    ) -> Result<QualificationExecutionStep, QualificationExecutionError> {
        if self.finished {
            return Err(QualificationExecutionError::SessionFinished);
        }
        if self.cancelled || abort.is_aborted() {
            self.cancelled = true;
            return Err(QualificationExecutionError::Cancelled);
        }
        let vector = self
            .suite
            .vectors
            .get(self.next_vector)
            .expect("a validated qualification suite is non-empty and unfinished");
        let outcome = match execute_qualification_vector(vector, self.platform, abort) {
            Ok(value) => value,
            Err(QualificationExecutionError::Cancelled) => {
                self.cancelled = true;
                return Err(QualificationExecutionError::Cancelled);
            }
            Err(error) => return Err(error),
        };
        if abort.is_aborted() {
            self.cancelled = true;
            return Err(QualificationExecutionError::Cancelled);
        }
        self.vector_outcomes
            .push(QualificationPlatformVectorOutcome {
                vector_id: vector.id.clone(),
                input_digest: vector.input_digest,
                outcome,
            });
        self.next_vector += 1;
        let progress = self.progress();
        abort.observe_progress(
            progress.completed_vectors as f64 / progress.total_vectors.max(1) as f64,
        );
        if self.next_vector < self.suite.vectors.len() {
            return Ok(QualificationExecutionStep::InProgress(progress));
        }

        let run = QualificationPlatformRun::try_new(
            self.platform,
            self.source.clone(),
            &self.suite,
            self.vector_outcomes.clone(),
        )?;
        self.finished = true;
        Ok(QualificationExecutionStep::Complete { progress, run })
    }
}

/// Deterministic, synchronous qualification executor shared by desktop and
/// WebAssembly builds. It uses only `rspice-core` APIs available on both
/// targets and publishes the runtime selected by the compilation target.
#[derive(Debug, Default, Clone, Copy)]
pub struct QualificationExecutionService;

impl QualificationExecutionService {
    pub fn execute_current_platform(
        suite: &QualificationSuite,
        source: &ModelSourceEvidenceBinding,
        abort: &dyn rspice_core::AbortSignal,
    ) -> Result<QualificationPlatformRun, QualificationExecutionError> {
        let mut ignore_progress = |_progress: &QualificationExecutionProgress| {};
        Self::execute_current_platform_with_progress(suite, source, abort, &mut ignore_progress)
    }

    pub fn execute_current_platform_with_progress(
        suite: &QualificationSuite,
        source: &ModelSourceEvidenceBinding,
        abort: &dyn rspice_core::AbortSignal,
        progress: &mut dyn FnMut(&QualificationExecutionProgress),
    ) -> Result<QualificationPlatformRun, QualificationExecutionError> {
        let mut session = QualificationExecutionSession::try_new(suite, source)?;
        progress(&session.progress());
        loop {
            match session.step(abort)? {
                QualificationExecutionStep::InProgress(step_progress) => {
                    progress(&step_progress);
                }
                QualificationExecutionStep::Complete {
                    progress: step_progress,
                    run,
                } => {
                    progress(&step_progress);
                    return Ok(run);
                }
            }
        }
    }

    /// Assemble immutable parity evidence only after independently produced
    /// Desktop and WebAssembly runs provide exact coverage.
    pub fn assemble_evidence(
        evidence_id: impl Into<String>,
        suite: &QualificationSuite,
        source: &ModelSourceEvidenceBinding,
        runs: Vec<QualificationPlatformRun>,
    ) -> QualificationResult<QualificationEvidence> {
        validate_execution_contract(suite, source)?;
        if runs.len() != QualificationPlatform::REQUIRED.len() {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::EvidenceCoverageMismatch,
                "platform_runs",
                "exactly one real Desktop run and one real WebAssembly run are required",
            ));
        }
        let mut platforms = BTreeSet::new();
        for (index, run) in runs.iter().enumerate() {
            if !platforms.insert(run.platform) {
                return Err(QualificationValidationError::new(
                    QualificationErrorCode::DuplicateId,
                    format!("platform_runs[{index}].platform"),
                    "qualification platform run is duplicated",
                ));
            }
            run.validate_bound(suite, source)?;
        }
        if !QualificationPlatform::REQUIRED
            .iter()
            .all(|platform| platforms.contains(platform))
        {
            return Err(QualificationValidationError::new(
                QualificationErrorCode::EvidenceCoverageMismatch,
                "platform_runs",
                "Desktop and WebAssembly platform runs are both required",
            ));
        }

        let mut vector_outcomes = Vec::with_capacity(suite.vectors.len());
        for vector in &suite.vectors {
            let mut outcomes = Vec::with_capacity(QualificationPlatform::REQUIRED.len());
            for platform in QualificationPlatform::REQUIRED {
                let run = runs
                    .iter()
                    .find(|value| value.platform == platform)
                    .expect("required platform coverage checked above");
                let platform_vector =
                    find_ci(&run.vector_outcomes, &vector.id, |value| &value.vector_id)
                        .expect("platform run coverage validated above");
                outcomes.push(platform_vector.outcome.clone());
            }
            vector_outcomes.push(QualificationVectorOutcome::try_new(
                vector.id.clone(),
                vector.input_digest,
                outcomes,
            )?);
        }
        let evidence = QualificationEvidence::try_new(
            evidence_id,
            source.clone(),
            suite.id.clone(),
            suite.revision,
            vector_outcomes,
        )?;
        evidence.validate_bound(suite, source)?;
        Ok(evidence)
    }
}

enum ExecutedQualificationAnalysis {
    OperatingPoint(rspice_core::SimulationResult),
    DcSweep(Vec<(f64, rspice_core::SimulationResult)>),
    AcSweep(Vec<rspice_core::analysis::AcResult>),
    Noise(Vec<rspice_core::analysis::NoiseResult>),
    Transient(rspice_core::engine::TransientResult),
}

pub(super) fn validate_execution_contract(
    suite: &QualificationSuite,
    source: &ModelSourceEvidenceBinding,
) -> QualificationResult<()> {
    suite.validate()?;
    source.validate("execution.source")?;
    source.require_project_bound("execution.source")?;
    for (index, vector) in suite.vectors.iter().enumerate() {
        vector.validate_source_binding(source, &format!("suite.vectors[{index}].source"))?;
    }
    Ok(())
}

fn execute_qualification_vector(
    vector: &QualificationVector,
    platform: QualificationPlatform,
    abort: &dyn rspice_core::AbortSignal,
) -> Result<PlatformQualificationOutcome, QualificationExecutionError> {
    let input = match std::str::from_utf8(&vector.executable_input) {
        Ok(value) => value,
        Err(error) => {
            return failed_platform_outcome(
                platform,
                QualificationFailureStage::Input,
                "input-not-utf8",
                format!("retained executable input is not UTF-8: {error}"),
            );
        }
    };
    let netlist = match rspice_core::Netlist::parse_validated_with_abort(input, abort) {
        Ok(value) => value,
        Err(error) if error.is_aborted() || abort.is_aborted() => {
            return Err(QualificationExecutionError::Cancelled);
        }
        Err(error) => {
            return failed_platform_outcome(
                platform,
                QualificationFailureStage::Parse,
                "netlist-parse-failed",
                error.to_string(),
            );
        }
    };
    let engine = rspice_core::Engine::default();
    let executed = match &vector.analysis {
        QualificationAnalysis::DcOperatingPoint => engine
            .run_dc_op_with_abort(&netlist, abort)
            .map(ExecutedQualificationAnalysis::OperatingPoint),
        QualificationAnalysis::DcSweep {
            source,
            start,
            stop,
            step,
        } => engine
            .run_dc_sweep_with_abort(&netlist, source, start.get(), stop.get(), step.get(), abort)
            .map(ExecutedQualificationAnalysis::DcSweep),
        QualificationAnalysis::AcSweep { frequencies } => {
            let frequencies = frequencies
                .iter()
                .map(|frequency| frequency.get())
                .collect::<Vec<_>>();
            engine
                .run_ac_with_abort(&netlist, &frequencies, abort)
                .map(ExecutedQualificationAnalysis::AcSweep)
        }
        QualificationAnalysis::Noise {
            output_node,
            output_reference,
            input_source,
            frequencies,
            temperature_kelvin,
        } => {
            let frequencies = frequencies
                .iter()
                .map(|frequency| frequency.get())
                .collect::<Vec<_>>();
            engine
                .run_noise_named_with_input_source_and_abort(
                    &netlist,
                    output_node,
                    output_reference.as_deref(),
                    input_source,
                    &frequencies,
                    temperature_kelvin.get(),
                    abort,
                )
                .map(ExecutedQualificationAnalysis::Noise)
        }
        QualificationAnalysis::Transient {
            stop_time,
            max_step,
        } => engine
            .run_tran_with_abort(&netlist, stop_time.get(), max_step.get(), abort)
            .map(ExecutedQualificationAnalysis::Transient),
    };
    let executed = match executed {
        Ok(value) => value,
        Err(error) if error.descriptor().code == rspice_core::SimulationErrorCode::Aborted => {
            return Err(QualificationExecutionError::Cancelled);
        }
        Err(error) => {
            let code = error.descriptor().code.to_string();
            return failed_platform_outcome(
                platform,
                QualificationFailureStage::Simulation,
                code,
                error.to_string(),
            );
        }
    };
    if abort.is_aborted() {
        return Err(QualificationExecutionError::Cancelled);
    }

    let mut references = Vec::with_capacity(vector.outputs.len());
    for output in &vector.outputs {
        let reference = find_ci(&vector.references, &output.quantity, |value| {
            &value.quantity
        })
        .expect("vector output/reference coverage validated before execution");
        let observed = match measure_qualification_output(output, &executed) {
            Ok(value) if value.is_finite() => value,
            Ok(_) => {
                return failed_platform_outcome(
                    platform,
                    QualificationFailureStage::Measurement,
                    "non-finite-output",
                    format!("output {:?} produced a non-finite value", output.quantity),
                );
            }
            Err(message) => {
                return failed_platform_outcome(
                    platform,
                    QualificationFailureStage::Measurement,
                    "output-unavailable",
                    message,
                );
            }
        };
        let evidence = match ReferenceErrorEvidence::try_new(
            output.quantity.clone(),
            reference.expected.get(),
            observed,
            reference.absolute_tolerance.get(),
            reference.relative_tolerance.get(),
        ) {
            Ok(value) => value,
            Err(error) => {
                return failed_platform_outcome(
                    platform,
                    QualificationFailureStage::Measurement,
                    "error-computation-failed",
                    error.to_string(),
                );
            }
        };
        references.push(evidence);
    }
    PlatformQualificationOutcome::try_new(platform, references).map_err(Into::into)
}

fn failed_platform_outcome(
    platform: QualificationPlatform,
    stage: QualificationFailureStage,
    code: impl Into<String>,
    message: impl Into<String>,
) -> Result<PlatformQualificationOutcome, QualificationExecutionError> {
    let failure = QualificationExecutionFailure::try_new(stage, code, message)?;
    PlatformQualificationOutcome::try_failed(platform, failure).map_err(Into::into)
}

fn measure_qualification_output(
    output: &QualificationOutputDefinition,
    executed: &ExecutedQualificationAnalysis,
) -> Result<f64, String> {
    let (sweep_value, result) = match (executed, output.sample) {
        (
            ExecutedQualificationAnalysis::OperatingPoint(result),
            QualificationSample::OperatingPoint,
        ) => (None, result),
        (ExecutedQualificationAnalysis::DcSweep(points), QualificationSample::FirstSweepPoint) => {
            let (sweep, result) = points
                .first()
                .ok_or_else(|| "DC sweep completed without producing a result point".to_owned())?;
            (Some(*sweep), result)
        }
        (ExecutedQualificationAnalysis::DcSweep(points), QualificationSample::LastSweepPoint) => {
            let (sweep, result) = points
                .last()
                .ok_or_else(|| "DC sweep completed without producing a result point".to_owned())?;
            (Some(*sweep), result)
        }
        (
            ExecutedQualificationAnalysis::DcSweep(points),
            QualificationSample::SweepPoint { index },
        ) => {
            let (sweep, result) = points.get(index).ok_or_else(|| {
                format!(
                    "DC sweep output requests point {index}, but only {} points were produced",
                    points.len()
                )
            })?;
            (Some(*sweep), result)
        }
        (ExecutedQualificationAnalysis::AcSweep(points), sample) => {
            let index = selected_frequency_index(sample, points.len(), "AC sweep")?;
            let point = points.get(index).ok_or_else(|| {
                format!(
                    "AC output requests frequency point {index}, but only {} points were produced",
                    points.len()
                )
            })?;
            return measure_ac_output(&output.probe, point);
        }
        (ExecutedQualificationAnalysis::Noise(points), sample) => {
            let index = selected_frequency_index(sample, points.len(), "noise sweep")?;
            let point = points.get(index).ok_or_else(|| {
                format!(
                    "noise output requests frequency point {index}, but only {} points were produced",
                    points.len()
                )
            })?;
            return measure_noise_output(&output.probe, point);
        }
        (ExecutedQualificationAnalysis::Transient(result), sample) => {
            let index = selected_time_index(sample, result.time.len())?;
            return measure_transient_output(&output.probe, result, index);
        }
        _ => return Err("output sample is incompatible with the executed analysis".to_owned()),
    };

    match &output.probe {
        QualificationProbe::NodeVoltage { node } => result
            .try_voltage_named(node)
            .ok_or_else(|| format!("node voltage {:?} is unavailable", node)),
        QualificationProbe::BranchCurrent { branch } => result
            .branch_current_named(branch)
            .ok_or_else(|| format!("branch current {:?} is unavailable", branch)),
        QualificationProbe::DcObservable { expression } => result
            .try_dc_observable_named(expression)
            .ok_or_else(|| format!("DC observable {:?} is unavailable", expression)),
        QualificationProbe::SweepValue => sweep_value
            .ok_or_else(|| "sweep value is unavailable for an operating-point result".to_owned()),
        _ => Err("output probe is incompatible with a DC result".to_owned()),
    }
}

fn selected_frequency_index(
    sample: QualificationSample,
    point_count: usize,
    analysis: &str,
) -> Result<usize, String> {
    if point_count == 0 {
        return Err(format!(
            "{analysis} completed without producing a result point"
        ));
    }
    match sample {
        QualificationSample::FirstFrequencyPoint => Ok(0),
        QualificationSample::LastFrequencyPoint => Ok(point_count - 1),
        QualificationSample::FrequencyPoint { index } if index < point_count => Ok(index),
        QualificationSample::FrequencyPoint { index } => Err(format!(
            "{analysis} output requests point {index}, but only {point_count} points were produced"
        )),
        _ => Err(format!(
            "output sample is incompatible with the executed {analysis}"
        )),
    }
}

fn selected_time_index(sample: QualificationSample, point_count: usize) -> Result<usize, String> {
    if point_count == 0 {
        return Err("transient analysis completed without producing a result point".to_owned());
    }
    match sample {
        QualificationSample::FirstTimePoint => Ok(0),
        QualificationSample::LastTimePoint => Ok(point_count - 1),
        QualificationSample::TimePoint { index } if index < point_count => Ok(index),
        QualificationSample::TimePoint { index } => Err(format!(
            "transient output requests point {index}, but only {point_count} points were produced"
        )),
        _ => Err("output sample is incompatible with the executed transient analysis".to_owned()),
    }
}

fn measure_ac_output(
    probe: &QualificationProbe,
    point: &rspice_core::analysis::AcResult,
) -> Result<f64, String> {
    let node_voltage = |node: &str| -> Result<num_complex::Complex64, String> {
        if is_ground_name(node) {
            return Ok(num_complex::Complex64::new(0.0, 0.0));
        }
        let index = point
            .node_names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(node))
            .ok_or_else(|| format!("AC node voltage {node:?} is unavailable"))?;
        point
            .voltages
            .get(index)
            .copied()
            .ok_or_else(|| format!("AC node voltage {node:?} is unavailable"))
    };
    let branch_current = |branch: &str| -> Result<num_complex::Complex64, String> {
        let index = point
            .branch_names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(branch))
            .ok_or_else(|| format!("AC branch current {branch:?} is unavailable"))?;
        point
            .currents
            .get(index)
            .copied()
            .ok_or_else(|| format!("AC branch current {branch:?} is unavailable"))
    };

    match probe {
        QualificationProbe::AcNodeVoltageMagnitude { node } => Ok(node_voltage(node)?.norm()),
        QualificationProbe::AcNodeVoltagePhaseDegrees { node } => {
            Ok(node_voltage(node)?.arg().to_degrees())
        }
        QualificationProbe::AcNodeVoltageReal { node } => Ok(node_voltage(node)?.re),
        QualificationProbe::AcNodeVoltageImaginary { node } => Ok(node_voltage(node)?.im),
        QualificationProbe::AcBranchCurrentMagnitude { branch } => {
            Ok(branch_current(branch)?.norm())
        }
        QualificationProbe::AcBranchCurrentPhaseDegrees { branch } => {
            Ok(branch_current(branch)?.arg().to_degrees())
        }
        QualificationProbe::AcBranchCurrentReal { branch } => Ok(branch_current(branch)?.re),
        QualificationProbe::AcBranchCurrentImaginary { branch } => Ok(branch_current(branch)?.im),
        QualificationProbe::AcEffectiveCapacitance {
            branch,
            excitation_magnitude,
        } => Ok(branch_current(branch)?.im
            / (std::f64::consts::TAU * point.frequency * excitation_magnitude.get())),
        QualificationProbe::FrequencyValue => Ok(point.frequency),
        _ => Err("output probe is incompatible with an AC result".to_owned()),
    }
}

fn measure_noise_output(
    probe: &QualificationProbe,
    point: &rspice_core::analysis::NoiseResult,
) -> Result<f64, String> {
    match probe {
        QualificationProbe::FrequencyValue => Ok(point.frequency),
        QualificationProbe::NoiseOutputDensity => Ok(point.output_noise_density),
        QualificationProbe::NoiseInputReferredDensity => Ok(point.input_referred_density),
        QualificationProbe::NoiseOutputAmplitude => point
            .output_noise_density
            .is_sign_positive()
            .then(|| point.output_noise_density.sqrt())
            .ok_or_else(|| "output-noise density is negative".to_owned()),
        QualificationProbe::NoiseInputReferredAmplitude => point
            .input_referred_density
            .is_sign_positive()
            .then(|| point.input_referred_density.sqrt())
            .ok_or_else(|| "input-referred-noise density is negative".to_owned()),
        _ => Err("output probe is incompatible with a noise result".to_owned()),
    }
}

fn measure_transient_output(
    probe: &QualificationProbe,
    result: &rspice_core::engine::TransientResult,
    index: usize,
) -> Result<f64, String> {
    match probe {
        QualificationProbe::TransientNodeVoltage { node } => {
            result.try_voltage_at_named(node, index).ok_or_else(|| {
                format!("transient node voltage {node:?} is unavailable at point {index}")
            })
        }
        QualificationProbe::TransientBranchCurrent { branch } => result
            .try_branch_current_waveform_named(branch)
            .and_then(|waveform| waveform.get(index))
            .copied()
            .ok_or_else(|| {
                format!("transient branch current {branch:?} is unavailable at point {index}")
            }),
        QualificationProbe::TimeValue => result
            .time
            .get(index)
            .copied()
            .ok_or_else(|| format!("transient time is unavailable at point {index}")),
        _ => Err("output probe is incompatible with a transient result".to_owned()),
    }
}

fn is_ground_name(name: &str) -> bool {
    matches!(
        name.trim().to_ascii_lowercase().as_str(),
        "0" | "gnd" | "gnd!"
    )
}

#[cfg(target_arch = "wasm32")]
const fn current_qualification_platform() -> QualificationPlatform {
    QualificationPlatform::WebAssembly
}

#[cfg(not(target_arch = "wasm32"))]
const fn current_qualification_platform() -> QualificationPlatform {
    QualificationPlatform::Desktop
}
