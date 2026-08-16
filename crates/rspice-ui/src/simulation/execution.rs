//! Immutable simulation preparation and one-use dispatch authorization.
//!
//! The engine never receives live editor state. A controller first resolves
//! every execution input into a [`PreparedRunSnapshot`], then consumes the
//! snapshot's generation-bound permit immediately before dispatch.

mod artifact;
mod canonical;
mod permit;
mod snapshot;

pub(in crate::simulation) use artifact::{
    ExecutionArtifactEnvelope, ExecutionArtifactKind, PreparedDependencyBinding,
    ResolvedExecutionDependencies, TransientTrajectoryArtifact,
};
#[cfg(any(target_arch = "wasm32", test))]
pub(in crate::simulation) use canonical::f64_sequence_digest;
pub(in crate::simulation) use canonical::{
    analysis_kind_tag, content_digest, drc_receipt_digest, manual_deck_analysis_instance_id,
    manual_source_receipt_digest, operating_point_effective_source_digest,
    sealed_dependency_closure_digest,
};
pub(in crate::simulation) use permit::{ExecutionPermit, ExecutionPermitIssuer};
pub(in crate::simulation) use snapshot::{
    AuthorizedRunDispatch, AuthorizedTaskDispatch, CrossProbeSnapshot, ExecutionTargetCapabilities,
    ModelSourceIdentity, PreparedRunSet, PreparedRunSnapshot, PreparedTask, ResolvedTaskDispatch,
    RunSourceReceipt, SavePolicy, SnapshotParts, TouchstoneExportPolicy,
};
pub(crate) use snapshot::{
    PreparationError, PreparationStage, PreparedRunMetadata, execution_target_supports_cancellation,
};
