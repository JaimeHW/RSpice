//! The symbol canvas's retained drafts run through the whole application: a draft
//! survives only while the same view, revision and window stay open, across the
//! project session and the documents it closes.

use super::app_state_of;
use crate::schematic::symbol_editor::interaction::tests::viewport;
use crate::schematic::symbol_editor::interaction::*;
use crate::state::{CellViewRef, Point};
use crate::workbench::AppState;
use crate::workbench::SymbolTool;
use rspice_schematic_editor::symbol_editor::interaction::SymbolCanvasInput;
use rspice_schematic_editor::symbol_editor::interaction::SymbolCanvasRequest;

/// [`open_symbol`](crate::schematic::symbol_editor::interaction::tests::open_symbol) made a whole application.
fn open_symbol() -> AppState {
    app_state_of(crate::schematic::symbol_editor::interaction::tests::open_symbol())
}

fn request(state: &AppState, input: SymbolCanvasInput) -> SymbolCanvasRequest {
    SymbolCanvasRequest {
        source: crate::workbench::app::symbol_editor_request_source(&state.model, &state.workbench),
        tool: state.design.symbol.editor.tool,
        selection: state.design.symbol.editor.effective_selection(),
        viewport: viewport(),
        input,
    }
}

fn apply(state: &mut AppState, request: SymbolCanvasRequest) {
    let mut document = state.model.load_active_symbol_document().unwrap();
    let mut metadata =
        crate::workbench::app::load_active_symbol_editor_metadata(&state.model, &document).unwrap();
    apply_canvas_request(
        &mut state.canvas_parts(),
        &mut document,
        &mut metadata,
        request,
    );
}

#[test]
fn retained_symbol_drafts_require_the_same_open_view_revision_and_window() {
    use crate::workbench::state::WorkspaceDocumentId;

    for change in ["navigation", "revision", "replacement", "close", "window"] {
        let mut state = open_symbol();
        state.open_project_session();
        let symbol = state.model.workspace.content.active_view.clone();
        let before = state.model.load_active_symbol_document().unwrap();
        let points = vec![Point::origin(), Point::new(20, 10)];
        state.design.symbol.editor.tool = SymbolTool::Line;
        state.design.symbol.editor.pending_polyline = points.clone();
        state.design.symbol.editor.dragging_origin = true;
        state.design.symbol.editor.marquee_start = Some(Point::origin());
        let pending = request(
            &state,
            SymbolCanvasInput {
                clicked: true,
                ..Default::default()
            },
        );
        if change == "revision" {
            let mut metadata =
                crate::workbench::app::load_active_symbol_editor_metadata(&state.model, &before)
                    .unwrap();
            metadata.revision += 1;
            state
                .store_active_symbol_editor_bundle(&before, &metadata)
                .unwrap();
        }
        if change == "close" {
            crate::workbench::app_state::services_access::close_active_document(&mut state)
                .unwrap();
            assert!(
                !state
                    .model
                    .workspace
                    .session
                    .symbol_sessions
                    .contains_key(&symbol.key())
            );
        } else {
            state.open_workspace_view(CellViewRef::new("work", "amp", "schematic"));
        }
        if change == "replacement" {
            state.model.design_execution_epoch += 1;
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
        bind_active_canvas(&mut state.canvas_parts());
        assert!(!state.design.symbol.editor.dragging_origin, "{change}");
        assert!(
            state.design.symbol.editor.marquee_start.is_none(),
            "{change}"
        );
        if change == "navigation" {
            assert_eq!(state.design.symbol.editor.pending_polyline, points);
        } else {
            assert!(
                state.design.symbol.editor.pending_polyline.is_empty(),
                "{change}"
            );
        }
        apply(&mut state, pending);
        assert_eq!(
            state.model.load_active_symbol_document().unwrap(),
            before,
            "{change}"
        );
        assert!(!state.can_undo_active_symbol_document(), "{change}");
    }
}
