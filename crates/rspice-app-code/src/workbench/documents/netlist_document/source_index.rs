//! Parsed outlines of netlist text, cached against the bytes they were
//! parsed from.

use std::sync::Arc;

use super::{Model, NetlistDocumentState};
use crate::state::NetlistSourceIndex;

/// The outline and line offsets of the buffer the editor is showing.
///
/// Parsing costs the whole deck and the navigator reads the outline on every
/// frame, so a large netlist would spend the frame budget re-deriving a
/// structure that did not change. The index is rebuilt only when the bytes
/// differ from the ones it was built from: that comparison is a memcmp, and
/// unlike a revision counter it cannot go stale behind a writer that forgets
/// to bump it. It reads the model and keeps the index in the netlist
/// editor's state.
pub fn visible_source_index_in(
    model: &Model,
    netlist: &mut NetlistDocumentState,
) -> Arc<NetlistSourceIndex> {
    let source = model.simulation.source.netlist_content.as_str();
    if !netlist.source_index.describes(source) {
        netlist.source_index = Arc::new(NetlistSourceIndex::parse(source));
    }
    Arc::clone(&netlist.source_index)
}

/// [`visible_source_index_in`] for the project's own top-level deck, whatever
/// the editor shows. Until the Netlist workspace has been opened, a restored
/// session or a reopened project has put nothing in the editor, and an
/// include file the editor has open is not the deck.
pub fn root_deck_source_index(
    model: &Model,
    netlist: &mut NetlistDocumentState,
) -> Arc<NetlistSourceIndex> {
    let Some(deck) = model.workspace.content.netlist_source.as_deref() else {
        return visible_source_index_in(model, netlist);
    };
    if netlist.source_index.describes(deck) {
        return Arc::clone(&netlist.source_index);
    }
    if !netlist.root_deck_index.describes(deck) {
        netlist.root_deck_index = Arc::new(NetlistSourceIndex::parse(deck));
    }
    Arc::clone(&netlist.root_deck_index)
}
