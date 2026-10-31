//! The project feature: the Project workspace, the project launcher and the
//! Library/Cellview Manager.
//!
//! What the workspace keeps between frames is declared here, beneath the
//! session aggregate that holds the project slot and the wire that persists
//! it, so both name it without reaching up into the surfaces.

pub(crate) mod commands;
pub(crate) mod configuration_state;
pub(crate) mod workspace_state;

use crate::state::ViewType;
use crate::workbench::app::dialogs::library::new_view_target;
use crate::workbench::app::{open_design_management_dialog, open_project_revision_history};
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::docks::navigator::project::ProjectNavigator;
use crate::workbench::framework::command_registry::{CommandAvailability, CommandSpec};
use crate::workbench::framework::feature::{
    CommandSet, Cx, Feature, FeatureNavigator, FeatureRegistry, NoHandles, Reads,
};
use crate::workbench::framework::requests::{
    FocusProjectLauncherSearch, OpenProjectDialog, ProjectFileAction,
};
use crate::workbench::lifecycle::project_lifecycle::{LifecycleView, has_unsaved_changes};
use crate::workbench::slots::ProjectState;
use crate::workbench::state::{WorkbenchSession, Workspace};
use crate::workbench::{RouteTransitionSource, SurfaceId, SurfaceRoute};
use commands::ProjectCommand;
use workspace_state::{ProjectLauncherFilter, ProjectLauncherPage};

/// The project feature.
pub(crate) struct ProjectFeature;

impl Feature for ProjectFeature {
    type State = ProjectState;
    type Session = WorkbenchSession;
    type Handles = NoHandles;

    fn register(registry: &mut FeatureRegistry<Self>) {
        registry.commands::<ProjectCommand>();
        registry.navigator(FeatureNavigator {
            show: show_navigator,
        });
    }
}

impl ProjectFeature {
    /// Keep browser-restored manager routes and the transient egui dialog
    /// owner synchronized. Commands open both in one transaction; this hook
    /// covers deep links, Back/Forward, and restored application sessions.
    pub(in crate::workbench) fn synchronize_design_management_route(cx: &mut Cx<'_, Self>) {
        let route_active = cx.wb.current_route().surface_id() == SurfaceId::DesignManagement;
        if route_active && !cx.state.design_management.open {
            open_design_management_dialog(
                &mut cx.state.design_management,
                cx.model,
                cx.platform,
                cx.wb,
            );
        } else if !route_active && cx.state.design_management.open {
            cx.state.design_management.close_and_discard();
        }
    }
}

/// The Project workspace's navigator: the rail of the page in front, over
/// the feature's state, the model and the navigator's filter.
fn show_navigator(cx: &mut Cx<'_, ProjectFeature>, ui: &mut egui::Ui) {
    let project = &mut *cx.state;
    crate::workbench::docks::navigator::project::project(
        ui,
        &mut ProjectNavigator {
            page: project.project_page,
            configuration: &mut project.configuration,
            configuration_dialog: &mut project.configuration_dialog,
            dependency_selection: &mut project.project_dependency_selection,
            checkpoint_recovery: &project.project_checkpoint_recovery,
            checkpoint_selection: &mut project.project_checkpoint_selection,
            model: cx.model,
            filter: cx.wb.navigator_filter(),
        },
    );
}

#[cfg(target_arch = "wasm32")]
impl ProjectFeature {
    /// Take the next project the browser finished reading or promoting, if
    /// any, for the shell to open.
    pub(in crate::workbench) fn poll_browser_project_import(
        cx: &mut Cx<'_, Self>,
    ) -> Option<crate::workbench::workflows::project_workflow::BrowserProjectOpen> {
        crate::workbench::workflows::project_workflow::poll_browser_project_import(
            cx.model,
            cx.platform,
        )
    }
}

