//! The symbol editor's actions run through a whole application.

use crate::state::Point;
use crate::state::{CellViewRef, SymbolEditorMetadata, SymbolShape};
use crate::workbench::app::actions::symbol::tests::open_symbol;
use crate::workbench::app::actions::symbol::*;
use crate::workbench::app_state::AppState;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design::commands::DesignCommand;
use rspice_design::symbol::{SYMBOL_EDITOR_METADATA_KEY, SymbolAttributeKind};
use rspice_schematic_editor::symbol_editor::commands::{SymbolEditAction, SymbolTransform};
use rspice_schematic_editor::symbol_editor::interaction::bind_canvas_source;
use rspice_schematic_editor::symbol_editor::session::{SymbolSelection, SymbolTool};

fn metadata_encoding(state: &AppState) -> Option<String> {
    state
        .model
        .library_manager
        .get_library("work")
        .unwrap()
        .get_cell("amp")
        .unwrap()
        .get_view("symbol")
        .unwrap()
        .metadata
        .get(SYMBOL_EDITOR_METADATA_KEY)
        .cloned()
}

/// Duplicate in the symbol editor copied the selection over the user's
/// clipboard and pasted it from there, so a copy made before was lost. It
/// leaves the clipboard alone, as the schematic's Duplicate does (audit E-15).
#[test]
fn duplicate_leaves_the_symbol_clipboard_alone() {
    let mut app = open_symbol();
    let mut circle = SymbolSelection::default();
    circle.shapes.insert(0);
    app.state.design.symbol.editor.set_selection(circle);
    Command::Copy.execute(&mut app);
    let clipboard = app.state.design.symbol.editor.clipboard.clone();
    assert_eq!(clipboard.shapes.len(), 1);

    let mut dot = SymbolSelection::default();
    dot.shapes.insert(1);
    app.state.design.symbol.editor.set_selection(dot);
    Command::Duplicate.execute(&mut app);

    let document = app.state.model.load_active_symbol_document().unwrap();
    assert_eq!(document.body.len(), 3, "the dot was duplicated");
    assert!(matches!(document.body[2], SymbolShape::Dot { .. }));
    assert_eq!(app.state.design.symbol.editor.clipboard, clipboard);
}

#[test]
fn cut_paste_commands_preserve_order_unique_pin_names_and_undo() {
    let mut app = open_symbol();
    let before = app.state.model.load_active_symbol_document().unwrap();
    let mut selection = SymbolSelection::single_pin("IN");
    selection.shapes.extend([0, 1]);
    app.state.design.symbol.editor.set_selection(selection);
    Command::Cut.execute(&mut app);
    assert_eq!(app.state.design.symbol.editor.clipboard.shapes, before.body);
    assert_eq!(
        app.state.design.symbol.editor.clipboard.pins,
        vec![before.pins[0].clone()]
    );
    assert!(
        app.state
            .model
            .load_active_symbol_document()
            .unwrap()
            .body
            .is_empty()
    );
    assert!(
        app.state
            .design
            .symbol
            .editor
            .effective_selection()
            .is_empty()
    );
    assert!(app.state.undo_active_symbol_document().unwrap());
    assert_eq!(
        app.state.model.load_active_symbol_document().unwrap(),
        before
    );

    // The clipboard's bounds center is (-9, 1); hover wins over viewport center.
    app.state.design.canvas_hover = Some((31.0, 21.0));
    app.state.design.canvas_view_center = Some((100.0, 100.0));
    Command::Paste.execute(&mut app);
    let pasted = app.state.model.load_active_symbol_document().unwrap();
    assert_eq!(pasted.body.len(), 4);
    let pin = pasted.pin("IN_copy2").unwrap();
    assert_eq!(pin.position, Some(Point::new(0, 30)));
    assert_eq!(pin.side(), before.pins[0].side());
    assert_eq!(pin.offset(), 30);
    assert_eq!(
        app.state.design.symbol.editor.effective_selection().shapes,
        [2, 3].into()
    );
    assert_eq!(
        app.state.design.symbol.editor.effective_selection().pins,
        ["IN_copy2".to_owned()].into()
    );
    assert!(app.state.undo_active_symbol_document().unwrap());
    assert_eq!(
        app.state.model.load_active_symbol_document().unwrap(),
        before
    );
    assert!(app.state.redo_active_symbol_document().unwrap());
    assert_eq!(
        app.state.model.load_active_symbol_document().unwrap(),
        pasted
    );
}

