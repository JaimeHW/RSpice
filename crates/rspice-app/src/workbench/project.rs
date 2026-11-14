//! The project feature: the Project workspace, the project launcher and the
//! Library/Cellview Manager.
//!
//! What the workspace keeps between frames is declared here, beneath the
//! session aggregate that holds the project slot and the wire that persists
//! it, so both name it without reaching up into the surfaces.

pub(crate) mod commands;
pub(crate) mod configuration_state;
pub(crate) mod library_dialogs;
mod new_project;
mod revision_history;
pub(crate) mod workspace_state;

use rspice_app_state::veriloga_library::builtin_veriloga_model;

use crate::state::ViewType;
use crate::state::command_catalog::CommandKey;
use crate::workbench::app::dialogs::check_and_save::CheckAndSaveParts;
use crate::workbench::app::dialogs::design_management::{
    DesignManagementParts, DesignManagementTab, open_design_management_on,
    render_design_management_dialog,
};
use crate::workbench::app::dialogs::project::configuration::{
    ConfigurationTransaction, commit_configuration,
};
use crate::workbench::app::dialogs::project::configuration_dialogs::{
    ConfigurationDialogParts, render_configuration_dialog,
};
use crate::workbench::app::{open_check_and_save_dialog, open_design_management_dialog};
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::docks::inspector::project::{ProjectInspector, ProjectInspectorAction};
use crate::workbench::docks::navigator::project::ProjectNavigator;
use crate::workbench::framework::command_registry::{CommandAvailability, CommandSpec, ids};
use crate::workbench::framework::dialogs::DialogKey;
use crate::workbench::framework::feature::{
    CommandSet, Cx, Feature, FeatureDialog, FeatureInspector, FeatureNavigator, FeatureRegistry,
    NoHandles, Peers, Reads,
};
use crate::workbench::framework::requests::{FocusProjectLauncherSearch, ProjectFileAction};
use crate::workbench::lifecycle::project_lifecycle::{
    LifecycleParts, LifecycleView, has_unsaved_changes,
};
use crate::workbench::slots::ProjectState;
use crate::workbench::state::{WorkbenchSession, Workspace};
use crate::workbench::{RouteTransitionSource, SurfaceId, SurfaceRoute};
use commands::ProjectCommand;
use library_dialogs::new_view_target;
use revision_history::open_project_revision_history;
use workspace_state::{ProjectLauncherFilter, ProjectLauncherPage, ProjectPage};

/// The project feature.
pub(crate) struct ProjectFeature;

impl Feature for ProjectFeature {
    type State = ProjectState;
    type Session = WorkbenchSession;
    type Handles = NoHandles;
    /// The Overview offers these, each drawn as the dispatcher answers it.
    const FOREIGN_AVAILABILITY: &'static [CommandKey] = &[
        ids::open_workspace(Workspace::Simulate),
        ids::open_workspace(Workspace::Results),
        ids::open_workspace(Workspace::Netlist),
        ids::CONFIGURATION_SETS,
        ids::MODELS_CATALOG,
        ids::OPEN_NETLIST,
        ProjectCommand::NewCell.key(),
        ProjectCommand::RevisionHistory.key(),
        ProjectCommand::Page(ProjectPage::Library).key(),
    ];

    fn register(registry: &mut FeatureRegistry<Self>) {
        registry.commands::<ProjectCommand>();
        registry.navigator(FeatureNavigator {
            show: show_navigator,
        });
        registry.inspector(FeatureInspector {
            show: show_inspector,
            // What the section holds typed is committed by its own fields as
            // the reader leaves them, so it holds nothing for the shell to
            // commit.
            commit_pending_edit: |_, _| {},
        });
        registry.dialogs(DIALOGS);
    }
}

