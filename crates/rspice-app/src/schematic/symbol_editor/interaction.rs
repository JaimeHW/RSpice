//! App validation, undo and publication for symbol canvas requests.

use crate::schematic::view::CanvasParts;
use crate::state::{SymbolDocument, SymbolEditorMetadata};
use crate::ui::widgets::{CanvasEntry, CanvasEntryChoice};
use crate::workbench::app::actions::{
    apply_symbol_edit_request, symbol_pin_name_request, symbol_text_request,
};
use crate::workbench::app::load_active_symbol_editor_metadata;
use rspice_schematic_editor::symbol_editor::SymbolViewport;
use rspice_schematic_editor::symbol_editor::commands::pin_name_refusal;
use rspice_schematic_editor::symbol_editor::interaction::{
    SymbolCanvasInput, SymbolCanvasRequest, SymbolEditCapabilities, bind_canvas_source,
    canvas_request, edit_canvas, track_handle, track_pointer, turn_pin_tool_side,
};
use rspice_schematic_editor::symbol_editor::session::SymbolTool;

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
    track_handle(
        &mut state.design.symbol.editor,
        document,
        viewport,
        response.hover_pos(),
    );
    if let Some(source) = &state.design.symbol.editor.canvas_source
        && let Some(request) =
            canvas_request(source, &state.design.symbol.editor, viewport, response)
        && apply_canvas_request(state, document, metadata, request)
    {
        begin_touch_pan(state, &response.ctx);
    }
}

/// A finger's drag that picked nothing up pans the view, as a schematic's
/// does: the camera catches up with the travel the finger made before the
/// drag was recognised, and the navigation pass follows it from here until
/// it lifts.
fn begin_touch_pan(state: &mut CanvasParts<'_>, ctx: &egui::Context) {
    rspice_schematic_editor::view::selection_drag::set_touch_pan(ctx, true);
    let Some(travel) =
        ctx.input(|input| Some(input.pointer.interact_pos()? - input.pointer.press_origin()?))
    else {
        return;
    };
    match state.symbol_in_place_mut() {
        Some(session) => {
            session.pan.0 += f64::from(travel.x);
            session.pan.1 += f64::from(travel.y);
        }
        None => {
            let editor = &mut state.design.symbol.editor;
            editor.pan.0 += travel.x;
            editor.pan.1 += travel.y;
        }
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

/// The name entry the Pin tool opens at the pin it has just placed. The
/// pin is in `document` under the name it was offered, which the entry
/// holds selected so typing replaces it; Enter keeps what was typed, and
/// Escape or a click elsewhere keeps the name the pin has.
pub(super) fn show_pin_name_entry(
    ctx: &egui::Context,
    state: &mut CanvasParts<'_>,
    viewport: SymbolViewport,
    document: &SymbolDocument,
) {
    let Some(draft) = state.design.symbol.editor.pin_name_draft.as_mut() else {
        return;
    };
    let Some(tip) = document.pin(&draft.pin).and_then(|pin| pin.position) else {
        state.design.symbol.editor.pin_name_draft = None;
        return;
    };
    let field = egui::Id::new("symbol-editor.pin-name-entry");
    let take_caret = !std::mem::replace(&mut draft.caret_placed, true);
    let refusal = (draft.text != draft.pin)
        .then(|| pin_name_refusal(document, &draft.pin, &draft.text))
        .flatten();
    let choice = CanvasEntry::new("symbol-pin-name", "Pin", viewport.world_to_screen(tip))
        .commit_enabled(refusal.is_none())
        .status(refusal.map_or(Ok(None), Err))
        .show(ctx, |rows| {
            rows.text("Name", field, &mut draft.text, "");
            Some(field)
        });
    if take_caret {
        ctx.memory_mut(|memory| memory.request_focus(field));
        let mut edit = egui::text_edit::TextEditState::load(ctx, field).unwrap_or_default();
        edit.cursor
            .set_char_range(Some(egui::text::CCursorRange::two(
                egui::text::CCursor::new(0),
                egui::text::CCursor::new(draft.text.chars().count()),
            )));
        edit.store(ctx, field);
    }
    match choice {
        CanvasEntryChoice::Commit => {
            if let Some(request) =
                symbol_pin_name_request(&mut state.design.symbol, state.model, state.workbench)
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
            state.design.symbol.editor.pin_name_draft = None;
            crate::workbench::app_state::request_engineering_canvas_focus(ctx);
        }
        CanvasEntryChoice::None => {}
    }
}

/// R while the Pin tool is armed on the symbol in front turns the pin it is
/// about to place a quarter clockwise, and while a pin is being dragged it
/// turns that pin about its tip. The key is the pin's rather than Draw
/// rectangle's while the canvas holds the keyboard, or holds the pointer
/// with no field holding the keyboard.
pub(crate) fn turn_pin_being_placed(
    ctx: &egui::Context,
    state: &mut CanvasParts<'_>,
    canvas_has_focus: bool,
) -> bool {
    let editor = &state.design.symbol.editor;
    let over_canvas = editor.pointer.is_some() && ctx.memory(|memory| memory.focused().is_none());
    let placing = editor.tool == SymbolTool::PlacePin && editor.pin_name_draft.is_none();
    let dragging = editor.dragging_pin.is_some();
    if state.model.workspace.content.active_view_type() != Some(crate::state::ViewType::Symbol)
        || !(placing || dragging)
        || !(canvas_has_focus || over_canvas)
        || !ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::R))
    {
        return false;
    }
    if dragging {
        turn_dragged_pin(state);
    } else if let Ok(document) = state.model.load_active_symbol_document() {
        let pointer = state.design.symbol.editor.pointer;
        turn_pin_tool_side(&mut state.design.symbol.editor, &document, pointer);
    }
    ctx.request_repaint();
    true
}

