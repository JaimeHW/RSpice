//! The workbench commands the design feature takes a part in.
//!
//! The view commands move the open schematic's or symbol's viewport. Results
//! takes them first where it is in front; the design feature takes them in a
//! schematic or symbol editor, and nowhere else is there a view to move.
//! Each acts on the innermost view: a symbol edited in place, then the
//! symbol editor, then the schematic.
//!
//! The edit commands (Cut, Copy, Paste, Duplicate, Delete, Select all and
//! Deselect all) act on the selection in a schematic or symbol editor;
//! Results takes Copy first where it is in front. Find in design opens the
//! Navigator's search from anywhere but a Verilog-A cell view.
//!
//! Undo and Redo step the design's histories everywhere the shell lets the
//! key through: the shell steps the netlist page's own history first.
//!
//! Cancel lets go of the innermost thing the design workspace holds, from
//! any workspace: the shell asks the design feature first, Results after it.

use rspice_schematic_editor::symbol_editor::commands::finish_cancel;
use rspice_schematic_editor::view::navigation::ZoomRequest;

use crate::state::ViewType;
use crate::workbench::app::actions::workspace::symbol_in_place::{
    SymbolInPlaceExit, in_place_bounds, in_place_placement, in_place_session, in_place_session_mut,
    symbol_in_place_canvas_idle,
};
use crate::workbench::app::actions::workspace::{
    HistoryDirection, HistoryStepParts, take_history_step,
};
use crate::workbench::app::actions::{
    copy_selected_symbol_shape, delete_selected_symbol_item, duplicate_selected_symbol_shape,
    paste_symbol_shape, select_all_symbol_items,
};
use crate::workbench::app::dialogs::selection::commands::{
    schematic_selection_has_deletable_object, schematic_selection_has_duplicable_object,
    schematic_selection_has_live_object,
};
use crate::workbench::app::{
    cancel_armed_array_selection_in, cancel_armed_move_selection_in,
    cancel_armed_stretch_selection_in, cancel_floating_paste_in, copy_active_schematic_selection,
    cut_schematic_selection, delete_schematic_selection, duplicate_schematic_selection,
    paste_schematic_clipboard, select_all_schematic_objects,
};
use crate::workbench::app_state::Model;
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::{Cx, Reads, RoutedCommand};
use crate::workbench::framework::requests::FocusNavigatorSearch;
use crate::workbench::slots::DesignState;
use crate::workbench::state::{Drawer, WorkbenchSession, Workspace};

use super::DesignFeature;

/// How far one Zoom in or Zoom out magnifies.
const COMMAND_ZOOM_FACTOR: f64 = 1.2;

pub(super) const ROUTED: &[RoutedCommand<DesignFeature>] = &[
    RoutedCommand {
        key: ids::ZOOM_IN,
        takes: in_editor,
        enabled: |_| true,
        execute: zoom_in,
    },
    RoutedCommand {
        key: ids::ZOOM_OUT,
        takes: in_editor,
        enabled: |_| true,
        execute: zoom_out,
    },
    RoutedCommand {
        key: ids::ZOOM_FIT,
        takes: in_editor,
        enabled: |_| true,
        execute: zoom_fit,
    },
    RoutedCommand {
        key: ids::ZOOM_ONE_TO_ONE,
        takes: in_editor,
        enabled: |_| true,
        execute: zoom_one_to_one,
    },
    RoutedCommand {
        key: ids::ZOOM_TO_SELECTION,
        takes: in_editor,
        enabled: |reads| {
            if symbol_editor(reads.wb.workspace, reads.model) {
                !reads.state.symbol.editor.effective_selection().is_empty()
            } else {
                schematic_selection_has_live_object(&reads.model.schematic)
            }
        },
        execute: zoom_to_selection,
    },
    RoutedCommand {
        key: ids::ZOOM_BOX,
        takes: in_editor,
        enabled: |_| true,
        execute: |cx| {
            if symbol_editor(cx.wb.workspace, cx.model) {
                cx.state.symbol.editor.zoom_box.arm();
            } else {
                cx.wb.activate(Workspace::Design);
                cx.model.schematic.session.editor.zoom_box.arm();
            }
        },
    },
    RoutedCommand {
        key: ids::ZOOM_PREVIOUS,
        takes: in_editor,
        enabled: can_step_back,
        execute: zoom_previous,
    },
    RoutedCommand {
        key: ids::CUT,
        takes: in_editor,
        enabled: can_cut,
        execute: cut_selection,
    },
    RoutedCommand {
        key: ids::COPY,
        takes: in_editor,
        enabled: can_copy,
        execute: copy_selection,
    },
    RoutedCommand {
        key: ids::PASTE,
        takes: in_editor,
        enabled: can_paste,
        execute: paste_clipboard,
    },
    RoutedCommand {
        key: ids::DUPLICATE,
        takes: in_editor,
        enabled: can_duplicate,
        execute: duplicate_selection,
    },
    RoutedCommand {
        key: ids::DELETE,
        takes: in_editor,
        enabled: can_delete,
        execute: delete_selection,
    },
    RoutedCommand {
        key: ids::SELECT_ALL,
        takes: in_editor,
        enabled: |_| true,
        execute: select_all,
    },
    RoutedCommand {
        key: ids::DESELECT_ALL,
        takes: in_editor,
        enabled: has_selection,
        execute: deselect_all,
    },
    RoutedCommand {
        key: ids::FIND_IN_DESIGN,
        takes: finds_in_design,
        enabled: |_| true,
        execute: find_in_design,
    },
    RoutedCommand {
        key: ids::UNDO,
        takes: |_| true,
        enabled: can_undo,
        execute: undo,
    },
    RoutedCommand {
        key: ids::REDO,
        takes: |_| true,
        enabled: can_redo,
        execute: redo,
    },
    RoutedCommand {
        key: ids::CANCEL,
        takes: |reads| cancel_step(reads.state, reads.model, reads.wb).is_some(),
        enabled: |_| true,
        execute: cancel,
    },
];

