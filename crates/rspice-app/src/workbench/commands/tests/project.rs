//! Project-scoped commands and the closed-project boundary.
//!
//! With no project open the product offers the Project workspace and nothing
//! else, and every mutating project command is refused while a project
//! operation is still in flight. The boundary is asserted per command rather
//! than once, because each one reaches it by its own route and a subcommand
//! that skips the gate is invisible to a single aggregate check.

use super::*;
use crate::workbench::cloud::commands::CloudCommand;
use crate::workbench::framework::command_registry::CommandFlags;
use crate::workbench::framework::requests::FocusProjectLauncherSearch;
use crate::workbench::hardcopy::commands::HardcopyCommand;
use crate::workbench::models::commands::ModelsCommand;

#[test]
fn every_project_tab_has_one_discoverable_command_with_a_stable_identity() {
    let expected = [
        (ProjectPage::Overview, "project-overview"),
        (ProjectPage::Library, "project-library"),
        (
            ProjectPage::Configuration,
            "project-testbench-configuration",
        ),
        (ProjectPage::Dependencies, "project-dependencies"),
        (ProjectPage::Recovery, "project-recovery"),
    ];
    assert_eq!(ProjectPage::ALL.len(), expected.len());

    for (page, stable_id) in expected {
        let command = Command::ProjectPage(page);
        assert!(
            vocabulary::COMMAND_REGISTRY.contains(&command),
            "project tab is absent from the command registry: {page:?}"
        );
        assert_eq!(command.stable_id(), stable_id);
        assert_eq!(Command::from_stable_id(stable_id), Some(command));
        assert!(
            command.requires_open_project(),
            "project tab bypasses the open-project boundary: {page:?}"
        );
    }
}

#[test]
fn project_tab_commands_activate_the_project_workspace_and_exact_tab() {
    for page in ProjectPage::ALL {
        let mut app = RSpiceApp::test_instance();
        app.state.open_project_session();
        app.state.workbench.workspace = Workspace::Results;
        app.state.project.project_page = ProjectPage::Overview;

        Command::ProjectPage(page).execute(&mut app);

        assert_eq!(app.state.workbench.workspace, Workspace::Project);
        assert_eq!(app.state.project.project_page, page);
    }
}

#[test]
fn project_operation_gate_covers_every_mutating_project_command() {
    for command in [
        Command::ProjectLauncher,
        Command::RecentProjects,
        Command::NewProject,
        Command::OpenProject,
        Command::Save,
        Command::SaveAs,
        Command::SaveAll,
        Command::RevertActiveDocument,
        Command::CloseActiveDocument,
        Command::CloseProject,
        Command::NewCell,
        Command::OpenDocument,
        Command::ImportNetlist,
        Command::ImportVerilogA,
        Command::CheckAndSave,
    ] {
        assert!(
            command.blocked_by_project_operation(),
            "ungated: {command:?}"
        );
    }
    assert!(!Command::Copy.blocked_by_project_operation());
    let blocked = |command: ResultsCommand| {
        command
            .declared_spec()
            .flags
            .contains(CommandFlags::BLOCKED_BY_PROJECT_OPERATION)
    };
    assert!(blocked(ResultsCommand::ImportResultDataset));
    assert!(!blocked(ResultsCommand::ExportWaveformsCsv));
    assert!(
        ModelsCommand::ModelEditor
            .declared_spec()
            .flags
            .contains(CommandFlags::BLOCKED_BY_PROJECT_OPERATION)
    );
}

#[test]
fn closed_projects_expose_only_the_project_workspace() {
    assert!(workspace_available(false, Workspace::Project));
    for workspace in Workspace::ALL {
        if workspace != Workspace::Project {
            assert!(!workspace_available(false, workspace));
        }
        assert!(workspace_available(true, workspace));
    }
}

#[test]
fn new_cell_command_captures_exact_library_catalog_revision() {
    let mut app = RSpiceApp::test_instance();
    let revision = app.state.model.library_manager.revision();

    Command::NewCell.execute(&mut app);

    assert!(app.state.project.new_cell_dialog);
    assert_eq!(app.state.project.new_cell_library_revision, revision);
}