/// The dialog steps the feature draws, under the keys the shell's dialog
/// order lists them by, in that order: New project, Check and save, Design
/// management, Revision history, the configuration dialogs, New Cell, New
/// View, Copy Cell, Rename Cell, the deletion review and the staged
/// deletion. Then the library and view membership dialogs the Library
/// surface draws, which hold a modal gate while open. Each is open while its
/// form is. The staged deletion draws nothing, so nothing of it is ever open.
/// Last come the technology attachment dialog, the launcher and the
/// examples browser. Their steps only hold the gate, in the order the
/// shell's modal-only list names them, and each is drawn where its work is.
const DIALOGS: &[FeatureDialog<ProjectFeature>] = &[
    FeatureDialog {
        key: DialogKey("project.new_project"),
        is_open: |project| project.new_project.open,
        render: ProjectFeature::render_new_project_dialog,
    },
    FeatureDialog {
        key: DialogKey("project.check_and_save"),
        is_open: |project| project.check_and_save.open,
        render: ProjectFeature::render_check_and_save_dialog,
    },
    FeatureDialog {
        key: DialogKey("project.design_management"),
        is_open: |project| project.design_management.open,
        render: ProjectFeature::render_design_management_dialog,
    },
    FeatureDialog {
        key: DialogKey("project.project_revision_history"),
        is_open: |project| project.project_revision_history.open,
        render: ProjectFeature::render_project_revision_history_dialog,
    },
    FeatureDialog {
        key: DialogKey("project.configuration"),
        is_open: |project| {
            project.configuration_dialog.is_open()
                || project.configuration_dialog.leaving.is_some()
                || project.configuration.pending_selection.is_some()
        },
        render: ProjectFeature::render_configuration_dialog,
    },
    FeatureDialog {
        key: DialogKey("project.new_cell"),
        is_open: |project| project.new_cell_dialog,
        render: ProjectFeature::process_new_cell_dialog,
    },
    FeatureDialog {
        key: DialogKey("project.new_view"),
        is_open: |project| project.new_view_dialog,
        render: ProjectFeature::process_new_view_dialog,
    },
    FeatureDialog {
        key: DialogKey("project.copy_cell"),
        is_open: |project| project.copy_cell_dialog,
        render: ProjectFeature::process_copy_cell_dialog,
    },
    FeatureDialog {
        key: DialogKey("project.rename_cell"),
        is_open: |project| project.rename_cell_dialog,
        render: ProjectFeature::process_rename_cell_dialog,
    },
    FeatureDialog {
        key: DialogKey("project.library_deletion_review"),
        is_open: |project| project.library_deletion_review.target.is_some(),
        render: ProjectFeature::process_library_deletion_review_dialog,
    },
    FeatureDialog {
        key: DialogKey("project.pending_library_deletions"),
        is_open: |_| false,
        render: |cx, _| ProjectFeature::process_pending_library_deletions(cx),
    },
    FeatureDialog {
        key: DialogKey("project.new_library"),
        is_open: |project| project.new_library_dialog,
        render: ProjectFeature::process_new_library_dialog,
    },
    FeatureDialog {
        key: DialogKey("project.rename_library"),
        is_open: |project| project.rename_library_dialog,
        render: ProjectFeature::process_rename_library_dialog,
    },
    FeatureDialog {
        key: DialogKey("project.delete_library"),
        is_open: |project| project.delete_library_dialog,
        render: ProjectFeature::process_delete_library_dialog,
    },
    FeatureDialog {
        key: DialogKey("project.rename_view"),
        is_open: |project| project.rename_view_dialog,
        render: ProjectFeature::process_rename_view_dialog,
    },
    FeatureDialog {
        key: DialogKey("project.technology_attachment"),
        is_open: |project| project.technology_attachment.open,
        // The shell host draws it on the Project workspace's stage, after
        // the page or the landing, in the feature's context.
        render: |_, _| {},
    },
    FeatureDialog {
        key: DialogKey("project.launcher"),
        is_open: |project| project.project_launcher_open,
        // The frame draws it through the shell host's launcher
        // composition.
        render: |_, _| {},
    },
    FeatureDialog {
        key: DialogKey("project.examples"),
        is_open: |project| project.examples_selection.is_some(),
        // The frame's examples composition draws it.
        render: |_, _| {},
    },
];

