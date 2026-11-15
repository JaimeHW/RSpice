//! The symbol surface's requests answered through the whole application: each one
//! rechecks the open view and the project's write authority before it acts.

use crate::workbench::AppState;
use rspice_app_design::schematic::symbol_editor::*;
use rspice_schematic_editor::symbol_editor::surface::SymbolSurfaceAction;
use rspice_schematic_editor::symbol_editor::surface::SymbolSurfaceRequest;

#[test]
fn symbol_surface_requests_recheck_the_view_and_write_authority() {
    use rspice_app_state::state::{Cell, CellViewRef, Library, View, ViewType};
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
            state.model.library_manager.add_library(library);
            state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
            let before = state.model.load_active_symbol_document().unwrap();
            let request = SymbolSurfaceRequest {
                source: rspice_app_design::workbench::app::actions::workspace::symbol_document::symbol_editor_request_source(
                    &state.model,
                    &state.workbench,
                ),
                action,
            };
            if stale_view {
                state.open_workspace_view(CellViewRef::new("work", "amp", "schematic"));
                state.open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
            } else {
                state
                    .model
                    .workspace
                    .content
                    .set_active_read_only_reference(true);
            }
            assert!(
                !apply_surface_request(&mut state.canvas_frame(), request),
                "{action:?}, stale view: {stale_view}"
            );
            assert!(!state.project.copy_cell_dialog);
            assert_eq!(state.model.load_active_symbol_document().unwrap(), before);
            assert!(!state.can_undo_active_symbol_document());

            state
                .model
                .workspace
                .content
                .set_active_read_only_reference(false);
            let request = SymbolSurfaceRequest {
                source: rspice_app_design::workbench::app::actions::workspace::symbol_document::symbol_editor_request_source(
                    &state.model,
                    &state.workbench,
                ),
                action: SymbolSurfaceAction::CopyToEditableLibrary,
            };
            assert!(apply_surface_request(&mut state.canvas_frame(), request));
            crate::workbench::commands::answer_surface_requests(&mut state);
            assert!(state.project.copy_cell_dialog);
            assert_eq!(state.project.copy_cell_source_library, "work");
            assert_eq!(state.project.copy_cell_source_cell, "amp");
        }
    }
}