/// Whether the symbol editor is in front: a symbol open in the Design or
/// Models workspace.
pub(super) fn symbol_editor(workspace: Workspace, model: &Model) -> bool {
    matches!(workspace, Workspace::Design | Workspace::Models)
        && model.workspace.content.active_view_type() == Some(ViewType::Symbol)
}

/// Whether a schematic or testbench is in front in the Design workspace.
fn schematic_editor(workspace: Workspace, model: &Model) -> bool {
    workspace == Workspace::Design
        && matches!(
            model.workspace.content.active_view_type(),
            Some(ViewType::Schematic | ViewType::Testbench)
        )
}

fn in_editor(reads: &Reads<'_, DesignFeature>) -> bool {
    symbol_editor(reads.wb.workspace, reads.model) || in_schematic(reads)
}

fn in_schematic(reads: &Reads<'_, DesignFeature>) -> bool {
    schematic_editor(reads.wb.workspace, reads.model)
}

fn zoom_in(cx: &mut Cx<'_, DesignFeature>) {
    if let Some(session) = in_place_session_mut(&mut cx.state.symbol.in_place, cx.model) {
        session.zoom_request = Some(ZoomRequest::By(COMMAND_ZOOM_FACTOR));
    } else if symbol_editor(cx.wb.workspace, cx.model) {
        cx.state.symbol.editor.zoom_request = Some(ZoomRequest::By(COMMAND_ZOOM_FACTOR));
    } else {
        cx.model.schematic.session.editor.zoom_request = Some(ZoomRequest::By(COMMAND_ZOOM_FACTOR));
    }
}

fn zoom_out(cx: &mut Cx<'_, DesignFeature>) {
    if let Some(session) = in_place_session_mut(&mut cx.state.symbol.in_place, cx.model) {
        session.zoom_request = Some(ZoomRequest::By(COMMAND_ZOOM_FACTOR.recip()));
    } else if symbol_editor(cx.wb.workspace, cx.model) {
        cx.state.symbol.editor.zoom_request = Some(ZoomRequest::By(COMMAND_ZOOM_FACTOR.recip()));
    } else {
        cx.model.schematic.session.editor.zoom_request =
            Some(ZoomRequest::By(COMMAND_ZOOM_FACTOR.recip()));
    }
}

/// Fit frames the symbol edited in place on its parent, fits the symbol
/// editor's drawing, or fits the schematic's drawing sheet.
fn zoom_fit(cx: &mut Cx<'_, DesignFeature>) {
    if let Some(bounds) = in_place_session(&cx.state.symbol.in_place, cx.model)
        .and_then(|session| in_place_bounds(session, cx.model))
        && let Some(session) = in_place_session_mut(&mut cx.state.symbol.in_place, cx.model)
    {
        session.zoom_request = Some(ZoomRequest::Frame(bounds));
    } else if symbol_editor(cx.wb.workspace, cx.model) {
        cx.state.symbol.editor.needs_fit = true;
    } else {
        let editor = &mut cx.model.schematic.session.editor;
        editor.needs_drawing_sheet_fit = true;
        editor.needs_fit = false;
    }
}

