//! App authority, history and publication for symbol editor commands.

use crate::diagnostics::ConsoleMessage;
use crate::state::Point;
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
    pub(crate) fn activate_symbol_tool(&mut self, tool: SymbolTool) {
        let document = (tool == SymbolTool::PlacePin)
            .then(|| self.state.load_active_symbol_document().ok())
            .flatten();
        commands::activate_tool(&mut self.state.ui.symbol.editor, tool, document.as_ref());
    }

    pub(crate) fn select_all_symbol_items(&mut self) {
        let document = match self.state.load_active_symbol_document() {
            Ok(document) => document,
            Err(error) => {
                self.state.push_user_message(ConsoleMessage::warning(error));
                return;
            }
        };
        self.state
            .ui
            .symbol
            .editor
            .set_selection(SymbolSelection::all_in(&document));
    }

    pub(crate) fn copy_selected_symbol_shape(&mut self) {
        if let Some(clipboard) = self.selected_symbol_items() {
            self.state.ui.symbol.editor.clipboard = clipboard;
        }
    }

    pub(crate) fn paste_symbol_shape(&mut self) {
        let clipboard = self.state.ui.symbol.editor.clipboard.clone();
        if let Some(request) = self.symbol_paste_request(clipboard) {
            apply_edit_request(&mut self.state, request);
        }
    }

    /// Paste a copy of the selection; the user's clipboard is not touched.
    pub(crate) fn duplicate_selected_symbol_shape(&mut self) {
        if let Some(items) = self.selected_symbol_items()
            && let Some(request) = self.symbol_paste_request(items)
        {
            apply_current_edit_request(&mut self.state, request);
        }
    }

    /// The selected pins and shapes, as a clipboard would hold them.
    fn selected_symbol_items(&mut self) -> Option<SymbolClipboard> {
        let document = match self.state.load_active_symbol_document() {
            Ok(document) => document,
            Err(error) => {
                self.state.push_user_message(ConsoleMessage::warning(error));
                return None;
            }
        };
        let selection = self.state.ui.symbol.editor.effective_selection();
        Some(commands::clipboard_from_selection(&document, &selection))
    }

    /// A paste of `items` at the pointer, or at the middle of the view.
    fn symbol_paste_request(&mut self, items: SymbolClipboard) -> Option<SymbolEditRequest> {
        if self.state.deny_read_only_edit() || items.is_empty() {
            return None;
        }
        let target = self
            .state
            .ui
            .canvas_hover
            .or(self.state.ui.canvas_view_center)
            .map(|(x, y)| Point::new(x.round() as i32, y.round() as i32))
            .unwrap_or_else(|| Point::new(10, 10));
        let action = SymbolEditAction::Paste {
            clipboard: items,
            target,
        };
        Some(edit_request(&self.state, action))
    }

    pub(crate) fn delete_selected_symbol_item(&mut self, cut: bool) {
        if self.state.ui.symbol.editor.effective_selection().is_empty()
            || self.state.deny_read_only_edit()
        {
            return;
        }
        let request = edit_request(&self.state, SymbolEditAction::Delete { cut });
        apply_edit_request(&mut self.state, request);
    }

    pub(in crate::workbench) fn transform_selected_symbol_item(
        &mut self,
        transform: SymbolTransform,
    ) {
        if self.state.ui.symbol.editor.effective_selection().is_empty()
            || self.state.deny_read_only_edit()
        {
            return;
        }
        let request = edit_request(&self.state, SymbolEditAction::Transform(transform));
        apply_edit_request(&mut self.state, request);
    }

    pub(crate) fn transform_selected_symbol_pin(&mut self, transform: SymbolPinTransform) {
        if self.state.ui.symbol.editor.effective_selection().pins.len() != 1
            || self.state.deny_read_only_edit()
        {
            return;
        }
        let request = edit_request(&self.state, SymbolEditAction::TransformPin(transform));
        apply_edit_request(&mut self.state, request);
    }

    pub(in crate::workbench) fn finish_pending_symbol_polyline(&mut self) {
        // A command can arrive after navigation and before the new canvas has rendered.
        let source = self.state.symbol_editor_request_source();
        bind_canvas_source(&mut self.state.ui.symbol.editor, source);
        if self.state.ui.symbol.editor.pending_polyline.len() < 2
            || self.state.deny_read_only_edit()
        {
            self.state.ui.symbol.editor.pending_polyline.clear();
            return;
        }
        let points = self.state.ui.symbol.editor.pending_polyline.clone();
        let request = edit_request(&self.state, SymbolEditAction::FinishPolyline { points });
        apply_edit_request(&mut self.state, request);
    }
}

fn edit_request(state: &AppState, action: SymbolEditAction) -> SymbolEditRequest {
    SymbolEditRequest {
        source: state.symbol_editor_request_source(),
        selection: state.ui.symbol.editor.effective_selection(),
        tool: state.ui.symbol.editor.tool,
        action,
    }
}

fn apply_edit_request(state: &mut AppState, request: SymbolEditRequest) {
    if let SymbolEditAction::Paste { clipboard, .. } = &request.action
        && clipboard != &state.ui.symbol.editor.clipboard
    {
        return;
    }
    apply_current_edit_request(state, request);
}

/// Apply `request` while the source, selection and tool it was made against
/// still hold. A duplicate pastes what the selection held when it was asked
/// for rather than the clipboard, so the selection check alone guards it.
fn apply_current_edit_request(state: &mut AppState, request: SymbolEditRequest) {
    if request.source != state.symbol_editor_request_source()
        || request.selection != state.ui.symbol.editor.effective_selection()
        || request.tool != state.ui.symbol.editor.tool
        || state.application_modal_open()
    {
        return;
    }
    if let SymbolEditAction::FinishPolyline { points } = &request.action
        && points != &state.ui.symbol.editor.pending_polyline
    {
        return;
    }
    if state.deny_read_only_edit() {
        return;
    }
    let mut document = match state.load_active_symbol_document() {
        Ok(document) => document,
        Err(error) => {
            state.push_user_message(ConsoleMessage::warning(error));
            return;
        }
    };
    // Completing a line historically stores only geometry. Do not materialize editor metadata.
    let mut metadata = if request.action.document_only() {
        None
    } else {
        match state.load_active_symbol_editor_metadata(&document) {
            Ok(metadata) => Some(metadata),
            Err(error) => {
                state.push_user_message(ConsoleMessage::warning(error));
                return;
            }
        }
    };
    let capabilities = SymbolEditCapabilities {
        edit: !state.schematic_edit_read_only(),
    };
    let outcome = commands::apply_edit(
        &mut state.ui.symbol.editor,
        &mut document,
        metadata.as_mut(),
        request.action,
        capabilities,
    );
    if let Some(before) = outcome.undo_before {
        state.record_symbol_edit(&before);
    }
    if outcome.changed {
        let stored = match metadata {
            Some(metadata) => state.store_active_symbol_editor_bundle(&document, &metadata),
            None => state.store_active_symbol_document(&document),
        };
        if let Err(error) = stored {
            state.push_user_message(ConsoleMessage::warning(error));
        }
    }
}

#[cfg(test)]
mod tests;
