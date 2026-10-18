//! Validate editor request targets and route cross-feature effects through app owners.

use crate::workbench::app_state::AppState;
use rspice_schematic_editor::requests::{EditorAction, EditorRequest};
use rspice_schematic_editor::view::keyboard_navigation;
use rspice_schematic_editor::view::pointer_target::{
    PointerSelectionEffect, PointerTarget, select_pointer_target,
};

pub(super) use crate::workbench::app::schematic_editor_request_source as editor_request_source;

/// Whether `target` is an instance the user can descend into. Picking one
/// leaves a net highlight lit, so the descent (the descend key, the second
/// click of a double-click, or the menu's Descend) carries it into the
/// instance and back out.
pub(super) fn picks_instance_to_descend(state: &AppState, target: Option<PointerTarget>) -> bool {
    matches!(target, Some(PointerTarget::Component(id))
        if state.hierarchy_master_for_component(id).is_some())
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
            let (document, editor) = state.schematic.document_and_editor();
            keyboard_navigation::focus_keyboard_object(document, &mut editor.selection, object);
            state.dialogs.interaction.schematic_keyboard_focus = Some(object);
            state.schematic.session.editor.net_highlight.clear();
        }
        EditorAction::SelectPointer {
            target,
            additive,
            alt_held,
        } => {
            let kept = (!additive && picks_instance_to_descend(state, target))
                .then(|| state.schematic.session.editor.net_highlight.clone());
            if let PointerSelectionEffect::HighlightWire(id) = select_pointer_target(
                &mut state.schematic.session.editor,
                target,
                additive,
                alt_held,
            ) {
                super::interaction::highlight_canvas_net(state, |net| net.wire_ids.contains(&id));
            }
            if let Some(kept) = kept {
                state.schematic.session.editor.net_highlight = kept;
            }
        }
    }
}
