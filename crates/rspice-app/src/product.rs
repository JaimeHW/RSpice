//! Canonical product contracts shared across application layers.
//!
//! Portable identities and object vocabulary live in `rspice-app-types`;
//! command routing remains owned by the application.

mod command;

pub use command::CommandId;
pub use rspice_app_types::product::{
    AnalysisInstanceId, CaptureGroupId, ContentDigest, DatasetBinding, DatasetId,
    DerivedAnalysisIdentity, DesignVariableId, JobId, ModelSourceId, ObjectRef, ObjectRevision,
    ProcessCorner, ProductObjectKind, ProjectId, ResultDocumentId, RevisionError, RunId,
    SavedOutputId, SimulationCampaignId, SimulationPlanId, SpecificationId, TransactionId,
    VerificationEvidenceId, manual_deck_analysis_instance_id_from_tag, short_identity,
};
