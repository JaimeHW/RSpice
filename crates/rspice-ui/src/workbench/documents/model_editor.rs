//! Transactional presentation state for the governed device-model editor.
//!
//! Committed source bytes and model metadata remain owned by the project
//! model-library domain. This module retains only an open candidate, local
//! text-entry buffers, validation evidence, and section/dialog presentation.

mod qualification_run;
mod release_candidate;

pub(crate) use qualification_run::*;
// Crate-private: `release_candidate` exposes only `pub(super)` helpers, which
// the sibling modules reach through `use super::*`.
use release_candidate::*;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::diagnostics::ConsoleMessage;
use crate::product::{ContentDigest, ModelSourceId, ObjectRevision};
use crate::state::model_library::{
    ApprovalDecision, CompatibilityAssessment, CompatibilityDisposition, ConsumerChange,
    ConsumerImpactAssessment, CorrelationMatrix, DocumentReference, DocumentationDeclaration,
    DocumentationSet, FiniteBounds, FiniteF64, FiniteValue, LicenseDeclaration, LicenseScope,
    ModelCorrelationState, ModelDefinitionMetadata, ModelFileIdentity, ModelLibraryManager,
    ModelQualificationState, ModelReleaseCandidate, ModelReleaseIdentity, ModelSectionDefinition,
    ModelSectionQualification, ModelSourceAuthority, ModelSourceEvidenceBinding, ParameterDataType,
    ParameterDefinition, ParameterSource, ParameterValue, PlatformCompatibilityEvidence,
    ProjectModelDefinition, ProjectModelRevisionDefinition, PromotionApproval,
    PromotionApprovalRole, QualificationAnalysis, QualificationErrorCode,
    QualificationExecutionProgress, QualificationExecutionSession, QualificationExecutionStep,
    QualificationOutputDefinition, QualificationPlatform, QualificationProbe,
    QualificationReference, QualificationSample, QualificationSuite, QualificationVector,
    QualificationVectorDisposition, QualificationVectorDispositionCause,
    QualificationVectorRequiredAction, ReleaseCandidateIdentity, RequiredDocumentation,
};
use crate::workbench::RSpiceApp;
use sha2::{Digest as _, Sha256};

use crate::workbench::{RouteTransitionSource, SurfaceId, SurfaceRoute};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ModelEditorSection {
    #[default]
    Parameters,
    Sections,
    Statistics,
    Temperature,
    Tests,
    Release,
}

