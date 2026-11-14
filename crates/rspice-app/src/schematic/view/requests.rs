//! Validate editor request targets and route cross-feature effects through app owners.

use super::parts::CanvasParts;
use crate::workbench::{
    app::{delete_schematic_selection, hierarchy_master_for_component},
    app_state::Model,
    framework::requests::{Drain, Request},
};
use rspice_results_ui::selection::SelectedResultTrace;
use rspice_schematic_editor::requests::{EditorAction, EditorRequest};
use rspice_schematic_editor::view::keyboard_navigation;
use rspice_schematic_editor::view::pointer_target::{
    PointerSelectionEffect, PointerTarget, select_pointer_target,
};

pub(crate) use crate::workbench::app::schematic_editor_request_source as editor_request_source;

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

/// What the design surface asks of another feature's workspace. The shell
/// answers it once the surface has drawn, in the frame the reader asked.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum DesignSurfaceRequest {
    /// Show the operating-point viewer with its device filter set to
    /// `device`, which is empty when the selected report does not hold the
    /// clicked device.
    ShowOperatingPoint { device: String },
    /// Open the project's specification editor, for a requirement link.
    OpenSpecificationEditor,
    /// Open the code feature's New Verilog-A device form, for the canvas
    /// menu's New Verilog-A device.
    NewVerilogADevice,
    /// Open Create model-bound symbol, starting from the models feature's
    /// selected model if there is one, for the canvas menu's Create symbol
    /// from ports.
    CreateSymbolFromPorts,
    /// Open the generated deck at the selected instance's card, for the
    /// canvas menu's Show in netlist. The canvas has opened the Netlist
    /// workspace already.
    ShowInNetlist,
    /// Open the project's copy-cell dialog on `cell` of `library`, for the
    /// read-only symbol's Copy to editable library.
    CopyToEditableLibrary { library: String, cell: String },
}

impl DesignSurfaceRequest {
    /// Whether answering it opens a dialog that holds the application
    /// modal. While such a request waits, the surface reads its dialog as
    /// open, as it did when it opened the dialog in place.
    pub(crate) const fn opens_dialog(&self) -> bool {
        matches!(
            self,
            Self::NewVerilogADevice
                | Self::CreateSymbolFromPorts
                | Self::CopyToEditableLibrary { .. }
        )
    }
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
