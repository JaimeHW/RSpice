//! Outlines of netlist text, read off the engine's index of it and built
//! away from the editor's thread.

use std::path::PathBuf;
use std::sync::Arc;

use rspice_core::netlist::SourceKind;

use super::dialect::ShownSource;
use super::index_worker::BackgroundIndex;
use super::{Model, NetlistDocumentState};
use crate::state::{NetlistSourceIndex, RevisedText};

/// An outline kept against the revision of the text it was built from, and
/// where that text is read from, which with the text decides how the engine
/// reads it.
///
/// The revision is the key. Every write to a [`RevisedText`] takes a new
/// one, so an outline kept for a revision is the outline of exactly that
/// text, and a frame checks it by comparing two numbers, not the deck. How
/// the engine reads the text is worked out only for a build, since deciding
/// it reads the whole text; the build reads the text shared, not copied, and
/// puts it in one piece there rather than on the frame.
pub(crate) type SourceIndexSlot = BackgroundIndex<(Option<ShownSource>, u64), NetlistSourceIndex>;

/// How the engine reads `text` shown from `shown`: as the root deck, an
/// include file, or Spectre. Text shown from no source is read as a deck.
fn kind_of(shown: Option<&ShownSource>, text: &str) -> SourceKind {
    shown.map_or(SourceKind::DeckRoot, |shown| shown.kind(text))
}

/// The outline of `source` from `slot` for a frame, with whether it is the
/// outline of `source` as it stands. A build of it starts when it is neither
/// built nor building, away from the frame. While the writer types
/// (`typed`), the last outline built stands in until it is built; text put
/// in the buffer any other way has no outline until its own is built, so a
/// frame never shows another text's outline.
fn latest_in(
    ctx: &egui::Context,
    slot: &mut SourceIndexSlot,
    shown: Option<&ShownSource>,
    source: &RevisedText,
    typed: bool,
) -> Option<(Arc<NetlistSourceIndex>, bool)> {
    let revision = source.revision();
    let is_current =
        |built: &(Option<ShownSource>, u64)| built.1 == revision && built.0.as_ref() == shown;
    if slot.wants(is_current) {
        let (read, from) = (source.clone(), shown.cloned());
        // The outline of the same source a moment ago, read again only where
        // the edit since touched it.
        let last = slot
            .built()
            .filter(|(built, _)| built.0.as_ref() == shown)
            .map(|(built, index)| (built.1, index));
        let wake = ctx.clone();
        slot.start(
            (shown.cloned(), revision),
            move || match last {
                Some((built, last)) => edited_from(&last, built, from.as_ref(), read),
                None => index_of(from.as_ref(), read),
            },
            move || wake.request_repaint(),
        );
    }
    let (key, index) = slot.built()?;
    let current = is_current(key);
    (current || typed).then_some((index, current))
}

/// The index of `text` shown from `shown`, which shares the text.
fn index_of(shown: Option<&ShownSource>, text: RevisedText) -> NetlistSourceIndex {
    let kind = kind_of(shown, &text);
    NetlistSourceIndex::of_text(Arc::new(text), kind)
}

/// The index of `text`, built from `last`, the index of the same source at
/// revision `built`, before an edit: the part the edit touched read again,
/// the rest shared. Where the edit is the editor says when it knows; else
/// the two texts are compared. An edit that may change how the whole text
/// is read, SPICE or Spectre, reads it all.
fn edited_from(
    last: &NetlistSourceIndex,
    built: u64,
    shown: Option<&ShownSource>,
    text: RevisedText,
) -> NetlistSourceIndex {
    use rspice_core::netlist::DeckTextChange;
    let Some(before) = last.deck().text(0) else {
        return index_of(shown, text);
    };
    let change = text
        .edit_since(built)
        .or_else(|| DeckTextChange::between(&**before, &text));
    let Some(change) = change else {
        return NetlistSourceIndex::clone(last);
    };
    // The lines the edit spans, before it and after it.
    let deck = last.deck();
    let line_span = |byte: usize| {
        deck.line_of(0, byte)
            .and_then(|line| deck.line_span(0, line))
    };
    let before_lines = line_span(change.start)
        .zip(line_span(change.old_end))
        .map_or_else(String::new, |(first, end)| {
            before.slice(first.start..end.end).to_ascii_lowercase()
        });
    let after_lines = text
        .slice(text.line_start_at(change.start)..text.line_end_at(change.new_end))
        .to_ascii_lowercase();
    if [before_lines, after_lines]
        .iter()
        .any(|written| written.contains("/*") || written.contains("*/") || written.contains("lang"))
    {
        return index_of(shown, text);
    }
    last.edited(change, Arc::new(text))
}

