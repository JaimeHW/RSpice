//! Commands over device models: the editor, the library scan, qualification.
//!
//! Selection authority is what these share. A model command names exactly one
//! coherent, project-owned definition or it is unavailable — it never opens on
//! an ambiguous selection, on a source the project does not own, or on a
//! selection retained from a project that has since closed.

use super::*;

#[test]
fn model_editor_command_has_mockup_identity_and_fail_closed_selection_authority() {
    use crate::state::model_library::{DeviceModel, ModelLibrary, ModelType};

    assert_eq!(Command::ModelEditor.stable_id(), "model-editor");
    assert_eq!(
        Command::ModelEditor.spec().label,
        "Device model and parameter editor\u{2026}"
    );
    assert_eq!(Command::ModelEditor.spec().group, "Models");
    assert_eq!(
        Command::from_stable_id("model-editor"),
        Some(Command::ModelEditor)
    );
    assert!(Command::ModelEditor.shortcut_bindings().is_empty());

    let registry_index = vocabulary::COMMAND_REGISTRY
        .iter()
        .position(|command| *command == Command::ModelEditor)
        .expect("model editor command must be registered");
    assert_eq!(
        vocabulary::COMMAND_REGISTRY[registry_index - 1],
        Command::ModelCreateProjectCopy
    );

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    app.state.model_library_manager.selected_library = None;
    app.state.workbench.selected_model = None;
    assert_eq!(
        Command::ModelEditor.availability(&app),
        CommandAvailability::Disabled("select one model in Model & library catalog")
    );

    let mut built_in = ModelLibrary::new("command-editor-built-in");
    built_in.add_model(DeviceModel::new("readonly_nch", ModelType::Nmos));
    app.state.model_library_manager.add_library(built_in);
    app.state
        .model_library_manager
        .select_library("command-editor-built-in");
    app.state.workbench.selected_model = Some("readonly_nch".to_owned());
    assert_eq!(
        Command::ModelEditor.availability(&app),
        CommandAvailability::Disabled(
            "the selected model is built-in; create an editable project copy first"
        )
    );
}

#[test]
fn editable_project_copy_command_publishes_opens_and_records_undo_history() {
    use crate::state::model_library::{DeviceModel, ModelLibrary, ModelSourceAuthority, ModelType};

    assert_eq!(
        Command::ModelCreateProjectCopy.stable_id(),
        "model-create-project-copy"
    );
    assert_eq!(
        Command::ModelCreateProjectCopy.spec().label,
        "Create editable project copy"
    );
    assert_eq!(Command::ModelCreateProjectCopy.spec().group, "Models");
    assert_eq!(
        Command::from_stable_id("model-create-project-copy"),
        Some(Command::ModelCreateProjectCopy)
    );
    assert!(
        vocabulary::COMMAND_REGISTRY.contains(&Command::ModelCreateProjectCopy),
        "project-copy action must be reachable through the command registry"
    );
    assert!(Command::ModelCreateProjectCopy.blocked_by_project_operation());

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    let initial_project_revision = app.state.workspace.project.revision();
    let mut built_in = ModelLibrary::new("command copy built-in");
    built_in.pdk_name = "Example PDK".to_owned();
    built_in.technology_node = "45nm".to_owned();
    let mut model = DeviceModel::new("copy_nch", ModelType::Nmos);
    model.spice_type = Some("NMOS".to_owned());
    model.spice_level = Some(1);
    model.description = "Built-in command copy".to_owned();
    model.parameters.insert("vth0".to_owned(), 0.46);
    built_in.add_model(model);
    app.state.model_library_manager.add_library(built_in);
    app.state
        .model_library_manager
        .select_library("command copy built-in");
    app.state.workbench.selected_model = Some("copy_nch".to_owned());

    assert_eq!(
        Command::ModelCreateProjectCopy.availability(&app),
        CommandAvailability::Available
    );
    assert!(!Command::ModelEditor.is_enabled(&app));
    Command::ModelCreateProjectCopy.execute(&mut app);

    assert!(
        app.state.workspace.project.revision() > initial_project_revision,
        "copy publication advances the guarded project revision"
    );
    assert!(app.state.workspace.project_metadata_dirty);
    assert_eq!(
        app.state.model_library_manager.selected_library.as_deref(),
        Some("copy_nch project")
    );
    assert_eq!(
        app.state.workbench.selected_model.as_deref(),
        Some("copy_nch")
    );
    let project_copy = app
        .state
        .model_library_manager
        .get_library("copy_nch project")
        .expect("command publishes the copy");
    assert!(matches!(
        project_copy.source_authority,
        ModelSourceAuthority::ProjectOwned { .. }
    ));
    assert_eq!(project_copy.pdk_name, "Example PDK");
    assert_eq!(project_copy.technology_node, "45nm");
    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        crate::workbench::SurfaceId::ModelEditor
    );
    let draft = app
        .state
        .workbench
        .model_editor
        .draft
        .as_ref()
        .expect("the exact committed copy opens in the editor");
    assert_eq!(draft.library_name, "copy_nch project");
    assert_eq!(draft.model_name, "copy_nch");
    assert_eq!(
        draft.base_project_revision,
        app.state.workspace.project.revision()
    );
    assert!(Command::ModelEditor.is_enabled(&app));

    assert!(app.state.can_undo_project_design());
    let undo_description = app
        .state
        .undo_project_design()
        .expect("copy undo succeeds")
        .expect("copy records one history item");
    assert!(undo_description.starts_with("create editable project model "));
    assert!(
        app.state
            .model_library_manager
            .get_library("copy_nch project")
            .is_none(),
        "undo removes the newly created project library"
    );
    assert!(app.state.can_redo_project_design());
    app.state
        .redo_project_design()
        .expect("copy redo succeeds")
        .expect("copy redo has one history item");
    assert!(
        app.state
            .model_library_manager
            .get_library("copy_nch project")
            .is_some(),
        "redo restores the authenticated project copy"
    );
}