#[test]
fn project_owned_subcommands_cannot_bypass_the_closed_project_boundary() {
    // This independent expectation list prevents the predicate under test
    // from silently omitting a newly exposed submenu route.
    for command in [
        Command::NewCell,
        Command::ImportNetlist,
        Command::ImportVerilogA,
        Command::ExportNetlist(crate::io::NetlistFormat::Spice),
        Command::FindInDesign,
        Command::CheckAndSave,
        Command::SelectionBulkEdit,
        Command::ConnectivityManager,
        Command::ProjectPage(ProjectPage::Overview),
        Command::ProjectPage(ProjectPage::Library),
        Command::ProjectPage(ProjectPage::Configuration),
        Command::ProjectPage(ProjectPage::Dependencies),
        Command::ProjectPage(ProjectPage::Recovery),
        Command::SimulationPage(SimulationPage::Variables),
        Command::PreflightChecks,
        Command::ManageSimulationPlans,
        Command::SimulationOptions,
        Command::GenerateNetlist,
        Command::VerificationPage(VerificationPage::Yield),
        Command::CompileVerilogA,
        Command::AutomationConsole,
    ] {
        assert!(
            command.requires_open_project(),
            "missing closed-project boundary: {command:?}"
        );
    }
    for command in [
        ResultsCommand::ImportResultDataset,
        ResultsCommand::ExportWaveformsCsv,
        ResultsCommand::WaveformCalculator,
        ResultsCommand::CompareResultDatasets,
        ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Waves),
        ResultsCommand::EditSpecifications,
    ] {
        assert!(
            command
                .declared_spec()
                .flags
                .contains(CommandFlags::REQUIRES_OPEN_PROJECT),
            "missing closed-project boundary: {command:?}"
        );
    }
    for command in [
        ModelsCommand::Page(ModelsPage::Models),
        ModelsCommand::ModelBrowser,
        ModelsCommand::ModelEditor,
        ModelsCommand::PdkSettings,
        ModelsCommand::RescanModelLibraries,
    ] {
        assert!(
            command
                .declared_spec()
                .flags
                .contains(CommandFlags::REQUIRES_OPEN_PROJECT),
            "missing closed-project boundary: {command:?}"
        );
    }

    let commands: Vec<_> = vocabulary::COMMAND_REGISTRY
        .iter()
        .copied()
        .filter(|command| command.requires_open_project())
        .collect();
    assert!(!commands.is_empty());

    for command in commands {
        let mut app = RSpiceApp::test_instance();
        app.state.close_project_session();
        app.state.workbench.workspace = Workspace::Project;

        assert!(
            !command.is_enabled(&app),
            "enabled without project: {command:?}"
        );
        assert_eq!(
            command.availability(&app),
            CommandAvailability::Disabled("no project is open"),
            "wrong closed-project reason for {command:?}"
        );

        let before = app.state.model.log_buffer.revision();
        command.execute(&mut app);

        assert_eq!(
            app.state.workbench.workspace,
            Workspace::Project,
            "closed-project command changed workspace: {command:?}"
        );
        assert_eq!(
            app.state.model.log_buffer.revision(),
            before,
            "a disabled command is silent: {command:?}"
        );
        assert_eq!(
            app.state.prompt_text(),
            None,
            "a disabled command is silent: {command:?}"
        );
    }

    // The hardcopy feature's commands hold the same boundary, every one of
    // them, through the dispatcher by key.
    for command in HardcopyCommand::LISTED
        .into_iter()
        .chain(HardcopyCommand::REPORT_TOOLBAR)
    {
        let key = command.key();
        assert!(
            command
                .declared_spec()
                .flags
                .contains(CommandFlags::REQUIRES_OPEN_PROJECT),
            "missing closed-project boundary: {command:?}"
        );
        let mut app = RSpiceApp::test_instance();
        app.state.close_project_session();
        app.state.workbench.workspace = Workspace::Project;

        assert!(
            !app.command_key_enabled(key),
            "enabled without project: {command:?}"
        );
        assert_eq!(
            app.command_availability(key),
            CommandAvailability::Disabled("no project is open"),
            "wrong closed-project reason for {command:?}"
        );

        let before = app.state.model.log_buffer.revision();
        app.execute_command_key(key);

        assert_eq!(
            app.state.workbench.workspace,
            Workspace::Project,
            "closed-project command changed workspace: {command:?}"
        );
        assert_eq!(
            app.state.model.log_buffer.revision(),
            before,
            "a disabled command is silent: {command:?}"
        );
        assert_eq!(
            app.state.prompt_text(),
            None,
            "a disabled command is silent: {command:?}"
        );
    }

    // Publishing to the web needs a project too. A build with no cloud
    // endpoints offers the row not at all, and that rule outranks the
    // closed-project reason.
    let publish = CloudCommand::PublishToWeb;
    assert!(
        publish
            .declared_spec()
            .flags
            .contains(CommandFlags::REQUIRES_OPEN_PROJECT),
        "missing closed-project boundary: {publish:?}"
    );
    let mut app = RSpiceApp::test_instance();
    app.state.close_project_session();
    app.state.workbench.workspace = Workspace::Project;
    assert_eq!(
        app.cloud.account.availability(),
        crate::services::cloud_account::CloudAccountAvailability::UnconfiguredBuild
    );
    assert!(!app.command_key_enabled(publish.key()));
    assert_eq!(
        app.command_availability(publish.key()),
        CommandAvailability::Hidden
    );
    let before = app.state.model.log_buffer.revision();
    app.execute_command_key(publish.key());
    assert_eq!(app.state.workbench.workspace, Workspace::Project);
    assert_eq!(app.state.model.log_buffer.revision(), before);
    assert_eq!(app.state.prompt_text(), None);
    assert!(!app.state.cloud.publish_web.open);

    // The results feature's commands that need a project hold it too.
    let results: Vec<_> = ResultsCommand::ALL
        .into_iter()
        .filter(|command| {
            command
                .declared_spec()
                .flags
                .contains(CommandFlags::REQUIRES_OPEN_PROJECT)
        })
        .collect();
    assert!(!results.is_empty());
    for command in results {
        let key = command.key();
        let mut app = RSpiceApp::test_instance();
        app.state.close_project_session();
        app.state.workbench.workspace = Workspace::Project;

        assert!(
            !app.command_key_enabled(key),
            "enabled without project: {command:?}"
        );
        assert_eq!(
            app.command_availability(key),
            CommandAvailability::Disabled("no project is open"),
            "wrong closed-project reason for {command:?}"
        );

        let before = app.state.model.log_buffer.revision();
        app.execute_command_key(key);

        assert_eq!(
            app.state.workbench.workspace,
            Workspace::Project,
            "closed-project command changed workspace: {command:?}"
        );
        assert_eq!(
            app.state.model.log_buffer.revision(),
            before,
            "a disabled command is silent: {command:?}"
        );
        assert_eq!(
            app.state.prompt_text(),
            None,
            "a disabled command is silent: {command:?}"
        );
    }
}

