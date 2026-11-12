//! Typed workbench commands and their single dispatch boundary.
//!
//! The workbench never paints a visible action without routing it here.  A
//! command is omitted from a menu when its behavior is not implemented; the
//! UI does not advertise speculative or placeholder capability.

use crate::schematic::view::schematic_symbol_context;
use crate::schematic::view::selection_transform::{
    SelectionTransform, transform_selection_on_active_sheet,
};
use crate::workbench::code::commands::CodeCommand;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::framework::command_registry::{CommandOwner, ids};
use crate::workbench::menu_bar::{
    FileMenuAction, check_cellview, check_design, dispatch_file_menu_action,
};
use crate::workbench::state::pane_focus::PaneStep;
use crate::workbench::{AppState, RSpiceApp};
use rspice_schematic_editor::symbol_editor::commands::SymbolTransform;

use super::state::{ConsoleChannel, ConsolePage, Workspace};

pub(crate) mod code_context;
mod enablement;
mod registry;
pub(crate) use rspice_app_results::workbench::commands::result_navigation;
pub(crate) mod vocabulary;

/// A test seam on the dispatch boundary. While a recording is open,
/// [`Command::execute`] notes the command it was handed and returns before
/// acting, so a test can prove a route reaches the boundary without carrying
/// the command out.
#[cfg(test)]
pub(crate) mod dispatch_probe {
    use super::Command;
    use std::cell::RefCell;

    thread_local! {
        static REACHED: RefCell<Option<Vec<Command>>> = const { RefCell::new(None) };
    }

    /// Run `route` and return every command it handed to `Command::execute`.
    pub(crate) fn reached(route: impl FnOnce()) -> Vec<Command> {
        REACHED.with(|reached| *reached.borrow_mut() = Some(Vec::new()));
        route();
        REACHED.with(|reached| reached.borrow_mut().take().unwrap_or_default())
    }

    pub(in crate::workbench) fn intercept(command: Command) -> bool {
        REACHED.with(|reached| {
            reached
                .borrow_mut()
                .as_mut()
                .map(|log| log.push(command))
                .is_some()
        })
    }
}

fn selection_layout_command(
    command: Command,
) -> Option<crate::schematic::view::selection_layout::SelectionLayoutCommand> {
    use crate::schematic::view::selection_layout::SelectionLayoutCommand as Layout;
    Some(match command {
        Command::AlignSelectionLeft => Layout::AlignLeft,
        Command::AlignSelectionCenter => Layout::AlignCenter,
        Command::AlignSelectionRight => Layout::AlignRight,
        Command::AlignSelectionTop => Layout::AlignTop,
        Command::AlignSelectionMiddle => Layout::AlignMiddle,
        Command::AlignSelectionBottom => Layout::AlignBottom,
        Command::DistributeSelectionHorizontal => Layout::DistributeHorizontal,
        Command::DistributeSelectionVertical => Layout::DistributeVertical,
        _ => return None,
    })
}

/// Commands whose design-workspace execution can mutate the active schematic
/// or a project-owned schematic contract. Presentation and inspection
/// commands deliberately remain available in read-only contexts.
pub(crate) const fn command_edits_schematic(command: Command) -> bool {
    matches!(
        command,
        Command::Undo
            | Command::Redo
            | Command::Cut
            | Command::Paste
            | Command::Duplicate
            | Command::Delete
            | Command::FindReplace
            | Command::MoveSelection
            | Command::StretchSelection
            | Command::ArraySelection
            | Command::SwapToRSpiceSymbols
            | Command::UpdateInstanceInterface
            | Command::RotateSelection
            | Command::RotateSelectionCounterClockwise
            | Command::MirrorSelectionHorizontal
            | Command::MirrorSelectionVertical
            | Command::AlignSelectionLeft
            | Command::AlignSelectionCenter
            | Command::AlignSelectionRight
            | Command::AlignSelectionTop
            | Command::AlignSelectionMiddle
            | Command::AlignSelectionBottom
            | Command::DistributeSelectionHorizontal
            | Command::DistributeSelectionVertical
            | Command::SnapSelectionToGrid
            | Command::SnapToStandardValue
            | Command::JoinCollinearWires
    )
}

