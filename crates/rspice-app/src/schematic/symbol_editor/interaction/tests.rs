//! Symbol canvas transactions: pointer gestures, undo grouping and stale-source rejection.

use super::*;
use crate::state::{
    Cell, CellViewRef, Library, Point, PortDirection, SymbolPin, SymbolShape, View, ViewType,
};
use crate::workbench::SymbolTool;
use rspice_schematic_editor::symbol_editor::interaction::SymbolCanvasInput;
use rspice_schematic_editor::symbol_editor::session::SymbolSelection;

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
        source: state.symbol_editor_request_source(),
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
                    // accessibility-pointer-shim: synthetic input for the canvas handler test.
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

const TOP: &str = "work/top/schematic";
/// X1's world position in `top`; INP's wire is 10, INN's is 11.
const X1: Point = Point::new(100, 100);

/// Cell `amp` drawn as an OTA, INP above INN on the left edge, and placed as
/// X1 in an open parent `top` that wires each input out to the left.
///
/// With `hidden_vertex`, `top` has two sheets: X1 and its wires on Sheet 1,
/// and on Sheet 2 a wire whose vertex sits exactly where INP meets its wire.
fn ota_placed_in_open_parent(hidden_vertex: bool) -> AppState {
    use crate::state::{
        Component, ComponentType, LibraryCellInstance, PortSpec, SchematicState, SheetDefinition,
        SheetPortPolicy, SheetTemplate, Wire,
    };
    let mut state = open_symbol();
    let document = SymbolDocument {
        pins: vec![
            SymbolPin::new("INP", PortDirection::In, Some(Point::new(-30, -10))),
            SymbolPin::new("INN", PortDirection::In, Some(Point::new(-30, 10))),
        ],
        body: vec![SymbolShape::Polyline {
            points: vec![
                Point::new(-10, -40),
                Point::new(10, -40),
                Point::new(10, 40),
                Point::new(-10, 40),
            ],
            closed: true,
        }],
        ..SymbolDocument::default()
    };
    state
        .store_active_symbol_editor_bundle(
            &document,
            &SymbolEditorMetadata::for_document(&document),
        )
        .unwrap();
    let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
    binding.bind_interface(&[
        PortSpec {
            name: "INP".to_owned(),
            direction: PortDirection::In,
        },
        PortSpec {
            name: "INN".to_owned(),
            direction: PortDirection::In,
        },
    ]);
    let mut parent = SchematicState::default();
    let drawing = parent.document_mut_for_test();
    drawing
        .components
        .push(Component::new(1, ComponentType::CellInstance, X1).with_library_cell(binding));
    drawing
        .wires
        .push(Wire::segment(10, Point::new(70, 90), Point::new(40, 90)));
    drawing
        .wires
        .push(Wire::segment(11, Point::new(70, 110), Point::new(40, 110)));
    if hidden_vertex {
        drawing
            .wires
            .push(Wire::segment(12, Point::new(70, 90), Point::new(70, 40)));
        let catalogs = &mut state.workspace.content.design_management;
        let first = catalogs
            .bootstrap_for_cell_view(TOP, "Sheet 1", vec![1, 10, 11, 12])
            .unwrap();
        let catalog = catalogs.sheet_catalog_mut(TOP).unwrap();
        let second = catalog
            .create_sheet(
                SheetDefinition {
                    name: "Sheet 2".to_owned(),
                    template: SheetTemplate::AnalogSchematic,
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(2),
                },
                Some(first),
            )
            .unwrap();
        catalog
            .assign_objects(catalog.revision(), second, [12])
            .unwrap();
        catalog.set_active(first).unwrap();
    }
    state
        .workspace
        .insert_schematic_editor(TOP.to_owned(), parent);
    bind_active_canvas(&mut state);
    state
}

fn parent_document(state: &AppState) -> rspice_design::schematic::document::SchematicDocument {
    state.workspace.content.schematic_buffers[TOP]
        .document()
        .clone()
}

/// The whole of `top` as it would be written, for a bit-for-bit comparison.
fn parent_bits(state: &AppState) -> serde_json::Value {
    serde_json::to_value(parent_document(state)).unwrap()
}

/// Each X1 terminal and the wires its net holds, as the extractor reads
/// `top` against the symbol as it stands now.
fn parent_nets(state: &AppState) -> std::collections::BTreeMap<String, Vec<u64>> {
    use rspice_design::resolved_symbol::ResolvedCellSymbol;
    let symbol = state.load_active_symbol_document().unwrap();
    let parent = parent_document(state);
    let extracted = rspice_design::connectivity::extract(
        &parent,
        |_| {
            Some(ResolvedCellSymbol::from_authored_document(
                symbol.clone(),
                &[],
            ))
        },
        |_| None,
    );
    extracted
        .terminals
        .iter()
        .map(|terminal| {
            let mut wires = extracted
                .net_at(terminal.point)
                .map(|net| net.wires.clone())
                .unwrap_or_default();
            wires.sort_unstable();
            (terminal.pin.clone(), wires)
        })
        .collect()
}

/// Drag INP through `points`, one frame each, then release.
fn drag_inp(state: &mut AppState, points: &[Point]) {
    state.ui.symbol.editor.dragging_pin = Some("INP".to_owned());
    for point in points {
        bind_active_canvas(state);
        let request = request(
            state,
            SymbolCanvasInput {
                pointer: viewport().world_to_screen(*point),
                dragged: true,
                ..Default::default()
            },
        );
        apply(state, request);
    }
    bind_active_canvas(state);
    let request = request(
        state,
        SymbolCanvasInput {
            drag_stopped: true,
            ..Default::default()
        },
    );
    apply(state, request);
}