fn zoom_one_to_one(cx: &mut Cx<'_, DesignFeature>) {
    if let Some(session) = in_place_session_mut(&mut cx.state.symbol.in_place, cx.model) {
        session.zoom_request = Some(ZoomRequest::To(1.0));
    } else if symbol_editor(cx.wb.workspace, cx.model) {
        cx.state.symbol.editor.zoom_request = Some(ZoomRequest::To(1.0));
    } else {
        cx.model.schematic.session.editor.zoom_request = Some(ZoomRequest::To(1.0));
    }
}

/// Frame the selection: the symbol's pins and shapes, on the parent sheet
/// where the symbol is edited in place, or the schematic's objects.
fn zoom_to_selection(cx: &mut Cx<'_, DesignFeature>) {
    if symbol_editor(cx.wb.workspace, cx.model) {
        let selection = cx.state.symbol.editor.effective_selection();
        let placement = in_place_session(&cx.state.symbol.in_place, cx.model)
            .and_then(|session| in_place_placement(session, cx.model))
            .unwrap_or_default();
        let Some(bounds) = symbol_selection_bounds(cx.model, &selection, placement) else {
            return;
        };
        if let Some(session) = in_place_session_mut(&mut cx.state.symbol.in_place, cx.model) {
            session.zoom_request = Some(ZoomRequest::Frame(bounds));
        } else {
            cx.state.symbol.editor.zoom_request = Some(ZoomRequest::Frame(bounds));
        }
        return;
    }
    let symbol_context = cx.model.schematic_symbol_context();
    let schematic = &cx.model.schematic;
    if let Some((min_x, min_y, max_x, max_y)) =
        symbol_context.selection_bounds(schematic.document(), &schematic.session.editor.selection)
    {
        cx.model.schematic.session.editor.zoom_request = Some(ZoomRequest::Frame((
            f64::from(min_x),
            f64::from(min_y),
            f64::from(max_x),
            f64::from(max_y),
        )));
    }
}

/// The extent of the selected pins and shapes, carried by `placement` into
/// the plane the canvas frames: the identity in the symbol's own view, the
/// instance's transform onto its parent sheet in place.
fn symbol_selection_bounds(
    model: &Model,
    selection: &rspice_schematic_editor::symbol_editor::session::SymbolSelection,
    placement: rspice_schematic_editor::symbol_editor::placement::SymbolPlacement,
) -> Option<(f64, f64, f64, f64)> {
    use rspice_design_model::Point;
    let document = model.load_active_symbol_document().ok()?;
    let (min, max) = rspice_schematic_editor::symbol_editor::commands::clipboard_from_selection(
        &document, selection,
    )
    .bounds()?;
    let corners = [min, max, Point::new(min.x, max.y), Point::new(max.x, min.y)]
        .map(|corner| placement.place(corner));
    let low = |pick: fn(&Point) -> i32| corners.iter().map(pick).min().map(f64::from);
    let high = |pick: fn(&Point) -> i32| corners.iter().map(pick).max().map(f64::from);
    Some((
        low(|point| point.x)?,
        low(|point| point.y)?,
        high(|point| point.x)?,
        high(|point| point.y)?,
    ))
}

/// Whether the view in front has one to step back to.
fn can_step_back(reads: &Reads<'_, DesignFeature>) -> bool {
    if let Some(session) = in_place_session(&reads.state.symbol.in_place, reads.model) {
        session.view_history.can_step_back()
    } else if symbol_editor(reads.wb.workspace, reads.model) {
        reads.state.symbol.editor.view_history.can_step_back()
    } else {
        reads
            .model
            .schematic
            .session
            .editor
            .view_history
            .can_step_back()
    }
}

/// The view in front steps back to the one it showed before; stepping back
/// is not itself a view the history keeps.
fn zoom_previous(cx: &mut Cx<'_, DesignFeature>) {
    if let Some(session) = in_place_session_mut(&mut cx.state.symbol.in_place, cx.model) {
        if let Some(view) = session.view_history.step_back() {
            (session.zoom, session.pan) = view;
        }
    } else if symbol_editor(cx.wb.workspace, cx.model) {
        let editor = &mut cx.state.symbol.editor;
        if let Some(view) = editor.view_history.step_back() {
            (editor.zoom, editor.pan) = view;
        }
    } else {
        let editor = &mut cx.model.schematic.session.editor;
        if let Some(view) = editor.view_history.step_back() {
            (editor.zoom, editor.pan) = view;
        }
    }
}

