//! The workbench commands the design feature takes a part in: the view
//! commands move the open schematic's or symbol's viewport. Results takes
//! them first where it is in front; the design feature takes them in a
//! schematic or symbol editor, and nowhere else is there a view to move.
//! Each acts on the innermost view: a symbol edited in place, then the
//! symbol editor, then the schematic.

use rspice_schematic_editor::view::navigation::ZoomRequest;

use crate::state::ViewType;
use crate::workbench::app::actions::workspace::symbol_in_place::{
    in_place_bounds, in_place_session, in_place_session_mut,
};
use crate::workbench::app::dialogs::selection::commands::schematic_selection_has_live_object;
use crate::workbench::app_state::Model;
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::{Cx, Reads, RoutedCommand};
use crate::workbench::state::Workspace;

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
        takes: in_schematic,
        enabled: |reads| schematic_selection_has_live_object(&reads.model.schematic),
        execute: zoom_to_selection,
    },
    RoutedCommand {
        key: ids::ZOOM_BOX,
        takes: in_schematic,
        enabled: |_| true,
        execute: |cx| {
            cx.wb.activate(Workspace::Design);
            cx.model.schematic.session.editor.zoom_box.arm();
        },
    },
    RoutedCommand {
        key: ids::ZOOM_PREVIOUS,
        takes: in_editor,
        enabled: can_step_back,
        execute: zoom_previous,
    },
];

/// Whether the symbol editor is in front: a symbol open in the Design or
/// Models workspace.
pub(super) fn symbol_editor(workspace: Workspace, model: &Model) -> bool {
    matches!(workspace, Workspace::Design | Workspace::Models)
        && model.workspace.content.active_view_type() == ViewType::Symbol
}

/// Whether a schematic or testbench is in front in the Design workspace.
fn schematic_editor(workspace: Workspace, model: &Model) -> bool {
    workspace == Workspace::Design
        && matches!(
            model.workspace.content.active_view_type(),
            ViewType::Schematic | ViewType::Testbench
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
        let editor = &mut cx.state.symbol.editor;
        editor.zoom = (editor.zoom * COMMAND_ZOOM_FACTOR as f32).min(16.0);
    } else {
        cx.model.schematic.session.editor.zoom_request = Some(ZoomRequest::By(COMMAND_ZOOM_FACTOR));
    }
}

fn zoom_out(cx: &mut Cx<'_, DesignFeature>) {
    if let Some(session) = in_place_session_mut(&mut cx.state.symbol.in_place, cx.model) {
        session.zoom_request = Some(ZoomRequest::By(COMMAND_ZOOM_FACTOR.recip()));
    } else if symbol_editor(cx.wb.workspace, cx.model) {
        let editor = &mut cx.state.symbol.editor;
        editor.zoom = (editor.zoom / COMMAND_ZOOM_FACTOR as f32).max(0.1);
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
        cx.state.symbol.editor.zoom = 1.0;
    } else {
        cx.model.schematic.session.editor.zoom_request = Some(ZoomRequest::To(1.0));
    }
}

fn zoom_to_selection(cx: &mut Cx<'_, DesignFeature>) {
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
