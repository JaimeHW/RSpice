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

pub use catalog::{
    Availability, Mutability, ProductObjectContract, ProductObjectKind, ReadinessStage,
    WorkflowOwner,
};
pub use command::{
    AuthorityDenial, AuthorityDenialKind, CancellationReceipt, CommandId, CommandOutcome,
    ConflictReceipt, ExecutionFailure, FieldId, IdentifierError, RecoveryReceipt,
    TransactionReceipt, ValidationCode, ValidationIssue, ValidationSeverity,
};
pub use identity::{
    AnalysisInstanceId, AutomationPipelineId, ContentDigest, DatasetId, DesignId, DigestError,
    JobId, ModelBindingId, ObjectRef, ObjectRevision, ProjectId, ReleaseCandidateId,
    ResultDocumentId, RevisionError, RunId, RunSetId, SimulationPlanId, TestbenchId, UuidIdError,
    UuidIdParseError, VerificationEvidenceId, VerificationPlanId,
};
pub use lifecycle::{
    DatasetBinding, DatasetManifest, DatasetProvenance, JobEvent, JobState, LifecycleError,
    ReleaseCandidate, ReleaseCandidateState, ReleaseGateSnapshot, ResultDocument,
    ResultDocumentLayout, RunEvent, RunState, VerificationDisposition,
    VerificationDispositionEntry, VerificationEvidence,
};

/// Version of the implemented product-contract vocabulary.
pub const PRODUCT_CONTRACT_SCHEMA_VERSION: &str = "1.2.0";
