//! Netlist outline.
//!
//! The navigable structure of a deck — subcircuits, models, analyses,
//! includes — for the outline pane.

use rspice_core_netlist::netlist::{
    DeckCard, DeckCardKind, DeckIndex, DeckIndexSource, DeckSymbol, DeckSymbolKind, DeckTextChange,
    DeckTokenKind, SharedDeckText, SourceKind, comment_start,
};
use std::borrow::Cow;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

/// The source construct represented by an outline row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OutlineEntryKind {
    Title,
    Include,
    Library,
    Parameter,
    Global,
    Function,
    Option,
    Model,
    Subcircuit,
    EndSubcircuit,
    Analysis,
    Measurement,
    Output,
    Conditional,
    Control,
    Device,
}

/// Stable, canonical grouping used by the navigator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OutlineSectionKind {
    Source,
    Dependencies,
    Parameters,
    Globals,
    Functions,
    Options,
    Models,
    Subcircuits,
    Analyses,
    Measurements,
    Outputs,
    Conditionals,
    Controls,
    Devices,
}

impl OutlineEntryKind {
    const fn section(self) -> OutlineSectionKind {
        match self {
            Self::Title => OutlineSectionKind::Source,
            Self::Include | Self::Library => OutlineSectionKind::Dependencies,
            Self::Parameter => OutlineSectionKind::Parameters,
            Self::Global => OutlineSectionKind::Globals,
            Self::Function => OutlineSectionKind::Functions,
            Self::Option => OutlineSectionKind::Options,
            Self::Model => OutlineSectionKind::Models,
            Self::Subcircuit | Self::EndSubcircuit => OutlineSectionKind::Subcircuits,
            Self::Analysis => OutlineSectionKind::Analyses,
            Self::Measurement => OutlineSectionKind::Measurements,
            Self::Output => OutlineSectionKind::Outputs,
            Self::Conditional => OutlineSectionKind::Conditionals,
            Self::Control => OutlineSectionKind::Controls,
            Self::Device => OutlineSectionKind::Devices,
        }
    }
}

/// The word an entry is declared with: its card's first word, or one the
/// outline names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Keyword {
    Head,
    /// Spectre's `inline subckt`.
    InlineSubckt,
    /// A subcircuit's formal parameters.
    Params,
}

/// No scope or symbol.
const NONE: u32 = u32::MAX;

/// What one entry is read from: a card of the deck's first file, and the
/// parameter it names when a card gives a row to each.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EntryRec {
    kind: OutlineEntryKind,
    keyword: Keyword,
    /// By its position among the deck's cards.
    card: u32,
    /// By its position among the deck's symbols; `NONE` for a card's row.
    symbol: u32,
    /// The entry of the `.subckt` card the declaration is written inside.
    scope: u32,
}

/// One navigable source declaration, read from the deck's index of the text
/// when it is asked for. Lines and columns are one-based Unicode scalar
/// positions; `end_line` includes continuation cards.
#[derive(Clone, Copy)]
pub struct OutlineEntry<'a> {
    deck: &'a DeckIndex,
    rec: &'a EntryRec,
}

impl<'a> OutlineEntry<'a> {
    #[must_use]
    pub const fn kind(&self) -> OutlineEntryKind {
        self.rec.kind
    }

