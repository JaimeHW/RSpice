//! Immutable simulation preparation and one-use dispatch authorization.
//!
//! The engine never receives live editor state. A controller first resolves
//! every execution input into a [`PreparedRunSnapshot`], then consumes the
//! snapshot's generation-bound permit immediately before dispatch.

mod canonical;
mod permit;
mod snapshot;

pub(crate) use canonical::manual_deck_analysis_instance_id_from_tag;
pub(in crate::simulation) use canonical::{
    analysis_kind_tag, content_digest, drc_receipt_digest, manual_deck_analysis_instance_id,
    manual_source_receipt_digest,
};
pub(in crate::simulation) use permit::{ExecutionPermit, ExecutionPermitIssuer};
pub(in crate::simulation) use snapshot::{
    AuthorizedRunDispatch, AuthorizedTaskDispatch, CrossProbeSnapshot, ExecutionTargetCapabilities,
    ModelSourceIdentity, PreparedRunSnapshot, PreparedTask, RunSourceReceipt, SavePolicy,
    SnapshotParts, TouchstoneExportPolicy,
};
pub(crate) use snapshot::{
    PreparationError, PreparationStage, PreparedRunMetadata, execution_target_supports_cancellation,
};
