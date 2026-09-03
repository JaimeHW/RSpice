//! Assembly of the one JSON verdict a request produces.
//!
//! Every way a request can end -- succeeded with a result manifest, refused
//! for an unsupported family, stopped on a deadline, or bounded out by a
//! resource limit -- becomes an [`Execution`] here, and every failure carries
//! the code and the detail an operator needs to act on. Nothing in this module
//! runs an analysis; it only says what happened.

// This module was split out of `execute.rs` and still works against
// the executor's own wire types, failures and imports, so it takes the
// parent's imports rather than restating them.
use super::*;

/// Result-manifest format tag. Advanced with the protocol version whenever the
/// published result contract changes shape.
pub const RESULT_MANIFEST_FORMAT: &str = "rspice-result-v3";

/// One executed request: the wire response plus the results files it
/// declared, staged in memory until `main` writes and emits them together.
pub struct Execution {
    pub response: EngineResponse,
    pub artifacts: Vec<PendingArtifact>,
}

impl Execution {
    pub(super) fn failed(failure_code: &str, failure_detail: &str) -> Self {
        Self {
            response: EngineResponse::failed(failure_code, failure_detail),
            artifacts: Vec::new(),
        }
    }
}

pub(super) fn deck_plan_failure(error: &rspice_core::execution::DeckPlanError) -> Execution {
    if matches!(error, rspice_core::execution::DeckPlanError::Aborted) {
        failure_execution(&SimulationError::Aborted)
    } else {
        Execution::failed(
            "analysis.axis_materialization",
            &format!("The deck run plan is invalid: {error}"),
        )
    }
}

pub(super) fn materialization_failure(error: &MaterializedRunError) -> Execution {
    match error {
        MaterializedRunError::Aborted => failure_execution(&SimulationError::Aborted),
        MaterializedRunError::Simulation(error) => failure_execution(error),
        other => Execution::failed(
            "analysis.axis_materialization",
            &format!("The canonical deck materializer failed: {other}"),
        ),
    }
}

pub(super) fn resource_failure(failure: DirectiveFailure) -> Execution {
    match failure {
        DirectiveFailure::ResultArtifactLimit => Execution::failed(
            "resource.result_artifact_limit",
            "The planned coordinate/analysis product would produce more result artifacts than the protocol permits.",
        ),
        DirectiveFailure::ResultArtifactBytes => Execution::failed(
            "resource.result_artifact_bytes",
            "A result artifact exceeds the protocol byte limit.",
        ),
        DirectiveFailure::ResultSetBytes => Execution::failed(
            "resource.result_set_bytes",
            "The aggregate encoded result set exceeds the adapter memory budget.",
        ),
        _ => Execution::failed(
            "results.resource_accounting",
            "The adapter encountered an inconsistent result-resource accounting outcome.",
        ),
    }
}

/// Map one directive refusal onto the bounded wire failure vocabulary.
pub(super) fn directive_failure(failure: DirectiveFailure) -> Execution {
    match failure {
        DirectiveFailure::Engine(error) => failure_execution(&error),
        DirectiveFailure::NonFinite => Execution::failed(
            "results.nonfinite",
            "The analysis completed with non-finite values; the operating region of this circuit \
             is outside the solver's validated range.",
        ),
        DirectiveFailure::SeriesBudget => Execution::failed(
            "resource.series_limit",
            "The analysis produced more samples than one run may retain; shorten the window or \
             relax the timestep.",
        ),
        DirectiveFailure::ResultDocument(detail) => {
            Execution::failed("results.schema_mismatch", &detail)
        }
        DirectiveFailure::FrequencyGrid(error) => frequency_grid_failure(error),
        DirectiveFailure::InvalidAnalysis(detail) => {
            Execution::failed("analysis.invalid_configuration", &detail)
        }
        DirectiveFailure::UnsupportedForm(detail) => {
            Execution::failed("analysis.unsupported_form", &detail)
        }
        resource @ (DirectiveFailure::ResultArtifactLimit
        | DirectiveFailure::ResultArtifactBytes
        | DirectiveFailure::ResultSetBytes) => resource_failure(resource),
    }
}

pub(super) fn map_fft_document_error(error: FftResultDocumentError) -> DirectiveFailure {
    match error {
        FftResultDocumentError::Aborted => DirectiveFailure::Engine(SimulationError::Aborted),
        FftResultDocumentError::ArtifactTooLarge { .. } => DirectiveFailure::ResultArtifactBytes,
        other => DirectiveFailure::ResultDocument(other.to_string()),
    }
}