    fn card(&self) -> Option<DeckCard<'a>> {
        self.deck.card(self.rec.card as usize)
    }

    fn symbol(&self) -> Option<DeckSymbol<'a>> {
        (self.rec.symbol != NONE)
            .then(|| self.deck.symbol(self.rec.symbol as usize))
            .flatten()
    }

    /// The card's head as written; the whole first line for the title.
    fn head(&self) -> Cow<'a, str> {
        self.card().map_or(Cow::Borrowed(""), |card| card.head())
    }

    /// The first definition of `kind` the card writes.
    fn defined(&self, kind: DeckSymbolKind) -> Option<DeckSymbol<'a>> {
        self.card()?.symbols().find(|symbol| symbol.kind() == kind)
    }

    /// The card's words as written, its own first.
    fn words(&self) -> Vec<String> {
        self.card()
            .map_or_else(Vec::new, |card| card_words(self.deck, &card))
    }

    /// `.tran 1n`: the card's head and its first argument.
    fn directive_label(&self, words: &[String]) -> Cow<'a, str> {
        let head = self.head();
        match words.get(1) {
            Some(argument) => Cow::Owned(format!("{head} {argument}")),
            None => head,
        }
    }

    /// The path an include card names, quotes removed.
    fn include_path(&self) -> Option<Cow<'a, str>> {
        let card = self.rec.card as usize;
        self.deck
            .includes()
            .find(|include| include.card() == card)
            .map(|include| include.path())
    }

    #[must_use]
    pub fn label(&self) -> Cow<'a, str> {
        if let Some(parameter) = self.symbol() {
            return Cow::Owned(format!("{} {}", self.keyword(), parameter.name()));
        }
        match self.rec.kind {
            OutlineEntryKind::Title => trimmed(self.head()),
            OutlineEntryKind::Include | OutlineEntryKind::Library => {
                let head = self.head();
                match self
                    .include_path()
                    .or_else(|| self.words().get(1).cloned().map(Cow::Owned))
                {
                    Some(path) => Cow::Owned(format!("{head} {path}")),
                    None => head,
                }
            }
            OutlineEntryKind::Model | OutlineEntryKind::Subcircuit | OutlineEntryKind::Function => {
                match self.defined(symbol_kind(self.rec.kind)) {
                    Some(symbol) => Cow::Owned(format!("{} {}", self.head(), symbol.name())),
                    None => self.directive_label(&self.words()),
                }
            }
            OutlineEntryKind::Device => self.head(),
            _ => self.directive_label(&self.words()),
        }
    }

    /// What the declaration names: a parameter, a model, a device, an
    /// analysis.
    #[must_use]
    pub fn name(&self) -> Cow<'a, str> {
        if let Some(parameter) = self.symbol() {
            return parameter.name();
        }
        match self.rec.kind {
            OutlineEntryKind::Title => trimmed(self.head()),
            OutlineEntryKind::Include | OutlineEntryKind::Library => self
                .include_path()
                .or_else(|| self.words().get(1).cloned().map(Cow::Owned))
                .unwrap_or_default(),
            OutlineEntryKind::Parameter => self.directive_label(&self.words()),
            OutlineEntryKind::Model | OutlineEntryKind::Subcircuit | OutlineEntryKind::Function => {
                match self.defined(symbol_kind(self.rec.kind)) {
                    Some(symbol) => symbol.name(),
                    None => self.directive_label(&self.words()),
                }
            }
            // `.meas tran rise when v(out)=0.5`: the measurement's name.
            OutlineEntryKind::Measurement => {
                let words = self.words();
                match words.get(2) {
                    Some(name) => Cow::Owned(name.clone()),
                    None => self.directive_label(&words),
                }
            }
            _ => self.head(),
        }
    }

    /// What it is bound to or written with: a parameter's value, a model's
    /// type, the subcircuit or model a device is of, an analysis's
    /// arguments.
    #[must_use]
    pub fn detail(&self) -> Option<Cow<'a, str>> {
        let written =
            |symbol: DeckSymbol<'a>| Some(symbol.detail()).filter(|detail| !detail.is_empty());
        if let Some(parameter) = self.symbol() {
            return written(parameter);
        }
        match self.rec.kind {
            OutlineEntryKind::Title
            | OutlineEntryKind::Include
            | OutlineEntryKind::Library
            | OutlineEntryKind::Parameter => None,
            OutlineEntryKind::Model | OutlineEntryKind::Subcircuit | OutlineEntryKind::Function => {
                self.defined(symbol_kind(self.rec.kind)).and_then(written)
            }
            OutlineEntryKind::Device => self.defined(DeckSymbolKind::Instance).and_then(written),
            // The analysis a measurement reads.
            OutlineEntryKind::Measurement => self.words().get(1).cloned().map(Cow::Owned),
            // `.options reltol=1e-5`: what the card sets, saves or declares.
            _ => {
                let words = self.words();
                (words.len() > 1).then(|| Cow::Owned(words[1..].join(" ")))
            }
        }
    }

    /// The word the deck declares the entry with, lowercased: `.subckt` in
    /// SPICE, `subckt` in Spectre, `.param`, `parameters`.
    #[must_use]
    pub fn keyword(&self) -> Cow<'a, str> {
        match self.rec.keyword {
            Keyword::Head => {
                let head = self.head();
                if head.bytes().any(|byte| byte.is_ascii_uppercase()) {
                    Cow::Owned(head.to_ascii_lowercase())
                } else {
                    head
                }
            }
            Keyword::InlineSubckt => Cow::Borrowed("inline subckt"),
            Keyword::Params => Cow::Borrowed("params:"),
        }
    }

    /// The position, among the outline's entries, of the `.subckt` card the
    /// declaration is written inside; `None` at the top level.
    #[must_use]
    pub const fn scope(&self) -> Option<usize> {
        if self.rec.scope == NONE {
            None
        } else {
            Some(self.rec.scope as usize)
        }
    }

    /// The one-based line it starts on: a parameter's own, which is the line
    /// of a `+` continuation it sits on.
    #[must_use]
    pub fn line(&self) -> usize {
        match self.symbol() {
            Some(parameter) => parameter.line(),
            None => self.card().map_or(0, |card| *card.lines().start()),
        }
    }

    #[must_use]
    pub fn column(&self) -> usize {
        match self.symbol() {
            Some(parameter) => parameter.location().column,
            None => self.card().map_or(0, |card| card.location().column),
        }
    }

    #[must_use]
    pub fn end_line(&self) -> usize {
        match self.symbol() {
            Some(parameter) => parameter.line(),
            None => self.card().map_or(0, |card| *card.lines().end()),
        }
    }
}

impl std::fmt::Debug for OutlineEntry<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OutlineEntry")
            .field("kind", &self.kind())
            .field("label", &self.label())
            .field("name", &self.name())
            .field("detail", &self.detail())
            .field("keyword", &self.keyword())
            .field("scope", &self.scope())
            .field("line", &self.line())
            .field("column", &self.column())
            .field("end_line", &self.end_line())
            .finish()
    }
}

impl PartialEq for OutlineEntry<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.kind() == other.kind()
            && self.label() == other.label()
            && self.name() == other.name()
            && self.detail() == other.detail()
            && self.keyword() == other.keyword()
            && self.scope() == other.scope()
            && self.line() == other.line()
            && self.column() == other.column()
            && self.end_line() == other.end_line()
    }
}

