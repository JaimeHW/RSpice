//! Product object identities shared by persistence, services, and presentation.

mod catalog;
mod identity;
mod process_corner;

pub use catalog::ProductObjectKind;
pub use identity::{
    AnalysisInstanceId, CaptureGroupId, ContentDigest, DatasetBinding, DatasetId,
    DerivedAnalysisIdentity, DesignVariableId, JobId, ModelSourceId, ObjectRef, ObjectRevision,
    ProjectId, ResultDocumentId, RevisionError, RunId, SavedOutputId, SimulationCampaignId,
    SimulationPlanId, SpecificationId, TransactionId, VerificationEvidenceId,
    manual_deck_analysis_instance_id_from_tag, short_identity,
};
pub use process_corner::ProcessCorner;
