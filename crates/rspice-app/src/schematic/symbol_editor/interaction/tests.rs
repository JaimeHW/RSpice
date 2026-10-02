//! Symbol canvas transactions: pointer gestures, undo grouping and stale-source rejection.

use super::*;
use crate::state::{
    Cell, CellViewRef, Library, Point, PortDirection, SymbolPin, SymbolShape, View, ViewType,
};
use crate::workbench::{SymbolSelection, SymbolTool};
use rspice_schematic_editor::symbol_editor::interaction::SymbolCanvasInput;

fn open_symbol() -> AppState {
    let mut state = AppState::default();
    let mut library = Library::new("work");
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    cell.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(cell);
    state.library_manager.add_library(library);
    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    let document = SymbolDocument {
        pins: vec![SymbolPin::new(
            "IN",
            PortDirection::In,
            Some(Point::new(-40, 0)),
        )],
        body: vec![SymbolShape::Circle {
            center: Point::origin(),
            radius: 10,
        }],
        ..SymbolDocument::default()
    };
    state
        .store_active_symbol_editor_bundle(
            &document,
            &SymbolEditorMetadata::for_document(&document),
        )
        .unwrap();
    bind_active_canvas(&mut state);
    state
}

fn viewport() -> SymbolViewport {
    SymbolViewport {
        rect: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(400.0, 400.0)),
        zoom: 2.0,
        pan: egui::Vec2::ZERO,
    }
}

fn request(state: &AppState, input: SymbolCanvasInput) -> SymbolCanvasRequest {
    SymbolCanvasRequest {
        source: request_source(state),
        tool: state.ui.symbol.editor.tool,
        selection: state.ui.symbol.editor.effective_selection(),
        viewport: viewport(),
        input,
    }
}

fn apply(state: &mut AppState, request: SymbolCanvasRequest) {
    let mut document = state.load_active_symbol_document().unwrap();
    let mut metadata = state.load_active_symbol_editor_metadata(&document).unwrap();
    apply_canvas_request(state, &mut document, &mut metadata, request);
}

#[test]
fn drag_symbol_edit_records_one_undo_snapshot_per_gesture() {
    let mut state = open_symbol();
    let before = state.load_active_symbol_document().unwrap();
    state.ui.symbol.editor.dragging_origin = true;
    for point in [Point::new(10, 20), Point::new(20, 30)] {
        bind_active_canvas(&mut state);
        let request = request(
            &state,
            SymbolCanvasInput {
                pointer: viewport().world_to_screen(point),
                dragged: true,
                ..Default::default()
            },
        );
        apply(&mut state, request);
    }
    let after = state.load_active_symbol_document().unwrap();
    assert_eq!(after.origin, Point::new(20, 30));
    assert_eq!(
        state
            .ui
            .symbol
            .history
            .undo_depth(&state.workspace.content.active_key()),
        1
    );
    assert!(state.ui.symbol.editor.drag_undo_recorded);
    let request = request(
        &state,
        SymbolCanvasInput {
            drag_stopped: true,
            ..Default::default()
        },
    );
    apply(&mut state, request);
    assert!(!state.ui.symbol.editor.drag_undo_recorded);
    assert!(state.undo_active_symbol_document().unwrap());
    assert_eq!(state.load_active_symbol_document().unwrap(), before);
    assert!(state.redo_active_symbol_document().unwrap());
    assert_eq!(state.load_active_symbol_document().unwrap(), after);
}

#[test]
fn a_multi_object_grab_moves_the_whole_selection_as_one_edit() {
    let mut state = open_symbol();
    let before = state.load_active_symbol_document().unwrap();
    let mut selection = SymbolSelection::single_pin("IN");
    selection.toggle_shape(0);
    state.ui.symbol.editor.set_selection(selection);
    let ctx = egui::Context::default();
    let pointer = |point| viewport().world_to_screen(point);
    let button = |pos, pressed| egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    let start = pointer(Point::new(-40, 0));
    let middle = pointer(Point::new(-30, 20));
    let end = pointer(Point::new(-20, 30));
    for (frame, events) in [
        vec![egui::Event::PointerMoved(start)],
        vec![button(start, true)],
        // Cross egui's drag threshold while the pointer still hits the grabbed pin.
        vec![egui::Event::PointerMoved(pointer(Point::new(-36, 0)))],
        vec![egui::Event::PointerMoved(middle)],
        vec![egui::Event::PointerMoved(end)],
        vec![button(end, false)],
    ]
    .into_iter()
    .enumerate()
    {
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(viewport().rect),
                events,
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    bind_active_canvas(&mut state);
                    let response = ui.allocate_rect(viewport().rect, egui::Sense::click_and_drag());
                    let mut document = state.load_active_symbol_document().unwrap();
                    let mut metadata = state.load_active_symbol_editor_metadata(&document).unwrap();
                    handle_canvas(
                        &mut state,
                        &mut document,
                        &mut metadata,
                        viewport(),
                        &response,
                    );
                });
            },
        );
        if frame == 2 {
            assert!(state.ui.symbol.editor.dragging_group.is_some());
        }
    }
    let after = state.load_active_symbol_document().unwrap();
    assert_eq!(
        after.pin("IN").and_then(|pin| pin.position),
        Some(Point::new(-20, 30))
    );
    assert!(
        matches!(after.body.first(), Some(SymbolShape::Circle { center, .. }) if *center == Point::new(20, 30))
    );
    assert_eq!(
        state
            .ui
            .symbol
            .history
            .undo_depth(&state.workspace.content.active_key()),
        1
    );
    assert!(!state.ui.symbol.editor.drag_undo_recorded);
    assert!(state.undo_active_symbol_document().unwrap());
    assert_eq!(state.load_active_symbol_document().unwrap(), before);
}