/// `text` without the blanks around it, borrowed where it is.
fn trimmed(text: Cow<'_, str>) -> Cow<'_, str> {
    match text {
        Cow::Borrowed(text) => Cow::Borrowed(text.trim()),
        Cow::Owned(text) => Cow::Owned(text.trim().to_owned()),
    }
}

const fn symbol_kind(kind: OutlineEntryKind) -> DeckSymbolKind {
    match kind {
        OutlineEntryKind::Model => DeckSymbolKind::Model,
        OutlineEntryKind::Subcircuit => DeckSymbolKind::Subcircuit,
        _ => DeckSymbolKind::Function,
    }
}

/// The entries of an outline, read as they are asked for.
#[derive(Clone, Copy)]
pub struct OutlineEntries<'a> {
    deck: &'a DeckIndex,
    recs: &'a [EntryRec],
}

impl<'a> OutlineEntries<'a> {
    #[must_use]
    pub const fn len(&self) -> usize {
        self.recs.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.recs.is_empty()
    }

    #[must_use]
    pub fn get(&self, position: usize) -> Option<OutlineEntry<'a>> {
        let deck = self.deck;
        self.recs
            .get(position)
            .map(|rec| OutlineEntry { deck, rec })
    }

    #[must_use]
    pub fn first(&self) -> Option<OutlineEntry<'a>> {
        self.get(0)
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = OutlineEntry<'a>> + 'a {
        let deck = self.deck;
        self.recs.iter().map(move |rec| OutlineEntry { deck, rec })
    }
}

impl<'a> IntoIterator for OutlineEntries<'a> {
    type Item = OutlineEntry<'a>;
    type IntoIter = Box<dyn DoubleEndedIterator<Item = OutlineEntry<'a>> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter())
    }
}

impl std::fmt::Debug for OutlineEntries<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_list().entries(self.iter()).finish()
    }
}

/// A navigator section containing source-order entry indices.
///
/// What the navigator lists of a section, its declarations and the rows
/// they fall in under the subcircuits they are written in, is worked out
/// with the outline, where the outline is built, and shared: a frame that
/// lists a deck of a million devices reads them, it does not walk them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutlineSection {
    kind: OutlineSectionKind,
    entry_indices: Arc<[usize]>,
    /// The entries the section declares, when they are fewer than its
    /// entries: a subcircuit's end shares the subcircuits' section and
    /// declares nothing.
    declared: Option<Arc<[usize]>>,
    /// The rows the declarations are listed in when one is written inside a
    /// subcircuit; `None` when every one is at the top level.
    rows: Option<Arc<[OutlineRow]>>,
}

impl OutlineSection {
    #[must_use]
    pub const fn kind(&self) -> OutlineSectionKind {
        self.kind
    }

    #[must_use]
    pub fn entry_indices(&self) -> &[usize] {
        &self.entry_indices
    }

    /// The entries the section declares, in source order: every one but a
    /// subcircuit's end.
    #[must_use]
    pub fn declarations(&self) -> &Arc<[usize]> {
        self.declared.as_ref().unwrap_or(&self.entry_indices)
    }

    /// The rows the declarations are listed in when one of them is written
    /// inside a subcircuit, as [`rows_by_scope`] lists them; `None` when
    /// every one is at the top level and the rows are the declarations.
    #[must_use]
    pub const fn rows(&self) -> Option<&Arc<[OutlineRow]>> {
        self.rows.as_ref()
    }
}

/// One row of a section whose declarations are listed under the subcircuits
/// they are written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutlineRow {
    /// A declaration's entry, and whether it is listed under a subcircuit.
    Declaration { entry: usize, nested: bool },
    /// The entry of the `.subckt` card whose declarations follow.
    Subcircuit(usize),
}

/// The rows `declarations`, entries of `entries`, are listed in when one of
/// them is written inside a subcircuit: the top level's first, then each
/// subcircuit's under a row naming it, subcircuits in the order the deck
/// defines them. `None` when every declaration is at the top level.
#[must_use]
pub fn rows_by_scope(
    declarations: &[usize],
    entries: OutlineEntries<'_>,
) -> Option<Vec<OutlineRow>> {
    rows_of(declarations, entries.recs)
}

fn rows_of(declarations: &[usize], entries: &[EntryRec]) -> Option<Vec<OutlineRow>> {
    let scope_of = |position: &usize| {
        entries
            .get(*position)
            .map(|entry| entry.scope)
            .filter(|scope| *scope != NONE)
    };
    if declarations
        .iter()
        .all(|position| scope_of(position).is_none())
    {
        return None;
    }
    let mut inside = std::collections::BTreeMap::<u32, Vec<usize>>::new();
    let mut rows = Vec::with_capacity(declarations.len());
    for position in declarations {
        match scope_of(position) {
            Some(subcircuit) => inside.entry(subcircuit).or_default().push(*position),
            None => rows.push(OutlineRow::Declaration {
                entry: *position,
                nested: false,
            }),
        }
    }
    for (subcircuit, declared) in inside {
        rows.push(OutlineRow::Subcircuit(subcircuit as usize));
        rows.extend(declared.into_iter().map(|entry| OutlineRow::Declaration {
            entry,
            nested: true,
        }));
    }
    Some(rows)
}

/// Parsed, deterministic navigation model for exact source text.
///
/// Read off the engine's [`DeckIndex`] of the text, so a card is what the
/// engine reads as one: `+` lines continue it, its comments are not part of
/// it, and it knows the subcircuit it is written in. It holds the index,
/// shared, and an entry is a card or a name of it: what an entry says is
/// read from the text when it is asked for.
#[derive(Debug, Clone, Default)]
pub struct NetlistOutline {
    deck: DeckIndex,
    entries: Vec<EntryRec>,
    sections: Vec<OutlineSection>,
}

