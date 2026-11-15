//! Validate stretch canvas requests before the app-owned atomic commit.

use super::{
    SchematicSymbolContext, parts::CanvasParts, requests::editor_request_source,
    sheet_visibility::with_active_wire_topology, viewport::Viewport,
};
use crate::state::{Point, Tool};
use crate::workbench::app::{armed_stretch_selection_authority, cancel_armed_stretch_selection_in};
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
    state: &mut CanvasParts<'_>,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
) {
    if state.application_modal_open()
        || state.model.schematic.session.editor.tool != Tool::StretchSelection
        || !state.design.stretch_selection.armed
    {
        return;
    }
    if let Err(message) =
        armed_stretch_selection_authority(&state.design.stretch_selection, state.model)
    {
        state.prompt(format!("Stretch cancelled: {message}"));
        cancel_armed_stretch_selection_in(&mut state.design.stretch_selection, state.model);
        return;
    }
    if let Some(transition) = stretch_interaction::input(
        ui,
        response,
        viewport,
        StretchInputView {
            design: state.design_view(),
            selection: &state.model.schematic.session.editor.selection,
            canvas: &state.design.stretch_selection.canvas,
            policy: state.design.stretch_selection.policy,
            snap_engine: &state.model.schematic.session.editor.snap_engine,
        },
    ) {
        let request = capture(state, transition);
        apply(state, symbol_context, request);
    }
}

pub(crate) fn capture(
    state: &CanvasParts<'_>,
    transition: TransformInputTransition<StretchCanvasSession>,
) -> StretchInputRequest {
    StretchInputRequest {
        source: editor_request_source(state),
        selection: state.model.schematic.session.editor.selection.clone(),
        generation: state.design.stretch_selection.generation,
        policy: state.design.stretch_selection.policy,
        transition,
    }
}

pub(crate) fn apply(
    state: &mut CanvasParts<'_>,
    symbols: &SchematicSymbolContext,
    request: StretchInputRequest,
) {
    let draft = &state.design.stretch_selection;
    if request.source != editor_request_source(state)
        || request.selection != state.model.schematic.session.editor.selection
        || request.generation != draft.generation
        || request.policy != draft.policy
        || request.transition.expected != draft.canvas
        || state.model.schematic.session.editor.tool != Tool::StretchSelection
        || !draft.armed
        || state.application_modal_open()
        || armed_stretch_selection_authority(&state.design.stretch_selection, state.model).is_err()
    {
        return;
    }
    state.design.stretch_selection.canvas = request.transition.next;
    if request.transition.commit {
        commit_armed_stretch_selection(state, symbols);
    }
}

