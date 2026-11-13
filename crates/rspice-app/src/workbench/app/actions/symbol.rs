//! App authority, history and publication for symbol editor commands.

use crate::state::Point;
use crate::workbench::app::actions::workspace::deny_read_only_edit;
use crate::workbench::app::actions::workspace::symbol_document::{
    SymbolUiState, load_active_symbol_editor_metadata, record_symbol_edit,
    store_active_symbol_document, store_active_symbol_editor_bundle, symbol_editor_request_source,
};
use crate::workbench::app_state::Model;
use crate::workbench::state::WorkbenchSession;
use crate::workbench::{AppState, RSpiceApp};
use rspice_schematic_editor::symbol_editor::commands::{
    self, SymbolEditAction, SymbolEditRequest, SymbolPinTransform, SymbolTransform,
};
use rspice_schematic_editor::symbol_editor::interaction::{
    SymbolEditCapabilities, bind_canvas_source,
};
use rspice_schematic_editor::symbol_editor::session::{
    SymbolClipboard, SymbolSelection, SymbolTool,
};

impl RSpiceApp {
    pub(in crate::workbench) fn finish_pending_symbol_polyline(&mut self) {
        // A command can arrive after navigation and before the new canvas has rendered.
        let source = self.state.symbol_editor_request_source();
        bind_canvas_source(&mut self.state.design.symbol.editor, source);
        if self.state.design.symbol.editor.pending_polyline.len() < 2
            || self.state.deny_read_only_edit()
        {
            self.state.design.symbol.editor.pending_polyline.clear();
            return;
        }
        let points = self.state.design.symbol.editor.pending_polyline.clone();
        let request = edit_request(&self.state, SymbolEditAction::FinishPolyline { points });
        apply_edit_request(&mut self.state, request);
    }
}

/// Arm `tool` on the symbol canvas. A pin starts from the document in front,
/// so the tool offers the next pin it would place.
pub(crate) fn activate_symbol_tool(symbol: &mut SymbolUiState, model: &Model, tool: SymbolTool) {
    let document = (tool == SymbolTool::PlacePin)
        .then(|| model.load_active_symbol_document().ok())
        .flatten();
    commands::activate_tool(&mut symbol.editor, tool, document.as_ref());
}

/// Turn the one selected pin to the next side, or across the body, as one
/// edit. `modal_open` is whether anything holding the application modal is
/// open, which refuses the edit.
pub(crate) fn transform_selected_symbol_pin(
    symbol: &mut SymbolUiState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    modal_open: bool,
    transform: SymbolPinTransform,
) {
    if symbol.editor.effective_selection().pins.len() != 1 || deny_read_only_edit(model, wb) {
        return;
    }
    let request = symbol_edit_request(symbol, model, wb, SymbolEditAction::TransformPin(transform));
    apply_symbol_edit_request(symbol, model, wb, modal_open, request);
}

/// Turn or flip the selected pins and shapes as one edit. `modal_open` is
/// whether anything holding the application modal is open, which refuses
/// the edit.
pub(crate) fn transform_selected_symbol_item(
    symbol: &mut SymbolUiState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    modal_open: bool,
    transform: SymbolTransform,
) {
    if symbol.editor.effective_selection().is_empty() || deny_read_only_edit(model, wb) {
        return;
    }
    let request = symbol_edit_request(symbol, model, wb, SymbolEditAction::Transform(transform));
    apply_symbol_edit_request(symbol, model, wb, modal_open, request);
}

pub(crate) fn select_all_symbol_items(
    symbol: &mut SymbolUiState,
    model: &Model,
    wb: &mut WorkbenchSession,
) {
    let document = match model.load_active_symbol_document() {
        Ok(document) => document,
        Err(error) => {
            wb.prompt(error);
            return;
        }
    };
    symbol
        .editor
        .set_selection(SymbolSelection::all_in(&document));
}

pub(crate) fn copy_selected_symbol_shape(
    symbol: &mut SymbolUiState,
    model: &Model,
    wb: &mut WorkbenchSession,
) {
    if let Some(clipboard) = selected_symbol_items(symbol, model, wb) {
        symbol.editor.clipboard = clipboard;
    }
}

/// Paste the symbol editor's clipboard at the pointer, or at the middle of
/// the view. `modal_open` is whether anything holding the application modal
/// is open, which refuses the edit.
pub(crate) fn paste_symbol_shape(
    symbol: &mut SymbolUiState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    canvas_hover: Option<(f64, f64)>,
    canvas_view_center: Option<(f64, f64)>,
    modal_open: bool,
) {
    let clipboard = symbol.editor.clipboard.clone();
    if let Some(request) = symbol_paste_request(
        symbol,
        model,
        wb,
        canvas_hover,
        canvas_view_center,
        clipboard,
    ) {
        apply_symbol_edit_request(symbol, model, wb, modal_open, request);
    }
}

/// Paste a copy of the selection; the user's clipboard is not touched.
pub(crate) fn duplicate_selected_symbol_shape(
    symbol: &mut SymbolUiState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    canvas_hover: Option<(f64, f64)>,
    canvas_view_center: Option<(f64, f64)>,
    modal_open: bool,
) {
    if let Some(items) = selected_symbol_items(symbol, model, wb)
        && let Some(request) =
            symbol_paste_request(symbol, model, wb, canvas_hover, canvas_view_center, items)
    {
        apply_current_symbol_edit_request(symbol, model, wb, modal_open, request);
    }
}

