//! Commands publish through the app's symbol transaction owner and reject stale input.

use super::*;
use crate::state::{
    Cell, CellViewRef, Library, PortDirection, SymbolDocument, SymbolEditorMetadata, SymbolPin,
    SymbolShape, View, ViewType,
};
use crate::workbench::commands::vocabulary::Command;
use rspice_design::symbol::{SYMBOL_EDITOR_METADATA_KEY, SymbolAttributeKind};

fn open_symbol() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let mut library = Library::new("work");
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    let mut view = View::new("symbol", ViewType::Symbol);
    SymbolDocument {
        pins: vec![
            SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, 10))),
            SymbolPin::new("in_COPY", PortDirection::In, None),
        ],
        body: vec![
            SymbolShape::Circle {
                center: Point::origin(),
                radius: 10,
            },
            SymbolShape::Dot {
                center: Point::new(20, 10),
                radius: 2,
            },
        ],
        ..SymbolDocument::default()
    }
    .store_in_view(&mut view)
    .unwrap();
    cell.add_view(view);
    library.add_cell(cell);
    app.state.library_manager.add_library(library);
    app.state
        .open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    app.state
        .workbench
        .activate(crate::workbench::state::Workspace::Design);
    app
}

fn metadata_encoding(state: &AppState) -> Option<String> {
    state
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

#[test]
fn cut_paste_commands_preserve_order_unique_pin_names_and_undo() {
    let mut app = open_symbol();
    let before = app.state.load_active_symbol_document().unwrap();
    let mut selection = SymbolSelection::single_pin("IN");
    selection.shapes.extend([0, 1]);
    app.state.ui.symbol.editor.set_selection(selection);
    Command::Cut.execute(&mut app);
    assert_eq!(app.state.ui.symbol.editor.clipboard.shapes, before.body);
    assert_eq!(
        app.state.ui.symbol.editor.clipboard.pins,
        vec![before.pins[0].clone()]
    );
    assert!(
        app.state
            .load_active_symbol_document()
            .unwrap()
            .body
            .is_empty()
    );
    assert!(app.state.ui.symbol.editor.effective_selection().is_empty());
    assert!(app.state.undo_active_symbol_document().unwrap());
    assert_eq!(app.state.load_active_symbol_document().unwrap(), before);

    // The clipboard's bounds center is (-9, 1); hover wins over viewport center.
    app.state.ui.canvas_hover = Some((31.0, 21.0));
    app.state.ui.canvas_view_center = Some((100.0, 100.0));
    Command::Paste.execute(&mut app);
    let pasted = app.state.load_active_symbol_document().unwrap();
    assert_eq!(pasted.body.len(), 4);
    let pin = pasted.pin("IN_copy2").unwrap();
    assert_eq!(pin.position, Some(Point::new(0, 30)));
    assert_eq!(pin.side(), before.pins[0].side());
    assert_eq!(pin.offset(), 30);
    assert_eq!(
        app.state.ui.symbol.editor.effective_selection().shapes,
        [2, 3].into()
    );
    assert_eq!(
        app.state.ui.symbol.editor.effective_selection().pins,
        ["IN_copy2".to_owned()].into()
    );
    assert!(app.state.undo_active_symbol_document().unwrap());
    assert_eq!(app.state.load_active_symbol_document().unwrap(), before);
    assert!(app.state.redo_active_symbol_document().unwrap());
    assert_eq!(app.state.load_active_symbol_document().unwrap(), pasted);
}

#[test]
fn symbol_transforms_keep_geometry_and_metadata_in_one_undo_entry() {
    let mut app = open_symbol();
    let mut before = app.state.load_active_symbol_document().unwrap();
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
    app.state.ui.symbol.editor.set_selection(selection);
    app.transform_selected_symbol_item(SymbolTransform::Rotate);
    let after = app.state.load_active_symbol_document().unwrap();
    assert_eq!(after.pin("IN").unwrap().position, Some(Point::new(20, -30)));
    assert_eq!(
        after.pin("IN").unwrap().side(),
        before.pin("IN").unwrap().side()
    );
    assert_eq!(after.name_anchor, Point::new(-10, 40));
    assert_eq!(
        app.state
            .load_active_symbol_editor_metadata(&after)
            .unwrap()
            .attribute(SymbolAttributeKind::Reference)
            .unwrap()
            .position,
        after.name_anchor
    );
    assert_eq!(
        app.state
            .ui
            .symbol
            .history
            .undo_depth(&app.state.workspace.content.active_key()),
        1
    );
    assert!(app.state.undo_active_symbol_document().unwrap());
    assert_eq!(app.state.load_active_symbol_document().unwrap(), before);
    assert_eq!(metadata_encoding(&app.state), encoded);

    app.state.ui.symbol.editor.select_pin("IN");
    Command::SymbolRotatePin.execute(&mut app);
    let rotated = app.state.load_active_symbol_document().unwrap();
    assert_eq!(
        rotated.pin("IN").unwrap().side(),
        crate::state::SymbolPinSide::Top
    );
    assert_eq!(rotated.pin("IN").unwrap().offset(), 10);
    Command::SymbolMirrorPin.execute(&mut app);
    assert_eq!(
        app.state
            .load_active_symbol_document()
            .unwrap()
            .pin("IN")
            .unwrap()
            .offset(),
        -10
    );
    assert!(app.state.undo_active_symbol_document().unwrap());
    assert_eq!(app.state.load_active_symbol_document().unwrap(), rotated);
}

#[test]
fn queued_symbol_commands_recheck_source_selection_tool_clipboard_and_authority() {
    for change in [
        "revision",
        "view",
        "selection",
        "tool",
        "clipboard",
        "reference",
        "session",
        "modal",
    ] {
        let mut app = open_symbol();
        app.state.ui.symbol.editor.select_shape(0);
        app.copy_selected_symbol_shape();
        let request = edit_request(
            &app.state,
            SymbolEditAction::Paste {
                clipboard: app.state.ui.symbol.editor.clipboard.clone(),
                target: Point::new(100, 100),
            },
        );
        let before = app.state.load_active_symbol_document().unwrap();
        match change {
            "revision" => app.state.store_active_symbol_document(&before).unwrap(),
            "view" => {
                app.state
                    .open_workspace_view(CellViewRef::new("work", "amp", "schematic"));
                app.state
                    .open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
            }
            "selection" => app.state.ui.symbol.editor.select_shape(1),
            "tool" => app.state.ui.symbol.editor.tool = SymbolTool::Text,
            "clipboard" => app.state.ui.symbol.editor.clipboard.shapes.clear(),
            "reference" => app
                .state
                .workspace
                .content
                .set_active_read_only_reference(true),
            "session" => app.state.schematic.session.read_only = true,
            "modal" => app.state.dialogs.preferences_open = true,
            _ => unreachable!(),
        }
        let selection = app.state.ui.symbol.editor.effective_selection();
        let clipboard = app.state.ui.symbol.editor.clipboard.clone();
        apply_edit_request(&mut app.state, request);
        assert_eq!(
            app.state.load_active_symbol_document().unwrap(),
            before,
            "{change}"
        );
        assert_eq!(
            app.state.ui.symbol.editor.effective_selection(),
            selection,
            "{change}"
        );
        assert_eq!(app.state.ui.symbol.editor.clipboard, clipboard, "{change}");
        assert!(!app.state.can_undo_active_symbol_document(), "{change}");
    }
}

#[test]
fn cancel_finishes_open_geometry_without_materializing_metadata_or_crossing_views() {
    let mut app = open_symbol();
    let before = app.state.load_active_symbol_document().unwrap();
    assert!(metadata_encoding(&app.state).is_none());
    app.state.ui.symbol.editor.tool = SymbolTool::Polygon;
    let points = vec![Point::origin(), Point::new(10, 20), Point::new(20, 0)];
    app.state.ui.symbol.editor.pending_polyline = points.clone();
    app.execute_symbol_shortcut_command(Command::Cancel);
    assert_eq!(
        app.state.load_active_symbol_document().unwrap().body.last(),
        Some(&SymbolShape::Polyline {
            points: points.clone(),
            closed: false
        })
    );
    assert!(metadata_encoding(&app.state).is_none());
    assert!(app.state.ui.symbol.editor.pending_polyline.is_empty());
    assert_eq!(app.state.ui.symbol.editor.tool, SymbolTool::Select);
    assert!(app.state.undo_active_symbol_document().unwrap());
    assert_eq!(app.state.load_active_symbol_document().unwrap(), before);

    let source = app.state.symbol_editor_request_source();
    bind_canvas_source(&mut app.state.ui.symbol.editor, source);
    app.state.ui.symbol.editor.pending_polyline = points;
    app.state
        .open_workspace_view(CellViewRef::new("work", "amp", "schematic"));
    app.state
        .open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    app.execute_symbol_shortcut_command(Command::Cancel);
    assert_eq!(app.state.load_active_symbol_document().unwrap(), before);
    assert!(app.state.ui.symbol.editor.pending_polyline.is_empty());
    assert!(!app.state.can_undo_active_symbol_document());
}
