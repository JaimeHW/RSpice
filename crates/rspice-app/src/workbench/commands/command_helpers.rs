//! Command-dispatch helpers that apply concrete workbench state transitions.

use super::*;
use crate::workbench::app::actions::workspace::{
    HistoryStepParts, project_step_comes_first, take_history_step,
};
use crate::workbench::design::commands::DesignCommand;
use crate::workbench::documents::code_workspace::persist_global_veriloga_library;
use crate::workbench::framework::feature::ModelsPort;
use crate::workbench::framework::requests::ProjectFileAction;
use crate::workbench::project::commands::{ProjectRequest, ReservedLibraryRevision};
use crate::workbench::project::workspace_state::ProjectPage;
use crate::workbench::session::project_transactions::go_on_leaving;
use crate::workbench::session::project_transactions::{
    apply_workspace_pruned, void_veriloga_evidence,
};
use rspice_app_code::workbench::code::commands::CodeHandOff;
use rspice_app_project::workbench::app::dialogs::project::configuration_dialogs::Leaving;
use rspice_app_project::workbench::app::dialogs::project::new_project::NewProjectDialogState;
use rspice_app_state::diagnostics::{LogSeverity, LogSource};
use rspice_app_state::state::ViewType;

pub(super) fn open_command_palette(state: &mut AppState) {
    let route = super::super::SurfaceRoute::surface(super::super::SurfaceId::CommandPalette);
    if state.workbench.current_route() != route
        && let Err(error) = state
            .workbench
            .navigate(route, super::super::RouteTransitionSource::User)
    {
        state.prompt(format!("The Command Palette could not be opened: {error}"));
        return;
    }
    state.workbench.command_palette.open_routed();
}

pub(super) fn open_help_center(state: &mut AppState, page: crate::workbench::app::HelpCenterPage) {
    let route = super::super::SurfaceRoute::surface(super::super::SurfaceId::HelpCenter);
    if state.workbench.current_route() != route
        && let Err(error) = state
            .workbench
            .navigate(route, super::super::RouteTransitionSource::User)
    {
        state.prompt(format!("The Help Center could not be opened: {error}"));
        return;
    }
    state.shell.help_center.open_routed(page);
}

/// Whether the netlist document — not merely the workspace that hosts it — is
/// the thing on screen.
/// The label a menu row shows for `command` right now: Undo and Redo name
/// the step they take where the drawing's history names it, Repeat names
/// the command it repeats, and every other row reads the vocabulary's label.
pub(crate) fn menu_label(command: Command, app: &RSpiceApp) -> std::borrow::Cow<'static, str> {
    let label = command.spec().label;
    let named = match command {
        Command::Undo | Command::Redo if command.is_enabled(app) => {
            drawing_step_name(app, command == Command::Undo)
                .map(|step| format!("{label} {}", lower_first_word(step)))
        }
        Command::RepeatLastCommand => {
            vocabulary::last_repeatable(app.state.workbench.last_repeatable_command.as_deref())
                .map(|spec| format!("Repeat {}", lower_first_word(spec.label)))
        }
        _ => None,
    };
    named.map_or(label.into(), Into::into)
}

/// The name of the step Undo (`undo`) or Redo would take, where that step
/// is the schematic or symbol editor's own: their histories name each step.
/// The netlist page's and the project's histories do not, so a step either
/// of them takes first leaves the row unnamed. A project step older than the
/// drawing's does not: the command takes the drawing's step, and says so.
fn drawing_step_name(app: &RSpiceApp, undo: bool) -> Option<&str> {
    let state = &app.state;
    let netlist = netlist_page_is_visible(state)
        && if undo {
            crate::workbench::documents::netlist_document::can_undo_netlist_edit(
                &state.code.netlist,
            )
        } else {
            crate::workbench::documents::netlist_document::can_redo_netlist_edit(
                &state.code.netlist,
            )
        };
    let direction = if undo {
        HistoryDirection::Undo
    } else {
        HistoryDirection::Redo
    };
    if netlist
        || state.workbench.active_source_editor.is_some()
        || project_step_comes_first(&state.model, &state.design.symbol, direction)
    {
        return None;
    }
    if active_symbol_editor(app) {
        let key = state.model.workspace.content.active_key();
        let history = &state.design.symbol.history;
        return if undo {
            history.undo_step(&key)
        } else {
            history.redo_step(&key)
        };
    }
    if !active_schematic_editor(app) {
        return None;
    }
    let history = state.model.schematic.history();
    let step = if undo {
        history.undo_description()
    } else {
        history.redo_description()
    };
    step.map(str::trim).filter(|step| !step.is_empty())
}

