//! Validate editor request targets and route cross-feature effects through app owners.

use super::parts::CanvasParts;
use crate::workbench::{
    app::delete_schematic_selection,
    framework::requests::{Drain, Request},
};
use rspice_results_ui::selection::SelectedResultTrace;
use rspice_schematic_editor::requests::{EditorAction, EditorRequest};
use rspice_schematic_editor::view::keyboard_navigation;
use rspice_schematic_editor::view::pointer_target::{
    PointerSelectionEffect, select_pointer_target,
};

pub use crate::workbench::app::schematic_editor_request_source as editor_request_source;

pub fn apply_editor_request(state: &mut CanvasParts<'_>, request: EditorRequest) {
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
        }
        EditorAction::SelectPointer {
            target,
            additive,
            alt_held,
        } => {
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
        }
    }
}

/// What the design surface asks of another feature's workspace. The shell
/// answers it once the surface has drawn, in the frame the reader asked.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DesignSurfaceRequest {
    /// Show the operating-point viewer with its device filter set to
    /// `device`, which is empty when the selected report does not hold the
    /// clicked device.
    ShowOperatingPoint { device: String },
    /// Open Verify's Specifications view, for a requirement link.
    OpenSpecifications,
    /// Open the source feature's New Verilog-A device form, for the canvas
    /// menu's New Verilog-A device.
    NewVerilogADevice,
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
            Self::NewVerilogADevice | Self::CopyToEditableLibrary { .. }
        )
    }
}

impl Request for DesignSurfaceRequest {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// What a probe asks of the plot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProbePlot {
    /// Put the trace on the plot; one already there stays.
    Show,
    /// Put the trace on the plot, or take it off when it is there.
    Toggle,
}

/// A trace of the last run a probe named, for the results workspace to plot
/// through the tick its Data Browser uses, and to select. The workspace
/// says what it did, because only it knows whether the trace was already
/// on the plot. It is a request of its own, so no surface request raised
/// later in the frame replaces it. While it waits, the canvas's facts read
/// it before the results workspace's selection, so the frame of the click
/// paints the probe it selected.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlotProbedTrace {
    pub trace: SelectedResultTrace,
    pub plot: ProbePlot,
    /// The signal as the status line names it: the design's spelling.
    pub display: String,
}

impl Request for PlotProbedTrace {
    const DRAIN: Drain = Drain::AfterContribution;
}

/// The voltage between two nets a two-net probe named, `V(a) - V(b)`, for
/// the results workspace to keep on `analysis`'s view as an expression, as
/// the calculator's Plot keeps one, and to say what it did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlotProbedDifference {
    pub analysis: rspice_results::result_presentation::AnalysisPresentationKey,
    /// The difference as the run names its signals.
    pub expression: String,
    /// The same as the status line names it: the design's spelling.
    pub display: String,
}

impl Request for PlotProbedDifference {
    const DRAIN: Drain = Drain::AfterContribution;
}
