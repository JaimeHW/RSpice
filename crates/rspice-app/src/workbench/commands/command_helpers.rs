//! Command-dispatch helpers that apply concrete workbench state transitions.

use super::*;
use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::ViewType;
use crate::workbench::app::dialogs::project::configuration_dialogs::{Leaving, go_on_leaving};
use crate::workbench::app::dialogs::project::new_project::NewProjectDialogState;
pub(super) use crate::workbench::app::dialogs::selection::commands::schematic_selection_has_live_object;
use crate::workbench::app::dialogs::source_document;
use crate::workbench::code::commands::{CodeCommand, CodeRequest, SourceDocumentRequest};
use crate::workbench::documents::code_workspace::persist_global_veriloga_library;
use crate::workbench::framework::requests::ProjectFileAction;
use crate::workbench::project::commands::{ProjectRequest, ReservedLibraryRevision};
use crate::workbench::project::workspace_state::ProjectPage;
use crate::workbench::session::project_transactions::{
    apply_workspace_pruned, void_veriloga_evidence,
};

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
/// Clear the drawing's selection: the symbol's in a symbol editor, the
/// active sheet's otherwise, in the session and in the sheet's buffer, as
/// Select all writes both.
pub(super) fn deselect_all(app: &mut RSpiceApp) {
    if active_symbol_editor(app) {
        app.state.design.symbol.editor.clear_selection();
        return;
    }
    let state = &mut app.state;
    state.model.schematic.session.editor.selection.clear();
    let active_key = state
        .model
        .workspace
        .content
        .active_schematic_reference()
        .key();
    if let Some(mut buffer) = state.model.workspace.schematic_editor_mut(&active_key) {
        buffer.editor.session.editor.selection.clear();
    }
}

