//! Validate stretch canvas requests before the app-owned atomic commit.

use super::{SchematicSymbolContext, requests::editor_request_source, viewport::Viewport};
use crate::diagnostics::ConsoleMessage;
use crate::state::{Point, Tool};
use crate::workbench::{
    app::{armed_stretch_selection_authority, cancel_armed_stretch_selection},
    app_state::AppState,
};
use egui::{Response, Ui};
use rspice_schematic_editor::{
    session::stretch::StretchCanvasSession,
    view::{
        stretch_interaction::{self, StretchInputRequest, StretchInputView},
        transform_input::TransformInputTransition,
    },
};

pub(super) fn handle_armed_stretch_selection(
    ui: &Ui,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
) {
    if state.application_modal_open()
        || state.schematic.session.editor.tool != Tool::StretchSelection
        || !state.dialogs.stretch_selection.armed
    {
        return;
    }
    if let Err(message) = armed_stretch_selection_authority(state) {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Stretch selection cancelled: {message}"
        )));
        cancel_armed_stretch_selection(state);
        return;
    }
    if let Some(transition) = stretch_interaction::input(
        ui,
        response,
        viewport,
        StretchInputView {
            design: super::schematic_design_view(state),
            selection: &state.schematic.session.editor.selection,
            canvas: &state.dialogs.stretch_selection.canvas,
            policy: state.dialogs.stretch_selection.policy,
            snap_engine: &state.schematic.session.editor.snap_engine,
        },
    ) {
        let request = capture(state, transition);
        apply(state, symbol_context, request);
    }
}

fn capture(
    state: &AppState,
    transition: TransformInputTransition<StretchCanvasSession>,
) -> StretchInputRequest {
    StretchInputRequest {
        source: editor_request_source(state),
        selection: state.schematic.session.editor.selection.clone(),
        generation: state.dialogs.stretch_selection.generation,
        policy: state.dialogs.stretch_selection.policy,
        transition,
    }
}

fn apply(state: &mut AppState, symbols: &SchematicSymbolContext, request: StretchInputRequest) {
    let draft = &state.dialogs.stretch_selection;
    if request.source != editor_request_source(state)
        || request.selection != state.schematic.session.editor.selection
        || request.generation != draft.generation
        || request.policy != draft.policy
        || request.transition.expected != draft.canvas
        || state.schematic.session.editor.tool != Tool::StretchSelection
        || !draft.armed
        || state.application_modal_open()
        || armed_stretch_selection_authority(state).is_err()
    {
        return;
    }
    state.dialogs.stretch_selection.canvas = request.transition.next;
    if request.transition.commit {
        commit_armed_stretch_selection(state, symbols);
    }
}

