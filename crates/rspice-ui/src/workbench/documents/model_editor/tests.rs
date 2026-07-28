//! Tests for the editor document's atomicity guarantees.
//!
//! The recurring assertion is completeness: a qualification pass publishes
//! only a whole platform run, and candidate creation and promotion either
//! happen entirely or not at all.

use super::*;

fn definition() -> ProjectModelDefinition {
    ProjectModelDefinition {
        name: "nch_owned".to_owned(),
        spice_type: "NMOS".to_owned(),
        description: "Project model".to_owned(),
        numeric_parameters: BTreeMap::from([("level".to_owned(), 1.0), ("vth0".to_owned(), 0.48)]),
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

fn multi_model_include_library() -> (ModelLibraryManager, PathBuf, PathBuf, Vec<u8>, Vec<u8>) {
    use crate::state::model_library::{ModelSourceContent, ModelSourceEdge, ModelSourcePin};

    let mut first_manager = ModelLibraryManager::new();
    first_manager
        .create_project_model("multi-owned", &definition())
        .expect("create first model");
    let second_definition = ProjectModelDefinition {
        name: "pch_owned".to_owned(),
        spice_type: "PMOS".to_owned(),
        description: "Included project model".to_owned(),
        numeric_parameters: BTreeMap::from([("level".to_owned(), 1.0), ("vth0".to_owned(), -0.51)]),
        string_parameters: BTreeMap::new(),
    };
    let mut second_manager = ModelLibraryManager::new();
    second_manager
        .create_project_model("second", &second_definition)
        .expect("create included model");
    let second_library = second_manager
        .get_library("second")
        .expect("included library");

    let mut library = first_manager
        .get_library("multi-owned")
        .expect("first library")
        .clone();
    let root = library.root_path.clone().expect("root");
    let child = root.with_file_name("included.model");
    let first_source = library.source_contents[0].bytes.clone();
    let second_source = second_library.source_contents[0].bytes.clone();
    let mut root_source = first_source.clone();
    root_source.extend_from_slice(b".include \"included.model\"\n");
    let root_digest = ContentDigest::from_bytes(Sha256::digest(root_source.as_slice()).into());
    let child_digest = ContentDigest::from_bytes(Sha256::digest(second_source.as_slice()).into());
    let ModelSourceAuthority::ProjectOwned {
        source_id,
        revision,
        ..
    } = library.source_authority
    else {
        panic!("first source is project-owned")
    };
    library.source_authority = ModelSourceAuthority::ProjectOwned {
        source_id,
        revision,
        digest: root_digest,
    };
    library.source_closure = vec![
        ModelSourcePin {
            path: root.clone(),
            digest: root_digest,
        },
        ModelSourcePin {
            path: child.clone(),
            digest: child_digest,
        },
    ];
    library.source_contents = vec![
        ModelSourceContent {
            path: root.clone(),
            bytes: root_source,
        },
        ModelSourceContent {
            path: child.clone(),
            bytes: second_source.clone(),
        },
    ];
    library.source_edges = vec![ModelSourceEdge {
        owner: root.clone(),
        requested_path: "included.model".to_owned(),
        target: child.clone(),
    }];
    let mut second_model = second_library.models["pch_owned"].clone();
    second_model.file_path = Some(child.clone());
    library.models.insert("pch_owned".to_owned(), second_model);
    let mut second_metadata = second_library.model_definition_metadata["pch_owned"].clone();
    second_metadata.source_identity = Some(ModelFileIdentity {
        source_id: source_id.to_string(),
        revision: revision.get(),
        content_digest: child_digest.to_string(),
        display_name: "included.model".to_owned(),
    });
    library
        .model_definition_metadata
        .insert("pch_owned".to_owned(), second_metadata);
    first_manager.add_library(library);
    (first_manager, root, child, first_source, second_source)
}

#[test]
fn editor_opens_only_exact_project_owned_models() {
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
fn editor_opens_a_specific_model_from_an_authenticated_include_closure() {
    let (manager, _root, child, _first_source, _second_source) = multi_model_include_library();
    let resolved = resolve_project_model_for_editor(&manager, "multi-owned", "pch_owned")
        .expect("resolve included model");
    assert_eq!(resolved.source_path, child);
    assert_eq!(resolved.definition.base.name, "pch_owned");

    let mut editor = ModelEditorState::default();
    editor
        .open(
            &manager,
            "multi-owned",
            "pch_owned",
            ObjectRevision::INITIAL,
        )
        .expect("open included model");
    assert_eq!(
        editor.draft.as_ref().expect("draft").model_name,
        "pch_owned"
    );
}

#[test]
fn selected_fragment_save_preserves_adjacent_models_and_include_graph() {
    let (mut manager, root, child, first_source, second_source) = multi_model_include_library();
    let resolved = resolve_project_model_for_editor(&manager, "multi-owned", "pch_owned")
        .expect("resolve included model");
    let before = manager
        .get_library("multi-owned")
        .expect("library before")
        .clone();
    let mut changed = resolved.definition.clone();
    changed.base.description = "Revised included project model".to_owned();
    let commit = manager
        .replace_project_model_revision_in_library(
            "multi-owned",
            resolved.source_id,
            resolved.library_revision,
            resolved.model_revision,
            "pch_owned",
            resolved.model_digest,
            &changed,
            &resolved.qualification,
        )
        .expect("replace exact included model fragment");
    assert_eq!(commit.after.source_edges, before.source_edges);
    assert_eq!(commit.after.models["nch_owned"], before.models["nch_owned"]);
    assert_eq!(
        commit.after.model_definition_metadata["nch_owned"],
        before.model_definition_metadata["nch_owned"]
    );
    let root_after = commit
        .after
        .source_contents
        .iter()
        .find(|content| content.path == root)
        .expect("root after");
    let mut expected_root = first_source;
    expected_root.extend_from_slice(b".include \"included.model\"\n");
    assert_eq!(root_after.bytes, expected_root);
    let child_after = commit
        .after
        .source_contents
        .iter()
        .find(|content| content.path == child)
        .expect("child after");
    assert_ne!(child_after.bytes, second_source);
    assert!(
        child_after
            .bytes
            .windows(b"Revised included project model".len())
            .any(|bytes| bytes == b"Revised included project model")
    );
    let reopened = resolve_project_model_for_editor(&manager, "multi-owned", "pch_owned")
        .expect("reopen exact revised model");
    assert_eq!(
        reopened.definition.base.description,
        "Revised included project model"
    );
    assert_eq!(
        reopened.library_revision,
        ObjectRevision::new(2).expect("library revision 2")
    );
    assert_eq!(
        reopened.model_revision,
        ObjectRevision::new(2).expect("model revision 2")
    );
}

#[test]
fn editing_one_model_preserves_a_qualified_sectioned_sibling_revision() {
    let (mut manager, _root, _child, _first_source, _second_source) = multi_model_include_library();

    let first = resolve_project_model_for_editor(&manager, "multi-owned", "nch_owned")
        .expect("resolve first model");
    let mut sectioned = first.definition.clone();
    sectioned.metadata.sections.push(ModelSectionDefinition {
        name: "TT".to_owned(),
        parent: None,
        overrides: BTreeMap::new(),
        model_files: vec![
            sectioned
                .metadata
                .source_identity
                .clone()
                .expect("model source identity"),
        ],
        qualification: ModelSectionQualification::Unqualified,
    });
    manager
        .replace_project_model_revision_in_library(
            "multi-owned",
            first.source_id,
            first.library_revision,
            first.model_revision,
            "nch_owned",
            first.model_digest,
            &sectioned,
            &first.qualification,
        )
        .expect("publish sectioned first model");

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    app.state.model_library_manager = manager;
    let project_revision = app.state.workspace.project.revision();
    app.state
        .workbench
        .model_editor
        .open(
            &app.state.model_library_manager,
            "multi-owned",
            "nch_owned",
            project_revision,
        )
        .expect("open sectioned sibling");
    populate_operating_point_suite(&mut app.state.workbench.model_editor);
    app.state
        .workbench
        .model_editor
        .qualification_authoring
        .model_section = "TT".to_owned();
    assert!(
        app.state
            .workbench
            .model_editor
            .commit_qualification_suite()
    );
    start_qualification_execution(&mut app).expect("start desktop qualification");
    advance_qualification_execution(&mut app);

    let draft = app
        .state
        .workbench
        .model_editor
        .draft
        .as_mut()
        .expect("qualified sibling draft");
    let desktop = draft.qualification.platform_runs[0].clone();
    let mut webassembly = desktop.clone();
    webassembly.platform = QualificationPlatform::WebAssembly;
    for vector in &mut webassembly.vector_outcomes {
        vector.outcome.platform = QualificationPlatform::WebAssembly;
    }
    draft
        .qualification
        .upsert_platform_run_atomically(webassembly)
        .expect("retain WebAssembly parity run");
    let source = desktop.source.clone();
    let evidence = draft
        .qualification
        .assemble_and_upsert_evidence_atomically("tt-evidence", "dc-op", &source)
        .expect("assemble exact section evidence");
    let evidence_digest = evidence.content_digest().expect("evidence digest");
    draft
        .qualification
        .validate_exact_section_evidence_digest(&source, "TT", evidence_digest)
        .expect("exact TT evidence");
    draft.metadata.sections[0].qualification = ModelSectionQualification::Qualified {
        evidence_digest: Some(evidence_digest.to_string()),
    };
    let sibling_metadata = draft.metadata.clone();
    let sibling_qualification = draft.qualification.clone();
    let sibling_revision = draft.base_source_revision;
    let sibling_digest = draft.base_source_digest;
    let sibling_source_id = draft.source_id;
    let library = app
        .state
        .model_library_manager
        .get_library_mut("multi-owned")
        .expect("multi-model library");
    library
        .model_definition_metadata
        .insert("nch_owned".to_owned(), sibling_metadata.clone());
    library
        .model_qualification
        .insert("nch_owned".to_owned(), sibling_qualification.clone());

    let mut manager = app.state.model_library_manager.clone();
    let second = resolve_project_model_for_editor(&manager, "multi-owned", "pch_owned")
        .expect("resolve editable sibling");
    let mut changed_second = second.definition.clone();
    changed_second.base.description = "Edited without touching qualified sibling".to_owned();
    manager
        .replace_project_model_revision_in_library(
            "multi-owned",
            second.source_id,
            second.library_revision,
            second.model_revision,
            "pch_owned",
            second.model_digest,
            &changed_second,
            &second.qualification,
        )
        .expect("edit only second model");

    let retained = resolve_project_model_for_editor(&manager, "multi-owned", "nch_owned")
        .expect("qualified sibling remains resolvable");
    assert_eq!(retained.source_id, sibling_source_id);
    assert_eq!(retained.model_revision, sibling_revision);
    assert_eq!(retained.model_digest, sibling_digest);
    assert_eq!(retained.definition.metadata, sibling_metadata);
    assert_eq!(retained.qualification, sibling_qualification);
    assert!(retained.library_revision > first.library_revision);
    let retained_source = ModelSourceEvidenceBinding::try_new_project_bound(
        "nch_owned",
        retained.source_id,
        retained.model_digest,
        retained.model_revision,
    )
    .expect("retained source binding");
    retained
        .qualification
        .validate_exact_section_evidence_digest(&retained_source, "TT", evidence_digest)
        .expect("untouched qualified section evidence remains exact");
}

#[test]
fn opening_a_different_model_never_replaces_an_unsaved_candidate() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    app.state
        .model_library_manager
        .create_project_model("first-owned-model", &definition())
        .expect("create first model");
    let second = ProjectModelDefinition {
        name: "pch_owned".to_owned(),
        spice_type: "PMOS".to_owned(),
        description: "Second project model".to_owned(),
        ..definition()
    };
    app.state
        .model_library_manager
        .create_project_model("second-owned-model", &second)
        .expect("create second model");
    app.state
        .workbench
        .model_editor
        .open(
            &app.state.model_library_manager,
            "first-owned-model",
            "nch_owned",
            app.state.workspace.project.revision(),
        )
        .expect("open first candidate");
    app.state
        .workbench
        .model_editor
        .draft
        .as_mut()
        .expect("first draft")
        .description = "Unsaved first-model description".to_owned();

    let error = open_project_model(&mut app, "second-owned-model", "pch_owned")
        .expect_err("dirty candidate replacement must be blocked");
    assert_eq!(
        error,
        "Unsaved model candidate 'first-owned-model/nch_owned' is open; save or discard it before opening 'second-owned-model/pch_owned'"
    );
    let retained = app
        .state
        .workbench
        .model_editor
        .draft
        .as_ref()
        .expect("first draft retained");
    assert_eq!(retained.library_name, "first-owned-model");
    assert_eq!(retained.model_name, "nch_owned");
    assert_eq!(retained.description, "Unsaved first-model description");

    open_project_model(&mut app, "first-owned-model", "nch_owned")
        .expect("reselecting the same dirty candidate retains it");
    assert_eq!(
        app.state
            .workbench
            .model_editor
            .draft
            .as_ref()
            .expect("same draft retained")
            .description,
        "Unsaved first-model description"
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
        unit: "1/V".to_owned(),
        lower_bound: "0".to_owned(),
        upper_bound: "10".to_owned(),
        description: "Channel-length modulation".to_owned(),
    });

    let delta = draft.delta().expect("valid delta");
    assert!(delta.identity_changed);
    assert_eq!(delta.added_parameters, ["pclm"]);
    assert_eq!(delta.removed_parameters, ["version_tag"]);
    assert_eq!(delta.changed_parameters, ["vth0"]);
}

#[test]
fn definition_and_qualification_dirty_state_are_independent() {
    let (_, mut editor) = opened_editor();
    let draft = editor.draft.as_mut().expect("draft");
    assert!(!draft.definition_is_dirty());
    assert!(!draft.qualification_is_dirty());
    assert!(!draft.is_dirty());

    draft.qualification.schema_version = draft.qualification.schema_version.wrapping_add(1);
    assert!(!draft.definition_is_dirty());
    assert!(draft.qualification_is_dirty());
    assert!(draft.is_dirty());

    draft.qualification = draft.base_qualification.clone();
    draft
        .parameters
        .iter_mut()
        .find(|parameter| parameter.name == "vth0")
        .expect("parameter")
        .value = "0.5".to_owned();
    assert!(draft.definition_is_dirty());
    assert!(!draft.qualification_is_dirty());
    assert!(draft.is_dirty());
}

#[test]
fn parameter_schema_fields_are_validated_and_projected_into_the_saved_definition() {
    let (_, mut editor) = opened_editor();
    let draft = editor.draft.as_mut().expect("draft");
    let parameter = draft
        .parameters
        .iter_mut()
        .find(|parameter| parameter.name == "vth0")
        .expect("parameter");
    parameter.unit = "V".to_owned();
    parameter.lower_bound = "-0.25".to_owned();
    parameter.upper_bound = "1.25".to_owned();
    parameter.description = "Threshold voltage".to_owned();

    let definition = draft.definition().expect("valid typed schema");
    let parameter = definition
        .metadata
        .parameters
        .iter()
        .find(|parameter| parameter.name == "vth0")
        .expect("typed parameter");
    assert_eq!(parameter.unit.as_deref(), Some("V"));
    assert_eq!(
        parameter.bounds,
        Some(FiniteBounds {
            lower: Some(FiniteF64::new(-0.25).expect("finite lower")),
            upper: Some(FiniteF64::new(1.25).expect("finite upper")),
        })
    );
    assert_eq!(parameter.description, "Threshold voltage");

    draft
        .parameters
        .iter_mut()
        .find(|parameter| parameter.name == "vth0")
        .expect("parameter")
        .upper_bound = "not-a-number".to_owned();
    let diagnostics = draft.definition().expect_err("invalid bound is blocked");
    assert!(
        diagnostics.iter().any(|diagnostic| {
            diagnostic.field.ends_with(".upper_bound")
                && diagnostic.message.contains("Invalid numeric bound")
        }),
        "{diagnostics:?}"
    );
}

#[test]
fn semantic_delta_marks_parameter_schema_and_type_changes() {
    let (_, mut editor) = opened_editor();
    let draft = editor.draft.as_mut().expect("draft");
    let parameter = draft
        .parameters
        .iter_mut()
        .find(|parameter| parameter.name == "vth0")
        .expect("numeric parameter");
    parameter.unit = "V".to_owned();
    parameter.lower_bound = "0.1".to_owned();
    parameter.upper_bound = "0.9".to_owned();
    parameter.description = "Reviewed threshold-voltage schema".to_owned();

    let delta = draft.delta().expect("valid schema delta");
    assert_eq!(delta.changed_parameters, ["vth0"]);

    let (_, mut editor) = opened_editor();
    let draft = editor.draft.as_mut().expect("draft");
    let parameter = draft
        .parameters
        .iter_mut()
        .find(|parameter| parameter.name == "version_tag")
        .expect("string parameter");
    parameter.kind = ModelParameterKind::Numeric;
    parameter.value = "2".to_owned();
    parameter.description = "Numeric model revision selector".to_owned();

    let delta = draft.delta().expect("valid type delta");
    assert_eq!(delta.changed_parameters, ["version_tag"]);
}

#[test]
fn inherited_value_edit_requires_an_explicit_target_section() {
    let (_, mut editor) = opened_editor();
    let draft = editor.draft.as_mut().expect("draft");
    let source_identity = draft
        .metadata
        .source_identity
        .clone()
        .expect("project-owned model identity");
    draft.metadata.sections.push(ModelSectionDefinition {
        name: "TT".to_owned(),
        parent: None,
        overrides: BTreeMap::new(),
        model_files: vec![source_identity],
        qualification: ModelSectionQualification::Unqualified,
    });
    draft
        .metadata
        .parameters
        .iter_mut()
        .find(|parameter| parameter.name == "vth0")
        .expect("typed parameter")
        .source = ParameterSource::Inherited {
        from_section: "TT".to_owned(),
    };
    assert!(
        draft.definition().is_ok(),
        "unchanged inherited value is valid"
    );

    draft
        .parameters
        .iter_mut()
        .find(|parameter| parameter.name == "vth0")
        .expect("draft parameter")
        .value = "0.5".to_owned();
    let diagnostics = draft
        .definition()
        .expect_err("implicit inherited-value override must fail closed");
    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic.field.ends_with(".value")
            && diagnostic
                .message
                .contains("cannot be edited without an explicit target section")
    }));
}

