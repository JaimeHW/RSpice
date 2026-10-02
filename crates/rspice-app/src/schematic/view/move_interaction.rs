//! Validate move canvas requests and retain the app-owned undo transaction.

use super::{SchematicSymbolContext, requests::editor_request_source, viewport::Viewport};
use crate::diagnostics::ConsoleMessage;
use crate::state::{Point, Tool};
use crate::workbench::{
    app::{armed_move_selection_authority, cancel_armed_move_selection},
    app_state::AppState,
};
use egui::{Response, Ui};
use rspice_schematic_editor::view::{
    move_interaction::{self, MoveInputRequest, MoveInputView},
    transform_input::TransformInputTransition,
};

pub(super) fn handle_armed_move_selection(
    ui: &Ui,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
) {
    if state.application_modal_open()
        || state.schematic.session.editor.tool != Tool::MoveSelection
        || !state.dialogs.move_selection.armed
    {
        return;
    }
    if let Err(message) = armed_move_selection_authority(state) {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Move selection cancelled: {message}"
        )));
        cancel_armed_move_selection(state);
        return;
    }
    if let Some(transition) = move_interaction::input(
        ui,
        response,
        viewport,
        symbol_context,
        MoveInputView {
            design: super::schematic_design_view(state),
            canvas: &state.dialogs.move_selection.canvas,
            selection: &state.schematic.session.editor.selection,
            filter: state.ui.schematic_selection_filter,
            snap_engine: &state.schematic.session.editor.snap_engine,
        },
        || state.workspace.content.active_view.display_path(),
    ) {
        let request = capture(state, transition);
        apply(state, symbol_context, request);
    }
}

fn capture(state: &AppState, transition: TransformInputTransition) -> MoveInputRequest {
    MoveInputRequest {
        source: editor_request_source(state),
        selection: state.schematic.session.editor.selection.clone(),
        generation: state.dialogs.move_selection.generation,
        mode: state.dialogs.move_selection.mode,
        transition,
    }
}

fn apply(state: &mut AppState, symbols: &SchematicSymbolContext, request: MoveInputRequest) {
    let draft = &state.dialogs.move_selection;
    if request.source != editor_request_source(state)
        || request.selection != state.schematic.session.editor.selection
        || request.generation != draft.generation
        || request.mode != draft.mode
        || request.transition.expected != draft.canvas
        || state.schematic.session.editor.tool != Tool::MoveSelection
        || !draft.armed
        || state.application_modal_open()
        || armed_move_selection_authority(state).is_err()
    {
        return;
    }
    state.dialogs.move_selection.canvas = request.transition.next;
    if request.transition.commit {
        commit_armed_move_selection(state, symbols);
    }
}

