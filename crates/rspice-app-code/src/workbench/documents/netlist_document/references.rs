//! Find references and Rename for a deck: every place the engine's index of
//! the deck and its includes binds to the name under the caret, and the
//! edits renaming it makes.
//!
//! The places are the index's, never a search for the name's text: a read
//! the index binds to another definition of the same name is not one of
//! them. A rename refuses, and says where and why, rather than leave the
//! deck reading something else: a read of the name the index cannot bind
//! (inside a string, an expression it does not read, a `.control` block),
//! a file not open beside the deck, or a new name a reader of the old one
//! would already see.

use std::collections::BTreeMap;
use std::ops::Range;
use std::sync::Arc;

use rspice_core::netlist::{DeckCardKind, DeckIndex, DeckNameAt, DeckSymbolKind, DeckTokenKind};

use super::language::{NetlistProjectIndex, project_index_now};
use super::{
    Model, NetlistDocumentState, NetlistRoot, OwnedNetlistReplacement, canonical_root_document_in,
};
use crate::workbench::{MessageCatalog, MessageId};

/// What a name under the caret names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DeckName {
    /// A definition, by its position among the index's symbols.
    Symbol(usize),
    /// A node, by its name and the scope it belongs to.
    Node { name: String, scope: usize },
}

/// One place a name is written: its file in the index and its bytes there.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DeckPlace {
    pub(crate) file: usize,
    pub(crate) span: Range<usize>,
}

/// The name under the caret, with the index that bound it and the texts the
/// index was built from, one to a file in the index's order.
#[derive(Debug, Clone)]
pub(crate) struct NameAtCaret {
    pub(crate) index: Arc<NetlistProjectIndex>,
    pub(crate) texts: Arc<[String]>,
    pub(crate) name: DeckName,
    /// The name as its definition writes it.
    pub(crate) written: String,
}

impl NameAtCaret {
    /// What the name is, in a word, and the same word heading a row.
    pub(crate) fn kind_words(&self) -> (MessageId, MessageId) {
        match &self.name {
            DeckName::Node { .. } => (MessageId::NetlistKindNode, MessageId::NetlistKindNodeLabel),
            DeckName::Symbol(symbol) => self.index.deck().symbol(*symbol).map_or(
                (MessageId::NetlistKindName, MessageId::NetlistKindNameLabel),
                |symbol| kind_words(symbol.kind()),
            ),
        }
    }

    /// Every place the name is written, in file and source order.
    pub(crate) fn places(&self) -> Vec<DeckPlace> {
        places(self.index.deck(), &self.texts, &self.name)
    }
}

/// What a definition of `kind` is, in a word, and the same word heading a
/// row.
const fn kind_words(kind: DeckSymbolKind) -> (MessageId, MessageId) {
    match kind {
        DeckSymbolKind::Parameter => (
            MessageId::NetlistKindParameter,
            MessageId::NetlistKindParameterLabel,
        ),
        DeckSymbolKind::Function => (
            MessageId::NetlistKindFunction,
            MessageId::NetlistKindFunctionLabel,
        ),
        DeckSymbolKind::Subcircuit => (
            MessageId::NetlistKindSubcircuit,
            MessageId::NetlistKindSubcircuitLabel,
        ),
        DeckSymbolKind::Model => (
            MessageId::NetlistKindModel,
            MessageId::NetlistKindModelLabel,
        ),
        DeckSymbolKind::Instance => (
            MessageId::NetlistKindInstance,
            MessageId::NetlistKindInstanceLabel,
        ),
    }
}

/// The texts `index` was built from, one to a file in its order, read from
/// the open deck and its includes, or the generated netlist and the files it
/// includes; `None` once any of them has changed.
fn indexed_texts(
    model: &Model,
    netlist: &NetlistDocumentState,
    index: &NetlistProjectIndex,
) -> Option<Arc<[String]>> {
    let root = index.root()?;
    let texts = (0..index.deck().files().len())
        .map(|file| file_text(model, netlist, root, index.file_dependency(file)))
        .collect::<Option<Vec<_>>>()?;
    let borrowed = texts.iter().map(String::as_str).collect::<Vec<_>>();
    index.indexed(&borrowed).then(|| texts.into())
}

