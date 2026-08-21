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
mod compatibility;
mod corner;
mod corner_expansion;
mod correlation;
mod definition_metadata;
mod facts;
mod library;
mod manager;
mod model;
mod project_revision;
mod qualification;
mod types;

pub use authoring::ProjectModelDefinition;
pub(crate) use compatibility::{
    models_have_compatible_device_family, placement_component_for_model,
    validate_component_model_compatibility,
};
pub use corner::{CornerSectionBinding, CornerSectionDomain, ProcessCorner, stated_temperatures};
pub use corner_expansion::RetainedClosure;
// Test-only aliases: the submodule is private, so this path is the only
// way the tests can name these.
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
pub use facts::{ClosureFacts, closure_facts, envelope_is_invalid, short_digest};
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use library::is_foreign_platform_absolute_path;
pub use library::{
    ModelLibrary, ModelSourceAuthority, ModelSourceContent, ModelSourceEdge, ModelSourcePin,
    ModelSubcircuitInterface, PackPartPin,
};
pub(crate) use library::{
    first_unreachable_source, is_portable_absolute_path, project_owned_source_path,
    subcircuit_interface_key,
};
#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) use manager::normalize_browser_bundle_member_path;
pub use manager::{
    ModelConsumerScope, ModelExecutionPlan, ModelLibraryManager, ModelResolutionRecord,
    ModelValidationFinding, ModelValidationFindingSeverity, ModelValidationReceipt, PackModelHit,
    ProjectModelCommit, SealedModelExecutionSources, SimulationPlanModelBinding,
};
pub(crate) use manager::{SealedModelLibraryVerilogAAuthority, model_library_source_digest};
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
