//! Immutable simulation preparation and one-use dispatch authorization.
//!
//! The engine never receives live editor state. A controller first resolves
//! every execution input into a [`PreparedRunSnapshot`], then consumes the
//! snapshot's generation-bound permit immediately before dispatch.

mod artifact;
mod canonical;
mod permit;
mod sealed_source;
mod snapshot;

pub(in crate::simulation) use artifact::{
    ExecutionArtifactEnvelope, ExecutionArtifactKind, PreparedDependencyBinding,
    ResolvedExecutionDependencies, TransientTrajectoryArtifact,
};
#[cfg(any(target_arch = "wasm32", test))]
pub(in crate::simulation) use canonical::f64_sequence_digest;
pub(in crate::simulation) use canonical::{
    analysis_kind_tag, canonical_analysis_kind, content_digest, drc_receipt_digest,
    manual_deck_analysis_instance_id, manual_source_receipt_digest,
    operating_point_effective_source_digest, sealed_dependency_closure_digest,
};
pub(in crate::simulation) use permit::{ExecutionPermit, ExecutionPermitIssuer};
pub(crate) use sealed_source::sealed_executable_source_digest;
pub(in crate::simulation) use sealed_source::{
    generated_executable_source_digest, manual_executable_source_digest,
};
pub(in crate::simulation) use snapshot::{
    AuthorizedRunDispatch, AuthorizedTaskDispatch, CrossProbeSnapshot, ExecutionTargetCapabilities,
    ModelSourceIdentity, PSS_SPECTRUM_ROLE, PreparedRunSet, PreparedRunSnapshot, PreparedTask,
    ResolvedTaskDispatch, RunSourceReceipt, SavePolicy, SnapshotParts, TouchstoneExportPolicy,
    result_source_domain,
};
pub(crate) use snapshot::{
    PreparationError, PreparationStage, PreparedRunMetadata, execution_target_supports_cancellation,
};

/// What the execution target will do with a multi-point run: the target's own
/// name, and how many of its tasks run at once.
///
/// A declared space immediately raises "how long will that take", and the
/// honest answer is a property of the target rather than of the plan. It is
/// read here rather than restated on a form, so a surface can never quote a
/// concurrency the dispatcher does not have.
#[must_use]
pub(crate) fn execution_target_parallelism() -> (&'static str, u64) {
    let capabilities = ExecutionTargetCapabilities::current();
    (capabilities.label(), capabilities.max_parallel_tasks)
}

/// The deck line a parse failure named, where it named one.
///
/// Read out of the error's own fields, never out of its prose. A preflight
/// blocker that offers to open the netlist at a line has to be sure the line
/// came from the parser rather than from a sentence that happened to contain
/// a number.
pub(in crate::simulation) fn parse_error_line(
    error: &rspice_core::netlist::ParseError,
) -> Option<usize> {
    use rspice_core::netlist::ParseError;
    match error {
        ParseError::Syntax { line, .. } => Some(*line),
        ParseError::DuplicateName { duplicate_line, .. } => Some(*duplicate_line),
        _ => None,
    }
}