/// The text of the file `identity` names under `root`, the root itself for
/// `None`.
fn file_text(
    model: &Model,
    netlist: &NetlistDocumentState,
    root: NetlistRoot,
    identity: Option<&str>,
) -> Option<String> {
    if root == NetlistRoot::Deck {
        return netlist
            .open_deck
            .as_ref()?
            .file(identity)
            .map(|file| file.text.as_str().to_owned());
    }
    let document = canonical_root_document_in(model, netlist, root)?;
    match identity {
        None => Some(document.source().to_owned()),
        Some(identity) => document
            .dependencies()
            .iter()
            .find(|dependency| dependency.locator().logical_identity() == identity)
            .and_then(crate::state::DependencyMetadata::source)
            .map(str::to_owned),
    }
}

/// The name under the caret of the source the editor shows, as the deck's
/// index binds it.
pub(crate) fn name_at_caret(
    model: &Model,
    netlist: &mut NetlistDocumentState,
    messages: MessageCatalog,
) -> Result<NameAtCaret, String> {
    let place_caret = || messages.text(MessageId::NetlistFollowPlaceCaret);
    let still_reading = || messages.text(MessageId::NetlistFollowStillReading);
    let index = project_index_now(model, netlist)
        .ok_or_else(|| messages.text(MessageId::NetlistFollowNoNames))?;
    let buffer = &model.simulation.source.netlist_content;
    let file = index
        .file_holding(netlist.front.include(), buffer)
        .ok_or_else(still_reading)?;
    let texts = indexed_texts(model, netlist, &index).ok_or_else(still_reading)?;
    let byte = buffer
        .char_indices()
        .nth(netlist.cursor_char_index)
        .map_or(buffer.len(), |(byte, _)| byte);
    let deck = index.deck();
    let name = match deck.name_at(file, byte) {
        Some(DeckNameAt::Definition(symbol)) => DeckName::Symbol(symbol),
        Some(DeckNameAt::Reference(reference)) => {
            let read = deck.reference(reference).ok_or_else(place_caret)?;
            DeckName::Symbol(read.target().ok_or_else(|| {
                messages.format(MessageId::NetlistFollowUndefined, &[("name", &read.name())])
            })?)
        }
        None => deck
            .node_at(file, byte)
            .map(|node| DeckName::Node {
                name: node.name().into_owned(),
                scope: node.scope(),
            })
            .ok_or_else(place_caret)?,
    };
    let written = match &name {
        DeckName::Symbol(symbol) => {
            let symbol = deck.symbol(*symbol).ok_or_else(place_caret)?;
            if symbol.kind() == DeckSymbolKind::Instance {
                return Err(place_caret());
            }
            symbol.name().into_owned()
        }
        DeckName::Node { name, .. } => name.clone(),
    };
    Ok(NameAtCaret {
        index,
        texts,
        name,
        written,
    })
}

/// Every place `name` is written in the deck and its includes, in file and
/// source order: a definition and the reads the index binds to it, with the
/// `name=value` an instance of a subcircuit gives one of its parameters; or
/// every place a node of one scope is written.
fn places(deck: &DeckIndex, texts: &[String], name: &DeckName) -> Vec<DeckPlace> {
    let mut places = match name {
        DeckName::Node { name, scope } => deck
            .nodes()
            .filter(|node| node.scope() == *scope && node.name().eq_ignore_ascii_case(name))
            .map(|node| DeckPlace {
                file: node.file(),
                span: node.span(),
            })
            .collect::<Vec<_>>(),
        DeckName::Symbol(symbol) => {
            let Some(definition) = deck.symbol(*symbol) else {
                return Vec::new();
            };
            let mut places = std::iter::once(definition.location())
                .chain(deck.references_to(*symbol).map(|read| read.location()))
                .map(|location| DeckPlace {
                    file: location.file,
                    span: location.span,
                })
                .collect::<Vec<_>>();
            places.extend(instance_assignments(deck, texts, *symbol));
            places
        }
    };
    places.sort_by_key(|place| (place.file, place.span.start, place.span.end));
    places.dedup();
    places
}