/// Open the project launcher on its Projects page, on the launcher's route,
/// with the recovery catalog asked to refresh and the search field asked to
/// take focus. A route the session refuses is recorded, and the launcher
/// stays closed.
pub(crate) fn open_project_launcher(
    project: &mut ProjectState,
    wb: &mut WorkbenchSession,
    platform: &mut PlatformState,
) {
    let route = SurfaceRoute::surface(SurfaceId::ProjectLauncher);
    if wb.current_route() != route
        && let Err(error) = wb.navigate(route, RouteTransitionSource::User)
    {
        wb.record_route_diagnostic(format!("The Project Launcher could not be opened: {error}"));
        return;
    }
    project.project_launcher_open = true;
    project.project_launcher_page = ProjectLauncherPage::Projects;
    platform.recovery_catalog.request_refresh();
    wb.outbox.push(FocusProjectLauncherSearch);
}

/// Open the project launcher filtered to the recent projects.
fn open_recent_projects(
    project: &mut ProjectState,
    wb: &mut WorkbenchSession,
    platform: &mut PlatformState,
) {
    open_project_launcher(project, wb, platform);
    project.project_launcher_filter = ProjectLauncherFilter::Recent;
}

/// Open New cell on the selected library while it can be written, else on
/// the first library that can, against the catalog revision it opened on.
fn open_new_cell_dialog(project: &mut ProjectState, model: &Model) {
    let selected = model
        .library_manager
        .selected_library
        .as_ref()
        .and_then(|name| {
            model
                .library_manager
                .get_library(name)
                .filter(|library| !library.read_only)
                .map(|library| library.name.clone())
        });
    let target_library = selected.or_else(|| {
        model
            .library_manager
            .libraries_sorted()
            .into_iter()
            .find(|library| !library.read_only)
            .map(|library| library.name.clone())
    });
    let library_revision = model.library_manager.revision();

    project.new_cell_library = target_library.unwrap_or_default();
    project.new_cell_name.clear();
    project.new_cell_description.clear();
    project.new_cell_create_schematic = true;
    project.new_cell_create_symbol = false;
    project.new_cell_create_testbench = false;
    project.new_cell_create_veriloga = false;
    project.new_cell_error = None;
    project.new_cell_library_revision = library_revision;
    project.new_cell_dialog = true;
}

/// Open New View on the cell the command targets; why it cannot open there
/// is stated on the status bar.
fn open_new_view_dialog(project: &mut ProjectState, model: &Model, wb: &mut WorkbenchSession) {
    if let Some((library, cell)) = new_view_target(model)
        && let Err(error) = project.open_new_view_dialog(model, &library, &cell, None)
    {
        wb.prompt(error);
    }
}

/// Open Design management on its own route; a route the session refuses
/// is stated on the status bar.
fn open_design_management(
    project: &mut ProjectState,
    model: &mut Model,
    platform: &PlatformState,
    wb: &mut WorkbenchSession,
) {
    let route = SurfaceRoute::surface(SurfaceId::DesignManagement);
    match wb.navigate(route, RouteTransitionSource::User) {
        Ok(_) => open_design_management_dialog(&mut project.design_management, model, platform, wb),
        Err(error) => wb.prompt(error.to_string()),
    }
}

/// Whether a schematic or testbench is the document in front of the Design
/// workspace.
fn schematic_in_front(reads: &Reads<'_, ProjectFeature>) -> bool {
    reads.wb.workspace == Workspace::Design
        && matches!(
            reads.model.workspace.content.active_view_type(),
            ViewType::Schematic | ViewType::Testbench
        )
}

/// Whether the open project holds work a save would write, read as the
/// project lifecycle reads the application.
fn project_has_unsaved_changes(reads: &Reads<'_, ProjectFeature>) -> bool {
    has_unsaved_changes(&LifecycleView {
        model: reads.model,
        platform: reads.platform,
        workspace: reads.wb.workspace,
        presentation: reads.results.saved_presentation(),
    })
}

impl CommandSet for ProjectCommand {
    type Feature = ProjectFeature;
    const REGISTERED: &'static [Self] = &Self::ALL;