#[test]
fn stale_symbol_input_cannot_edit_a_replaced_view_or_revision() {
    let mut state = open_symbol();
    state.ui.symbol.editor.tool = SymbolTool::Text;
    let pending = request(
        &state,
        SymbolCanvasInput {
            clicked: true,
            ..Default::default()
        },
    );
    let before = state.load_active_symbol_document().unwrap();
    let mut metadata = state.load_active_symbol_editor_metadata(&before).unwrap();
    metadata.revision += 1;
    state
        .store_active_symbol_editor_bundle(&before, &metadata)
        .unwrap();
    apply(&mut state, pending);
    assert_eq!(state.load_active_symbol_document().unwrap(), before);
    assert!(!state.can_undo_active_symbol_document());

    bind_active_canvas(&mut state);
    let pending = request(
        &state,
        SymbolCanvasInput {
            clicked: true,
            ..Default::default()
        },
    );
    state.open_workspace_view(CellViewRef::new("work", "amp", "schematic"));
    state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    bind_active_canvas(&mut state);
    apply(&mut state, pending);
    assert_eq!(state.load_active_symbol_document().unwrap(), before);
    assert!(!state.can_undo_active_symbol_document());
}

#[test]
fn changed_symbol_scope_discards_drafts_and_read_only_input_cannot_publish() {
    let mut state = open_symbol();
    state.ui.symbol.editor.pending_polyline = vec![Point::origin(), Point::new(10, 20)];
    state.ui.symbol.editor.shape_start = Some(Point::origin());
    state.ui.symbol.editor.marquee_start = Some(Point::origin());
    state.ui.symbol.editor.dragging_origin = true;
    state.active_schematic_epoch += 1;
    bind_active_canvas(&mut state);
    assert!(state.ui.symbol.editor.pending_polyline.is_empty());
    assert!(state.ui.symbol.editor.shape_start.is_none());
    assert!(state.ui.symbol.editor.marquee_start.is_none());
    assert!(!state.ui.symbol.editor.dragging_origin);

    state.workspace.content.set_active_read_only_reference(true);
    state.ui.symbol.editor.tool = SymbolTool::Text;
    let before = state.load_active_symbol_document().unwrap();
    let request = request(
        &state,
        SymbolCanvasInput {
            clicked: true,
            ..Default::default()
        },
    );
    apply(&mut state, request);
    assert_eq!(state.load_active_symbol_document().unwrap(), before);
    assert!(!state.can_undo_active_symbol_document());

    state
        .workspace
        .content
        .set_active_read_only_reference(false);
    state.schematic.session.read_only = true;
    let pending = self::request(
        &state,
        SymbolCanvasInput {
            clicked: true,
            ..Default::default()
        },
    );
    apply(&mut state, pending);
    assert_eq!(state.load_active_symbol_document().unwrap(), before);
    assert!(!state.can_undo_active_symbol_document());
}

#[test]
fn symbol_gesture_cannot_follow_the_same_master_into_another_occurrence() {
    use rspice_design::occurrence::{DocumentOccurrence, OccurrenceStep};
    let mut state = open_symbol();
    let occurrence = |name: &str| DocumentOccurrence {
        root: CellViewRef::new("work", "top", "schematic"),
        steps: vec![OccurrenceStep {
            instance_name: name.to_owned(),
            master: CellViewRef::new("work", "amp", "symbol"),
        }],
    };
    state
        .workspace
        .content
        .set_active_occurrence(occurrence("X1"));
    bind_active_canvas(&mut state);
    state.ui.symbol.editor.dragging_origin = true;
    let pending = request(
        &state,
        SymbolCanvasInput {
            pointer: viewport().world_to_screen(Point::new(20, 30)),
            dragged: true,
            ..Default::default()
        },
    );
    let before = state.load_active_symbol_document().unwrap();
    state
        .workspace
        .content
        .set_active_occurrence(occurrence("X2"));
    apply(&mut state, pending);
    bind_active_canvas(&mut state);
    assert!(!state.ui.symbol.editor.dragging_origin);
    assert_eq!(state.load_active_symbol_document().unwrap(), before);
    assert!(!state.can_undo_active_symbol_document());
}