/// The selected pins and shapes, as a clipboard would hold them.
fn selected_symbol_items(
    symbol: &SymbolUiState,
    model: &Model,
    wb: &mut WorkbenchSession,
) -> Option<SymbolClipboard> {
    let document = match model.load_active_symbol_document() {
        Ok(document) => document,
        Err(error) => {
            wb.prompt(error);
            return None;
        }
    };
    let selection = symbol.editor.effective_selection();
    Some(commands::clipboard_from_selection(&document, &selection))
}

/// A paste of `items` at the pointer, or at the middle of the view.
fn symbol_paste_request(
    symbol: &SymbolUiState,
    model: &Model,
    wb: &mut WorkbenchSession,
    canvas_hover: Option<(f64, f64)>,
    canvas_view_center: Option<(f64, f64)>,
    items: SymbolClipboard,
) -> Option<SymbolEditRequest> {
    if deny_read_only_edit(model, wb) || items.is_empty() {
        return None;
    }
    let target = canvas_hover
        .or(canvas_view_center)
        .map(|(x, y)| Point::new(x.round() as i32, y.round() as i32))
        .unwrap_or_else(|| Point::new(10, 10));
    let action = SymbolEditAction::Paste {
        clipboard: items,
        target,
    };
    Some(symbol_edit_request(symbol, model, wb, action))
}

/// Delete the selected pins and shapes; `cut` copies them first.
pub(crate) fn delete_selected_symbol_item(
    symbol: &mut SymbolUiState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    modal_open: bool,
    cut: bool,
) {
    if symbol.editor.effective_selection().is_empty() || deny_read_only_edit(model, wb) {
        return;
    }
    let request = symbol_edit_request(symbol, model, wb, SymbolEditAction::Delete { cut });
    apply_symbol_edit_request(symbol, model, wb, modal_open, request);
}

/// [`symbol_edit_request`] over the session.
fn edit_request(state: &AppState, action: SymbolEditAction) -> SymbolEditRequest {
    symbol_edit_request(&state.design.symbol, &state.model, &state.workbench, action)
}

fn symbol_edit_request(
    symbol: &SymbolUiState,
    model: &Model,
    wb: &WorkbenchSession,
    action: SymbolEditAction,
) -> SymbolEditRequest {
    SymbolEditRequest {
        source: symbol_editor_request_source(model, wb),
        selection: symbol.editor.effective_selection(),
        tool: symbol.editor.tool,
        action,
    }
}

/// [`apply_symbol_edit_request`] over the session, asking its modal gate.
fn apply_edit_request(state: &mut AppState, request: SymbolEditRequest) {
    let modal_open = state.application_modal_open();
    apply_symbol_edit_request(
        &mut state.design.symbol,
        &mut state.model,
        &mut state.workbench,
        modal_open,
        request,
    );
}

fn apply_symbol_edit_request(
    symbol: &mut SymbolUiState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    modal_open: bool,
    request: SymbolEditRequest,
) {
    if let SymbolEditAction::Paste { clipboard, .. } = &request.action
        && clipboard != &symbol.editor.clipboard
    {
        return;
    }
    apply_current_symbol_edit_request(symbol, model, wb, modal_open, request);
}

/// Apply `request` while the source, selection and tool it was made against
/// still hold and nothing holds the application modal (`modal_open`). A
/// duplicate pastes what the selection held when it was asked for rather
/// than the clipboard, so the selection check alone guards it.
fn apply_current_symbol_edit_request(
    symbol: &mut SymbolUiState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    modal_open: bool,
    request: SymbolEditRequest,
) {
    if request.source != symbol_editor_request_source(model, wb)
        || request.selection != symbol.editor.effective_selection()
        || request.tool != symbol.editor.tool
        || modal_open
    {
        return;
    }
    if let SymbolEditAction::FinishPolyline { points } = &request.action
        && points != &symbol.editor.pending_polyline
    {
        return;
    }
    if deny_read_only_edit(model, wb) {
        return;
    }
    let mut document = match model.load_active_symbol_document() {
        Ok(document) => document,
        Err(error) => {
            wb.prompt(error);
            return;
        }
    };
    // Completing a line historically stores only geometry. Do not materialize editor metadata.
    let mut metadata = if request.action.document_only() {
        None
    } else {
        match load_active_symbol_editor_metadata(model, &document) {
            Ok(metadata) => Some(metadata),
            Err(error) => {
                wb.prompt(error);
                return;
            }
        }
    };
    let capabilities = SymbolEditCapabilities {
        edit: !model.schematic_edit_read_only(),
    };
    let outcome = commands::apply_edit(
        &mut symbol.editor,
        &mut document,
        metadata.as_mut(),
        request.action,
        capabilities,
    );
    if let Some(before) = outcome.undo_before {
        record_symbol_edit(symbol, model, &before);
    }
    if outcome.changed {
        let stored = match metadata {
            Some(metadata) => {
                store_active_symbol_editor_bundle(symbol, model, wb, &document, &metadata)
            }
            None => store_active_symbol_document(symbol, model, &document),
        };
        if let Err(error) = stored {
            wb.prompt(error);
        }
    }
}

#[cfg(test)]
mod tests;
