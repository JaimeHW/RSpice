//! App validation, undo and publication for symbol canvas requests.

use crate::diagnostics::ConsoleMessage;
use crate::state::{SymbolDocument, SymbolEditorMetadata};
use crate::workbench::AppState;
use rspice_schematic_editor::symbol_editor::SymbolViewport;
use rspice_schematic_editor::symbol_editor::interaction::{
    SymbolCanvasRequest, SymbolEditCapabilities, SymbolRequestSource, bind_canvas_source,
    canvas_request, edit_canvas,
};

fn request_source(state: &AppState) -> SymbolRequestSource {
    SymbolRequestSource {
        project: state.workspace.content.project.id(),
        document: state.workspace.content.active_view.clone(),
        occurrence: state.workspace.content.active_occurrence().cloned(),
        design_epoch: state.design_execution_epoch,
        document_epoch: state.active_schematic_epoch,
        library_revision: state.library_manager.revision(),
    }
}

pub(super) fn bind_active_canvas(state: &mut AppState) {
    let source = request_source(state);
    bind_canvas_source(&mut state.ui.symbol.editor, source);
}

pub(super) fn handle_canvas(
    state: &mut AppState,
    document: &mut SymbolDocument,
    metadata: &mut SymbolEditorMetadata,
    viewport: SymbolViewport,
    response: &egui::Response,
) {
    let center = viewport.screen_to_world(viewport.rect.center());
    state.ui.canvas_view_center = Some((center.x as f64, center.y as f64));
    state.ui.canvas_hover = response.hover_pos().map(|position| {
        let point = viewport.screen_to_world(position);
        (point.x as f64, point.y as f64)
    });
    if let Some(source) = &state.ui.symbol.editor.canvas_source
        && let Some(request) = canvas_request(source, &state.ui.symbol.editor, viewport, response)
    {
        apply_canvas_request(state, document, metadata, request);
    }
}

fn apply_canvas_request(
    state: &mut AppState,
    document: &mut SymbolDocument,
    metadata: &mut SymbolEditorMetadata,
    request: SymbolCanvasRequest,
) {
    if request.source != request_source(state)
        || state.ui.symbol.editor.canvas_source.as_ref() != Some(&request.source)
        || request.tool != state.ui.symbol.editor.tool
        || request.selection != state.ui.symbol.editor.effective_selection()
        || state.application_modal_open()
    {
        return;
    }
    let capabilities = SymbolEditCapabilities {
        edit: !state.schematic_edit_read_only(),
    };
    let outcome = edit_canvas(
        &mut state.ui.symbol.editor,
        document,
        metadata,
        request.viewport,
        request.input,
        capabilities,
    );
    if outcome.edit_denied {
        state.deny_read_only_edit();
    }
    if let Some(before) = outcome.undo_before {
        state.record_symbol_edit(&before);
    }
    if outcome.changed {
        if let Err(error) = state.store_active_symbol_editor_bundle(document, metadata) {
            state.push_user_message(ConsoleMessage::warning(error));
        } else {
            // Advancing our own revision keeps every event of this drag on one undo entry.
            state.ui.symbol.editor.canvas_source = Some(request_source(state));
        }
    }
}

#[cfg(test)]
mod tests;
