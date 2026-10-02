//! Symbol inspector app coordination: validate, record history, publish and route CDF actions.

use crate::diagnostics::ConsoleMessage;
use crate::workbench::AppState;
use egui::Ui;
use rspice_schematic_editor::symbol_editor::inspector::{self, SymbolInspectorRequest};
use rspice_schematic_editor::symbol_editor::interaction::SymbolEditCapabilities;
use rspice_ui_kit::panels::inspector::muted_inspector_copy;

pub(super) fn show(ui: &mut Ui, state: &mut AppState) {
    let ports = state.active_symbol_ports();
    let Ok(document) = state.load_active_symbol_document() else {
        muted_inspector_copy(
            ui,
            "This symbol cellview could not be read. The editor surface reports the exact reason.",
        );
        return;
    };
    let Ok(metadata) = state.load_active_symbol_editor_metadata(&document) else {
        muted_inspector_copy(
            ui,
            "This symbol's authoring metadata could not be read. The editor surface reports the exact reason.",
        );
        return;
    };
    let request = inspector::show(
        ui,
        state.symbol_editor_request_source(),
        document,
        metadata,
        &ports,
        &state.ui.symbol.editor,
        SymbolEditCapabilities {
            edit: !state.schematic_edit_read_only() && !state.application_modal_open(),
        },
    );
    apply_request(state, request);
}

fn apply_request(state: &mut AppState, mut request: SymbolInspectorRequest) {
    if request.source != state.symbol_editor_request_source()
        || request.expected_selection != state.ui.symbol.editor.effective_selection()
        || state.application_modal_open()
    {
        return;
    }
    if (request.changed || !request.undo_before.is_empty()) && state.deny_read_only_edit() {
        return;
    }
    for before in request.undo_before {
        state.record_symbol_edit(&before);
    }
    state.ui.symbol.editor.set_selection(request.selection);
    if request.open_parameter_form {
        crate::workbench::app::open_symbol_parameter_form_dialog(state);
    }
    if request.changed {
        match state.commit_active_symbol_edit(&request.document, &request.metadata, &request.intent)
        {
            Ok(()) => request
                .session
                .accept_source(state.symbol_editor_request_source()),
            Err(error) => state.push_user_message(ConsoleMessage::warning(error)),
        }
    }
    state.ui.symbol.editor.inspector = request.session;
}

#[cfg(test)]
mod tests;