impl Command {
    /// What this command asks of a source editor when one is the document on
    /// screen: the same thing its shortcut does with the editor focused, never
    /// an edit of the schematic hidden behind it.
    pub(crate) const fn source_editor_request(
        self,
    ) -> Option<crate::workbench::framework::requests::SourceEditorRequest> {
        use crate::workbench::framework::requests::{SourceEditorRequest, StandardCommand};
        Some(match self {
            Self::Undo => SourceEditorRequest::Standard(StandardCommand::Undo),
            Self::Redo => SourceEditorRequest::Standard(StandardCommand::Redo),
            Self::Cut => SourceEditorRequest::Standard(StandardCommand::Cut),
            Self::Copy => SourceEditorRequest::Standard(StandardCommand::Copy),
            Self::Paste => SourceEditorRequest::Standard(StandardCommand::Paste),
            Self::Delete => SourceEditorRequest::Standard(StandardCommand::Delete),
            Self::SelectAll => SourceEditorRequest::Standard(StandardCommand::SelectAll),
            _ => return None,
        })
    }

    /// The placement and editing commands Repeat last command runs again,
    /// each with what it carries (a rotation's sense). An
    /// undo, a delete or a form is not repeated: Repeat on those would be a
    /// surprise rather than a shortcut.
    pub(crate) const fn repeatable(self) -> bool {
        matches!(
            self,
            Self::Duplicate
                | Self::Paste
                | Self::MoveSelection
                | Self::StretchSelection
                | Self::RotateSelection
                | Self::RotateSelectionCounterClockwise
                | Self::MirrorSelectionHorizontal
                | Self::MirrorSelectionVertical
                | Self::AlignSelectionLeft
                | Self::AlignSelectionCenter
                | Self::AlignSelectionRight
                | Self::AlignSelectionTop
                | Self::AlignSelectionMiddle
                | Self::AlignSelectionBottom
                | Self::DistributeSelectionHorizontal
                | Self::DistributeSelectionVertical
                | Self::SnapSelectionToGrid
                | Self::SnapToStandardValue
                | Self::JoinCollinearWires
        )
    }

    /// Shared by menu/palette dispatch and the keyboard dispatcher.
    pub(crate) fn commits_inspector_edit(self) -> bool {
        command_edits_schematic(self)
            || matches!(
                self,
                Self::Copy
                    | Self::OpenWorkspace(_)
                    | Self::PreviousDocument
                    | Self::NextDocument
                    | Self::CloseOtherDocuments
                    | Self::CloseAllDocuments
                    | Self::AscendHierarchy
                    | Self::DescendHierarchy
                    | Self::DescendHierarchyDirect
                    | Self::CheckCellview
                    | Self::CheckDesign
                    | Self::ToggleInspector
                    | Self::ToggleFocusMode
                    | Self::ResetLayout
            )
    }

