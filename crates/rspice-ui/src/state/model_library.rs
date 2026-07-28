//! Model Library Browser
//!
//! PDK model file navigation with corner/process selection.
//!
//! # Architecture
//!
//! Matches Cadence's model library management:
//! - **Model Library**: Collection of device models (e.g., `tsmc180.lib`)
//! - **Section/Corner**: Process corner within library (tt, ff, ss, etc.)
//! - **Model**: Individual device model (nmos, pmos, npn, etc.)

mod authoring;
mod corner;
mod correlation;
mod definition_metadata;
mod library;
mod manager;
mod model;
mod project_revision;
mod qualification;
mod types;

pub use authoring::ProjectModelDefinition;
pub use corner::ProcessCorner;
pub use correlation::{
    CorrelationAggregation, CorrelationAlignmentEvidence, CorrelationAlignmentPolicy,
    CorrelationCalculation, CorrelationDatasetClass, CorrelationDatasetRevision,
    CorrelationEvaluation,
    CorrelationEvidence, CorrelationExtrapolationPolicy, CorrelationMetricDefinition,
    CorrelationMetricDomain, CorrelationMetricOutcome, CorrelationObservation,
    CorrelationOutlierDecision, CorrelationOutlierDisposition, CorrelationReleaseRole,
    CorrelationReviewDecision,
    CorrelationSimulationProvenance, CorrelationSuite, MAX_CORRELATION_ROWS,
    MAX_CORRELATION_TEXT_BYTES, ModelCorrelationState,
};
pub use definition_metadata::{
    CorrelationMatrix, DefinitionMetadataError, FiniteBounds, FiniteF64,
    LookupInterpolation, MODEL_DEFINITION_METADATA_SCHEMA_VERSION, ModelDefinitionMetadata,
    ModelFileIdentity, ModelSectionDefinition,
    ModelSectionQualification, ParameterDataType, ParameterDefinition, ParameterSource,
    ParameterValue, StatisticalDefinition, StatisticalDistribution, StatisticalHierarchyScope,
    StatisticalVariableDefinition,
    TemperatureExtrapolationPolicy, TemperatureLawDefinition, TemperatureLawRepresentation,
};
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use library::is_foreign_platform_absolute_path;
pub use library::{
    ModelLibrary, ModelSourceAuthority, ModelSourceContent, ModelSourceEdge, ModelSourcePin,
};
pub(crate) use library::{
    first_unreachable_source, is_portable_absolute_path, project_owned_source_path,
};
pub use manager::{
    ModelLibraryManager, ProjectModelCommit, SealedModelExecutionSources,
};
pub use model::DeviceModel;
pub use project_revision::{
    ProjectModelRevisionDefinition,
};
pub use qualification::{
    ApprovalDecision, CompatibilityAssessment, CompatibilityDisposition, ConsumerChange,
    ConsumerImpactAssessment, DocumentReference, DocumentationDeclaration, DocumentationSet,
    FiniteValue, LicenseDeclaration, LicenseScope, MODEL_QUALIFICATION_SCHEMA_VERSION,
    ModelQualificationState, ModelReleaseCandidate, ModelReleaseIdentity,
    ModelSourceEvidenceBinding, NonNegativeFinite, PlatformCompatibilityEvidence,
    PlatformQualificationOutcome, PromotionApproval, PromotionApprovalRole, QualificationAnalysis,
    QualificationErrorCode, QualificationEvidence, QualificationExecutionProgress,
    QualificationExecutionSession, QualificationExecutionStep, QualificationOutputDefinition,
    QualificationPlatform, QualificationPlatformRun, QualificationPlatformVectorOutcome,
    QualificationProbe, QualificationReference, QualificationSample, QualificationSuite,
    QualificationVector, QualificationVectorDisposition, QualificationVectorDispositionCause,
    QualificationVectorOutcome, QualificationVectorRequiredAction, ReleaseCandidateIdentity,
    RequiredDocumentation,
};
pub use types::{ModelLevel, ModelType};
