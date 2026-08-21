//! Canonical commercial product contracts.
//!
//! These types are deliberately independent of egui and simulator internals.
//! They are the stable boundary shared by persistence, commands, services,
//! automation, and presentation. UI code may render these contracts but must
//! not invent alternate identities or readiness labels.

mod catalog;
mod command;
mod identity;
mod process_corner;

pub use catalog::ProductObjectKind;
pub use command::CommandId;
pub use identity::{
    AnalysisInstanceId, CaptureGroupId, ContentDigest, DatasetBinding, DatasetId,
    DerivedAnalysisIdentity, DesignVariableId, JobId, ModelSourceId, ObjectRef, ObjectRevision,
    ProjectId, ResultDocumentId, RevisionError, RunId, SavedOutputId, SimulationCampaignId,
    SimulationPlanId, SpecificationId, TransactionId, VerificationEvidenceId,
    manual_deck_analysis_instance_id_from_tag,
};
pub use process_corner::ProcessCorner;