impl ModelEditorSection {
    pub const ALL: [Self; 6] = [
        Self::Parameters,
        Self::Sections,
        Self::Statistics,
        Self::Temperature,
        Self::Tests,
        Self::Release,
    ];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Parameters => "Parameters",
            Self::Sections => "Sections",
            Self::Statistics => "Statistics",
            Self::Temperature => "Temperature",
            Self::Tests => "Tests",
            Self::Release => "Release",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelParameterKind {
    Numeric,
    String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum QualificationAuthoringAnalysis {
    #[default]
    DcOperatingPoint,
    DcSweep,
    AcSweep,
    Noise,
    Transient,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum QualificationAuthoringProbe {
    #[default]
    NodeVoltage,
    BranchCurrent,
    DcObservable,
    SweepValue,
    AcNodeVoltageMagnitude,
    AcNodeVoltagePhaseDegrees,
    AcNodeVoltageReal,
    AcNodeVoltageImaginary,
    AcBranchCurrentMagnitude,
    AcBranchCurrentPhaseDegrees,
    AcBranchCurrentReal,
    AcBranchCurrentImaginary,
    AcEffectiveCapacitance,
    FrequencyValue,
    NoiseOutputDensity,
    NoiseInputReferredDensity,
    NoiseOutputAmplitude,
    NoiseInputReferredAmplitude,
    TransientNodeVoltage,
    TransientBranchCurrent,
    TimeValue,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum QualificationAuthoringSample {
    #[default]
    OperatingPoint,
    FirstSweepPoint,
    LastSweepPoint,
    SweepPoint,
    FirstFrequencyPoint,
    LastFrequencyPoint,
    FrequencyPoint,
    FirstTimePoint,
    LastTimePoint,
    TimePoint,
}

/// Stable text buffers for one atomic suite/vector authoring transaction.
/// Parsing occurs only when the user commits, so focus and validation cannot
/// resize or replace neighboring form controls.
#[derive(Debug, Clone, Default)]
pub struct QualificationAuthoringDraft {
    pub suite_id: String,
    pub suite_name: String,
    pub vector_id: String,
    pub vector_name: String,
    /// Empty selects the top-level base card; otherwise this is the exact
    /// named project-model section executed by the vector.
    pub model_section: String,
    pub executable_input: String,
    pub analysis: QualificationAuthoringAnalysis,
    pub sweep_source: String,
    pub sweep_start: String,
    pub sweep_stop: String,
    pub sweep_step: String,
    /// Comma- or whitespace-separated, strictly increasing positive hertz.
    pub frequencies: String,
    pub noise_output_node: String,
    pub noise_output_reference: String,
    pub noise_input_source: String,
    pub noise_temperature_kelvin: String,
    pub transient_stop_time: String,
    pub transient_max_step: String,
    pub quantity: String,
    pub probe: QualificationAuthoringProbe,
    pub probe_target: String,
    pub excitation_magnitude: String,
    pub sample: QualificationAuthoringSample,
    pub sample_index: String,
    pub expected: String,
    pub absolute_tolerance: String,
    pub relative_tolerance: String,
    pub error: Option<String>,
}

/// Fixed-buffer authoring state for the governed release-candidate contract.
/// The dialog never mutates persisted qualification state until every entered
/// document, declaration, compatibility record, and independent decision can
/// be constructed and validated as one exact source-bound candidate.
#[derive(Debug, Clone)]
pub struct PromotionCandidateDraft {
    pub candidate_id: String,
    pub candidate_version: String,
    pub suite_id: String,
    pub evidence_id: String,
    pub model_description_id: String,
    pub model_description_digest: String,
    pub parameter_reference_id: String,
    pub parameter_reference_digest: String,
    pub qualification_report_id: String,
    pub qualification_report_digest: String,
    pub license_id: String,
    pub license_expression: String,
    pub license_scope: LicenseScope,
    pub commercial_use_allowed: bool,
    pub redistribution_allowed: bool,
    pub license_reviewed: bool,
    pub license_notice_id: String,
    pub license_notice_digest: String,
    pub consumer_change: ConsumerChange,
    pub consumer_summary: String,
    pub affected_consumer_ids: String,
    pub migration_plan_id: String,
    pub migration_plan_digest: String,
    pub consumer_reviewed: bool,
    pub desktop_compatibility: CompatibilityDisposition,
    pub desktop_evidence_id: String,
    pub desktop_evidence_digest: String,
    pub webassembly_compatibility: CompatibilityDisposition,
    pub webassembly_evidence_id: String,
    pub webassembly_evidence_digest: String,
    pub existing_projects_compatibility: CompatibilityDisposition,
    pub compatibility_reviewed: bool,
    pub model_owner_id: String,
    pub model_owner_decision: ApprovalDecision,
    pub model_owner_decision_revision: String,
    pub qualification_approver_id: String,
    pub qualification_approver_decision: ApprovalDecision,
    pub qualification_approver_decision_revision: String,
    pub error: Option<String>,
}

impl Default for PromotionCandidateDraft {
    fn default() -> Self {
        Self {
            candidate_id: String::new(),
            candidate_version: String::new(),
            suite_id: String::new(),
            evidence_id: String::new(),
            model_description_id: String::new(),
            model_description_digest: String::new(),
            parameter_reference_id: String::new(),
            parameter_reference_digest: String::new(),
            qualification_report_id: String::new(),
            qualification_report_digest: String::new(),
            license_id: String::new(),
            license_expression: String::new(),
            license_scope: LicenseScope::OrganizationInternal,
            commercial_use_allowed: false,
            redistribution_allowed: false,
            license_reviewed: false,
            license_notice_id: String::new(),
            license_notice_digest: String::new(),
            consumer_change: ConsumerChange::NoImpact,
            consumer_summary: String::new(),
            affected_consumer_ids: String::new(),
            migration_plan_id: String::new(),
            migration_plan_digest: String::new(),
            consumer_reviewed: false,
            desktop_compatibility: CompatibilityDisposition::Compatible,
            desktop_evidence_id: String::new(),
            desktop_evidence_digest: String::new(),
            webassembly_compatibility: CompatibilityDisposition::Compatible,
            webassembly_evidence_id: String::new(),
            webassembly_evidence_digest: String::new(),
            existing_projects_compatibility: CompatibilityDisposition::Compatible,
            compatibility_reviewed: false,
            model_owner_id: String::new(),
            model_owner_decision: ApprovalDecision::Approved,
            model_owner_decision_revision: "1".to_owned(),
            qualification_approver_id: String::new(),
            qualification_approver_decision: ApprovalDecision::Approved,
            qualification_approver_decision_revision: "1".to_owned(),
            error: None,
        }
    }
}

impl PromotionCandidateDraft {
    fn from_candidate(candidate: &ModelReleaseCandidate) -> Self {
        let mut value = Self {
            candidate_id: candidate.identity.id.clone(),
            candidate_version: candidate.identity.version.clone(),
            suite_id: candidate.suite_id.clone(),
            evidence_id: candidate.evidence_id.clone(),
            ..Self::default()
        };
        for declaration in &candidate.documentation.declarations {
            let id = declaration.document.id.clone();
            let digest = declaration.document.digest.to_string();
            match declaration.kind {
                RequiredDocumentation::ModelDescription => {
                    value.model_description_id = id;
                    value.model_description_digest = digest;
                }
                RequiredDocumentation::ParameterReference => {
                    value.parameter_reference_id = id;
                    value.parameter_reference_digest = digest;
                }
                RequiredDocumentation::QualificationReport => {
                    value.qualification_report_id = id;
                    value.qualification_report_digest = digest;
                }
            }
        }
        if let Some(license) = &candidate.license {
            value.license_id.clone_from(&license.license_id);
            value.license_expression.clone_from(&license.expression);
            value.license_scope = license.scope;
            value.commercial_use_allowed = license.commercial_use_allowed;
            value.redistribution_allowed = license.redistribution_allowed;
            value.license_reviewed = license.reviewed;
            value.license_notice_id.clone_from(&license.notice.id);
            value.license_notice_digest = license.notice.digest.to_string();
        }
        if let Some(impact) = &candidate.consumer_impact {
            value.consumer_change = impact.change;
            value.consumer_summary.clone_from(&impact.summary);
            value.affected_consumer_ids = impact.affected_consumer_ids.join(", ");
            value.consumer_reviewed = impact.reviewed;
            if let Some(plan) = &impact.migration_plan {
                value.migration_plan_id.clone_from(&plan.id);
                value.migration_plan_digest = plan.digest.to_string();
            }
        }
        if let Some(compatibility) = &candidate.compatibility {
            value.existing_projects_compatibility = compatibility.existing_projects;
            value.compatibility_reviewed = compatibility.reviewed;
            for platform in &compatibility.platforms {
                match platform.platform {
                    QualificationPlatform::Desktop => {
                        value.desktop_compatibility = platform.disposition;
                        value.desktop_evidence_id.clone_from(&platform.evidence.id);
                        value.desktop_evidence_digest = platform.evidence.digest.to_string();
                    }
                    QualificationPlatform::WebAssembly => {
                        value.webassembly_compatibility = platform.disposition;
                        value
                            .webassembly_evidence_id
                            .clone_from(&platform.evidence.id);
                        value.webassembly_evidence_digest = platform.evidence.digest.to_string();
                    }
                }
            }
        }
        for approval in &candidate.approvals {
            match approval.role {
                PromotionApprovalRole::ModelOwner => {
                    value.model_owner_id.clone_from(&approval.approver_id);
                    value.model_owner_decision = approval.decision;
                    value.model_owner_decision_revision =
                        approval.decision_revision.get().to_string();
                }
                PromotionApprovalRole::QualificationApprover => {
                    value
                        .qualification_approver_id
                        .clone_from(&approval.approver_id);
                    value.qualification_approver_decision = approval.decision;
                    value.qualification_approver_decision_revision =
                        approval.decision_revision.get().to_string();
                }
            }
        }
        value
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelParameterDraft {
    pub name: String,
    pub kind: ModelParameterKind,
    pub value: String,
    pub unit: String,
    pub lower_bound: String,
    pub upper_bound: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelFieldDiagnostic {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelValidationEvidence {
    pub source_digest: ContentDigest,
    pub source_revision: ObjectRevision,
    pub project_revision: ObjectRevision,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModelDefinitionDelta {
    pub identity_changed: bool,
    pub description_changed: bool,
    pub added_parameters: Vec<String>,
    pub removed_parameters: Vec<String>,
    pub changed_parameters: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelComparisonDisposition {
    Unchanged,
    Improved,
    Review,
    Blocking,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelReleaseComparisonRow {
    pub item: String,
    pub released: String,
    pub candidate: String,
    pub effect: String,
    pub disposition: ModelComparisonDisposition,
}

#[derive(Debug, Clone)]
pub(crate) struct ResolvedEditableProjectModel {
    pub(crate) source_id: ModelSourceId,
    pub(crate) library_revision: ObjectRevision,
    pub(crate) model_revision: ObjectRevision,
    pub(crate) model_digest: ContentDigest,
    pub(crate) source_path: PathBuf,
    pub(crate) definition: ProjectModelRevisionDefinition,
    pub(crate) qualification: ModelQualificationState,
}

/// Resolve one selected model from an authenticated project-owned source
/// closure. A library may contain many models and include members, but the
/// selected canonical revision must occur exactly once in the source file
/// recorded by that model's projection. This is the shared fail-closed gate
/// used by command availability, editor open, and stale-candidate validation.
pub(crate) fn resolve_project_model_for_editor(
    manager: &ModelLibraryManager,
    library_name: &str,
    model_name: &str,
) -> Result<ResolvedEditableProjectModel, String> {
    let library = manager
        .get_library(library_name)
        .ok_or_else(|| format!("Model library '{library_name}' does not exist"))?;
    let ModelSourceAuthority::ProjectOwned {
        source_id,
        revision: library_revision,
        digest: root_digest,
    } = library.source_authority
    else {
        return Err(format!(
            "Model '{model_name}' is not project-owned; create an editable project copy before opening the model editor"
        ));
    };
    let root_path = library.root_path.as_ref().ok_or_else(|| {
        format!("Project-owned model library '{library_name}' has no retained root identity")
    })?;
    if library.source_closure.is_empty()
        || library.source_closure.len() != library.source_contents.len()
    {
        return Err(format!(
            "Project-owned model library '{library_name}' has an incomplete retained source closure"
        ));
    }

    let mut pins = BTreeMap::new();
    for pin in &library.source_closure {
        if pins.insert(pin.path.clone(), pin.digest).is_some() {
            return Err(format!(
                "Project-owned model library '{library_name}' repeats retained source '{}'",
                pin.path.display()
            ));
        }
    }
    let mut contents = BTreeMap::new();
    for content in &library.source_contents {
        if contents
            .insert(content.path.clone(), &content.bytes)
            .is_some()
        {
            return Err(format!(
                "Project-owned model library '{library_name}' repeats retained bytes for '{}'",
                content.path.display()
            ));
        }
    }
    if pins.keys().ne(contents.keys()) {
        return Err(format!(
            "Project-owned model library '{library_name}' retained pins and bytes do not describe the same closure"
        ));
    }
    for (path, expected) in &pins {
        let bytes = contents
            .get(path)
            .expect("pin/content key equality was checked above");
        let actual = ContentDigest::from_bytes(Sha256::digest(bytes).into());
        if actual != *expected {
            return Err(format!(
                "Project-owned model source '{}' fails its retained content digest",
                path.display()
            ));
        }
    }
    if pins.get(root_path) != Some(&root_digest) {
        return Err(format!(
            "Project-owned model library '{library_name}' root digest is inconsistent with its revision authority"
        ));
    }
    if library
        .source_edges
        .iter()
        .any(|edge| !pins.contains_key(&edge.owner) || !pins.contains_key(&edge.target))
        || crate::state::model_library::first_unreachable_source(
            root_path,
            &library.source_closure,
            &library.source_edges,
        )
        .is_some()
    {
        return Err(format!(
            "Project-owned model library '{library_name}' has an invalid retained include graph"
        ));
    }

    let model = library.models.get(model_name).ok_or_else(|| {
        format!("Model '{model_name}' does not exist in library '{library_name}'")
    })?;
    let source_path = model
        .file_path
        .clone()
        .ok_or_else(|| format!("Model '{model_name}' has no retained source-file projection"))?;
    let source_bytes = contents.get(&source_path).ok_or_else(|| {
        format!(
            "Model '{model_name}' points outside the retained source closure at '{}'",
            source_path.display()
        )
    })?;
    let metadata = library
        .model_definition_metadata
        .get(model_name)
        .cloned()
        .ok_or_else(|| {
            format!("Model '{model_name}' has no typed project-owned definition metadata")
        })?;
    let definition = ProjectModelRevisionDefinition::new(
        ProjectModelDefinition::from_device_model(model),
        metadata,
    );
    let canonical = definition
        .canonical_source()
        .map_err(|error| format!("Retained model revision is invalid: {error}"))?;
    let occurrences = source_bytes
        .windows(canonical.len())
        .filter(|candidate| *candidate == canonical.as_bytes())
        .count();
    if occurrences != 1 {
        return Err(format!(
            "Model '{model_name}' canonical revision must occur exactly once in retained source '{}' (found {occurrences})",
            source_path.display()
        ));
    }
    let model_digest = ContentDigest::from_bytes(Sha256::digest(canonical.as_bytes()).into());
    let source_identity = definition
        .project_source_identity()
        .map_err(|error| format!("Project model source identity is invalid: {error}"))?;
    if let Some(identity) = source_identity.as_ref()
        && (identity.source_id != source_id || identity.content_digest != model_digest)
    {
        return Err(format!(
            "Model '{model_name}' metadata is bound to a different retained source revision"
        ));
    }
    let model_revision = source_identity.map_or(library_revision, |identity| identity.revision);

    Ok(ResolvedEditableProjectModel {
        source_id,
        library_revision,
        model_revision,
        model_digest,
        source_path,
        definition,
        qualification: library
            .model_qualification
            .get(model_name)
            .cloned()
            .unwrap_or_default(),
    })
}

#[derive(Debug, Clone)]
pub struct ModelEditorDraft {
    pub library_name: String,
    pub model_name: String,
    pub source_id: ModelSourceId,
    /// Revision of the entire retained library closure used only for
    /// optimistic concurrency and stale-library detection.
    pub base_library_revision: ObjectRevision,
    /// Independent revision of this model's canonical fragment. Qualification
    /// suites, evidence, and releases bind to this revision.
    pub base_source_revision: ObjectRevision,
    pub base_source_digest: ContentDigest,
    /// Exact retained closure member containing this model's canonical card.
    /// Library authority remains rooted at `source_id`/`base_source_revision`;
    /// this path prevents a multi-source edit from drifting into another file.
    pub source_path: PathBuf,
    pub base_project_revision: ObjectRevision,
    pub name: String,
    pub spice_type: String,
    pub description: String,
    pub parameters: Vec<ModelParameterDraft>,
    pub metadata: ModelDefinitionMetadata,
    pub qualification: ModelQualificationState,
    base_definition: ProjectModelRevisionDefinition,
    base_qualification: ModelQualificationState,
}

impl ModelEditorDraft {
    fn open(
        manager: &ModelLibraryManager,
        library_name: &str,
        model_name: &str,
        project_revision: ObjectRevision,
    ) -> Result<Self, String> {
        let resolved = resolve_project_model_for_editor(manager, library_name, model_name)?;
        let source_id = resolved.source_id;
        let library_revision = resolved.library_revision;
        let model_revision = resolved.model_revision;
        let digest = resolved.model_digest;
        let source_path = resolved.source_path;
        let metadata = resolved.definition.metadata.clone();
        let qualification = resolved.qualification;
        let base_definition = resolved.definition;
        let mut parameters = metadata
            .parameters
            .iter()
            .map(|parameter| ModelParameterDraft {
                name: parameter.name.clone(),
                kind: match parameter.data_type {
                    ParameterDataType::Numeric => ModelParameterKind::Numeric,
                    ParameterDataType::String => ModelParameterKind::String,
                },
                value: match &parameter.value {
                    ParameterValue::Numeric(value) => value.to_string(),
                    ParameterValue::String(value) => value.clone(),
                },
                unit: parameter.unit.clone().unwrap_or_default(),
                lower_bound: parameter
                    .bounds
                    .and_then(|bounds| bounds.lower)
                    .map_or_else(String::new, |value| value.to_string()),
                upper_bound: parameter
                    .bounds
                    .and_then(|bounds| bounds.upper)
                    .map_or_else(String::new, |value| value.to_string()),
                description: parameter.description.clone(),
            })
            .collect::<Vec<_>>();
        parameters.sort_by(|left, right| {
            left.name
                .to_ascii_lowercase()
                .cmp(&right.name.to_ascii_lowercase())
                .then_with(|| left.name.cmp(&right.name))
        });
        Ok(Self {
            library_name: library_name.to_owned(),
            model_name: model_name.to_owned(),
            source_id,
            base_library_revision: library_revision,
            base_source_revision: model_revision,
            base_source_digest: digest,
            source_path,
            base_project_revision: project_revision,
            name: base_definition.base.name.clone(),
            spice_type: base_definition.base.spice_type.clone(),
            description: base_definition.base.description.clone(),
            parameters,
            metadata,
            qualification: qualification.clone(),
            base_definition,
            base_qualification: qualification,
        })
    }

    pub fn definition(&self) -> Result<ProjectModelRevisionDefinition, Vec<ModelFieldDiagnostic>> {
        let mut numeric_parameters = BTreeMap::new();
        let mut string_parameters = BTreeMap::new();
        let mut typed_parameters = Vec::with_capacity(self.parameters.len());
        let mut diagnostics = Vec::new();
        let mut canonical_names = BTreeSet::new();

        for (index, parameter) in self.parameters.iter().enumerate() {
            let field = format!("parameters[{index}]");
            let canonical_name = parameter.name.to_ascii_lowercase();
            if !canonical_names.insert(canonical_name) {
                diagnostics.push(ModelFieldDiagnostic {
                    field: format!("{field}.name"),
                    message: format!(
                        "Parameter '{}' duplicates another case-insensitive name",
                        parameter.name
                    ),
                });
                continue;
            }
            match parameter.kind {
                ModelParameterKind::Numeric => match parameter.value.trim().parse::<f64>() {
                    Ok(value) if value.is_finite() => {
                        numeric_parameters.insert(parameter.name.clone(), value);
                        typed_parameters.push(self.typed_parameter(
                            index,
                            parameter,
                            ParameterDataType::Numeric,
                            ParameterValue::Numeric(
                                FiniteF64::new(value).expect("finite value was checked above"),
                            ),
                            &mut diagnostics,
                        ));
                    }
                    Ok(_) => diagnostics.push(ModelFieldDiagnostic {
                        field: format!("{field}.value"),
                        message: "Numeric parameter must be finite".to_owned(),
                    }),
                    Err(error) => diagnostics.push(ModelFieldDiagnostic {
                        field: format!("{field}.value"),
                        message: format!("Invalid numeric value: {error}"),
                    }),
                },
                ModelParameterKind::String => {
                    string_parameters.insert(parameter.name.clone(), parameter.value.clone());
                    typed_parameters.push(self.typed_parameter(
                        index,
                        parameter,
                        ParameterDataType::String,
                        ParameterValue::String(parameter.value.clone()),
                        &mut diagnostics,
                    ));
                }
            }
        }

        let base = ProjectModelDefinition {
            name: self.name.clone(),
            spice_type: self.spice_type.clone(),
            description: self.description.clone(),
            numeric_parameters,
            string_parameters,
        };
        if let Err(message) = base.validate() {
            diagnostics.push(ModelFieldDiagnostic {
                field: "definition".to_owned(),
                message,
            });
        }
        typed_parameters.sort_by(|left, right| {
            left.name
                .to_ascii_lowercase()
                .cmp(&right.name.to_ascii_lowercase())
                .then_with(|| left.name.cmp(&right.name))
        });
        let mut metadata = self.metadata.clone();
        synchronize_parameter_overrides(&mut metadata, &typed_parameters);
        metadata.parameters = typed_parameters;
        let definition = ProjectModelRevisionDefinition::new(base, metadata);
        if diagnostics.is_empty()
            && let Err(error) = definition.expected_source_digest()
        {
            diagnostics.push(ModelFieldDiagnostic {
                field: "definition".to_owned(),
                message: error.to_string(),
            });
        }
        if diagnostics.is_empty() {
            Ok(definition)
        } else {
            Err(diagnostics)
        }
    }

    fn typed_parameter(
        &self,
        index: usize,
        draft: &ModelParameterDraft,
        data_type: ParameterDataType,
        value: ParameterValue,
        diagnostics: &mut Vec<ModelFieldDiagnostic>,
    ) -> ParameterDefinition {
        let name = draft.name.as_str();
        let existing = self
            .metadata
            .parameters
            .iter()
            .find(|parameter| parameter.name.eq_ignore_ascii_case(name));
        let bounds = match data_type {
            ParameterDataType::String => {
                if !draft.lower_bound.is_empty() || !draft.upper_bound.is_empty() {
                    diagnostics.push(ModelFieldDiagnostic {
                        field: format!("parameters[{index}].bounds"),
                        message: "String parameters cannot declare numeric bounds".to_owned(),
                    });
                }
                None
            }
            ParameterDataType::Numeric => {
                let lower = parse_optional_finite_bound(
                    &draft.lower_bound,
                    format!("parameters[{index}].lower_bound"),
                    diagnostics,
                );
                let upper = parse_optional_finite_bound(
                    &draft.upper_bound,
                    format!("parameters[{index}].upper_bound"),
                    diagnostics,
                );
                if lower.is_some() || upper.is_some() {
                    Some(FiniteBounds { lower, upper })
                } else {
                    None
                }
            }
        };
        if let Some(existing) = existing
            && matches!(&existing.source, ParameterSource::Inherited { .. })
            && existing.value != value
        {
            diagnostics.push(ModelFieldDiagnostic {
                field: format!("parameters[{index}].value"),
                message: format!(
                    "Inherited parameter '{name}' cannot be edited without an explicit target section"
                ),
            });
        }
        ParameterDefinition {
            name: name.to_owned(),
            data_type,
            value,
            unit: (!draft.unit.is_empty()).then(|| draft.unit.clone()),
            bounds,
            source: existing.map_or_else(
                || ParameterSource::Declared {
                    source: "project model source".to_owned(),
                },
                |parameter| parameter.source.clone(),
            ),
            description: draft.description.clone(),
        }
    }

    #[must_use]
    pub fn definition_is_dirty(&self) -> bool {
        self.definition()
            .map_or(true, |candidate| candidate != self.base_definition)
    }

    #[must_use]
    pub fn qualification_is_dirty(&self) -> bool {
        self.qualification != self.base_qualification
    }

    #[must_use]
    pub fn is_dirty(&self) -> bool {
        self.definition_is_dirty() || self.qualification_is_dirty()
    }

    pub fn validate(
        &self,
        manager: &ModelLibraryManager,
        project_revision: ObjectRevision,
    ) -> Result<ModelValidationEvidence, Vec<ModelFieldDiagnostic>> {
        let definition = self.definition()?;
        let retained =
            resolve_project_model_for_editor(manager, &self.library_name, &self.model_name);
        let source_matches = retained.is_ok_and(|resolved| {
            resolved.source_id == self.source_id
                && resolved.library_revision == self.base_library_revision
                && resolved.model_revision == self.base_source_revision
                && resolved.model_digest == self.base_source_digest
                && resolved.source_path == self.source_path
        });
        if !source_matches || project_revision != self.base_project_revision {
            return Err(vec![ModelFieldDiagnostic {
                field: "source".to_owned(),
                message: "The project or model source changed after this candidate was opened; reload or compare before saving"
                    .to_owned(),
            }]);
        }
        let source_digest = definition.expected_source_digest().map_err(|error| {
            vec![ModelFieldDiagnostic {
                field: "definition".to_owned(),
                message: error.to_string(),
            }]
        })?;
        self.qualification
            .validate_for_model(&definition.base.name)
            .map_err(|error| {
                vec![ModelFieldDiagnostic {
                    field: error.path,
                    message: error.message,
                }]
            })?;
        Ok(ModelValidationEvidence {
            source_digest,
            source_revision: self.base_source_revision,
            project_revision,
        })
    }

    pub fn delta(&self) -> Result<ModelDefinitionDelta, Vec<ModelFieldDiagnostic>> {
        let candidate = self.definition()?;
        let base_parameters = parameter_definitions(&self.base_definition.metadata);
        let candidate_parameters = parameter_definitions(&candidate.metadata);
        let added_parameters = candidate_parameters
            .keys()
            .filter(|name| !base_parameters.contains_key(*name))
            .cloned()
            .collect();
        let removed_parameters = base_parameters
            .keys()
            .filter(|name| !candidate_parameters.contains_key(*name))
            .cloned()
            .collect();
        let changed_parameters = candidate_parameters
            .iter()
            .filter_map(|(name, parameter)| {
                base_parameters
                    .get(name)
                    .is_some_and(|base_parameter| *base_parameter != *parameter)
                    .then_some(name.clone())
            })
            .collect();
        let base = &self.base_definition.base;
        let candidate = &candidate.base;
        Ok(ModelDefinitionDelta {
            identity_changed: candidate.name != base.name
                || candidate.spice_type != base.spice_type,
            description_changed: candidate.description != base.description,
            added_parameters,
            removed_parameters,
            changed_parameters,
        })
    }

    /// Compare the exact open definition with one immutable promoted release.
    /// The release must retain canonical source bytes whose digest matches its
    /// source identity; a legacy identity-only record fails closed rather than
    /// fabricating parameter values.
    pub fn compare_release(
        &self,
        release_id: &str,
    ) -> Result<Vec<ModelReleaseComparisonRow>, String> {
        if self.definition_is_dirty() {
            return Err("Save the model definition before comparing it with a release".to_owned());
        }
        self.qualification
            .validate_for_model(&self.model_name)
            .map_err(|error| error.to_string())?;
        let release = self
            .qualification
            .releases
            .iter()
            .find(|release| release.identity.id.eq_ignore_ascii_case(release_id))
            .ok_or_else(|| format!("Model release '{release_id}' does not exist"))?;
        if release.definition_source.is_empty() {
            return Err(format!(
                "Release '{}' predates retained definition snapshots and cannot support an exact numerical comparison",
                release.identity.id
            ));
        }
        let released_metadata = release.definition_metadata.as_ref().ok_or_else(|| {
            format!(
                "Release '{}' predates retained typed definition metadata and cannot support an exact semantic comparison",
                release.identity.id
            )
        })?;
        let source = rspice_core::netlist::decode_source_bytes(&release.definition_source)
            .map_err(|error| format!("Released definition source cannot be decoded: {error}"))?;
        let mut parser = rspice_core::library::LibParser::new(Path::new("."));
        let parsed = parser.parse_string(&source);
        if !parsed.is_ok() || parsed.top_level_models.len() != 1 {
            return Err(format!(
                "Released definition source is not one valid top-level model: {}",
                parsed
                    .errors
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("; ")
            ));
        }
        let released = &parsed.top_level_models[0];
        let candidate = self.definition().map_err(|diagnostics| {
            diagnostics
                .into_iter()
                .map(|diagnostic| format!("{}: {}", diagnostic.field, diagnostic.message))
                .collect::<Vec<_>>()
                .join("; ")
        })?;

        let release_source_matches = release
            .source
            .model_id
            .eq_ignore_ascii_case(&self.model_name)
            && release.source.source_id == Some(self.source_id)
            && release.source.source_revision == self.base_source_revision
            && release.source.source_digest == self.base_source_digest;
        let mut rows = vec![ModelReleaseComparisonRow {
            item: "Source revision".to_owned(),
            released: format!(
                "r{} · {}",
                release.source.source_revision.get(),
                release.source.source_digest
            ),
            candidate: format!(
                "r{} · {}",
                self.base_source_revision.get(),
                self.base_source_digest
            ),
            effect: if release_source_matches {
                "exact source match".to_owned()
            } else {
                "source revision changed".to_owned()
            },
            disposition: if release_source_matches {
                ModelComparisonDisposition::Unchanged
            } else {
                ModelComparisonDisposition::Review
            },
        }];
        for (item, before, after) in [
            (
                "Model identity",
                released.name.clone(),
                candidate.base.name.clone(),
            ),
            (
                "SPICE model type",
                format!("{:?}", released.model_type).to_ascii_uppercase(),
                candidate.base.spice_type.to_ascii_uppercase(),
            ),
            (
                "Description",
                released.description.clone().unwrap_or_default(),
                candidate.base.description.clone(),
            ),
        ] {
            let unchanged = before == after;
            rows.push(ModelReleaseComparisonRow {
                item: item.to_owned(),
                released: before,
                candidate: after,
                effect: if unchanged {
                    "unchanged".to_owned()
                } else {
                    "definition changed".to_owned()
                },
                disposition: if unchanged {
                    ModelComparisonDisposition::Unchanged
                } else {
                    ModelComparisonDisposition::Review
                },
            });
        }

        let mut released_parameters = BTreeMap::<String, (String, String, Option<f64>)>::new();
        for (name, value) in &released.parameters {
            released_parameters.insert(
                name.to_ascii_lowercase(),
                (name.clone(), value.to_string(), Some(*value)),
            );
        }
        for (name, value) in &released.string_params {
            released_parameters.insert(
                name.to_ascii_lowercase(),
                (name.clone(), value.clone(), None),
            );
        }
        let mut candidate_parameters = BTreeMap::<String, (String, String, Option<f64>)>::new();
        for (name, value) in &candidate.base.numeric_parameters {
            candidate_parameters.insert(
                name.to_ascii_lowercase(),
                (name.clone(), value.to_string(), Some(*value)),
            );
        }
        for (name, value) in &candidate.base.string_parameters {
            candidate_parameters.insert(
                name.to_ascii_lowercase(),
                (name.clone(), value.clone(), None),
            );
        }
        let names = released_parameters
            .keys()
            .chain(candidate_parameters.keys())
            .cloned()
            .collect::<BTreeSet<_>>();
        for key in names {
            let before = released_parameters.get(&key);
            let after = candidate_parameters.get(&key);
            let item = after
                .or(before)
                .map_or_else(|| key.clone(), |value| value.0.clone());
            let (released_value, candidate_value, effect, disposition) = match (before, after) {
                (Some(before), Some(after)) if before.1 == after.1 => (
                    before.1.clone(),
                    after.1.clone(),
                    "unchanged".to_owned(),
                    ModelComparisonDisposition::Unchanged,
                ),
                (Some(before), Some(after)) => {
                    let effect = match (before.2, after.2) {
                        (Some(before), Some(after)) if before != 0.0 => {
                            format!("{:+.6}%", (after - before) / before.abs() * 100.0)
                        }
                        (Some(before), Some(after)) => format!("{:+.6e}", after - before),
                        _ => "value changed".to_owned(),
                    };
                    (
                        before.1.clone(),
                        after.1.clone(),
                        effect,
                        ModelComparisonDisposition::Review,
                    )
                }
                (None, Some(after)) => (
                    "—".to_owned(),
                    after.1.clone(),
                    "parameter added".to_owned(),
                    ModelComparisonDisposition::Review,
                ),
                (Some(before), None) => (
                    before.1.clone(),
                    "—".to_owned(),
                    "parameter removed".to_owned(),
                    ModelComparisonDisposition::Review,
                ),
                (None, None) => continue,
            };
            rows.push(ModelReleaseComparisonRow {
                item,
                released: released_value,
                candidate: candidate_value,
                effect,
                disposition,
            });
        }

        append_typed_metadata_comparison_rows(&mut rows, released_metadata, &candidate.metadata)?;

        let released_evidence = self
            .qualification
            .evidence
            .iter()
            .find(|evidence| evidence.id.eq_ignore_ascii_case(&release.evidence_id));
        let current_evidence = self
            .qualification
            .candidates
            .iter()
            .filter(|candidate| {
                candidate
                    .source
                    .model_id
                    .eq_ignore_ascii_case(&self.model_name)
                    && candidate.source.source_id == Some(self.source_id)
                    && candidate.source.source_digest == self.base_source_digest
                    && candidate.source.source_revision == self.base_source_revision
            })
            .filter_map(|candidate| {
                self.qualification.evidence.iter().find(|evidence| {
                    evidence.id.eq_ignore_ascii_case(&candidate.evidence_id)
                        && evidence
                            .source
                            .model_id
                            .eq_ignore_ascii_case(&self.model_name)
                        && evidence.source.source_id == Some(self.source_id)
                        && evidence.source.source_digest == self.base_source_digest
                        && evidence.source.source_revision == self.base_source_revision
                })
            })
            .collect::<Vec<_>>();
        if let (Some(released_evidence), [current_evidence]) =
            (released_evidence, current_evidence.as_slice())
        {
            let released_summary = qualification_evidence_summary(released_evidence);
            let current_summary = qualification_evidence_summary(current_evidence);
            let disposition = match (released_evidence.passed, current_evidence.passed) {
                (false, true) => ModelComparisonDisposition::Improved,
                (true, false) => ModelComparisonDisposition::Blocking,
                _ if released_summary == current_summary => ModelComparisonDisposition::Unchanged,
                _ => ModelComparisonDisposition::Review,
            };
            rows.push(ModelReleaseComparisonRow {
                item: "Runtime qualification".to_owned(),
                released: released_summary.clone(),
                candidate: current_summary.clone(),
                effect: if released_summary == current_summary {
                    "unchanged".to_owned()
                } else {
                    "qualification disposition changed".to_owned()
                },
                disposition,
            });
            append_numerical_evidence_rows(&mut rows, released_evidence, current_evidence);
        }
        Ok(rows)
    }
}

/// Runtime-only state for one cooperative qualification pass. A completed
/// suite is committed to the draft only after its final vector succeeds or
/// fails with a complete, validated platform record. Partial vectors never
/// escape this session.
#[derive(Debug, Clone)]
pub struct ModelQualificationExecution {
    suite_ids: Vec<String>,
    suite_index: usize,
    source: ModelSourceEvidenceBinding,
    session: QualificationExecutionSession,
    pub progress: QualificationExecutionProgress,
    pub assembled_evidence: usize,
}

impl ModelQualificationExecution {
    #[must_use]
    pub fn current_suite_id(&self) -> &str {
        &self.suite_ids[self.suite_index]
    }

    #[must_use]
    pub const fn completed_suites(&self) -> usize {
        self.suite_index
    }

    #[must_use]
    pub fn total_suites(&self) -> usize {
        self.suite_ids.len()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ModelEditorState {
    pub active_section: ModelEditorSection,
    pub selected_parameter: Option<usize>,
    pub draft: Option<ModelEditorDraft>,
    pub diagnostics: Vec<ModelFieldDiagnostic>,
    pub validation: Option<ModelValidationEvidence>,
    pub comparison_open: bool,
    pub comparison_release_id: String,
    pub parameter_schema_open: bool,
    pub parameter_schema_section: String,
    pub parameter_schema_parameter: String,
    pub parameter_schema_override_value: String,
    pub parameter_schema_override_error: Option<String>,
    pub new_section_open: bool,
    pub new_section_name: String,
    pub new_section_parent: String,
    pub new_section_error: Option<String>,
    pub correlation_matrix_open: bool,
    pub correlation_matrix_candidates: Vec<CorrelationMatrix>,
    pub correlation_matrix_edits: Vec<Vec<Vec<String>>>,
    pub correlation_matrix_error: Option<String>,
    pub temperature_preview_open: bool,
    pub qualification_plan_open: bool,
    pub qualification_authoring_open: bool,
    pub qualification_authoring: QualificationAuthoringDraft,
    pub promotion_review_open: bool,
    pub promotion_candidate: PromotionCandidateDraft,
    pub promotion_record_id: String,
    pub promotion_release_id: String,
    pub promotion_release_version: String,
    pub promotion_error: Option<String>,
    pub qualification_execution: Option<ModelQualificationExecution>,
    pub qualification_execution_notice: Option<String>,
}

impl ModelEditorState {
    pub fn open(
        &mut self,
        manager: &ModelLibraryManager,
        library_name: &str,
        model_name: &str,
        project_revision: ObjectRevision,
    ) -> Result<(), String> {
        let draft = ModelEditorDraft::open(manager, library_name, model_name, project_revision)?;
        *self = Self {
            draft: Some(draft),
            ..Self::default()
        };
        Ok(())
    }

    pub fn invalidate_candidate_evidence(&mut self) {
        self.validation = None;
        self.diagnostics.clear();
    }

    pub fn begin_parameter_schema(&mut self) {
        self.parameter_schema_open = true;
        self.parameter_schema_override_error = None;
        self.refresh_parameter_schema_override_editor();
    }

    pub fn refresh_parameter_schema_override_editor(&mut self) {
        let Some(draft) = self.draft.as_ref() else {
            self.parameter_schema_section.clear();
            self.parameter_schema_parameter.clear();
            self.parameter_schema_override_value.clear();
            return;
        };
        if !draft.metadata.sections.iter().any(|section| {
            section
                .name
                .eq_ignore_ascii_case(&self.parameter_schema_section)
        }) {
            self.parameter_schema_section = draft
                .metadata
                .sections
                .first()
                .map_or_else(String::new, |section| section.name.clone());
        }
        if !draft.metadata.parameters.iter().any(|parameter| {
            parameter
                .name
                .eq_ignore_ascii_case(&self.parameter_schema_parameter)
        }) {
            self.parameter_schema_parameter = draft
                .metadata
                .parameters
                .first()
                .map_or_else(String::new, |parameter| parameter.name.clone());
        }
        self.parameter_schema_override_value = draft
            .metadata
            .sections
            .iter()
            .find(|section| {
                section
                    .name
                    .eq_ignore_ascii_case(&self.parameter_schema_section)
            })
            .and_then(|section| {
                section
                    .overrides
                    .iter()
                    .find(|(name, _)| name.eq_ignore_ascii_case(&self.parameter_schema_parameter))
                    .map(|(_, value)| value)
            })
            .or_else(|| {
                draft
                    .metadata
                    .parameters
                    .iter()
                    .find(|parameter| {
                        parameter
                            .name
                            .eq_ignore_ascii_case(&self.parameter_schema_parameter)
                    })
                    .map(|parameter| &parameter.value)
            })
            .map_or_else(String::new, parameter_value_text);
        self.parameter_schema_override_error = None;
    }

    pub fn parameter_schema_override_exists(&self) -> bool {
        let Some(draft) = self.draft.as_ref() else {
            return false;
        };
        draft.metadata.sections.iter().any(|section| {
            section
                .name
                .eq_ignore_ascii_case(&self.parameter_schema_section)
                && section
                    .overrides
                    .keys()
                    .any(|name| name.eq_ignore_ascii_case(&self.parameter_schema_parameter))
        })
    }

    pub fn commit_parameter_schema_override(&mut self) -> bool {
        let Some(draft) = self.draft.as_mut() else {
            self.parameter_schema_override_error =
                Some("No project-owned model candidate is open".to_owned());
            return false;
        };
        let mut metadata = draft.metadata.clone();
        let Some(parameter) = metadata.parameters.iter().find(|parameter| {
            parameter
                .name
                .eq_ignore_ascii_case(&self.parameter_schema_parameter)
        }) else {
            self.parameter_schema_override_error =
                Some("Select a retained model parameter".to_owned());
            return false;
        };
        let parameter_name = parameter.name.clone();
        let value = match parameter.data_type {
            ParameterDataType::Numeric => {
                let parsed = match self.parameter_schema_override_value.trim().parse::<f64>() {
                    Ok(value) => value,
                    Err(error) => {
                        self.parameter_schema_override_error =
                            Some(format!("Override value must be a finite number: {error}"));
                        return false;
                    }
                };
                let Ok(value) = FiniteF64::new(parsed) else {
                    self.parameter_schema_override_error =
                        Some("Override value must be finite".to_owned());
                    return false;
                };
                ParameterValue::Numeric(value)
            }
            ParameterDataType::String => {
                ParameterValue::String(self.parameter_schema_override_value.clone())
            }
        };
        let Some(section) = metadata.sections.iter_mut().find(|section| {
            section
                .name
                .eq_ignore_ascii_case(&self.parameter_schema_section)
        }) else {
            self.parameter_schema_override_error =
                Some("Select a retained process section".to_owned());
            return false;
        };
        let key = section
            .overrides
            .keys()
            .find(|name| name.eq_ignore_ascii_case(&parameter_name))
            .cloned()
            .unwrap_or(parameter_name);
        section.overrides.insert(key, value);
        if let Err(error) = metadata.validate() {
            self.parameter_schema_override_error = Some(error.to_string());
            return false;
        }
        draft.metadata = metadata;
        self.invalidate_candidate_evidence();
        self.parameter_schema_override_error = None;
        true
    }

    pub fn remove_parameter_schema_override(&mut self) -> bool {
        let Some(draft) = self.draft.as_mut() else {
            self.parameter_schema_override_error =
                Some("No project-owned model candidate is open".to_owned());
            return false;
        };
        let mut metadata = draft.metadata.clone();
        let Some(section) = metadata.sections.iter_mut().find(|section| {
            section
                .name
                .eq_ignore_ascii_case(&self.parameter_schema_section)
        }) else {
            self.parameter_schema_override_error =
                Some("Select a retained process section".to_owned());
            return false;
        };
        let Some(key) = section
            .overrides
            .keys()
            .find(|name| name.eq_ignore_ascii_case(&self.parameter_schema_parameter))
            .cloned()
        else {
            self.parameter_schema_override_error =
                Some("The selected parameter has no override in this section".to_owned());
            return false;
        };
        section.overrides.remove(&key);
        if let Err(error) = metadata.validate() {
            self.parameter_schema_override_error = Some(error.to_string());
            return false;
        }
        draft.metadata = metadata;
        self.invalidate_candidate_evidence();
        self.refresh_parameter_schema_override_editor();
        true
    }

    /// Begin a fresh section transaction without retaining text from a prior
    /// cancelled or committed dialog.
    pub fn begin_new_section(&mut self) {
        self.new_section_name.clear();
        self.new_section_parent.clear();
        self.new_section_error = None;
        self.new_section_open = true;
    }

    /// Add one process section to the open candidate. The candidate is rolled
    /// back exactly if the new node creates an invalid identity, parent, or
    /// inheritance graph.
    pub fn commit_new_section(&mut self) -> bool {
        let Some(draft) = self.draft.as_mut() else {
            self.new_section_error = Some("No project-owned model candidate is open".to_owned());
            return false;
        };
        let name = self.new_section_name.clone();
        if name.trim() != name || name.is_empty() {
            self.new_section_error =
                Some("Section name must not be empty or contain outer whitespace".to_owned());
            return false;
        }
        let parent = if self.new_section_parent.is_empty() {
            None
        } else {
            Some(self.new_section_parent.clone())
        };
        let source_identity = draft
            .metadata
            .source_identity
            .clone()
            .or_else(|| {
                draft
                    .metadata
                    .sections
                    .first()
                    .and_then(|section| section.model_files.first())
                    .cloned()
            })
            .unwrap_or_else(|| ModelFileIdentity {
                source_id: draft.source_id.to_string(),
                revision: draft.base_source_revision.get(),
                content_digest: draft.base_source_digest.to_string(),
                display_name: format!("{}.model", draft.library_name),
            });
        let section = ModelSectionDefinition {
            name,
            parent,
            overrides: BTreeMap::new(),
            model_files: vec![source_identity],
            qualification: ModelSectionQualification::Unqualified,
        };
        let previous = draft.metadata.sections.clone();
        draft.metadata.sections.push(section);
        draft.metadata.sections.sort_by(|left, right| {
            left.name
                .to_ascii_lowercase()
                .cmp(&right.name.to_ascii_lowercase())
                .then_with(|| left.name.cmp(&right.name))
        });
        if let Err(diagnostics) = draft.definition() {
            draft.metadata.sections = previous;
            self.new_section_error = Some(
                diagnostics
                    .into_iter()
                    .map(|diagnostic| format!("{}: {}", diagnostic.field, diagnostic.message))
                    .collect::<Vec<_>>()
                    .join("; "),
            );
            return false;
        }
        self.new_section_open = false;
        self.new_section_error = None;
        self.invalidate_candidate_evidence();
        true
    }

    /// Begin a correlation edit transaction with stable text buffers. Keeping
    /// partial numeric input out of the typed definition prevents focus and
    /// validation changes from moving or rebuilding matrix controls.
    pub fn begin_correlation_matrix_edit(&mut self) {
        self.correlation_matrix_candidates = self
            .draft
            .as_ref()
            .map(|draft| {
                let mut matrices = draft.metadata.statistics.correlation_matrices.clone();
                let mut groups = BTreeMap::<String, Vec<String>>::new();
                for variable in &draft.metadata.statistics.variables {
                    if let Some(group) = variable.correlation_group.as_ref() {
                        groups
                            .entry(group.clone())
                            .or_default()
                            .push(variable.name.clone());
                    }
                }
                for (group, variables) in groups {
                    if matrices
                        .iter()
                        .any(|matrix| matrix.group.eq_ignore_ascii_case(&group))
                    {
                        continue;
                    }
                    let dimension = variables.len();
                    matrices.push(CorrelationMatrix {
                        group,
                        variables,
                        coefficients: (0..dimension)
                            .map(|row| {
                                (0..dimension)
                                    .map(|column| {
                                        FiniteF64::new(if row == column { 1.0 } else { 0.0 })
                                            .expect("identity correlation is finite")
                                    })
                                    .collect()
                            })
                            .collect(),
                    });
                }
                matrices.sort_by(|left, right| {
                    left.group
                        .to_ascii_lowercase()
                        .cmp(&right.group.to_ascii_lowercase())
                        .then_with(|| left.group.cmp(&right.group))
                });
                matrices
            })
            .unwrap_or_default();
        self.correlation_matrix_edits = self
            .correlation_matrix_candidates
            .iter()
            .map(|matrix| {
                matrix
                    .coefficients
                    .iter()
                    .map(|row| row.iter().map(ToString::to_string).collect())
                    .collect()
            })
            .collect();
        self.correlation_matrix_error = None;
        self.correlation_matrix_open = true;
    }

    /// Commit the complete matrix set only after every coefficient and the
    /// resulting model definition validate. Failure leaves the candidate
    /// byte-for-byte unchanged so no partial matrix can escape the dialog.
    pub fn commit_correlation_matrix_edit(&mut self) -> bool {
        let Some(draft) = self.draft.as_mut() else {
            self.correlation_matrix_error =
                Some("No project-owned model candidate is open".to_owned());
            return false;
        };
        let matrices = &self.correlation_matrix_candidates;
        if self.correlation_matrix_edits.len() != matrices.len() {
            self.correlation_matrix_error =
                Some("The candidate correlation schema changed; reopen the editor".to_owned());
            return false;
        }
        let mut parsed = Vec::with_capacity(matrices.len());
        for (matrix_index, (matrix, rows)) in matrices
            .iter()
            .zip(&self.correlation_matrix_edits)
            .enumerate()
        {
            let dimension = matrix.variables.len();
            if rows.len() != dimension || rows.iter().any(|row| row.len() != dimension) {
                self.correlation_matrix_error = Some(format!(
                    "Correlation matrix '{}' no longer matches its variable order",
                    matrix.group
                ));
                return false;
            }
            let mut coefficients = Vec::with_capacity(dimension);
            for (row_index, row) in rows.iter().enumerate() {
                let mut parsed_row = Vec::with_capacity(dimension);
                for (column_index, value) in row.iter().enumerate() {
                    let parsed_value = match value.trim().parse::<f64>() {
                        Ok(value) if value.is_finite() => value,
                        Ok(_) => {
                            self.correlation_matrix_error = Some(format!(
                                "Matrix '{}' coefficient [{row_index}, {column_index}] must be finite",
                                matrix.group
                            ));
                            return false;
                        }
                        Err(error) => {
                            self.correlation_matrix_error = Some(format!(
                                "Matrix '{}' coefficient [{row_index}, {column_index}] is invalid: {error}",
                                matrix.group
                            ));
                            return false;
                        }
                    };
                    parsed_row.push(
                        FiniteF64::new(parsed_value)
                            .expect("finite correlation value was checked above"),
                    );
                }
                coefficients.push(parsed_row);
            }
            let mut candidate = matrix.clone();
            candidate.coefficients = coefficients;
            parsed.push(candidate);
            debug_assert_eq!(matrix_index + 1, parsed.len());
        }

        let previous = draft.metadata.statistics.correlation_matrices.clone();
        draft.metadata.statistics.correlation_matrices = parsed;
        if let Err(diagnostics) = draft.definition() {
            draft.metadata.statistics.correlation_matrices = previous;
            self.correlation_matrix_error = Some(
                diagnostics
                    .into_iter()
                    .map(|diagnostic| format!("{}: {}", diagnostic.field, diagnostic.message))
                    .collect::<Vec<_>>()
                    .join("; "),
            );
            return false;
        }
        self.correlation_matrix_open = false;
        self.correlation_matrix_candidates.clear();
        self.correlation_matrix_edits.clear();
        self.correlation_matrix_error = None;
        self.invalidate_candidate_evidence();
        true
    }

    pub fn validate_candidate(
        &mut self,
        manager: &ModelLibraryManager,
        project_revision: ObjectRevision,
    ) -> bool {
        let Some(draft) = self.draft.as_ref() else {
            self.validation = None;
            self.diagnostics = vec![ModelFieldDiagnostic {
                field: "source".to_owned(),
                message: "No project-owned model candidate is open".to_owned(),
            }];
            return false;
        };
        match draft.validate(manager, project_revision) {
            Ok(evidence) => {
                self.validation = Some(evidence);
                self.diagnostics.clear();
                true
            }
            Err(diagnostics) => {
                self.validation = None;
                self.diagnostics = diagnostics;
                false
            }
        }
    }

    pub fn begin_qualification_suite(&mut self) {
        self.qualification_authoring = QualificationAuthoringDraft::default();
        self.qualification_authoring_open = true;
    }

    pub fn begin_promotion_review(&mut self) {
        let mut candidate = PromotionCandidateDraft::default();
        let mut retained_candidate_identity = None;
        if let Some(draft) = self.draft.as_ref() {
            candidate.candidate_id = format!(
                "{}-candidate-r{}",
                draft.model_name,
                draft.base_source_revision.get()
            );
            candidate.candidate_version =
                format!("candidate-r{}", draft.base_source_revision.get());
            if let Some(evidence) = draft.qualification.evidence.iter().find(|evidence| {
                evidence
                    .source
                    .model_id
                    .eq_ignore_ascii_case(&draft.model_name)
                    && evidence.source.source_id == Some(draft.source_id)
                    && evidence.source.source_digest == draft.base_source_digest
                    && evidence.source.source_revision == draft.base_source_revision
                    && evidence.passed
            }) {
                candidate.suite_id.clone_from(&evidence.suite_id);
                candidate.evidence_id.clone_from(&evidence.id);
            }
            if let Some(retained_candidate) =
                draft.qualification.candidates.iter().find(|candidate| {
                    candidate
                        .source
                        .model_id
                        .eq_ignore_ascii_case(&draft.model_name)
                        && candidate.source.source_id == Some(draft.source_id)
                        && candidate.source.source_digest == draft.base_source_digest
                        && candidate.source.source_revision == draft.base_source_revision
                })
            {
                candidate = PromotionCandidateDraft::from_candidate(retained_candidate);
                retained_candidate_identity = Some((
                    retained_candidate.identity.id.clone(),
                    retained_candidate.identity.version.clone(),
                    draft.model_name.clone(),
                    draft.base_source_revision,
                ));
            }
        }
        self.promotion_candidate = candidate;
        if let Some((candidate_id, candidate_version, model_name, revision)) =
            retained_candidate_identity
        {
            self.promotion_record_id = format!("{candidate_id}-promotion");
            self.promotion_release_id = format!("{model_name}-release-r{}", revision.get());
            self.promotion_release_version = candidate_version;
        } else {
            self.promotion_record_id.clear();
            self.promotion_release_id.clear();
            self.promotion_release_version.clear();
        }
        self.promotion_error = None;
        self.promotion_review_open = true;
    }

    /// Create one complete governed release candidate from the dialog's fixed
    /// buffers. Every document and approval is parsed before a cloned
    /// qualification aggregate is touched; invalid input leaves the draft
    /// byte-for-byte unchanged.
    pub fn commit_release_candidate(&mut self) -> bool {
        if self.qualification_execution.is_some() {
            self.promotion_candidate.error = Some(
                "Finish or cancel the active qualification run before creating a release candidate"
                    .to_owned(),
            );
            return false;
        }
        let Some(draft) = self.draft.as_mut() else {
            self.promotion_candidate.error =
                Some("No project-owned model candidate is open".to_owned());
            return false;
        };
        if draft.definition_is_dirty() {
            self.promotion_candidate.error = Some(
                "Save the model definition before creating a source-bound release candidate"
                    .to_owned(),
            );
            return false;
        }
        let fields = self.promotion_candidate.clone();
        let result = build_release_candidate(draft, &fields);
        let candidate = match result {
            Ok(candidate) => candidate,
            Err(error) => {
                self.promotion_candidate.error = Some(error);
                return false;
            }
        };
        let mut qualification = draft.qualification.clone();
        let exact_candidate_indices = qualification
            .candidates
            .iter()
            .enumerate()
            .filter_map(|(index, retained)| {
                (retained
                    .source
                    .model_id
                    .eq_ignore_ascii_case(&draft.model_name)
                    && retained.source.source_id == Some(draft.source_id)
                    && retained.source.source_digest == draft.base_source_digest
                    && retained.source.source_revision == draft.base_source_revision)
                    .then_some(index)
            })
            .collect::<Vec<_>>();
        match exact_candidate_indices.as_slice() {
            [] => qualification.candidates.push(candidate.clone()),
            [index]
                if qualification.promotions.iter().any(|promotion| {
                    promotion
                        .candidate_identity
                        .id
                        .eq_ignore_ascii_case(&qualification.candidates[*index].identity.id)
                }) =>
            {
                self.promotion_candidate.error = Some(
                    "The retained release candidate has already been promoted and is immutable"
                        .to_owned(),
                );
                return false;
            }
            [index]
                if qualification.candidates[*index]
                    .identity
                    .id
                    .eq_ignore_ascii_case(&candidate.identity.id) =>
            {
                qualification.candidates[*index] = candidate.clone();
            }
            [..] => {
                self.promotion_candidate.error = Some(
                    "This exact model source already has a different or ambiguous release candidate identity; repair the retained qualification record before continuing"
                        .to_owned(),
                );
                return false;
            }
        }
        qualification.candidates.sort_by(|left, right| {
            left.identity
                .id
                .to_ascii_lowercase()
                .cmp(&right.identity.id.to_ascii_lowercase())
                .then_with(|| left.identity.id.cmp(&right.identity.id))
        });
        if let Err(error) = qualification.validate_for_model(&draft.model_name) {
            self.promotion_candidate.error = Some(error.to_string());
            return false;
        }
        draft.qualification = qualification;
        self.promotion_candidate.error = None;
        self.promotion_record_id = format!("{}-promotion", candidate.identity.id);
        self.promotion_release_id = format!(
            "{}-release-r{}",
            draft.model_name,
            draft.base_source_revision.get()
        );
        self.promotion_release_version = fields.candidate_version;
        self.qualification_execution_notice = Some(
            "Governed release candidate retained. Review its derived checklist, promote it, then save the qualification revision to publish the transaction."
                .to_owned(),
        );
        true
    }

    /// Promote the selected, fully qualified candidate through the domain's
    /// atomic release transaction. No release or audit record survives a
    /// failed gate, duplicate identity, or stale source binding.
    pub fn commit_promotion(
        &mut self,
        candidate_id: &str,
        correlation: &ModelCorrelationState,
    ) -> bool {
        if self.qualification_execution.is_some() {
            self.promotion_error = Some(
                "Finish or cancel the active qualification run before promoting a model".to_owned(),
            );
            return false;
        }
        let Some(draft) = self.draft.as_mut() else {
            self.promotion_error = Some("No project-owned model candidate is open".to_owned());
            return false;
        };
        if draft.definition_is_dirty() {
            self.promotion_error = Some(
                "Save the model definition before promoting a source-bound candidate".to_owned(),
            );
            return false;
        }
        let source = match ModelSourceEvidenceBinding::try_new_project_bound(
            &draft.model_name,
            draft.source_id,
            draft.base_source_digest,
            draft.base_source_revision,
        ) {
            Ok(source) => source,
            Err(error) => {
                self.promotion_error = Some(format!(
                    "Model promotion cannot authenticate its exact source binding: {error}"
                ));
                return false;
            }
        };
        if let Err(error) = correlation.require_release_approval(&draft.model_name, &source) {
            self.promotion_error = Some(format!(
                "Model promotion is blocked by measurement correlation: {error}"
            ));
            return false;
        }
        let identity = ModelReleaseIdentity {
            id: self.promotion_release_id.clone(),
            model_id: draft.model_name.clone(),
            version: self.promotion_release_version.clone(),
        };
        let (definition_source, definition_metadata) = match draft
            .definition()
            .map_err(|diagnostics| {
                diagnostics
                    .into_iter()
                    .map(|diagnostic| format!("{}: {}", diagnostic.field, diagnostic.message))
                    .collect::<Vec<_>>()
                    .join("; ")
            })
            .and_then(|definition| {
                let source = definition
                    .canonical_source()
                    .map_err(|error| error.to_string())?;
                Ok((source.into_bytes(), definition.metadata))
            }) {
            Ok(definition) => definition,
            Err(error) => {
                self.promotion_error = Some(error);
                return false;
            }
        };
        let mut qualification = draft.qualification.clone();
        let Some(candidate) = qualification
            .candidates
            .iter_mut()
            .find(|candidate| candidate.identity.id.eq_ignore_ascii_case(candidate_id))
        else {
            self.promotion_error = Some(format!(
                "Release candidate '{candidate_id}' no longer exists"
            ));
            return false;
        };
        candidate.definition_source = definition_source;
        candidate.definition_metadata = Some(definition_metadata);
        if let Err(error) = qualification.validate_for_model(&draft.model_name) {
            self.promotion_error = Some(error.to_string());
            return false;
        }
        match qualification.promote_candidate_atomically(
            self.promotion_record_id.clone(),
            identity,
            candidate_id,
        ) {
            Ok(()) => {
                draft.qualification = qualification;
                self.promotion_review_open = false;
                self.promotion_error = None;
                self.qualification_execution_notice = Some(
                    "Model release promoted in the candidate. Save the qualification revision to retain the immutable release and audit record."
                        .to_owned(),
                );
                true
            }
            Err(error) => {
                self.promotion_error = Some(error.to_string());
                false
            }
        }
    }

    /// Add one exact executable suite/vector contract atomically. Invalid
    /// fields leave the retained qualification aggregate byte-for-byte intact.
    pub fn commit_qualification_suite(&mut self) -> bool {
        let result = self.build_qualification_suite();
        let mut suite = match result {
            Ok(suite) => suite,
            Err(error) => {
                self.qualification_authoring.error = Some(error);
                return false;
            }
        };
        let Some(draft) = self.draft.as_mut() else {
            self.qualification_authoring.error =
                Some("No project-owned model candidate is open".to_owned());
            return false;
        };
        let mut qualification = draft.qualification.clone();
        if let Some(existing_index) = qualification
            .suites
            .iter()
            .position(|existing| existing.id.eq_ignore_ascii_case(&suite.id))
        {
            let existing = qualification.suites[existing_index].clone();
            if existing.name != suite.name {
                self.qualification_authoring.error = Some(format!(
                    "Suite ID '{}' already belongs to '{}'; use the exact retained suite name when adding a vector",
                    existing.id, existing.name
                ));
                return false;
            }
            if qualification
                .evidence
                .iter()
                .any(|evidence| evidence.suite_id.eq_ignore_ascii_case(&existing.id))
                || qualification
                    .candidates
                    .iter()
                    .any(|candidate| candidate.suite_id.eq_ignore_ascii_case(&existing.id))
            {
                self.qualification_authoring.error = Some(format!(
                    "Suite '{}' has retained evidence or release candidates and is immutable; create a new suite identity",
                    existing.id
                ));
                return false;
            }
            let mut vectors = existing.vectors;
            vectors.append(&mut suite.vectors);
            let revision = match existing.revision.next() {
                Ok(revision) => revision,
                Err(error) => {
                    self.qualification_authoring.error = Some(error.to_string());
                    return false;
                }
            };
            suite = match QualificationSuite::try_new(existing.id, existing.name, revision, vectors)
            {
                Ok(suite) => suite,
                Err(error) => {
                    self.qualification_authoring.error = Some(error.to_string());
                    return false;
                }
            };
            qualification
                .platform_runs
                .retain(|run| !run.suite_id.eq_ignore_ascii_case(&suite.id));
            qualification.suites[existing_index] = suite;
        } else {
            qualification.suites.push(suite);
        }
        qualification.suites.sort_by(|left, right| {
            left.id
                .to_ascii_lowercase()
                .cmp(&right.id.to_ascii_lowercase())
                .then_with(|| left.id.cmp(&right.id))
        });
        if let Err(error) = qualification.validate_for_model(&draft.model_name) {
            self.qualification_authoring.error = Some(error.to_string());
            return false;
        }
        draft.qualification = qualification;
        self.qualification_authoring_open = false;
        self.qualification_authoring = QualificationAuthoringDraft::default();
        self.qualification_execution_notice = Some(
            "Qualification suite or vector added to the candidate. Save the qualification revision to retain it."
                .to_owned(),
        );
        true
    }

    fn require_idle_qualification_draft(&mut self) -> Result<&mut ModelEditorDraft, String> {
        if self.qualification_execution.is_some() {
            return Err(
                "Finish or cancel the active qualification run before editing its plan".to_owned(),
            );
        }
        self.draft
            .as_mut()
            .ok_or_else(|| "No project-owned model candidate is open".to_owned())
    }

    /// Replace/edit one retained vector through the domain's atomic,
    /// revision-advancing transaction.
    pub fn replace_qualification_vector(
        &mut self,
        suite_id: &str,
        vector_id: &str,
        replacement: QualificationVector,
    ) -> Result<(), String> {
        let draft = self.require_idle_qualification_draft()?;
        draft
            .qualification
            .replace_vector_atomically(suite_id, vector_id, replacement)
            .map_err(|error| error.to_string())?;
        self.qualification_execution_notice = Some(format!(
            "Qualification vector {vector_id:?} was replaced atomically; prior uncommitted suite runs were invalidated."
        ));
        Ok(())
    }

    pub fn delete_qualification_vector(
        &mut self,
        suite_id: &str,
        vector_id: &str,
    ) -> Result<(), String> {
        let draft = self.require_idle_qualification_draft()?;
        draft
            .qualification
            .delete_vector_atomically(suite_id, vector_id)
            .map_err(|error| error.to_string())?;
        self.qualification_execution_notice = Some(format!(
            "Qualification vector {vector_id:?} was retired from suite {suite_id:?}."
        ));
        Ok(())
    }

    pub fn delete_qualification_suite(&mut self, suite_id: &str) -> Result<(), String> {
        let draft = self.require_idle_qualification_draft()?;
        draft
            .qualification
            .delete_suite_atomically(suite_id)
            .map_err(|error| error.to_string())?;
        self.qualification_execution_notice = Some(format!(
            "Editable qualification suite {suite_id:?} was deleted; evidence and release lineage were unchanged."
        ));
        Ok(())
    }

    pub fn record_qualification_vector_disposition(
        &mut self,
        disposition_id: impl Into<String>,
        suite_id: &str,
        vector_id: &str,
        cause: QualificationVectorDispositionCause,
        required_action: QualificationVectorRequiredAction,
        reason: impl Into<String>,
    ) -> Result<QualificationVectorDisposition, String> {
        let draft = self.require_idle_qualification_draft()?;
        let current_source = ModelSourceEvidenceBinding::try_new_project_bound(
            draft.model_name.clone(),
            draft.source_id,
            draft.base_source_digest,
            draft.base_source_revision,
        )
        .map_err(|error| error.to_string())?;
        let disposition = draft
            .qualification
            .record_vector_disposition_atomically(
                disposition_id,
                suite_id,
                vector_id,
                &current_source,
                cause,
                required_action,
                reason,
            )
            .map_err(|error| error.to_string())?;
        self.qualification_execution_notice = Some(format!(
            "Vector disposition {:?} was retained as a blocking, non-waiving qualification record.",
            disposition.id
        ));
        Ok(disposition)
    }

    pub fn resolve_qualification_vector_rerun(
        &mut self,
        disposition_id: &str,
    ) -> Result<(), String> {
        let draft = self.require_idle_qualification_draft()?;
        draft
            .qualification
            .resolve_vector_disposition_by_rerun_atomically(disposition_id)
            .map_err(|error| error.to_string())?;
        self.qualification_execution_notice = Some(format!(
            "Vector disposition {disposition_id:?} was closed by exact passing Desktop and WebAssembly rerun evidence."
        ));
        Ok(())
    }

    fn build_qualification_suite(&self) -> Result<QualificationSuite, String> {
        let draft = self
            .draft
            .as_ref()
            .ok_or_else(|| "No project-owned model candidate is open".to_owned())?;
        if draft.definition_is_dirty() {
            return Err(
                "Save the model definition before binding a qualification vector to it".to_owned(),
            );
        }
        let fields = &self.qualification_authoring;
        let source = ModelSourceEvidenceBinding::try_new_project_bound(
            draft.model_name.clone(),
            draft.source_id,
            draft.base_source_digest,
            draft.base_source_revision,
        )
        .map_err(|error| error.to_string())?;
        let definition = draft.definition().map_err(|diagnostics| {
            diagnostics
                .into_iter()
                .map(|diagnostic| format!("{}: {}", diagnostic.field, diagnostic.message))
                .collect::<Vec<_>>()
                .join("; ")
        })?;
        let model_source = definition
            .canonical_source()
            .map_err(|error| error.to_string())?
            .into_bytes();
        if fields.model_section.trim() != fields.model_section {
            return Err("Model section must not contain outer whitespace".to_owned());
        }
        let model_section =
            (!fields.model_section.is_empty()).then(|| fields.model_section.clone());
        let execution_model_source = definition
            .qualification_model_source(model_section.as_deref())
            .map_err(|error| error.to_string())?
            .into_bytes();
        // SPICE reserves the first physical line as a deck title. Keep the
        // selected model card after an explicit title so a section card that
        // begins directly with `.model` is parsed rather than discarded.
        let mut executable_input = b"RSpice model qualification\n".to_vec();
        executable_input.extend_from_slice(&execution_model_source);
        if !executable_input.ends_with(b"\n") {
            executable_input.push(b'\n');
        }
        executable_input.extend_from_slice(fields.executable_input.as_bytes());
        let analysis = match fields.analysis {
            QualificationAuthoringAnalysis::DcOperatingPoint => {
                QualificationAnalysis::DcOperatingPoint
            }
            QualificationAuthoringAnalysis::DcSweep => QualificationAnalysis::DcSweep {
                source: fields.sweep_source.clone(),
                start: FiniteValue::new(parse_finite(&fields.sweep_start, "Sweep start")?)
                    .map_err(|error| error.to_string())?,
                stop: FiniteValue::new(parse_finite(&fields.sweep_stop, "Sweep stop")?)
                    .map_err(|error| error.to_string())?,
                step: FiniteValue::new(parse_finite(&fields.sweep_step, "Sweep step")?)
                    .map_err(|error| error.to_string())?,
            },
            QualificationAuthoringAnalysis::AcSweep => QualificationAnalysis::AcSweep {
                frequencies: parse_frequency_axis(&fields.frequencies)?,
            },
            QualificationAuthoringAnalysis::Noise => QualificationAnalysis::Noise {
                output_node: fields.noise_output_node.clone(),
                output_reference: (!fields.noise_output_reference.is_empty())
                    .then(|| fields.noise_output_reference.clone()),
                input_source: fields.noise_input_source.clone(),
                frequencies: parse_frequency_axis(&fields.frequencies)?,
                temperature_kelvin: FiniteValue::new(parse_finite(
                    &fields.noise_temperature_kelvin,
                    "Noise temperature",
                )?)
                .map_err(|error| error.to_string())?,
            },
            QualificationAuthoringAnalysis::Transient => QualificationAnalysis::Transient {
                stop_time: FiniteValue::new(parse_finite(
                    &fields.transient_stop_time,
                    "Transient stop time",
                )?)
                .map_err(|error| error.to_string())?,
                max_step: FiniteValue::new(parse_finite(
                    &fields.transient_max_step,
                    "Transient maximum step",
                )?)
                .map_err(|error| error.to_string())?,
            },
        };
        let probe = match fields.probe {
            QualificationAuthoringProbe::NodeVoltage => QualificationProbe::NodeVoltage {
                node: fields.probe_target.clone(),
            },
            QualificationAuthoringProbe::BranchCurrent => QualificationProbe::BranchCurrent {
                branch: fields.probe_target.clone(),
            },
            QualificationAuthoringProbe::DcObservable => QualificationProbe::DcObservable {
                expression: fields.probe_target.clone(),
            },
            QualificationAuthoringProbe::SweepValue => QualificationProbe::SweepValue,
            QualificationAuthoringProbe::AcNodeVoltageMagnitude => {
                QualificationProbe::AcNodeVoltageMagnitude {
                    node: fields.probe_target.clone(),
                }
            }
            QualificationAuthoringProbe::AcNodeVoltagePhaseDegrees => {
                QualificationProbe::AcNodeVoltagePhaseDegrees {
                    node: fields.probe_target.clone(),
                }
            }
            QualificationAuthoringProbe::AcNodeVoltageReal => {
                QualificationProbe::AcNodeVoltageReal {
                    node: fields.probe_target.clone(),
                }
            }
            QualificationAuthoringProbe::AcNodeVoltageImaginary => {
                QualificationProbe::AcNodeVoltageImaginary {
                    node: fields.probe_target.clone(),
                }
            }
            QualificationAuthoringProbe::AcBranchCurrentMagnitude => {
                QualificationProbe::AcBranchCurrentMagnitude {
                    branch: fields.probe_target.clone(),
                }
            }
            QualificationAuthoringProbe::AcBranchCurrentPhaseDegrees => {
                QualificationProbe::AcBranchCurrentPhaseDegrees {
                    branch: fields.probe_target.clone(),
                }
            }
            QualificationAuthoringProbe::AcBranchCurrentReal => {
                QualificationProbe::AcBranchCurrentReal {
                    branch: fields.probe_target.clone(),
                }
            }
            QualificationAuthoringProbe::AcBranchCurrentImaginary => {
                QualificationProbe::AcBranchCurrentImaginary {
                    branch: fields.probe_target.clone(),
                }
            }
            QualificationAuthoringProbe::AcEffectiveCapacitance => {
                QualificationProbe::AcEffectiveCapacitance {
                    branch: fields.probe_target.clone(),
                    excitation_magnitude: FiniteValue::new(parse_finite(
                        &fields.excitation_magnitude,
                        "AC excitation magnitude",
                    )?)
                    .map_err(|error| error.to_string())?,
                }
            }
            QualificationAuthoringProbe::FrequencyValue => QualificationProbe::FrequencyValue,
            QualificationAuthoringProbe::NoiseOutputDensity => {
                QualificationProbe::NoiseOutputDensity
            }
            QualificationAuthoringProbe::NoiseInputReferredDensity => {
                QualificationProbe::NoiseInputReferredDensity
            }
            QualificationAuthoringProbe::NoiseOutputAmplitude => {
                QualificationProbe::NoiseOutputAmplitude
            }
            QualificationAuthoringProbe::NoiseInputReferredAmplitude => {
                QualificationProbe::NoiseInputReferredAmplitude
            }
            QualificationAuthoringProbe::TransientNodeVoltage => {
                QualificationProbe::TransientNodeVoltage {
                    node: fields.probe_target.clone(),
                }
            }
            QualificationAuthoringProbe::TransientBranchCurrent => {
                QualificationProbe::TransientBranchCurrent {
                    branch: fields.probe_target.clone(),
                }
            }
            QualificationAuthoringProbe::TimeValue => QualificationProbe::TimeValue,
        };
        let sample = match fields.sample {
            QualificationAuthoringSample::OperatingPoint => QualificationSample::OperatingPoint,
            QualificationAuthoringSample::FirstSweepPoint => QualificationSample::FirstSweepPoint,
            QualificationAuthoringSample::LastSweepPoint => QualificationSample::LastSweepPoint,
            QualificationAuthoringSample::SweepPoint => QualificationSample::SweepPoint {
                index: fields
                    .sample_index
                    .trim()
                    .parse::<usize>()
                    .map_err(|error| format!("Sweep point index is invalid: {error}"))?,
            },
            QualificationAuthoringSample::FirstFrequencyPoint => {
                QualificationSample::FirstFrequencyPoint
            }
            QualificationAuthoringSample::LastFrequencyPoint => {
                QualificationSample::LastFrequencyPoint
            }
            QualificationAuthoringSample::FrequencyPoint => QualificationSample::FrequencyPoint {
                index: parse_sample_index(&fields.sample_index, "Frequency point index")?,
            },
            QualificationAuthoringSample::FirstTimePoint => QualificationSample::FirstTimePoint,
            QualificationAuthoringSample::LastTimePoint => QualificationSample::LastTimePoint,
            QualificationAuthoringSample::TimePoint => QualificationSample::TimePoint {
                index: parse_sample_index(&fields.sample_index, "Time point index")?,
            },
        };
        let output = QualificationOutputDefinition::try_new(fields.quantity.clone(), probe, sample)
            .map_err(|error| error.to_string())?;
        let reference = QualificationReference::try_new(
            fields.quantity.clone(),
            parse_finite(&fields.expected, "Expected value")?,
            parse_non_negative(&fields.absolute_tolerance, "Absolute tolerance")?,
            parse_non_negative(&fields.relative_tolerance, "Relative tolerance")?,
        )
        .map_err(|error| error.to_string())?;
        let vector = QualificationVector::try_new_source_section_bound(
            fields.vector_id.clone(),
            fields.vector_name.clone(),
            source,
            model_source,
            model_section,
            execution_model_source,
            executable_input,
            analysis,
            vec![output],
            vec![reference],
        )
        .map_err(|error| error.to_string())?;
        QualificationSuite::try_new(
            fields.suite_id.clone(),
            fields.suite_name.clone(),
            ObjectRevision::INITIAL,
            vec![vector],
        )
        .map_err(|error| error.to_string())
    }
}

#[cfg(test)]
mod tests;