#[test]
fn explicit_override_value_edit_updates_its_section_delta() {
    let (_, mut editor) = opened_editor();
    let draft = editor.draft.as_mut().expect("draft");
    let original = ParameterValue::Numeric(FiniteF64::new(0.48).expect("finite original"));
    let source_identity = draft
        .metadata
        .source_identity
        .clone()
        .expect("project-owned model identity");
    draft.metadata.sections.push(ModelSectionDefinition {
        name: "TT".to_owned(),
        parent: None,
        overrides: BTreeMap::from([("vth0".to_owned(), original)]),
        model_files: vec![source_identity],
        qualification: ModelSectionQualification::Unqualified,
    });
    draft
        .metadata
        .parameters
        .iter_mut()
        .find(|parameter| parameter.name == "vth0")
        .expect("typed parameter")
        .source = ParameterSource::Overridden {
        section: "TT".to_owned(),
    };
    draft
        .parameters
        .iter_mut()
        .find(|parameter| parameter.name == "vth0")
        .expect("draft parameter")
        .value = "0.5".to_owned();

    let definition = draft.definition().expect("explicit override remains valid");
    assert_eq!(
        definition.metadata.sections[0].overrides["vth0"],
        ParameterValue::Numeric(FiniteF64::new(0.5).expect("finite edit"))
    );
}

