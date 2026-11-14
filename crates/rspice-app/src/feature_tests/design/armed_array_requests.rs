//! An armed array answers its canvas request through the whole application: the
//! request rechecks the source, the draft and the operation it was armed under, and one refused for a replaced
//! draft never cancels the replacement.

use crate::schematic::view::array_interaction::*;
use crate::schematic::view::requests::editor_request_source;
use crate::schematic::view::schematic_symbol_context;
use crate::state::Tool;
use crate::state::{ComponentType, Point};
use crate::workbench::app_state::AppState;
use rspice_schematic_editor::view::transform_input::TransformInputTransition;

#[test]
fn array_requests_recheck_the_source_draft_and_operation_without_cancelling_a_replacement() {
    for change in [
        "none",
        "document",
        "selection",
        "kind",
        "count",
        "naming",
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
        crate::workbench::app::open_array_selection_dialog_in(
            &mut state.design.array_selection,
            &mut state.design.move_selection,
            &mut state.design.stretch_selection,
            &mut state.model,
            &mut state.workbench,
        );
        state.design.array_selection.arm();
        state.model.schematic.arm_tool(Tool::ArraySelection);
        let expected = state.design.array_selection.canvas.clone();
        let mut next = expected.clone();
        next.preview_delta = Point::new(100, 0);
        let request = capture(
            &crate::schematic::view::canvas_parts(&mut state),
            TransformInputTransition {
                expected,
                next,
                commit: true,
            },
        );
        match change {
            "document" => state.model.active_schematic_epoch += 1,
            "selection" => state.model.schematic.session.editor.selection.clear(),
            "kind" => {
                state.design.array_selection.kind = crate::state::SchematicArrayKind::Rectangular
            }
            "count" => state.design.array_selection.count = "4".to_owned(),
            "naming" => state.design.array_selection.naming = "R20…R27".to_owned(),
            "draft" => state.design.array_selection.canvas.anchor = Some(Point::new(10, 0)),
            "modal" => state.settings.preferences_open = true,
            "read-only" => state.model.schematic.session.read_only = true,
            "tool" => state.model.schematic.arm_tool(Tool::Select),
            "replacement" => {
                crate::workbench::app::cancel_armed_array_selection_in(
                    &mut state.design.array_selection,
                    &mut state.model,
                );
                crate::workbench::app::open_array_selection_dialog_in(
                    &mut state.design.array_selection,
                    &mut state.design.move_selection,
                    &mut state.design.stretch_selection,
                    &mut state.model,
                    &mut state.workbench,
                );
                state.design.array_selection.arm();
                state.model.schematic.arm_tool(Tool::ArraySelection);
                assert_eq!(
                    request.transition.expected,
                    state.design.array_selection.canvas
                );
                assert_eq!(request.source, editor_request_source(&state));
                assert_ne!(request.generation, state.design.array_selection.generation);
            }
            "none" => {}
            _ => unreachable!(),
        }
        let canvas = state.design.array_selection.canvas.clone();
        let symbols = schematic_symbol_context(&state);
        apply(
            &mut crate::schematic::view::canvas_parts(&mut state),
            &symbols,
            request,
        );
        if change == "none" {
            assert_eq!(state.model.schematic.document().components.len(), 8);
            assert_eq!(
                state.model.schematic.undo_description(),
                Some("create array")
            );
        } else {
            assert_eq!(
                state.model.schematic.document().components.len(),
                1,
                "{change}"
            );
            assert!(!state.model.schematic.can_undo(), "{change}");
            assert_eq!(state.design.array_selection.canvas, canvas, "{change}");
            assert!(state.design.array_selection.armed, "{change}");
        }
    }
}
