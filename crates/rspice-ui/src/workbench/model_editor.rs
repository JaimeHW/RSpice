//! Transactional presentation state for the governed device-model editor.
//!
//! Committed source bytes and model metadata remain owned by the project
//! model-library domain. This module retains only an open candidate, local
//! text-entry buffers, validation evidence, and section/dialog presentation.

use std::collections::{BTreeMap, BTreeSet};

use crate::common::RSpiceApp;
use crate::common::app::ConsoleMessage;
use crate::product::{ContentDigest, ModelSourceId, ObjectRevision};
use crate::state::model_library::{
    FiniteF64, ModelDefinitionMetadata, ModelLibraryManager, ModelQualificationState,
    ModelSourceAuthority, ParameterDataType, ParameterDefinition, ParameterSource, ParameterValue,
    ProjectModelDefinition, ProjectModelRevisionDefinition,
};

use super::{RouteTransitionSource, SurfaceId, SurfaceRoute};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelParameterDraft {
    pub name: String,
    pub kind: ModelParameterKind,
    pub value: String,
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

impl ModelDefinitionDelta {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        !self.identity_changed
            && !self.description_changed
            && self.added_parameters.is_empty()
            && self.removed_parameters.is_empty()
            && self.changed_parameters.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct ModelEditorDraft {
    pub library_name: String,
    pub model_name: String,
    pub source_id: ModelSourceId,
    pub base_source_revision: ObjectRevision,
    pub base_source_digest: ContentDigest,
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
        let library = manager
            .get_library(library_name)
            .ok_or_else(|| format!("Model library '{library_name}' does not exist"))?;
        let ModelSourceAuthority::ProjectOwned {
            source_id,
            revision,
            digest,
        } = library.source_authority
        else {
            return Err(format!(
                "Model '{model_name}' is not project-owned; create an editable project copy before opening the model editor"
            ));
        };
        if library.models.len() != 1
            || library.source_closure.len() != 1
            || library.source_contents.len() != 1
            || !library.source_edges.is_empty()
        {
            return Err(format!(
                "Model library '{library_name}' is not one coherent editable project-model revision"
            ));
        }
        let model = library.models.get(model_name).ok_or_else(|| {
            format!("Model '{model_name}' does not exist in library '{library_name}'")
        })?;
        let base_card = ProjectModelDefinition::from_device_model(model);
        let metadata = library
            .model_definition_metadata
            .get(model_name)
            .cloned()
            .ok_or_else(|| {
                format!("Model '{model_name}' has no typed project-owned definition metadata")
            })?;
        let qualification = library
            .model_qualification
            .get(model_name)
            .cloned()
            .unwrap_or_default();
        let base_definition = ProjectModelRevisionDefinition::new(base_card, metadata.clone());
        let canonical_source = base_definition
            .canonical_source()
            .map_err(|error| format!("Retained model revision is invalid: {error}"))?;
        if library.source_contents[0].bytes != canonical_source.as_bytes() {
            return Err(format!(
                "Model library '{library_name}' retained bytes do not match its typed definition"
            ));
        }
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
            base_source_revision: revision,
            base_source_digest: digest,
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
                            &parameter.name,
                            ParameterDataType::Numeric,
                            ParameterValue::Numeric(
                                FiniteF64::new(value).expect("finite value was checked above"),
                            ),
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
                        &parameter.name,
                        ParameterDataType::String,
                        ParameterValue::String(parameter.value.clone()),
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
        name: &str,
        data_type: ParameterDataType,
        value: ParameterValue,
    ) -> ParameterDefinition {
        let existing = self
            .metadata
            .parameters
            .iter()
            .find(|parameter| parameter.name.eq_ignore_ascii_case(name));
        ParameterDefinition {
            name: name.to_owned(),
            data_type,
            value,
            unit: existing.and_then(|parameter| parameter.unit.clone()),
            bounds: existing.and_then(|parameter| parameter.bounds),
            source: existing.map_or_else(
                || ParameterSource::Declared {
                    source: "project model source".to_owned(),
                },
                |parameter| parameter.source.clone(),
            ),
            description: existing.map_or_else(
                || format!("Project-owned {name} model parameter"),
                |parameter| parameter.description.clone(),
            ),
        }
    }

    #[must_use]
    pub fn is_dirty(&self) -> bool {
        self.definition().map_or(true, |candidate| {
            candidate != self.base_definition || self.qualification != self.base_qualification
        })
    }

    pub fn validate(
        &self,
        manager: &ModelLibraryManager,
        project_revision: ObjectRevision,
    ) -> Result<ModelValidationEvidence, Vec<ModelFieldDiagnostic>> {
        let definition = self.definition()?;
        let Some(library) = manager.get_library(&self.library_name) else {
            return Err(vec![ModelFieldDiagnostic {
                field: "source".to_owned(),
                message: format!("Model library '{}' was removed", self.library_name),
            }]);
        };
        let authority_matches = matches!(
            library.source_authority,
            ModelSourceAuthority::ProjectOwned {
                source_id,
                revision,
                digest,
            } if source_id == self.source_id
                && revision == self.base_source_revision
                && digest == self.base_source_digest
        );
        if !authority_matches || project_revision != self.base_project_revision {
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
        let base = &self.base_definition.base;
        let candidate = &candidate.base;
        let base_values = parameter_values(base);
        let candidate_values = parameter_values(candidate);
        let added_parameters = candidate_values
            .keys()
            .filter(|name| !base_values.contains_key(*name))
            .cloned()
            .collect();
        let removed_parameters = base_values
            .keys()
            .filter(|name| !candidate_values.contains_key(*name))
            .cloned()
            .collect();
        let changed_parameters = candidate_values
            .iter()
            .filter_map(|(name, value)| {
                base_values
                    .get(name)
                    .is_some_and(|base_value| base_value != value)
                    .then_some(name.clone())
            })
            .collect();
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

fn parameter_values(definition: &ProjectModelDefinition) -> BTreeMap<String, String> {
    definition
        .numeric_parameters
        .iter()
        .map(|(name, value)| (name.to_ascii_lowercase(), format!("number:{value}")))
        .chain(
            definition
                .string_parameters
                .iter()
                .map(|(name, value)| (name.to_ascii_lowercase(), format!("string:{value}"))),
        )
        .collect()
}

#[derive(Debug, Clone, Default)]
pub struct ModelEditorState {
    pub active_section: ModelEditorSection,
    pub parameter_filter: String,
    pub selected_parameter: Option<usize>,
    pub draft: Option<ModelEditorDraft>,
    pub diagnostics: Vec<ModelFieldDiagnostic>,
    pub validation: Option<ModelValidationEvidence>,
    pub comparison_open: bool,
    pub parameter_schema_open: bool,
    pub new_section_open: bool,
    pub correlation_matrix_open: bool,
    pub temperature_preview_open: bool,
    pub qualification_plan_open: bool,
    pub promotion_review_open: bool,
    pub close_review_open: bool,
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

/// Open an exact project-owned model revision in the transactional editor.
pub fn open_project_model(
    app: &mut RSpiceApp,
    library_name: &str,
    model_name: &str,
) -> Result<(), String> {
    let mut editor = ModelEditorState::default();
    editor.open(
        &app.state.model_library_manager,
        library_name,
        model_name,
        app.state.workspace.project.revision(),
    )?;
    app.state
        .workbench
        .navigate(
            SurfaceRoute::surface(SurfaceId::ModelEditor),
            RouteTransitionSource::User,
        )
        .map_err(|error| error.to_string())?;
    app.state.workbench.model_editor = editor;
    app.state.model_library_manager.select_library(library_name);
    app.state.workbench.selected_model = Some(model_name.to_owned());
    Ok(())
}

/// Validate the open model draft and bind the resulting evidence to its exact
/// model-source and project revisions.
pub fn validate_open_candidate(app: &mut RSpiceApp) -> bool {
    let valid = app.state.workbench.model_editor.validate_candidate(
        &app.state.model_library_manager,
        app.state.workspace.project.revision(),
    );
    let message = if valid {
        ConsoleMessage::info("Model candidate is valid and bound to the current source revision.")
    } else {
        let summary = app
            .state
            .workbench
            .model_editor
            .diagnostics
            .first()
            .map_or("Model candidate is invalid.", |diagnostic| {
                diagnostic.message.as_str()
            });
        ConsoleMessage::warning(format!("Model candidate validation failed: {summary}"))
    };
    app.state.push_user_message(message);
    valid
}

/// Publish the open candidate as one guarded project/model revision and then
/// reopen the committed revision as the new immutable editing base.
pub fn save_open_candidate(app: &mut RSpiceApp) -> Result<ObjectRevision, String> {
    if !app.state.project_lifecycle.project_open {
        return Err("Model revision cannot be saved without an open project".to_owned());
    }
    if app.state.workbench.safe_mode.project_read_only() {
        return Err("Model revision cannot be saved while the project is read-only".to_owned());
    }
    let draft = app
        .state
        .workbench
        .model_editor
        .draft
        .clone()
        .ok_or_else(|| "No project-owned model candidate is open".to_owned())?;
    if !draft.is_dirty() {
        return Err("Model candidate has no semantic changes to save".to_owned());
    }
    if !validate_open_candidate(app) {
        return Err("Model revision was not saved because validation failed".to_owned());
    }
    let definition = draft.definition().map_err(|diagnostics| {
        diagnostics
            .into_iter()
            .map(|diagnostic| format!("{}: {}", diagnostic.field, diagnostic.message))
            .collect::<Vec<_>>()
            .join("; ")
    })?;
    let mut candidate = app.state.model_library_manager.clone();
    let commit = candidate.replace_project_model_revision(
        &draft.library_name,
        draft.source_id,
        draft.base_source_revision,
        &definition,
        &draft.qualification,
    )?;
    let description = format!(
        "save model revision {}/{}",
        commit.library_name, commit.model_name
    );
    let committed_revision =
        app.state
            .publish_project_model_candidate(candidate, commit, description)?;
    app.invalidate_simulation_preflight();
    app.state
        .model_library_manager
        .select_library(&draft.library_name);
    app.state.workbench.selected_model = Some(definition.base.name.clone());
    app.state.workbench.model_editor.open(
        &app.state.model_library_manager,
        &draft.library_name,
        &definition.base.name,
        committed_revision,
    )?;
    app.state
        .workbench
        .model_editor
        .validate_candidate(&app.state.model_library_manager, committed_revision);
    app.state.push_user_message(ConsoleMessage::info(format!(
        "Saved model revision {}/{} at project revision {}.",
        draft.library_name,
        definition.base.name,
        committed_revision.get()
    )));
    Ok(committed_revision)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn definition() -> ProjectModelDefinition {
        ProjectModelDefinition {
            name: "nch_owned".to_owned(),
            spice_type: "NMOS".to_owned(),
            description: "Project model".to_owned(),
            numeric_parameters: BTreeMap::from([
                ("level".to_owned(), 1.0),
                ("vth0".to_owned(), 0.48),
            ]),
            string_parameters: BTreeMap::from([("version_tag".to_owned(), "r1".to_owned())]),
        }
    }

    fn opened_editor() -> (ModelLibraryManager, ModelEditorState) {
        let mut manager = ModelLibraryManager::new();
        manager
            .create_project_model("owned-models", &definition())
            .expect("create project model");
        let mut editor = ModelEditorState::default();
        editor
            .open(
                &manager,
                "owned-models",
                "nch_owned",
                ObjectRevision::INITIAL,
            )
            .expect("open editor");
        (manager, editor)
    }

    #[test]
    fn editor_opens_only_exact_project_owned_single_card_models() {
        let (mut manager, editor) = opened_editor();
        assert!(!editor.draft.as_ref().expect("draft").is_dirty());

        manager.add_library(crate::state::model_library::ModelLibrary::new("built-in"));
        let mut rejected = ModelEditorState::default();
        assert!(
            rejected
                .open(&manager, "built-in", "missing", ObjectRevision::INITIAL)
                .expect_err("built-in source is read-only")
                .contains("not project-owned")
        );
    }

    #[test]
    fn editor_validation_is_revision_bound_and_reports_typed_input_errors() {
        let (mut manager, mut editor) = opened_editor();
        assert!(editor.validate_candidate(&manager, ObjectRevision::INITIAL));
        let evidence = editor.validation.expect("validation evidence");

        let draft = editor.draft.as_mut().expect("draft");
        draft
            .parameters
            .iter_mut()
            .find(|parameter| parameter.name == "vth0")
            .expect("parameter")
            .value = "not-a-number".to_owned();
        editor.invalidate_candidate_evidence();
        assert!(!editor.validate_candidate(&manager, ObjectRevision::INITIAL));
        assert!(editor.diagnostics[0].field.ends_with(".value"));

        editor
            .open(
                &manager,
                "owned-models",
                "nch_owned",
                ObjectRevision::INITIAL,
            )
            .expect("reopen editor");
        let draft = editor.draft.as_ref().expect("draft");
        manager
            .replace_project_model(
                "owned-models",
                draft.source_id,
                draft.base_source_revision,
                &ProjectModelDefinition {
                    description: "Concurrent change".to_owned(),
                    ..definition()
                },
            )
            .expect("advance model source");
        assert!(!editor.validate_candidate(&manager, ObjectRevision::INITIAL));
        assert!(editor.diagnostics[0].message.contains("changed after"));
        assert_ne!(
            evidence.source_revision,
            ObjectRevision::new(2).expect("revision")
        );
    }

    #[test]
    fn semantic_delta_is_deterministic_across_numeric_and_string_parameters() {
        let (_, mut editor) = opened_editor();
        let draft = editor.draft.as_mut().expect("draft");
        draft.name = "nch_candidate".to_owned();
        draft
            .parameters
            .iter_mut()
            .find(|parameter| parameter.name == "vth0")
            .expect("parameter")
            .value = "0.5".to_owned();
        draft
            .parameters
            .retain(|parameter| parameter.name != "version_tag");
        draft.parameters.push(ModelParameterDraft {
            name: "pclm".to_owned(),
            kind: ModelParameterKind::Numeric,
            value: "1.1".to_owned(),
        });

        let delta = draft.delta().expect("valid delta");
        assert!(delta.identity_changed);
        assert_eq!(delta.added_parameters, ["pclm"]);
        assert_eq!(delta.removed_parameters, ["version_tag"]);
        assert_eq!(delta.changed_parameters, ["vth0"]);
    }

    #[test]
    fn save_controller_publishes_once_and_reopens_the_committed_revision() {
        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.project_open = true;
        app.state
            .model_library_manager
            .create_project_model("owned-models", &definition())
            .expect("create project model fixture");
        let starting_project_revision = app.state.workspace.project.revision();
        let starting_execution_epoch = app.state.design_execution_epoch;
        app.state
            .workbench
            .model_editor
            .open(
                &app.state.model_library_manager,
                "owned-models",
                "nch_owned",
                starting_project_revision,
            )
            .expect("open editor");
        app.state
            .workbench
            .model_editor
            .draft
            .as_mut()
            .expect("draft")
            .parameters
            .iter_mut()
            .find(|parameter| parameter.name == "vth0")
            .expect("parameter")
            .value = "0.51".to_owned();

        let committed_project_revision =
            save_open_candidate(&mut app).expect("publish model revision");
        assert_eq!(
            committed_project_revision,
            starting_project_revision.next().expect("project revision")
        );
        assert!(app.state.workspace.project_metadata_dirty);
        assert_eq!(
            app.state.design_execution_epoch,
            starting_execution_epoch.wrapping_add(1)
        );
        let library = app
            .state
            .model_library_manager
            .get_library("owned-models")
            .expect("library");
        assert_eq!(
            library.project_source_revision(),
            Some(ObjectRevision::new(2).expect("source revision"))
        );
        assert_eq!(library.models["nch_owned"].parameters["vth0"], 0.51);
        let reopened = app
            .state
            .workbench
            .model_editor
            .draft
            .as_ref()
            .expect("reopened draft");
        assert_eq!(reopened.base_project_revision, committed_project_revision);
        assert_eq!(
            reopened.base_source_revision,
            ObjectRevision::new(2).expect("source revision")
        );
        assert!(!reopened.is_dirty());
        assert!(app.state.workbench.model_editor.validation.is_some());
    }

    #[test]
    fn sectioned_editor_publishes_metadata_only_revision_without_losing_execution_sections() {
        let mut seed = ModelLibraryManager::new();
        seed.create_project_model("seed", &definition())
            .expect("create metadata seed");
        let metadata = seed
            .get_library("seed")
            .expect("seed library")
            .model_definition_metadata["nch_owned"]
            .clone();
        let mut revision = ProjectModelRevisionDefinition::new(definition(), metadata);
        revision
            .metadata
            .sections
            .push(crate::state::model_library::ModelSectionDefinition {
                name: "TT".to_owned(),
                parent: None,
                overrides: BTreeMap::from([(
                    "vth0".to_owned(),
                    ParameterValue::Numeric(FiniteF64::new(0.5).expect("finite fixture")),
                )]),
                model_files: Vec::new(),
                qualification: crate::state::model_library::ModelSectionQualification::Unqualified,
            });

        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.project_open = true;
        app.state
            .model_library_manager
            .create_project_model_revision(
                "sectioned",
                &revision,
                &ModelQualificationState::default(),
            )
            .expect("create sectioned revision");
        let project_revision = app.state.workspace.project.revision();
        app.state
            .workbench
            .model_editor
            .open(
                &app.state.model_library_manager,
                "sectioned",
                "nch_owned",
                project_revision,
            )
            .expect("open sectioned editor");
        assert!(
            !app.state
                .workbench
                .model_editor
                .draft
                .as_ref()
                .unwrap()
                .is_dirty()
        );
        app.state
            .workbench
            .model_editor
            .draft
            .as_mut()
            .unwrap()
            .metadata
            .parameters
            .iter_mut()
            .find(|parameter| parameter.name == "vth0")
            .unwrap()
            .unit = Some("V".to_owned());

        save_open_candidate(&mut app).expect("save metadata-only revision");
        let library = app
            .state
            .model_library_manager
            .get_library("sectioned")
            .expect("saved sectioned library");
        assert_eq!(library.selected_corner.as_deref(), Some("TT"));
        assert_eq!(library.corners.len(), 1);
        assert_eq!(
            library.model_definition_metadata["nch_owned"]
                .parameters
                .iter()
                .find(|parameter| parameter.name == "vth0")
                .and_then(|parameter| parameter.unit.as_deref()),
            Some("V")
        );
        let cards = app
            .state
            .model_library_manager
            .reference_process_model_cards(crate::simulation::dialog::corner::ProcessCorner::TT)
            .expect("materialize saved section")
            .join("\n");
        assert!(cards.contains("VTH0=0.5"), "{cards}");
    }
}
