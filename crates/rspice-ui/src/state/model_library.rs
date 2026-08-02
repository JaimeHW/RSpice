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
pub use corner::{CornerSectionDomain, ProcessCorner};
// Test-only aliases: the submodule is private, so this path is the only
// way the tests can name these.
#[cfg(test)]
pub use corner::CornerSectionBinding;
pub use correlation::{
    CorrelationAggregation, CorrelationAlignmentEvidence, CorrelationAlignmentPolicy,
    CorrelationCalculation, CorrelationDatasetClass, CorrelationDatasetRevision,
    CorrelationEvaluation, CorrelationEvidence, CorrelationExtrapolationPolicy,
    CorrelationMetricDefinition, CorrelationMetricDomain, CorrelationMetricOutcome,
    CorrelationObservation, CorrelationOutlierDecision, CorrelationOutlierDisposition,
    CorrelationReleaseRole, CorrelationReviewDecision, CorrelationSimulationProvenance,
    CorrelationSuite, MAX_CORRELATION_ROWS, MAX_CORRELATION_TEXT_BYTES, ModelCorrelationState,
};
pub use definition_metadata::{
    CorrelationMatrix, DefinitionMetadataError, FiniteBounds, FiniteF64, LookupInterpolation,
    ModelDefinitionMetadata, ModelFileIdentity, ModelSectionDefinition, ModelSectionQualification,
    ParameterDataType, ParameterDefinition, ParameterSource, ParameterValue,
    StatisticalDistribution, StatisticalHierarchyScope, TemperatureExtrapolationPolicy,
    TemperatureLawDefinition, TemperatureLawRepresentation,
};
// Test-only aliases: `definition_metadata` is private, so this path is the
// only way the tests can name these.
#[cfg(test)]
pub use definition_metadata::{
    MODEL_DEFINITION_METADATA_SCHEMA_VERSION, StatisticalDefinition, StatisticalVariableDefinition,
};
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use library::is_foreign_platform_absolute_path;
pub use library::{
    ModelLibrary, ModelSourceAuthority, ModelSourceContent, ModelSourceEdge, ModelSourcePin,
    ModelSubcircuitInterface,
};
pub(crate) use library::{
    first_unreachable_source, is_portable_absolute_path, project_owned_source_path,
    subcircuit_interface_key,
};
pub use manager::{
    ModelLibraryManager, PackModelHit, ProjectModelCommit, SealedModelExecutionSources,
};
pub use model::DeviceModel;
pub use project_revision::ProjectModelRevisionDefinition;
pub use qualification::{
    ApprovalDecision, CompatibilityAssessment, CompatibilityDisposition, ConsumerChange,
    ConsumerImpactAssessment, DocumentReference, DocumentationDeclaration, DocumentationSet,
    FiniteValue, LicenseDeclaration, LicenseScope, ModelQualificationState, ModelReleaseCandidate,
    ModelReleaseIdentity, ModelSourceEvidenceBinding, NonNegativeFinite,
    PlatformCompatibilityEvidence, PromotionApproval, PromotionApprovalRole, QualificationAnalysis,
    QualificationErrorCode, QualificationEvidence, QualificationExecutionProgress,
    QualificationExecutionSession, QualificationExecutionStep, QualificationOutputDefinition,
    QualificationPlatform, QualificationPlatformRun, QualificationProbe, QualificationReference,
    QualificationSample, QualificationSuite, QualificationVector, QualificationVectorDisposition,
    QualificationVectorDispositionCause, QualificationVectorOutcome,
    QualificationVectorRequiredAction, ReleaseCandidateIdentity, RequiredDocumentation,
};
#[cfg(test)]
pub use qualification::{
    MODEL_QUALIFICATION_SCHEMA_VERSION, PlatformQualificationOutcome,
    QualificationPlatformVectorOutcome,
};
pub use types::{ModelLevel, ModelType};
