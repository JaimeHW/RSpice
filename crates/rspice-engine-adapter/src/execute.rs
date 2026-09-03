//! Deck execution: request-kind resolution, canonical planning, engine
//! invocation, typed result publication, and bounded artifact emission.
//!
//! # Result contract
//!
//! Every analysis this executor runs publishes exactly one
//! [`rspice_core::execution::AnalysisResultDocument`]. The document names the
//! canonical analysis identity the planner assigned, the shared-deck
//! coordinate it was produced at, the elaborated topology fingerprint, and the
//! output and checkpoint namespaces. The adapter defines no result schema of
//! its own for those families: the artifact content type is derived from the
//! core document's schema identifier and version.
//!
//! Two adapter-owned documents remain, and neither is a result projection:
//! [`crate::axis_execution_document`] is the STEP/TEMP orchestration record
//! and references the typed documents by path, schema, and family, and
//! [`crate::fft_result_document`] carries transient `.FFT` spectra until core
//! assigns post-process analysis identities (see that module).
//!
//! # Protocol compatibility
//!
//! Protocol 4 replaced protocol 3 with this contract. Relative to protocol 3 a
//! consumer must expect:
//!
//! * every analysis family the planner recognizes, not only OP/DC/AC/TRAN/
//!   NOISE, with `.SP`, `.PNOISE`, and `.FOUR` refused by name rather than
//!   silently skipped;
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
use crate::fft_result_document::{
    FFT_RESULT_DOCUMENT_CONTENT_TYPE, FFT_RESULT_DOCUMENT_SCHEMA, FFT_RESULT_DOCUMENT_VERSION,
    FftResultDocumentError,
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

/// Result-manifest format tag. Advanced with the protocol version whenever the
/// published result contract changes shape.
pub const RESULT_MANIFEST_FORMAT: &str = "rspice-result-v3";

/// Default wall-clock ceiling for all engine work in one request. The worker
/// holds the authoritative external deadline; this internal one exists so a
/// pathological deck produces the bounded `engine.time_limit` outcome instead
/// of an opaque kill.
pub const DEFAULT_SOLVE_BUDGET: Duration = Duration::from_secs(240);

/// Launch variable that overrides [`DEFAULT_SOLVE_BUDGET`].
///
/// The value is a positive number of seconds. The launch contract clears the
/// environment before starting this executor, so the only way this is set is
/// the worker deliberately setting it, which is exactly the authority that
/// owns the external deadline it has to stay under.
pub const SOLVE_BUDGET_SECONDS_ENV: &str = "RSPICE_ENGINE_SOLVE_BUDGET_SECONDS";

/// Resolve the solve budget from the launch environment.
///
/// A malformed or non-positive value is refused rather than rounded into
/// something plausible: silently substituting the default for a budget the
/// operator meant to shorten would let a request outlive the deadline the
/// worker is holding it to.
pub fn solve_budget_from_env() -> Result<Duration, String> {
    // Read through `var_os` so a value that is not valid UTF-8 is refused as
    // a malformed budget rather than mistaken for an unset one.
    let Some(value) = std::env::var_os(SOLVE_BUDGET_SECONDS_ENV) else {
        return Ok(DEFAULT_SOLVE_BUDGET);
    };
    let value = value.to_str().ok_or_else(|| {
        format!("{SOLVE_BUDGET_SECONDS_ENV} is not valid UTF-8; expected a number of seconds")
    })?;
    parse_solve_budget(value)
}

fn parse_solve_budget(value: &str) -> Result<Duration, String> {
    let seconds: f64 = value.trim().parse().map_err(|_| {
        format!("{SOLVE_BUDGET_SECONDS_ENV} is {value:?}; expected a positive number of seconds")
    })?;
    if !seconds.is_finite() || seconds <= 0.0 {
        return Err(format!(
            "{SOLVE_BUDGET_SECONDS_ENV} is {value:?}; expected a positive number of seconds"
        ));
    }
    Duration::try_from_secs_f64(seconds).map_err(|_| {
        format!("{SOLVE_BUDGET_SECONDS_ENV} is {value:?}; the budget does not fit a duration")
    })
}

/// The engine-owned analysis request. Version 1 selects which deck-carried
/// directive class runs; directive parameters stay in the deck so the digest
/// over the revision covers the complete simulation definition.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AnalysisRequestV1 {
    kind: String,
}

