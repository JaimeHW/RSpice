//! Canonical product contracts shared across application layers.
//!
//! Portable identities and object vocabulary live in `rspice-app-types`;
//! command routing remains owned by the application.

mod command;

pub use command::CommandId;
#[cfg(test)]
pub use rspice_app_types::product::DerivedAnalysisIdentity;
pub use rspice_app_types::product::{
    AnalysisInstanceId, CaptureGroupId, ContentDigest, DatasetBinding, DatasetId, DesignVariableId,
    ModelSourceId, ObjectRef, ObjectRevision, ProcessCorner, ProductObjectKind, ProjectId,
    ResultDocumentId, RunId, SavedOutputId, SimulationCampaignId, SimulationPlanId, TransactionId,
    VerificationEvidenceId, short_identity,
};
