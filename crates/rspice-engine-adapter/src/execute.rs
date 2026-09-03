//! Deck execution: request-kind resolution, canonical planning, and engine
//! invocation.
//!
//! What this file orchestrates lives beside it: [`deadline`] owns the
//! wall-clock budget and the stop labels, [`artifacts`] owns the staging and
//! publication of everything a request produces, and [`response`] owns the one
//! JSON verdict a request ends with.
//!
//! # Result contract
//!
//! Every analysis this executor runs publishes at least one
//! [`rspice_core::execution::AnalysisResultDocument`]. The document names the
//! canonical analysis identity the planner assigned, the shared-deck
//! coordinate it was produced at, the elaborated topology fingerprint, and the
//! output and checkpoint namespaces. The adapter defines no result schema of
//! its own for those families: the artifact content type is derived from the
//! core document's schema identifier and version.
//!
//! A card whose result is more than one document publishes them all, staged in
//! the same artifact transaction: `.SP DONOISE` publishes the port-noise sweep
//! beside the scattering one, and a transient publishes one spectrum per
//! authored `.FFT` card and one Fourier document per authored `.FOUR` operand
//! beside its own. A child artifact's file name is the parent's stem plus its
//! own namespace component — its analysis tag when it has an identity of its
//! own (`tran-001.fft-001.result.json`, `tran-001.four-001.result.json`), and
//! its result family when it shares the parent's identity, as port noise does
//! (`sp-001.port-noise.result.json`).
//!
//! One adapter-owned document remains, and it is not a result projection:
//! [`crate::axis_execution_document`] is the STEP/TEMP orchestration record
//! and references the typed documents by path, schema, and family.
//!
//! # Protocol compatibility
//!
//! Protocol 4 replaced protocol 3 with this contract. Relative to protocol 3 a
//! consumer must expect:
//!
//! * every analysis family the planner recognizes, not only OP/DC/AC/TRAN/
//!   NOISE, with a family this build cannot project refused by name rather
//!   than silently skipped;
//! * `analysis.kind` `mixed_signal` removed — a mixed-signal deck is a
//!   transient and is requested as `transient`;
//! * one typed JSON artifact per analysis instead of a CSV plus an
//!   `rspice-analog-result-v1` document, with the `rspice-analysis-result`
//!   content type;
//! * result artifact names built from the canonical namespace components
//!   (`op-001.result.json`, `run-<id>__op-001.result.json`);
//! * `result_manifest.format` `rspice-result-v3`, and measurements projected
//!   generically from the typed document rather than from a per-family column
//!   list.
//!
//! A protocol-3 consumer is not compatible with this build; the launch
//! contract refuses a protocol-3 environment before a request is read.

use std::collections::{BTreeMap, HashSet};
use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicU8, Ordering};
use std::time::{Duration, Instant};

use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::{SimulationConfig, SpiceDialect};
use rspice_core::execution::bounded_io::{BoundedAbortWriter, BoundedWriteFailure};
use rspice_core::execution::{
    AnalysisKind as PlannedAnalysisKind, AxisAssignment, AxisKind, DeckPlan, MaterializedAnalysis,
    MaterializedRunError, ResultCoordinate, ResultNamespaces, RunAxisValue, RunCoordinate,
    StepAxisTarget, TopologyFingerprint,
};
use rspice_core::{Engine, Netlist, SimulationError};
use rspice_output::AtomicArtifactSet;
#[cfg(test)]
use rspice_output::write_atomic;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::axis_execution_document::{
    AnalysisExecution, AxisAnalysisKind, AxisAssignmentDocument, AxisAssignmentKind,
    AxisExecutionDocument, CoordinateExecution, MeasurementDocument, OutputNamespaceDocument,
    ResultDocumentReference, StepTargetDocument,
};
use crate::document::CircuitContent;
use crate::failure::DirectiveFailure;
use crate::family::{
    REQUEST_KINDS, matches_request, planned_kind_for_request, refusal_for_request,
    request_kind_name, run_directive, unmapped_deck_card,
};
use crate::measure::{Measurement, canonical_decimal, finalize_measurements};
use crate::result_artifact::{
    PendingArtifact, ResultSchemaSignature, encode_result_artifact, measurements_from_document,
    result_document_content_type,
};
use crate::wire::{
    EngineResponse, EngineResultArtifactDescriptor, MAX_ENGINE_ARTIFACT_BYTES,
    MAX_ENGINE_RESPONSE_BYTES, MAX_ENGINE_RESULT_ARTIFACTS, MAX_ENGINE_RESULT_MANIFEST_BYTES,
    MAX_ENGINE_RETAINED_RESULT_BYTES, valid_result_path,
};