    /// The part of the product that carries the command out: the slice whose
    /// code its dispatch reaches. Commands that act on whichever document is
    /// in front (edit, find and zoom) belong to the workbench.
    const fn owner(self) -> CommandOwner {
        match self {
            Self::OpenWorkspace(_)
            | Self::Exit
            | Self::RepeatLastCommand
            | Self::KeyboardShortcuts
            | Self::HelpCenter
            | Self::Examples
            | Self::ReleaseNotes
            | Self::SystemDiagnostics
            | Self::SupportBundle
            | Self::LegalPrivacy
            | Self::About => CommandOwner::Shell,
            Self::Undo
            | Self::Redo
            | Self::Cut
            | Self::Copy
            | Self::Paste
            | Self::Duplicate
            | Self::Delete
            | Self::SelectAll
            | Self::DeselectAll
            | Self::FindInDesign
            | Self::ZoomIn
            | Self::ZoomOut
            | Self::ZoomFit
            | Self::ZoomOneToOne
            | Self::ZoomToSelection
            | Self::ZoomBox
            | Self::ZoomPrevious
            | Self::ToggleFullScreen
            | Self::RevertActiveDocument
            | Self::CloseActiveDocument
            | Self::ToggleNavigator
            | Self::ToggleInspector
            | Self::ToggleConsole
            | Self::OpenConsole
            | Self::OpenProblems
            | Self::OpenNotifications
            | Self::ToggleConsoleMaximized
            | Self::ClearConsole
            | Self::ToggleFocusMode
            | Self::ResetLayout
            | Self::NewApplicationWindow
            | Self::DetachDocument
            | Self::MoveDocumentToWindow
            | Self::ReattachDocument
            | Self::ConsolidateWindows
            | Self::MonitorRecovery
            | Self::PreviousDocument
            | Self::NextDocument
            | Self::NextPane
            | Self::PreviousPane
            | Self::CloseOtherDocuments
            | Self::CloseAllDocuments
            | Self::WindowManager
            | Self::MinimizeWindow
            | Self::ZoomWindow
            | Self::CommandPalette => CommandOwner::Framework,
            Self::ConfigurationSets => CommandOwner::Project,
            Self::SelectOfKind(_)
            | Self::MoveSelection
            | Self::StretchSelection
            | Self::ArraySelection
            | Self::SwapToRSpiceSymbols
            | Self::UpdateInstanceInterface
            | Self::HighlightNet
            | Self::SymbolPinTool
            | Self::SymbolPolylineTool
            | Self::SymbolRectangleTool
            | Self::SymbolCircleTool
            | Self::SymbolArcTool
            | Self::SymbolPolygonTool
            | Self::SymbolTextTool
            | Self::SymbolRotatePin
            | Self::SymbolMirrorPin
            | Self::SymbolUpdatePinsFromContract
            | Self::SymbolSave
            | Self::EditSymbolInPlace
            | Self::ExitSymbolInPlace
            | Self::RotateSelection
            | Self::RotateSelectionCounterClockwise
            | Self::MirrorSelectionHorizontal
            | Self::MirrorSelectionVertical
            | Self::AlignSelectionLeft
            | Self::AlignSelectionCenter
            | Self::AlignSelectionRight
            | Self::AlignSelectionTop
            | Self::AlignSelectionMiddle
            | Self::AlignSelectionBottom
            | Self::DistributeSelectionHorizontal
            | Self::DistributeSelectionVertical
            | Self::SnapSelectionToGrid
            | Self::SnapToStandardValue
            | Self::SetStandardValueSeries(_)
            | Self::JoinCollinearWires
            | Self::Cancel
            | Self::AscendHierarchy
            | Self::DescendHierarchy
            | Self::DescendHierarchyDirect
            | Self::CheckCellview
            | Self::CheckDesign
            | Self::ClearChecks
            | Self::NextViolation
            | Self::PreviousViolation
            | Self::ObjectProperties
            | Self::FindReplace
            | Self::AnnotateOperatingPoint
            | Self::AnnotateTransientVoltages => CommandOwner::Design,
        }
    }

    /// Run this command through the shell's dispatcher, as every route does.
    pub fn execute(self, app: &mut RSpiceApp) {
        self.dispatch(app);
    }