#[test]
fn editable_project_copy_command_accepts_external_models_and_rejects_owned_or_read_only_state() {
    use std::collections::BTreeMap;

    use crate::state::model_library::{
        DeviceModel, ModelLibrary, ModelSourceAuthority, ModelType, ProjectModelDefinition,
    };

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    let mut external = ModelLibrary::new("external command source");
    external.source_authority = ModelSourceAuthority::External;
    external.add_model(DeviceModel::new("external_nch", ModelType::Nmos));
    app.state.model_library_manager.add_library(external);
    app.state
        .model_library_manager
        .select_library("external command source");
    app.state.workbench.selected_model = Some("external_nch".to_owned());
    assert_eq!(
        Command::ModelCreateProjectCopy.availability(&app),
        CommandAvailability::Available
    );

    app.state
        .model_library_manager
        .create_project_model(
            "already owned",
            &ProjectModelDefinition {
                name: "owned_nch".to_owned(),
                spice_type: "NMOS".to_owned(),
                description: "Already editable".to_owned(),
                numeric_parameters: BTreeMap::new(),
                string_parameters: BTreeMap::new(),
            },
        )
        .expect("owned fixture");
    app.state
        .model_library_manager
        .select_library("already owned");
    app.state.workbench.selected_model = Some("owned_nch".to_owned());
    assert_eq!(
        Command::ModelCreateProjectCopy.availability(&app),
        CommandAvailability::Disabled("the selected model is already an editable project copy")
    );

    app.state
        .model_library_manager
        .select_library("external command source");
    app.state.workbench.selected_model = Some("external_nch".to_owned());
    app.state.workbench.safe_mode.activate(
        crate::workbench::state::LocalSafeModeOptions {
            open_project_read_only: true,
            ..crate::workbench::state::LocalSafeModeOptions::default()
        },
        "read-only copy test".to_owned(),
    );
    assert_eq!(
        Command::ModelCreateProjectCopy.availability(&app),
        CommandAvailability::Disabled("the project is open read-only")
    );
}

