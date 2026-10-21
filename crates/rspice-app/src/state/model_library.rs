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

mod compatibility;
mod manager;
mod source_label;

pub(crate) use compatibility::{
    models_have_compatible_device_family, placement_component_for_model,
    validate_component_model_compatibility,
};
pub use rspice_model_library::ProjectModelDefinition;
pub use rspice_model_library::RetainedClosure;
pub use rspice_model_library::correlation::{
    CorrelationAggregation, CorrelationAlignmentEvidence, CorrelationAlignmentPolicy,
    CorrelationCalculation, CorrelationDatasetClass, CorrelationDatasetRevision,
    CorrelationEvaluation, CorrelationEvidence, CorrelationExtrapolationPolicy,
    CorrelationMetricDefinition, CorrelationMetricDomain, CorrelationMetricOutcome,
    CorrelationObservation, CorrelationOutlierDecision, CorrelationOutlierDisposition,
    CorrelationReleaseRole, CorrelationReviewDecision, CorrelationSimulationProvenance,
    CorrelationSuite, MAX_CORRELATION_ROWS, MAX_CORRELATION_TEXT_BYTES, ModelCorrelationState,
};
pub(crate) use rspice_model_library::device_class::{SUBCIRCUIT_CLASS, card_device};
pub use rspice_model_library::{
    CornerSectionBinding, CornerSectionDomain, ProcessCorner, stated_temperatures,
};
pub use rspice_model_library::{
    CorrelationMatrix, FiniteBounds, FiniteF64, LookupInterpolation, ModelDefinitionMetadata,
    ModelFileIdentity, ModelSectionDefinition, ModelSectionQualification, ParameterDataType,
    ParameterDefinition, ParameterSource, ParameterValue, StatisticalDistribution,
    StatisticalHierarchyScope, TemperatureExtrapolationPolicy, TemperatureLawDefinition,
    TemperatureLawRepresentation,
};
// Metadata fixtures used by application integration tests.
pub(crate) use manager::model_library_source_digest;
pub use manager::{
    ModelLibraryManager, PackModelHit, ProjectModelCommit, SealedModelExecutionSources,
};
pub use rspice_model_library::DeviceModel;
pub use rspice_model_library::ProjectModelRevisionDefinition;
pub(crate) use rspice_model_library::first_unreachable_source;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use rspice_model_library::is_foreign_platform_absolute_path;
#[cfg(test)]
pub(crate) use rspice_model_library::is_portable_absolute_path;
pub use rspice_model_library::qualification::{
    ApprovalDecision, CompatibilityAssessment, CompatibilityDisposition, ConsumerChange,
    ConsumerImpactAssessment, DocumentReference, DocumentationDeclaration, DocumentationSet,
    FiniteValue, LicenseDeclaration, LicenseScope, ModelQualificationState, ModelReleaseCandidate,
    ModelReleaseIdentity, ModelSourceEvidenceBinding, NonNegativeFinite,
    PlatformCompatibilityEvidence, PromotionApproval, PromotionApprovalRole, QualificationAnalysis,
    QualificationErrorCode, QualificationEvidence, QualificationOutputDefinition,
    QualificationPlatform, QualificationPlatformRun, QualificationProbe, QualificationReference,
    QualificationSample, QualificationSuite, QualificationVector, QualificationVectorDisposition,
    QualificationVectorDispositionCause, QualificationVectorOutcome,
    QualificationVectorRequiredAction, ReleaseCandidateIdentity, RequiredDocumentation,
};
#[cfg(test)]
pub use rspice_model_library::qualification::{
    MODEL_QUALIFICATION_SCHEMA_VERSION, PlatformQualificationOutcome,
    QualificationPlatformVectorOutcome,
};
pub(crate) use rspice_model_library::{BinFamily, BindableModel, model_bin_family};
pub use rspice_model_library::{ClosureFacts, closure_facts, envelope_is_invalid};
pub use rspice_model_library::{
    ModelConsumerScope, ModelResolutionRecord, ModelValidationFinding,
    ModelValidationFindingSeverity, SimulationPlanModelBinding,
};
pub use rspice_model_library::{ModelLevel, ModelType};
pub use rspice_model_library::{ModelLibrary, ModelSourceAuthority, PackPartPin};
#[cfg(test)]
pub use rspice_model_library::{
    ModelSourceContent, ModelSourceEdge, ModelSourcePin, ModelSubcircuitInterface,
};
#[cfg(test)]
pub use rspice_model_library::{StatisticalDefinition, StatisticalVariableDefinition};
pub use source_label::short_digest;