/// Why one request stopped early.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopCause {
    /// The caller asked this process to stop.
    Cancelled,
    /// The solve budget expired.
    Deadline,
}

const STOP_NONE: u8 = 0;
const STOP_CANCELLED: u8 = 1;
const STOP_DEADLINE: u8 = 2;

/// The one abort source every analysis in a request sees: the caller's
/// cancellation or this executor's own solve budget, whichever fires first.
///
/// Which of the two fired is remembered because the two are different
/// outcomes to the operator — a cancelled job is not a job that ran too long
/// — and by the time the engine has unwound to the response mapping the
/// distinction is otherwise gone: `SimulationError::Aborted` says only that
/// something asked it to stop.
pub struct RunAbort<'a> {
    start: Instant,
    budget: Duration,
    cancel: &'a dyn AbortSignal,
    cause: AtomicU8,
}

impl<'a> RunAbort<'a> {
    pub fn new(cancel: &'a dyn AbortSignal, budget: Duration) -> Self {
        Self {
            start: Instant::now(),
            budget,
            cancel,
            cause: AtomicU8::new(STOP_NONE),
        }
    }

    /// The first cause observed, or `None` while the request is still live.
    pub fn stop_cause(&self) -> Option<StopCause> {
        match self.cause.load(Ordering::SeqCst) {
            STOP_CANCELLED => Some(StopCause::Cancelled),
            STOP_DEADLINE => Some(StopCause::Deadline),
            _ => None,
        }
    }

    fn record(&self, cause: u8) -> bool {
        let _ = self
            .cause
            .compare_exchange(STOP_NONE, cause, Ordering::SeqCst, Ordering::SeqCst);
        true
    }
}

impl AbortSignal for RunAbort<'_> {
    fn is_aborted(&self) -> bool {
        // Cancellation is checked first: a caller that asked to stop deserves
        // that answer even if the budget expired in the same instant.
        if self.cancel.is_aborted() {
            return self.record(STOP_CANCELLED);
        }
        if self.start.elapsed() >= self.budget {
            return self.record(STOP_DEADLINE);
        }
        false
    }
}

/// One executed request: the wire response plus the results files it
/// declared, staged in memory until `main` writes and emits them together.
pub struct Execution {
    pub response: EngineResponse,
    pub artifacts: Vec<PendingArtifact>,
}

