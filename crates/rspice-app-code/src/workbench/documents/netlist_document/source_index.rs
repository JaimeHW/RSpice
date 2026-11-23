//! Outlines of netlist text, read off the engine's index of it and built
//! away from the editor's thread while the writer types.

use std::path::PathBuf;
use std::sync::Arc;

use rspice_core::netlist::SourceKind;

use super::dialect::ShownSource;
use super::index_worker::BackgroundIndex;
use super::{Model, NetlistDocumentState};
use crate::state::NetlistSourceIndex;

/// An outline kept against the exact text it was built from, and where that
/// text is read from, which with the text decides how the engine reads it.
///
/// The text is the key: comparing it is a memcmp, and unlike a revision
/// counter it cannot go stale behind a writer that forgets to bump one. How
/// the engine reads the text is worked out only for a build, since deciding
/// it reads the whole text; the key and the build share one copy of it.
pub(crate) type SourceIndexSlot =
    BackgroundIndex<(Option<ShownSource>, Arc<str>), NetlistSourceIndex>;

/// How the engine reads `text` shown from `shown`: as the root deck, an
/// include file, or Spectre. Text shown from no source is read as a deck.
fn kind_of(shown: Option<&ShownSource>, text: &str) -> SourceKind {
    shown.map_or(SourceKind::DeckRoot, |shown| shown.kind(text))
}

/// The outline of `source` from `slot` while the writer types it: the one
/// built for it, else the last one built while it builds.
fn latest_in(
    ctx: &egui::Context,
    slot: &mut SourceIndexSlot,
    shown: Option<&ShownSource>,
    source: &str,
) -> Arc<NetlistSourceIndex> {
    if slot.wants(|built| built.0.as_ref() == shown && *built.1 == *source) {
        let text: Arc<str> = Arc::from(source);
        let (read, from) = (Arc::clone(&text), shown.cloned());
        let wake = ctx.clone();
        slot.start(
            (shown.cloned(), text),
            move || NetlistSourceIndex::parse_as(&read, kind_of(from.as_ref(), &read)),
            move || wake.request_repaint(),
        );
    }
    slot.last_built()
        .unwrap_or_else(|| now_in(slot, shown, source))
}

fn now_in(
    slot: &mut SourceIndexSlot,
    shown: Option<&ShownSource>,
    source: &str,
) -> Arc<NetlistSourceIndex> {
    slot.wait_for(|built| built.0.as_ref() == shown && *built.1 == *source)
        .unwrap_or_else(|| {
            slot.store(
                (shown.cloned(), Arc::from(source)),
                NetlistSourceIndex::parse_as(source, kind_of(shown, source)),
            )
        })
}

/// The file the deck is read from: the open deck's, else the project's first
/// deck's. Its extension tells a `.scs` deck from a SPICE one.
pub(super) fn root_deck_path(model: &Model, netlist: &NetlistDocumentState) -> PathBuf {
    if let Some(deck) = &netlist.open_deck {
        return deck.root.path.clone();
    }
    let content = &model.workspace.content;
    content
        .decks
        .first()
        .map(|deck| {
            deck.file_path(content.project.data_root())
                .unwrap_or_else(|| PathBuf::from(deck.name()))
        })
        .unwrap_or_default()
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
    let shown = super::dialect::shown_source_in(model, netlist);
    latest_in(ctx, &mut netlist.source_index, shown.as_ref(), source)
}

/// [`visible_source_index_in`] of the buffer as it stands, waiting for its
/// build to finish.
pub fn visible_source_index_now(
    model: &Model,
    netlist: &mut NetlistDocumentState,
) -> Arc<NetlistSourceIndex> {
    let source = model.simulation.source.netlist_content.as_str();
    let shown = super::dialect::shown_source_in(model, netlist);
    now_in(&mut netlist.source_index, shown.as_ref(), source)
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
    let shown = model.simulation.source.netlist_content.as_str();
    match netlist
        .open_deck
        .as_ref()
        .map(|deck| deck.root.text.as_str())
    {
        Some(deck) if deck != shown => {
            let root = ShownSource {
                path: root_deck_path(model, netlist),
                root: true,
            };
            now_in(&mut netlist.root_deck_index, Some(&root), deck)
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
    let shown = model.simulation.source.netlist_content.as_str();
    match netlist
        .open_deck
        .as_ref()
        .map(|deck| deck.root.text.as_str())
    {
        Some(deck) if deck != shown => {
            let root = ShownSource {
                path: root_deck_path(model, netlist),
                root: true,
            };
            now_in(&mut netlist.root_deck_index, Some(&root), deck)
        }
        _ => visible_source_index_now(model, netlist),
    }
}