    fn spec(self) -> CommandSpec {
        self.declared_spec()
    }

    /// The launcher, the commands that open or start a project, New cell and
    /// the pages run whenever the shell lets them through. Save needs a
    /// project or a standalone schematic to write, Save all unsaved project
    /// work, New view a cell in a writable library, and the two design
    /// commands that edit the drawing an editable schematic in front.
    fn availability(self, reads: &Reads<'_, ProjectFeature>) -> CommandAvailability {
        let open = reads.platform.project_lifecycle.is_open();
        let (enabled, reason) = match self {
            Self::Save => (
                open || reads.model.schematic.session.current_file.is_some()
                    || reads.platform.browser_schematic_save_name.is_some(),
                "no project is open",
            ),
            Self::SaveAs | Self::CloseProject => (open, "no project is open"),
            Self::SaveAll => (
                open && project_has_unsaved_changes(reads),
                "no unsaved project changes",
            ),
            Self::NewView => (
                new_view_target(reads.model).is_some(),
                "select a cell in a writable library",
            ),
            Self::DesignManagement => (
                schematic_in_front(reads) && open && !reads.model.schematic_edit_read_only(),
                "open an editable schematic",
            ),
            Self::RevisionHistory => (open, "open a project"),
            Self::CheckAndSave if !schematic_in_front(reads) => {
                (false, "open an editable schematic or testbench")
            }
            Self::CheckAndSave => (
                !reads.model.schematic_edit_read_only(),
                "the active schematic is read-only",
            ),
            Self::ProjectLauncher
            | Self::RecentProjects
            | Self::OpenProject
            | Self::NewProject
            | Self::NewCell
            | Self::OpenSchematic
            | Self::Page(_) => (true, ""),
        };
        if enabled {
            CommandAvailability::Available
        } else {
            CommandAvailability::Disabled(reason)
        }
    }

    /// The file commands are the shell's file workflow's, and Check and save,
    /// which runs the design checks beyond the feature's context, is opened
    /// by the shell; each is raised for the shell to take as soon as the
    /// command returns.
    fn execute(self, cx: &mut Cx<'_, ProjectFeature>) {
        match self {
            Self::ProjectLauncher => open_project_launcher(cx.state, cx.wb, cx.platform),
            Self::RecentProjects => open_recent_projects(cx.state, cx.wb, cx.platform),
            Self::NewProject => cx.wb.outbox.push(ProjectFileAction::NewProject),
            Self::OpenProject => cx.wb.outbox.push(ProjectFileAction::OpenProject),
            // File Save always persists the project, including incomplete or
            // unvalidated owned netlist work. Publishing a standalone source
            // file is an explicit Netlist-toolbar operation and must never
            // replace project persistence just because that editor is active.
            Self::Save => cx.wb.outbox.push(ProjectFileAction::Save),
            Self::SaveAs => cx.wb.outbox.push(ProjectFileAction::SaveAs),
            Self::SaveAll => cx.wb.outbox.push(ProjectFileAction::SaveAll),
            Self::CloseProject => cx.wb.outbox.push(ProjectFileAction::CloseProject),
            Self::NewCell => open_new_cell_dialog(cx.state, cx.model),
            Self::NewView => open_new_view_dialog(cx.state, cx.model, cx.wb),
            Self::OpenSchematic => cx.wb.outbox.push(ProjectFileAction::OpenSchematic),
            Self::Page(page) => {
                cx.wb.activate(Workspace::Project);
                cx.state.project_page = page;
            }
            Self::DesignManagement => {
                open_design_management(cx.state, cx.model, cx.platform, cx.wb);
            }
            Self::RevisionHistory => open_project_revision_history(
                &mut cx.state.project_revision_history,
                cx.model,
                cx.platform,
            ),
            Self::CheckAndSave => cx.wb.outbox.push(OpenProjectDialog::CheckAndSave),
        }
    }
}

#[cfg(test)]
mod tests;