impl PartialEq for NetlistOutline {
    fn eq(&self, other: &Self) -> bool {
        self.entries().iter().eq(other.entries().iter()) && self.sections == other.sections
    }
}

impl Eq for NetlistOutline {}

impl NetlistOutline {
    #[must_use]
    pub fn parse(source: &str) -> Self {
        Self::parse_as(source, SourceKind::DeckRoot)
    }

    /// The outline of `source` read as the engine reads a source of `kind`.
    #[must_use]
    pub fn parse_as(source: &str, kind: SourceKind) -> Self {
        Self::from_deck(deck_index_of(Arc::new(source.to_owned()), kind))
    }

    /// The outline of the first file of `deck`.
    #[must_use]
    pub fn from_deck(deck: DeckIndex) -> Self {
        let mut entries = Vec::<EntryRec>::new();
        // The entry each `.subckt` card became, by the scope it opens.
        let mut scope_entries = vec![NONE; deck.scope_count()];
        let entry_of_scope =
            |scope: usize, scope_entries: &[u32]| scope_entries.get(scope).copied().unwrap_or(NONE);
        for card in deck.cards_of(0) {
            let kind = card.kind();
            // `.end`, `.endl` and a `simulator lang=` switch declare nothing
            // a reader goes to.
            if matches!(kind, DeckCardKind::End | DeckCardKind::EndLibrarySection)
                || (kind == DeckCardKind::Directive
                    && card.head().eq_ignore_ascii_case("simulator"))
            {
                continue;
            }
            let id = card.id() as u32;
            // A parameter card gives a row to every name it assigns, each
            // with its value, as Spectre's `parameters` lists them.
            if kind == DeckCardKind::Parameter {
                let before = entries.len();
                for parameter in card
                    .symbols()
                    .filter(|symbol| symbol.kind() == DeckSymbolKind::Parameter)
                {
                    entries.push(EntryRec {
                        kind: OutlineEntryKind::Parameter,
                        keyword: Keyword::Head,
                        card: id,
                        symbol: parameter.id() as u32,
                        scope: entry_of_scope(parameter.scope(), &scope_entries),
                    });
                }
                if entries.len() > before {
                    continue;
                }
            }
            let entry_kind = match kind {
                DeckCardKind::Title => OutlineEntryKind::Title,
                DeckCardKind::Include => OutlineEntryKind::Include,
                DeckCardKind::Library => OutlineEntryKind::Library,
                DeckCardKind::Parameter => OutlineEntryKind::Parameter,
                DeckCardKind::Model => OutlineEntryKind::Model,
                DeckCardKind::Subcircuit => OutlineEntryKind::Subcircuit,
                DeckCardKind::Function => OutlineEntryKind::Function,
                DeckCardKind::Instance => OutlineEntryKind::Device,
                DeckCardKind::Analysis => OutlineEntryKind::Analysis,
                DeckCardKind::Measurement => OutlineEntryKind::Measurement,
                DeckCardKind::Global => OutlineEntryKind::Global,
                DeckCardKind::Option => OutlineEntryKind::Option,
                DeckCardKind::Output => OutlineEntryKind::Output,
                DeckCardKind::Conditional => OutlineEntryKind::Conditional,
                DeckCardKind::EndSubcircuit => OutlineEntryKind::EndSubcircuit,
                _ => OutlineEntryKind::Control,
            };
            // A `.subckt` card is written in the scope around it, and so is
            // its `.ends`, which closes the scope it is read in.
            let mut scope = card.scope();
            if kind == DeckCardKind::EndSubcircuit {
                scope = deck
                    .scope(scope)
                    .and_then(|scope| scope.parent())
                    .unwrap_or(0);
            }
            let keyword =
                if kind == DeckCardKind::Subcircuit && card.head().eq_ignore_ascii_case("inline") {
                    Keyword::InlineSubckt
                } else {
                    Keyword::Head
                };
            let position = entries.len() as u32;
            entries.push(EntryRec {
                kind: entry_kind,
                keyword,
                card: id,
                symbol: NONE,
                scope: entry_of_scope(scope, &scope_entries),
            });
            // A subcircuit's `params:` formals are its parameters, listed
            // under it like a `parameters` statement written inside it.
            if kind == DeckCardKind::Subcircuit {
                for symbol in card.symbols() {
                    match symbol.kind() {
                        DeckSymbolKind::Subcircuit => {
                            if let Some(slot) = symbol
                                .opens()
                                .and_then(|opens| scope_entries.get_mut(opens))
                            {
                                *slot = position;
                            }
                        }
                        DeckSymbolKind::Parameter => entries.push(EntryRec {
                            kind: OutlineEntryKind::Parameter,
                            keyword: Keyword::Params,
                            card: id,
                            symbol: symbol.id() as u32,
                            scope: position,
                        }),
                        _ => {}
                    }
                }
            }
        }
        entries.shrink_to_fit();
        let sections = sections_of(&entries);
        Self {
            deck,
            entries,
            sections,
        }
    }

    #[must_use]
    pub fn entries(&self) -> OutlineEntries<'_> {
        OutlineEntries {
            deck: &self.deck,
            recs: &self.entries,
        }
    }

    #[must_use]
    pub fn sections(&self) -> &[OutlineSection] {
        &self.sections
    }

    /// The engine's index of the source, which the outline is read from.
    #[must_use]
    pub const fn deck(&self) -> &DeckIndex {
        &self.deck
    }

    #[cfg(test)]
    fn entry_at_or_before_line(&self, line: usize) -> Option<OutlineEntry<'_>> {
        self.entries()
            .iter()
            .rev()
            .find(|entry| entry.line() <= line)
    }

    #[cfg(test)]
    fn filtered_entries(&self, query: &str) -> Vec<OutlineEntry<'_>> {
        let query = query.trim().to_lowercase();
        self.entries()
            .iter()
            .filter(|entry| query.is_empty() || entry.label().to_lowercase().contains(&query))
            .collect()
    }
}

