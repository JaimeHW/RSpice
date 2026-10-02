//! Validate editor request targets and route cross-feature effects through app owners.

use crate::workbench::app_state::AppState;
use rspice_schematic_editor::requests::{EditorAction, EditorRequest};
use rspice_schematic_editor::view::keyboard_navigation;
use rspice_schematic_editor::view::pointer_target::{
    PointerSelectionEffect, select_pointer_target,
};

pub(super) use crate::workbench::app::schematic_editor_request_source as editor_request_source;

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