mod artifacts;
mod deadline;
mod response;

pub use deadline::{
    CANCELLED_FAILURE_CODE, DEFAULT_SOLVE_BUDGET, RunAbort, SOLVE_BUDGET_SECONDS_ENV, StopCause,
    TIME_LIMIT_FAILURE_CODE, solve_budget_from_env,
};
pub use response::{Execution, RESULT_MANIFEST_FORMAT};

pub use artifacts::write_artifacts;
use artifacts::{
    artifact_reference, artifacts_per_directive, execute_analysis,
    preflight_planned_artifact_count, validate_artifact_budget, validate_pending_artifact_budget,
};
use deadline::label_stop_cause;
use response::{
    deck_plan_failure, directive_failure, failure_execution, finalize_execution,
    materialization_failure, measurement_document, resource_failure, run_manifest, succeeded,
};

/// The engine-owned analysis request. Version 1 selects which deck-carried
/// directive class runs; directive parameters stay in the deck so the digest
/// over the revision covers the complete simulation definition.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AnalysisRequestV1 {
    kind: String,
}

/// Executes one validated request with no caller cancellation and the default
/// solve budget.
///
/// Retained for embedding that has no cancellation source of its own; the
/// executor binary always calls [`execute_with_abort`].
pub fn execute(analysis: &Value, content: &CircuitContent, engine_build: &str) -> Execution {
    execute_with_abort(
        analysis,
        content,
        engine_build,
        &rspice_core::abort_signal::NoAbort,
        DEFAULT_SOLVE_BUDGET,
    )
}

/// Executes one validated request against the interpreted circuit content.
/// Customer-content problems become canonical failures here; this function
/// touches no filesystem, so nothing in it can raise a process fault.
///
/// `cancel` is the caller's stop request — a signal handler, a supervising
/// thread — and `solve_budget` is this executor's own wall-clock ceiling. The
/// two combine into one abort source, and the failure the caller sees says
/// which of them fired.
pub fn execute_with_abort(
    analysis: &Value,
    content: &CircuitContent,
    engine_build: &str,
    cancel: &dyn AbortSignal,
    solve_budget: Duration,
) -> Execution {
    let abort = RunAbort::new(cancel, solve_budget);
    let execution = execute_bounded(analysis, content, engine_build, &abort);
    let execution = finalize_execution(execution, &abort);
    label_stop_cause(execution, abort.stop_cause())
}

fn execute_bounded(
    analysis: &Value,
    content: &CircuitContent,
    engine_build: &str,
    abort: &RunAbort<'_>,
) -> Execution {
    if abort.is_aborted() {
        return failure_execution(&SimulationError::Aborted);
    }
    let request: AnalysisRequestV1 = match serde_json::from_value(analysis.clone()) {
        Ok(request) => request,
        Err(_) => {
            return Execution::failed(
                "analysis.invalid",
                "The analysis request is not a valid version-1 object; expected {\"kind\": ...}.",
            );
        }
    };
    if let Some(reason) = refusal_for_request(&request.kind) {
        return Execution::failed("analysis.unsupported_kind", reason);
    }
    let Some(kind) = planned_kind_for_request(&request.kind) else {
        let supported = REQUEST_KINDS
            .iter()
            .map(|(name, _)| *name)
            .collect::<Vec<_>>()
            .join(", ");
        return Execution::failed(
            "analysis.unsupported_kind",
            &format!(
                "Analysis kind {:?} is not supported by this engine build; supported kinds are \
                 {supported}.",
                request.kind
            ),
        );
    };
    let request_kind = request_kind_name(kind).unwrap_or(&request.kind);

    let expanded_netlist = match content {
        CircuitContent::Empty => {
            // The reserved empty circuit: a well-defined trivial solution for
            // an operating point (this is the deterministic release smoke),
            // and an honest refusal for every waveform analysis.
            if kind == PlannedAnalysisKind::Op {
                return succeeded(
                    request_kind,
                    engine_build,
                    Vec::new(),
                    Vec::new(),
                    0,
                    None,
                    abort,
                );
            }
            return Execution::failed(
                "analysis.empty_circuit",
                &format!("An empty circuit has no {request_kind} response to compute."),
            );
        }
        CircuitContent::Deck { expanded_netlist } => expanded_netlist,
    };

    let netlist = match Netlist::parse_validated(expanded_netlist) {
        Ok(netlist) => netlist,
        Err(error) => {
            return Execution::failed(
                "netlist.parse_error",
                &format!("The netlist could not be parsed: {error}"),
            );
        }
    };

    // Production solver configuration, pinned to the ngspice dialect the
    // published netlist grammar targets. This is the exact configuration the
    // release conformance covers; changing it is an engine_build change.
    let engine = Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Ngspice,
        ..SimulationConfig::default()
    });

    execute_planned_netlist(kind, request_kind, &netlist, engine_build, &engine, abort)
}

