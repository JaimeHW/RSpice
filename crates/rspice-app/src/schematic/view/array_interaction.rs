//! Exclusive canvas interaction for the transactional Create Array command.

use egui::{Response, Ui};

use crate::diagnostics::ConsoleMessage;
use crate::state::{Point, SchematicArrayKind, SchematicArrayPlacement};
use crate::workbench::app_state::AppState;

use super::SchematicSymbolContext;
use super::snap_resolution::resolve_grid_pointer;
use super::viewport::Viewport;

const DELTA_OVERFLOW: &str =
    "The requested array placement exceeds the schematic coordinate range.";

pub(super) fn handle_armed_array_selection(
    ui: &Ui,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    grid_size: i32,
    symbol_context: &SchematicSymbolContext,
) {
    if let Err(message) = crate::workbench::app::armed_array_selection_authority(state) {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Create array cancelled: {message}"
        )));
        crate::workbench::app::cancel_armed_array_selection(state);
        return;
    }

    retain_canvas_focus_from_pointer(response);
    let (keyboard_step, keyboard_commit) = consume_keyboard(ui, response.has_focus(), grid_size);
    if keyboard_step != Point::origin() {
        let requested =
            checked_accumulate_delta(state.dialogs.array_selection.preview_delta, keyboard_step);
        let draft = &mut state.dialogs.array_selection;
        draft.pointer_drag = false;
        if draft.kind == SchematicArrayKind::RadialDocumentation && draft.anchor.is_none() {
            draft.anchor = Some(Point::origin());
        }
        update_preview_delta(state, requested);
    }
    if keyboard_commit {
        commit_armed_array_selection(state, symbol_context);
        return;
    }

    if response.drag_started_by(egui::PointerButton::Primary)
        && let Some(position) = ui
            .input(|input| input.pointer.press_origin())
            .or_else(|| response.interact_pointer_pos())
    {
        let point = resolve_grid_pointer(state, viewport, position).snapped_position;
        let draft = &mut state.dialogs.array_selection;
        draft.anchor = Some(point);
        draft.preview_delta = Point::origin();
        draft.pointer_drag = true;
        draft.preview_error = None;
    }

    if response.dragged_by(egui::PointerButton::Primary)
        && state.dialogs.array_selection.pointer_drag
        && let (Some(anchor), Some(position)) = (
            state.dialogs.array_selection.anchor,
            response
                .hover_pos()
                .or_else(|| response.interact_pointer_pos()),
        )
    {
        let destination = resolve_grid_pointer(state, viewport, position).snapped_position;
        if state.dialogs.array_selection.kind == SchematicArrayKind::RadialDocumentation {
            state.dialogs.array_selection.anchor = Some(destination);
            update_preview_delta(state, Ok(Point::origin()));
        } else {
            update_preview_delta(state, checked_pointer_delta(anchor, destination));
        }
    }

    if response.drag_stopped_by(egui::PointerButton::Primary)
        && state.dialogs.array_selection.pointer_drag
    {
        state.dialogs.array_selection.pointer_drag = false;
        commit_armed_array_selection(state, symbol_context);
        return;
    }

    if response.clicked_by(egui::PointerButton::Primary)
        && let Some(position) = response.interact_pointer_pos()
    {
        let point = resolve_grid_pointer(state, viewport, position).snapped_position;
        if state.dialogs.array_selection.kind == SchematicArrayKind::RadialDocumentation {
            let draft = &mut state.dialogs.array_selection;
            draft.anchor = Some(point);
            draft.preview_delta = Point::origin();
            update_preview_delta(state, Ok(Point::origin()));
            commit_armed_array_selection(state, symbol_context);
        } else if let Some(anchor) = state.dialogs.array_selection.anchor {
            update_preview_delta(state, checked_pointer_delta(anchor, point));
            commit_armed_array_selection(state, symbol_context);
        } else {
            let draft = &mut state.dialogs.array_selection;
            draft.anchor = Some(point);
            draft.preview_delta = Point::origin();
            draft.preview_error = None;
        }
    } else if !state.dialogs.array_selection.pointer_drag
        && let Some(position) = response.hover_pos()
    {
        let point = resolve_grid_pointer(state, viewport, position).snapped_position;
        if state.dialogs.array_selection.kind == SchematicArrayKind::RadialDocumentation {
            state.dialogs.array_selection.anchor = Some(point);
            update_preview_delta(state, Ok(Point::origin()));
        } else if let Some(anchor) = state.dialogs.array_selection.anchor {
            update_preview_delta(state, checked_pointer_delta(anchor, point));
        }
    }
}

