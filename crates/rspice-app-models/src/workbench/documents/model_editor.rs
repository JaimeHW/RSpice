//! Transactional presentation state for the governed device-model editor.
//!
//! Committed source bytes and model metadata remain owned by the project
//! model-library domain. This module retains only an open candidate, local
//! text-entry buffers, validation evidence, and section/dialog presentation.

mod operations;
mod parameters;

pub use operations::*;
// Crate-private: `parameters` exposes only `pub(super)` helpers.
use parameters::*;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::product::{ContentDigest, ModelSourceId, ObjectRevision};
use crate::state::model_library::{
    CorrelationMatrix, FiniteBounds, FiniteF64, ModelDefinitionMetadata, ModelFileIdentity,
    ModelLibrary, ModelLibraryManager, ModelSectionDefinition, ModelSourceAuthority,
    ParameterDataType, ParameterDefinition, ParameterSource, ParameterValue,
    ProjectModelDefinition, ProjectModelRevisionDefinition,
};
use sha2::{Digest as _, Sha256};

use crate::workbench::{RouteTransitionSource, SurfaceId, SurfaceRoute};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ModelEditorSection {
    #[default]
    Parameters,
    Sections,
    Statistics,
    Temperature,
}

impl ModelEditorSection {
    #[cfg(test)]
    pub const ALL: [Self; 4] = [
        Self::Parameters,
        Self::Sections,
        Self::Statistics,
        Self::Temperature,
    ];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Parameters => "Parameters",
            Self::Sections => "Sections",
            Self::Statistics => "Statistics",
            Self::Temperature => "Temperature",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ModelParameterKind {
    Numeric,
    String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone)]
pub(crate) struct ResolvedEditableProjectModel {
    pub(crate) source_id: ModelSourceId,
    pub(crate) library_revision: ObjectRevision,
    pub(crate) model_revision: ObjectRevision,
    pub(crate) model_digest: ContentDigest,
    pub(crate) source_path: PathBuf,
    pub(crate) definition: ProjectModelRevisionDefinition,
}

#[cfg(test)]
thread_local! {
    /// How many closures have been authenticated on this thread.
    ///
    /// Authentication is the expensive half of resolving a model and depends
    /// only on the library, so a caller that resolves many models from one
    /// library must not repeat it per model. Counting is the only way to state
    /// that as a test: the work is invisible in the result, and a timing check
    /// on a fixture small enough to build is noise.
    pub(crate) static CLOSURE_AUTHENTICATIONS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}

/// A project-owned library whose retained closure has been authenticated.
///
/// Holding one is the proof that every retained source matched its pin, that
/// pins and bytes describe the same set, and that the include graph is rooted
/// and acyclic. Resolving a model then costs only that model's own work, which
/// is what makes it affordable to resolve every model in a library.
#[derive(Debug)]
pub(crate) struct VerifiedProjectClosure<'a> {
    source_id: ModelSourceId,
    library_revision: ObjectRevision,
    contents: BTreeMap<PathBuf, &'a Vec<u8>>,
}

/// Authenticate a project-owned library's retained source closure.
///
/// This is the expensive half of resolving a model — it digests every byte of
/// every retained source — and it depends only on the library. Callers that
/// resolve more than one model from the same library must call this once and
/// pass the result to [`resolve_project_model_in_closure`]; calling the
/// combined [`resolve_project_model_for_editor`] in a loop re-digests the whole
/// closure per model.
pub(crate) fn verify_project_library_closure<'a>(
    library: &'a ModelLibrary,
    library_name: &str,
) -> Result<VerifiedProjectClosure<'a>, String> {
    let ModelSourceAuthority::ProjectOwned {
        source_id,
        revision: library_revision,
        digest: root_digest,
    } = library.source_authority
    else {
        // Counted below the authority match on purpose: a library that is not
        // project-owned has no retained closure to digest, so asking about one
        // costs nothing and is not what the counter is watching for.
        return Err(format!(
            "Library '{library_name}' is not project-owned; create an editable project copy before opening the model editor"
        ));
    };

    #[cfg(test)]
    CLOSURE_AUTHENTICATIONS.with(|count| count.set(count.get() + 1));

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

    Ok(VerifiedProjectClosure {
        source_id,
        library_revision,
        contents,
    })
}

/// How many times `needle` occurs in `haystack`, counting no further than two.
///
/// The caller only needs to distinguish "never", "once" and "more than once".
/// Text sources take `str::find`'s linear search; a source that is not UTF-8
/// falls back to the naive scan so the invariant still holds for it.
fn occurrences_up_to_two(haystack: &[u8], needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    match std::str::from_utf8(haystack) {
        Ok(text) => text.match_indices(needle).take(2).count(),
        Err(_) => haystack
            .windows(needle.len())
            .filter(|candidate| *candidate == needle.as_bytes())
            .take(2)
            .count(),
    }
}