/// Refuse a deck that authors a card this build cannot represent losslessly.
///
/// The adapter runs the requested kind only, but leaving a card it cannot
/// publish unexecuted would drop authored intent from the response without
/// saying so, so the whole request is refused with the card, its canonical
/// identity, and the exact reason.
fn unmapped_deck_analysis_detail(netlist: &Netlist, plan: &DeckPlan) -> Option<String> {
    for (command, id) in plan.authored_analyses(netlist) {
        if let Some((card, reason)) = unmapped_deck_card(command) {
            let instance = id.map(|id| format!(" ({id})")).unwrap_or_default();
            return Some(format!(
                "The deck authors a {card} card{instance}. {reason}"
            ));
        }
    }
    None
}

fn execute_planned_netlist(
    kind: PlannedAnalysisKind,
    request_kind: &str,
    netlist: &Netlist,
    engine_build: &str,
    engine: &Engine,
    deadline: &dyn AbortSignal,
) -> Execution {
    let plan = match DeckPlan::from_netlist_with_abort(
        netlist,
        &engine.config().resource_limits,
        deadline,
    ) {
        Ok(plan) => plan,
        Err(error) => return deck_plan_failure(&error),
    };
    if let Err(detail) = validate_adapter_axes(&plan) {
        return Execution::failed("analysis.axis_unsupported", &detail);
    }
    if let Some(detail) = unmapped_deck_analysis_detail(netlist, &plan) {
        return Execution::failed("analysis.unsupported_kind", &detail);
    }
    let materializer =
        match engine.prepare_deck_plan_materializer_with_abort(netlist, &plan, deadline) {
            Ok(materializer) => materializer,
            Err(error) => return materialization_failure(&error),
        };
    let has_axes = !plan.axes().is_empty();
    let matching_directive_count = materializer
        .analyses()
        .iter()
        .filter(|analysis| matches_request(kind, analysis.id().kind()))
        .count();
    if matching_directive_count == 0 {
        return Execution::failed(
            "analysis.directive_missing",
            &format!(
                "The requested {request_kind} analysis needs a matching directive in the deck, \
                 and the deck declares none."
            ),
        );
    }
    let Some(axis_kind) = axis_analysis_kind(kind) else {
        return Execution::failed(
            "analysis.unsupported_kind",
            "The requested analysis family has no orchestration record in this build.",
        );
    };
    let Some(per_directive) = artifacts_per_directive(netlist, &plan, kind) else {
        return resource_failure(DirectiveFailure::ResultArtifactLimit);
    };
    if let Err(failure) = preflight_planned_artifact_count(
        materializer.len(),
        matching_directive_count,
        per_directive,
    ) {
        return resource_failure(failure);
    }

    let mut measurements = Vec::new();
    let mut artifacts = Vec::new();
    let mut retained_artifact_bytes = 0u64;
    let mut runs = Vec::<CoordinateExecution>::new();
    if has_axes && runs.try_reserve_exact(materializer.len()).is_err() {
        return Execution::failed(
            "resource.allocation",
            "The adapter could not allocate the bounded run manifest.",
        );
    }
    let mut topology: Option<TopologyFingerprint> = None;
    let mut result_schemas = BTreeMap::<String, ResultSchemaSignature>::new();
    let mut fft_signature: Option<Vec<String>> = None;
    for run_index in 0..materializer.len() {
        let materialized = match materializer.materialize_run_with_abort(run_index, deadline) {
            Ok(materialized) => materialized,
            Err(error) => return materialization_failure(&error),
        };
        if materialized.coordinate().ordinal() != run_index {
            return Execution::failed(
                "analysis.axis_materialization",
                "The canonical materializer returned a mismatched coordinate ordinal.",
            );
        }
        if has_axes {
            if let Some(expected) = topology {
                if expected != materialized.topology_fingerprint() {
                    return Execution::failed(
                        "results.conditional_topology",
                        "A run axis conditionally changes the elaborated circuit topology; the adapter refuses a result set with coordinate-dependent signal schemas.",
                    );
                }
            } else {
                topology = Some(materialized.topology_fingerprint());
            }
        }
        let coordinate = materialized.coordinate().clone();
        let run_topology = materialized.topology_fingerprint();
        let (_coordinate, coordinate_netlist, _topology, analyses) = materialized.into_parts();
        if has_axes && kind == PlannedAnalysisKind::Tran {
            let coordinate_fft_signature = fft_schema_signature(&coordinate_netlist);
            if let Some(expected) = &fft_signature {
                if expected != &coordinate_fft_signature {
                    return Execution::failed(
                        "results.conditional_schema",
                        "A run axis conditionally changes the transient FFT result schema.",
                    );
                }
            } else {
                fft_signature = Some(coordinate_fft_signature);
            }
        }

        let mut analysis_records = Vec::<AnalysisExecution>::new();
        if has_axes
            && analysis_records
                .try_reserve_exact(matching_directive_count)
                .is_err()
        {
            return Execution::failed(
                "resource.allocation",
                "The adapter could not allocate the bounded analysis manifest.",
            );
        }
        for analysis in analyses
            .iter()
            .filter(|analysis| matches_request(kind, analysis.id().kind()))
        {
            let budget = MAX_ENGINE_RETAINED_RESULT_BYTES.saturating_sub(retained_artifact_bytes);
            let outcome = match execute_analysis(
                engine,
                &coordinate_netlist,
                &plan,
                analysis,
                &analyses,
                &coordinate,
                run_topology,
                has_axes,
                deadline,
                budget,
            ) {
                Ok(outcome) => outcome,
                Err(failure) => return directive_failure(failure),
            };
            let analysis_id = analysis.output_namespace().analysis_component();
            if let Some(expected) = result_schemas.get(&analysis_id) {
                if expected != &outcome.schema_signature {
                    return Execution::failed(
                        "results.conditional_schema",
                        "A run axis conditionally changes a typed result signal schema.",
                    );
                }
            } else {
                result_schemas.insert(analysis_id.clone(), outcome.schema_signature.clone());
            }

            let Some(combined_count) = artifacts.len().checked_add(outcome.artifacts.len()) else {
                return Execution::failed(
                    "resource.result_artifact_limit",
                    "The run produced more result artifacts than the protocol permits.",
                );
            };
            if let Err(failure) = validate_artifact_budget(
                combined_count,
                artifacts
                    .iter()
                    .chain(&outcome.artifacts)
                    .map(|artifact: &PendingArtifact| artifact.content.len() as u64),
            ) {
                return resource_failure(failure);
            }
            let outcome_bytes = match outcome.artifacts.iter().try_fold(0u64, |total, artifact| {
                total.checked_add(artifact.content.len() as u64)
            }) {
                Some(bytes) => bytes,
                None => return resource_failure(DirectiveFailure::ResultSetBytes),
            };
            retained_artifact_bytes = match retained_artifact_bytes.checked_add(outcome_bytes) {
                Some(bytes) if bytes <= MAX_ENGINE_RETAINED_RESULT_BYTES => bytes,
                _ => return resource_failure(DirectiveFailure::ResultSetBytes),
            };
            let outcome_measurements = if has_axes {
                finalize_measurements(outcome.measurements)
            } else {
                outcome.measurements
            };
            let mut artifact_references = Vec::new();
            let mut measurement_documents = Vec::new();
            if has_axes {
                if artifact_references
                    .try_reserve_exact(outcome.artifacts.len())
                    .is_err()
                    || measurement_documents
                        .try_reserve_exact(outcome_measurements.len())
                        .is_err()
                {
                    return Execution::failed(
                        "resource.allocation",
                        "The adapter could not allocate bounded execution provenance.",
                    );
                }
                artifact_references.extend(outcome.artifacts.iter().map(artifact_reference));
                measurement_documents.extend(outcome_measurements.iter().map(measurement_document));
            }
            if !has_axes {
                measurements.extend(outcome_measurements);
            }
            artifacts.extend(outcome.artifacts);
            if has_axes {
                analysis_records.push(AnalysisExecution {
                    analysis_id,
                    output_namespace: OutputNamespaceDocument {
                        coordinate: analysis.output_namespace().coordinate_component(),
                        analysis: analysis.output_namespace().analysis_component(),
                    },
                    artifacts: artifact_references,
                    measurements: measurement_documents,
                });
            }
        }
        if has_axes && analysis_records.len() != matching_directive_count {
            return Execution::failed(
                "analysis.axis_materialization",
                "A materialized coordinate did not preserve the requested directive cardinality.",
            );
        }
        if has_axes {
            let run = match run_manifest(&coordinate, analysis_records) {
                Ok(run) => run,
                Err(detail) => return Execution::failed("results.schema_mismatch", &detail),
            };
            runs.push(run);
        }
    }

    if let Err(failure) = validate_pending_artifact_budget(&artifacts) {
        return resource_failure(failure);
    }

    let axis_execution = if has_axes {
        match AxisExecutionDocument::new_with_abort(axis_kind, runs, deadline) {
            Ok(document) => Some(document),
            Err(error) => {
                return Execution::failed("results.schema_mismatch", &error.to_string());
            }
        }
    } else {
        None
    };
    succeeded(
        request_kind,
        engine_build,
        finalize_measurements(measurements),
        artifacts,
        matching_directive_count,
        axis_execution,
        deadline,
    )
}