/// `text` to follow a verb mid-sentence: a capitalised first word reads in
/// lower case ("Draw wire" becomes "draw wire"), a name or an acronym keeps
/// its case ("R1", "AC").
fn lower_first_word(text: &str) -> String {
    let first_word = text.split_whitespace().next().unwrap_or_default();
    let mut characters = first_word.chars();
    let capitalised =
        characters.next().is_some_and(char::is_uppercase) && characters.all(char::is_lowercase);
    if !capitalised {
        return text.to_owned();
    }
    let mut characters = text.chars();
    characters.next().map_or_else(String::new, |first| {
        first.to_lowercase().chain(characters).collect()
    })
}

pub(super) fn netlist_page_is_visible(state: &crate::workbench::AppState) -> bool {
    crate::workbench::documents::code_workspace::netlist_page_shown(
        state.workbench.workspace,
        &state.code.code_workspace,
    )
}

pub(super) fn activate_workspace(app: &mut RSpiceApp, workspace: Workspace) {
    app.state.workbench.activate(workspace);
}

pub(super) const fn workspace_available(project_open: bool, workspace: Workspace) -> bool {
    project_open || matches!(workspace, Workspace::Project)
}

pub(super) fn active_symbol_editor(app: &RSpiceApp) -> bool {
    matches!(
        app.state.workbench.workspace,
        Workspace::Design | Workspace::Models
    ) && app.state.model.workspace.content.active_view_type()
        == Some(rspice_app_state::state::ViewType::Symbol)
}

pub(in crate::workbench) fn active_schematic_editor(app: &RSpiceApp) -> bool {
    app.state.workbench.workspace == Workspace::Design
        && matches!(
            app.state.model.workspace.content.active_view_type(),
            Some(
                rspice_app_state::state::ViewType::Schematic
                    | rspice_app_state::state::ViewType::Testbench
            )
        )
}

/// Take a history step `direction`: the netlist page's own, which the code
/// feature takes in its context, where that page is in front with a step to
/// take that way; otherwise the design feature's, routed. What either raised
/// is answered as the step returns, as a feature's command has what it
/// raised answered.
// Interim: until code takes a part in Undo and Redo.
pub(super) fn history_command(app: &mut RSpiceApp, direction: HistoryDirection) {
    if netlist_history_steps(&app.state, direction) {
        take_history_step(
            app,
            app_history_parts,
            Some(RSpiceApp::take_netlist_history_step),
            direction,
        );
    } else {
        app.route_command_key(match direction {
            HistoryDirection::Undo => ids::UNDO,
            HistoryDirection::Redo => ids::REDO,
        });
    }
    app.after_contribution();
}

/// Whether the netlist page is in front with a step of its own history to
/// take `direction`: the history the shell steps itself.
pub(super) fn netlist_history_steps(
    state: &crate::workbench::AppState,
    direction: HistoryDirection,
) -> bool {
    netlist_page_is_visible(state)
        && match direction {
            HistoryDirection::Undo => {
                crate::workbench::documents::netlist_document::can_undo_netlist_edit(
                    &state.code.netlist,
                )
            }
            HistoryDirection::Redo => {
                crate::workbench::documents::netlist_document::can_redo_netlist_edit(
                    &state.code.netlist,
                )
            }
        }
}

/// What a history step the shell takes reads and writes of the application.
fn app_history_parts(app: &mut RSpiceApp) -> HistoryStepParts<'_> {
    HistoryStepParts {
        design: app.state.design.history_parts(&app.state.model),
        model: &mut app.state.model,
        wb: &mut app.state.workbench,
        runs: &mut app.state.runs,
        run_controller: &mut app.simulation_controller,
        selected_model: ModelsPort::selected_model(&*app.state.models),
    }
}

pub(in crate::workbench) fn file_action(app: &mut RSpiceApp, action: FileMenuAction) {
    dispatch_file_menu_action(&mut app.state, action, app.file_workflow_io.as_ref());
}

/// Carry out the project feature's file command `action` through the shell's
/// file workflow, as its File menu row does.
pub(in crate::workbench) fn run_project_file_action(
    app: &mut RSpiceApp,
    action: ProjectFileAction,
) {
    file_action(
        app,
        match action {
            ProjectFileAction::NewProject => FileMenuAction::NewProject,
            ProjectFileAction::OpenProject => FileMenuAction::OpenProject,
            ProjectFileAction::OpenSchematic => FileMenuAction::Open,
            ProjectFileAction::Save => FileMenuAction::Save,
            ProjectFileAction::SaveAs => FileMenuAction::SaveProjectAs,
            ProjectFileAction::SaveAll => FileMenuAction::SaveAll,
            ProjectFileAction::CloseProject => FileMenuAction::CloseProject,
        },
    );
}

