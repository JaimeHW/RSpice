//! Symbol inspector app coordination: validate, record history, publish and route CDF actions.

use crate::workbench::app::actions::workspace::deny_read_only_edit;
use crate::workbench::app::actions::workspace::symbol_document::{
    active_symbol_ports, commit_active_symbol_edit, load_active_symbol_editor_metadata,
    record_symbol_edit, symbol_editor_request_source,
};
use crate::workbench::design::DesignFeature;
use crate::workbench::framework::feature::Cx;
use egui::Ui;
use rspice_schematic_editor::symbol_editor::inspector::{self, SymbolInspectorRequest};
use rspice_schematic_editor::symbol_editor::interaction::SymbolEditCapabilities;
use rspice_ui_kit::panels::inspector::muted_inspector_copy;

pub(in crate::workbench) fn show(ui: &mut Ui, cx: &mut Cx<'_, DesignFeature>) {
    let ports = active_symbol_ports(cx.model);
    let Ok(document) = cx.model.load_active_symbol_document() else {
        muted_inspector_copy(
            ui,
            "This symbol could not be read. The symbol editor shows why.",
        );
        return;
    };
    let Ok(metadata) = load_active_symbol_editor_metadata(cx.model, &document) else {
        muted_inspector_copy(
            ui,
            "This symbol's editor data could not be read. The symbol editor shows why.",
        );
        return;
    };
    let request = inspector::show(
        ui,
        symbol_editor_request_source(cx.model, cx.wb),
        document,
        metadata,
        &ports,
        &cx.state.symbol.editor,
        SymbolEditCapabilities {
            edit: !cx.model.schematic_edit_read_only() && !cx.application_modal_open(),
        },
    );
    apply_request(cx, request);
}

fn apply_request(cx: &mut Cx<'_, DesignFeature>, mut request: SymbolInspectorRequest) {
    if request.source != symbol_editor_request_source(cx.model, cx.wb)
        || request.expected_selection != cx.state.symbol.editor.effective_selection()
        || cx.application_modal_open()
    {
        return;
    }
    if (request.changed || !request.undo_before.is_empty()) && deny_read_only_edit(cx.model, cx.wb)
    {
        return;
    }
    for before in request.undo_before {
        record_symbol_edit(&mut cx.state.symbol, cx.model, &before);
    }
    cx.state.symbol.editor.set_selection(request.selection);
    if request.open_parameter_form {
        crate::workbench::app::open_symbol_parameter_form_dialog(
            &mut cx.state.symbol_parameter_form,
            cx.model,
            cx.wb,
            cx.platform,
        );
    }
    if request.changed {
        match commit_active_symbol_edit(
            &mut cx.state.symbol,
            cx.model,
            cx.wb,
            &request.document,
            &request.metadata,
            &request.intent,
        ) {
            Ok(()) => request
                .session
                .accept_source(symbol_editor_request_source(cx.model, cx.wb)),
            Err(error) => cx.wb.prompt(error),
        }
    }
    cx.state.symbol.editor.inspector = request.session;
}

#[cfg(test)]
mod tests;
