//! An armed stretch answers its canvas request through the whole application: the
//! request rechecks the source, the target and the policy it was armed under, and one refused for a replaced
//! draft never cancels the replacement.

use super::app_state_of;
use crate::schematic::view::requests::editor_request_source;
use crate::schematic::view::schematic_symbol_context;
use crate::schematic::view::stretch_interaction::*;
use crate::workbench::app::cancel_armed_stretch_selection_in;
use crate::workbench::app_state::AppState;
use rspice_app_state::state::Point;
use rspice_app_state::state::Tool;
use rspice_schematic_editor::view::transform_input::TransformInputTransition;

/// [`selected_stretch_wire`](crate::schematic::view::stretch_interaction::tests::selected_stretch_wire) made a whole application.
fn selected_stretch_wire() -> AppState {
    app_state_of(crate::schematic::view::stretch_interaction::tests::selected_stretch_wire())
}

fn arm_test_stretch(state: &mut AppState) {
    crate::workbench::app::arm_stretch_selection_in(
        &mut state.design.stretch_selection,
        &mut state.design.move_selection,
        &mut state.design.array_selection,
        state.design.schematic_visibility.wire_routing,
        &mut state.model,
        &mut state.workbench,
    );
    assert!(state.design.stretch_selection.armed);
}

#[test]
fn stretch_requests_recheck_source_target_and_policy_without_cancelling_a_replacement() {
    for change in [
        "none",
        "document",
        "selection",
        "target",
        "policy",
        "draft",
        "modal",
        "read-only",
        "tool",
        "replacement",
    ] {
        let mut state = selected_stretch_wire();
        arm_test_stretch(&mut state);
        let expected = state.design.stretch_selection.canvas.clone();
        let mut next = expected.clone();
        next.gesture.preview_delta = Point::new(0, 10);
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
            "target" => state.design.stretch_selection.canvas.target = None,
            "policy" => {
                state.design.stretch_selection.policy =
                    rspice_app_state::state::StretchOrthogonalPolicy::AllowDiagonal
            }
            "draft" => {
                state.design.stretch_selection.canvas.gesture.anchor = Some(Point::new(10, 0))
            }
            "modal" => state.settings.preferences_open = true,
            "read-only" => state.model.schematic.session.read_only = true,
            "tool" => state.model.schematic.arm_tool(Tool::Select),
            "replacement" => {
                cancel_armed_stretch_selection_in(
                    &mut state.design.stretch_selection,
                    &mut state.model,
                );
                arm_test_stretch(&mut state);
                assert_eq!(
                    request.transition.expected,
                    state.design.stretch_selection.canvas
                );
                assert_eq!(request.source, editor_request_source(&state));
                assert_ne!(
                    request.generation,
                    state.design.stretch_selection.generation
                );
            }
            "none" => {}
            _ => unreachable!(),
        }
        let canvas = state.design.stretch_selection.canvas.clone();
        let symbols = schematic_symbol_context(&state);
        apply(&mut state.canvas_parts(), &symbols, request);
        if change == "none" {
            assert_eq!(
                state.model.schematic.document().wires[0].points[1],
                Point::new(0, 30)
            );
            assert_eq!(
                state.model.schematic.undo_description(),
                Some("stretch selection")
            );
        } else {
            assert_eq!(
                state.model.schematic.document().wires[0].points[1],
                Point::new(0, 20),
                "{change}"
            );
            assert!(!state.model.schematic.can_undo(), "{change}");
            assert_eq!(state.design.stretch_selection.canvas, canvas, "{change}");
            assert!(state.design.stretch_selection.armed, "{change}");
        }
    }
}