#[test]
fn parameter_schema_authors_updates_and_removes_typed_section_overrides() {
    let (_, mut editor) = opened_editor();
    editor.begin_new_section();
    editor.new_section_name = "TT".to_owned();
    assert!(editor.commit_new_section());

    editor.begin_parameter_schema();
    editor.parameter_schema_section = "TT".to_owned();
    editor.parameter_schema_parameter = "vth0".to_owned();
    editor.refresh_parameter_schema_override_editor();
    editor.parameter_schema_override_value = "0.5".to_owned();
    assert!(editor.commit_parameter_schema_override());
    assert!(editor.parameter_schema_override_exists());
    assert_eq!(
        editor.draft.as_ref().unwrap().metadata.sections[0].overrides["vth0"],
        ParameterValue::Numeric(FiniteF64::new(0.5).expect("finite override"))
    );

    let retained = editor.draft.as_ref().unwrap().metadata.clone();
    editor.parameter_schema_override_value = "NaN".to_owned();
    assert!(!editor.commit_parameter_schema_override());
    assert_eq!(editor.draft.as_ref().unwrap().metadata, retained);
    assert!(
        editor
            .parameter_schema_override_error
            .as_deref()
            .is_some_and(|error| error.contains("finite"))
    );

    assert!(editor.remove_parameter_schema_override());
    assert!(!editor.parameter_schema_override_exists());
    assert!(
        editor.draft.as_ref().unwrap().metadata.sections[0]
            .overrides
            .is_empty()
    );
    assert!(editor.draft.as_ref().unwrap().definition().is_ok());
}