    /// The shell's legacy table: what each command does once the
    /// dispatcher's prologue has let it through. [`Command::dispatch`] is
    /// its one caller, so no command skips the prologue.
    pub(super) fn run_legacy_arm(self, app: &mut RSpiceApp) {
        match self {
            Self::OpenWorkspace(workspace) => {
                if workspace_available(app.state.platform.project_lifecycle.is_open(), workspace) {
                    activate_workspace(app, workspace);
                }
            }
            Self::RevertActiveDocument => file_action(app, FileMenuAction::RevertActiveDocument),
            Self::CloseActiveDocument => file_action(app, FileMenuAction::CloseActiveDocument),
            Self::Exit => file_action(app, FileMenuAction::Exit),
            Self::Undo => app.action_edit_undo(),
            Self::Redo => app.action_edit_redo(),
            // An edit command acts on the selection in the editor the reader
            // is working in, and the part that takes it there carries it out:
            // Results copies what it has selected where it is in front, the
            // design feature edits in a schematic or symbol editor. Anywhere
            // else there is nothing to edit, so nothing happens.
            Self::Cut
            | Self::Copy
            | Self::Paste
            | Self::Duplicate
            | Self::Delete
            | Self::SelectAll => {
                app.route_command_key(self.key());
            }
            Self::DeselectAll => deselect_all(app),
            Self::SelectOfKind(kind) => {
                crate::workbench::app::dialogs::selection::commands::select_schematic_objects_of_kind(
                    &mut app.state,
                    kind,
                );
            }
            Self::ObjectProperties => {
                if active_symbol_editor(app) {
                    app.state.workbench.inspector_visible = true;
                    app.state.workbench.drawer = Some(super::state::Drawer::Inspector);
                } else if crate::workbench::app::shared_properties_available(&app.state) {
                    // Several instances: one form for what they share.
                    crate::workbench::app::open_shared_properties(&mut app.state);
                } else {
                    crate::workbench::app::open_selected_object_properties(&mut app.state);
                }
            }
            // The netlist page's find is code's. Elsewhere the design feature
            // opens the Navigator's search, except in a Verilog-A cell view,
            // which has no instances or nets to find: Find searches its
            // source bundle instead.
            Self::FindInDesign => {
                if netlist_page_is_visible(&app.state) {
                    app.state.code.netlist.find.open = true;
                } else if !app.route_command_key(ids::FIND_IN_DESIGN) {
                    app.execute_command_key(CodeCommand::FindCodeDocument.key());
                }
            }
            // A view command moves the view the reader is working in, and the
            // part that takes it there carries it out: Results where it is in
            // front, the design feature in a schematic or symbol editor.
            // Anywhere else there is no view to move, so nothing happens.
            Self::ZoomIn
            | Self::ZoomOut
            | Self::ZoomFit
            | Self::ZoomOneToOne
            | Self::ZoomToSelection
            | Self::ZoomBox
            | Self::ZoomPrevious => {
                app.route_command_key(self.key());
            }
            // Off leaves the violations on the drawing: hiding them as well
            // is the visibility dialog's choice.
            Self::AnnotateOperatingPoint => {
                use crate::state::SchematicAnnotationVisibility;
                let annotations = &mut app.state.design.schematic_visibility.annotations;
                *annotations = if *annotations == SchematicAnnotationVisibility::OperatingPoint {
                    SchematicAnnotationVisibility::ViolationsOnly
                } else {
                    SchematicAnnotationVisibility::OperatingPoint
                };
            }
            // On, the status bar says which instant the badges read.
            Self::AnnotateTransientVoltages => {
                use crate::state::SchematicAnnotationVisibility;
                let annotations = &mut app.state.design.schematic_visibility.annotations;
                if *annotations == SchematicAnnotationVisibility::TransientVoltages {
                    *annotations = SchematicAnnotationVisibility::ViolationsOnly;
                } else {
                    *annotations = SchematicAnnotationVisibility::TransientVoltages;
                    let status = crate::schematic::view::transient_annotation_status(&app.state);
                    app.state.workbench.prompt(status);
                }
            }
            Self::HighlightNet => {
                crate::schematic::view::highlight_selected_net(&mut app.state);
            }
            Self::ToggleFullScreen => {
                if app.state.workbench.full_screen_presentation {
                    app.state.exit_full_screen_presentation();
                } else {
                    app.state.enter_full_screen_presentation();
                }
            }
            Self::ToggleNavigator => {
                if app.state.workbench.navigator_visible {
                    app.state.workbench.dismiss_navigator();
                } else {
                    app.state.workbench.navigator_visible = true;
                }
            }
            Self::ToggleInspector => {
                if app.state.workbench.inspector_visible {
                    app.state.workbench.dismiss_inspector();
                } else {
                    app.state.workbench.inspector_visible = true;
                }
            }
            Self::ToggleConsole => {
                if app.state.workbench.focus_mode {
                    app.state.workbench.focus_mode = false;
                    app.state.workbench.console_visible = true;
                } else {
                    app.state.workbench.console_visible = !app.state.workbench.console_visible;
                }
                if !app.state.workbench.console_visible {
                    app.state.workbench.console_maximized = false;
                }
            }
            Self::OpenProblems => {
                app.state.workbench.focus_mode = false;
                app.state.workbench.console_visible = true;
                app.state.workbench.console_maximized = false;
                app.state.workbench.console_page = super::state::ConsolePage::Problems;
            }
            Self::OpenConsole => app.state.workbench.open_console(),
            Self::OpenNotifications => app.state.workbench.notification_center_open = true,
            Self::ToggleConsoleMaximized => {
                app.state.workbench.focus_mode = false;
                app.state.workbench.console_maximized = !app.state.workbench.console_maximized;
                app.state.workbench.console_visible = true;
            }
            // The console's clear control is this command's only route, and it
            // sits on one page at a time. Clearing every page from a control
            // labelled for the visible one would discard work the user cannot
            // see, so the command owns the same page scope the control shows.
            Self::ClearConsole => match app.state.workbench.console_page {
                ConsolePage::Console
                    if app.state.workbench.console_view.channel == ConsoleChannel::Session =>
                {
                    app.state.clear_primary_log()
                }
                ConsolePage::Console => {}
                ConsolePage::Problems
                | ConsolePage::Measurements
                | ConsolePage::TaskLog
                | ConsolePage::References => {}
            },
            Self::ToggleFocusMode => {
                app.state.workbench.focus_mode = !app.state.workbench.focus_mode;
                if app.state.workbench.focus_mode {
                    app.state.workbench.close_drawer();
                }
            }
            Self::ResetLayout => app.state.workbench.reset_layout(),
            Self::NewApplicationWindow => crate::workbench::app::open_window_workflow(
                app,
                crate::workbench::app::WindowWorkflow::NewApplicationWindow,
            ),
            Self::DetachDocument => crate::workbench::app::open_window_workflow(
                app,
                crate::workbench::app::WindowWorkflow::DetachDocument,
            ),
            Self::MoveDocumentToWindow => crate::workbench::app::open_window_workflow(
                app,
                crate::workbench::app::WindowWorkflow::MoveDocument,
            ),
            Self::ReattachDocument => crate::workbench::app::open_window_workflow(
                app,
                crate::workbench::app::WindowWorkflow::ReattachDocument,
            ),
            Self::ConsolidateWindows => crate::workbench::app::open_window_workflow(
                app,
                crate::workbench::app::WindowWorkflow::ConsolidateWindows,
            ),
            Self::MonitorRecovery => crate::workbench::app::open_window_workflow(
                app,
                crate::workbench::app::WindowWorkflow::MonitorRecovery,
            ),
            Self::PreviousDocument => {
                crate::workbench::chrome::document_bar::cycle_document(&mut app.state, true);
            }
            Self::NextDocument => {
                crate::workbench::chrome::document_bar::cycle_document(&mut app.state, false);
            }
            Self::NextPane => app.state.workbench.pane_focus.request(PaneStep::Next),
            Self::PreviousPane => app.state.workbench.pane_focus.request(PaneStep::Previous),
            Self::CloseOtherDocuments => {
                let count =
                    crate::workbench::chrome::document_bar::close_other_documents(&mut app.state);
                app.state.prompt(format!(
                    "Closed {count} other document tab(s); the project data is kept."
                ));
            }
            Self::CloseAllDocuments => {
                let count =
                    crate::workbench::chrome::document_bar::close_all_documents(&mut app.state);
                app.state.prompt(format!(
                    "Closed {count} document tab(s); the project data is kept."
                ));
            }
            Self::WindowManager => app
                .state
                .workbench
                .window_session_dialog
                .open(crate::workbench::app::WindowSessionPage::Windows),
            Self::MinimizeWindow => app
                .state
                .workbench
                .outbox
                .push(crate::workbench::framework::requests::WindowFrame::Minimize),
            Self::ZoomWindow => app
                .state
                .workbench
                .outbox
                .push(crate::workbench::framework::requests::WindowFrame::Zoom),
            Self::MoveSelection => {
                crate::workbench::app::arm_move_selection(&mut app.state);
            }
            Self::StretchSelection => {
                crate::workbench::app::arm_stretch_selection(&mut app.state);
            }
            Self::ArraySelection => {
                crate::workbench::app::open_array_selection_dialog(&mut app.state);
            }
            Self::SwapToRSpiceSymbols => {
                app.state.model.schematic.swap_to_rspice_symbols();
            }
            // The operation reports its own outcome, including the name of any
            // terminal the master dropped, so no route composes a second
            // account of what happened.
            Self::UpdateInstanceInterface => {
                let outcome = app
                    .state
                    .model
                    .schematic
                    .update_selected_instance_interface(
                        &app.state.model.library_manager,
                        &app.state.model.workspace.content.schematic_buffers,
                    );
                app.state.prompt(match outcome {
                    Ok(summary) => summary,
                    Err(error) => error.to_string(),
                });
            }
            Self::FindReplace => crate::workbench::app::open_find_replace(&mut app.state),
            Self::RepeatLastCommand => match vocabulary::last_repeatable(
                app.state.workbench.last_repeatable_command.as_deref(),
            ) {
                Some(spec) if app.command_key_enabled(spec.key) => {
                    app.execute_command_key(spec.key);
                }
                Some(spec) => app.state.prompt(format!(
                    "{} cannot run here, so it was not repeated.",
                    spec.label
                )),
                None => app.state.prompt("No placement or edit has run yet."),
            },
            Self::ConfigurationSets => {
                crate::workbench::app::dialogs::project::configuration_dialogs::show_configuration_page(
                    &mut app.state,
                );
            }
            Self::SymbolPinTool => app.activate_symbol_tool(super::SymbolTool::PlacePin),
            Self::SymbolPolylineTool => app.activate_symbol_tool(super::SymbolTool::Line),
            Self::SymbolRectangleTool => app.activate_symbol_tool(super::SymbolTool::Rectangle),
            Self::SymbolCircleTool => app.activate_symbol_tool(super::SymbolTool::Circle),
            Self::SymbolArcTool => app.activate_symbol_tool(super::SymbolTool::Arc),
            Self::SymbolPolygonTool => app.activate_symbol_tool(super::SymbolTool::Polygon),
            Self::SymbolTextTool => app.activate_symbol_tool(super::SymbolTool::Text),
            Self::SymbolRotatePin => app.transform_selected_symbol_pin(
                rspice_schematic_editor::symbol_editor::commands::SymbolPinTransform::Rotate,
            ),
            Self::SymbolMirrorPin => app.transform_selected_symbol_pin(
                rspice_schematic_editor::symbol_editor::commands::SymbolPinTransform::Mirror,
            ),
            Self::SymbolUpdatePinsFromContract => {
                match app.state.update_active_symbol_pins_from_contract() {
                    Ok(text) | Err(text) => app.state.prompt(text),
                }
            }
            // The checks run before the dialog opens, so a symbol that cannot
            // be published says which row blocked it instead of presenting a
            // save transaction with a disabled primary control.
            Self::SymbolSave => match app.state.refuse_unsavable_active_symbol() {
                Some(refusal) => app.state.prompt(refusal),
                None => {
                    app.state.design.symbol.editor.save_dialog_open = true;
                    app.state.design.symbol.editor.save_error = None;
                }
            },
            Self::EditSymbolInPlace => {
                app.state.edit_selected_symbol_in_place();
            }
            Self::ExitSymbolInPlace => app.state.request_symbol_in_place_exit(
                super::app::actions::workspace::symbol_in_place::SymbolInPlaceExit::Parent,
            ),
            Self::RotateSelection if active_symbol_editor(app) => {
                app.transform_selected_symbol_item(SymbolTransform::Rotate);
            }
            Self::RotateSelectionCounterClockwise if active_symbol_editor(app) => {
                app.transform_selected_symbol_item(SymbolTransform::RotateCounterClockwise);
            }
            Self::MirrorSelectionHorizontal if active_symbol_editor(app) => {
                app.transform_selected_symbol_item(SymbolTransform::MirrorHorizontal);
            }
            Self::MirrorSelectionVertical if active_symbol_editor(app) => {
                app.transform_selected_symbol_item(SymbolTransform::MirrorVertical);
            }
            Self::RotateSelection
            | Self::RotateSelectionCounterClockwise
            | Self::MirrorSelectionHorizontal
            | Self::MirrorSelectionVertical => {
                let transform = match self {
                    Self::RotateSelectionCounterClockwise => {
                        SelectionTransform::RotateCounterClockwise
                    }
                    Self::MirrorSelectionHorizontal => SelectionTransform::MirrorHorizontal,
                    Self::MirrorSelectionVertical => SelectionTransform::MirrorVertical,
                    _ => SelectionTransform::RotateClockwise,
                };
                // A copy floating under the pointer is what these act on.
                if !crate::workbench::app::turn_floating_paste(&mut app.state, transform) {
                    let symbol_context = schematic_symbol_context(&app.state);
                    transform_selection_on_active_sheet(&mut app.state, &symbol_context, transform);
                }
            }
            Self::JoinCollinearWires => {
                app.state.join_collinear_wires_on_active_sheet();
            }
            Self::SnapSelectionToGrid => {
                let symbol_context = schematic_symbol_context(&app.state);
                crate::schematic::view::grid_snap_selection::run_snap_selection_to_grid(
                    &mut app.state,
                    &symbol_context,
                );
            }
            Self::SnapToStandardValue => {
                crate::schematic::view::standard_value_snap::run_snap_to_standard_value(
                    &mut app.state,
                );
            }
            Self::SetStandardValueSeries(series) => {
                let workspace = &mut app.state.model.workspace.content;
                if workspace.standard_value_series != series {
                    workspace.standard_value_series = series;
                    workspace.project_metadata_dirty = true;
                }
            }
            Self::AlignSelectionLeft
            | Self::AlignSelectionCenter
            | Self::AlignSelectionRight
            | Self::AlignSelectionTop
            | Self::AlignSelectionMiddle
            | Self::AlignSelectionBottom
            | Self::DistributeSelectionHorizontal
            | Self::DistributeSelectionVertical => {
                let command =
                    selection_layout_command(self).expect("selection layout command is mapped");
                let symbol_context = schematic_symbol_context(&app.state);
                match crate::schematic::view::selection_layout::apply_selection_layout(
                    &mut app.state,
                    &symbol_context,
                    command,
                ) {
                    // The moved selection is the answer to a completed layout.
                    Ok(true) => {}
                    Ok(false) => app
                        .state
                        .prompt(format!("{} produced no geometry change.", command.label())),
                    Err(error) => app
                        .state
                        .prompt(format!("{} was not applied: {error}.", command.label())),
                }
            }
            Self::Cancel => {
                if active_symbol_editor(app) && app.state.symbol_in_place_canvas_idle() {
                    // Nothing on the canvas to cancel: Escape leaves the
                    // in-place edit, as Return does.
                    app.state.request_symbol_in_place_exit(
                        super::app::actions::workspace::symbol_in_place::SymbolInPlaceExit::Parent,
                    );
                } else if active_symbol_editor(app) {
                    // The symbol canvas's one interaction in flight is an
                    // open polyline; Escape keeps what has been drawn.
                    app.finish_pending_symbol_polyline();
                    rspice_schematic_editor::symbol_editor::commands::finish_cancel(
                        &mut app.state.design.symbol.editor,
                    );
                } else if app.state.workbench.workspace == Workspace::Design
                    && app.state.workbench.inline_edit.session().is_some()
                {
                    app.state.workbench.inline_edit.end();
                } else if app.state.cancel_schematic_drag() {
                    // A pointer edit is the innermost active interaction.
                } else if app.state.model.schematic.session.editor.zoom_box.armed {
                    app.state.model.schematic.session.editor.zoom_box.disarm();
                } else if app.state.design.descend_hierarchy.open {
                    app.state.design.descend_hierarchy.close();
                } else if crate::workbench::app::cancel_floating_paste(&mut app.state) {
                    // The floating copy is let go; the drawing never changed.
                } else if app.state.design.move_selection.armed {
                    crate::workbench::app::cancel_armed_move_selection(&mut app.state);
                } else if app.state.design.stretch_selection.armed {
                    crate::workbench::app::cancel_armed_stretch_selection(&mut app.state);
                } else if app.state.design.array_selection.armed {
                    crate::workbench::app::cancel_armed_array_selection(&mut app.state);
                } else if app.state.workbench.drawer.is_some() {
                    app.state.workbench.close_drawer();
                } else if app.state.design.object_properties.open {
                    app.state.design.object_properties.close();
                } else if app.state.design.tabbed_property_dialog.open {
                    app.state.design.tabbed_property_dialog.attempt_close();
                } else if app.route_command_key(ids::CANCEL) {
                    // The workspace in front let go of what it had placed.
                } else {
                    app.state.model.schematic.cancel_interaction_step();
                }
            }
            Self::AscendHierarchy => {
                app.state.ascend_workspace_level();
            }
            Self::DescendHierarchy => {
                crate::workbench::app::open_descend_hierarchy_dialog(&mut app.state);
            }
            Self::DescendHierarchyDirect => {
                app.state.open_selected_instance_master();
            }
            Self::CheckCellview if active_symbol_editor(app) => {
                app.state.run_active_symbol_pin_checks();
            }
            Self::CheckCellview => check_cellview(&mut app.state),
            Self::CheckDesign => check_design(&mut app.state),
            Self::ClearChecks => {
                if active_symbol_editor(app) {
                    app.state.clear_active_symbol_pin_checks();
                } else {
                    app.state.clear_active_design_check();
                }
                app.state.model.design_checks.drc_results = None;
                app.state.model.design_checks.drc_checked_version = 0;
            }
            Self::NextViolation => {
                crate::schematic::view::violations::cycle_violation(&mut app.state, 1);
            }
            Self::PreviousViolation => {
                crate::schematic::view::violations::cycle_violation(&mut app.state, -1);
            }
            Self::CommandPalette => open_command_palette(&mut app.state),
            Self::KeyboardShortcuts => app.state.shell.shortcuts_help = true,
            Self::HelpCenter => {
                crate::workbench::app::dialogs::help::site::open_docs(&mut app.state)
            }
            Self::Examples => app.state.project.open_examples(),
            Self::ReleaseNotes => crate::workbench::app::dialogs::help::site::open_release_notes(
                &mut app.state.workbench.outbox,
            ),
            Self::SystemDiagnostics => open_help_center(
                &mut app.state,
                crate::workbench::app::HelpCenterPage::Diagnostics,
            ),
            Self::SupportBundle => open_help_center(
                &mut app.state,
                crate::workbench::app::HelpCenterPage::SupportBundle,
            ),
            Self::LegalPrivacy => open_help_center(
                &mut app.state,
                crate::workbench::app::HelpCenterPage::LegalPrivacy,
            ),
            Self::About => app.state.shell.about = true,
        }
    }
}

mod command_helpers;

pub(crate) use command_helpers::menu_label;
use command_helpers::*;
pub(in crate::workbench) use command_helpers::{
    active_schematic_editor, answer_code_request, answer_netlist_editor_request,
    answer_project_request, file_action, foreign_import_refusal, run_project_file_action,
};

#[cfg(test)]
pub(crate) mod tests;