/// The label a menu row shows for `command` right now: Undo and Redo name
/// the step they take where the drawing's history names it, Repeat names
/// the command it repeats, and every other row reads the vocabulary's label.
pub(crate) fn menu_label(command: Command, app: &RSpiceApp) -> std::borrow::Cow<'static, str> {
    let label = command.spec().label;
    let named = match command {
        Command::Undo | Command::Redo if command.is_enabled(app) => {
            schematic_step_name(app, command == Command::Undo)
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
/// is the schematic editor's own: its history names each step. The netlist
/// page's and the project's histories, and the symbol editor's, do not.
fn schematic_step_name(app: &RSpiceApp, undo: bool) -> Option<&str> {
    let state = &app.state;
    let elsewhere = if undo {
        (netlist_page_is_visible(state)
            && crate::workbench::documents::netlist_document::can_undo_netlist_edit(state))
            || state.model.can_undo_project_design()
    } else {
        (netlist_page_is_visible(state)
            && crate::workbench::documents::netlist_document::can_redo_netlist_edit(state))
            || state.model.can_redo_project_design()
    };
    if elsewhere
        || state.workbench.active_source_editor.is_some()
        || active_symbol_editor(app)
        || !active_schematic_editor(app)
    {
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
    ) && app.state.model.workspace.content.active_view_type() == crate::state::ViewType::Symbol
}

/// Mockup-owned `G` command: cycle only the canvas presentation. Snap pitch
/// and target classes remain independent in Grid, snap and wire routing.
pub(super) fn cycle_canvas_grid(app: &mut RSpiceApp) {
    if active_symbol_editor(app) {
        let enabled = !app.state.design.symbol.editor.show_grid;
        app.state.design.symbol.editor.show_grid = enabled;
        return;
    }

    app.state.design.cycle_grid_style();
}

pub(in crate::workbench) fn active_schematic_editor(app: &RSpiceApp) -> bool {
    app.state.workbench.workspace == Workspace::Design
        && matches!(
            app.state.model.workspace.content.active_view_type(),
            crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
        )
}

/// Return whether Delete can resolve at least one selected identity to a live
/// complete object. Segment and vertex handles are edit subobjects rather than
/// independently persisted conductors, so the destructive review promotes a
/// live handle to its owning wire before commit.
pub(super) fn schematic_selection_has_deletable_object(
    schematic: &crate::state::SchematicState,
) -> bool {
    schematic_selection_has_live_object(schematic)
        || schematic.document().wires.iter().any(|wire| {
            schematic
                .session
                .editor
                .selection
                .wire_segments
                .iter()
                .any(|selected| {
                    selected.wire_id == wire.id && selected.segment_index < wire.segment_count()
                })
                || schematic
                    .session
                    .editor
                    .selection
                    .wire_vertices
                    .iter()
                    .any(|selected| {
                        selected.wire_id == wire.id && selected.vertex_index < wire.vertex_count()
                    })
        })
}

pub(super) fn schematic_selection_has_duplicable_object(
    schematic: &crate::state::SchematicState,
) -> bool {
    let selection = &schematic.session.editor.selection;
    selection.wire_segments.is_empty()
        && selection.wire_vertices.is_empty()
        && (schematic
            .document()
            .components
            .iter()
            .any(|component| selection.has_component(component.id))
            || schematic
                .document()
                .wires
                .iter()
                .any(|wire| selection.has_wire(wire.id))
            || schematic
                .document()
                .net_labels
                .iter()
                .any(|label| selection.has_net_label(label.id))
            || schematic
                .document()
                .buses
                .iter()
                .any(|bus| selection.has_bus(bus.id))
            || schematic
                .document()
                .bus_taps
                .iter()
                .any(|tap| selection.has_bus_tap(tap.id))
            || schematic
                .document()
                .design_notes
                .iter()
                .any(|note| selection.has_design_note(note.id))
            || schematic
                .document()
                .documentation_shapes
                .iter()
                .any(|shape| selection.has_documentation_shape(shape.id))
            || schematic
                .document()
                .probes
                .iter()
                .any(|probe| selection.has_probe(probe.id)))
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

/// Carry out what the code feature handed over, as the shell's table did in
/// place: open or import a netlist or a Verilog-A file through the file
/// workflow, import an LTspice or QSPICE schematic, export the deck from
/// outside the netlist page, export the bill of materials, open the
/// generated deck, show the selected instance in it, and
/// open the Code workspace's find, validation and document tools,
/// language navigation from the caret, the Verilog-A compile, its build
/// settings and the symbol of the module it compiles.
// Interim: each answer is the body the shell's table ran for the command,
// until the code feature carries it out over its own parts.
pub(in crate::workbench) fn answer_code_request(app: &mut RSpiceApp, request: CodeRequest) {
    match request {
        CodeRequest::OpenNetlist => file_action(app, FileMenuAction::OpenNetlist),
        CodeRequest::ImportNetlist => file_action(app, FileMenuAction::ImportNetlist),
        CodeRequest::ImportVerilogA => file_action(app, FileMenuAction::ImportVerilogA),
        CodeRequest::ImportLtspiceSchematic => {
            import_foreign_schematic(app, rspice_design::foreign_schematic::ForeignTool::Ltspice)
        }
        CodeRequest::ImportQspiceSchematic => {
            import_foreign_schematic(app, rspice_design::foreign_schematic::ForeignTool::Qspice)
        }
        CodeRequest::ExportNetlist(format) => {
            crate::workbench::menu_bar::action_export_netlist_with_io(
                &mut app.state,
                format,
                app.export_workflow_io.as_ref(),
            );
        }
        CodeRequest::ExportBillOfMaterials => {
            crate::workbench::menu_bar::action_export_bill_of_materials_with_io(
                &mut app.state,
                app.export_workflow_io.as_ref(),
            );
        }
        CodeRequest::GenerateNetlist => {
            crate::workbench::menu_bar::action_view_netlist(&mut app.state);
            activate_workspace(app, Workspace::Netlist);
        }
        CodeRequest::ShowInNetlist => app.state.show_selected_instance_in_netlist(),
        CodeRequest::FindCodeDocument => {
            if let Err(error) =
                crate::workbench::documents::code_workspace::open_active_source_search(app)
            {
                app.state.prompt(error);
            }
        }
        CodeRequest::ValidateCodeDocument => {
            crate::workbench::workflows::netlist_workflow::validate_visible_netlist_source(app);
        }
        CodeRequest::ManageSourceDocument => {
            if let Err(error) = crate::workbench::app::open_source_document_dialog(&mut app.state) {
                app.state.prompt(error);
            }
        }
        CodeRequest::GoToDefinition => navigate_active_source(app, CodeCommand::GoToDefinition),
        CodeRequest::FindReferences => navigate_active_source(app, CodeCommand::FindReferences),
        CodeRequest::RenameSymbol => navigate_active_source(app, CodeCommand::RenameSymbol),
        CodeRequest::CompileVerilogA => {
            // The command compiles the active Verilog-A cell view and
            // takes the reader to where its result is shown: the view is
            // edited in Design, so that is where a compile of it lands,
            // whichever workspace offered the command.
            activate_workspace(app, Workspace::Design);
            // The closure is taken now, at the click; the frame loop
            // starts the compile on its next pass. Nothing is asked.
            if let Err(error) =
                crate::workbench::documents::code_workspace::request_veriloga_compile(app)
            {
                app.state.prompt(error);
            }
        }
        CodeRequest::VerilogABuildSettings => {
            activate_workspace(app, Workspace::Design);
            if let Err(error) = crate::workbench::app::open_veriloga_build_settings(app) {
                app.state.prompt(error);
            }
        }
        CodeRequest::VerilogAOpenSymbol => {
            activate_workspace(app, Workspace::Design);
            crate::workbench::app::actions::veriloga_symbol::open_veriloga_symbol(&mut app.state);
        }
        CodeRequest::CreateVerilogADevice => {
            if let Err(error) = app.commit_new_veriloga_device() {
                app.state.code.new_veriloga_device.error = Some(error);
            }
        }
        CodeRequest::ImportVerilogACells => {
            if let Err(error) = app.commit_veriloga_import()
                && let Some(form) = app.state.code.code_workspace.veriloga.import_form.as_mut()
            {
                form.error = Some(error);
            }
        }
        CodeRequest::SaveVerilogABuildSettings { compile } => {
            match crate::workbench::app::dialogs::veriloga_build_settings::save_veriloga_build_settings(
                app,
            ) {
                Ok(_) if compile => {
                    if let Err(error) =
                        crate::workbench::documents::code_workspace::request_veriloga_compile(app)
                    {
                        app.state.prompt(error);
                    }
                }
                Ok(_) => {}
                Err(error) => {
                    if let Some(draft) = app.state.code.veriloga_build_settings.as_mut() {
                        draft.error = Some(error);
                    }
                }
            }
        }
        CodeRequest::ReplaceVerilogASource => {
            if let Err(error) =
                crate::workbench::documents::code_workspace::commit_veriloga_source_replacement(app)
            {
                app.state.record(
                    crate::diagnostics::LogSeverity::Error,
                    crate::diagnostics::LogSource::Studio,
                    error,
                );
            }
        }
        CodeRequest::ReplaceSourceSearchMatches => {
            if let Err(error) =
                crate::workbench::documents::code_workspace::commit_source_search_replace(app)
            {
                if let Some(search) = app.state.code.code_workspace.source_search.as_mut() {
                    search.error = Some(error);
                } else {
                    app.state.prompt(error);
                }
            }
        }
        CodeRequest::SourceDocument(request) => answer_source_document_request(app, request),
        CodeRequest::CommitSymbolRename => {
            if let Err(error) =
                crate::workbench::documents::code_workspace::commit_language_rename(app)
                && let Some(open) = app.state.code.code_workspace.language_tools.as_mut()
            {
                open.error = Some(error);
            }
        }
        CodeRequest::UseLibSection {
            dependency,
            original,
            replacement,
            selected,
        } => {
            use crate::workbench::documents::netlist_document::{
                OwnedNetlistReplacement, replace_owned_sources_atomically,
            };
            let edit = match dependency.as_deref() {
                Some(identity) => {
                    OwnedNetlistReplacement::dependency(identity, &original, replacement, 1)
                }
                None => OwnedNetlistReplacement::root(&original, replacement, 1),
            };
            match replace_owned_sources_atomically(&mut app.state, vec![edit]) {
                Ok(_) => app.state.prompt(selected),
                Err(error) => app.state.prompt(error),
            }
        }
        CodeRequest::RelinkNetlistDependency(identity) => {
            crate::workbench::workflows::netlist_workflow::request_dependency_relink(
                &mut app.state,
                &identity,
            );
        }
        CodeRequest::SourceFileAction {
            language,
            bundle_id,
            action,
            path,
        } => {
            let opened =
                crate::workbench::documents::code_workspace::open_source_file_dialog_in_bundle(
                    app, language, bundle_id, action, &path, &path,
                )
                .and_then(|()| crate::workbench::app::open_source_document_dialog(&mut app.state));
            if let Err(error) = opened {
                app.state.code.code_workspace.source_file_dialog = None;
                app.state.prompt(error);
            }
        }
        CodeRequest::OpenCellInstance(usage) => {
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
        // Interim: each of the netlist page's transactions commits through a
        // candidate copy of the whole state, so the shell runs the body the
        // dialog ran in place over what the dialog stored.
        CodeRequest::CommitNetlistRename => {
            let messages = app.state.workbench.messages();
            match crate::workbench::documents::netlist_document::language::commit_rename(
                &mut app.state,
            ) {
                Ok(count) => app.state.prompt(messages.format(
                    crate::workbench::MessageId::NetlistRenameSucceeded,
                    &[("count", &count.to_string())],
                )),
                Err(error) => app.state.code.netlist.rename_dialog.error = Some(error),
            }
        }
        CodeRequest::RestoreOwnedNetlistRevision => {
            let index = app.state.code.netlist.comparison_dialog.selected_history_index;
            match crate::workbench::documents::netlist_document::restore_owned_revision(
                &mut app.state,
                index,
            ) {
                Ok(()) => app.state.code.netlist.comparison_dialog.open = false,
                Err(error) => app.state.prompt(error),
            }
        }
        CodeRequest::ApplyExternalNetlistChange => {
            match crate::workbench::workflows::netlist_workflow::apply_staged_external_netlist_change(
                &mut app.state,
            ) {
                Ok(()) => {
                    app.state.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        "Applied the resolved external change to the SPICE source.",
                    );
                }
                Err(error) => {
                    if let Some(current) = app.state.code.netlist.external_change.as_mut() {
                        current.error = Some(error);
                    }
                }
            }
        }
        CodeRequest::CommitNetlistImport => {
            crate::workbench::workflows::netlist_workflow::commit_staged_netlist_import(
                &mut app.state,
            );
        }
        CodeRequest::ApplyNetlistFind => {
            if let Some(step) = app.state.code.netlist.find.pending.take() {
                crate::workbench::workflows::netlist_workflow::apply_netlist_find_step(
                    &mut app.state,
                    step,
                );
            }
        }
        CodeRequest::CopyNetlistDependency => {
            let messages = app.state.workbench.messages();
            match crate::workbench::documents::netlist_document::copy_active_dependency_to_project(
                &mut app.state,
            ) {
                Ok(_) => {
                    app.state.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        messages.text(crate::workbench::MessageId::NetlistCopySucceeded),
                    );
                }
                Err(error) => {
                    app.state
                        .record(LogSeverity::Error, LogSource::Studio, error);
                }
            }
        }
        CodeRequest::ApplyNetlistQuickFix => {
            let answer =
                crate::workbench::documents::netlist_document::language::apply_preferred_quick_fix(
                    &mut app.state,
                );
            app.state.prompt(answer.unwrap_or_else(|refusal| refusal));
        }
        CodeRequest::BeginNetlistProfileReview => {
            crate::workbench::workflows::netlist_workflow::begin_owned_netlist_profile_review(
                &mut app.state,
            );
        }
        CodeRequest::FormatNetlistDocument => {
            crate::workbench::workflows::netlist_workflow::format_owned_netlist(app);
        }
        CodeRequest::ImportDroppedNetlist(dropped) => {
            crate::workbench::workflows::netlist_workflow::import_dropped_netlist(
                &mut app.state,
                dropped,
            );
        }
    }
}

/// Carry out what a source document dialog handed over: create the
/// Automation workspace its draft describes or the top deck its creation
/// form drafted, or a lifecycle choice on the document it manages, then the
/// close the same frame asked for.
// Interim: each answer is the body the dialog ran in place, until the code
// feature carries it out over its own parts.
fn answer_source_document_request(app: &mut RSpiceApp, request: SourceDocumentRequest) {
    match request {
        SourceDocumentRequest::CreateWorkspace => {
            match crate::workbench::documents::code_workspace::commit_source_workspace_dialog(app) {
                Ok(message) => {
                    app.state
                        .record(LogSeverity::Info, LogSource::Studio, message);
                }
                Err(error) => {
                    if let Some(state) = app
                        .state
                        .code
                        .code_workspace
                        .source_workspace_dialog
                        .as_mut()
                    {
                        state.error = Some(error);
                    }
                }
            }
        }
        SourceDocumentRequest::Document {
            action,
            language,
            logical_path,
            close,
        } => {
            app.apply_lifecycle_action(action, language, &logical_path);
            if close {
                source_document::close_document_lifecycle(&mut app.state.code.code_workspace);
            }
        }
        SourceDocumentRequest::Netlist { action, close } => {
            source_document::apply_netlist_lifecycle_action(&mut app.state, action, close);
        }
        SourceDocumentRequest::CreateTopDeck => {
            source_document::commit_new_top_deck(&mut app.state);
        }
    }
}

/// Go to definition, Find references or Rename symbol from the caret of the
/// source editor on screen, saying why not when it cannot.
fn navigate_active_source(app: &mut RSpiceApp, command: CodeCommand) {
    if let Err(error) = crate::workbench::app::navigate_active_source(app, command) {
        app.state.prompt(error);
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
            let saved =
                crate::workbench::workflows::project_workflow::save_project_as(&mut app.state);
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
            if Command::CheckDesign.is_enabled(app) {
                Command::CheckDesign.execute(app);
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

/// Why the LTspice or QSPICE schematic import cannot run now, if it cannot.
pub(in crate::workbench) fn foreign_import_refusal(
    model: &crate::workbench::app_state::Model,
    platform: &crate::workbench::app_state::PlatformState,
) -> Option<&'static str> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        crate::workbench::workflows::foreign_schematic_workflow::foreign_import_refusal_in(
            model, platform,
        )
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (model, platform);
        Some("the source program's symbol library is read from this computer's disk")
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
        if let Some(refusal) = foreign_import_refusal(&app.state.model, &app.state.platform) {
            app.state
                .prompt(format!("The schematic cannot be imported: {refusal}."));
        }
    }
}