/// The navigator's sections of `entries`, in their canonical order.
fn sections_of(entries: &[EntryRec]) -> Vec<OutlineSection> {
    let canonical_order = [
        OutlineSectionKind::Source,
        OutlineSectionKind::Dependencies,
        OutlineSectionKind::Parameters,
        OutlineSectionKind::Globals,
        OutlineSectionKind::Functions,
        OutlineSectionKind::Options,
        OutlineSectionKind::Models,
        OutlineSectionKind::Subcircuits,
        OutlineSectionKind::Analyses,
        OutlineSectionKind::Measurements,
        OutlineSectionKind::Outputs,
        OutlineSectionKind::Conditionals,
        OutlineSectionKind::Controls,
        OutlineSectionKind::Devices,
    ];
    let mut by_section = vec![Vec::<usize>::new(); canonical_order.len()];
    for (index, entry) in entries.iter().enumerate() {
        by_section[entry.kind.section() as usize].push(index);
    }
    canonical_order
        .into_iter()
        .zip(by_section)
        .filter(|(_, entry_indices)| !entry_indices.is_empty())
        .map(|(kind, entry_indices)| {
            let declared = entry_indices
                .iter()
                .any(|index| entries[*index].kind == OutlineEntryKind::EndSubcircuit)
                .then(|| {
                    entry_indices
                        .iter()
                        .copied()
                        .filter(|index| entries[*index].kind != OutlineEntryKind::EndSubcircuit)
                        .collect::<Arc<[usize]>>()
                });
            let rows =
                rows_of(declared.as_deref().unwrap_or(&entry_indices), entries).map(Arc::from);
            OutlineSection {
                kind,
                entry_indices: entry_indices.into(),
                declared,
                rows,
            }
        })
        .collect()
}

/// The engine's index of `source` read as a deck of its own.
fn deck_index_of(source: SharedDeckText, kind: SourceKind) -> DeckIndex {
    DeckIndex::build(&[DeckIndexSource::new("", source, kind)])
}

/// The words of a card as written, its own first: tokens the engine's lexer
/// reads with nothing between them are one word (`v(out)`, `gain=10`), and a
/// quoted string is its text. A title, a `.control` block and a device have
/// none.
fn card_words(deck: &DeckIndex, card: &DeckCard<'_>) -> Vec<String> {
    if matches!(
        card.kind(),
        DeckCardKind::Title | DeckCardKind::Control | DeckCardKind::Instance
    ) {
        return Vec::new();
    }
    let Some(text) = deck.text(card.file()) else {
        return Vec::new();
    };
    let mut words = Vec::<String>::new();
    let mut end = None::<usize>;
    for token in card.tokens() {
        let written = text.slice(token.span.clone());
        let written = if token.kind == DeckTokenKind::String {
            written.trim_matches('"')
        } else {
            &written
        };
        match words.last_mut() {
            Some(word) if end == Some(token.span.start) => word.push_str(written),
            _ => words.push(written.to_owned()),
        }
        end = Some(token.span.end);
    }
    words
}

/// One exact source and its outline, which reads any card back by line.
///
/// Parsing a deck costs the deck. The navigator projects the outline on every
/// frame and materializes a card for every declaration it draws, so both are
/// done once per change here rather than once per frame at the call site.
/// It holds no copy of the source: the engine's index of it shares it, and
/// the outline reads from that.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetlistSourceIndex {
    outline: NetlistOutline,
    kind: SourceKind,
}

impl Default for NetlistSourceIndex {
    fn default() -> Self {
        Self {
            outline: NetlistOutline::default(),
            kind: SourceKind::DeckRoot,
        }
    }
}

/// The index of a large deck holds large tables: whoever lets go of it last,
/// it is freed away from that thread.
impl Drop for NetlistSourceIndex {
    fn drop(&mut self) {
        crate::freed_away::free_away(self);
    }
}

impl NetlistSourceIndex {
    /// The index of `source` read as a root deck.
    #[must_use]
    pub fn parse(source: &str) -> Self {
        Self::parse_as(source, SourceKind::DeckRoot)
    }

    /// The index of `source` read as the engine reads a source of `kind`.
    #[must_use]
    pub fn parse_as(source: &str, kind: SourceKind) -> Self {
        Self::of_text(Arc::new(source.to_owned()), kind)
    }

    /// The index of `text`, shared rather than copied, read as the engine
    /// reads a source of `kind`.
    #[must_use]
    pub fn of_text(text: SharedDeckText, kind: SourceKind) -> Self {
        Self::of_deck(deck_index_of(text, kind), kind)
    }

    /// The index of the first file of `deck`, read as a source of `kind`.
    fn of_deck(deck: DeckIndex, kind: SourceKind) -> Self {
        Self {
            outline: NetlistOutline::from_deck(deck),
            kind,
        }
    }

    /// How the engine reads the source.
    #[must_use]
    pub const fn kind(&self) -> SourceKind {
        self.kind
    }