impl ProjectFeature {
    /// Keep browser-restored manager routes and the transient egui dialog
    /// owner synchronized. Commands open both in one transaction; this hook
    /// covers deep links, Back/Forward, and restored application sessions.
    pub(crate) fn synchronize_design_management_route(cx: &mut Cx<'_, Self>) {
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

    /// Commit `transaction` over the Configuration page, the model and the
    /// run tier a committed change voids, and keep its refusal, if any, for
    /// the page to state.
    pub(in crate::workbench) fn commit_configuration(
        cx: &mut Cx<'_, Self>,
        transaction: ConfigurationTransaction,
    ) {
        let refusal = commit_configuration(
            &mut cx.state.configuration,
            cx.model,
            cx.runs,
            cx.handles.run_controller,
            transaction,
        )
        .err();
        cx.state.configuration.refusal = refusal;
    }

    /// Draw the configuration dialogs over what the feature's context lends
    /// them. Going on leaving the project once its configuration edits are
    /// settled is asked of the shell, which carries it out as they return.
    pub(in crate::workbench) fn render_configuration_dialog(
        cx: &mut Cx<'_, Self>,
        ctx: &egui::Context,
    ) {
        let project = &mut *cx.state;
        render_configuration_dialog(
            &mut ConfigurationDialogParts {
                dialogs: &mut project.configuration_dialog,
                configuration: &mut project.configuration,
                project_page: &mut project.project_page,
                model: &mut *cx.model,
                platform: &*cx.platform,
                wb: &mut *cx.wb,
                runs: &mut *cx.runs,
                run_controller: &mut *cx.handles.run_controller,
            },
            ctx,
        );
    }

    /// Draw Design management over what the feature's context lends it.
    /// Create hierarchy, which design owns, is asked of the shell, which
    /// opens it as the dialog returns.
    pub(in crate::workbench) fn render_design_management_dialog(
        cx: &mut Cx<'_, Self>,
        ctx: &egui::Context,
    ) {
        let project = &mut *cx.state;
        render_design_management_dialog(
            &mut DesignManagementParts {
                dialog: &mut project.design_management,
                project_page: &mut project.project_page,
                configuration_tab: &mut project.configuration.tab,
                model: &mut *cx.model,
                platform: &*cx.platform,
                wb: &mut *cx.wb,
                runs: &mut *cx.runs,
                run_controller: &mut *cx.handles.run_controller,
            },
            ctx,
        );
    }

    /// Draw Check and save over what the feature's context lends it. Saving
    /// the validated revision is asked of the shell, which carries it out as
    /// the dialog returns.
    pub(in crate::workbench) fn render_check_and_save_dialog(
        cx: &mut Cx<'_, Self>,
        ctx: &egui::Context,
    ) {
        check_and_save_parts(cx).render_check_and_save_dialog(ctx);
    }
}

/// What Check and save is lent of the project feature's context: its dialog,
/// the model, the session and the platform state, what the design checks
/// read of the run tier and design, and what its report reads of the results
/// workspace, design and code.
fn check_and_save_parts<'c>(cx: &'c mut Cx<'_, ProjectFeature>) -> CheckAndSaveParts<'c> {
    CheckAndSaveParts {
        dialog: &mut cx.state.check_and_save,
        model: &mut *cx.model,
        platform: &*cx.platform,
        wb: &mut *cx.wb,
        facts: cx.runs.slot_facts(cx.peers.design().review_markers()),
        presentation: cx.peers.results().saved_presentation(),
        design: cx.peers.design(),
        code: cx.peers.code(),
        commands: cx.commands,
    }
}

/// The project as the lifecycle reads it, from the parts of the project
/// feature's context it reads: the model, the platform state, the workspace
/// in front and the results workspace's saved presentation.
pub(in crate::workbench) fn lifecycle_view<'a>(
    model: &'a Model,
    platform: &'a PlatformState,
    wb: &WorkbenchSession,
    peers: &'a Peers<'_>,
) -> LifecycleView<'a> {
    LifecycleView {
        model,
        platform,
        workspace: wb.workspace,
        presentation: peers.results().saved_presentation(),
    }
}