#[test]
fn correlation_edit_is_symmetric_transactional_and_fail_closed() {
    let (_, mut editor) = opened_editor();
    let draft = editor.draft.as_mut().expect("draft");
    draft.metadata.statistics.variables = vec![
        crate::state::model_library::StatisticalVariableDefinition {
            name: "level_variation".to_owned(),
            parameter: "level".to_owned(),
            distribution: crate::state::model_library::StatisticalDistribution::Normal {
                sigma: FiniteF64::new(0.1).expect("finite sigma"),
            },
            correlation_group: Some("process".to_owned()),
            hierarchy: crate::state::model_library::StatisticalHierarchyScope::Global,
            description: "Level variation".to_owned(),
        },
        crate::state::model_library::StatisticalVariableDefinition {
            name: "threshold_variation".to_owned(),
            parameter: "vth0".to_owned(),
            distribution: crate::state::model_library::StatisticalDistribution::Normal {
                sigma: FiniteF64::new(0.02).expect("finite sigma"),
            },
            correlation_group: Some("process".to_owned()),
            hierarchy: crate::state::model_library::StatisticalHierarchyScope::Global,
            description: "Threshold variation".to_owned(),
        },
    ];
    draft.metadata.statistics.correlation_matrices =
        vec![crate::state::model_library::CorrelationMatrix {
            group: "process".to_owned(),
            variables: vec![
                "level_variation".to_owned(),
                "threshold_variation".to_owned(),
            ],
            coefficients: vec![
                vec![
                    FiniteF64::new(1.0).expect("finite"),
                    FiniteF64::new(0.25).expect("finite"),
                ],
                vec![
                    FiniteF64::new(0.25).expect("finite"),
                    FiniteF64::new(1.0).expect("finite"),
                ],
            ],
        }];
    assert!(draft.definition().is_ok());

    editor.begin_correlation_matrix_edit();
    editor.correlation_matrix_edits[0][0][1] = "not-a-number".to_owned();
    assert!(!editor.commit_correlation_matrix_edit());
    assert_eq!(
        editor
            .draft
            .as_ref()
            .expect("draft")
            .metadata
            .statistics
            .correlation_matrices[0]
            .coefficients[0][1]
            .get(),
        0.25
    );

    editor.begin_correlation_matrix_edit();
    editor.correlation_matrix_edits[0][0][1] = "0.5".to_owned();
    editor.correlation_matrix_edits[0][1][0] = "0.5".to_owned();
    assert!(editor.commit_correlation_matrix_edit());
    let matrix = &editor
        .draft
        .as_ref()
        .expect("draft")
        .metadata
        .statistics
        .correlation_matrices[0];
    assert_eq!(matrix.coefficients[0][1].get(), 0.5);
    assert_eq!(matrix.coefficients[1][0].get(), 0.5);
    assert!(editor.validation.is_none());
}