#[test]
fn standalone_schematic_save_remains_available_without_a_project() {
    let mut app = RSpiceApp::test_instance();
    app.state.close_project_session();
    app.state.model.schematic.session.current_file = Some("standalone.rsch".into());

    assert!(!Command::Save.requires_open_project());
    assert!(Command::Save.is_enabled(&app));
    assert_eq!(
        Command::Save.availability(&app),
        CommandAvailability::Available
    );

    app.state.model.schematic.session.current_file = None;
    app.state.platform.browser_schematic_save_name = Some("browser-import.rsch".to_owned());
    assert!(Command::Save.is_enabled(&app));
    assert_eq!(
        Command::Save.availability(&app),
        CommandAvailability::Available
    );
}

#[test]
fn recent_projects_opens_the_launcher_on_the_real_recent_filter() {
    let mut state = AppState::default();
    state.project.project_launcher_filter = ProjectLauncherFilter::Pinned;
    state.project.project_launcher_open = false;
    let _ = state.workbench.outbox.take::<FocusProjectLauncherSearch>();

    open_recent_projects(&mut state);

    assert!(state.project.project_launcher_open);
    assert!(
        state
            .workbench
            .outbox
            .peek::<FocusProjectLauncherSearch>()
            .is_some()
    );
    assert_eq!(
        state.project.project_launcher_page,
        crate::workbench::project::workspace_state::ProjectLauncherPage::Projects
    );
    assert_eq!(
        state.project.project_launcher_filter,
        ProjectLauncherFilter::Recent
    );
}
