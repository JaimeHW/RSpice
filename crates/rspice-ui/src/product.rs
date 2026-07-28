//! Canonical commercial product contracts.
//!
//! These types are deliberately independent of egui and simulator internals.
//! They are the stable boundary shared by persistence, commands, services,
//! automation, and presentation. UI code may render these contracts but must
//! not invent alternate identities, readiness labels, or lifecycle rules.

mod catalog;
mod command;
mod identity;
mod lifecycle;

pub use catalog::ProductObjectKind;
pub use command::{CommandId, FieldId, ValidationCode};
pub use identity::{
    AnalysisInstanceId, AutomationPipelineId, ContentDigest, DatasetId, DesignId, DesignVariableId,
    DigestError, JobId, ModelBindingId, ModelSourceId, ObjectRef, ObjectRevision, ProjectId,
    ReleaseCandidateId, ResultDocumentId, RevisionError, RunId, SavedOutputId, SimulationPlanId,
    VerificationEvidenceId,
};
pub use lifecycle::DatasetBinding;

/// Version of the implemented product-contract vocabulary.
pub const PRODUCT_CONTRACT_SCHEMA_VERSION: &str = "1.2.0";