fn checked_accumulate_delta(current: Point, step: Point) -> Result<Point, &'static str> {
    let Some(x) = current.x.checked_add(step.x) else {
        return Err(DELTA_OVERFLOW);
    };
    let Some(y) = current.y.checked_add(step.y) else {
        return Err(DELTA_OVERFLOW);
    };
    Ok(Point::new(x, y))
}

fn checked_pointer_delta(anchor: Point, destination: Point) -> Result<Point, &'static str> {
    let Some(x) = destination.x.checked_sub(anchor.x) else {
        return Err(DELTA_OVERFLOW);
    };
    let Some(y) = destination.y.checked_sub(anchor.y) else {
        return Err(DELTA_OVERFLOW);
    };
    Ok(Point::new(x, y))
}

pub(super) fn array_placement(state: &AppState) -> Result<SchematicArrayPlacement, &'static str> {
    let draft = &state.dialogs.array_selection;
    match draft.kind {
        SchematicArrayKind::RadialDocumentation => {
            let base = draft.anchor.unwrap_or_else(Point::origin);
            let Some(x) = base.x.checked_add(draft.preview_delta.x) else {
                return Err(DELTA_OVERFLOW);
            };
            let Some(y) = base.y.checked_add(draft.preview_delta.y) else {
                return Err(DELTA_OVERFLOW);
            };
            Ok(SchematicArrayPlacement::Center(Point::new(x, y)))
        }
        SchematicArrayKind::Linear | SchematicArrayKind::Rectangular => {
            Ok(SchematicArrayPlacement::Pitch(draft.preview_delta))
        }
    }
}

fn update_preview_delta(state: &mut AppState, requested: Result<Point, &'static str>) {
    let requested = match requested {
        Ok(requested) => requested,
        Err(message) => {
            let draft = &mut state.dialogs.array_selection;
            draft.preview_delta = Point::origin();
            draft.preview_error = Some(message.to_owned());
            return;
        }
    };
    state.dialogs.array_selection.preview_delta = requested;
    // The painter keys its retained immutable preview by the complete plan.
    // Commit independently rebuilds the exact final candidate, so a prior
    // plan's cached error can never reject a newly valid pointer position.
    state.dialogs.array_selection.preview_error = None;
}

fn retain_canvas_focus_from_pointer(response: &Response) {
    if response.clicked_by(egui::PointerButton::Primary)
        || response.drag_started_by(egui::PointerButton::Primary)
    {
        response.request_focus();
    }
}

fn consume_keyboard(ui: &Ui, canvas_has_focus: bool, grid_size: i32) -> (Point, bool) {
    if !canvas_has_focus {
        return (Point::origin(), false);
    }
    ui.input_mut(|input| {
        let mut step = Point::origin();
        if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft) {
            step.x = -grid_size;
        }
        if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight) {
            step.x = grid_size;
        }
        if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) {
            step.y = -grid_size;
        }
        if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) {
            step.y = grid_size;
        }
        let commit = input.consume_key(egui::Modifiers::NONE, egui::Key::Enter);
        (step, commit)
    })
}