#[test]
fn new_section_transaction_is_validated_and_rolls_back_on_duplicate_identity() {
    let (_, mut editor) = opened_editor();
    editor.begin_new_section();
    editor.new_section_name = "TT".to_owned();
    assert!(editor.commit_new_section());
    let draft = editor.draft.as_ref().expect("open draft");
    assert_eq!(draft.metadata.sections.len(), 1);
    assert_eq!(draft.metadata.sections[0].name, "TT");
    assert!(draft.is_dirty());

    editor.begin_new_section();
    editor.new_section_name = "tt".to_owned();
    assert!(!editor.commit_new_section());
    assert!(
        editor
            .new_section_error
            .as_deref()
            .is_some_and(|message| message.contains("duplicate case-insensitive"))
    );
    assert_eq!(
        editor
            .draft
            .as_ref()
            .expect("open draft")
            .metadata
            .sections
            .len(),
        1
    );
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

    let committed_project_revision = save_open_candidate(&mut app).expect("publish model revision");
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
        .create_project_model_revision("sectioned", &revision, &ModelQualificationState::default())
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
        .parameters
        .iter_mut()
        .find(|parameter| parameter.name == "vth0")
        .unwrap()
        .unit = "V".to_owned();

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

fn populate_operating_point_suite(editor: &mut ModelEditorState) {
    editor.begin_qualification_suite();
    let authoring = &mut editor.qualification_authoring;
    authoring.suite_id = "dc-op".to_owned();
    authoring.suite_name = "DC operating point".to_owned();
    authoring.vector_id = "nominal".to_owned();
    authoring.vector_name = "Nominal bias".to_owned();
    authoring.executable_input =
        "V1 out 0 1\nR1 out 0 1k\nMbind 0 0 0 0 nch_owned\n.op\n.end\n".to_owned();
    authoring.quantity = "v(out)".to_owned();
    authoring.probe_target = "out".to_owned();
    authoring.expected = "1".to_owned();
    authoring.absolute_tolerance = "1e-9".to_owned();
    authoring.relative_tolerance = "1e-9".to_owned();
}

#[test]
fn qualification_suite_authoring_is_atomic_and_source_bound() {
    let (_manager, mut editor) = opened_editor();
    populate_operating_point_suite(&mut editor);
    assert!(
        editor.commit_qualification_suite(),
        "{:?}",
        editor.qualification_authoring.error
    );
    let draft = editor.draft.as_ref().expect("draft");
    assert_eq!(draft.qualification.suites.len(), 1);
    let vector = &draft.qualification.suites[0].vectors[0];
    assert_eq!(vector.source.model_id, draft.model_name);
    assert_eq!(vector.source.source_digest, draft.base_source_digest);
    assert!(draft.qualification_is_dirty());

    populate_operating_point_suite(&mut editor);
    editor.qualification_authoring.vector_id = "nominal-second".to_owned();
    editor.qualification_authoring.vector_name = "Nominal bias second".to_owned();
    assert!(editor.commit_qualification_suite());
    let suite = &editor.draft.as_ref().expect("draft").qualification.suites[0];
    assert_eq!(suite.vectors.len(), 2);
    assert_eq!(suite.revision, ObjectRevision::new(2).expect("revision"));

    populate_operating_point_suite(&mut editor);
    editor.qualification_authoring.expected = "not-a-number".to_owned();
    assert!(!editor.commit_qualification_suite());
    assert_eq!(
        editor
            .draft
            .as_ref()
            .expect("draft retained")
            .qualification
            .suites
            .len(),
        1
    );
    assert_eq!(
        editor
            .draft
            .as_ref()
            .expect("draft retained")
            .qualification
            .suites[0]
            .vectors
            .len(),
        2
    );
    assert!(editor.qualification_authoring.error.is_some());
}

#[test]
fn advanced_qualification_authoring_builds_executable_analysis_contracts() {
    let (_manager, mut editor) = opened_editor();

    populate_operating_point_suite(&mut editor);
    {
        let fields = &mut editor.qualification_authoring;
        fields.suite_id = "ac-cv".to_owned();
        fields.suite_name = "AC and capacitance".to_owned();
        fields.analysis = QualificationAuthoringAnalysis::AcSweep;
        fields.frequencies = "1e3, 1e4".to_owned();
        fields.probe = QualificationAuthoringProbe::AcEffectiveCapacitance;
        fields.probe_target = "V1".to_owned();
        fields.excitation_magnitude = "1".to_owned();
        fields.sample = QualificationAuthoringSample::FirstFrequencyPoint;
        fields.executable_input =
            "V1 out 0 DC 0 AC 1\nR1 out 0 1k\nMbind 0 0 0 0 nch_owned\n.end\n".to_owned();
        fields.expected = "0".to_owned();
    }
    assert!(
        editor.commit_qualification_suite(),
        "{:?}",
        editor.qualification_authoring.error
    );

    editor.begin_qualification_suite();
    {
        let fields = &mut editor.qualification_authoring;
        fields.suite_id = "noise".to_owned();
        fields.suite_name = "Noise".to_owned();
        fields.vector_id = "noise-nominal".to_owned();
        fields.vector_name = "Nominal output noise".to_owned();
        fields.executable_input = "V1 input 0 DC 0 AC 1\nR1 input out 1k\nR2 out 0 1k\nMbind out input 0 0 nch_owned\n.end\n".to_owned();
        fields.analysis = QualificationAuthoringAnalysis::Noise;
        fields.frequencies = "1000".to_owned();
        fields.noise_output_node = "out".to_owned();
        fields.noise_input_source = "V1".to_owned();
        fields.noise_temperature_kelvin = "300.15".to_owned();
        fields.quantity = "onoise".to_owned();
        fields.probe = QualificationAuthoringProbe::NoiseOutputDensity;
        fields.sample = QualificationAuthoringSample::FirstFrequencyPoint;
        fields.expected = "0".to_owned();
        fields.absolute_tolerance = "1".to_owned();
        fields.relative_tolerance = "0".to_owned();
    }
    assert!(editor.commit_qualification_suite());

    editor.begin_qualification_suite();
    {
        let fields = &mut editor.qualification_authoring;
        fields.suite_id = "transient".to_owned();
        fields.suite_name = "Transient".to_owned();
        fields.vector_id = "transient-nominal".to_owned();
        fields.vector_name = "Nominal waveform".to_owned();
        fields.executable_input = "V1 out 0 DC 1\nMbind out out 0 0 nch_owned\n.end\n".to_owned();
        fields.analysis = QualificationAuthoringAnalysis::Transient;
        fields.transient_stop_time = "1e-6".to_owned();
        fields.transient_max_step = "1e-7".to_owned();
        fields.quantity = "v(out)".to_owned();
        fields.probe = QualificationAuthoringProbe::TransientNodeVoltage;
        fields.probe_target = "out".to_owned();
        fields.sample = QualificationAuthoringSample::LastTimePoint;
        fields.expected = "1".to_owned();
        fields.absolute_tolerance = "1e-8".to_owned();
        fields.relative_tolerance = "1e-8".to_owned();
    }
    assert!(editor.commit_qualification_suite());

    let suites = &editor.draft.as_ref().unwrap().qualification.suites;
    assert!(suites.iter().any(|suite| matches!(
        suite.vectors[0].analysis,
        QualificationAnalysis::AcSweep { .. }
    )));
    assert!(suites.iter().any(|suite| matches!(
        suite.vectors[0].analysis,
        QualificationAnalysis::Noise { .. }
    )));
    assert!(suites.iter().any(|suite| matches!(
        suite.vectors[0].analysis,
        QualificationAnalysis::Transient { .. }
    )));
}

#[test]
fn cooperative_qualification_publishes_only_a_complete_platform_run() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    app.state
        .model_library_manager
        .create_project_model("owned-models", &definition())
        .expect("create model");
    let revision = app.state.workspace.project.revision();
    app.state
        .workbench
        .model_editor
        .open(
            &app.state.model_library_manager,
            "owned-models",
            "nch_owned",
            revision,
        )
        .expect("open editor");
    populate_operating_point_suite(&mut app.state.workbench.model_editor);
    assert!(
        app.state
            .workbench
            .model_editor
            .commit_qualification_suite(),
        "{:?}",
        app.state
            .workbench
            .model_editor
            .qualification_authoring
            .error
    );
    assert!(start_qualification_execution(&mut app).is_ok());
    assert!(
        app.state
            .workbench
            .model_editor
            .draft
            .as_ref()
            .expect("draft")
            .qualification
            .platform_runs
            .is_empty()
    );

    advance_qualification_execution(&mut app);
    assert!(
        app.state
            .workbench
            .model_editor
            .qualification_execution
            .is_none()
    );
    let draft = app
        .state
        .workbench
        .model_editor
        .draft
        .as_ref()
        .expect("draft");
    assert_eq!(draft.qualification.platform_runs.len(), 1);
    assert!(draft.qualification.evidence.is_empty());
    assert!(draft.qualification_is_dirty());
}

