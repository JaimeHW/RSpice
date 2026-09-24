//! Project-scoped commands and the closed-project boundary.
//!
//! With no project open the product offers the Project workspace and nothing
//! else, and every mutating project command is refused while a project
//! operation is still in flight. The boundary is asserted per command rather
//! than once, because each one reaches it by its own route and a subcommand
//! that skips the gate is invisible to a single aggregate check.

use super::*;

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
        app.state.project_lifecycle.project_open = true;
        app.state.workbench.workspace = Workspace::Results;
        app.state.workbench.project_page = ProjectPage::Overview;

        Command::ProjectPage(page).execute(&mut app);

        assert_eq!(app.state.workbench.workspace, Workspace::Project);
        assert_eq!(app.state.workbench.project_page, page);
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
        Command::ImportResultDataset,
        Command::CheckAndSave,
        Command::ModelEditor,
    ] {
        assert!(
            command.blocked_by_project_operation(),
            "ungated: {command:?}"
        );
    }
    assert!(!Command::Copy.blocked_by_project_operation());
    assert!(!Command::ExportWaveformsCsv.blocked_by_project_operation());
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
    let revision = app.state.library_manager.revision();

    Command::NewCell.execute(&mut app);

    assert!(app.state.dialogs.new_cell_dialog);
    assert_eq!(app.state.dialogs.new_cell_library_revision, revision);
}

#[test]
fn project_owned_subcommands_cannot_bypass_the_closed_project_boundary() {
    // This independent expectation list prevents the predicate under test
    // from silently omitting a newly exposed submenu route.
    for command in [
        Command::NewCell,
        Command::ImportNetlist,
        Command::ImportVerilogA,
        Command::ImportResultDataset,
        Command::ExportSchematicSvg,
        Command::ExportWaveformsCsv,
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
        Command::WaveformCalculator,
        Command::CompareResultDatasets,
        Command::ResultViewer(crate::workbench::ResultViewer::Waves),
        Command::EditSpecifications,
        Command::VerificationPage(VerificationPage::Yield),
        Command::ModelsPage(ModelsPage::Models),
        Command::ModelBrowser,
        Command::ModelEditor,
        Command::PdkSettings,
        Command::RescanModelLibraries,
        Command::CompileVerilogA,
        Command::AutomationConsole,
        Command::VisualizationStudio,
        Command::ReportAuthoring,
    ] {
        assert!(
            command.requires_open_project(),
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
        app.state.project_lifecycle.project_open = false;
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

        command.execute(&mut app);

        assert_eq!(
            app.state.workbench.workspace,
            Workspace::Project,
            "closed-project command changed workspace: {command:?}"
        );
        assert!(
            app.state
                .log_buffer
                .entries()
                .any(|entry| entry.message == "Open a project before using this command."),
            "closed-project command did not explain its boundary: {command:?}"
        );
    }
}

#[test]
fn standalone_schematic_save_remains_available_without_a_project() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = false;
    app.state.schematic.current_file = Some("standalone.rsch".into());

    assert!(!Command::Save.requires_open_project());
    assert!(Command::Save.is_enabled(&app));
    assert_eq!(
        Command::Save.availability(&app),
        CommandAvailability::Available
    );

    app.state.schematic.current_file = None;
    app.state.browser_schematic_save_name = Some("browser-import.rsch".to_owned());
    assert!(Command::Save.is_enabled(&app));
    assert_eq!(
        Command::Save.availability(&app),
        CommandAvailability::Available
    );
}

#[test]
fn recent_projects_opens_the_launcher_on_the_real_recent_filter() {
    let mut workbench = WorkbenchState::default();
    workbench.project_launcher_filter = ProjectLauncherFilter::Pinned;
    workbench.project_launcher_open = false;
    workbench.focus_project_launcher_search = false;

    open_recent_projects(&mut workbench);

    assert!(workbench.project_launcher_open);
    assert!(workbench.focus_project_launcher_search);
    assert_eq!(
        workbench.project_launcher_page,
        crate::workbench::state::ProjectLauncherPage::Projects
    );
    assert_eq!(
        workbench.project_launcher_filter,
        ProjectLauncherFilter::Recent
    );
}
