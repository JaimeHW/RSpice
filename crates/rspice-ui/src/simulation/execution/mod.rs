//! Immutable simulation preparation and one-use dispatch authorization.
//!
//! The engine never receives live editor state. A controller first resolves
//! every execution input into a [`PreparedRunSnapshot`], then consumes the
//! snapshot's generation-bound permit immediately before dispatch.

mod canonical;
mod permit;
mod snapshot;

pub(in crate::simulation) use canonical::{
    analysis_config_digest, analysis_instance_id, analysis_kind_tag, content_digest,
    drc_receipt_digest, manual_source_receipt_digest,
};
pub(in crate::simulation) use permit::{ExecutionPermit, ExecutionPermitIssuer};
pub(in crate::simulation) use snapshot::{
    AuthorizedRunDispatch, AuthorizedTaskDispatch, CrossProbeSnapshot, ExecutionTargetCapabilities,
    ModelSourceIdentity, PreparedRunSnapshot, RunSourceReceipt, SavePolicy, SnapshotParts,
    TouchstoneExportPolicy,
};
pub(crate) use snapshot::{
    PreparationError, PreparationStage, PreparedRunMetadata, execution_target_supports_cancellation,
};