#[test]
fn selected_vector_rerun_executes_only_its_complete_suite() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    app.state
        .model_library_manager
        .create_project_model("owned-models", &definition())
        .unwrap();
    let revision = app.state.workspace.project.revision();
    app.state
        .workbench
        .model_editor
        .open(
            &app.state.model_library_manager,
            "owned-models",
            "nch_owned",
            revision,
        )
        .unwrap();
    populate_operating_point_suite(&mut app.state.workbench.model_editor);
    assert!(
        app.state
            .workbench
            .model_editor
            .commit_qualification_suite()
    );
    populate_operating_point_suite(&mut app.state.workbench.model_editor);
    {
        let fields = &mut app.state.workbench.model_editor.qualification_authoring;
        fields.suite_id = "dc-second".to_owned();
        fields.suite_name = "Second DC suite".to_owned();
        fields.vector_id = "second-vector".to_owned();
        fields.vector_name = "Second vector".to_owned();
    }
    assert!(
        app.state
            .workbench
            .model_editor
            .commit_qualification_suite()
    );

    start_qualification_vector_execution(&mut app, "dc-second", "second-vector").unwrap();
    assert_eq!(
        app.state
            .workbench
            .model_editor
            .qualification_execution
            .as_ref()
            .unwrap()
            .suite_ids,
        vec!["dc-second".to_owned()]
    );
    assert!(cancel_qualification_execution(&mut app));
    assert!(start_qualification_vector_execution(&mut app, "dc-second", "missing-vector").is_err());
}

