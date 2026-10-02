//! Cases for symbol canvas app composition.

use super::*;
use crate::state::{Point, PortDirection, SymbolPin, SymbolShape};
use crate::workbench::SymbolTool;

#[test]
fn symbol_editor_has_no_direct_product_key_bypass() {
    for source in [
        include_str!("../symbol_editor.rs"),
        include_str!("../../../../rspice-schematic-editor/src/symbol_editor/interaction.rs"),
        include_str!("../../../../rspice-schematic-editor/src/symbol_editor/surface.rs"),
    ] {
        assert!(!source.contains(concat!("input.", "key_pressed(")));
        assert!(!source.contains(concat!("ctx.input_mut(|input| input.", "consume_key")));
        assert!(!source.contains(concat!("Use S", " for select")));
        assert!(!source.contains(concat!("Escape", " to cancel")));
    }
}

#[test]
fn symbol_surface_requests_recheck_the_view_and_write_authority() {
    use crate::state::{Cell, CellViewRef, Library, View, ViewType};
    for action in [
        SymbolSurfaceAction::CopyToEditableLibrary,
        SymbolSurfaceAction::GenerateFromSchematic,
    ] {
        for stale_view in [true, false] {
            let mut state = AppState::default();
            let mut library = Library::new("work");
            let mut cell = Cell::new("amp");
            cell.add_view(View::new("schematic", ViewType::Schematic));
            cell.add_view(View::new("symbol", ViewType::Symbol));
            library.add_cell(cell);
            state.library_manager.add_library(library);
            state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
            let before = state.load_active_symbol_document().unwrap();
            let request = SymbolSurfaceRequest {
                source: state.symbol_editor_request_source(),
                action,
            };
            if stale_view {
                state.open_workspace_view(CellViewRef::new("work", "amp", "schematic"));
                state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
            } else {
                state.workspace.content.set_active_read_only_reference(true);
            }
            assert!(
                !apply_surface_request(&mut state, request),
                "{action:?}, stale view: {stale_view}"
            );
            assert!(!state.dialogs.copy_cell_dialog);
            assert_eq!(state.load_active_symbol_document().unwrap(), before);
            assert!(!state.can_undo_active_symbol_document());

            state
                .workspace
                .content
                .set_active_read_only_reference(false);
            let request = SymbolSurfaceRequest {
                source: state.symbol_editor_request_source(),
                action: SymbolSurfaceAction::CopyToEditableLibrary,
            };
            assert!(apply_surface_request(&mut state, request));
            assert!(state.dialogs.copy_cell_dialog);
            assert_eq!(state.dialogs.copy_cell_source_library, "work");
            assert_eq!(state.dialogs.copy_cell_source_cell, "amp");
        }
    }
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