/// Carry out what the code feature handed back from a request because it
/// reaches past the feature's context, as the shell's table did in place:
/// open or import a netlist or a Verilog-A file through the file workflow,
/// import an LTspice or QSPICE schematic, show the selected instance in the
/// generated deck, open the symbol of the module the Verilog-A view
/// compiles, create the drafted device or the imported cells and open them
/// in Design, open a placed instance of the cell, and the netlist import's
/// transactions over the whole project.
pub(in crate::workbench) fn carry_out_code_hand_off(app: &mut RSpiceApp, hand_off: CodeHandOff) {
    match hand_off {
        CodeHandOff::OpenNetlist => file_action(app, FileMenuAction::OpenNetlist),
        CodeHandOff::ImportNetlist => file_action(app, FileMenuAction::ImportNetlist),
        CodeHandOff::ImportVerilogA => file_action(app, FileMenuAction::ImportVerilogA),
        CodeHandOff::ImportLtspiceSchematic => {
            import_foreign_schematic(app, rspice_design::foreign_schematic::ForeignTool::Ltspice)
        }
        CodeHandOff::ImportQspiceSchematic => {
            import_foreign_schematic(app, rspice_design::foreign_schematic::ForeignTool::Qspice)
        }
        CodeHandOff::ShowInNetlist => app.state.show_selected_instance_in_netlist(),
        CodeHandOff::VerilogAOpenSymbol => {
            activate_workspace(app, Workspace::Design);
            crate::workbench::app::code_adapters::open_veriloga_symbol(&mut app.state);
        }
        CodeHandOff::CreateVerilogADevice => {
            if let Err(error) = app.commit_new_veriloga_device() {
                app.state.code.new_veriloga_device.error = Some(error);
            }
        }
        CodeHandOff::ImportVerilogACells => {
            if let Err(error) = app.commit_veriloga_import()
                && let Some(form) = app.state.code.code_workspace.veriloga.import_form.as_mut()
            {
                form.error = Some(error);
            }
        }
        CodeHandOff::OpenCellInstance(usage) => {
            app.state.open_workspace_view(usage.schematic.clone());
            let position = app
                .state
                .model
                .schematic
                .document()
                .components
                .iter()
                .find(|component| component.id == usage.component_id)
                .map(|component| component.pos);
            let editor = &mut app.state.model.schematic.session.editor;
            editor.selection.select_only_component(usage.component_id);
            editor.net_highlight.clear();
            editor.center_request = position;
        }
        CodeHandOff::CommitNetlistImport => {
            app.state.commit_staged_netlist_import();
        }
        CodeHandOff::BeginNetlistProfileReview => {
            app.state.begin_owned_netlist_profile_review();
        }
        CodeHandOff::ImportDroppedNetlist(dropped) => {
            app.state.import_dropped_netlist(dropped);
        }
    }
}

