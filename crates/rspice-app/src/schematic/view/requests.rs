//! Validate editor request targets and route cross-feature effects through app owners.

use crate::workbench::app_state::AppState;
use rspice_schematic_editor::requests::{EditorAction, EditorRequest, EditorRequestSource};
use rspice_schematic_editor::view::keyboard_navigation;
use rspice_schematic_editor::view::pointer_target::{
    PointerSelectionEffect, select_pointer_target,
};

pub(super) fn editor_request_source(state: &AppState) -> EditorRequestSource {
    let document = state.workspace.content.active_schematic_reference();
    let document_key = document.key();
    let sheet = state
        .workspace
        .content
        .design_management
        .sheet_catalog(&document_key)
        .map(|catalog| (catalog.active_sheet_id(), catalog.revision()));
    EditorRequestSource {
        project: state.workspace.content.project.id(),
        document,
        occurrence: state.workspace.content.active_occurrence().cloned(),
        design_epoch: state.design_execution_epoch,
        document_epoch: state.active_schematic_epoch,
        content_version: state.schematic.content_version(),
        topology_version: state.schematic.topology_version(),
        symbol_revision: super::symbol_context_revision(state),
        sheet,
    }
}

pub(super) fn apply_editor_request(state: &mut AppState, request: EditorRequest) {
    if request.source != editor_request_source(state)
        || request.selection != state.schematic.session.editor.selection
        || state.schematic.session.editor.tool != crate::state::Tool::Select
        || state.application_modal_open()
    {
        return;
    }
    match request.action {
        EditorAction::DeleteSelection => {
            if !state.schematic.session.read_only && !state.active_view_read_only() {
                state.delete_schematic_selection();
            }
        }
        EditorAction::Focus(object) => {
            let (document, selection) = state.schematic.document_and_selection();
            keyboard_navigation::focus_keyboard_object(document, selection, object);
            state.dialogs.interaction.schematic_keyboard_focus = Some(object);
            state.schematic.session.editor.net_highlight.clear();
        }
        EditorAction::SelectPointer {
            target,
            additive,
            alt_held,
        } => {
            if let PointerSelectionEffect::HighlightWire(id) = select_pointer_target(
                &mut state.schematic.session.editor,
                target,
                additive,
                alt_held,
            ) {
                super::interaction::highlight_canvas_net(state, |net| net.wire_ids.contains(&id));
            }
        }
    }
}