/// INP dragged down the left edge, past INN, to below it.
const PAST_INN: [Point; 4] = [
    Point::new(-30, 0),
    Point::new(-30, 10),
    Point::new(-30, 20),
    Point::new(-30, 30),
];

#[test]
fn dragging_a_pin_past_another_leaves_every_parent_net_as_it_was() {
    let mut state = ota_placed_in_open_parent(false);
    let nets = parent_nets(&state);
    assert_eq!(nets["INP"], vec![10]);
    assert_eq!(nets["INN"], vec![11]);

    drag_inp(&mut state, &PAST_INN);

    let symbol = state.load_active_symbol_document().unwrap();
    assert_eq!(
        symbol.pin("INP").and_then(|pin| pin.position),
        Some(Point::new(-30, 30))
    );
    assert_eq!(parent_nets(&state), nets);
    let parent = parent_document(&state);
    assert_eq!(parent.wires[0].points[0], Point::new(70, 130));
    assert_eq!(parent.wires[1].points[0], Point::new(70, 110));
}

#[test]
fn symbol_undo_puts_the_parent_back_exactly() {
    let mut state = ota_placed_in_open_parent(false);
    let before = parent_bits(&state);
    drag_inp(&mut state, &PAST_INN);
    assert_ne!(parent_bits(&state), before);

    assert!(state.undo_active_symbol_document().unwrap());

    assert_eq!(parent_bits(&state), before);
}

#[test]
fn the_parent_undoes_a_whole_pin_drag_as_one_step() {
    let mut state = ota_placed_in_open_parent(false);
    let before = parent_bits(&state);
    let depth = state.workspace.content.schematic_buffers[TOP]
        .history()
        .undo_count();

    drag_inp(&mut state, &PAST_INN);

    assert_eq!(
        state.workspace.content.schematic_buffers[TOP]
            .history()
            .undo_count(),
        depth + 1
    );
    let mut undone = false;
    state
        .workspace
        .for_each_schematic_editor_mut(|key, parent| {
            if key == TOP {
                undone = parent.undo();
            }
        });
    assert!(undone);
    assert_eq!(parent_bits(&state), before);
}

#[test]
fn a_pin_drag_leaves_another_sheets_vertex_on_the_old_pin_point() {
    let mut state = ota_placed_in_open_parent(true);
    let hidden = parent_document(&state).wires[2].clone();

    drag_inp(&mut state, &PAST_INN);

    let parent = parent_document(&state);
    assert_eq!(parent.wires[0].points[0], Point::new(70, 130));
    assert_eq!(parent.wires[2], hidden);
}

#[test]
fn a_pin_is_never_dropped_on_another_pin() {
    let mut state = ota_placed_in_open_parent(false);
    let nets = parent_nets(&state);

    drag_inp(&mut state, &[Point::new(-30, 0), Point::new(-30, 10)]);

    let symbol = state.load_active_symbol_document().unwrap();
    assert_eq!(
        symbol.pin("INP").and_then(|pin| pin.position),
        Some(Point::new(-30, 0)),
        "INP holds its last free position rather than landing on INN"
    );
    assert_eq!(parent_nets(&state), nets);
}

#[test]
fn retained_symbol_drafts_require_the_same_open_view_revision_and_window() {
    use crate::workbench::state::WorkspaceDocumentId;

    for change in ["navigation", "revision", "replacement", "close", "window"] {
        let mut state = open_symbol();
        state.project_lifecycle.authority.open_session();
        let symbol = state.workspace.content.active_view.clone();
        let before = state.load_active_symbol_document().unwrap();
        let points = vec![Point::origin(), Point::new(20, 10)];
        state.ui.symbol.editor.tool = SymbolTool::Line;
        state.ui.symbol.editor.pending_polyline = points.clone();
        state.ui.symbol.editor.dragging_origin = true;
        state.ui.symbol.editor.marquee_start = Some(Point::origin());
        let pending = request(
            &state,
            SymbolCanvasInput {
                clicked: true,
                ..Default::default()
            },
        );
        if change == "revision" {
            let mut metadata = state.load_active_symbol_editor_metadata(&before).unwrap();
            metadata.revision += 1;
            state
                .store_active_symbol_editor_bundle(&before, &metadata)
                .unwrap();
        }
        if change == "close" {
            crate::workbench::lifecycle::project_lifecycle::close_active_document(&mut state)
                .unwrap();
            assert!(
                !state
                    .workspace
                    .session
                    .symbol_sessions
                    .contains_key(&symbol.key())
            );
        } else {
            state.open_workspace_view(CellViewRef::new("work", "amp", "schematic"));
        }
        if change == "replacement" {
            state.design_execution_epoch += 1;
        }
        if change == "window" {
            let layout = state.workbench.current_workspace_layout();
            let window = state
                .workbench
                .window_session
                .detach_document(
                    WorkspaceDocumentId::CellView(symbol.clone()),
                    "Symbol",
                    layout,
                    false,
                )
                .unwrap();
            state.workbench.window_session.set_current(window).unwrap();
        }
        state.open_workspace_view(symbol);
        bind_active_canvas(&mut state);
        assert!(!state.ui.symbol.editor.dragging_origin, "{change}");
        assert!(state.ui.symbol.editor.marquee_start.is_none(), "{change}");
        if change == "navigation" {
            assert_eq!(state.ui.symbol.editor.pending_polyline, points);
        } else {
            assert!(
                state.ui.symbol.editor.pending_polyline.is_empty(),
                "{change}"
            );
        }
        apply(&mut state, pending);
        assert_eq!(
            state.load_active_symbol_document().unwrap(),
            before,
            "{change}"
        );
        assert!(!state.can_undo_active_symbol_document(), "{change}");
    }
}