fn frequency_grid_failure(error: rspice_core::analysis::FrequencyGridError) -> Execution {
    use rspice_core::analysis::FrequencyGridError;

    match error {
        FrequencyGridError::Aborted => failure_execution(&SimulationError::Aborted),
        FrequencyGridError::Allocation { .. } | FrequencyGridError::PointCountOverflow => {
            Execution::failed(
                "resource.frequency_grid",
                &format!("The frequency grid exceeds adapter resources: {error}"),
            )
        }
        _ => Execution::failed(
            "analysis.invalid_frequency_grid",
            &format!("The authored frequency grid is invalid: {error}"),
        ),
    }
}

pub(super) fn measurement_document(measurement: &Measurement) -> MeasurementDocument {
    MeasurementDocument {
        name: measurement.name.clone(),
        unit: measurement.unit.clone(),
        value_decimal: measurement.value_decimal.clone(),
        sample_count: measurement.sample_count,
        series_sha256: measurement.series_sha256.clone(),
    }
}

pub(super) fn run_manifest(
    coordinate: &RunCoordinate,
    analyses: Vec<AnalysisExecution>,
) -> Result<CoordinateExecution, String> {
    let mut assignments = Vec::new();
    assignments
        .try_reserve_exact(coordinate.assignments().len())
        .map_err(|_| "Unable to allocate bounded coordinate assignments.".to_owned())?;
    for assignment in coordinate.assignments() {
        assignments.push(axis_assignment_manifest(assignment)?);
    }
    Ok(CoordinateExecution {
        ordinal: coordinate.ordinal() + 1,
        coordinate_id: coordinate.stable_id().to_string(),
        coordinate_namespace: coordinate.stable_tag(),
        assignments,
        analyses,
    })
}

fn axis_assignment_manifest(assignment: &AxisAssignment) -> Result<AxisAssignmentDocument, String> {
    let kind = match assignment.kind() {
        AxisKind::Step => AxisAssignmentKind::Step,
        AxisKind::Temperature => AxisAssignmentKind::Temperature,
        other => {
            return Err(format!(
                "Coordinate contains unsupported {other:?} axis assignment."
            ));
        }
    };
    let target = assignment
        .step_target()
        .map(step_target_manifest)
        .transpose()?;
    let value = match assignment.value() {
        RunAxisValue::Numeric(value) => canonical_decimal(*value).ok_or_else(|| {
            format!(
                "Coordinate axis {:?} contains a non-finite value.",
                assignment.name()
            )
        })?,
        _ => {
            return Err(format!(
                "Coordinate axis {:?} contains an unsupported value shape.",
                assignment.name()
            ));
        }
    };
    Ok(AxisAssignmentDocument {
        kind,
        name: assignment.name().to_owned(),
        value_index: assignment.value_index(),
        value_decimal: value,
        target,
    })
}

fn step_target_manifest(target: &StepAxisTarget) -> Result<StepTargetDocument, String> {
    Ok(match target {
        StepAxisTarget::Parameter { name } => StepTargetDocument::Parameter { name: name.clone() },
        StepAxisTarget::Device { name, parameter } => StepTargetDocument::Device {
            name: name.clone(),
            parameter: parameter.clone(),
        },
        StepAxisTarget::Model { name, parameter } => StepTargetDocument::Model {
            name: name.clone(),
            parameter: parameter.clone(),
        },
        StepAxisTarget::Temperature => StepTargetDocument::Temperature,
        _ => return Err("Coordinate contains an unsupported STEP target.".to_owned()),
    })
}

pub(super) fn succeeded(
    request_kind: &str,
    engine_build: &str,
    measurements: Vec<Measurement>,
    artifacts: Vec<PendingArtifact>,
    directive_count: usize,
    axis_execution: Option<AxisExecutionDocument>,
    abort: &dyn AbortSignal,
) -> Execution {
    let mut manifest = serde_json::json!({
        "format": RESULT_MANIFEST_FORMAT,
        "analysis_kind": request_kind,
        "engine": {"name": "rspice", "build": engine_build},
        "typed_result_schema": {
            "name": rspice_core::execution::ANALYSIS_RESULT_DOCUMENT_SCHEMA,
            "version": rspice_core::execution::ANALYSIS_RESULT_DOCUMENT_VERSION,
            "content_type": result_document_content_type(),
        },
        "directives": directive_count,
        "measurements": measurements
            .iter()
            .map(Measurement::to_manifest_value)
            .collect::<Vec<_>>(),
    });
    if let Some(axis_execution) = axis_execution {
        let execution_value = match axis_execution
            .to_value_with_abort(abort, MAX_ENGINE_RESULT_MANIFEST_BYTES as u64)
        {
            Ok(value) => value,
            Err(crate::axis_execution_document::AxisExecutionDocumentError::Aborted) => {
                return failure_execution(&SimulationError::Aborted);
            }
            Err(crate::axis_execution_document::AxisExecutionDocumentError::DocumentTooLarge {
                ..
            }) => {
                return Execution::failed(
                    "results.manifest_too_large",
                    "The axis execution manifest exceeds its protocol byte limit.",
                );
            }
            Err(error) => {
                return Execution::failed("results.schema_mismatch", &error.to_string());
            }
        };
        manifest["axis_execution"] = execution_value;
    }
    if artifacts
        .iter()
        .any(|artifact| artifact.content_type == FFT_RESULT_DOCUMENT_CONTENT_TYPE)
    {
        manifest["typed_fft_result_schema"] = serde_json::json!({
            "name": FFT_RESULT_DOCUMENT_SCHEMA,
            "version": FFT_RESULT_DOCUMENT_VERSION,
            "content_type": FFT_RESULT_DOCUMENT_CONTENT_TYPE,
        });
    }
    let descriptors = artifacts
        .iter()
        .map(|artifact| EngineResultArtifactDescriptor {
            path: format!("results/{}", artifact.file_name),
            content_type: artifact.content_type.clone(),
        })
        .collect();
    Execution {
        response: EngineResponse::Succeeded {
            result_manifest: manifest,
            result_artifacts: descriptors,
        },
        artifacts,
    }
}