impl Execution {
    fn failed(failure_code: &str, failure_detail: &str) -> Self {
        Self {
            response: EngineResponse::failed(failure_code, failure_detail),
            artifacts: Vec::new(),
        }
    }
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
    if let Err(failure) = preflight_planned_artifact_count(
        materializer.len(),
        matching_directive_count,
        kind,
        !netlist.fft_analyses.is_empty(),
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

/// Everything one executed analysis contributes to the response.
struct AnalysisOutcome {
    measurements: Vec<Measurement>,
    artifacts: Vec<PendingArtifact>,
    schema_signature: ResultSchemaSignature,
}

/// Run one materialized analysis and stage its typed artifacts.
#[allow(clippy::too_many_arguments)]
// The identity a result document carries — analysis, coordinate, topology,
// namespaces — is exactly this many independent inputs, and threading them
// through a struct would only move the same list one call earlier.
fn execute_analysis(
    engine: &Engine,
    netlist: &Netlist,
    analysis: &MaterializedAnalysis,
    peers: &[MaterializedAnalysis],
    coordinate: &RunCoordinate,
    topology: TopologyFingerprint,
    has_axes: bool,
    abort: &dyn AbortSignal,
    byte_limit: u64,
) -> Result<AnalysisOutcome, DirectiveFailure> {
    let projection = run_directive(engine, netlist, analysis, peers, abort)?;
    let mut builder = projection
        .builder
        .topology_fingerprint(topology)
        .namespaces(ResultNamespaces {
            output: analysis.output_namespace().components().join("/"),
            checkpoint: analysis.checkpoint_namespace().components().join("/"),
        });
    if has_axes {
        builder = builder.coordinate(ResultCoordinate::from_run_coordinate(coordinate));
    }
    let document = builder
        .build_with_abort(abort)
        .map_err(crate::failure::map_result_document_error)?;
    let schema_signature = ResultSchemaSignature::from_document(&document);
    let measurements = measurements_from_document(&document, abort)?;

    let file_stem = artifact_stem(analysis, coordinate, has_axes);
    let artifact = encode_result_artifact(
        format!("{file_stem}.result.json"),
        &document,
        abort,
        byte_limit,
    )?;
    let mut artifacts = vec![artifact];

    // Transient `.FFT` spectra keep their adapter-owned bundle until core
    // assigns post-process analysis identities; see `fft_result_document`.
    if let Some(fft) = projection.fft {
        let remaining = remaining_bytes(&artifacts, byte_limit)?;
        let content = fft
            .to_json_with_abort(abort, remaining.min(MAX_ENGINE_ARTIFACT_BYTES))
            .map_err(map_fft_document_error)?;
        artifacts.push(PendingArtifact {
            file_name: format!("{file_stem}.fft.result.json"),
            content_type: FFT_RESULT_DOCUMENT_CONTENT_TYPE.to_owned(),
            result_kind: "fft".to_owned(),
            content,
        });
    }
    remaining_bytes(&artifacts, byte_limit)?;

    Ok(AnalysisOutcome {
        measurements,
        artifacts,
        schema_signature,
    })
}

/// Bytes of the analysis budget still unused after the staged artifacts.
fn remaining_bytes(
    artifacts: &[PendingArtifact],
    byte_limit: u64,
) -> Result<u64, DirectiveFailure> {
    artifacts
        .iter()
        .try_fold(0u64, |total, artifact| {
            total.checked_add(artifact.content.len() as u64)
        })
        .and_then(|used| byte_limit.checked_sub(used))
        .ok_or(DirectiveFailure::ResultSetBytes)
}

/// Canonical artifact stem for one analysis at one coordinate.
fn artifact_stem(
    analysis: &MaterializedAnalysis,
    coordinate: &RunCoordinate,
    has_axes: bool,
) -> String {
    let analysis_component = analysis.output_namespace().analysis_component();
    if has_axes {
        format!("{}__{analysis_component}", coordinate.stable_tag())
    } else {
        analysis_component
    }
}

fn artifact_reference(artifact: &PendingArtifact) -> ResultDocumentReference {
    let (schema, schema_version, result_kind) =
        if artifact.content_type == FFT_RESULT_DOCUMENT_CONTENT_TYPE {
            (
                FFT_RESULT_DOCUMENT_SCHEMA.to_owned(),
                FFT_RESULT_DOCUMENT_VERSION,
                "fft".to_owned(),
            )
        } else {
            (
                rspice_core::execution::ANALYSIS_RESULT_DOCUMENT_SCHEMA.to_owned(),
                rspice_core::execution::ANALYSIS_RESULT_DOCUMENT_VERSION,
                artifact.result_kind.clone(),
            )
        };
    ResultDocumentReference {
        path: format!("results/{}", artifact.file_name),
        content_type: artifact.content_type.clone(),
        schema,
        schema_version,
        result_kind,
    }
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
        PlannedAnalysisKind::Envelope => AxisAnalysisKind::Envelope,
        _ => return None,
    })
}

fn deck_plan_failure(error: &rspice_core::execution::DeckPlanError) -> Execution {
    if matches!(error, rspice_core::execution::DeckPlanError::Aborted) {
        failure_execution(&SimulationError::Aborted)
    } else {
        Execution::failed(
            "analysis.axis_materialization",
            &format!("The deck run plan is invalid: {error}"),
        )
    }
}

