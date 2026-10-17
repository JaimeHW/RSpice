//! Bind array canvas requests to the app-owned checked transaction.

use super::{SchematicSymbolContext, requests::editor_request_source, viewport::Viewport};
use crate::state::{SchematicArrayPlacement, Tool};
use crate::workbench::{app::armed_array_selection_authority, app_state::AppState};
use egui::{Response, Ui};
use rspice_schematic_editor::view::array_interaction::{self, ArrayInputRequest, ArrayInputView};
use rspice_schematic_editor::view::transform_input::TransformInputTransition;

pub(super) fn handle_armed_array_selection(
    ui: &Ui,
    response: &Response,
    state: &mut AppState,
    viewport: &Viewport,
    grid_size: i32,
    symbol_context: &SchematicSymbolContext,
) {
    if state.application_modal_open()
        || state.schematic.session.editor.tool != Tool::ArraySelection
        || !state.dialogs.array_selection.armed
    {
        return;
    }
    if let Err(message) = armed_array_selection_authority(state) {
        state.prompt(format!("Create array cancelled: {message}"));
        crate::workbench::app::cancel_armed_array_selection(state);
        return;
    }
    if let Some(transition) = array_interaction::input(
        ui,
        response,
        viewport,
        ArrayInputView {
            canvas: &state.dialogs.array_selection.canvas,
            kind: state.dialogs.array_selection.kind,
            grid_size,
            snap_engine: &state.schematic.session.editor.snap_engine,
        },
    ) {
        let request = capture(state, transition);
        apply(state, symbol_context, request);
    }
}

fn capture(state: &AppState, transition: TransformInputTransition) -> ArrayInputRequest {
    let draft = &state.dialogs.array_selection;
    ArrayInputRequest {
        source: editor_request_source(state),
        selection: state.schematic.session.editor.selection.clone(),
        generation: draft.generation,
        kind: draft.kind,
        count: draft.count.clone(),
        naming: draft.naming.clone(),
        transition,
    }
}

fn apply(state: &mut AppState, symbols: &SchematicSymbolContext, request: ArrayInputRequest) {
    let draft = &state.dialogs.array_selection;
    if request.source != editor_request_source(state)
        || request.selection != state.schematic.session.editor.selection
        || request.generation != draft.generation
        || request.kind != draft.kind
        || request.count != draft.count
        || request.naming != draft.naming
        || request.transition.expected != draft.canvas
        || state.schematic.session.editor.tool != Tool::ArraySelection
        || !draft.armed
        || state.application_modal_open()
        || armed_array_selection_authority(state).is_err()
    {
        return;
    }
    state.dialogs.array_selection.canvas = request.transition.next;
    if request.transition.commit {
        commit_armed_array_selection(state, symbols);
    }
}

pub(super) fn array_placement(state: &AppState) -> Result<SchematicArrayPlacement, &'static str> {
    array_interaction::array_placement(
        state.dialogs.array_selection.kind,
        &state.dialogs.array_selection.canvas,
    )
}