fn now_in(
    slot: &mut SourceIndexSlot,
    shown: Option<&ShownSource>,
    source: &RevisedText,
) -> Arc<NetlistSourceIndex> {
    let revision = source.revision();
    slot.wait_for(|built| built.1 == revision && built.0.as_ref() == shown)
        .unwrap_or_else(|| slot.store((shown.cloned(), revision), index_of(shown, source.clone())))
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
/// frame, built away from it. `None` until the outline of text put in the
/// buffer by other than typing (a file opened, a deck loaded) is built:
/// what reads it shows it is being read. While the writer types, the last
/// outline built stands in until the one of the buffer as it stands is
/// built. It reads the model and keeps the outline in the netlist editor's
/// state.
pub fn visible_source_index_in(
    ctx: &egui::Context,
    model: &Model,
    netlist: &mut NetlistDocumentState,
) -> Option<Arc<NetlistSourceIndex>> {
    shown_index_in(ctx, model, netlist).map(|(index, _)| index)
}

/// [`visible_source_index_in`] only when it is the outline of the buffer as
/// it stands, not one that stands in for it.
pub(crate) fn current_source_index_in(
    ctx: &egui::Context,
    model: &Model,
    netlist: &mut NetlistDocumentState,
) -> Option<Arc<NetlistSourceIndex>> {
    shown_index_in(ctx, model, netlist).and_then(|(index, current)| current.then_some(index))
}

fn shown_index_in(
    ctx: &egui::Context,
    model: &Model,
    netlist: &mut NetlistDocumentState,
) -> Option<(Arc<NetlistSourceIndex>, bool)> {
    if netlist.shown_language_off() {
        return None;
    }
    let typed = netlist.is_being_typed();
    if !typed && netlist.waits_for_indexes() {
        return Some((visible_source_index_now(model, netlist), true));
    }
    let shown = super::dialect::shown_source_in(model, netlist);
    latest_in(
        ctx,
        &mut netlist.source_index,
        shown.as_ref(),
        &model.simulation.source.netlist_content,
        typed,
    )
}

/// [`visible_source_index_in`] of the buffer as it stands, waiting for its
/// build to finish: what an action answers from.
pub fn visible_source_index_now(
    model: &Model,
    netlist: &mut NetlistDocumentState,
) -> Arc<NetlistSourceIndex> {
    let shown = super::dialect::shown_source_in(model, netlist);
    now_in(
        &mut netlist.source_index,
        shown.as_ref(),
        &model.simulation.source.netlist_content,
    )
}

/// The project's top-level deck when the editor shows other text: until the
/// Netlist workspace has been opened, a restored session or a reopened
/// project has put nothing in the editor, and an include file the editor has
/// open is not the deck. Whether the two are the same text is compared once
/// for each pair of their revisions.
fn root_deck_apart(
    model: &Model,
    netlist: &NetlistDocumentState,
) -> Option<(ShownSource, RevisedText)> {
    let deck = &netlist.open_deck.as_ref()?.root.text;
    if netlist
        .root_deck_shown
        .same(deck, &model.simulation.source.netlist_content)
    {
        return None;
    }
    let root = ShownSource {
        path: root_deck_path(model, netlist),
        root: true,
    };
    Some((root, deck.clone()))
}

/// [`visible_source_index_in`] for the project's own top-level deck, whatever
/// the editor shows, built away from the frame: `None` until it is built. A
/// deck the editor is not showing is not being typed, so no older outline
/// stands in for it.
pub fn root_deck_source_index(
    ctx: &egui::Context,
    model: &Model,
    netlist: &mut NetlistDocumentState,
) -> Option<Arc<NetlistSourceIndex>> {
    if netlist.language_off().is_some() {
        return None;
    }
    match root_deck_apart(model, netlist) {
        Some((root, deck)) if netlist.waits_for_indexes() => {
            Some(now_in(&mut netlist.root_deck_index, Some(&root), &deck))
        }
        Some((root, deck)) => {
            latest_in(ctx, &mut netlist.root_deck_index, Some(&root), &deck, false)
                .map(|(index, _)| index)
        }
        None => visible_source_index_in(ctx, model, netlist),
    }
}

/// [`root_deck_source_index`] of the deck as it stands, waiting for its build
/// to finish.
pub fn root_deck_source_index_now(
    model: &Model,
    netlist: &mut NetlistDocumentState,
) -> Arc<NetlistSourceIndex> {
    match root_deck_apart(model, netlist) {
        Some((root, deck)) => now_in(&mut netlist.root_deck_index, Some(&root), &deck),
        None => visible_source_index_now(model, netlist),
    }
}