fn populate_complete_promotion_candidate(editor: &mut ModelEditorState) {
    let digest = ContentDigest::from_bytes([0x5a; 32]).to_string();
    let fields = &mut editor.promotion_candidate;
    fields.model_description_id = "model-description".to_owned();
    fields.model_description_digest.clone_from(&digest);
    fields.parameter_reference_id = "parameter-reference".to_owned();
    fields.parameter_reference_digest.clone_from(&digest);
    fields.qualification_report_id = "qualification-report".to_owned();
    fields.qualification_report_digest.clone_from(&digest);
    fields.license_id = "commercial-project-license".to_owned();
    fields.license_expression = "LicenseRef-RSpice-Project".to_owned();
    fields.license_scope = LicenseScope::OrganizationInternal;
    fields.commercial_use_allowed = true;
    fields.license_reviewed = true;
    fields.license_notice_id = "license-notice".to_owned();
    fields.license_notice_digest.clone_from(&digest);
    fields.consumer_change = ConsumerChange::NoImpact;
    fields.consumer_summary = "No known consumer behavior change".to_owned();
    fields.consumer_reviewed = true;
    fields.desktop_compatibility = CompatibilityDisposition::Compatible;
    fields.desktop_evidence_id = "desktop-compatibility".to_owned();
    fields.desktop_evidence_digest.clone_from(&digest);
    fields.webassembly_compatibility = CompatibilityDisposition::Compatible;
    fields.webassembly_evidence_id = "wasm-compatibility".to_owned();
    fields.webassembly_evidence_digest.clone_from(&digest);
    fields.existing_projects_compatibility = CompatibilityDisposition::Compatible;
    fields.compatibility_reviewed = true;
    fields.model_owner_id = "model-owner".to_owned();
    fields.qualification_approver_id = "independent-qualification-reviewer".to_owned();
}