fn axis_analysis_kind(kind: PlannedAnalysisKind) -> Option<AxisAnalysisKind> {
    Some(match kind {
        PlannedAnalysisKind::Op | PlannedAnalysisKind::ImplicitOp => {
            AxisAnalysisKind::OperatingPoint
        }
        PlannedAnalysisKind::Dc => AxisAnalysisKind::DcSweep,
        PlannedAnalysisKind::Tran => AxisAnalysisKind::Transient,
        PlannedAnalysisKind::Ac => AxisAnalysisKind::AcSmallSignal,
        PlannedAnalysisKind::Noise => AxisAnalysisKind::Noise,
        PlannedAnalysisKind::Distortion => AxisAnalysisKind::Distortion,
        PlannedAnalysisKind::TransferFunction => AxisAnalysisKind::TransferFunction,
        PlannedAnalysisKind::Stb => AxisAnalysisKind::Stability,
        PlannedAnalysisKind::Sensitivity => AxisAnalysisKind::Sensitivity,
        PlannedAnalysisKind::PoleZero => AxisAnalysisKind::PoleZero,
        PlannedAnalysisKind::MonteCarlo => AxisAnalysisKind::MonteCarlo,
        PlannedAnalysisKind::HarmonicBalance => AxisAnalysisKind::HarmonicBalance,
        PlannedAnalysisKind::Pss => AxisAnalysisKind::Pss,
        PlannedAnalysisKind::Pac => AxisAnalysisKind::Pac,
        PlannedAnalysisKind::PNoise => AxisAnalysisKind::PNoise,
        PlannedAnalysisKind::Sp => AxisAnalysisKind::SParameters,
        PlannedAnalysisKind::Envelope => AxisAnalysisKind::Envelope,
        _ => return None,
    })
}