/// Whether the symbol editor's selection can be cut, duplicated or deleted:
/// the symbol can change and something is selected.
fn symbol_selection_editable(reads: &Reads<'_, DesignFeature>) -> bool {
    !reads.model.active_view_read_only()
        && !reads.state.symbol.editor.effective_selection().is_empty()
}

fn can_cut(reads: &Reads<'_, DesignFeature>) -> bool {
    if symbol_editor(reads.wb.workspace, reads.model) {
        symbol_selection_editable(reads)
    } else {
        !reads.model.schematic_edit_read_only()
            && schematic_selection_has_live_object(&reads.model.schematic)
    }
}

fn can_copy(reads: &Reads<'_, DesignFeature>) -> bool {
    if symbol_editor(reads.wb.workspace, reads.model) {
        !reads.state.symbol.editor.effective_selection().is_empty()
    } else {
        schematic_selection_has_live_object(&reads.model.schematic)
    }
}

fn can_paste(reads: &Reads<'_, DesignFeature>) -> bool {
    if symbol_editor(reads.wb.workspace, reads.model) {
        !reads.model.active_view_read_only() && !reads.state.symbol.editor.clipboard.is_empty()
    } else {
        !reads.model.schematic_edit_read_only() && !reads.state.schematic_clipboard.is_empty()
    }
}

fn can_duplicate(reads: &Reads<'_, DesignFeature>) -> bool {
    if symbol_editor(reads.wb.workspace, reads.model) {
        symbol_selection_editable(reads)
    } else {
        !reads.model.schematic_edit_read_only()
            && schematic_selection_has_duplicable_object(&reads.model.schematic)
    }
}

fn can_delete(reads: &Reads<'_, DesignFeature>) -> bool {
    if symbol_editor(reads.wb.workspace, reads.model) {
        symbol_selection_editable(reads)
    } else {
        !reads.model.schematic_edit_read_only()
            && schematic_selection_has_deletable_object(&reads.model.schematic)
    }
}

fn cut_selection(cx: &mut Cx<'_, DesignFeature>) {
    if symbol_editor(cx.wb.workspace, cx.model) {
        let modal_open = cx.application_modal_open();
        delete_selected_symbol_item(&mut cx.state.symbol, cx.model, cx.wb, modal_open, true);
    } else {
        cut_schematic_selection(&mut cx.state.schematic_clipboard, cx.model, cx.wb);
    }
}

/// Copy keeps only what the active sheet shows of the schematic's
/// selection, then copies it.
fn copy_selection(cx: &mut Cx<'_, DesignFeature>) {
    if symbol_editor(cx.wb.workspace, cx.model) {
        copy_selected_symbol_shape(&mut cx.state.symbol, cx.model, cx.wb);
    } else {
        cx.model.retain_selection_on_active_sheet();
        copy_active_schematic_selection(&mut cx.state.schematic_clipboard, cx.model);
    }
}

fn paste_clipboard(cx: &mut Cx<'_, DesignFeature>) {
    if symbol_editor(cx.wb.workspace, cx.model) {
        let modal_open = cx.application_modal_open();
        paste_symbol_shape(
            &mut cx.state.symbol,
            cx.model,
            cx.wb,
            cx.state.canvas_hover,
            cx.state.canvas_view_center,
            modal_open,
        );
    } else {
        paste_schematic_clipboard(cx.state.floating_copy_parts(), cx.model, cx.wb);
    }
}

fn duplicate_selection(cx: &mut Cx<'_, DesignFeature>) {
    if symbol_editor(cx.wb.workspace, cx.model) {
        let modal_open = cx.application_modal_open();
        duplicate_selected_symbol_shape(
            &mut cx.state.symbol,
            cx.model,
            cx.wb,
            cx.state.canvas_hover,
            cx.state.canvas_view_center,
            modal_open,
        );
    } else {
        duplicate_schematic_selection(cx.state.floating_copy_parts(), cx.model, cx.wb);
    }
}

fn delete_selection(cx: &mut Cx<'_, DesignFeature>) {
    if symbol_editor(cx.wb.workspace, cx.model) {
        let modal_open = cx.application_modal_open();
        delete_selected_symbol_item(&mut cx.state.symbol, cx.model, cx.wb, modal_open, false);
    } else {
        delete_schematic_selection(cx.model, cx.wb);
    }
}

