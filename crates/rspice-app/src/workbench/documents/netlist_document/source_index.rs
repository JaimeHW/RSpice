//! Parsed outlines of netlist text, cached against the bytes they were
//! parsed from.

use std::sync::Arc;

use super::AppState;
use crate::state::NetlistSourceIndex;

/// The outline and line offsets of the buffer the editor is showing.
///
/// Parsing costs the whole deck and the navigator reads the outline on every
/// frame, so a large netlist would spend the frame budget re-deriving a
/// structure that did not change. The index is rebuilt only when the bytes
/// differ from the ones it was built from: that comparison is a memcmp, and
/// unlike a revision counter it cannot go stale behind a writer that forgets
/// to bump it.
pub(crate) fn visible_source_index(state: &mut AppState) -> Arc<NetlistSourceIndex> {
    let source = state.simulation.source.netlist_content.as_str();
    if !state.ui.netlist.source_index.describes(source) {
        state.ui.netlist.source_index = Arc::new(NetlistSourceIndex::parse(source));
    }
    Arc::clone(&state.ui.netlist.source_index)
}

/// [`visible_source_index`] for the project's own top-level deck, whatever
/// the editor shows. Until the Netlist workspace has been opened, a restored
/// session or a reopened project has put nothing in the editor, and an
/// include file the editor has open is not the deck.
pub(crate) fn root_deck_source_index(state: &mut AppState) -> Arc<NetlistSourceIndex> {
    let Some(deck) = state.workspace.content.netlist_source.as_deref() else {
        return visible_source_index(state);
    };
    if state.ui.netlist.source_index.describes(deck) {
        return Arc::clone(&state.ui.netlist.source_index);
    }
    if !state.ui.netlist.root_deck_index.describes(deck) {
        state.ui.netlist.root_deck_index = Arc::new(NetlistSourceIndex::parse(deck));
    }
    Arc::clone(&state.ui.netlist.root_deck_index)
}