/// Resolve one selected model from an already-authenticated closure.
///
/// A library may contain many models and include members, but the selected
/// canonical revision must occur exactly once in the source file recorded by
/// that model's projection.
pub(crate) fn resolve_project_model_in_closure(
    library: &ModelLibrary,
    library_name: &str,
    model_name: &str,
    closure: &VerifiedProjectClosure<'_>,
) -> Result<ResolvedEditableProjectModel, String> {
    let VerifiedProjectClosure {
        source_id,
        library_revision,
        contents,
    } = closure;
    let (source_id, library_revision) = (*source_id, *library_revision);

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
    let occurrences = occurrences_up_to_two(source_bytes, &canonical);
    if occurrences != 1 {
        return Err(format!(
            "Model '{model_name}' definition must appear exactly once in source '{}' (found {})",
            source_path.display(),
            if occurrences < 2 {
                occurrences.to_string()
            } else {
                "more than 1".to_owned()
            }
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
    })
}

/// Authenticate a library's closure and resolve one model from it.
///
/// The shared fail-closed gate used by command availability, editor open, and
/// stale-candidate validation — each of which wants exactly one model. Anything
/// resolving several models from one library should verify the closure once
/// instead; see [`verify_project_library_closure`].
pub(crate) fn resolve_project_model_for_editor(
    manager: &ModelLibraryManager,
    library_name: &str,
    model_name: &str,
) -> Result<ResolvedEditableProjectModel, String> {
    let library = manager
        .get_library(library_name)
        .ok_or_else(|| format!("Model library '{library_name}' does not exist"))?;
    let closure = verify_project_library_closure(library, library_name)?;
    resolve_project_model_in_closure(library, library_name, model_name, &closure)
}

#[derive(Debug, Clone)]
pub struct ModelEditorDraft {
    pub library_name: String,
    pub model_name: String,
    pub source_id: ModelSourceId,
    /// Revision of the entire retained library closure used only for
    /// optimistic concurrency and stale-library detection.
    pub base_library_revision: ObjectRevision,
    /// Independent revision of this model's canonical fragment.
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
    base_definition: ProjectModelRevisionDefinition,
}

/// Serializable recovery payload for an unsaved project-model candidate.
///
/// Immutable base definitions are reconstructed from the authenticated
/// project model library on restore rather than trusted from session bytes.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelEditorRecoveryDraft {
    library_name: String,
    model_name: String,
    source_id: ModelSourceId,
    base_library_revision: ObjectRevision,
    base_source_revision: ObjectRevision,
    base_source_digest: ContentDigest,
    source_path: PathBuf,
    base_project_revision: ObjectRevision,
    name: String,
    spice_type: String,
    description: String,
    parameters: Vec<ModelParameterDraft>,
    metadata: ModelDefinitionMetadata,
}

impl ModelEditorRecoveryDraft {
    #[must_use]
    pub fn capture(draft: &ModelEditorDraft) -> Self {
        Self {
            library_name: draft.library_name.clone(),
            model_name: draft.model_name.clone(),
            source_id: draft.source_id,
            base_library_revision: draft.base_library_revision,
            base_source_revision: draft.base_source_revision,
            base_source_digest: draft.base_source_digest,
            source_path: draft.source_path.clone(),
            base_project_revision: draft.base_project_revision,
            name: draft.name.clone(),
            spice_type: draft.spice_type.clone(),
            description: draft.description.clone(),
            parameters: draft.parameters.clone(),
            metadata: draft.metadata.clone(),
        }
    }

    pub fn restore(
        self,
        manager: &ModelLibraryManager,
        project_revision: ObjectRevision,
    ) -> Result<ModelEditorDraft, String> {
        if project_revision != self.base_project_revision {
            return Err(format!(
                "project revision changed from r{} to r{}",
                self.base_project_revision.get(),
                project_revision.get()
            ));
        }
        let base = ModelEditorDraft::open(
            manager,
            &self.library_name,
            &self.model_name,
            project_revision,
        )?;
        if base.source_id != self.source_id
            || base.base_library_revision != self.base_library_revision
            || base.base_source_revision != self.base_source_revision
            || base.base_source_digest != self.base_source_digest
            || base.source_path != self.source_path
        {
            return Err(
                "the project model source no longer matches the recovered candidate authority"
                    .to_owned(),
            );
        }
        Ok(ModelEditorDraft {
            library_name: base.library_name,
            model_name: base.model_name,
            source_id: base.source_id,
            base_library_revision: base.base_library_revision,
            base_source_revision: base.base_source_revision,
            base_source_digest: base.base_source_digest,
            source_path: base.source_path,
            base_project_revision: base.base_project_revision,
            name: self.name,
            spice_type: self.spice_type,
            description: self.description,
            parameters: self.parameters,
            metadata: self.metadata,
            base_definition: base.base_definition,
        })
    }
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
            base_definition,
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
    pub fn is_dirty(&self) -> bool {
        self.definition_is_dirty()
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
}

#[derive(Debug, Clone, Default)]
pub struct ModelEditorState {
    pub active_section: ModelEditorSection,
    pub selected_parameter: Option<usize>,
    pub draft: Option<ModelEditorDraft>,
    pub diagnostics: Vec<ModelFieldDiagnostic>,
    pub validation: Option<ModelValidationEvidence>,
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
}

#[cfg(test)]
mod tests;