#[test]
fn governed_candidate_creation_and_promotion_are_complete_and_atomic() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    app.state
        .model_library_manager
        .create_project_model("owned-models", &definition())
        .expect("create model");
    let revision = app.state.workspace.project.revision();
    app.state
        .workbench
        .model_editor
        .open(
            &app.state.model_library_manager,
            "owned-models",
            "nch_owned",
            revision,
        )
        .expect("open editor");
    populate_operating_point_suite(&mut app.state.workbench.model_editor);
    assert!(
        app.state
            .workbench
            .model_editor
            .commit_qualification_suite()
    );
    assert!(start_qualification_execution(&mut app).is_ok());
    advance_qualification_execution(&mut app);

    let draft = app
        .state
        .workbench
        .model_editor
        .draft
        .as_mut()
        .expect("draft");
    let desktop = draft.qualification.platform_runs[0].clone();
    let mut webassembly = desktop.clone();
    webassembly.platform = QualificationPlatform::WebAssembly;
    for vector in &mut webassembly.vector_outcomes {
        vector.outcome.platform = QualificationPlatform::WebAssembly;
    }
    draft
        .qualification
        .upsert_platform_run_atomically(webassembly)
        .expect("retain exact WebAssembly run");
    let source = desktop.source.clone();
    draft
        .qualification
        .assemble_and_upsert_evidence_atomically("dc-op-evidence", "dc-op", &source)
        .expect("assemble exact parity evidence");

    let exact_evidence_id = draft.qualification.evidence[0].id.clone();
    let mut foreign_identity_evidence = draft.qualification.evidence[0].clone();
    foreign_identity_evidence.id = "foreign-source-evidence".to_owned();
    foreign_identity_evidence.source.source_id = Some(ModelSourceId::new());
    draft
        .qualification
        .evidence
        .insert(0, foreign_identity_evidence);

    app.state.workbench.model_editor.begin_promotion_review();
    assert_eq!(
        app.state
            .workbench
            .model_editor
            .promotion_candidate
            .evidence_id,
        exact_evidence_id,
        "promotion prefill must include source_id in its exact-source projection"
    );
    app.state
        .workbench
        .model_editor
        .draft
        .as_mut()
        .expect("draft")
        .qualification
        .evidence
        .retain(|evidence| evidence.id != "foreign-source-evidence");
    populate_complete_promotion_candidate(&mut app.state.workbench.model_editor);
    assert!(app.state.workbench.model_editor.commit_release_candidate());
    let candidate = app
        .state
        .workbench
        .model_editor
        .draft
        .as_ref()
        .expect("draft")
        .qualification
        .candidates
        .first()
        .expect("candidate")
        .clone();
    assert!(candidate.checklist.is_complete());
    assert!(!candidate.definition_source.is_empty());
    assert!(candidate.definition_metadata.is_some());

    let before_duplicate = app
        .state
        .workbench
        .model_editor
        .draft
        .as_ref()
        .expect("draft")
        .qualification
        .clone();
    app.state
        .workbench
        .model_editor
        .promotion_candidate
        .candidate_id = "conflicting-candidate".to_owned();
    assert!(!app.state.workbench.model_editor.commit_release_candidate());
    assert_eq!(
        app.state
            .workbench
            .model_editor
            .draft
            .as_ref()
            .expect("draft")
            .qualification,
        before_duplicate
    );

    let invalid_correlation = ModelCorrelationState {
        schema_version: u32::MAX,
        ..ModelCorrelationState::default()
    };
    assert!(
        !app.state
            .workbench
            .model_editor
            .commit_promotion(&candidate.identity.id, &invalid_correlation),
        "the authoritative promotion transaction must fail closed when retained correlation state is invalid"
    );
    assert!(
        app.state
            .workbench
            .model_editor
            .commit_promotion(&candidate.identity.id, &ModelCorrelationState::default())
    );
    let qualification = &app
        .state
        .workbench
        .model_editor
        .draft
        .as_ref()
        .expect("draft")
        .qualification;
    assert_eq!(qualification.releases.len(), 1);
    assert_eq!(qualification.promotions.len(), 1);
    assert_eq!(
        qualification.releases[0].definition_metadata,
        candidate.definition_metadata
    );
}