/// The project where the lifecycle changes it, from the same parts as
/// [`lifecycle_view`], the model and the platform state lent mutably.
pub(in crate::workbench) fn lifecycle_parts<'a>(
    model: &'a mut Model,
    platform: &'a mut PlatformState,
    wb: &WorkbenchSession,
    peers: &'a Peers<'_>,
) -> LifecycleParts<'a> {
    LifecycleParts {
        model,
        platform,
        workspace: wb.workspace,
        presentation: peers.results().saved_presentation(),
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

/// The Project workspace's inspector: the page in front, over the feature's
/// state and the model, with what the design checks read of the other
/// features and the project as its lifecycle reads it. What the reader asked
/// of it is raised once it has drawn: a command, a configuration to use for
/// runs, which the feature commits in place so the Configuration page states
/// a refusal in the same frame, or what only the shell carries out.
fn show_inspector(cx: &mut Cx<'_, ProjectFeature>, ui: &mut egui::Ui) {
    let model = &*cx.model;
    let project = &mut *cx.state;
    let action = crate::workbench::docks::inspector::project::project(
        ui,
        &mut ProjectInspector {
            page: project.project_page,
            configuration: &mut project.configuration,
            dependency_selection: &project.project_dependency_selection,
            checkpoint_recovery: &project.project_checkpoint_recovery,
            checkpoint_selection: &project.project_checkpoint_selection,
            builtin_library_open: project.builtin_models.open,
            builtin_model: project
                .builtin_models
                .model
                .as_deref()
                .and_then(builtin_veriloga_model),
            model,
            facts: cx.runs.slot_facts(cx.peers.design().review_markers()),
            lifecycle: lifecycle_view(model, cx.platform, cx.wb, &cx.peers),
        },
    );
    match action {
        Some(ProjectInspectorAction::Execute(key)) => cx.execute(key),
        Some(ProjectInspectorAction::UseForRuns(target)) => {
            ProjectFeature::commit_configuration(cx, ConfigurationTransaction::UseForRuns(target))
        }
        Some(ProjectInspectorAction::Shell(request)) => cx.wb.outbox.push(request),
        None => {}
    }
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

/// Open the project launcher with every project listed: File ▸ Open recent
/// ends in it, so the launcher it opens is not narrowed again to the recent
/// ones.
fn open_all_projects(
    project: &mut ProjectState,
    wb: &mut WorkbenchSession,
    platform: &mut PlatformState,
) {
    open_project_launcher(project, wb, platform);
    project.project_launcher_filter = ProjectLauncherFilter::All;
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
    tab: DesignManagementTab,
) {
    let route = SurfaceRoute::surface(SurfaceId::DesignManagement);
    match wb.navigate(route, RouteTransitionSource::User) {
        Ok(_) => {
            open_design_management_on(&mut project.design_management, model, platform, wb, tab)
        }
        Err(error) => wb.prompt(error.to_string()),
    }
}

/// Open New View on the cell whose schematic is in front, preset to a
/// symbol; why it cannot open there is stated on the status bar.
fn open_create_symbol_dialog(project: &mut ProjectState, model: &Model, wb: &mut WorkbenchSession) {
    let reference = model.workspace.content.active_schematic_reference();
    if let Err(error) = project.open_new_view_dialog(
        model,
        &reference.library,
        &reference.cell,
        Some(ViewType::Symbol),
    ) {
        wb.prompt(error);
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
            Self::DesignManagement | Self::Variants | Self::Annotate => (
                schematic_in_front(reads) && open && !reads.model.schematic_edit_read_only(),
                "open an editable schematic",
            ),
            Self::CreateSymbol => (
                schematic_in_front(reads) && open,
                "open a schematic or testbench",
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

    /// The file commands are the shell's file workflow's, each raised for
    /// the shell to take as soon as the command returns. Check and save opens
    /// in the feature's context, over what the other features show.
    fn execute(self, cx: &mut Cx<'_, ProjectFeature>) {
        match self {
            Self::ProjectLauncher => open_all_projects(cx.state, cx.wb, cx.platform),

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
            Self::DesignManagement => open_design_management(
                cx.state,
                cx.model,
                cx.platform,
                cx.wb,
                DesignManagementTab::Sheets,
            ),
            Self::Variants => open_design_management(
                cx.state,
                cx.model,
                cx.platform,
                cx.wb,
                DesignManagementTab::Variants,
            ),
            Self::Annotate => open_design_management(
                cx.state,
                cx.model,
                cx.platform,
                cx.wb,
                DesignManagementTab::Annotation,
            ),
            Self::CreateSymbol => open_create_symbol_dialog(cx.state, cx.model, cx.wb),
            Self::RevisionHistory => open_project_revision_history(
                &mut cx.state.project_revision_history,
                cx.model,
                cx.platform,
            ),
            Self::CheckAndSave => open_check_and_save_dialog(&mut check_and_save_parts(cx)),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests;