pub(super) enum WireSerializationFailure {
    Aborted,
    TooLarge,
    Invalid,
}

fn serialize_wire_bounded<T: Serialize + ?Sized>(
    value: &T,
    abort: &dyn AbortSignal,
    byte_limit: u64,
) -> Result<Vec<u8>, WireSerializationFailure> {
    if abort.is_aborted() {
        return Err(WireSerializationFailure::Aborted);
    }
    let mut writer = BoundedAbortWriter::new(abort, byte_limit);
    if serde_json::to_writer(&mut writer, value).is_err() {
        return Err(match writer.failure() {
            Some(BoundedWriteFailure::Aborted) => WireSerializationFailure::Aborted,
            Some(BoundedWriteFailure::ByteLimitExceeded { .. }) => {
                WireSerializationFailure::TooLarge
            }
            Some(BoundedWriteFailure::AllocationFailed) | None => WireSerializationFailure::Invalid,
        });
    }
    if abort.is_aborted() {
        return Err(WireSerializationFailure::Aborted);
    }
    Ok(writer.into_bytes())
}

pub(super) fn finalize_execution(execution: Execution, abort: &dyn AbortSignal) -> Execution {
    if abort.is_aborted() {
        return failure_execution(&SimulationError::Aborted);
    }
    if let EngineResponse::Succeeded {
        result_manifest, ..
    } = &execution.response
    {
        match serialize_wire_bounded(
            result_manifest,
            abort,
            MAX_ENGINE_RESULT_MANIFEST_BYTES as u64,
        ) {
            Ok(_) => {}
            Err(WireSerializationFailure::Aborted) => {
                return failure_execution(&SimulationError::Aborted);
            }
            Err(WireSerializationFailure::TooLarge) => {
                return Execution::failed(
                    "results.manifest_too_large",
                    "The result manifest exceeds its protocol byte limit; reduce the number of saved signals in the deck.",
                );
            }
            Err(WireSerializationFailure::Invalid) => {
                return Execution::failed(
                    "results.manifest_invalid",
                    "The result manifest could not be serialized.",
                );
            }
        }
    }
    match serialize_wire_bounded(&execution.response, abort, MAX_ENGINE_RESPONSE_BYTES as u64) {
        Ok(_) => execution,
        Err(WireSerializationFailure::Aborted) => failure_execution(&SimulationError::Aborted),
        Err(WireSerializationFailure::TooLarge) => Execution::failed(
            "results.manifest_too_large",
            "The result manifest exceeds the response budget; reduce the number of saved signals in the deck.",
        ),
        Err(WireSerializationFailure::Invalid) => Execution::failed(
            "results.manifest_invalid",
            "The result manifest could not be serialized.",
        ),
    }
}

/// Maps every engine error onto the bounded failure vocabulary through the
/// engine's own stable descriptor, so new engine variants cannot silently
/// widen the wire surface.
///
/// `SimulationError::Aborted` says only that something asked the engine to
/// stop, so it lands on the cancellation code here; [`label_stop_cause`]
/// promotes it to the time-limit code when the executor's own deadline is
/// what fired. Reporting a caller-requested cancellation as a time limit —
/// which is what this function used to do unconditionally — tells the
/// operator their deck is too slow when in fact they stopped it.
pub(super) fn failure_execution(error: &SimulationError) -> Execution {
    let code = match error.descriptor().code {
        rspice_core::engine::SimulationErrorCode::Aborted => CANCELLED_FAILURE_CODE.to_owned(),
        rspice_core::engine::SimulationErrorCode::TimeLimitExceeded => {
            TIME_LIMIT_FAILURE_CODE.to_owned()
        }
        stable => format!("engine.{}", stable.as_str()),
    };
    Execution::failed(&code, &error.to_string())
}