fn commit_armed_move_selection(state: &mut AppState, symbol_context: &SchematicSymbolContext) {
    if let Err(message) = armed_move_selection_authority(state) {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Move selection cancelled: {message}"
        )));
        cancel_armed_move_selection(state);
        return;
    }
    let delta = state.dialogs.move_selection.canvas.preview_delta;
    let mode = state.dialogs.move_selection.mode;
    if delta == Point::origin() {
        state.push_user_message(ConsoleMessage::info(
            "Move selection finished without changing geometry; no undo record was created."
                .to_owned(),
        ));
        cancel_armed_move_selection(state);
        return;
    }
    state.schematic.begin_operation("move selection");
    let movement = state
        .schematic
        .move_selection_with_mode_resolved(delta, mode, |component| {
            symbol_context.terminal_points(component)
        });
    match movement {
        Ok(true) => {
            let automatic_junctions = state
                .schematic
                .document()
                .document_policy
                .wire_junctions
                .automatic_junctions();
            state
                .schematic
                .cleanup_wire_topology_with_junction_policy(automatic_junctions);
            let recorded = state.schematic.end_operation();
            state.sync_active_schematic_to_workspace();
            state.push_user_message(ConsoleMessage::info(format!(
                "Moved {} selected objects by ({}, {}) in {} mode; {}.",
                state.schematic.live_movable_selection_count(),
                delta.x,
                delta.y,
                mode.label(),
                if recorded {
                    "one undo record committed"
                } else {
                    "geometry was unchanged"
                }
            )));
            cancel_armed_move_selection(state);
        }
        Ok(false) => {
            state.schematic.cancel_operation();
            state.push_user_message(ConsoleMessage::info(
                "Move selection produced no geometry change; no undo record was created."
                    .to_owned(),
            ));
            cancel_armed_move_selection(state);
        }
        Err(error) => {
            state.schematic.cancel_operation();
            state.dialogs.move_selection.canvas.preview_error = Some(error.to_string());
            state.dialogs.move_selection.canvas.anchor = None;
            state.dialogs.move_selection.canvas.preview_delta = Point::origin();
            state.push_user_message(ConsoleMessage::warning(format!(
                "Move selection was not committed: {error}"
            )));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schematic::view::schematic_symbol_context;
    use crate::state::{Component, ComponentType, Wire};
    fn arm_test_move(state: &mut AppState, mode: crate::state::MoveSelectionMode) {
        crate::workbench::app::open_move_selection_dialog(state);
        state.dialogs.move_selection.mode = mode;
        state.dialogs.move_selection.arm();
        state.schematic.arm_tool(Tool::MoveSelection);
    }
    #[test]
    fn armed_move_commits_once_syncs_workspace_and_retains_selection() {
        let mut state = AppState::default();
        state
            .schematic
            .document_mut_for_test()
            .components
            .push(Component::new(1, ComponentType::Resistor, Point::origin()));
        let terminal = state.schematic.document().components[0].terminal_positions()[0].1;
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::segment(2, terminal, Point::new(20, 0)));
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(1);
        state.schematic.init_undo_history();
        arm_test_move(&mut state, crate::state::MoveSelectionMode::Connected);
        state.dialogs.move_selection.canvas.preview_delta = Point::origin();
        let symbols = schematic_symbol_context(&state);

        let ctx = egui::Context::default();
        for commit in [false, true] {
            let events = if commit {
                [egui::Key::ArrowDown, egui::Key::Enter]
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
                            egui::Id::new("move-canvas"),
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
                        handle_armed_move_selection(ui, &response, &mut state, &viewport, &symbols);
                    });
                },
            );
        }

        assert_eq!(
            state.schematic.document().components[0].pos,
            Point::new(0, 10)
        );
        assert_eq!(
            state.schematic.document().wires[0].points[0],
            Point::new(terminal.x, terminal.y + 10)
        );
        assert_eq!(state.schematic.undo_description(), Some("move selection"));
        assert!(state.schematic.session.editor.selection.has_component(1));
        assert_eq!(state.schematic.session.editor.tool, Tool::Select);
        assert!(!state.dialogs.move_selection.armed);
        assert_eq!(
            state
                .workspace
                .active_schematic()
                .expect("active workspace buffer")
                .document()
                .components[0]
                .pos,
            Point::new(0, 10)
        );
        assert!(state.schematic.undo());
        assert_eq!(
            state.schematic.document().components[0].pos,
            Point::origin()
        );
        assert!(
            !state.schematic.can_undo(),
            "the gesture owns one undo record"
        );
    }
    #[test]
    fn cancelling_armed_move_preserves_geometry_selection_and_history() {
        let mut state = AppState::default();
        state
            .schematic
            .document_mut_for_test()
            .components
            .push(Component::new(1, ComponentType::Resistor, Point::origin()));
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(1);
        state.schematic.init_undo_history();
        arm_test_move(&mut state, crate::state::MoveSelectionMode::Shove);
        state.dialogs.move_selection.canvas.preview_delta = Point::new(40, 10);

        crate::workbench::app::cancel_armed_move_selection(&mut state);

        assert_eq!(
            state.schematic.document().components[0].pos,
            Point::origin()
        );
        assert!(state.schematic.session.editor.selection.has_component(1));
        assert_eq!(state.schematic.session.editor.tool, Tool::Select);
        assert!(!state.schematic.can_undo());
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
                .schematic
                .add_component(ComponentType::Resistor, Point::origin());
            state
                .schematic
                .session
                .editor
                .selection
                .select_only_component(id);
            state.schematic.clear_undo_history();
            arm_test_move(&mut state, crate::state::MoveSelectionMode::Connected);
            let expected = state.dialogs.move_selection.canvas.clone();
            let mut next = expected.clone();
            next.preview_delta = Point::new(100, 0);
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
                "mode" => {
                    state.dialogs.move_selection.mode = crate::state::MoveSelectionMode::Shove
                }
                "draft" => state.dialogs.move_selection.canvas.anchor = Some(Point::new(10, 0)),
                "modal" => state.dialogs.preferences_open = true,
                "read-only" => state.schematic.session.read_only = true,
                "tool" => state.schematic.arm_tool(Tool::Select),
                "replacement" => {
                    cancel_armed_move_selection(&mut state);
                    arm_test_move(&mut state, crate::state::MoveSelectionMode::Connected);
                    assert_eq!(
                        request.transition.expected,
                        state.dialogs.move_selection.canvas
                    );
                    assert_eq!(request.source, editor_request_source(&state));
                    assert_ne!(request.generation, state.dialogs.move_selection.generation);
                }
                "none" => {}
                _ => unreachable!(),
            }
            let canvas = state.dialogs.move_selection.canvas.clone();
            let symbols = schematic_symbol_context(&state);
            apply(&mut state, &symbols, request);
            if change == "none" {
                assert_eq!(
                    state.schematic.document().components[0].pos,
                    Point::new(100, 0)
                );
                assert_eq!(state.schematic.undo_description(), Some("move selection"));
            } else {
                assert_eq!(
                    state.schematic.document().components[0].pos,
                    Point::origin(),
                    "{change}"
                );
                assert!(!state.schematic.can_undo(), "{change}");
                assert_eq!(state.dialogs.move_selection.canvas, canvas, "{change}");
                assert!(state.dialogs.move_selection.armed, "{change}");
            }
        }
    }
}