pub(super) fn commit_armed_stretch_selection(
    state: &mut CanvasParts<'_>,
    symbol_context: &SchematicSymbolContext,
) {
    if let Err(message) =
        armed_stretch_selection_authority(&state.design.stretch_selection, state.model)
    {
        state.prompt(format!("Stretch cancelled: {message}"));
        cancel_armed_stretch_selection_in(&mut state.design.stretch_selection, state.model);
        return;
    }
    if let Some(message) = state
        .design
        .stretch_selection
        .canvas
        .gesture
        .preview_error
        .clone()
    {
        let draft = &mut state.design.stretch_selection;
        draft.canvas.gesture.reset_candidate();
        state.prompt(format!("Stretch not committed: {message}"));
        return;
    }
    let delta = state.design.stretch_selection.canvas.gesture.preview_delta;
    let policy = state.design.stretch_selection.policy;
    let target = state
        .design
        .stretch_selection
        .canvas
        .target
        .expect("validated stretch target");
    if delta == Point::origin() {
        state.prompt("Nothing stretched.");
        cancel_armed_stretch_selection_in(&mut state.design.stretch_selection, state.model);
        return;
    }

    state.model.schematic.begin_operation("stretch selection");
    // Another sheet's wire or junction can share the stretched segment's
    // coordinates; it neither anchors the stretch nor moves with it.
    let stretched = with_active_wire_topology(state.model, |schematic, scope| {
        schematic.stretch_target_resolved(
            delta,
            target,
            policy,
            |component| symbol_context.terminal_points(component),
            |component| symbol_context.component_bounds_tuple(component),
            scope,
        )
    });
    match stretched {
        Ok(true) => {
            let recorded = state.model.schematic.end_operation();
            state.sync_active_schematic_to_workspace();
            state.prompt(if recorded {
                format!(
                    "Stretched the selection by ({}, {}) with {}.",
                    delta.x,
                    delta.y,
                    policy.label(),
                )
            } else {
                "Nothing stretched.".to_owned()
            });
            cancel_armed_stretch_selection_in(&mut state.design.stretch_selection, state.model);
        }
        Ok(false) => {
            state.model.schematic.cancel_operation();
            state.prompt("Nothing stretched.");
            cancel_armed_stretch_selection_in(&mut state.design.stretch_selection, state.model);
        }
        Err(error) => {
            state.model.schematic.cancel_operation();
            let draft = &mut state.design.stretch_selection;
            draft.canvas.gesture.preview_error = Some(error.to_string());
            draft.canvas.gesture.reset_candidate();
            state.prompt(format!("Stretch not committed: {error}"));
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::schematic::view::schematic_symbol_context;
    use crate::state::{Junction, Wire};
    use crate::workbench::design::test_host::{DesignTestHost, canvas_facts, canvas_parts};

    pub(crate) fn selected_stretch_wire() -> DesignTestHost {
        selected_stretch_wire_in(DesignTestHost::default())
    }

    fn selected_stretch_wire_in(mut state: DesignTestHost) -> DesignTestHost {
        state
            .model
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
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_wire_segment(7, 1);
        state.model.schematic.init_undo_history();
        state
    }

    fn arm_test_stretch(state: &mut DesignTestHost) {
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
    fn commit_records_once_syncs_workspace_and_retains_selection() {
        let mut state = selected_stretch_wire();
        arm_test_stretch(&mut state);
        state.design.stretch_selection.canvas.gesture.preview_delta = Point::origin();
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
                            ui,
                            &response,
                            &mut canvas_parts(&mut state),
                            &viewport,
                            &symbols,
                        );
                    });
                },
            );
        }

        assert_eq!(
            state.model.schematic.document().wires[0].points[1],
            Point::new(0, 30)
        );
        assert_eq!(
            state.model.schematic.document().wires[0].points[2],
            Point::new(20, 30)
        );
        assert_eq!(
            state.model.schematic.undo_description(),
            Some("stretch selection")
        );
        assert!(
            state
                .model
                .schematic
                .session
                .editor
                .selection
                .has_wire_segment(7, 1)
        );
        assert_eq!(
            state.model.schematic.session.editor.tool,
            crate::state::Tool::Select
        );
        assert!(!state.design.stretch_selection.armed);
        assert_eq!(
            state
                .model
                .workspace
                .active_schematic()
                .expect("active workspace buffer")
                .document()
                .wires[0]
                .points[1],
            Point::new(0, 30)
        );
        assert!(state.model.schematic.undo());
        assert_eq!(
            state.model.schematic.document().wires[0].points[1],
            Point::new(0, 20)
        );
        assert!(
            !state.model.schematic.can_undo(),
            "the gesture owns one undo record"
        );
    }

    #[test]
    fn cancel_preserves_geometry_selection_and_history() {
        let mut state = selected_stretch_wire();
        let baseline = state.model.schematic.document().wires[0].clone();
        arm_test_stretch(&mut state);
        state.design.stretch_selection.canvas.gesture.preview_delta = Point::new(0, 10);

        cancel_armed_stretch_selection_in(&mut state.design.stretch_selection, &mut state.model);

        assert_eq!(state.model.schematic.document().wires[0], baseline);
        assert!(
            state
                .model
                .schematic
                .session
                .editor
                .selection
                .has_wire_segment(7, 1)
        );
        assert_eq!(
            state.model.schematic.session.editor.tool,
            crate::state::Tool::Select
        );
        assert!(!state.model.schematic.can_undo());
    }

    /// A junction on Sheet 2 sits on the stretched segment's coordinates. It
    /// is another sheet's wiring: it neither anchors the stretch nor moves.
    #[test]
    fn another_sheets_junction_neither_anchors_nor_follows_a_stretch() {
        let mut state = selected_stretch_wire_in(
            crate::schematic::view::sheet_visibility::two_sheet_host(&[7], &[21]),
        );
        let hidden = Junction::new(21, Point::new(10, 20));
        state
            .model
            .schematic
            .document_mut_for_test()
            .junctions
            .push(hidden);
        arm_test_stretch(&mut state);
        state.design.stretch_selection.canvas.gesture.preview_delta = Point::new(0, 10);
        let symbols = schematic_symbol_context(&state);

        commit_armed_stretch_selection(&mut canvas_parts(&mut state), &symbols);

        assert_eq!(
            state.model.schematic.document().wires[0].points[1],
            Point::new(0, 30)
        );
        assert_eq!(state.model.schematic.document().junctions, vec![hidden]);
        assert_eq!(
            state.model.schematic.undo_description(),
            Some("stretch selection")
        );
    }

    /// A drawn route stretches with the segment beside it, which is its own
    /// wire. Sheet 2's wire ends on the other corner: it is another sheet's
    /// wiring, so it neither follows the corner nor pins it.
    #[test]
    fn a_route_stretches_with_its_neighbour_and_leaves_another_sheets_wire() {
        let mut state = crate::schematic::view::sheet_visibility::two_sheet_host(&[7, 8], &[21]);
        let hidden = Wire::segment(21, Point::new(20, 20), Point::new(20, 0));
        state.model.schematic.document_mut_for_test().wires = vec![
            Wire::segment(8, Point::new(0, 0), Point::new(0, 20)),
            Wire::segment(7, Point::new(0, 20), Point::new(20, 20)),
            hidden.clone(),
        ];
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_wire_segment(7, 0);
        state.model.schematic.init_undo_history();
        arm_test_stretch(&mut state);
        state.design.stretch_selection.canvas.gesture.preview_delta = Point::new(0, 10);
        let symbols = schematic_symbol_context(&state);

        commit_armed_stretch_selection(&mut canvas_parts(&mut state), &symbols);

        let wire = |id: u64| {
            state
                .model
                .schematic
                .document()
                .wires
                .iter()
                .find(|wire| wire.id == id)
                .cloned()
        };
        assert_eq!(
            wire(7).map(|wire| wire.points),
            Some(vec![Point::new(0, 30), Point::new(20, 30)])
        );
        assert_eq!(
            wire(8).map(|wire| wire.points),
            Some(vec![Point::new(0, 0), Point::new(0, 30)])
        );
        assert_eq!(wire(21), Some(hidden));
    }

    /// The preview runs with the other sheets out of reach, as the commit
    /// does: Sheet 2's junction on the segment refuses neither.
    #[test]
    fn another_sheets_junction_does_not_refuse_the_stretch_preview() {
        let mut state = selected_stretch_wire_in(
            crate::schematic::view::sheet_visibility::two_sheet_host(&[7], &[21]),
        );
        let hidden = Junction::new(21, Point::new(10, 20));
        state
            .model
            .schematic
            .document_mut_for_test()
            .junctions
            .push(hidden);
        let symbols = schematic_symbol_context(&state);

        let candidate = crate::schematic::view::preview::stretch_preview_candidate(
            &canvas_facts(&state),
            &symbols,
            Point::new(0, 10),
            crate::state::StretchTarget::WireSegment {
                wire_id: 7,
                segment_index: 1,
            },
            crate::state::StretchOrthogonalPolicy::default(),
        )
        .expect("the hidden junction is no anchor")
        .expect("the stretch moves the segment");

        assert_eq!(candidate.wires[0].points[1], Point::new(0, 30));
    }

    #[test]
    fn rejected_commit_preserves_authority_selection_and_undo_history() {
        let mut state = selected_stretch_wire();
        state
            .model
            .schematic
            .document_mut_for_test()
            .junctions
            .push(Junction::new(1, Point::new(10, 20)));
        let baseline = state.model.schematic.document().wires[0].clone();
        arm_test_stretch(&mut state);
        state.design.stretch_selection.canvas.gesture.preview_delta = Point::new(0, 10);
        let symbols = schematic_symbol_context(&state);

        commit_armed_stretch_selection(&mut canvas_parts(&mut state), &symbols);

        assert_eq!(state.model.schematic.document().wires[0], baseline);
        assert!(
            state
                .model
                .schematic
                .session
                .editor
                .selection
                .has_wire_segment(7, 1)
        );
        assert_eq!(
            state.model.schematic.session.editor.tool,
            crate::state::Tool::StretchSelection
        );
        assert!(state.design.stretch_selection.armed);
        assert!(
            state
                .design
                .stretch_selection
                .canvas
                .gesture
                .preview_error
                .is_some()
        );
        assert!(!state.model.schematic.can_undo());
    }
}
