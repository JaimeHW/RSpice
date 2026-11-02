//! One model-qualification vector executed on this image's solver.
//!
//! The model editor's qualification session runs each vector here. The
//! worker wire carries a vector and its outcome as JSON, which the workspace's
//! `float_roundtrip` parser returns bit for bit.

use rspice_model_library::qualification::*;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Why a vector produced no platform outcome. Parse, solver and measurement
/// failures are failed outcomes, not errors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualificationVectorError {
    Cancelled,
    InvalidContract(QualificationValidationError),
}

/// A vector's platform outcome, or why it has none.
pub type QualificationVectorResult = Result<PlatformQualificationOutcome, QualificationVectorError>;

impl From<QualificationValidationError> for QualificationVectorError {
    fn from(error: QualificationValidationError) -> Self {
        Self::InvalidContract(error)
    }
}

enum ExecutedQualificationAnalysis {
    OperatingPoint(rspice_core::SimulationResult),
    DcSweep(Vec<(f64, rspice_core::SimulationResult)>),
    AcSweep(Vec<rspice_core::analysis::AcResult>),
    Noise(Vec<rspice_core::analysis::NoiseResult>),
    Transient(rspice_core::engine::TransientResult),
}

/// Execute one vector and measure its outputs against the declared references.
///
/// Parse, solver and measurement failures become a failed platform outcome;
/// only cancellation and an invalid contract are errors.
pub fn execute_qualification_vector(
    vector: &QualificationVector,
    platform: QualificationPlatform,
    abort: &dyn rspice_core::AbortSignal,
) -> Result<PlatformQualificationOutcome, QualificationVectorError> {
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
            return Err(QualificationVectorError::Cancelled);
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
            return Err(QualificationVectorError::Cancelled);
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
        return Err(QualificationVectorError::Cancelled);
    }

    // A probe names node 0 only where this deck's dialect folds its name onto
    // it; `GND!` is an ordinary node to ngspice and is measured like one.
    let ground = netlist.ground_policy();
    let mut references = Vec::with_capacity(vector.outputs.len());
    for output in &vector.outputs {
        let reference = vector
            .reference(&output.quantity)
            .expect("vector output/reference coverage validated before execution");
        let observed = match measure_qualification_output(output, &executed, ground) {
            Ok(value) if value.is_finite() => value,
            Ok(_) => {
                return failed_platform_outcome(
                    platform,
                    QualificationFailureStage::Measurement,
                    "non-finite-output",
                    format!("output '{}' produced a non-finite value", output.quantity),
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
) -> Result<PlatformQualificationOutcome, QualificationVectorError> {
    let failure = QualificationExecutionFailure::try_new(stage, code, message)?;
    PlatformQualificationOutcome::try_failed(platform, failure).map_err(Into::into)
}

fn measure_qualification_output(
    output: &QualificationOutputDefinition,
    executed: &ExecutedQualificationAnalysis,
    ground: rspice_core::netlist::GroundPolicy,
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
            return measure_ac_output(&output.probe, point, ground);
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
            .ok_or_else(|| format!("node voltage '{}' is unavailable", node)),
        QualificationProbe::BranchCurrent { branch } => result
            .branch_current_named(branch)
            .ok_or_else(|| format!("branch current '{}' is unavailable", branch)),
        QualificationProbe::DcObservable { expression } => result
            .try_dc_observable_named(expression)
            .ok_or_else(|| format!("DC observable '{}' is unavailable", expression)),
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
    ground: rspice_core::netlist::GroundPolicy,
) -> Result<f64, String> {
    let node_voltage = |node: &str| -> Result<num_complex::Complex64, String> {
        if ground.is_ground(node) {
            return Ok(num_complex::Complex64::new(0.0, 0.0));
        }
        let index = point
            .node_names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(node))
            .ok_or_else(|| format!("AC node voltage '{node}' is unavailable"))?;
        point
            .voltages
            .get(index)
            .copied()
            .ok_or_else(|| format!("AC node voltage '{node}' is unavailable"))
    };
    let branch_current = |branch: &str| -> Result<num_complex::Complex64, String> {
        let index = point
            .branch_names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(branch))
            .ok_or_else(|| format!("AC branch current '{branch}' is unavailable"))?;
        point
            .currents
            .get(index)
            .copied()
            .ok_or_else(|| format!("AC branch current '{branch}' is unavailable"))
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
                format!("transient node voltage '{node}' is unavailable at point {index}")
            })
        }
        QualificationProbe::TransientBranchCurrent { branch } => result
            .try_branch_current_waveform_named(branch)
            .and_then(|waveform| waveform.get(index))
            .copied()
            .ok_or_else(|| {
                format!("transient branch current '{branch}' is unavailable at point {index}")
            }),
        QualificationProbe::TimeValue => result
            .time
            .get(index)
            .copied()
            .ok_or_else(|| format!("transient time is unavailable at point {index}")),
        _ => Err("output probe is incompatible with a transient result".to_owned()),
    }
}

/// The browser worker's wire revision for qualification requests.
pub const QUALIFICATION_WORKER_PROTOCOL_VERSION: u32 = 1;

/// One vector posted to the browser simulation worker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationWorkerRequest {
    pub protocol_version: u32,
    pub platform: QualificationPlatform,
    pub vector: QualificationVector,
}

/// The worker's answer: the vector's outcome exactly as an in-process run
/// returns it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationWorkerResponse {
    pub protocol_version: u32,
    pub result: QualificationVectorResult,
}

/// Encode one vector for the browser simulation worker.
pub fn encode_qualification_worker_request(
    vector: &QualificationVector,
    platform: QualificationPlatform,
) -> Result<String, String> {
    serde_json::to_string(&QualificationWorkerRequest {
        protocol_version: QUALIFICATION_WORKER_PROTOCOL_VERSION,
        platform,
        vector: vector.clone(),
    })
    .map_err(|error| format!("the qualification vector could not be encoded: {error}"))
}

/// Execute one encoded request: what the browser simulation worker does with a
/// qualification message.
pub fn run_qualification_worker_request(request: &str) -> Result<String, String> {
    let request: QualificationWorkerRequest = serde_json::from_str(request)
        .map_err(|error| format!("the qualification request is unreadable: {error}"))?;
    if request.protocol_version != QUALIFICATION_WORKER_PROTOCOL_VERSION {
        return Err(format!(
            "unsupported qualification request protocol {}",
            request.protocol_version
        ));
    }
    let result =
        execute_qualification_vector(&request.vector, request.platform, &rspice_core::NoAbort);
    serde_json::to_string(&QualificationWorkerResponse {
        protocol_version: QUALIFICATION_WORKER_PROTOCOL_VERSION,
        result,
    })
    .map_err(|error| format!("the qualification outcome could not be encoded: {error}"))
}

/// Decode the worker's answer to one request.
pub fn decode_qualification_worker_response(
    response: &str,
) -> Result<QualificationVectorResult, String> {
    let response: QualificationWorkerResponse = serde_json::from_str(response)
        .map_err(|error| format!("the qualification outcome is unreadable: {error}"))?;
    if response.protocol_version != QUALIFICATION_WORKER_PROTOCOL_VERSION {
        return Err(format!(
            "unsupported qualification outcome protocol {}",
            response.protocol_version
        ));
    }
    Ok(response.result)
}