fn commit_armed_array_selection(state: &mut AppState, symbol_context: &SchematicSymbolContext) {
    if let Err(message) = armed_array_selection_authority(state) {
        state.prompt(format!("Create array cancelled: {message}"));
        crate::workbench::app::cancel_armed_array_selection(state);
        return;
    }
    let plan = match array_placement(state)
        .map_err(str::to_owned)
        .and_then(|placement| crate::workbench::app::armed_array_selection_plan(state, placement))
    {
        Ok(plan) => plan,
        Err(message) => {
            state.dialogs.array_selection.canvas.reset_candidate();
            state.prompt(format!("Array not created: {message}"));
            return;
        }
    };

    let sheets = &state.workspace.content.design_management;
    let catalog = sheets.sheet_catalog(&state.workspace.content.active_schematic_reference().key());
    let on_sheet =
        |id| rspice_schematic_editor::view::design_view::object_is_on_active_sheet(catalog, id);
    match state.schematic.array_selection_resolved(
        &plan,
        &on_sheet,
        |component| symbol_context.named_terminal_points(component),
        |component| symbol_context.component_bounds_tuple(component),
    ) {
        Ok(impact) => {
            state.sync_active_schematic_to_workspace();
            state.prompt(format!("Created {} array members.", impact.replicas));
            crate::workbench::app::cancel_armed_array_selection(state);
        }
        Err(error) => {
            state.dialogs.array_selection.canvas.preview_error = Some(error.to_string());
            state.dialogs.array_selection.canvas.reset_candidate();
            state.prompt(format!("Array not created: {error}"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schematic::view::schematic_symbol_context;
    use crate::state::{Component, ComponentType, Point};

    #[test]
    fn armed_array_commit_creates_one_transaction_and_returns_to_select() {
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
        state.schematic.recalculate_runtime_state();
        state.schematic.clear_undo_history();
        crate::workbench::app::open_array_selection_dialog(&mut state);
        state.dialogs.array_selection.arm();
        state.dialogs.array_selection.canvas.preview_delta = Point::new(100, 0);
        state.schematic.arm_tool(Tool::ArraySelection);
        let symbols = schematic_symbol_context(&state);

        let ctx = egui::Context::default();
        state.dialogs.array_selection.canvas.preview_delta.x -=
            state.schematic.document().grid_size;
        for commit in [false, true] {
            let events = if commit {
                [egui::Key::ArrowRight, egui::Key::Enter]
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
                            egui::Id::new("array-canvas"),
                            // accessibility-pointer-shim: synthetic input for the array handler test.
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
                        let grid_size = state.schematic.document().grid_size;
                        handle_armed_array_selection(
                            ui, &response, &mut state, &viewport, grid_size, &symbols,
                        );
                    });
                },
            );
        }

        assert_eq!(state.schematic.document().components.len(), 8);
        assert_eq!(state.schematic.undo_description(), Some("create array"));
        assert_eq!(state.schematic.session.editor.tool, Tool::Select);
        assert!(!state.dialogs.array_selection.armed);
        assert!(state.schematic.undo());
        assert_eq!(state.schematic.document().components.len(), 1);
        assert!(!state.schematic.can_undo());
    }

    /// Arm Create array on the selected parts and wires with `pitch`, and
    /// commit it.
    fn commit_array_of(state: &mut AppState, components: &[u64], wires: &[u64], pitch: Point) {
        let selection = &mut state.schematic.session.editor.selection;
        selection.clear();
        for id in components {
            selection.select_component(*id);
        }
        for id in wires {
            selection.select_wire(*id);
        }
        state.schematic.recalculate_runtime_state();
        state.schematic.clear_undo_history();
        crate::workbench::app::open_array_selection_dialog(state);
        state.dialogs.array_selection.arm();
        state.dialogs.array_selection.canvas.preview_delta = pitch;
        state.schematic.arm_tool(Tool::ArraySelection);
        let symbols = schematic_symbol_context(state);
        commit_armed_array_selection(state, &symbols);
    }

    #[test]
    fn another_sheets_wire_does_not_block_the_array() {
        use crate::state::Wire;
        // R1 on the active sheet; on Sheet 2, a wire right where the
        // first replica will stand.
        let mut state = crate::schematic::view::sheet_visibility::two_sheet_state(&[1], &[2]);
        let drawing = state.schematic.document_mut_for_test();
        drawing
            .components
            .push(Component::new(1, ComponentType::Resistor, Point::origin()));
        drawing.wires.push(Wire::segment(
            2,
            Point::new(-200, 100),
            Point::new(400, 100),
        ));

        commit_array_of(&mut state, &[1], &[], Point::new(0, 100));

        assert_eq!(
            state.dialogs.array_selection.canvas.preview_error, None,
            "a wire the active sheet does not show refused the array"
        );
        assert_eq!(state.schematic.document().components.len(), 8);
    }

    #[test]
    fn the_array_takes_no_wire_or_junction_from_another_sheet() {
        use crate::state::{Junction, Wire};
        // R3 and R4 stand unconnected on the active sheet, under a selected
        // wire. Sheet 2 reuses the layout: a wire joins its two parts at the
        // same points, and a junction sits on the selected wire's line.
        let mut state =
            crate::schematic::view::sheet_visibility::two_sheet_state(&[1, 3, 5], &[2, 6]);
        let r3 = Component::new(1, ComponentType::Resistor, Point::origin());
        let r4 = Component::new(3, ComponentType::Resistor, Point::new(100, 0));
        let r3_right = r3.terminal_positions()[1].1;
        let r4_left = r4.terminal_positions()[0].1;
        let drawing = state.schematic.document_mut_for_test();
        drawing.components.extend([r3, r4]);
        drawing.wires.push(Wire::segment(2, r3_right, r4_left));
        drawing
            .wires
            .push(Wire::segment(5, Point::new(-50, -50), Point::new(150, -50)));
        drawing
            .junctions
            .push(Junction::new(6, Point::new(50, -50)));

        commit_array_of(&mut state, &[1, 3], &[5], Point::new(0, 200));

        assert_eq!(state.dialogs.array_selection.canvas.preview_error, None);
        let document = state.schematic.document();
        assert_eq!(document.components.len(), 16);
        assert!(
            document
                .wires
                .iter()
                .filter(|wire| wire.id != 2 && wire.id != 5)
                .all(|wire| wire.points.iter().all(|point| point.y % 200 == 150)),
            "only the selected wire is replicated: {:?}",
            document.wires
        );
        assert_eq!(
            document.junctions.len(),
            1,
            "Sheet 2's junction was replicated onto the array"
        );
    }

    #[test]
    fn rejected_array_candidate_stays_armed_without_mutating_the_document() {
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
        state.schematic.recalculate_runtime_state();
        state.schematic.clear_undo_history();
        crate::workbench::app::open_array_selection_dialog(&mut state);
        state.dialogs.array_selection.arm();
        state.dialogs.array_selection.canvas.preview_delta = Point::new(1, 0);
        state.schematic.arm_tool(Tool::ArraySelection);
        let symbols = schematic_symbol_context(&state);

        commit_armed_array_selection(&mut state, &symbols);

        assert_eq!(state.schematic.document().components.len(), 1);
        assert!(state.dialogs.array_selection.armed);
        assert_eq!(state.schematic.session.editor.tool, Tool::ArraySelection);
        assert!(state.dialogs.array_selection.canvas.preview_error.is_some());
        assert!(!state.schematic.can_undo());
    }

    /// A replica whose wire end or pin would land inside a wire already on
    /// the sheet is refused, so Create array can never leave the undotted,
    /// unsplit T a paste used to (audit W-9). R1 is arrayed downwards, once
    /// with a stub wire off its right pin and once bare; a rail runs through
    /// the first copy's stub end, then through its pin.
    #[test]
    fn an_array_copy_touching_a_wire_is_refused_rather_than_left_as_a_bare_t() {
        use crate::state::Wire;
        let part = Component::new(1, ComponentType::Resistor, Point::origin());
        let pin = part.terminal_positions()[1].1;
        let stub_end = Point::new(pin.x + 20, pin.y);
        for (case, stub, touch) in [("stub end", true, stub_end), ("pin", false, pin)] {
            let mut state = AppState::default();
            let drawing = state.schematic.document_mut_for_test();
            drawing.components.push(part.clone());
            if stub {
                drawing.wires.push(Wire::segment(2, pin, stub_end));
            }
            drawing.wires.push(Wire::segment(
                3,
                Point::new(touch.x, touch.y + 50),
                Point::new(touch.x, touch.y + 150),
            ));
            let wire_points = |state: &AppState| -> Vec<Vec<Point>> {
                let document = state.schematic.document();
                document
                    .wires
                    .iter()
                    .map(|wire| wire.points.clone())
                    .collect()
            };
            let before = wire_points(&state);

            let wires: &[u64] = if stub { &[2] } else { &[] };
            commit_array_of(&mut state, &[1], wires, Point::new(0, 100));

            let refusal = state
                .dialogs
                .array_selection
                .canvas
                .preview_error
                .clone()
                .unwrap_or_default();
            assert!(refusal.contains("object 3 at"), "{case}: {refusal:?}");
            let document = state.schematic.document();
            assert_eq!(document.components.len(), 1, "{case}");
            assert!(document.junctions.is_empty(), "{case}");
            assert_eq!(wire_points(&state), before, "{case}");
            assert!(!state.schematic.can_undo(), "{case}");
        }
    }

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
                .schematic
                .add_component(ComponentType::Resistor, Point::origin());
            state
                .schematic
                .session
                .editor
                .selection
                .select_only_component(id);
            state.schematic.clear_undo_history();
            crate::workbench::app::open_array_selection_dialog(&mut state);
            state.dialogs.array_selection.arm();
            state.schematic.arm_tool(Tool::ArraySelection);
            let expected = state.dialogs.array_selection.canvas.clone();
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
                "kind" => {
                    state.dialogs.array_selection.kind =
                        crate::state::SchematicArrayKind::Rectangular
                }
                "count" => state.dialogs.array_selection.count = "4".to_owned(),
                "naming" => state.dialogs.array_selection.naming = "R20…R27".to_owned(),
                "draft" => state.dialogs.array_selection.canvas.anchor = Some(Point::new(10, 0)),
                "modal" => state.dialogs.preferences_open = true,
                "read-only" => state.schematic.session.read_only = true,
                "tool" => state.schematic.arm_tool(Tool::Select),
                "replacement" => {
                    crate::workbench::app::cancel_armed_array_selection(&mut state);
                    crate::workbench::app::open_array_selection_dialog(&mut state);
                    state.dialogs.array_selection.arm();
                    state.schematic.arm_tool(Tool::ArraySelection);
                    assert_eq!(
                        request.transition.expected,
                        state.dialogs.array_selection.canvas
                    );
                    assert_eq!(request.source, editor_request_source(&state));
                    assert_ne!(request.generation, state.dialogs.array_selection.generation);
                }
                "none" => {}
                _ => unreachable!(),
            }
            let canvas = state.dialogs.array_selection.canvas.clone();
            let symbols = schematic_symbol_context(&state);
            apply(&mut state, &symbols, request);
            if change == "none" {
                assert_eq!(state.schematic.document().components.len(), 8);
                assert_eq!(state.schematic.undo_description(), Some("create array"));
            } else {
                assert_eq!(state.schematic.document().components.len(), 1, "{change}");
                assert!(!state.schematic.can_undo(), "{change}");
                assert_eq!(state.dialogs.array_selection.canvas, canvas, "{change}");
                assert!(state.dialogs.array_selection.armed, "{change}");
            }
        }
    }
}
