//! Commands over device models: the editor, the library scan, qualification.
//!
//! Selection authority is what these share. A model command names exactly one
//! coherent, project-owned definition or it is unavailable — it never opens on
//! an ambiguous selection, on a source the project does not own, or on a
//! selection retained from a project that has since closed.

use super::harness;
use crate::workbench::framework::command_registry::CommandAvailability;
use crate::workbench::models::commands::ModelsCommand;
use crate::workbench::models::model_editor_workflow::{
    ModelEditorWorkflow, active_model_editor_workflow, close_model_editor_workflow,
};

#[test]
fn model_editor_command_has_fail_closed_selection_authority() {
    use crate::state::model_library::{DeviceModel, ModelLibrary, ModelType};

    let mut app = harness();
    app.open_project_session();
    app.model.model_library_manager.selected_library = None;
    app.state.selected_model = None;
    assert_eq!(
        app.availability(ModelsCommand::ModelEditor.key()),
        CommandAvailability::Disabled("select one model in Model & library catalog")
    );

    let mut built_in = ModelLibrary::new("command-editor-built-in");
    built_in.add_model(DeviceModel::new("readonly_nch", ModelType::Nmos));
    app.model.model_library_manager.add_library(built_in);
    app.model
        .model_library_manager
        .select_library("command-editor-built-in")
        .expect("the fixture library is loaded");
    app.state.selected_model = Some("readonly_nch".to_owned());
    assert_eq!(
        app.availability(ModelsCommand::ModelEditor.key()),
        CommandAvailability::Disabled(
            "the selected model is built-in; create an editable project copy first"
        )
    );
}

#[test]
fn editable_project_copy_command_accepts_external_models_and_rejects_owned_or_read_only_state() {
    use std::collections::BTreeMap;

    use crate::state::model_library::{
        DeviceModel, ModelLibrary, ModelSourceAuthority, ModelType, ProjectModelDefinition,
    };

    let mut app = harness();
    app.open_project_session();
    let mut external = ModelLibrary::new("external command source");
    external.source_authority = ModelSourceAuthority::External;
    external.add_model(DeviceModel::new("external_nch", ModelType::Nmos));
    app.model.model_library_manager.add_library(external);
    app.model
        .model_library_manager
        .select_library("external command source")
        .expect("the fixture library is loaded");
    app.state.selected_model = Some("external_nch".to_owned());
    assert_eq!(
        app.availability(ModelsCommand::ModelCreateProjectCopy.key()),
        CommandAvailability::Available
    );

    app.model
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
    app.model
        .model_library_manager
        .select_library("already owned")
        .expect("the fixture library is loaded");
    app.state.selected_model = Some("owned_nch".to_owned());
    assert_eq!(
        app.availability(ModelsCommand::ModelCreateProjectCopy.key()),
        CommandAvailability::Disabled("the selected model is already an editable project copy")
    );

    app.model
        .model_library_manager
        .select_library("external command source")
        .expect("the fixture library is loaded");
    app.state.selected_model = Some("external_nch".to_owned());
    app.model.safe_mode.activate(
        crate::state::write_policy::LocalSafeModeOptions {
            open_project_read_only: true,
            ..crate::state::write_policy::LocalSafeModeOptions::default()
        },
        "read-only copy test".to_owned(),
    );
    assert_eq!(
        app.availability(ModelsCommand::ModelCreateProjectCopy.key()),
        CommandAvailability::Disabled("the project is open read-only")
    );
}

#[test]
fn model_editor_command_accepts_one_coherent_project_owned_definition() {
    use std::collections::BTreeMap;

    use crate::state::model_library::ProjectModelDefinition;

    let mut app = harness();
    app.open_project_session();
    let commit = app
        .model
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
    app.model
        .model_library_manager
        .select_library(&commit.library_name)
        .expect("the fixture library is loaded");
    app.state.selected_model = Some(commit.model_name);

    assert_eq!(
        app.availability(ModelsCommand::ModelEditor.key()),
        CommandAvailability::Available
    );
    assert!(
        app.availability(ModelsCommand::ModelEditor.key())
            .is_available()
    );

    app.model.safe_mode.activate(
        crate::state::write_policy::LocalSafeModeOptions {
            open_project_read_only: true,
            ..crate::state::write_policy::LocalSafeModeOptions::default()
        },
        "read-only model review".to_owned(),
    );
    assert_eq!(
        app.availability(ModelsCommand::ModelEditor.key()),
        CommandAvailability::Available
    );
    app.execute(ModelsCommand::ModelEditor.key());
    assert_eq!(
        app.wb.current_route().surface_id(),
        crate::workbench::SurfaceId::ModelEditor
    );
    assert!(app.state.model_editor.draft.is_some());
    assert_eq!(
        app.availability(ModelsCommand::ModelSaveRevision.key()),
        CommandAvailability::Disabled("the project is open read-only")
    );
    assert_eq!(
        app.availability(ModelsCommand::ModelRunQualificationTests.key()),
        CommandAvailability::Disabled("qualification cannot run while the project is read-only")
    );
    assert!(
        app.availability(ModelsCommand::ModelValidate.key())
            .is_available()
    );
    app.execute(ModelsCommand::ModelValidate.key());
    assert_eq!(
        active_model_editor_workflow(app.state.model_editor_reads(&app.model, &app.platform))
            .map(|request| request.workflow),
        Some(ModelEditorWorkflow::ValidateCandidate)
    );
    close_model_editor_workflow();
}

#[test]
fn qualification_command_requires_a_suite_for_the_exact_open_source() {
    use std::collections::BTreeMap;

    use crate::state::model_library::ProjectModelDefinition;

    let mut app = harness();
    app.open_project_session();
    app.model
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
    let project_revision = app.model.workspace.content.project.revision();
    app.state
        .model_editor
        .open(
            &app.model.model_library_manager,
            "command-qualification-owned",
            "qualification_nch",
            project_revision,
        )
        .expect("open editor");
    app.state.model_editor.begin_qualification_suite();
    let authoring = &mut app.state.model_editor.qualification_authoring;
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
    assert!(app.state.model_editor.commit_qualification_suite());
    assert!(
        app.state
            .model_editor
            .validate_candidate(&app.model.model_library_manager, project_revision)
    );
    assert!(
        app.availability(ModelsCommand::ModelRunQualificationTests.key())
            .is_available()
    );

    app.state
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
            .model_editor
            .draft
            .as_ref()
            .expect("draft")
            .qualification
            .validate_for_model("qualification_nch")
            .is_ok()
    );
    assert!(
        !app.availability(ModelsCommand::ModelRunQualificationTests.key())
            .is_available()
    );
}