fn select_all(cx: &mut Cx<'_, DesignFeature>) {
    if symbol_editor(cx.wb.workspace, cx.model) {
        select_all_symbol_items(&mut cx.state.symbol, cx.model, cx.wb);
    } else {
        select_all_schematic_objects(cx.state.schematic_selection_filter, cx.model, cx.wb);
    }
}

/// Whether the drawing in front has anything selected: the symbol's pins and
/// shapes, or the schematic's objects.
fn has_selection(reads: &Reads<'_, DesignFeature>) -> bool {
    if symbol_editor(reads.wb.workspace, reads.model) {
        !reads.state.symbol.editor.effective_selection().is_empty()
    } else {
        !reads.model.schematic.session.editor.selection.is_empty()
    }
}

/// Clear the drawing's selection: the symbol's in a symbol editor, the
/// active sheet's otherwise, in the session and in the sheet's buffer, as
/// Select all writes both.
fn deselect_all(cx: &mut Cx<'_, DesignFeature>) {
    if symbol_editor(cx.wb.workspace, cx.model) {
        cx.state.symbol.editor.clear_selection();
        return;
    }
    let model = &mut *cx.model;
    model.schematic.session.editor.selection.clear();
    let active_key = model.workspace.content.active_schematic_reference().key();
    if let Some(mut buffer) = model.workspace.schematic_editor_mut(&active_key) {
        buffer.editor.session.editor.selection.clear();
    }
}

/// Whether Find searches the design: anywhere but a Verilog-A cell view in
/// the Design workspace, which has no instances or nets to find. The shell
/// gives the netlist page's find to code before it routes the command.
fn finds_in_design(reads: &Reads<'_, DesignFeature>) -> bool {
    reads.wb.workspace != Workspace::Design
        || !reads.model.workspace.content.active_view_is_hdl_source()
}

/// Bring the Navigator forward on the Design workspace and focus its search
/// box. The box is the Navigator tab's; the Component shelf that Place
/// instance brings forward has none.
fn find_in_design(cx: &mut Cx<'_, DesignFeature>) {
    cx.wb.activate(Workspace::Design);
    cx.wb.navigator_visible = true;
    cx.wb.drawer = Some(Drawer::Navigator);
    cx.state.show_navigator();
    cx.wb.outbox.push(FocusNavigatorSearch);
}

/// Whether `workspace` edits the simulation setup, so that its Undo and Redo
/// step the setup's own history: the Simulate pages, and the Verify views,
/// whose specification and acceptance edits are plan changes.
fn steps_the_setup(workspace: Workspace) -> bool {
    matches!(workspace, Workspace::Simulate | Workspace::Verify)
}

/// Whether Undo would take a step: on a Simulate page or a Verify view, where
/// the setup's own history holds one; elsewhere never while the project is
/// read-only or a schematic edit is under way, and otherwise where the
/// project's history, the symbol editor's or an editable schematic's holds
/// one.
fn can_undo(reads: &Reads<'_, DesignFeature>) -> bool {
    let model = reads.model;
    if steps_the_setup(reads.wb.workspace) {
        model.can_undo_simulation_setup()
    } else if model.safe_mode.project_read_only() || reads.state.schematic_edit_in_progress(model) {
        false
    } else if model.can_undo_project_design() {
        true
    } else if symbol_editor(reads.wb.workspace, model) {
        reads
            .state
            .symbol
            .history
            .can_undo(&model.workspace.content.active_key())
    } else {
        in_schematic(reads) && !model.schematic_edit_read_only() && model.schematic.can_undo()
    }
}

/// Whether Redo would take a step, as [`can_undo`] answers for Undo.
fn can_redo(reads: &Reads<'_, DesignFeature>) -> bool {
    let model = reads.model;
    if steps_the_setup(reads.wb.workspace) {
        model.can_redo_simulation_setup()
    } else if model.safe_mode.project_read_only() || reads.state.schematic_edit_in_progress(model) {
        false
    } else if model.can_redo_project_design() {
        true
    } else if symbol_editor(reads.wb.workspace, model) {
        reads
            .state
            .symbol
            .history
            .can_redo(&model.workspace.content.active_key())
    } else {
        in_schematic(reads)
            && !model.schematic_edit_read_only()
            && model.can_redo_active_schematic()
    }
}

fn undo(cx: &mut Cx<'_, DesignFeature>) {
    take_history_step(cx, history_parts, None, HistoryDirection::Undo);
}