fn materialization_failure(error: &MaterializedRunError) -> Execution {
    match error {
        MaterializedRunError::Aborted => failure_execution(&SimulationError::Aborted),
        MaterializedRunError::Simulation(error) => failure_execution(error),
        MaterializedRunError::AlterUnsupported => Execution::failed(
            "analysis.axis_unsupported",
            "Textual .ALTER variants are not representable by this adapter contract.",
        ),
        other => Execution::failed(
            "analysis.axis_materialization",
            &format!("The canonical deck materializer failed: {other}"),
        ),
    }
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

fn preflight_planned_artifact_count(
    coordinate_count: usize,
    directive_count: usize,
    kind: PlannedAnalysisKind,
    has_fft: bool,
) -> Result<(), DirectiveFailure> {
    let artifacts_per_directive =
        1usize + usize::from(has_fft && kind == PlannedAnalysisKind::Tran);
    let count = coordinate_count
        .checked_mul(directive_count)
        .and_then(|count| count.checked_mul(artifacts_per_directive))
        .ok_or(DirectiveFailure::ResultArtifactLimit)?;
    validate_artifact_budget(count, std::iter::empty())
}

fn resource_failure(failure: DirectiveFailure) -> Execution {
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
fn directive_failure(failure: DirectiveFailure) -> Execution {
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

fn validate_artifact_budget(
    artifact_count: usize,
    byte_lengths: impl IntoIterator<Item = u64>,
) -> Result<(), DirectiveFailure> {
    if artifact_count > MAX_ENGINE_RESULT_ARTIFACTS {
        return Err(DirectiveFailure::ResultArtifactLimit);
    }
    let mut total = 0u64;
    for length in byte_lengths {
        if !(1..=MAX_ENGINE_ARTIFACT_BYTES).contains(&length) {
            return Err(DirectiveFailure::ResultArtifactBytes);
        }
        total = total
            .checked_add(length)
            .ok_or(DirectiveFailure::ResultSetBytes)?;
        if total > MAX_ENGINE_RETAINED_RESULT_BYTES {
            return Err(DirectiveFailure::ResultSetBytes);
        }
    }
    Ok(())
}

fn validate_pending_artifact_budget(artifacts: &[PendingArtifact]) -> Result<(), DirectiveFailure> {
    validate_artifact_budget(
        artifacts.len(),
        artifacts
            .iter()
            .map(|artifact| artifact.content.len() as u64),
    )
}

fn map_fft_document_error(error: FftResultDocumentError) -> DirectiveFailure {
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

fn measurement_document(measurement: &Measurement) -> MeasurementDocument {
    MeasurementDocument {
        name: measurement.name.clone(),
        unit: measurement.unit.clone(),
        value_decimal: measurement.value_decimal.clone(),
        sample_count: measurement.sample_count,
        series_sha256: measurement.series_sha256.clone(),
    }
}

fn run_manifest(
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

fn succeeded(
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

enum WireSerializationFailure {
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

fn finalize_execution(execution: Execution, abort: &dyn AbortSignal) -> Execution {
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

/// A request stopped because its caller asked it to.
pub const CANCELLED_FAILURE_CODE: &str = "engine.cancelled";
/// A request stopped because it exhausted its solve budget.
pub const TIME_LIMIT_FAILURE_CODE: &str = "engine.time_limit";

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
fn failure_execution(error: &SimulationError) -> Execution {
    let code = match error.descriptor().code {
        rspice_core::engine::SimulationErrorCode::Aborted => CANCELLED_FAILURE_CODE.to_owned(),
        stable => format!("engine.{}", stable.as_str()),
    };
    Execution::failed(&code, &error.to_string())
}

/// Re-label a stop outcome once the whole request has unwound and the cause
/// that fired first is known.
fn label_stop_cause(execution: Execution, cause: Option<StopCause>) -> Execution {
    let EngineResponse::Failed {
        failure_code,
        failure_detail,
    } = &execution.response
    else {
        return execution;
    };
    if failure_code != CANCELLED_FAILURE_CODE {
        return execution;
    }
    match cause {
        // Nothing this executor owns fired, so the engine stopped for an
        // abort source further in. Leave the honest cancellation label.
        None | Some(StopCause::Cancelled) => execution,
        Some(StopCause::Deadline) => Execution {
            response: EngineResponse::failed(TIME_LIMIT_FAILURE_CODE, failure_detail),
            artifacts: execution.artifacts,
        },
    }
}

/// Stages the complete response-gated artifact set, atomically replaces each
/// member, and restores every predecessor on a returned commit failure. The
/// caller emits the success manifest only after this function completes, so
/// an interrupted process never advertises a partial set.
pub fn write_artifacts(results_dir: &Path, artifacts: &[PendingArtifact]) -> Result<(), String> {
    validate_pending_artifact_budget(artifacts)
        .map_err(|_| "result artifact set exceeds the protocol budget".to_owned())?;
    let mut names = HashSet::new();
    for artifact in artifacts {
        let result_path = format!("results/{}", artifact.file_name);
        if !valid_result_path(&result_path) {
            return Err("result artifact set contains an invalid destination name".to_owned());
        }
        if !names.insert(artifact.file_name.to_ascii_lowercase()) {
            return Err("result artifact set contains duplicate destinations".to_owned());
        }
    }
    let mut transaction = AtomicArtifactSet::new();
    for artifact in artifacts {
        let destination = results_dir.join(&artifact.file_name);
        transaction
            .stage::<io::Error, _>(&destination, |writer| {
                writer.write_all(artifact.content.as_bytes())
            })
            .map_err(|error| error.to_string())?;
    }
    transaction.commit().map_err(|error| error.to_string())
}

#[cfg(test)]
fn publish_artifact<E>(
    destination: &Path,
    write: impl FnOnce(&mut dyn std::io::Write) -> Result<(), E>,
) -> Result<(), rspice_output::AtomicArtifactError<E>>
where
    E: std::error::Error + 'static,
{
    write_atomic(destination, write)
}

#[cfg(test)]
mod artifact_publication_tests {
    use super::*;
    use rspice_output::{AtomicArtifactError, stale_artifacts};
    use std::path::PathBuf;
    use std::sync::atomic::AtomicU64;

    static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new(tag: &str) -> Self {
            let serial = NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "rspice-engine-adapter-{tag}-{}-{serial}",
                std::process::id()
            ));
            std::fs::create_dir(&path).expect("create adapter publication test directory");
            Self(path)
        }

        fn destination(&self) -> PathBuf {
            self.0.join("result.json")
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn artifact(file_name: &str, content: &str) -> PendingArtifact {
        PendingArtifact {
            file_name: file_name.to_owned(),
            content_type: "application/json".to_owned(),
            result_kind: "op".to_owned(),
            content: content.to_owned(),
        }
    }

    fn seed(destination: &Path, preexisting: bool) {
        if preexisting {
            std::fs::write(destination, b"old complete artifact")
                .expect("seed existing adapter artifact");
        }
    }

    fn assert_old_or_absent(destination: &Path, preexisting: bool) {
        if preexisting {
            assert_eq!(
                std::fs::read(destination).expect("read preserved adapter artifact"),
                b"old complete artifact"
            );
        } else {
            assert!(
                !destination.exists(),
                "failed publication created a destination"
            );
        }
        assert!(
            stale_artifacts(destination)
                .expect("inspect adapter staging artifacts")
                .is_empty(),
            "failed publication left a staging artifact"
        );
    }

    #[test]
    fn artifact_write_failure_preserves_existing_or_absent_destination() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("failure");
            let destination = directory.destination();
            seed(&destination, preexisting);

            let error = publish_artifact(&destination, |writer| -> io::Result<()> {
                writer.write_all(b"partial replacement")?;
                Err(io::Error::other("injected adapter serialization failure"))
            })
            .expect_err("injected artifact write must fail");

            assert!(matches!(error, AtomicArtifactError::Write(_)));
            assert_old_or_absent(&destination, preexisting);
        }
    }

    #[test]
    fn artifact_success_replaces_existing_or_absent_destination() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("success");
            let destination = directory.destination();
            seed(&destination, preexisting);

            write_artifacts(
                &directory.0,
                &[artifact("result.json", "{\"complete\":true}\n")],
            )
            .expect("publish adapter artifact");

            assert_eq!(
                std::fs::read(&destination).expect("read published adapter artifact"),
                b"{\"complete\":true}\n"
            );
            assert!(
                stale_artifacts(&destination)
                    .expect("inspect adapter staging artifacts")
                    .is_empty(),
                "successful publication left a staging artifact"
            );
        }
    }

    #[test]
    fn result_artifact_count_and_byte_budgets_accept_only_the_exact_boundaries() {
        assert!(
            validate_artifact_budget(
                MAX_ENGINE_RESULT_ARTIFACTS,
                [MAX_ENGINE_RETAINED_RESULT_BYTES]
            )
            .is_ok()
        );
        assert!(matches!(
            validate_artifact_budget(1, [MAX_ENGINE_RETAINED_RESULT_BYTES + 1]),
            Err(DirectiveFailure::ResultSetBytes)
        ));
        assert!(matches!(
            validate_artifact_budget(2, [MAX_ENGINE_RETAINED_RESULT_BYTES, 1]),
            Err(DirectiveFailure::ResultSetBytes)
        ));
        assert!(matches!(
            validate_artifact_budget(MAX_ENGINE_RESULT_ARTIFACTS + 1, [0]),
            Err(DirectiveFailure::ResultArtifactLimit)
        ));
        assert!(matches!(
            validate_artifact_budget(1, [MAX_ENGINE_ARTIFACT_BYTES + 1]),
            Err(DirectiveFailure::ResultArtifactBytes)
        ));
        assert!(matches!(
            validate_artifact_budget(1, [0]),
            Err(DirectiveFailure::ResultArtifactBytes)
        ));
    }

    #[test]
    fn transient_default_tmax_uses_the_shared_core_contract() {
        assert!(
            matches!(rspice_core::execution::resolve_transient_maximum_step(1.0e-6, 1.0e-3, None, None), Ok(value) if value == 1.0e-6),
            "TSTEP is the default ceiling when it is smaller"
        );
        assert!(
            matches!(rspice_core::execution::resolve_transient_maximum_step(10.0e-3, 1.0, Some(0.9), None), Ok(value) if (value - 2.0e-3).abs() < 1.0e-15),
            "(TSTOP-TSTART)/50 is the default ceiling when it is smaller"
        );
        assert!(
            matches!(rspice_core::execution::resolve_transient_maximum_step(1.0e-6, 1.0e-3, None, Some(7.0e-6)), Ok(value) if value == 7.0e-6),
            "an explicit valid TMAX overrides both defaults"
        );
        assert!(
            rspice_core::execution::resolve_transient_maximum_step(1.0e-6, 1.0e-3, None, Some(0.0))
                .is_err()
        );
        assert!(
            rspice_core::execution::resolve_transient_maximum_step(1.0e-6, 1.0, Some(1.0), None)
                .is_err()
        );
    }

    #[test]
    fn result_artifact_destinations_are_single_component_and_case_unique() {
        let directory = TestDirectory::new("destination-validation");
        for invalid in [
            "../escape.json",
            "sub/escape.json",
            "sub\\escape.json",
            "C:escape",
        ] {
            let error = write_artifacts(&directory.0, &[artifact(invalid, "complete")])
                .expect_err("invalid artifact destination must fail closed");
            assert!(error.contains("invalid destination name"), "{error}");
        }

        let duplicate = [
            artifact("Result.json", "first"),
            artifact("result.json", "second"),
        ];
        let error = write_artifacts(&directory.0, &duplicate)
            .expect_err("case-folded duplicate destinations must fail closed");
        assert!(error.contains("duplicate destinations"), "{error}");
        assert!(
            std::fs::read_dir(&directory.0)
                .expect("read destination-validation directory")
                .next()
                .is_none()
        );
    }

    #[test]
    fn later_commit_failure_rolls_back_absent_and_preexisting_predecessors() {
        for preexisting in [false, true] {
            let directory = TestDirectory::new("set-rollback");
            let first = directory.0.join("first.json");
            seed(&first, preexisting);
            let invalid_second = directory.0.join("second.json");
            std::fs::create_dir(&invalid_second).expect("create commit-failing destination");
            let artifacts = [
                artifact("first.json", "new first"),
                artifact("second.json", "new second"),
            ];

            let error = write_artifacts(&directory.0, &artifacts)
                .expect_err("second destination must fail commit");
            assert!(error.contains("predecessor set was restored"), "{error}");
            assert_old_or_absent(&first, preexisting);
            assert!(invalid_second.is_dir());
            let names = std::fs::read_dir(&directory.0)
                .expect("read transaction directory")
                .map(|entry| entry.expect("read transaction entry").file_name())
                .collect::<Vec<_>>();
            assert!(
                names.iter().all(|name| !name
                    .to_string_lossy()
                    .contains(rspice_output::PREDECESSOR_MARKER)),
                "rollback backup remained: {names:?}"
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn non_regular_predecessors_fail_before_any_destination_is_committed() {
        use std::os::unix::net::UnixListener;

        let directory = TestDirectory::new("non-regular-predecessor");
        let first = directory.0.join("first.json");
        std::fs::write(&first, b"old first").expect("seed first predecessor");
        let socket = directory.0.join("second.json");
        let _listener = UnixListener::bind(&socket).expect("bind predecessor socket");
        let artifacts = [
            artifact("first.json", "new first"),
            artifact("second.json", "new second"),
        ];

        let error = write_artifacts(&directory.0, &artifacts)
            .expect_err("a socket predecessor must fail closed");
        assert!(
            error.contains("neither a regular file nor a directory"),
            "{error}"
        );
        assert_eq!(
            std::fs::read(&first).expect("read untouched first predecessor"),
            b"old first"
        );
        assert!(socket.exists());
    }

    #[test]
    fn cancelled_axis_planning_returns_a_cancellation_without_artifacts() {
        let netlist = Netlist::parse_validated(
            "cancelled axis plan\n\
             .param r=1k\n\
             V1 in 0 1\n\
             R1 in 0 {r}\n\
             .step param r list 1k 2k\n\
             .op\n\
             .end\n",
        )
        .expect("axis fixture parses");
        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Ngspice,
            ..SimulationConfig::default()
        });
        let execution = execute_planned_netlist(
            PlannedAnalysisKind::Op,
            "operating_point",
            &netlist,
            "test",
            &engine,
            &rspice_core::abort_signal::ImmediateAbort,
        );
        assert!(execution.artifacts.is_empty());
        match execution.response {
            EngineResponse::Failed { failure_code, .. } => {
                assert_eq!(failure_code, CANCELLED_FAILURE_CODE)
            }
            EngineResponse::Succeeded { .. } => panic!("cancelled planning succeeded"),
        }
    }
}

/// A stop is either the caller's or the clock's, and the wire says which.
#[cfg(test)]
mod stop_cause_tests {
    use super::*;
    use rspice_core::abort_signal::{ImmediateAbort, NoAbort};

    const STEP_DECK: &str = "stop cause fixture\n\
         .param r=1k\n\
         V1 in 0 1\n\
         R1 in 0 {r}\n\
         .step param r list 1k 2k\n\
         .op\n\
         .end\n";

    fn content() -> CircuitContent {
        CircuitContent::Deck {
            expanded_netlist: STEP_DECK.to_owned(),
        }
    }

    fn failure_code(execution: &Execution) -> &str {
        match &execution.response {
            EngineResponse::Failed { failure_code, .. } => failure_code,
            EngineResponse::Succeeded { .. } => panic!("a stopped request reported success"),
        }
    }

    #[test]
    fn a_caller_requested_stop_is_reported_as_a_cancellation() {
        let execution = execute_with_abort(
            &serde_json::json!({"kind": "operating_point"}),
            &content(),
            "test",
            &ImmediateAbort,
            DEFAULT_SOLVE_BUDGET,
        );

        assert_eq!(failure_code(&execution), CANCELLED_FAILURE_CODE);
        assert!(
            execution.artifacts.is_empty(),
            "a cancelled request must declare no artifacts"
        );
    }

    #[test]
    fn an_exhausted_solve_budget_is_reported_as_a_time_limit() {
        // A zero budget is expired at its first poll, which makes the deadline
        // branch deterministic instead of a race against a real solve.
        let execution = execute_with_abort(
            &serde_json::json!({"kind": "operating_point"}),
            &content(),
            "test",
            &NoAbort,
            Duration::ZERO,
        );

        assert_eq!(failure_code(&execution), TIME_LIMIT_FAILURE_CODE);
        assert!(
            execution.artifacts.is_empty(),
            "a request that ran out of time must declare no artifacts"
        );
    }

    #[test]
    fn cancellation_wins_when_both_causes_are_live() {
        let execution = execute_with_abort(
            &serde_json::json!({"kind": "operating_point"}),
            &content(),
            "test",
            &ImmediateAbort,
            Duration::ZERO,
        );

        assert_eq!(
            failure_code(&execution),
            CANCELLED_FAILURE_CODE,
            "a caller who asked to stop must be told their request was cancelled, \
             not that it was too slow"
        );
    }

    #[test]
    fn a_completed_request_is_never_relabelled() {
        let execution = execute_with_abort(
            &serde_json::json!({"kind": "operating_point"}),
            &CircuitContent::Deck {
                expanded_netlist: "trivial\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n".to_owned(),
            },
            "test",
            &NoAbort,
            DEFAULT_SOLVE_BUDGET,
        );

        assert!(matches!(
            execution.response,
            EngineResponse::Succeeded { .. }
        ));
    }

    #[test]
    fn the_solve_budget_override_is_parsed_and_bounded() {
        assert_eq!(
            parse_solve_budget("30"),
            Ok(Duration::from_secs(30)),
            "a plain number of seconds is the documented form"
        );
        assert_eq!(parse_solve_budget(" 0.5 "), Ok(Duration::from_millis(500)));
        for refused in ["0", "-1", "abc", "", "inf", "NaN"] {
            assert!(
                parse_solve_budget(refused).is_err(),
                "{refused:?} must be refused rather than silently defaulted"
            );
        }
    }
}