#[test]
fn model_editor_command_accepts_one_coherent_project_owned_definition() {
    use std::collections::BTreeMap;

    use crate::state::model_library::ProjectModelDefinition;

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    let commit = app
        .state
        .model_library_manager
        .create_project_model(
            "command-editor-owned",
            &ProjectModelDefinition {
                name: "command_nch".to_owned(),
                spice_type: "NMOS".to_owned(),
                description: "Command dispatch fixture".to_owned(),
                numeric_parameters: BTreeMap::from([
                    ("level".to_owned(), 1.0),
                    ("vth0".to_owned(), 0.48),
                ]),
                string_parameters: BTreeMap::new(),
            },
        )
        .expect("create coherent project-owned model");
    app.state
        .model_library_manager
        .select_library(&commit.library_name);
    app.state.workbench.selected_model = Some(commit.model_name);

    assert_eq!(
        Command::ModelEditor.availability(&app),
        CommandAvailability::Available
    );
    assert!(Command::ModelEditor.is_enabled(&app));

    app.state.workbench.safe_mode.activate(
        crate::workbench::state::LocalSafeModeOptions {
            open_project_read_only: true,
            ..crate::workbench::state::LocalSafeModeOptions::default()
        },
        "read-only model review".to_owned(),
    );
    assert_eq!(
        Command::ModelEditor.availability(&app),
        CommandAvailability::Available
    );
    Command::ModelEditor.execute(&mut app);
    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        crate::workbench::SurfaceId::ModelEditor
    );
    assert!(app.state.workbench.model_editor.draft.is_some());
    assert_eq!(
        Command::ModelSaveRevision.availability(&app),
        CommandAvailability::Disabled("the project is open read-only")
    );
    assert_eq!(
        Command::ModelRunQualificationTests.availability(&app),
        CommandAvailability::Disabled("qualification cannot run while the project is read-only")
    );
    assert!(Command::ModelValidate.is_enabled(&app));
    Command::ModelValidate.execute(&mut app);
    assert_eq!(
        active_model_editor_workflow(&app).map(|request| request.workflow),
        Some(ModelEditorWorkflow::ValidateCandidate)
    );
    close_model_editor_workflow();
}

#[test]
fn model_editor_command_requires_an_open_project_even_with_a_retained_selection() {
    use std::collections::BTreeMap;

    use crate::state::model_library::ProjectModelDefinition;

    let mut app = RSpiceApp::test_instance();
    let commit = app
        .state
        .model_library_manager
        .create_project_model(
            "command-editor-closed-project",
            &ProjectModelDefinition {
                name: "retained_nch".to_owned(),
                spice_type: "NMOS".to_owned(),
                description: "Retained selection without an open project".to_owned(),
                numeric_parameters: BTreeMap::from([("level".to_owned(), 1.0)]),
                string_parameters: BTreeMap::new(),
            },
        )
        .expect("create retained project-owned model");
    app.state
        .model_library_manager
        .select_library(&commit.library_name);
    app.state.workbench.selected_model = Some(commit.model_name);
    app.state.project_lifecycle.project_open = false;

    assert_eq!(
        Command::ModelEditor.availability(&app),
        CommandAvailability::Disabled("no project is open")
    );
    assert!(!Command::ModelEditor.is_enabled(&app));
    assert_eq!(
        selected_project_model_for_editor(&app),
        Err("open a project before editing a device model")
    );
}