#[test]
fn symbol_transforms_keep_geometry_and_metadata_in_one_undo_entry() {
    let mut app = open_symbol();
    let mut before = app.state.model.load_active_symbol_document().unwrap();
    before.origin = Point::new(10, 20);
    before.name_anchor = Point::new(30, 40);
    let metadata = SymbolEditorMetadata::for_document(&before);
    app.state
        .store_active_symbol_editor_bundle(&before, &metadata)
        .unwrap();
    let encoded = metadata_encoding(&app.state);
    let mut selection = SymbolSelection::single_pin("IN");
    selection.shapes.insert(0);
    selection.attributes.insert(SymbolAttributeKind::Reference);
    app.state.design.symbol.editor.set_selection(selection);
    let modal_open = app.state.application_modal_open();
    let state = &mut app.state;
    transform_selected_symbol_item(
        &mut state.design.symbol,
        &mut state.model,
        &mut state.workbench,
        modal_open,
        SymbolTransform::Rotate,
    );
    let after = app.state.model.load_active_symbol_document().unwrap();
    assert_eq!(after.pin("IN").unwrap().position, Some(Point::new(20, -30)));
    assert_eq!(
        after.pin("IN").unwrap().side(),
        before.pin("IN").unwrap().side()
    );
    assert_eq!(after.name_anchor, Point::new(-10, 40));
    assert_eq!(
        crate::workbench::app::load_active_symbol_editor_metadata(&app.state.model, &after)
            .unwrap()
            .attribute(SymbolAttributeKind::Reference)
            .unwrap()
            .position,
        after.name_anchor
    );
    assert_eq!(
        app.state
            .design
            .symbol
            .history
            .undo_depth(&app.state.model.workspace.content.active_key()),
        1
    );
    assert!(app.state.undo_active_symbol_document().unwrap());
    assert_eq!(
        app.state.model.load_active_symbol_document().unwrap(),
        before
    );
    assert_eq!(metadata_encoding(&app.state), encoded);

    app.state.design.symbol.editor.select_pin("IN");
    app.execute_command_key(DesignCommand::SymbolRotatePin.key());
    let rotated = app.state.model.load_active_symbol_document().unwrap();
    assert_eq!(
        rotated.pin("IN").unwrap().side(),
        crate::state::SymbolPinSide::Top
    );
    assert_eq!(rotated.pin("IN").unwrap().offset(), 10);
    app.execute_command_key(DesignCommand::SymbolMirrorPin.key());
    assert_eq!(
        app.state
            .model
            .load_active_symbol_document()
            .unwrap()
            .pin("IN")
            .unwrap()
            .offset(),
        -10
    );
    assert!(app.state.undo_active_symbol_document().unwrap());
    assert_eq!(
        app.state.model.load_active_symbol_document().unwrap(),
        rotated
    );
}

#[test]
fn cancel_finishes_open_geometry_without_materializing_metadata_and_rejects_stale_requests() {
    let mut app = open_symbol();
    let before = app.state.model.load_active_symbol_document().unwrap();
    assert!(metadata_encoding(&app.state).is_none());
    app.state.design.symbol.editor.tool = SymbolTool::Polygon;
    let points = vec![Point::origin(), Point::new(10, 20), Point::new(20, 0)];
    app.state.design.symbol.editor.pending_polyline = points.clone();
    app.execute_shortcut_command(Command::Cancel);
    assert_eq!(
        app.state
            .model
            .load_active_symbol_document()
            .unwrap()
            .body
            .last(),
        Some(&SymbolShape::Polyline {
            points: points.clone(),
            closed: false
        })
    );
    assert!(metadata_encoding(&app.state).is_none());
    assert!(app.state.design.symbol.editor.pending_polyline.is_empty());
    assert_eq!(app.state.design.symbol.editor.tool, SymbolTool::Select);
    assert!(app.state.undo_active_symbol_document().unwrap());
    assert_eq!(
        app.state.model.load_active_symbol_document().unwrap(),
        before
    );

    let source =
        crate::workbench::app::symbol_editor_request_source(&app.state.model, &app.state.workbench);
    bind_canvas_source(&mut app.state.design.symbol.editor, source);
    app.state.design.symbol.editor.tool = SymbolTool::Polygon;
    app.state.design.symbol.editor.pending_polyline = points.clone();
    let pending = symbol_edit_request(
        &app.state.design.symbol,
        &app.state.model,
        &app.state.workbench,
        SymbolEditAction::FinishPolyline {
            points: points.clone(),
        },
    );
    app.state
        .open_workspace_view(CellViewRef::new("work", "amp", "schematic"));
    app.state
        .open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    let modal_open = app.state.application_modal_open();
    apply_symbol_edit_request(
        &mut app.state.design.symbol,
        &mut app.state.model,
        &mut app.state.workbench,
        modal_open,
        pending,
    );
    assert_eq!(
        app.state.model.load_active_symbol_document().unwrap(),
        before
    );
    assert_eq!(app.state.design.symbol.editor.pending_polyline, points);
    assert!(!app.state.can_undo_active_symbol_document());
}
