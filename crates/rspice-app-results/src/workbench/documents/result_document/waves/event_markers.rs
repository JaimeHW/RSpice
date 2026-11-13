//! The strip's event markers as the Results workspace hosts them: the run's
//! HDL severity records and finish, its assertion violations and a stop time
//! with expected outputs beyond it, on the panes' time axis.
//!
//! A marker moves the cursor the lanes' edge step moves, so stepping along
//! edges and along markers is one gesture. Opening a marker's source line is
//! the application's: the strip leaves the request in
//! [`ResultsState::source_request`](super::super::ResultsState) for it.
use super::*;
use crate::ui::plot::CursorPair;
use rspice_results_ui::waves::logic::{self, MarkerStripInput};
use rspice_results_ui::waves::stack::EventLaneFrame;

/// Height of `model`'s marker strip: none when its run has no marker.
pub(super) fn marker_height(model: &StripModel) -> f32 {
    model
        .event_markers
        .as_deref()
        .map_or(0.0, logic::markers_height)
}

/// Draw `model`'s marker strip on the panes' time axis and apply what the
/// reader asked of it.
pub(super) fn show_markers(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    model: &StripModel,
    frame: EventLaneFrame,
) {
    let Some(markers) = model.event_markers.clone() else {
        return;
    };
    let digits = usize::from(
        state
            .results_reads()
            .presentation_policy()
            .readout()
            .displayed_significant_digits()
            .get(),
    );
    let session = &mut state.results.session;
    let cursors = if session.cursor_strip == Some(model.analysis_index) {
        session.cursors
    } else {
        CursorPair::default()
    };
    let response = logic::show_marker_strip(
        ui,
        &MarkerStripInput {
            markers: &markers,
            x_range: frame.x_range,
            left_margin: frame.left_margin,
            name_width: frame.name_width,
            right_margin: frame.right_margin,
            cursor: cursors.a,
            cursor_b: cursors.b,
        },
        &|t| rspice_results_ui::waves::fmt_in_unit_readout(t, &model.x_unit, digits),
    );
    if let Some(t) = response.cursor_to {
        super::lanes::step_cursor(session, model.analysis_index, t);
    }
    if let Some(source) = response.open_source {
        state.results.source_request = Some(source);
    }
}
