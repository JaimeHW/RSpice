//! App availability and command dispatch for the touch strip.
//!
//! The strip shows whenever a schematic or symbol canvas is driven by
//! touch, at any width (audit G-12). Every command button runs the same
//! command the menus and keys run; the two latches change what a one-finger
//! gesture does.

use crate::state::command_catalog::CommandKey;
use crate::state::{Tool, ViewType};
use crate::workbench::SurfaceId;
use crate::workbench::design::DesignFeature;
use crate::workbench::design::commands::DesignCommand;
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::Cx;
use egui::{Context, Rect};
use rspice_schematic_editor::view::mobile_controls::{
    self, MobileCanvasAction, MobileCanvasCapabilities, MobileCanvasRequest, StripCanvas,
};
use rspice_schematic_editor::view::pointer_target::canvas_touch_mode;

use super::parts::{
    CanvasFacts, canvas_application_modal_open_in, canvas_facts_in, canvas_parts_in,
};
use super::shelf_drag::PlacementTransform;

/// Draw the strip in the design feature's context, which holds the
/// dispatcher's answers for every button's command but Cancel's, which is
/// offered while an operation is live, and the modal facts. A
/// command a button runs is raised through the context, and the shell runs
/// it once the surface has drawn.
pub(crate) fn show(ctx: &Context, cx: &mut Cx<'_, DesignFeature>, content_rect: Rect) {
    if !canvas_touch_mode(ctx) {
        return;
    }
    let source = super::requests::editor_request_source(&canvas_facts_in(cx));
    let capabilities = MobileCanvasCapabilities {
        canvas: strip_canvas_of(cx.model.workspace.content.active_view_type()).unwrap_or_default(),
        rotate: action_enabled(cx, MobileCanvasAction::Rotate),
        mirror: action_enabled(cx, MobileCanvasAction::Mirror),
        cancel: action_enabled(cx, MobileCanvasAction::Cancel),
        undo: action_enabled(cx, MobileCanvasAction::Undo),
        redo: action_enabled(cx, MobileCanvasAction::Redo),
        fit: action_enabled(cx, MobileCanvasAction::Fit),
    };
    if let Some(request) = mobile_controls::show(ctx, content_rect, &source, capabilities) {
        execute_action(ctx, cx, request);
    }
}

const fn product_command(action: MobileCanvasAction) -> Option<CommandKey> {
    match action {
        MobileCanvasAction::SelectBox | MobileCanvasAction::SelectAdd => None,
        MobileCanvasAction::Rotate => Some(DesignCommand::RotateSelection.key()),
        MobileCanvasAction::Mirror => Some(DesignCommand::MirrorSelectionHorizontal.key()),
        MobileCanvasAction::Cancel => Some(ids::CANCEL),
        MobileCanvasAction::Undo => Some(ids::UNDO),
        MobileCanvasAction::Redo => Some(ids::REDO),
        MobileCanvasAction::Fit => Some(ids::ZOOM_FIT),
    }
}

/// The canvas the strip stands on, by the view in front; `None` for a view
/// that draws neither a schematic nor a symbol.
const fn strip_canvas_of(view_type: Option<ViewType>) -> Option<StripCanvas> {
    match view_type {
        Some(ViewType::Schematic | ViewType::Testbench) => Some(StripCanvas::Schematic),
        Some(ViewType::Symbol) => Some(StripCanvas::Symbol),
        _ => None,
    }
}

/// Whether a route, a move, a placement or a box is live: what Esc would
/// cancel on the canvas. On a symbol, whether a tool is armed, a shape or a
/// name is part made, or something is held.
pub(crate) fn operation_live(state: &CanvasFacts<'_>) -> bool {
    if state.model.workspace.content.active_view_type() == Some(ViewType::Symbol) {
        return state.design.symbol.editor.cancellable();
    }
    let editor = &state.model.schematic.session.editor;
    state.model.schematic.canvas_settings_change_blocked()
        || editor.tool != Tool::Select
        || editor.zoom_box.armed
        || state.design.move_selection.armed
        || state.design.stretch_selection.armed
        || state.design.array_selection.armed
        || state.schematic_drag_in_progress()
}

fn action_enabled(cx: &Cx<'_, DesignFeature>, action: MobileCanvasAction) -> bool {
    match action {
        MobileCanvasAction::SelectBox | MobileCanvasAction::SelectAdd => true,
        MobileCanvasAction::Cancel => operation_live(&canvas_facts_in(cx)),
        MobileCanvasAction::Rotate | MobileCanvasAction::Mirror
            if !symbol_in_front(cx)
                && super::shelf_drag::something_floats(&canvas_facts_in(cx)) =>
        {
            true
        }
        _ => product_command(action).is_some_and(|key| cx.foreign_available(key)),
    }
}

fn symbol_in_front(cx: &Cx<'_, DesignFeature>) -> bool {
    cx.model.workspace.content.active_view_type() == Some(ViewType::Symbol)
}

/// What R or M would turn while something floats under the finger.
const fn floating_transform(action: MobileCanvasAction) -> Option<PlacementTransform> {
    match action {
        MobileCanvasAction::Rotate => Some(PlacementTransform::RotateClockwise),
        MobileCanvasAction::Mirror => Some(PlacementTransform::ToggleHorizontalMirror),
        _ => None,
    }
}

pub(crate) fn execute_action(
    ctx: &Context,
    cx: &mut Cx<'_, DesignFeature>,
    request: MobileCanvasRequest,
) {
    if request.source != super::requests::editor_request_source(&canvas_facts_in(cx))
        || cx.wb.current_route().surface_id() != SurfaceId::Design
        || strip_canvas_of(cx.model.workspace.content.active_view_type()).is_none()
        || canvas_application_modal_open_in(cx)
        || !action_enabled(cx, request.action)
    {
        return;
    }
    let latches = mobile_controls::touch_latches(ctx);
    match request.action {
        MobileCanvasAction::SelectBox => cx.wb.prompt(if latches.select_box {
            "A drag on empty canvas draws a selection box."
        } else {
            "A drag on empty canvas pans."
        }),
        MobileCanvasAction::SelectAdd => cx.wb.prompt(if latches.select_add {
            "A tap adds to the selection or takes out of it."
        } else {
            "A tap selects one object."
        }),
        action => {
            // The placement ghost, or the selection Move carries, turns as
            // R and M turn it; the selection on the sheet turns otherwise.
            let floated = !symbol_in_front(cx)
                && floating_transform(action).is_some_and(|transform| {
                    super::shelf_drag::turn_what_floats(&mut canvas_parts_in(cx), transform)
                });
            if !floated && let Some(key) = product_command(action) {
                cx.execute(key);
            }
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;
