//! The project feature's own tests: the harness they run against, and what
//! the feature's commands do over its state, the model and the session.
//! What the shell's prologue adds, and how the commands are listed, is
//! tested with the shell's commands.

use rspice_app_state::workbench::workspace::restore_model_library_manager;

use crate::state::pdk_config::PdkConfig;
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::harness::FeatureHarness;
use crate::workbench::framework::requests::FocusProjectLauncherSearch;
use crate::workbench::state::{WorkbenchSession, Workspace};

use super::ProjectFeature;
use super::commands::ProjectCommand;
use super::workspace_state::{ProjectLauncherFilter, ProjectPage};

/// The project feature on its own: nothing of its own open, over the model a
/// new session starts with (from the empty PDK configuration a test session
/// reads) and a new session, with every peer answering as a fresh session
/// does.
pub(crate) fn harness() -> FeatureHarness<ProjectFeature> {
    FeatureHarness::with_model(
        Model::session_start(PdkConfig::default(), None, restore_model_library_manager),
        WorkbenchSession::default(),
    )
}

#[test]
fn project_tab_commands_activate_the_project_workspace_and_exact_tab() {
    for page in ProjectPage::ALL {
        let mut harness = harness();
        harness.open_project_session();
        harness.wb.workspace = Workspace::Results;
        harness.state.project_page = ProjectPage::Overview;

        harness.execute(ProjectCommand::Page(page).key());

        assert_eq!(harness.wb.workspace, Workspace::Project);
        assert_eq!(harness.state.project_page, page);
    }
}

#[test]
fn new_cell_command_captures_exact_library_catalog_revision() {
    let mut harness = harness();
    let revision = harness.model.library_manager.revision();

    harness.execute(ProjectCommand::NewCell.key());

    assert!(harness.state.new_cell_dialog);
    assert_eq!(harness.state.new_cell_library_revision, revision);
}

/// All projects…, which ends File ▸ Open recent, opens the launcher on every
/// project, not on the filter it was left on.
#[test]
fn all_projects_opens_the_launcher_on_every_project() {
    let mut harness = harness();
    harness.state.project_launcher_filter = ProjectLauncherFilter::Recent;
    harness.state.project_launcher_open = false;
    let _ = harness.wb.outbox.take::<FocusProjectLauncherSearch>();

    harness.execute(ProjectCommand::ProjectLauncher.key());

    assert!(harness.state.project_launcher_open);
    assert!(
        harness
            .wb
            .outbox
            .peek::<FocusProjectLauncherSearch>()
            .is_some()
    );
    assert_eq!(
        harness.state.project_launcher_page,
        crate::workbench::project::workspace_state::ProjectLauncherPage::Projects
    );
    assert_eq!(
        harness.state.project_launcher_filter,
        ProjectLauncherFilter::All
    );
}
