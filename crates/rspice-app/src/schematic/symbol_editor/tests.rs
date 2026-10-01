//! Cases for the symbol editing surface.

use super::*;
use crate::state::{PortDirection, SymbolPin};

#[test]
fn symbol_editor_has_no_direct_product_key_bypass() {
    let source = include_str!("../symbol_editor.rs");
    assert!(!source.contains(concat!("input.", "key_pressed(")));
    assert!(!source.contains(concat!("ctx.input_mut(|input| input.", "consume_key")));
    assert!(!source.contains(concat!("Use S", " for select")));
    assert!(!source.contains(concat!("Escape", " to cancel")));
}

#[test]
fn finishing_pending_polyline_creates_one_selected_shape() {
    let mut state = AppState::default();
    let mut document = SymbolDocument::default();
    state.ui.symbol.editor.pending_polyline = vec![Point::new(0, 0), Point::new(10, 0)];

    assert!(finish_pending_polyline(&mut state, &mut document));

    assert_eq!(document.body.len(), 1);
    assert_eq!(state.ui.symbol.editor.selected_shape, Some(0));
    assert!(state.ui.symbol.editor.selection.shapes.contains(&0));
    assert!(state.ui.symbol.editor.pending_polyline.is_empty());
}

#[test]
fn place_pin_without_available_pin_does_not_record_undo() {
    let mut state = AppState::default();
    let mut document = SymbolDocument::default();

    assert!(!place_selected_pin(
        &mut state,
        &mut document,
        Point::new(10, 0)
    ));

    assert!(!state.can_undo_active_symbol_document());
    assert!(document.pins.is_empty());
}

#[test]
fn drag_symbol_edit_records_one_undo_snapshot_per_gesture() {
    let mut state = AppState::default();
    let document = SymbolDocument::default();

    record_drag_symbol_edit(&mut state, &document);
    record_drag_symbol_edit(&mut state, &document);

    let key = state.workspace.content.active_key();
    assert_eq!(
        state.ui.symbol.history.undo_depth(&key),
        1,
        "a drag must create one undo transaction no matter how many snap buckets it crosses"
    );
    assert!(state.ui.symbol.editor.drag_undo_recorded);

    state.ui.symbol.editor.clear_drag_state();

    assert!(!state.ui.symbol.editor.drag_undo_recorded);
}

#[test]
fn symbol_canvas_accessibility_label_reports_editing_contract() {
    let mut state = AppState::default();
    state.ui.symbol.editor.tool = SymbolTool::Circle;
    state.ui.symbol.editor.select_shape(0);
    let document = SymbolDocument {
        pins: vec![SymbolPin::new(
            "OUT",
            PortDirection::Out,
            Some(Point::new(20, 0)),
        )],
        body: vec![SymbolShape::Circle {
            center: Point::origin(),
            radius: 10,
        }],
        ..SymbolDocument::default()
    };

    let label = symbol_canvas_accessibility_label(
        &document,
        &state,
        false,
        crate::workbench::commands::vocabulary::CommandPlatform::Desktop,
        egui::os::OperatingSystem::Windows,
    );

    assert!(label.starts_with(
        "Symbol editor canvas. 1 shape; 1 pin, 1 pin placed; 1 item selected. Active tool: Circle. Editable."
    ));
    assert!(label.contains("P: Place symbol pin"));
    assert!(label.contains("Escape: Cancel active command"));
}

/// Shift-click grows the selection instead of replacing it, which is the
/// only way a group drag can be armed at all.
#[test]
fn a_multi_object_grab_moves_the_whole_selection_as_one_edit() {
    let mut state = AppState::default();
    let mut document = SymbolDocument {
        pins: vec![SymbolPin::new(
            "IN",
            PortDirection::In,
            Some(Point::new(-40, 0)),
        )],
        body: vec![SymbolShape::Circle {
            center: Point::new(0, 0),
            radius: 10,
        }],
        ..SymbolDocument::default()
    };
    let mut editor = SymbolEditorMetadata::for_document(&document);
    let mut selection = SymbolSelection::single_pin("IN");
    selection.toggle_shape(0);
    assert_eq!(selection.len(), 2);
    state.ui.symbol.editor.set_selection(selection);

    record_drag_symbol_edit(&mut state, &document);
    translate_selection(&mut state, &mut document, &mut editor, Point::new(10, 20));
    record_drag_symbol_edit(&mut state, &document);

    assert_eq!(
        document.pin("IN").and_then(|pin| pin.position),
        Some(Point::new(-30, 20))
    );
    assert!(matches!(
        document.body.first(),
        Some(SymbolShape::Circle { center, .. }) if *center == Point::new(10, 20)
    ));
    let key = state.workspace.content.active_key();
    assert_eq!(
        state.ui.symbol.history.undo_depth(&key),
        1,
        "a group drag is one undo transaction, not one per moved object"
    );
}