    /// The index of `text`, which is this index's text after `change`: the
    /// part of the engine's index the edit touched read again and the rest
    /// shared, or all of it read when the edit changes what the deck
    /// defines.
    #[must_use]
    pub fn edited(&self, change: DeckTextChange, text: SharedDeckText) -> Self {
        match self.deck().edited(0, change, Arc::clone(&text)) {
            Some(deck) => Self::of_deck(deck, self.kind),
            None => Self::of_text(text, self.kind),
        }
    }

    /// Whether this index still describes `source`, which is the only thing
    /// that makes it usable. A byte comparison cannot go stale behind a writer
    /// that forgets to bump a revision counter.
    #[must_use]
    pub fn describes(&self, source: &str) -> bool {
        let Some(text) = self.deck().text(0) else {
            return source.is_empty();
        };
        if text.len() != source.len() {
            return false;
        }
        let mut at = 0;
        text.pieces(0).all(|piece| {
            let same = source.as_bytes().get(at..at + piece.len()) == Some(piece.as_bytes());
            at += piece.len();
            same
        })
    }

    #[must_use]
    pub const fn outline(&self) -> &NetlistOutline {
        &self.outline
    }

    /// The engine's index of the source, which the outline is read from.
    #[must_use]
    pub const fn deck(&self) -> &DeckIndex {
        self.outline.deck()
    }

    /// How many lines the source has, as `str::lines` counts them.
    #[must_use]
    pub fn line_count(&self) -> usize {
        self.deck().line_count(0)
    }

    /// The card at a one-based line, exactly as `str::lines` would yield it.
    #[must_use]
    pub fn card(&self, line: usize) -> Cow<'_, str> {
        let deck = self.deck();
        let (Some(span), Some(text)) = (deck.line_span(0, line), deck.text(0)) else {
            return Cow::Borrowed("");
        };
        let card = text.slice(span);
        let end = card
            .strip_suffix('\n')
            .map(|card| card.strip_suffix('\r').unwrap_or(card))
            .map_or(card.len(), str::len);
        match card {
            Cow::Borrowed(card) => Cow::Borrowed(&card[..end]),
            Cow::Owned(mut card) => {
                card.truncate(end);
                Cow::Owned(card)
            }
        }
    }
}

/// Direct source include/library card.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IncludeDirective {
    kind: IncludeKind,
    locator: String,
    section: Option<String>,
    line: usize,
    column: usize,
}

impl IncludeDirective {
    #[must_use]
    pub const fn kind(&self) -> IncludeKind {
        self.kind
    }

    #[must_use]
    pub fn locator(&self) -> &str {
        &self.locator
    }

    #[must_use]
    pub fn section(&self) -> Option<&str> {
        self.section.as_deref()
    }

    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }

    #[must_use]
    pub const fn column(&self) -> usize {
        self.column
    }
}

