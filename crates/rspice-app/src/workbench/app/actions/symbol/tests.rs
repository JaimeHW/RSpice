//! Commands publish through the app's symbol transaction owner and reject stale input.

use super::*;
use crate::state::{
    Cell, CellViewRef, Library, PortDirection, SymbolDocument, SymbolPin, SymbolShape, View,
    ViewType,
};
use crate::workbench::RSpiceApp;

pub(crate) fn open_symbol() -> RSpiceApp {
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
    app.state.model.library_manager.add_library(library);
    app.state
        .open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    app.state
        .workbench
        .activate(crate::workbench::state::Workspace::Design);
    app
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
        app.state.design.symbol.editor.select_shape(0);
        copy_selected_symbol_shape(
            &mut app.state.design.symbol,
            &app.state.model,
            &mut app.state.workbench,
        );
        let request = symbol_edit_request(
            &app.state.design.symbol,
            &app.state.model,
            &app.state.workbench,
            SymbolEditAction::Paste {
                clipboard: app.state.design.symbol.editor.clipboard.clone(),
                target: Point::new(100, 100),
            },
        );
        let before = app.state.model.load_active_symbol_document().unwrap();
        match change {
            "revision" => app.state.store_active_symbol_document(&before).unwrap(),
            "view" => {
                app.state
                    .open_workspace_view(CellViewRef::new("work", "amp", "schematic"));
                app.state
                    .open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
            }
            "selection" => app.state.design.symbol.editor.select_shape(1),
            "tool" => app.state.design.symbol.editor.tool = SymbolTool::Text,
            "clipboard" => app.state.design.symbol.editor.clipboard.shapes.clear(),
            "reference" => app
                .state
                .model
                .workspace
                .content
                .set_active_read_only_reference(true),
            "session" => app.state.model.schematic.session.read_only = true,
            "modal" => app.state.settings.preferences_open = true,
            _ => unreachable!(),
        }
        let selection = app.state.design.symbol.editor.effective_selection();
        let clipboard = app.state.design.symbol.editor.clipboard.clone();
        let modal_open = app.state.application_modal_open();
        apply_symbol_edit_request(
            &mut app.state.design.symbol,
            &mut app.state.model,
            &mut app.state.workbench,
            modal_open,
            request,
        );
        assert_eq!(
            app.state.model.load_active_symbol_document().unwrap(),
            before,
            "{change}"
        );
        assert_eq!(
            app.state.design.symbol.editor.effective_selection(),
            selection,
            "{change}"
        );
        assert_eq!(
            app.state.design.symbol.editor.clipboard, clipboard,
            "{change}"
        );
        assert!(!app.state.can_undo_active_symbol_document(), "{change}");
    }
}
