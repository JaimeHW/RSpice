//! App availability and command dispatch for the touch strip.
//!
//! The strip shows whenever the canvas is driven by touch, at any width
//! (audit G-12). Every command button runs the same `Command` the menus and
//! keys run; the two latches change what a one-finger gesture does.

use crate::state::Tool;
use crate::workbench::RSpiceApp;
use crate::workbench::SurfaceId;
use crate::workbench::commands::vocabulary::Command;
use egui::{Context, Rect};
use rspice_schematic_editor::view::mobile_controls::{
    self, MobileCanvasAction, MobileCanvasCapabilities, MobileCanvasRequest,
};
use rspice_schematic_editor::view::pointer_target::canvas_touch_mode;

use super::shelf_drag::PlacementTransform;

pub(crate) fn show(ctx: &Context, app: &mut RSpiceApp, content_rect: Rect) {
    if !canvas_touch_mode(ctx) {
        return;
    }
    let source = super::requests::editor_request_source(&app.state);
    let capabilities = MobileCanvasCapabilities {
        rotate: action_enabled(app, MobileCanvasAction::Rotate),
        mirror: action_enabled(app, MobileCanvasAction::Mirror),
        cancel: action_enabled(app, MobileCanvasAction::Cancel),
        undo: action_enabled(app, MobileCanvasAction::Undo),
        redo: action_enabled(app, MobileCanvasAction::Redo),
        fit: action_enabled(app, MobileCanvasAction::Fit),
    };
    if let Some(request) = mobile_controls::show(ctx, content_rect, &source, capabilities) {
        execute_action(ctx, app, request);
    }
}

const fn product_command(action: MobileCanvasAction) -> Option<Command> {
    match action {
        MobileCanvasAction::SelectBox | MobileCanvasAction::SelectAdd => None,
        MobileCanvasAction::Rotate => Some(Command::RotateSelection),
        MobileCanvasAction::Mirror => Some(Command::MirrorSelectionHorizontal),
        MobileCanvasAction::Cancel => Some(Command::Cancel),
        MobileCanvasAction::Undo => Some(Command::Undo),
        MobileCanvasAction::Redo => Some(Command::Redo),
        MobileCanvasAction::Fit => Some(Command::ZoomFit),
    }
}

/// Whether a route, a move, a placement or a box is live: what Esc would
/// cancel on the canvas.
fn operation_live(app: &RSpiceApp) -> bool {
    let state = &app.state;
    let editor = &state.schematic.session.editor;
    state.schematic.canvas_settings_change_blocked()
        || editor.tool != Tool::Select
        || editor.zoom_box.armed
        || state.dialogs.move_selection.armed
        || state.dialogs.stretch_selection.armed
        || state.dialogs.array_selection.armed
        || state.schematic_drag_in_progress()
}

fn action_enabled(app: &RSpiceApp, action: MobileCanvasAction) -> bool {
    match action {
        MobileCanvasAction::SelectBox | MobileCanvasAction::SelectAdd => true,
        MobileCanvasAction::Cancel => operation_live(app),
        MobileCanvasAction::Rotate | MobileCanvasAction::Mirror
            if super::shelf_drag::something_floats(&app.state) =>
        {
            true
        }
        _ => product_command(action).is_some_and(|command| command.is_enabled(app)),
    }
}

/// What R or M would turn while something floats under the finger.
const fn floating_transform(action: MobileCanvasAction) -> Option<PlacementTransform> {
    match action {
        MobileCanvasAction::Rotate => Some(PlacementTransform::RotateClockwise),
        MobileCanvasAction::Mirror => Some(PlacementTransform::ToggleHorizontalMirror),
        _ => None,
    }
}

fn execute_action(ctx: &Context, app: &mut RSpiceApp, request: MobileCanvasRequest) {
    if request.source != super::requests::editor_request_source(&app.state)
        || app.state.workbench.current_route().surface_id() != SurfaceId::Design
        || !matches!(
            app.state.workspace.content.active_view_type(),
            crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
        )
        || app.state.application_modal_open()
        || !action_enabled(app, request.action)
    {
        return;
    }
    let latches = mobile_controls::touch_latches(ctx);
    match request.action {
        MobileCanvasAction::SelectBox => app.state.prompt(if latches.select_box {
            "A drag on empty canvas draws a selection box."
        } else {
            "A drag on empty canvas pans."
        }),
        MobileCanvasAction::SelectAdd => app.state.prompt(if latches.select_add {
            "A tap adds to the selection or takes out of it."
        } else {
            "A tap selects one object."
        }),
        action => {
            // The placement ghost, or the selection Move carries, turns as
            // R and M turn it; the selection on the sheet turns otherwise.
            let floated = floating_transform(action).is_some_and(|transform| {
                super::shelf_drag::turn_what_floats(&mut app.state, transform)
            });
            if !floated && let Some(command) = product_command(action) {
                command.execute(app);
            }
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;