/// Whether two include-card lists describe the same dependency graph.
///
/// The graph is which files a source pulls in and from where in that source.
/// Which section of a library card the deck binds is a choice inside one of
/// those files, not an edge: rewriting it leaves every retained dependency's
/// direct-include index pointing at the same card, which is what an in-place
/// source replacement needs in order to keep the closure it already resolved.
#[must_use]
pub fn same_include_graph(left: &[IncludeDirective], right: &[IncludeDirective]) -> bool {
    left.len() == right.len()
        && std::iter::zip(left, right).all(|(left, right)| {
            left.kind == right.kind
                && left.locator == right.locator
                && left.line == right.line
                && left.column == right.column
        })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IncludeKind {
    Include,
    Library,
    VerilogA,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CardToken {
    text: String,
    column: usize,
}

pub fn parse_include_directives(source: &str) -> Vec<IncludeDirective> {
    source
        .lines()
        .enumerate()
        .filter_map(|(zero_line, raw_line)| {
            let trimmed = raw_line.trim_start();
            if trimmed.is_empty() || is_comment(trimmed) || trimmed.starts_with('+') {
                return None;
            }
            let tokens = tokenize_card(trimmed);
            let head = tokens.first()?.text.to_ascii_lowercase();
            let kind = match head.as_str() {
                ".include" | ".inc" | "`include" => IncludeKind::Include,
                // `.lib section` opens an in-file library section. A source
                // dependency requires both a locator and a section token.
                ".lib" if tokens.len() >= 3 => IncludeKind::Library,
                // LTspice's and PSpice's one-argument `.lib file` includes
                // the whole file, as the engine's include reader does.
                ".lib" if tokens.len() == 2 && names_model_file(&tokens[1].text) => {
                    IncludeKind::Include
                }
                ".veriloga" => IncludeKind::VerilogA,
                _ => return None,
            };
            let locator = tokens.get(1)?.text.clone();
            if locator.is_empty() {
                return None;
            }
            Some(IncludeDirective {
                kind,
                locator,
                section: (kind == IncludeKind::Library)
                    .then(|| tokens.get(2).map(|token| token.text.clone()))
                    .flatten(),
                line: zero_line + 1,
                column: raw_line.chars().take_while(|ch| ch.is_whitespace()).count()
                    + tokens[1].column,
            })
        })
        .collect()
}

/// Whether a one-argument `.lib` names a file rather than an in-file section:
/// it carries a path or a model-source extension, which no section name does.
fn names_model_file(argument: &str) -> bool {
    argument.contains(['/', '\\'])
        || std::path::Path::new(argument)
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| {
                rspice_core_netlist::netlist::MODEL_SOURCE_EXTENSIONS
                    .iter()
                    .any(|known| extension.eq_ignore_ascii_case(known))
            })
}

/// The whitespace-separated words of one source card, in order, with quotes
/// and trailing comments removed.
pub fn card_tokens(card: &str) -> Vec<String> {
    tokenize_card(card.trim_start())
        .into_iter()
        .map(|token| token.text)
        .collect()
}

/// Token text and one-based Unicode column for source-intelligence actions
/// that must identify the exact declaration/reference token without
/// retokenizing SPICE differently from the outline.
pub fn card_tokens_with_columns(card: &str) -> Vec<(String, usize)> {
    let trimmed = card.trim_start();
    let leading_columns = card.chars().take_while(|ch| ch.is_whitespace()).count();
    tokenize_card(trimmed)
        .into_iter()
        .map(|token| (token.text, token.column + leading_columns))
        .collect()
}

/// Whether a card is a comment as a whole, by the engine's comment rule.
fn is_comment(trimmed: &str) -> bool {
    comment_start(trimmed, SourceKind::Included) == Some(0)
}

/// Tokenize a SPICE card without normalizing or modifying the source. Both
/// quote styles are accepted and the returned text excludes the quote marks.
/// The card ends where the engine's comment rule starts its comment.
fn tokenize_card(card: &str) -> Vec<CardToken> {
    let card = &card[..comment_start(card, SourceKind::Included).unwrap_or(card.len())];
    let mut tokens = Vec::new();
    let mut chars = card.char_indices().peekable();

    while let Some((offset, ch)) = chars.next() {
        if ch.is_whitespace() {
            continue;
        }

        let start_column = card[..offset].chars().count() + 1;
        let mut text = String::new();
        if ch == '\'' || ch == '"' {
            let quote = ch;
            for (_, current) in chars.by_ref() {
                if current == quote {
                    break;
                }
                text.push(current);
            }
        } else {
            text.push(ch);
            while let Some((_, current)) = chars.peek().copied() {
                if current.is_whitespace() {
                    break;
                }
                chars.next();
                text.push(current);
            }
        }
        tokens.push(CardToken {
            text,
            column: start_column,
        });
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outline_is_grouped_canonically_while_entries_remain_in_source_order() {
        let source = "Precision amplifier\n.include \"vendor models/core.lib\"\n.param gain=10\nR1 in out 1k\n+ tc=1m\n.subckt amp in out\nM1 out in 0 0 nch\n.ends amp\n.ac dec 10 1 1g\n.meas ac peak max v(out)\n.end\n";
        let outline = NetlistOutline::parse(source);

        let entries = outline.entries();
        assert_eq!(entries.get(0).unwrap().kind(), OutlineEntryKind::Title);
        assert_eq!(entries.get(3).unwrap().label(), "R1");
        assert_eq!(entries.get(3).unwrap().end_line(), 5);
        assert_eq!(entries.get(4).unwrap().line(), 6);
        assert_eq!(
            outline
                .sections()
                .iter()
                .map(OutlineSection::kind)
                .collect::<Vec<_>>(),
            vec![
                OutlineSectionKind::Source,
                OutlineSectionKind::Dependencies,
                OutlineSectionKind::Parameters,
                OutlineSectionKind::Subcircuits,
                OutlineSectionKind::Analyses,
                OutlineSectionKind::Measurements,
                OutlineSectionKind::Devices,
            ]
        );
    }

    #[test]
    fn a_one_argument_lib_naming_a_file_is_an_include_and_a_bare_name_opens_a_section() {
        let source = "deck\n.LIB \"../../forward_slash.lib\"\n.lib ADI.lib\n.lib TT\n.end\n";
        let includes = parse_include_directives(source);

        assert_eq!(includes.len(), 2);
        assert_eq!(includes[0].kind(), IncludeKind::Include);
        assert_eq!(includes[0].locator(), "../../forward_slash.lib");
        assert_eq!(includes[1].kind(), IncludeKind::Include);
        assert_eq!(includes[1].locator(), "ADI.lib");
    }

    #[test]
    fn quoted_include_paths_and_library_sections_are_exact() {
        let source = "deck\n  .include \"models/Si Ge.lib\"\n.lib 'corners/process.lib' TT\n.lib LOCAL\n.veriloga \"models/device core.va\"\n`include \"constants.vams\"\n.end\n";
        let includes = parse_include_directives(source);

        assert_eq!(includes.len(), 4);
        assert_eq!(includes[0].kind(), IncludeKind::Include);
        assert_eq!(includes[0].locator(), "models/Si Ge.lib");
        assert_eq!(includes[0].line(), 2);
        assert_eq!(includes[0].column(), 12);
        assert_eq!(includes[1].kind(), IncludeKind::Library);
        assert_eq!(includes[1].locator(), "corners/process.lib");
        assert_eq!(includes[1].section(), Some("TT"));
        assert_eq!(includes[2].kind(), IncludeKind::VerilogA);
        assert_eq!(includes[2].locator(), "models/device core.va");
        assert_eq!(includes[2].section(), None);
        assert_eq!(includes[2].line(), 5);
        assert_eq!(includes[3].kind(), IncludeKind::Include);
        assert_eq!(includes[3].locator(), "constants.vams");
        assert_eq!(includes[3].line(), 6);

        let outline = NetlistOutline::parse(source);
        assert_eq!(
            outline
                .entries()
                .iter()
                .find(|entry| entry.label().contains("device core.va"))
                .map(|entry| entry.kind()),
            Some(OutlineEntryKind::Include)
        );
    }

    #[test]
    fn navigator_filter_and_line_lookup_are_unicode_safe() {
        let outline = NetlistOutline::parse("Δ amplifier\n.param μ=2\nRμ n 0 1k\n.end\n");
        assert_eq!(outline.filtered_entries("Μ").len(), 2);
        assert_eq!(
            outline
                .entry_at_or_before_line(3)
                .map(|entry| entry.label().into_owned())
                .as_deref(),
            Some("Rμ")
        );
        assert_eq!(outline.entry_at_or_before_line(0), None);
    }

    #[test]
    fn semantic_control_categories_are_not_collapsed_into_generic_directives() {
        let outline = NetlistOutline::parse(
            "deck\n.global vdd vss\n.global_param corner=1\n.func square(x) {x*x}\n.options reltol=1e-5\n.if corner\n.save v(out)\n.else\n.probe i(vdd)\n.endif\n.end\n",
        );
        let kinds = outline
            .entries()
            .iter()
            .map(|entry| entry.kind())
            .collect::<Vec<_>>();
        assert!(kinds.contains(&OutlineEntryKind::Global));
        assert!(kinds.contains(&OutlineEntryKind::Function));
        assert!(kinds.contains(&OutlineEntryKind::Option));
        assert_eq!(
            kinds
                .iter()
                .filter(|kind| **kind == OutlineEntryKind::Conditional)
                .count(),
            3
        );
        assert_eq!(
            kinds
                .iter()
                .filter(|kind| **kind == OutlineEntryKind::Output)
                .count(),
            2
        );
    }

    #[test]
    fn a_declaration_inside_a_subcircuit_names_the_subcircuit_it_is_written_in() {
        let outline = NetlistOutline::parse(
            "amp test\n.param n=10\nC1 a 0 10n ; load\n.subckt amp in out params: gain=1\nR1 in out\n+ {gain}\n.model nch nmos level=1\n.ends amp\nX1 a b amp\n.end\n",
        );
        let entries = outline.entries();
        let named = |name: &str| {
            entries
                .iter()
                .position(|entry| entry.name() == name)
                .unwrap_or_else(|| panic!("no entry named {name}: {entries:?}"))
        };
        let entry = |position: usize| entries.get(position).expect("an entry");
        let amp = named("amp");
        assert_eq!(entry(amp).kind(), OutlineEntryKind::Subcircuit);
        assert_eq!(entry(amp).detail().as_deref(), Some("2 ports"));
        assert_eq!(entry(amp).scope(), None);

        let resistor = entry(named("R1"));
        assert_eq!(resistor.scope(), Some(amp));
        assert_eq!((resistor.line(), resistor.end_line()), (5, 6));
        assert_eq!(resistor.detail().as_deref(), Some("{gain}"));
        let model = entry(named("nch"));
        assert_eq!(model.scope(), Some(amp));
        assert_eq!(model.detail().as_deref(), Some("nmos"));

        let instance = entry(named("X1"));
        assert_eq!(instance.scope(), None);
        assert_eq!(instance.detail().as_deref(), Some("amp"));
        let capacitor = entry(named("C1"));
        assert_eq!(capacitor.detail().as_deref(), Some("10n"));
        let parameter = entry(named("n"));
        assert_eq!(parameter.detail().as_deref(), Some("10"));
        assert!(
            entries
                .iter()
                .filter(|entry| entry.kind() == OutlineEntryKind::EndSubcircuit)
                .all(|entry| entry.scope().is_none())
        );
    }

    #[test]
    fn a_parameter_card_gives_each_name_its_own_row_with_its_value() {
        let outline = NetlistOutline::parse(
            "amp test\n.param vdd=1.8 gain=2\n+ bias=0.6\n.subckt amp in out params: w=2u\n.ends amp\n.end\n",
        );
        let parameters = outline
            .entries()
            .iter()
            .filter(|entry| entry.kind() == OutlineEntryKind::Parameter)
            .map(|entry| {
                (
                    entry.name().into_owned(),
                    entry.detail().map(Cow::into_owned),
                    entry.line(),
                    entry.scope(),
                )
            })
            .collect::<Vec<_>>();
        let parameters = parameters
            .iter()
            .map(|(name, detail, line, scope)| (name.as_str(), detail.as_deref(), *line, *scope))
            .collect::<Vec<_>>();
        let amp = outline
            .entries()
            .iter()
            .position(|entry| entry.kind() == OutlineEntryKind::Subcircuit);
        assert_eq!(
            parameters,
            [
                ("vdd", Some("1.8"), 2, None),
                ("gain", Some("2"), 2, None),
                ("bias", Some("0.6"), 3, None),
                ("w", Some("2u"), 4, amp),
            ]
        );
    }

    #[test]
    fn a_subcircuit_is_declared_with_the_decks_own_word() {
        let spice = NetlistOutline::parse("deck\n.SUBCKT amp in out\n.ENDS amp\n.end\n");
        let spectre = NetlistOutline::parse_as(
            "subckt amp (in out)\nparameters gain=2\nends amp\n",
            SourceKind::Spectre,
        );
        let keyword = |outline: &NetlistOutline| {
            outline
                .entries()
                .iter()
                .find(|entry| entry.kind() == OutlineEntryKind::Subcircuit)
                .map(|entry| entry.keyword().into_owned())
        };
        assert_eq!(keyword(&spice).as_deref(), Some(".subckt"));
        assert_eq!(keyword(&spectre).as_deref(), Some("subckt"));
        let gain = spectre
            .entries()
            .iter()
            .find(|entry| entry.name() == "gain")
            .expect("the parameter is listed");
        assert_eq!(
            (gain.detail().as_deref(), gain.scope()),
            (Some("2"), Some(0))
        );
    }
}