fn validate_adapter_axes(plan: &DeckPlan) -> Result<(), String> {
    for axis in plan.axes() {
        if !matches!(axis.kind(), AxisKind::Step | AxisKind::Temperature) {
            return Err(format!(
                "The {:?} axis {:?} is not representable by this adapter contract.",
                axis.kind(),
                axis.name()
            ));
        }
        if axis
            .values()
            .iter()
            .any(|value| !matches!(value, RunAxisValue::Numeric(_)))
        {
            return Err(format!(
                "Run axis {:?} contains a non-numeric coordinate value.",
                axis.name()
            ));
        }
        if axis.kind() == AxisKind::Step
            && !matches!(
                axis.step_target(),
                Some(
                    StepAxisTarget::Parameter { .. }
                        | StepAxisTarget::Device { .. }
                        | StepAxisTarget::Model { .. }
                )
            )
        {
            return Err(format!(
                "Run axis {:?} has an unsupported or missing STEP target.",
                axis.name()
            ));
        }
        if axis.kind() == AxisKind::Temperature
            && axis
                .step_target()
                .is_some_and(|target| !matches!(target, StepAxisTarget::Temperature))
        {
            return Err(format!(
                "Temperature axis {:?} has an incompatible STEP target.",
                axis.name()
            ));
        }
    }
    Ok(())
}

fn fft_schema_signature(netlist: &Netlist) -> Vec<String> {
    let mut signature = vec![format!(
        "mode={:?};metrics={:?}",
        netlist.options.fft_mode, netlist.options.fft_output_metrics
    )];
    signature.extend(
        netlist
            .fft_analyses
            .iter()
            .map(|analysis| format!("output={:?};format={:?}", analysis.output, analysis.format)),
    );
    signature
}
