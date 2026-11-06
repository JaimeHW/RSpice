//! Zoom previous on the waveform sheets.
//!
//! A waveform sheet keeps the views it showed, so View ▸ Zoom previous steps
//! it back through them the way it steps a drawing back. One view is
//! everything a wheel, a zoom box, a pan, an axis drag, the zoom keys or a fit
//! move: every strip's shared X window and every unit pane's Y window. The
//! quick view keeps those windows in the session. A project result document
//! keeps the windows of each pane's first unit pane in its own axes and the
//! rest in the session, so its view is read from both and put back into both.
//!
//! The history is what the reader looked at, not what the project holds: it
//! is never saved, and Undo does not step through it.

use std::collections::HashMap;

use egui::Ui;
use rspice_results::visualization_document::{AxisId, AxisRange, DocumentEdit};
use rspice_results_ui::session::ResultViewerState;
use rspice_schematic_editor::view::navigation::ViewHistory;
use rspice_vocabulary::product::ResultDocumentId;

use super::{
    AxisExtent, PlotPresentationKey, PlotView, ResultViewer, ResultsParts, ResultsReads,
    zoom_gesture_available,
};

/// The waveform sheet a history belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum WaveSheet {
    /// The quick view of the retained runs.
    QuickView,
    /// One project result document.
    Document(ResultDocumentId),
}

/// One pinned window in the session's waveform store: the plot and unit pane
/// it belongs to, then its X and Y intervals.
type PaneWindow = (
    (PlotPresentationKey, usize),
    Option<AxisExtent>,
    Option<AxisExtent>,
);

/// What one waveform sheet showed.
#[derive(Debug, Clone, Default)]
struct SheetView {
    /// The windows the session keeps for the sheet, in no particular order.
    windows: Vec<PaneWindow>,
    /// A result document's own axis ranges, in document order.
    axes: Vec<(AxisId, Option<AxisRange>)>,
}

impl PartialEq for SheetView {
    /// The same windows, in whatever order the session's map lists them.
    fn eq(&self, other: &Self) -> bool {
        self.axes == other.axes
            && self.windows.len() == other.windows.len()
            && self
                .windows
                .iter()
                .all(|window| other.windows.contains(window))
    }
}

/// Each waveform sheet's history, and the sheet the workspace last drew.
#[derive(Debug, Clone, Default)]
pub(super) struct WaveViewHistories {
    sheets: HashMap<WaveSheet, ViewHistory<SheetView>>,
    front: Option<WaveSheet>,
}

/// Note the view `sheet` ends the frame on. The workspace calls it once it
/// has drawn, so a wheel, a box, a key, Fit and a menu zoom are all caught
/// here.
pub(super) fn observe(ui: &Ui, state: &mut ResultsParts<'_>, sheet: WaveSheet) {
    let Some(view) = sheet_view(state.results_reads(), sheet) else {
        return;
    };
    let time = ui.input(|input| input.time);
    let histories = &mut state.results.view_histories;
    histories.front = Some(sheet);
    histories
        .sheets
        .entry(sheet)
        .or_default()
        .observe(view, time);
}

/// Whether the waveform sheet in front has an earlier view to go back to.
pub(crate) fn zoom_previous_available(state: ResultsReads<'_>) -> bool {
    let histories = &state.results.view_histories;
    zoom_gesture_available(state)
        && histories
            .front
            .and_then(|sheet| histories.sheets.get(&sheet))
            .is_some_and(ViewHistory::can_step_back)
}

/// Show the view the waveform sheet in front showed before this one.
pub(crate) fn zoom_previous(state: &mut ResultsParts<'_>) {
    let histories = &mut state.results.view_histories;
    let Some(sheet) = histories.front else {
        return;
    };
    let Some(view) = histories
        .sheets
        .get_mut(&sheet)
        .and_then(ViewHistory::step_back)
    else {
        return;
    };
    restore(state, sheet, &view);
}

/// What `sheet` shows now; `None` for a document the project no longer has.
fn sheet_view(state: ResultsReads<'_>, sheet: WaveSheet) -> Option<SheetView> {
    let axes = match sheet {
        WaveSheet::QuickView => Vec::new(),
        WaveSheet::Document(document_id) => state
            .model
            .workspace
            .content
            .visualization_document(document_id)?
            .axes()
            .iter()
            .map(|axis| (axis.id, axis.range))
            .collect(),
    };
    let windows = state
        .results
        .session
        .views
        .iter()
        .filter_map(|(&(viewer, plot, ordinal), view)| {
            (viewer == ResultViewer::Waves
                && kept_in_session(sheet, plot, ordinal)
                && view.is_zoomed())
            .then_some(((plot, ordinal), view.x, view.y))
        })
        .collect();
    Some(SheetView { windows, axes })
}

/// Whether the session keeps `sheet`'s window of unit pane `ordinal` of
/// `plot`. A document pane's first unit pane keeps its windows in the
/// document's axes, which the session only mirrors while the pane draws.
fn kept_in_session(sheet: WaveSheet, plot: PlotPresentationKey, ordinal: usize) -> bool {
    match (sheet, plot) {
        (WaveSheet::QuickView, PlotPresentationKey::Analysis(_)) => true,
        (WaveSheet::Document(document_id), PlotPresentationKey::Document(document, _)) => {
            document == document_id && ordinal != ResultViewerState::RETAINED_PANE_ORDINAL
        }
        _ => false,
    }
}

/// Put `view` back on `sheet`: the session's windows at once, and a
/// document's axis ranges through one transaction, which its panes project
/// from the next frame.
fn restore(state: &mut ResultsParts<'_>, sheet: WaveSheet, view: &SheetView) {
    let views = &mut state.results.session.views;
    views.retain(|&(viewer, plot, ordinal), _| {
        viewer != ResultViewer::Waves || !kept_in_session(sheet, plot, ordinal)
    });
    views.extend(
        view.windows.iter().map(|&((plot, ordinal), x, y)| {
            ((ResultViewer::Waves, plot, ordinal), PlotView { x, y })
        }),
    );
    let WaveSheet::Document(document_id) = sheet else {
        return;
    };
    let Some(document) = state
        .model
        .workspace
        .content
        .visualization_document(document_id)
    else {
        return;
    };
    let edits = view
        .axes
        .iter()
        .filter(|&&(axis_id, range)| {
            document
                .axes()
                .iter()
                .any(|axis| axis.id == axis_id && axis.range != range)
        })
        .map(|&(axis_id, range)| DocumentEdit::SetAxisRange { axis_id, range })
        .collect::<Vec<_>>();
    if edits.is_empty() {
        return;
    }
    let revision = document.revision();
    if let Err(error) = state
        .model
        .workspace
        .content
        .transact_visualization_document(document_id, revision, edits)
    {
        state.prompt(format!("Could not go back to the earlier view: {error}"));
    }
}