/// The pin being dragged turns about its tip, inside the drag's one edit.
fn turn_dragged_pin(state: &mut CanvasParts<'_>) {
    let Ok(mut document) = state.model.load_active_symbol_document() else {
        return;
    };
    let Ok(mut metadata) = load_active_symbol_editor_metadata(state.model, &document) else {
        return;
    };
    let editor = &state.design.symbol.editor;
    let request = SymbolCanvasRequest {
        source: state.symbol_editor_request_source(),
        tool: editor.tool,
        selection: editor.effective_selection(),
        // The turn reads no pointer: the pin keeps its tip.
        viewport: SymbolViewport::new(egui::Rect::NOTHING, 1.0, egui::Vec2::ZERO),
        input: SymbolCanvasInput {
            dragged: true,
            turn_pin: true,
            ..SymbolCanvasInput::default()
        },
    };
    apply_canvas_request(state, &mut document, &mut metadata, request);
}

/// Why `text` cannot be placed as typed, if it cannot.
fn text_refusal(text: &str) -> Option<&'static str> {
    text.chars()
        .any(char::is_control)
        .then_some("Text cannot hold control characters.")
}

/// Apply `request` to the symbol in front; whether it began a finger's pan.
pub(crate) fn apply_canvas_request(
    state: &mut CanvasParts<'_>,
    document: &mut SymbolDocument,
    metadata: &mut SymbolEditorMetadata,
    request: SymbolCanvasRequest,
) -> bool {
    if request.source != state.symbol_editor_request_source()
        || state.design.symbol.editor.canvas_source.as_ref() != Some(&request.source)
        || request.tool != state.design.symbol.editor.tool
        || request.selection != state.design.symbol.editor.effective_selection()
        || state.application_modal_open()
    {
        return false;
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
        state.record_symbol_edit(&before, &outcome.step);
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
    outcome.pan
}

#[cfg(test)]
pub(crate) mod tests;
