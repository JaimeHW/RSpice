//! The keys that act where the pointer stands over a strip's plot, as
//! ViVA's bindkeys do: A and B put a cursor there, H a horizontal cursor,
//! V, M and D a marker, L a note, and Q opens the properties of the marker
//! nearest it. From a menu or the palette, with the pointer on no plot,
//! each places what it names where the Cursors page's Add would.

use super::readout::with_readout;
use crate::ui::tokens::Tokens;
use crate::workbench::documents::result_document::{ResultsParts, ResultsReads};
use rspice_results_ui::waves::pointer::{self, AtPointer};
use rspice_ui_kit::plot::CursorId;

/// Whether `action` can run now, and why not.
pub(crate) fn availability(state: ResultsReads<'_>, action: AtPointer) -> Result<(), &'static str> {
    if !super::super::zoom_gesture_available(state) {
        return Err("open a waveform sheet in Results");
    }
    if action == AtPointer::Properties && state.results.session.document_markers.is_empty() {
        return Err("place a marker first");
    }
    Ok(())
}

/// Do what `action` names where the pointer stands, or says why not.
pub(crate) fn act(state: &mut ResultsParts<'_>, action: AtPointer) {
    let tokens = Tokens::default();
    match action {
        AtPointer::CursorA | AtPointer::CursorB => {
            let id = if action == AtPointer::CursorA {
                CursorId::A
            } else {
                CursorId::B
            };
            if !with_readout(state, &tokens, true, |session, source| {
                pointer::place_cursor(session, source, id)
            }) {
                state.prompt("There is no waveform strip to put the cursor on.");
            }
        }
        AtPointer::HorizontalCursor => {
            if !with_readout(state, &tokens, true, pointer::place_horizontal) {
                state
                    .prompt("Both horizontal cursors of the pane are placed; drag one to move it.");
            }
        }
        AtPointer::VerticalMarker | AtPointer::PointMarker | AtPointer::DeltaMarker => {
            let Some(spot) = with_readout(state, &tokens, true, |session, source| {
                pointer::marker_spot(session, source)
            }) else {
                state.prompt("There is no curve to put the marker on.");
                return;
            };
            let analysis = spot.analysis;
            let placement = super::super::MarkerPlacement {
                analysis,
                anchor: spot.anchor,
                trace_name: spot.trace_name,
                x: spot.x,
                samples: &spot.samples,
            };
            // A key places the marker and leaves its dialog to Q: the reader
            // pressing it is reading the plot, not filling in a form.
            let Some(selector) = super::super::place_marker(state, placement) else {
                return;
            };
            match action {
                AtPointer::VerticalMarker => {
                    if let Err(error) = super::super::commit_marker_edit(
                        state,
                        selector,
                        "",
                        super::super::MarkerKind::Vertical,
                    ) {
                        state.prompt(format!("Could not make the marker vertical: {error}"));
                    }
                }
                AtPointer::DeltaMarker => {
                    pointer::read_delta_against_base(
                        &mut state.results.session,
                        analysis,
                        selector,
                    );
                }
                _ => {}
            }
        }
        AtPointer::Note => {
            let Some(draft) = with_readout(state, &tokens, true, |session, source| {
                pointer::note_draft(session, source)
            }) else {
                state.prompt("There is no waveform strip to write the note on.");
                return;
            };
            if state.results.session.pane_for(draft.analysis).is_none() {
                state.prompt("No view keeps this strip, so the note was not added.");
                return;
            }
            state.results.session.note_draft = Some(draft);
        }
        AtPointer::Properties => match with_readout(state, &tokens, true, |session, source| {
            pointer::nearest_marker(session, source)
        }) {
            Some(selector) => super::marker_dialog::open(state, selector),
            None => state.prompt("There is no marker on this strip."),
        },
    }
}
