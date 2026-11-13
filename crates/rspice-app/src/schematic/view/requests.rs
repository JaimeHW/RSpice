//! Validate editor request targets and route cross-feature effects through app owners.

use super::parts::CanvasParts;
use crate::workbench::{
    ResultViewer,
    app::{delete_schematic_selection, hierarchy_master_for_component},
    app_state::{AppState, Model},
    documents::result_document::open_specification_editor,
    framework::requests::{Drain, Request},
};
use rspice_results_ui::selection::SelectedResultTrace;
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
pub(super) fn picks_instance_to_descend(model: &Model, target: Option<PointerTarget>) -> bool {
    matches!(target, Some(PointerTarget::Component(id))
        if hierarchy_master_for_component(model, id).is_some())
}

pub(super) fn apply_editor_request(state: &mut CanvasParts<'_>, request: EditorRequest) {
    if request.source != editor_request_source(state)
        || request.selection != state.model.schematic.session.editor.selection
        || state.model.schematic.session.editor.tool != crate::state::Tool::Select
        || state.application_modal_open()
    {
        return;
    }
    match request.action {
        EditorAction::DeleteSelection => {
            if !state.model.schematic.session.read_only && !state.active_view_read_only() {
                delete_schematic_selection(state.model, state.workbench);
            }
        }
        EditorAction::Focus(object) => {
            let (document, editor) = state.model.schematic.document_and_editor();
            keyboard_navigation::focus_keyboard_object(document, &mut editor.selection, object);
            state.design.interaction.schematic_keyboard_focus = Some(object);
            state.model.schematic.session.editor.net_highlight.clear();
        }
        EditorAction::SelectPointer {
            target,
            additive,
            alt_held,
        } => {
            let kept = (!additive && picks_instance_to_descend(state.model, target))
                .then(|| state.model.schematic.session.editor.net_highlight.clone());
            if let PointerSelectionEffect::HighlightWire(id) = select_pointer_target(
                &mut state.model.schematic.session.editor,
                target,
                additive,
                alt_held,
            ) {
                super::interaction::highlight_canvas_net(state.model, |net| {
                    net.wire_ids.contains(&id)
                });
            }
            if let Some(kept) = kept {
                state.model.schematic.session.editor.net_highlight = kept;
            }
        }
    }
}

/// What the canvas asks of the results workspace. The shell answers it with
/// [`answer_surface_request`] once the design surface has drawn, in the
/// frame the reader asked.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum DesignSurfaceRequest {
    /// Show the operating-point viewer with its device filter set to
    /// `device`, which is empty when the selected report does not hold the
    /// clicked device.
    ShowOperatingPoint { device: String },
    /// Open the project's specification editor, for a requirement link.
    OpenSpecificationEditor,
}

impl Request for DesignSurfaceRequest {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// The trace a probe selected, for the results workspace to select. It is
/// a request of its own, so no surface request raised later in the frame
/// replaces it. While it waits, the canvas's facts read it before the
/// results workspace's selection, so the frame of the click paints the
/// probe it selected.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SelectProbedTrace(pub(crate) SelectedResultTrace);

impl Request for SelectProbedTrace {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// Carry out what the canvas asked of the results workspace, as the canvas
/// once did in place.
pub(crate) fn answer_surface_request(state: &mut AppState, request: DesignSurfaceRequest) {
    match request {
        DesignSurfaceRequest::ShowOperatingPoint { device } => {
            let session = &mut state.results.results.session;
            session.op_filter = device;
            session.viewer = ResultViewer::Op;
        }
        DesignSurfaceRequest::OpenSpecificationEditor => {
            open_specification_editor(&mut state.results_parts());
        }
    }
}

/// Select the trace a probe selected.
pub(crate) fn select_probed_trace(state: &mut AppState, probed: SelectProbedTrace) {
    let SelectProbedTrace(trace) = probed;
    state.results.results.session.selected_trace = Some(trace);
}

/// Answer what the canvas raised, as the shell does once the surface has
/// drawn.
#[cfg(test)]
pub(crate) fn answer_surface_requests(state: &mut AppState) {
    use crate::workbench::framework::requests::drain;
    drain(
        state,
        |state| &mut state.workbench.outbox,
        select_probed_trace,
    );
    drain(
        state,
        |state| &mut state.workbench.outbox,
        answer_surface_request,
    );
}