fn commit_armed_stretch_selection(state: &mut AppState, symbol_context: &SchematicSymbolContext) {
    if let Err(message) = armed_stretch_selection_authority(state) {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Stretch selection cancelled: {message}"
        )));
        cancel_armed_stretch_selection(state);
        return;
    }
    if let Some(message) = state
        .dialogs
        .stretch_selection
        .canvas
        .gesture
        .preview_error
        .clone()
    {
        let draft = &mut state.dialogs.stretch_selection;
        draft.canvas.gesture.reset_candidate();
        state.push_user_message(ConsoleMessage::warning(format!(
            "Stretch selection was not committed: {message}"
        )));
        return;
    }
    let delta = state.dialogs.stretch_selection.canvas.gesture.preview_delta;
    let policy = state.dialogs.stretch_selection.policy;
    let target = state
        .dialogs
        .stretch_selection
        .canvas
        .target
        .expect("validated stretch target");
    if delta == Point::origin() {
        state.push_user_message(ConsoleMessage::info(
            "Stretch selection finished without changing geometry; no undo record was created."
                .to_owned(),
        ));
        cancel_armed_stretch_selection(state);
        return;
    }

    state.schematic.begin_operation("stretch selection");
    match state.schematic.stretch_target_resolved(
        delta,
        target,
        policy,
        |component| symbol_context.terminal_points(component),
        |component| symbol_context.component_bounds_tuple(component),
    ) {
        Ok(true) => {
            let recorded = state.schematic.end_operation();
            state.sync_active_schematic_to_workspace();
            state.push_user_message(ConsoleMessage::info(format!(
                "Stretched the selected geometry by ({}, {}) with {}; {}.",
                delta.x,
                delta.y,
                policy.label(),
                if recorded {
                    "one undo record committed"
                } else {
                    "geometry was unchanged"
                }
            )));
            cancel_armed_stretch_selection(state);
        }
        Ok(false) => {
            state.schematic.cancel_operation();
            state.push_user_message(ConsoleMessage::info(
                "Stretch selection produced no geometry change; no undo record was created."
                    .to_owned(),
            ));
            cancel_armed_stretch_selection(state);
        }
        Err(error) => {
            state.schematic.cancel_operation();
            let draft = &mut state.dialogs.stretch_selection;
            draft.canvas.gesture.preview_error = Some(error.to_string());
            draft.canvas.gesture.reset_candidate();
            state.push_user_message(ConsoleMessage::warning(format!(
                "Stretch selection was not committed: {error}"
            )));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schematic::view::schematic_symbol_context;
    use crate::state::{Junction, Wire};

    fn selected_stretch_wire() -> AppState {
        let mut state = AppState::default();
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::new(
                7,
                vec![
                    Point::new(0, 0),
                    Point::new(0, 20),
                    Point::new(20, 20),
                    Point::new(20, 0),
                ],
            ));
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_wire_segment(7, 1);
        state.schematic.init_undo_history();
        state
    }

    fn arm_test_stretch(state: &mut AppState) {
        crate::workbench::app::open_stretch_selection_dialog(state);
        assert!(state.dialogs.stretch_selection.open);
        state.dialogs.stretch_selection.arm();
        state
            .schematic
            .arm_tool(crate::state::Tool::StretchSelection);
    }

    #[test]
    fn commit_records_once_syncs_workspace_and_retains_selection() {
        let mut state = selected_stretch_wire();
        arm_test_stretch(&mut state);
        state.dialogs.stretch_selection.canvas.gesture.preview_delta = Point::origin();
        let symbols = schematic_symbol_context(&state);

        let ctx = egui::Context::default();
        for commit in [false, true] {
            let events = if commit {
                [
                    egui::Key::ArrowRight,
                    egui::Key::ArrowDown,
                    egui::Key::Enter,
                ]
                .into_iter()
                .map(|key| egui::Event::Key {
                    key,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                })
                .collect()
            } else {
                Vec::new()
            };
            let _ = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(600.0, 400.0),
                    )),
                    events,
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let bounds = ui.max_rect();
                        let response = ui.interact(
                            bounds,
                            egui::Id::new("stretch-canvas"),
                            // accessibility-pointer-shim: synthetic input for the stretch handler test.
                            egui::Sense::click_and_drag(),
                        );
                        if !commit {
                            response.request_focus();
                        }
                        let viewport = Viewport {
                            offset: egui::Pos2::ZERO,
                            zoom: 1.0,
                            bounds,
                        };
                        handle_armed_stretch_selection(
                            ui, &response, &mut state, &viewport, &symbols,
                        );
                    });
                },
            );
        }

        assert_eq!(
            state.schematic.document().wires[0].points[1],
            Point::new(0, 30)
        );
        assert_eq!(
            state.schematic.document().wires[0].points[2],
            Point::new(20, 30)
        );
        assert_eq!(
            state.schematic.undo_description(),
            Some("stretch selection")
        );
        assert!(
            state
                .schematic
                .session
                .editor
                .selection
                .has_wire_segment(7, 1)
        );
        assert_eq!(
            state.schematic.session.editor.tool,
            crate::state::Tool::Select
        );
        assert!(!state.dialogs.stretch_selection.armed);
        assert_eq!(
            state
                .workspace
                .active_schematic()
                .expect("active workspace buffer")
                .document()
                .wires[0]
                .points[1],
            Point::new(0, 30)
        );
        assert!(state.schematic.undo());
        assert_eq!(
            state.schematic.document().wires[0].points[1],
            Point::new(0, 20)
        );
        assert!(
            !state.schematic.can_undo(),
            "the gesture owns one undo record"
        );
    }

    #[test]
    fn cancel_preserves_geometry_selection_and_history() {
        let mut state = selected_stretch_wire();
        let baseline = state.schematic.document().wires[0].clone();
        arm_test_stretch(&mut state);
        state.dialogs.stretch_selection.canvas.gesture.preview_delta = Point::new(0, 10);

        crate::workbench::app::cancel_armed_stretch_selection(&mut state);

        assert_eq!(state.schematic.document().wires[0], baseline);
        assert!(
            state
                .schematic
                .session
                .editor
                .selection
                .has_wire_segment(7, 1)
        );
        assert_eq!(
            state.schematic.session.editor.tool,
            crate::state::Tool::Select
        );
        assert!(!state.schematic.can_undo());
    }

    #[test]
    fn rejected_commit_preserves_authority_selection_and_undo_history() {
        let mut state = selected_stretch_wire();
        state
            .schematic
            .document_mut_for_test()
            .junctions
            .push(Junction::new(1, Point::new(10, 20)));
        let baseline = state.schematic.document().wires[0].clone();
        arm_test_stretch(&mut state);
        state.dialogs.stretch_selection.canvas.gesture.preview_delta = Point::new(0, 10);
        let symbols = schematic_symbol_context(&state);

        commit_armed_stretch_selection(&mut state, &symbols);

        assert_eq!(state.schematic.document().wires[0], baseline);
        assert!(
            state
                .schematic
                .session
                .editor
                .selection
                .has_wire_segment(7, 1)
        );
        assert_eq!(
            state.schematic.session.editor.tool,
            crate::state::Tool::StretchSelection
        );
        assert!(state.dialogs.stretch_selection.armed);
        assert!(
            state
                .dialogs
                .stretch_selection
                .canvas
                .gesture
                .preview_error
                .is_some()
        );
        assert!(!state.schematic.can_undo());
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
            let expected = state.dialogs.stretch_selection.canvas.clone();
            let mut next = expected.clone();
            next.gesture.preview_delta = Point::new(0, 10);
            let request = capture(
                &state,
                TransformInputTransition {
                    expected,
                    next,
                    commit: true,
                },
            );
            match change {
                "document" => state.active_schematic_epoch += 1,
                "selection" => state.schematic.session.editor.selection.clear(),
                "target" => state.dialogs.stretch_selection.canvas.target = None,
                "policy" => {
                    state.dialogs.stretch_selection.policy =
                        crate::state::StretchOrthogonalPolicy::AllowDiagonal
                }
                "draft" => {
                    state.dialogs.stretch_selection.canvas.gesture.anchor = Some(Point::new(10, 0))
                }
                "modal" => state.dialogs.preferences_open = true,
                "read-only" => state.schematic.session.read_only = true,
                "tool" => state.schematic.arm_tool(Tool::Select),
                "replacement" => {
                    cancel_armed_stretch_selection(&mut state);
                    arm_test_stretch(&mut state);
                    assert_eq!(
                        request.transition.expected,
                        state.dialogs.stretch_selection.canvas
                    );
                    assert_eq!(request.source, editor_request_source(&state));
                    assert_ne!(
                        request.generation,
                        state.dialogs.stretch_selection.generation
                    );
                }
                "none" => {}
                _ => unreachable!(),
            }
            let canvas = state.dialogs.stretch_selection.canvas.clone();
            let symbols = schematic_symbol_context(&state);
            apply(&mut state, &symbols, request);
            if change == "none" {
                assert_eq!(
                    state.schematic.document().wires[0].points[1],
                    Point::new(0, 30)
                );
                assert_eq!(
                    state.schematic.undo_description(),
                    Some("stretch selection")
                );
            } else {
                assert_eq!(
                    state.schematic.document().wires[0].points[1],
                    Point::new(0, 20),
                    "{change}"
                );
                assert!(!state.schematic.can_undo(), "{change}");
                assert_eq!(state.dialogs.stretch_selection.canvas, canvas, "{change}");
                assert!(state.dialogs.stretch_selection.armed, "{change}");
            }
        }
    }
}