/// Carry out what the project feature handed over, as the feature did in
/// place: the Configurations command, opening the selected view where it is
/// edited, and what a library catalog change or a created or deleted
/// cell or view left outside the model, each with the revision it reserved,
/// in the order the change once did it.
pub(in crate::workbench) fn answer_project_request(app: &mut RSpiceApp, request: ProjectRequest) {
    match request {
        ProjectRequest::ConfigurationSets => Command::ConfigurationSets.execute(app),
        ProjectRequest::OpenView(reference, view_type) => {
            app.state.open_workspace_view(reference);
            // Source views are cell views too: Design shows a Verilog-A module's
            // editor and states what a SPICE or Verilog view is bound to.
            let workspace = match view_type {
                ViewType::Config => {
                    app.state.project.project_page = ProjectPage::Configuration;
                    Workspace::Project
                }
                _ => Workspace::Design,
            };
            Command::OpenWorkspace(workspace).execute(app);
        }
        ProjectRequest::SaveMigrationCopy => {
            let saved = app.state.save_project_as();
            if saved {
                app.state.record(
                    LogSeverity::Info,
                    LogSource::Studio,
                    "Saved a migration copy that keeps the current technology; no technology was attached.",
                );
            }
        }
        ProjectRequest::OpenRootAndCheck(root) => {
            app.state.open_workspace_view(root);
            Command::OpenWorkspace(Workspace::Design).execute(app);
            let check = DesignCommand::CheckDesign.key();
            if app.command_key_enabled(check) {
                app.execute_command_key(check);
            }
        }
        ProjectRequest::CatalogChanged {
            pruned,
            mutation,
            notice,
        } => {
            apply_workspace_pruned(&mut app.state, pruned);
            if let Some(mutation) = mutation {
                publish_reserved_revision(app, mutation);
            }
            app.state
                .record(LogSeverity::Info, LogSource::Studio, notice);
        }
        ProjectRequest::CellViewsCreated {
            schematics,
            mutation,
            notice,
            open,
            veriloga,
            persist_global_veriloga,
        } => {
            let seeded = app.state.new_schematic_document();
            for reference in schematics {
                app.state
                    .model
                    .workspace
                    .insert_schematic_editor(reference.key(), seeded.clone());
            }
            publish_reserved_revision(app, mutation);
            app.state
                .record(LogSeverity::Info, LogSource::Studio, notice);
            if let Some(view) = open {
                app.state.open_workspace_view(view);
            }
            if let Some((view, receipt)) = veriloga.map(|seeded| *seeded)
                && app.state.model.workspace.content.active_view == view
            {
                app.state.workbench.activate(Workspace::Design);
                app.state.code.code_workspace.veriloga.receipt = Some(receipt);
            }
            if persist_global_veriloga {
                persist_global_veriloga_library(&mut app.state.model);
            }
        }
        ProjectRequest::ViewOpened {
            mutation,
            view,
            veriloga,
            notice,
            persist_global_veriloga,
        } => {
            publish_reserved_revision(app, mutation);
            app.state.open_workspace_view(view);
            if let Some(receipt) = veriloga {
                app.state.workbench.activate(Workspace::Design);
                app.state.code.code_workspace.veriloga.receipt = Some(*receipt);
            }
            app.state
                .record(LogSeverity::Info, LogSource::Studio, notice);
            if persist_global_veriloga {
                persist_global_veriloga_library(&mut app.state.model);
            }
        }
        ProjectRequest::CellOrViewDeleted {
            removed_sources,
            pruned,
            mutation,
            remove_placements,
            notice,
            persist_global_veriloga,
        } => {
            void_veriloga_evidence(&mut app.state, &removed_sources);
            apply_workspace_pruned(&mut app.state, pruned);
            publish_reserved_revision(app, mutation);
            if let Some(target) = remove_placements {
                app.state.model.remove_deleted_master_placements(
                    target.library(),
                    target.cell(),
                    target.view(),
                    &target.display_path(),
                );
            }
            app.state
                .record(LogSeverity::Info, LogSource::Studio, notice);
            if persist_global_veriloga {
                persist_global_veriloga_library(&mut app.state.model);
            }
        }
        ProjectRequest::CreateProject => {
            let params = app.state.project.new_project.params();
            match app.state.create_new_project_with(&params) {
                Ok(()) => app.state.project.new_project = NewProjectDialogState::default(),
                Err(error) => app.state.project.new_project.error = Some(error),
            }
        }
        ProjectRequest::OpenCreateHierarchy => {
            crate::workbench::app::open_create_hierarchy_dialog(
                &mut app.state.design.create_hierarchy,
                &app.state.model,
                &mut app.state.workbench,
            );
        }
        ProjectRequest::CommitCheckAndSave => app.commit_check_and_save(),
        ProjectRequest::GoOnLeaving { action, recent } => {
            if let Some((action, path, kind)) =
                go_on_leaving(&mut app.state, Leaving { action, recent })
            {
                app.execute_pending_action(action, path, kind, None);
            }
        }
    }
}

/// Publish the revision a catalog change reserved, once what the change
/// left outside the model has been carried out.
fn publish_reserved_revision(app: &mut RSpiceApp, mutation: ReservedLibraryRevision) {
    let mutation = mutation.take();
    debug_assert!(
        mutation.is_some(),
        "a reserved revision is published once, by the answer"
    );
    if let Some(mutation) = mutation {
        app.state.model.publish_project_library_mutation(mutation);
    }
}

/// Imports an LTspice or QSPICE schematic, asking for the file.
pub(in crate::workbench) fn import_foreign_schematic(
    app: &mut RSpiceApp,
    tool: rspice_design::foreign_schematic::ForeignTool,
) {
    #[cfg(not(target_arch = "wasm32"))]
    crate::workbench::workflows::foreign_schematic_workflow::import_foreign_schematic_with_dialog(
        app, tool,
    );
    #[cfg(target_arch = "wasm32")]
    {
        let _ = tool;
        if let Some(refusal) =
            rspice_app_services::workbench::slots::platform::foreign_import_refusal_in(
                &app.state.model,
                &app.state.platform,
            )
        {
            app.state
                .prompt(format!("The schematic cannot be imported: {refusal}."));
        }
    }
}