fn redo(cx: &mut Cx<'_, DesignFeature>) {
    take_history_step(cx, history_parts, None, HistoryDirection::Redo);
}

/// What a history step reads and writes of the design feature's context.
fn history_parts<'a>(cx: &'a mut Cx<'_, DesignFeature>) -> HistoryStepParts<'a> {
    HistoryStepParts {
        design: cx.state.history_parts(cx.model),
        model: &mut *cx.model,
        wb: &mut *cx.wb,
        runs: &mut *cx.runs,
        run_controller: &mut *cx.handles.run_controller,
        selected_model: cx.peers.models().selected_model(),
    }
}

/// What Cancel lets go of, innermost first.
#[derive(Debug, Clone, Copy)]
enum CancelStep {
    /// Nothing on the in-place symbol's canvas to cancel: Escape leaves the
    /// in-place edit, as Return does.
    LeaveSymbolInPlace,
    /// The shape being drawn in the symbol editor, keeping its tool armed;
    /// with nothing part drawn, the tool itself.
    SymbolDraft,
    InlineEdit,
    /// A pointer edit on the schematic: the innermost active interaction.
    SchematicDrag,
    ZoomBox,
    DescendHierarchy,
    /// The floating copy; the drawing never changed.
    FloatingPaste,
    MoveSelection,
    StretchSelection,
    ArraySelection,
    Drawer,
    ObjectProperties,
    TabbedPropertyDialog,
}

/// The step Cancel takes here, or `None` when the design workspace holds
/// nothing for it to let go of.
fn cancel_step(state: &DesignState, model: &Model, wb: &WorkbenchSession) -> Option<CancelStep> {
    Some(if symbol_editor(wb.workspace, model) {
        if symbol_in_place_canvas_idle(&state.symbol, model) {
            CancelStep::LeaveSymbolInPlace
        } else {
            CancelStep::SymbolDraft
        }
    } else if wb.workspace == Workspace::Design && wb.inline_edit.session().is_some() {
        CancelStep::InlineEdit
    } else if state.interaction.drag.owner.is_some() {
        CancelStep::SchematicDrag
    } else if model.schematic.session.editor.zoom_box.armed {
        CancelStep::ZoomBox
    } else if state.descend_hierarchy.open {
        CancelStep::DescendHierarchy
    } else if state.floating_paste.is_some() {
        CancelStep::FloatingPaste
    } else if state.move_selection.armed {
        CancelStep::MoveSelection
    } else if state.stretch_selection.armed {
        CancelStep::StretchSelection
    } else if state.array_selection.armed {
        CancelStep::ArraySelection
    } else if wb.drawer.is_some() {
        CancelStep::Drawer
    } else if state.object_properties.open {
        CancelStep::ObjectProperties
    } else if state.tabbed_property_dialog.open {
        CancelStep::TabbedPropertyDialog
    } else {
        return None;
    })
}

fn cancel(cx: &mut Cx<'_, DesignFeature>) {
    let Some(step) = cancel_step(cx.state, cx.model, cx.wb) else {
        return;
    };
    match step {
        CancelStep::LeaveSymbolInPlace => {
            super::navigation(cx).request_symbol_in_place_exit(SymbolInPlaceExit::Parent);
        }
        CancelStep::SymbolDraft => finish_cancel(&mut cx.state.symbol.editor),
        CancelStep::InlineEdit => cx.wb.inline_edit.end(),
        CancelStep::SchematicDrag => {
            cx.state.interaction.drag.cancel_schematic_drag(cx.model);
        }
        CancelStep::ZoomBox => cx.model.schematic.session.editor.zoom_box.disarm(),
        CancelStep::DescendHierarchy => cx.state.descend_hierarchy.close(),
        CancelStep::FloatingPaste => {
            cancel_floating_paste_in(&mut cx.state.floating_paste, cx.wb);
        }
        CancelStep::MoveSelection => {
            cancel_armed_move_selection_in(&mut cx.state.move_selection, cx.model);
        }
        CancelStep::StretchSelection => {
            cancel_armed_stretch_selection_in(&mut cx.state.stretch_selection, cx.model);
        }
        CancelStep::ArraySelection => {
            cancel_armed_array_selection_in(&mut cx.state.array_selection, cx.model);
        }
        CancelStep::Drawer => cx.wb.close_drawer(),
        CancelStep::ObjectProperties => cx.state.object_properties.close(),
        CancelStep::TabbedPropertyDialog => {
            cx.state.tabbed_property_dialog.attempt_close();
        }
    }
}
