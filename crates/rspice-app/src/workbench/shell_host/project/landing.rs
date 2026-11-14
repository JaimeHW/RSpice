//! The landing's composition over the whole application.

use egui::Ui;

use crate::workbench::framework::command_registry::ids;
use crate::workbench::project::commands::ProjectCommand;
use crate::workbench::project::workspace_state::ProjectLauncherPage;
use crate::workbench::surfaces::project::landing::{LandingAction, landing};
use crate::workbench::{RSpiceApp, features};

/// The landing, drawn in the project feature's context, with what the reader
/// asked of it carried out by the shell as soon as it has drawn.
pub(crate) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    crate::workbench::lifecycle::recovery::refresh_catalog_if_requested(
        &mut app.state.platform,
        &app.state.model,
        app.file_workflow_io.as_ref(),
    );

    let action = landing(ui, &mut features::cx(app));

    if let Some(action) = action {
        execute_action(app, action);
    }
}

fn execute_action(app: &mut RSpiceApp, action: LandingAction) {
    match action {
        LandingAction::NewProject => app.execute_command_key(ProjectCommand::NewProject.key()),
        LandingAction::OpenProject => app.execute_command_key(ProjectCommand::OpenProject.key()),
        LandingAction::OpenNetlist => app.execute_command_key(ids::OPEN_NETLIST),
        LandingAction::OpenSchematic => {
            app.execute_command_key(ProjectCommand::OpenSchematic.key())
        }
        LandingAction::OpenRecent(recent) => {
            #[cfg(not(target_arch = "wasm32"))]
            app.open_recent_file(recent);
            #[cfg(target_arch = "wasm32")]
            {
                let _ = recent;
                app.execute_command_key(ProjectCommand::OpenProject.key());
            }
        }
        LandingAction::ReviewRecovery => {
            crate::workbench::project::open_project_launcher(
                &mut app.state.project,
                &mut app.state.workbench,
                &mut app.state.platform,
            );
            app.state.project.project_launcher_page = ProjectLauncherPage::Recovery;
            app.state.platform.recovery_catalog.request_refresh();
        }
    }
}
