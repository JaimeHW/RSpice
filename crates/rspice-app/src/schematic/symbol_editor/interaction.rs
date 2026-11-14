//! App validation, undo and publication for symbol canvas requests.

use crate::schematic::view::CanvasParts;
use crate::state::{SymbolDocument, SymbolEditorMetadata};
use crate::ui::widgets::{CanvasEntry, CanvasEntryChoice};
use crate::workbench::app::actions::{apply_symbol_edit_request, symbol_text_request};
use rspice_schematic_editor::symbol_editor::SymbolViewport;
use rspice_schematic_editor::symbol_editor::interaction::{
    SymbolCanvasRequest, SymbolEditCapabilities, bind_canvas_source, canvas_request, edit_canvas,
    track_pointer,
};

pub(crate) fn bind_active_canvas(state: &mut CanvasParts<'_>) {
    let source = state.symbol_editor_request_source();
    bind_canvas_source(&mut state.design.symbol.editor, source);
}

pub(super) fn handle_canvas(
    state: &mut CanvasParts<'_>,
    document: &mut SymbolDocument,
    metadata: &mut SymbolEditorMetadata,
    viewport: SymbolViewport,
    response: &egui::Response,
) {
    let center = viewport.screen_to_world(viewport.rect.center());
    state.design.canvas_view_center = Some((center.x as f64, center.y as f64));
    state.design.canvas_hover = response.hover_pos().map(|position| {
        let point = viewport.screen_to_world(position);
        (point.x as f64, point.y as f64)
    });
    track_pointer(
        &mut state.design.symbol.editor,
        viewport,
        response.hover_pos(),
    );
    if let Some(source) = &state.design.symbol.editor.canvas_source
        && let Some(request) =
            canvas_request(source, &state.design.symbol.editor, viewport, response)
    {
        apply_canvas_request(state, document, metadata, request);
    }
}

/// The entry the text tool opens where it was clicked. What is typed shows
/// on the canvas as it will stand; Enter places it as one edit, and Escape or
/// a click elsewhere lets it go with nothing placed.
pub(super) fn show_text_entry(
    ctx: &egui::Context,
    state: &mut CanvasParts<'_>,
    viewport: SymbolViewport,
) {
    let Some(draft) = state.design.symbol.editor.text_draft.as_mut() else {
        return;
    };
    let field = egui::Id::new("symbol-editor.text-entry");
    let anchor = viewport.world_to_screen(draft.anchor);
    // The canvas takes the keyboard on the click that opened the entry; the
    // entry takes it back once, so the reader can type at once.
    let take_caret = !std::mem::replace(&mut draft.caret_placed, true);
    let refusal = text_refusal(&draft.text);
    let choice = CanvasEntry::new("symbol-text", "Text", anchor)
        .commit_enabled(refusal.is_none() && !draft.text.trim().is_empty())
        .status(refusal.map_or(Ok(None), Err))
        .show(ctx, |rows| {
            rows.text("Text", field, &mut draft.text, "");
            Some(field)
        });
    if take_caret {
        ctx.memory_mut(|memory| memory.request_focus(field));
    }
    match choice {
        CanvasEntryChoice::Commit => {
            if let Some(request) =
                symbol_text_request(&mut state.design.symbol, state.model, state.workbench)
            {
                let modal_open = state.application_modal_open();
                apply_symbol_edit_request(
                    &mut state.design.symbol,
                    state.model,
                    state.workbench,
                    modal_open,
                    request,
                );
            }
            crate::workbench::app_state::request_engineering_canvas_focus(ctx);
        }
        CanvasEntryChoice::Cancelled => {
            state.design.symbol.editor.text_draft = None;
            crate::workbench::app_state::request_engineering_canvas_focus(ctx);
        }
        CanvasEntryChoice::None => {}
    }
}

/// Why `text` cannot be placed as typed, if it cannot.
fn text_refusal(text: &str) -> Option<&'static str> {
    text.chars()
        .any(char::is_control)
        .then_some("Text cannot hold control characters.")
}

pub(crate) fn apply_canvas_request(
    state: &mut CanvasParts<'_>,
    document: &mut SymbolDocument,
    metadata: &mut SymbolEditorMetadata,
    request: SymbolCanvasRequest,
) {
    if request.source != state.symbol_editor_request_source()
        || state.design.symbol.editor.canvas_source.as_ref() != Some(&request.source)
        || request.tool != state.design.symbol.editor.tool
        || request.selection != state.design.symbol.editor.effective_selection()
        || state.application_modal_open()
    {
        return;
    }
    let capabilities = SymbolEditCapabilities {
        edit: !state.schematic_edit_read_only(),
    };
    let outcome = edit_canvas(
        &mut state.design.symbol.editor,
        document,
        metadata,
        request.viewport,
        request.input,
        capabilities,
    );
    if outcome.edit_denied {
        state.deny_read_only_edit();
    }
    if let Some(refusal) = outcome.refusal {
        state.prompt(refusal);
    }
    if let Some(before) = outcome.undo_before {
        // A new gesture: whatever an earlier one held open is over.
        state.finish_symbol_parent_edit();
        state.record_symbol_edit(&before);
    }
    if outcome.changed {
        if let Err(error) = state.store_symbol_canvas_frame(document, metadata) {
            state.prompt(error);
        } else {
            // Advancing our own revision keeps every event of this drag on one undo entry.
            state.design.symbol.editor.canvas_source = Some(state.symbol_editor_request_source());
        }
    }
    // A drag reaches the parent schematics as one edit, closed when it ends;
    // a click is an edit of its own.
    if !request.input.dragged || request.input.drag_stopped {
        state.finish_symbol_parent_edit();
    }
}

#[cfg(test)]
pub(crate) mod tests;
