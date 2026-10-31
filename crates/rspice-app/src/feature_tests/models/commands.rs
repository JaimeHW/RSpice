//! Commands over device models where the shell decides the outcome: its
//! prologue holding a command back while no project is open, the command
//! registry a command is reachable through, the project history that undoes
//! what one published, and the notices the frame drains into the activity
//! feed. The models feature's own answers are its own tests'.

use crate::state::ModelsPage;
use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::command_registry;
use crate::workbench::framework::command_registry::{CommandAvailability, CommandFlags, ids};
use crate::workbench::models::commands::ModelsCommand;
use crate::workbench::models::model_editor_workflow::selected_project_model_for_editor;
use crate::workbench::state::Workspace;

#[test]
fn editable_project_copy_command_publishes_opens_and_records_undo_history() {
    use crate::state::model_library::{DeviceModel, ModelLibrary, ModelSourceAuthority, ModelType};

    let copy = ModelsCommand::ModelCreateProjectCopy;
    assert_eq!(copy.key().as_str(), "model-create-project-copy");
    assert_eq!(copy.declared_spec().label, "Create editable project copy");
    assert_eq!(copy.declared_spec().group, "Models");
    assert_eq!(
        command_registry().resolve("model-create-project-copy"),
        Some(copy.key())
    );
    assert!(
        command_registry().order().contains(&copy.key()),
        "project-copy action must be reachable through the command registry"
    );
    assert!(
        copy.declared_spec()
            .flags
            .contains(CommandFlags::BLOCKED_BY_PROJECT_OPERATION)
    );

    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    let initial_project_revision = app.state.model.workspace.content.project.revision();
    let mut built_in = ModelLibrary::new("command copy built-in");
    built_in.pdk_name = "Example PDK".to_owned();
    built_in.technology_node = "45nm".to_owned();
    let mut model = DeviceModel::new("copy_nch", ModelType::Nmos);
    model.spice_type = Some("NMOS".to_owned());
    model.spice_level = Some(1);
    model.description = "Built-in command copy".to_owned();
    model.parameters.insert("vth0".to_owned(), 0.46);
    built_in.add_model(model);
    app.state.model.model_library_manager.add_library(built_in);
    app.state
        .model
        .model_library_manager
        .select_library("command copy built-in")
        .expect("the fixture library is loaded");
    app.state.models.selected_model = Some("copy_nch".to_owned());

    assert_eq!(
        app.command_availability(ModelsCommand::ModelCreateProjectCopy.key()),
        CommandAvailability::Available
    );
    assert!(!app.command_key_enabled(ModelsCommand::ModelEditor.key()));
    app.execute_command_key(ModelsCommand::ModelCreateProjectCopy.key());

    assert!(
        app.state.model.workspace.content.project.revision() > initial_project_revision,
        "copy publication advances the guarded project revision"
    );
    assert!(app.state.model.workspace.content.project_metadata_dirty);
    assert_eq!(
        app.state
            .model
            .model_library_manager
            .selected_library
            .as_deref(),
        Some("copy_nch project")
    );
    assert_eq!(app.state.models.selected_model.as_deref(), Some("copy_nch"));
    let project_copy = app
        .state
        .model
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
        .models
        .model_editor
        .draft
        .as_ref()
        .expect("the exact committed copy opens in the editor");
    assert_eq!(draft.library_name, "copy_nch project");
    assert_eq!(draft.model_name, "copy_nch");
    assert_eq!(
        draft.base_project_revision,
        app.state.model.workspace.content.project.revision()
    );
    assert!(app.command_key_enabled(ModelsCommand::ModelEditor.key()));

    assert!(app.state.model.can_undo_project_design());
    let undo_description = app
        .state
        .undo_project_design()
        .expect("copy undo succeeds")
        .expect("copy records one history item");
    assert!(undo_description.starts_with("create editable project model "));
    assert!(
        app.state
            .model
            .model_library_manager
            .get_library("copy_nch project")
            .is_none(),
        "undo removes the newly created project library"
    );
    assert!(app.state.model.can_redo_project_design());
    app.state
        .redo_project_design()
        .expect("copy redo succeeds")
        .expect("copy redo has one history item");
    assert!(
        app.state
            .model
            .model_library_manager
            .get_library("copy_nch project")
            .is_some(),
        "redo restores the authenticated project copy"
    );
}