fn commit_armed_array_selection(state: &mut AppState, symbol_context: &SchematicSymbolContext) {
    if let Err(message) = crate::workbench::app::armed_array_selection_authority(state) {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Create array cancelled: {message}"
        )));
        crate::workbench::app::cancel_armed_array_selection(state);
        return;
    }
    let plan = match array_placement(state)
        .map_err(str::to_owned)
        .and_then(|placement| crate::workbench::app::armed_array_selection_plan(state, placement))
    {
        Ok(plan) => plan,
        Err(message) => {
            reset_failed_candidate(state);
            state.push_user_message(ConsoleMessage::warning(format!(
                "Create array was not committed: {message}"
            )));
            return;
        }
    };

    match state.schematic.array_selection_resolved(
        &plan,
        |component| symbol_context.named_terminal_points(component),
        |component| symbol_context.component_bounds_tuple(component),
    ) {
        Ok(impact) => {
            state.sync_active_schematic_to_workspace();
            state.push_user_message(ConsoleMessage::info(format!(
                "Created {} new array members; one undo record committed.",
                impact.replicas,
            )));
            crate::workbench::app::cancel_armed_array_selection(state);
        }
        Err(error) => {
            state.dialogs.array_selection.preview_error = Some(error.to_string());
            reset_failed_candidate(state);
            state.push_user_message(ConsoleMessage::warning(format!(
                "Create array was not committed: {error}"
            )));
        }
    }
}

fn reset_failed_candidate(state: &mut AppState) {
    let draft = &mut state.dialogs.array_selection;
    draft.anchor = None;
    draft.preview_delta = Point::origin();
    draft.pointer_drag = false;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Component, ComponentType, Tool};

    #[test]
    fn pointer_and_keyboard_delta_arithmetic_rejects_overflow() {
        assert_eq!(
            checked_accumulate_delta(Point::new(i32::MAX, 0), Point::new(1, 0)),
            Err(DELTA_OVERFLOW)
        );
        assert_eq!(
            checked_pointer_delta(Point::new(i32::MIN, 0), Point::new(i32::MAX, 0)),
            Err(DELTA_OVERFLOW)
        );
    }

    #[test]
    fn armed_array_commit_creates_one_transaction_and_returns_to_select() {
        let mut state = AppState::default();
        state.schematic.components.push(Component::new(
            1,
            ComponentType::Resistor,
            Point::origin(),
        ));
        state.schematic.selection.select_only_component(1);
        state.schematic.recalculate_runtime_state();
        state.schematic.clear_undo_history();
        crate::workbench::app::open_array_selection_dialog(&mut state);
        state.dialogs.array_selection.arm();
        state.dialogs.array_selection.preview_delta = Point::new(100, 0);
        state.schematic.arm_tool(Tool::ArraySelection);
        let symbols = SchematicSymbolContext::from_state(&state);

        commit_armed_array_selection(&mut state, &symbols);

        assert_eq!(state.schematic.components.len(), 8);
        assert_eq!(state.schematic.undo_description(), Some("create array"));
        assert_eq!(state.schematic.tool, Tool::Select);
        assert!(!state.dialogs.array_selection.armed);
        assert!(state.schematic.undo());
        assert_eq!(state.schematic.components.len(), 1);
        assert!(!state.schematic.can_undo());
    }

    #[test]
    fn rejected_array_candidate_stays_armed_without_mutating_the_document() {
        let mut state = AppState::default();
        state.schematic.components.push(Component::new(
            1,
            ComponentType::Resistor,
            Point::origin(),
        ));
        state.schematic.selection.select_only_component(1);
        state.schematic.recalculate_runtime_state();
        state.schematic.clear_undo_history();
        crate::workbench::app::open_array_selection_dialog(&mut state);
        state.dialogs.array_selection.arm();
        state.dialogs.array_selection.preview_delta = Point::new(1, 0);
        state.schematic.arm_tool(Tool::ArraySelection);
        let symbols = SchematicSymbolContext::from_state(&state);

        commit_armed_array_selection(&mut state, &symbols);

        assert_eq!(state.schematic.components.len(), 1);
        assert!(state.dialogs.array_selection.armed);
        assert_eq!(state.schematic.tool, Tool::ArraySelection);
        assert!(state.dialogs.array_selection.preview_error.is_some());
        assert!(!state.schematic.can_undo());
    }
}
