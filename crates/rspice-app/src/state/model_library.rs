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
pub(crate) mod compilation;
mod device_class;
mod manager;
mod source_label;

pub(crate) use compatibility::{
    models_have_compatible_device_family, placement_component_for_model,
    validate_component_model_compatibility,
};
pub(crate) use device_class::{SUBCIRCUIT_CLASS, card_device};
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
#[cfg(test)]
pub use rspice_model_library::{StatisticalDefinition, StatisticalVariableDefinition};
// Test-only alias: the class table itself is read only by the test that holds
// the workspace's class chips to it, which cannot live down here because
// `state` may not reference `workbench`.
#[cfg(test)]
pub(crate) use device_class::DEVICE_CLASS;
pub use manager::{
    ModelLibraryManager, PackModelHit, ProjectModelCommit, SealedModelExecutionSources,
};
pub(crate) use manager::{SealedModelLibraryVerilogAAuthority, model_library_source_digest};
pub use rspice_model_library::DeviceModel;
pub use rspice_model_library::ProjectModelRevisionDefinition;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use rspice_model_library::is_foreign_platform_absolute_path;
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
pub use rspice_model_library::{ClosureFacts, closure_facts, envelope_is_invalid};
pub use rspice_model_library::{
    ModelConsumerScope, ModelExecutionPlan, ModelResolutionRecord, ModelValidationFinding,
    ModelValidationFindingSeverity, ModelValidationReceipt, SimulationPlanModelBinding,
};
pub use rspice_model_library::{ModelLevel, ModelType};
pub use rspice_model_library::{
    ModelLibrary, ModelSourceAuthority, ModelSourceContent, ModelSourceEdge, ModelSourcePin,
    ModelSubcircuitInterface, PackPartPin, SEALED_MODEL_SOURCE_MARKER, labelled_pack,
};
pub(crate) use rspice_model_library::{
    first_unreachable_source, is_portable_absolute_path, project_owned_source_path,
    subcircuit_interface_key,
};
pub use source_label::short_digest;
