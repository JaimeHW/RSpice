//! An armed move answers its canvas request through the whole application: the
//! request rechecks the source, the mode and the draft it was armed under, and one refused for a replaced
//! draft never cancels the replacement.

use crate::workbench::app_state::AppState;
use rspice_app_design::schematic::view::move_interaction::*;
use rspice_app_design::schematic::view::requests::editor_request_source;
use rspice_app_design::schematic::view::schematic_symbol_context;
use rspice_app_design::workbench::app::dialogs::selection::move_selection::cancel_armed_move_selection_in;
use rspice_app_state::state::ComponentType;
use rspice_app_state::state::Point;
use rspice_app_state::state::Tool;
use rspice_schematic_editor::view::transform_input::TransformInputTransition;

fn arm_test_move(state: &mut AppState, mode: rspice_app_state::state::MoveSelectionMode) {
    state.design.move_selection.mode = mode;
    rspice_app_design::workbench::app::dialogs::selection::move_selection::arm_move_selection_in(
        &mut state.design.move_selection,
        &mut state.design.stretch_selection,
        &mut state.design.array_selection,
        &mut state.model,
    );
    assert!(state.design.move_selection.armed);
}

#[test]
fn move_requests_recheck_source_mode_and_draft_without_cancelling_a_replacement() {
    for change in [
        "none",
        "document",
        "selection",
        "mode",
        "draft",
        "modal",
        "read-only",
        "tool",
        "replacement",
    ] {
        let mut state = AppState::default();
        let id = state
            .model
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(id);
        state.model.schematic.clear_undo_history();
        arm_test_move(
            &mut state,
            rspice_app_state::state::MoveSelectionMode::Connected,
        );
        let expected = state.design.move_selection.canvas.clone();
        let mut next = expected.clone();
        next.preview_delta = Point::new(100, 0);
        let request = capture(
            &state.canvas_parts(),
            TransformInputTransition {
                expected,
                next,
                commit: true,
            },
        );
        match change {
            "document" => state.model.active_schematic_epoch += 1,
            "selection" => state.model.schematic.session.editor.selection.clear(),
            "mode" => {
                state.design.move_selection.mode = rspice_app_state::state::MoveSelectionMode::Shove
            }
            "draft" => state.design.move_selection.canvas.anchor = Some(Point::new(10, 0)),
            "modal" => state.settings.preferences_open = true,
            "read-only" => state.model.schematic.session.read_only = true,
            "tool" => state.model.schematic.arm_tool(Tool::Select),
            "replacement" => {
                cancel_armed_move_selection_in(&mut state.design.move_selection, &mut state.model);
                arm_test_move(
                    &mut state,
                    rspice_app_state::state::MoveSelectionMode::Connected,
                );
                assert_eq!(
                    request.transition.expected,
                    state.design.move_selection.canvas
                );
                assert_eq!(request.source, editor_request_source(&state));
                assert_ne!(request.generation, state.design.move_selection.generation);
            }
            "none" => {}
            _ => unreachable!(),
        }
        let canvas = state.design.move_selection.canvas.clone();
        let symbols = schematic_symbol_context(&state);
        apply(&mut state.canvas_parts(), &symbols, request);
        if change == "none" {
            assert_eq!(
                state.model.schematic.document().components[0].pos,
                Point::new(100, 0)
            );
            assert_eq!(
                state.model.schematic.undo_description(),
                Some("move selection")
            );
        } else {
            assert_eq!(
                state.model.schematic.document().components[0].pos,
                Point::origin(),
                "{change}"
            );
            assert!(!state.model.schematic.can_undo(), "{change}");
            assert_eq!(state.design.move_selection.canvas, canvas, "{change}");
            assert!(state.design.move_selection.armed, "{change}");
        }
    }
}