/// Where the instances of a subcircuit give the parameter `symbol` a value,
/// when `symbol` is one the subcircuit's header declares: `gain` in
/// `X1 a b amp gain=3`.
fn instance_assignments(deck: &DeckIndex, texts: &[String], symbol: usize) -> Vec<DeckPlace> {
    let Some(parameter) = deck.symbol(symbol) else {
        return Vec::new();
    };
    let name = parameter.name();
    let Some(subcircuit) = (parameter.kind() == DeckSymbolKind::Parameter)
        .then(|| deck.scope(parameter.scope())?.symbol())
        .flatten()
        .filter(|subcircuit| {
            deck.symbol(*subcircuit)
                .is_some_and(|subcircuit| subcircuit.card() == parameter.card())
        })
    else {
        return Vec::new();
    };
    deck.symbols()
        .filter(|instance| {
            instance.kind() == DeckSymbolKind::Instance && instance.target() == Some(subcircuit)
        })
        .flat_map(|instance| {
            let file = instance.file();
            let text = texts.get(file).map_or("", String::as_str);
            let tokens = deck.card_tokens(instance.card());
            tokens
                .windows(2)
                .filter(|pair| {
                    pair[0].kind == DeckTokenKind::Name
                        && pair[1].kind == DeckTokenKind::Equals
                        && text
                            .get(pair[0].span.clone())
                            .is_some_and(|written| written.eq_ignore_ascii_case(&name))
                })
                .map(|pair| DeckPlace {
                    file,
                    span: pair[0].span.clone(),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

/// The one-based line and column `byte` of `text` is at.
fn line_column(text: &str, byte: usize) -> (usize, usize) {
    let before = text.get(..byte).unwrap_or(text);
    let line_start = before.rfind('\n').map_or(0, |newline| newline + 1);
    (
        before.matches('\n').count() + 1,
        before[line_start..].chars().count() + 1,
    )
}

/// The physical line of `text` that `byte` is on, without its line break.
fn line_holding(text: &str, byte: usize) -> (Range<usize>, &str) {
    let start = text
        .get(..byte)
        .and_then(|before| before.rfind('\n'))
        .map_or(0, |newline| newline + 1);
    let end = text[start..].find('\n').map_or(text.len(), |at| start + at);
    let line = &text[start..end];
    (start..end, line.strip_suffix('\r').unwrap_or(line))
}

/// One row of the References page for each place `caret` names.
pub(crate) fn reference_rows(caret: &NameAtCaret) -> Vec<crate::workbench::state::SourceReference> {
    caret
        .places()
        .into_iter()
        .map(|place| {
            let text = caret.texts.get(place.file).map_or("", String::as_str);
            let (line, column) = line_column(text, place.span.start);
            crate::workbench::state::SourceReference {
                logical_path: caret.index.file_name(place.file).to_owned(),
                include: caret.index.file_dependency(place.file).map(str::to_owned),
                line,
                column,
                text: line_holding(text, place.span.start).1.trim().to_owned(),
            }
        })
        .collect()
}

/// A rename the reader is writing: the name, and the new name as it stands
/// with what renaming to it would do, worked out once for each new name.
#[derive(Debug, Clone)]
pub(crate) struct DeckRename {
    pub(crate) caret: NameAtCaret,
    pub(crate) replacement: String,
    /// Why the last rename asked for did not happen.
    pub(crate) error: Option<String>,
    /// The revision of the deck the name was read from.
    revision: Option<u64>,
    /// The includes open beside the deck, which a rename may change.
    writable: Vec<String>,
    planned: Option<(String, Result<RenamePlan, String>)>,
    /// The catalogue its refusals and receipt are written from.
    messages: MessageCatalog,
}

/// What a rename does: one row to each line it changes, and the new text of
/// each file it changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RenamePlan {
    pub(crate) rows: Vec<RenameRow>,
    /// How many places it rewrites.
    pub(crate) places: usize,
    /// Each file it changes, by its position in the index, with its new text.
    files: BTreeMap<usize, String>,
}

impl RenamePlan {
    /// How many files it changes.
    pub(crate) fn file_count(&self) -> usize {
        self.files.len()
    }
}

/// One line a rename changes, before and after.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RenameRow {
    pub(crate) file: String,
    pub(crate) line: usize,
    pub(crate) before: String,
    pub(crate) after: String,
}

impl DeckRename {
    /// Rename the name under the caret of the deck the editor shows.
    pub(crate) fn at_caret(
        model: &Model,
        netlist: &mut NetlistDocumentState,
        messages: MessageCatalog,
    ) -> Result<Self, String> {
        if netlist.front.root_kind() != NetlistRoot::Deck {
            return Err(messages.text(MessageId::NetlistRenameGeneratedDeck));
        }
        let caret = name_at_caret(model, netlist, messages)?;
        let replacement = caret.written.clone();
        let writable = netlist
            .open_deck
            .iter()
            .flat_map(|deck| {
                deck.includes
                    .iter()
                    .map(super::super::deck_files::OpenFile::key)
            })
            .collect();
        Ok(Self {
            caret,
            replacement,
            error: None,
            revision: deck_revision(model, netlist),
            writable,
            planned: None,
            messages,
        })
    }

    /// Whether the deck is still the one the name was read from.
    pub(crate) fn is_current(&self, model: &Model, netlist: &NetlistDocumentState) -> bool {
        deck_revision(model, netlist) == self.revision
    }

    /// The new name as written, without the space around it.
    pub(crate) fn new_name(&self) -> &str {
        self.replacement.trim()
    }

    /// Whether the new name is the old one.
    pub(crate) fn unchanged(&self) -> bool {
        self.new_name().is_empty() || self.new_name() == self.caret.written
    }

    /// What renaming to the new name does, or why it refuses.
    pub(crate) fn plan(&mut self) -> &Result<RenamePlan, String> {
        let new_name = self.new_name().to_owned();
        if self
            .planned
            .as_ref()
            .is_some_and(|(planned, _)| *planned != new_name)
        {
            self.planned = None;
        }
        let caret = &self.caret;
        let writable = &self.writable;
        let messages = self.messages;
        &self
            .planned
            .get_or_insert_with(|| {
                let plan = plan(caret, &new_name, messages, |identity| {
                    writable.iter().any(|open| open == identity)
                });
                (new_name.clone(), plan)
            })
            .1
    }
}

/// Whether `name` is one a deck reads as a single name.
fn readable_name(name: &str, node: bool) -> bool {
    !name.is_empty()
        && !name
            .chars()
            .any(|c| c.is_whitespace() || "(){}[]=,;'\"*$".contains(c))
        && (node || !name.starts_with(|first: char| first.is_ascii_digit() || first == '.'))
}

/// What renaming the name `caret` names to `new_name` does, or why it
/// refuses; `writable` says whether the project writes an include.
fn plan(
    caret: &NameAtCaret,
    new_name: &str,
    messages: MessageCatalog,
    writable: impl Fn(&str) -> bool,
) -> Result<RenamePlan, String> {
    let deck = caret.index.deck();
    let old = caret.written.as_str();
    let where_ = |file: usize, byte: usize| {
        let text = caret.texts.get(file).map_or("", String::as_str);
        messages.format(
            MessageId::NetlistSymbolPlace,
            &[
                ("file", caret.index.file_name(file)),
                ("line", &line_column(text, byte).0.to_string()),
            ],
        )
    };
    let node = matches!(caret.name, DeckName::Node { .. });
    if node && (old == "0" || old.eq_ignore_ascii_case("gnd")) {
        return Err(messages.text(MessageId::NetlistRenameGround));
    }
    if !readable_name(new_name, node) {
        return Err(messages.format(
            if node {
                MessageId::NetlistRenameUnreadableNode
            } else {
                MessageId::NetlistRenameUnreadableName
            },
            &[
                ("name", new_name),
                ("characters", "( ) { } [ ] = , ; ' \" * $"),
            ],
        ));
    }
    let places = caret.places();
    if let Some(collision) = collision(caret, new_name, &places) {
        let (file, byte, kind) = collision;
        return Err(messages.format(
            MessageId::NetlistRenameCollision,
            &[
                ("name", new_name),
                ("kind", &messages.text(kind)),
                ("old", old),
                ("place", &where_(file, byte)),
            ],
        ));
    }
    for file in places.iter().map(|place| place.file) {
        if let Some(identity) = caret.index.file_dependency(file)
            && !writable(identity)
        {
            return Err(messages.format(
                MessageId::NetlistRenameIncludeNotOpen,
                &[("file", caret.index.file_name(file)), ("old", old)],
            ));
        }
    }
    if let Some((file, byte, inside)) = unbound_read(deck, &caret.texts, old, &places) {
        return Err(messages.format(
            MessageId::NetlistRenameUnbound,
            &[
                ("place", &where_(file, byte)),
                ("old", old),
                ("inside", &messages.text(inside)),
            ],
        ));
    }

    let mut rows = Vec::new();
    let mut files = BTreeMap::new();
    let mut by_file = BTreeMap::<usize, Vec<&DeckPlace>>::new();
    for place in &places {
        by_file.entry(place.file).or_default().push(place);
    }
    for (file, in_file) in by_file {
        let text = caret.texts.get(file).map_or("", String::as_str);
        let mut renamed = text.to_owned();
        for place in in_file.iter().rev() {
            renamed.replace_range(place.span.clone(), new_name);
        }
        let mut lines = BTreeMap::<usize, (Range<usize>, Vec<&DeckPlace>)>::new();
        for place in &in_file {
            let (range, _) = line_holding(text, place.span.start);
            lines
                .entry(range.start)
                .or_insert_with(|| (range, Vec::new()))
                .1
                .push(place);
        }
        for (start, (range, on_line)) in lines {
            let before = line_holding(text, start).1;
            let mut after = text[range.clone()].to_owned();
            for place in on_line.iter().rev() {
                after.replace_range(
                    place.span.start - range.start..place.span.end - range.start,
                    new_name,
                );
            }
            rows.push(RenameRow {
                file: caret.index.file_name(file).to_owned(),
                line: line_column(text, start).0,
                before: before.trim().to_owned(),
                after: after.trim_end_matches('\r').trim().to_owned(),
            });
        }
        files.insert(file, renamed);
    }
    Ok(RenamePlan {
        rows,
        places: places.len(),
        files,
    })
}

/// A definition of `new_name` a reader of the name `caret` names already
/// sees, as its file, byte and kind word: renaming would hand that reader
/// the other definition.
fn collision(
    caret: &NameAtCaret,
    new_name: &str,
    places: &[DeckPlace],
) -> Option<(usize, usize, MessageId)> {
    let deck = caret.index.deck();
    match &caret.name {
        DeckName::Node { name, scope } => deck
            .nodes()
            .find(|node| {
                let written = node.name();
                written.eq_ignore_ascii_case(new_name)
                    && !written.eq_ignore_ascii_case(name)
                    && (node.scope() == *scope || node.card_kind() == DeckCardKind::Global)
            })
            .map(|node| (node.file(), node.span().start, MessageId::NetlistKindNode)),
        DeckName::Symbol(symbol) => {
            let definition = deck.symbol(*symbol)?;
            // An instance reads its master as a subcircuit, else a model.
            let kinds: &[DeckSymbolKind] = match definition.kind() {
                DeckSymbolKind::Subcircuit | DeckSymbolKind::Model => {
                    &[DeckSymbolKind::Subcircuit, DeckSymbolKind::Model]
                }
                DeckSymbolKind::Parameter => &[DeckSymbolKind::Parameter],
                DeckSymbolKind::Function => &[DeckSymbolKind::Function],
                DeckSymbolKind::Instance => &[],
            };
            let scopes = std::iter::once(definition.scope())
                .chain(deck.references_to(*symbol).map(|read| read.scope()))
                .chain(places.iter().filter_map(|place| {
                    let line = deck.line_of(place.file, place.span.start)?;
                    Some(deck.scope_on_line(place.file, line))
                }))
                .collect::<std::collections::BTreeSet<_>>();
            scopes.into_iter().find_map(|scope| {
                kinds.iter().find_map(|kind| {
                    let other = deck.resolve(*kind, new_name, scope)?;
                    let other = deck.symbol(other).filter(|_| other != *symbol)?;
                    Some((other.file(), other.span().start, kind_words(other.kind()).0))
                })
            })
        }
    }
}

/// A read of `name` the index binds to nothing it knows, outside `places`:
/// inside a string, an expression it does not read, or a `.control` block.
/// Its file, byte, and where it is, in words.
fn unbound_read(
    deck: &DeckIndex,
    texts: &[String],
    name: &str,
    places: &[DeckPlace],
) -> Option<(usize, usize, MessageId)> {
    // A word starts at `byte`, so a name the index knows covers it when one
    // is written over it at all.
    let known = |file: usize, byte: usize| {
        places
            .iter()
            .any(|place| place.file == file && place.span.contains(&byte))
            || deck.name_at(file, byte).is_some()
            || deck.node_at(file, byte).is_some()
    };
    for card in deck.cards() {
        let file = card.file();
        let Some(text) = texts.get(file) else {
            continue;
        };
        match card.kind() {
            DeckCardKind::Title | DeckCardKind::Include | DeckCardKind::Library => {}
            DeckCardKind::Control => {
                let (start, _) = line_holding(text, card.location().span.start);
                let mut end = start.start;
                for _ in card.lines() {
                    end = text[end..].find('\n').map_or(text.len(), |at| end + at + 1);
                }
                if let Some(at) = words(&text[start.start..end], name).next() {
                    return Some((file, start.start + at, MessageId::NetlistInsideControl));
                }
            }
            _ => {
                for token in card.tokens() {
                    // A string is text, whatever reads it: a word in it may
                    // be the name or only look like it.
                    let (inside, bound_counts) = match token.kind {
                        DeckTokenKind::String => (MessageId::NetlistInsideString, false),
                        DeckTokenKind::Expression => (MessageId::NetlistInsideExpression, true),
                        _ => continue,
                    };
                    let Some(written) = text.get(token.span.clone()) else {
                        continue;
                    };
                    if let Some(at) = words(written, name)
                        .map(|at| token.span.start + at)
                        .find(|byte| !bound_counts || !known(file, *byte))
                    {
                        return Some((file, at, inside));
                    }
                }
            }
        }
    }
    None
}

/// The bytes of `text` each whole-word `name` starts at, ignoring case. A
/// word runs over letters, digits, `_` and `.`.
fn words<'a>(text: &'a str, name: &'a str) -> impl Iterator<Item = usize> + 'a {
    let part = |c: char| c.is_alphanumeric() || c == '_' || c == '.';
    let lower = text.to_ascii_lowercase();
    let wanted = name.to_ascii_lowercase();
    let starts = lower
        .match_indices(&wanted)
        .map(|(at, _)| at)
        .collect::<Vec<_>>();
    starts.into_iter().filter(move |at| {
        let before = text[..*at].chars().next_back();
        let after = text[at + name.len()..].chars().next();
        !before.is_some_and(part) && !after.is_some_and(part)
    })
}

/// The edits a rename makes, one to a file it changes.
pub(crate) fn replacements(caret: &NameAtCaret, plan: &RenamePlan) -> Vec<OwnedNetlistReplacement> {
    plan.files
        .iter()
        .filter_map(|(file, renamed)| {
            let original = caret.texts.get(*file)?;
            let count = plan
                .rows
                .iter()
                .filter(|row| row.file == caret.index.file_name(*file))
                .count();
            Some(match caret.index.file_dependency(*file) {
                None => OwnedNetlistReplacement::root(original, renamed.clone(), count),
                Some(identity) => {
                    OwnedNetlistReplacement::dependency(identity, original, renamed.clone(), count)
                }
            })
        })
        .collect()
}

/// Whether the texts the rename was planned over are still the deck's.
pub(crate) fn still_current(
    model: &Model,
    netlist: &NetlistDocumentState,
    caret: &NameAtCaret,
) -> bool {
    indexed_texts(model, netlist, &caret.index).is_some_and(|texts| texts == caret.texts)
}

/// The revision of the deck the editor's names are read from: the editor's
/// of the open deck's text, or the generated netlist's own.
fn deck_revision(model: &Model, netlist: &NetlistDocumentState) -> Option<u64> {
    let root = netlist.front.root_kind();
    if root == NetlistRoot::Deck {
        return netlist.open_deck.as_ref().map(|_| netlist.revision);
    }
    canonical_root_document_in(model, netlist, root).map(|document| document.revision().get())
}

/// Find references: every place the name under the caret is written, one
/// row each on the References page, which comes forward; and the count, in
/// words, for the status bar.
pub(crate) fn find_references(
    model: &Model,
    netlist: &mut NetlistDocumentState,
    workbench: &mut crate::workbench::state::WorkbenchSession,
) -> Result<String, String> {
    let messages = workbench.messages();
    let caret = name_at_caret(model, netlist, messages)?;
    let rows = reference_rows(&caret);
    let files = rows
        .iter()
        .map(|row| row.logical_path.as_str())
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    let count = rows.len();
    let revision = deck_revision(model, netlist).unwrap_or_default();
    workbench.show_references(crate::workbench::state::SourceReferences::deck(
        caret.written.clone(),
        revision,
        rows,
    ));
    Ok(messages.format(
        MessageId::NetlistReferencesFound,
        &[
            ("kind", &messages.text(caret.kind_words().1)),
            ("name", &caret.written),
            (
                "places",
                &messages.counted(count, MessageId::CountOnePlace, MessageId::CountPlaces),
            ),
            (
                "files",
                &messages.counted(files, MessageId::CountOneFile, MessageId::CountFiles),
            ),
        ],
    ))
}

/// Say on the References page when the deck its list was read from has
/// changed since.
pub(crate) fn settle_deck_references(
    model: &Model,
    netlist: &NetlistDocumentState,
    workbench: &mut crate::workbench::state::WorkbenchSession,
) {
    if let Some(list) = workbench.source_references.as_mut()
        && list.source == crate::workbench::state::ReferencesIn::Deck
        && !list.stale
        && deck_revision(model, netlist) != Some(list.revision)
    {
        list.stale = true;
    }
}

/// Rename the name the open rename names to its new name, as one change
/// that Undo takes back a file at a time. The words for the status bar.
pub(crate) fn commit_rename(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    rename: &mut DeckRename,
) -> Result<String, String> {
    let messages = rename.messages;
    if !rename.is_current(model, netlist) || !still_current(model, netlist, &rename.caret) {
        return Err(messages.text(MessageId::SourceRenameDeckChanged));
    }
    let new_name = rename.new_name().to_owned();
    let old = rename.caret.written.clone();
    let plan = rename.plan().clone()?;
    let replacements = replacements(&rename.caret, &plan);
    super::replace_deck_files_a_step_per_file(
        model,
        netlist,
        replacements,
        &messages.format(
            MessageId::NetlistRenameStep,
            &[("old", &old), ("new", &new_name)],
        ),
        messages,
    )?;
    Ok(messages.format(
        MessageId::NetlistRenamed,
        &[
            ("old", &old),
            ("new", &new_name),
            (
                "places",
                &messages.counted(
                    plan.places,
                    MessageId::CountOnePlace,
                    MessageId::CountPlaces,
                ),
            ),
            (
                "files",
                &messages.counted(
                    plan.file_count(),
                    MessageId::CountOneFile,
                    MessageId::CountFiles,
                ),
            ),
        ],
    ))
}

/// Open the deck at `place`: the file an include's identity names, or the
/// deck itself, and the one-based line and column there, with the caret on
/// it.
pub fn open_deck_place(
    ctx: &egui::Context,
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut std::collections::HashSet<crate::workbench::state::WorkspaceDocumentId>,
    runs: &mut rspice_app_runs::workbench::slots::runs::RunsState,
    workbench: &mut crate::workbench::state::WorkbenchSession,
    (include, line, column): (Option<&str>, usize, usize),
) -> Result<(), String> {
    let root = netlist.front.root_kind();
    super::language::open_project_location_in(
        model,
        netlist,
        open_documents,
        runs,
        &super::language::ProjectSourceLocation {
            root,
            dependency_identity: include.map(str::to_owned),
            display_name: String::new(),
            line,
            column,
        },
        workbench.messages(),
    )?;
    workbench.activate(crate::workbench::state::Workspace::Netlist);
    let buffer = &model.simulation.source.netlist_content;
    let line_start = buffer
        .split_inclusive('\n')
        .take(line.saturating_sub(1))
        .map(|line| line.chars().count())
        .sum::<usize>();
    crate::workbench::documents::text_editor_commands::queue_caret_char_index(
        ctx,
        super::editor_id_in(model, netlist),
        line_start + column.saturating_sub(1),
    );
    Ok(())
}

#[cfg(test)]
mod tests;
