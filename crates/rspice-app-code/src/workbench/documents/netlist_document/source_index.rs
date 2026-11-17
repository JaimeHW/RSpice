//! Outlines of netlist text, read off the engine's index of it and built
//! away from the editor's thread while the writer types.

use std::sync::Arc;

use super::index_worker::BackgroundIndex;
use super::{Model, NetlistDocumentState};
use crate::state::NetlistSourceIndex;

/// An outline kept against the exact text it was built from.
///
/// The text is the key: comparing it is a memcmp, and unlike a revision
/// counter it cannot go stale behind a writer that forgets to bump one.
pub(crate) type SourceIndexSlot = BackgroundIndex<String, NetlistSourceIndex>;

/// The outline of `source` from `slot` while the writer types it: the one
/// built for it, else the last one built while it builds.
fn latest_in(
    ctx: &egui::Context,
    slot: &mut SourceIndexSlot,
    source: &str,
) -> Arc<NetlistSourceIndex> {
    if slot.wants(|built| built == source) {
        let text = source.to_owned();
        let wake = ctx.clone();
        slot.start(
            source.to_owned(),
            move || NetlistSourceIndex::parse(&text),
            move || wake.request_repaint(),
        );
    }
    slot.last_built().unwrap_or_else(|| now_in(slot, source))
}

fn now_in(slot: &mut SourceIndexSlot, source: &str) -> Arc<NetlistSourceIndex> {
    slot.wait_for(|built| built == source)
        .unwrap_or_else(|| slot.store(source.to_owned(), NetlistSourceIndex::parse(source)))
}

/// The outline and line offsets of the buffer the editor is showing, for a
/// frame. While the writer types, the last outline built stands in until the
/// one of the buffer as it stands is built; text put in the buffer any other
/// way (a file opened, a deck loaded) is outlined before the frame shows it,
/// so a frame never shows another document's outline. It reads the model and
/// keeps the outline in the netlist editor's state.
pub fn visible_source_index_in(
    ctx: &egui::Context,
    model: &Model,
    netlist: &mut NetlistDocumentState,
) -> Arc<NetlistSourceIndex> {
    if !netlist.is_being_typed() {
        return visible_source_index_now(model, netlist);
    }
    let source = model.simulation.source.netlist_content.as_str();
    latest_in(ctx, &mut netlist.source_index, source)
}

/// [`visible_source_index_in`] of the buffer as it stands, waiting for its
/// build to finish.
pub fn visible_source_index_now(
    model: &Model,
    netlist: &mut NetlistDocumentState,
) -> Arc<NetlistSourceIndex> {
    now_in(
        &mut netlist.source_index,
        model.simulation.source.netlist_content.as_str(),
    )
}

/// [`visible_source_index_in`] for the project's own top-level deck, whatever
/// the editor shows. Until the Netlist workspace has been opened, a restored
/// session or a reopened project has put nothing in the editor, and an
/// include file the editor has open is not the deck. A deck the editor is
/// not showing is not being typed, so it is outlined as it stands.
pub fn root_deck_source_index(
    ctx: &egui::Context,
    model: &Model,
    netlist: &mut NetlistDocumentState,
) -> Arc<NetlistSourceIndex> {
    match model.workspace.content.netlist_source.as_deref() {
        Some(deck) if deck != model.simulation.source.netlist_content => {
            now_in(&mut netlist.root_deck_index, deck)
        }
        _ => visible_source_index_in(ctx, model, netlist),
    }
}

/// [`root_deck_source_index`] of the deck as it stands, waiting for its build
/// to finish.
pub fn root_deck_source_index_now(
    model: &Model,
    netlist: &mut NetlistDocumentState,
) -> Arc<NetlistSourceIndex> {
    match model.workspace.content.netlist_source.as_deref() {
        Some(deck) if deck != model.simulation.source.netlist_content => {
            now_in(&mut netlist.root_deck_index, deck)
        }
        _ => visible_source_index_now(model, netlist),
    }
}
