//! Immutable simulation preparation and one-use dispatch authorization.
//!
//! The engine never receives live editor state. A controller first resolves
//! every execution input into a [`PreparedRunSnapshot`], then consumes the
//! snapshot's generation-bound permit immediately before dispatch.

mod authorization;
mod permit;
#[cfg(test)]
mod qpss_artifact_tests;
mod snapshot;

pub(in crate::simulation) use authorization::PreparedRunAuthorization;
#[cfg(test)]
pub(in crate::simulation) use permit::ExecutionPermitIssuer;
pub(crate) use rspice_simulation::preparation::{PreparationError, PreparationStage};
pub(in crate::simulation) use snapshot::bound_cards;
pub(in crate::simulation) use snapshot::{
    AuthorizedRunDispatch, AuthorizedTaskDispatch, ExecutionTargetCapabilities,
    ModelSourceIdentity, PSS_SPECTRUM_ROLE, PreparedRunSet, PreparedRunSnapshot, PreparedTask,
    ResolvedTaskDispatch, RunSourceReceipt, SavePolicy, SnapshotParts, TouchstoneExportPolicy,
    result_source_domain,
};
pub(crate) use snapshot::{PreparedRunMetadata, execution_target_supports_cancellation};

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