#[test]
fn model_editor_command_requires_an_open_project_even_with_a_retained_selection() {
    use std::collections::BTreeMap;

    use crate::state::model_library::ProjectModelDefinition;

    let mut app = RSpiceApp::test_instance();
    let commit = app
        .state
        .model
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
        .model
        .model_library_manager
        .select_library(&commit.library_name)
        .expect("the fixture library is loaded");
    app.state.models.selected_model = Some(commit.model_name);
    app.state.close_project_session();

    assert_eq!(
        app.command_availability(ModelsCommand::ModelEditor.key()),
        CommandAvailability::Disabled("no project is open")
    );
    assert!(!app.command_key_enabled(ModelsCommand::ModelEditor.key()));
    assert_eq!(
        selected_project_model_for_editor(
            app.state
                .models
                .model_editor_reads(&app.state.model, &app.state.platform)
        ),
        Err("open a project before editing a device model")
    );
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
    app.state.open_project_session();
    app.state.model.pdk_config = crate::state::pdk_config::PdkConfig::default();
    app.state
        .model
        .pdk_config
        .add_library_path(root.to_string_lossy().into_owned());

    // The command itself reports, so the toolbar, the menu and the palette
    // all say the same. A clean rescan answers the gesture on the status bar
    // and in the Console; it interrupts nobody.
    let ctx = egui::Context::default();
    app.execute_command_key(ModelsCommand::RescanModelLibraries.key());
    app.state.drain_notices(&ctx);

    assert_eq!(app.state.model.pdk_config.discovered_files.len(), 1);
    let success = app
        .state
        .model
        .log_buffer
        .entries()
        .last()
        .expect("rescan receipt");
    assert_eq!(success.severity, crate::diagnostics::LogSeverity::Info);
    assert!(success.message.contains("Found 1 configured model file(s)"));
    assert!(
        app.state
            .prompt_text()
            .is_some_and(|prompt| prompt.contains("Found 1 configured model file(s)"))
    );
    assert!(app.state.workbench.toasts.activity().is_empty());

    // A library it could not read is worth interrupting for, with the way
    // to the Console line that lists every path.
    app.state
        .model
        .pdk_config
        .add_library_path(root.join("missing").to_string_lossy().into_owned());
    app.execute_command_key(ModelsCommand::RescanModelLibraries.key());
    app.state.drain_notices(&ctx);

    assert_eq!(app.state.model.pdk_config.discovered_files.len(), 1);
    let warning = app
        .state
        .model
        .log_buffer
        .entries()
        .last()
        .expect("warning receipt");
    assert_eq!(warning.severity, crate::diagnostics::LogSeverity::Warning);
    assert!(warning.message.contains("1 configured path error(s)"));
    assert!(warning.message.contains("Path does not exist"));
    let warning = warning.id;
    let activity = app.state.workbench.toasts.activity();
    assert_eq!(activity.len(), 1);
    assert_eq!(activity[0].kind(), crate::ui::widgets::ToastKind::Warn);
    assert!(activity[0].message().contains("1 configured path error(s)"));
    assert_eq!(
        activity[0].action(),
        Some(crate::ui::widgets::NotificationAction::ShowInConsole { log_id: warning })
    );

    std::fs::remove_dir_all(&root).expect("remove model-library fixture");
}

/// The model catalog and the PDK paths are raised by the keys other
/// features name them by, behind the prologue every command shares: with no
/// project open the catalog is refused and the workspace stays where it was;
/// with one, the catalog opens the Models workspace on its catalog page, and
/// the PDK paths open their dialog.
#[test]
fn model_commands_raised_by_key_run_behind_the_shared_prologue() {
    let mut app = RSpiceApp::test_instance();
    app.state.close_project_session();
    let workspace = app.state.workbench.workspace;
    assert_ne!(workspace, Workspace::Models);
    assert_eq!(
        app.command_availability(ids::MODELS_CATALOG),
        CommandAvailability::Disabled("no project is open")
    );
    app.execute_command_key(ids::MODELS_CATALOG);
    assert_eq!(app.state.workbench.workspace, workspace);

    app.state.open_project_session();
    app.state.models.models_page = ModelsPage::Bins;
    app.execute_command_key(ids::MODELS_CATALOG);
    assert_eq!(app.state.workbench.workspace, Workspace::Models);
    assert_eq!(app.state.models.models_page, ModelsPage::Models);

    assert!(!app.state.models.pdk_settings_dialog.open);
    app.execute_command_key(ids::PDK_SETTINGS);
    assert!(app.state.models.pdk_settings_dialog.open);
}