#[test]
fn qualification_command_requires_a_suite_for_the_exact_open_source() {
    use std::collections::BTreeMap;

    use crate::state::model_library::ProjectModelDefinition;

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    app.state
        .model_library_manager
        .create_project_model(
            "command-qualification-owned",
            &ProjectModelDefinition {
                name: "qualification_nch".to_owned(),
                spice_type: "NMOS".to_owned(),
                description: "Qualification command fixture".to_owned(),
                numeric_parameters: BTreeMap::from([("level".to_owned(), 1.0)]),
                string_parameters: BTreeMap::new(),
            },
        )
        .expect("create project model");
    let project_revision = app.state.workspace.project.revision();
    app.state
        .workbench
        .model_editor
        .open(
            &app.state.model_library_manager,
            "command-qualification-owned",
            "qualification_nch",
            project_revision,
        )
        .expect("open editor");
    app.state.workbench.model_editor.begin_qualification_suite();
    let authoring = &mut app.state.workbench.model_editor.qualification_authoring;
    authoring.suite_id = "dc-op".to_owned();
    authoring.suite_name = "DC operating point".to_owned();
    authoring.vector_id = "nominal".to_owned();
    authoring.vector_name = "Nominal bias".to_owned();
    authoring.executable_input =
        "V1 out 0 1\nR1 out 0 1k\nMbind 0 0 0 0 qualification_nch\n.op\n.end\n".to_owned();
    authoring.quantity = "v(out)".to_owned();
    authoring.probe_target = "out".to_owned();
    authoring.expected = "1".to_owned();
    authoring.absolute_tolerance = "1e-9".to_owned();
    authoring.relative_tolerance = "1e-9".to_owned();
    assert!(
        app.state
            .workbench
            .model_editor
            .commit_qualification_suite()
    );
    assert!(
        app.state
            .workbench
            .model_editor
            .validate_candidate(&app.state.model_library_manager, project_revision)
    );
    assert!(Command::ModelRunQualificationTests.is_enabled(&app));

    app.state
        .workbench
        .model_editor
        .draft
        .as_mut()
        .expect("draft")
        .qualification
        .suites[0]
        .vectors[0]
        .source
        .source_id = Some(crate::product::ModelSourceId::new());
    assert!(
        app.state
            .workbench
            .model_editor
            .draft
            .as_ref()
            .expect("draft")
            .qualification
            .validate_for_model("qualification_nch")
            .is_ok()
    );
    assert!(!Command::ModelRunQualificationTests.is_enabled(&app));
}

#[test]
fn model_library_rescan_discovers_files_and_reports_path_errors() {
    let nonce = crate::time_compat::unix_epoch().as_nanos();
    let root = std::env::temp_dir().join(format!(
        "rspice-command-rescan-{}-{nonce}",
        std::process::id()
    ));
    std::fs::create_dir_all(&root).expect("create model-library fixture");
    std::fs::write(root.join("device.lib"), ".model dtest d\n")
        .expect("write model-library fixture");

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    app.state.pdk_config = crate::state::pdk_config::PdkConfig::new();
    app.state
        .pdk_config
        .add_library_path(root.to_string_lossy().into_owned());
    let ctx = egui::Context::default();

    Command::RescanModelLibraries.execute_with_feedback(&mut app, &ctx);

    assert_eq!(app.state.pdk_config.discovered_files.len(), 1);
    let success = app
        .state
        .log_buffer
        .entries()
        .last()
        .expect("rescan receipt");
    assert_eq!(success.severity, crate::diagnostics::LogSeverity::Info);
    assert!(success.message.contains("found 1 configured model file(s)"));
    assert_eq!(
        app.state.ui.toasts.activity()[0].kind(),
        crate::ui::widgets::ToastKind::Success
    );
    assert!(
        app.state.ui.toasts.activity()[0]
            .message()
            .contains("found 1 configured model file(s)")
    );

    app.state
        .pdk_config
        .add_library_path(root.join("missing").to_string_lossy().into_owned());
    Command::RescanModelLibraries.execute_with_feedback(&mut app, &ctx);

    assert_eq!(app.state.pdk_config.discovered_files.len(), 1);
    let warning = app
        .state
        .log_buffer
        .entries()
        .last()
        .expect("warning receipt");
    assert_eq!(warning.severity, crate::diagnostics::LogSeverity::Warning);
    assert!(warning.message.contains("1 configured path error(s)"));
    assert!(warning.message.contains("Path does not exist"));
    assert_eq!(
        app.state.ui.toasts.activity()[0].kind(),
        crate::ui::widgets::ToastKind::Warn
    );
    assert!(
        app.state.ui.toasts.activity()[0]
            .message()
            .contains("1 configured path error(s)")
    );

    std::fs::remove_dir_all(&root).expect("remove model-library fixture");
}
