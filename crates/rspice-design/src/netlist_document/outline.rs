//! Netlist outline.
//!
//! The navigable structure of a deck — subcircuits, models, analyses,
//! includes — for the outline pane.

use rspice_core_netlist::netlist::{
    DeckCardKind, DeckIndex, DeckIndexSource, DeckSymbol, DeckSymbolKind, DeckTokenKind,
    SourceKind, comment_start,
};
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

/// One navigable source declaration. Lines and columns are one-based Unicode
/// scalar positions; `end_line` includes continuation cards.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutlineEntry {
    id: u64,
    kind: OutlineEntryKind,
    label: String,
    /// What the declaration names: a parameter, a model, a device, an
    /// analysis.
    name: String,
    /// What it is bound to or written with: a parameter's value, a model's
    /// type, the subcircuit or model a device is of, an analysis's arguments.
    detail: Option<String>,
    /// The entry of the `.subckt` card the declaration is written inside.
    scope: Option<usize>,
    line: usize,
    column: usize,
    end_line: usize,
}

impl OutlineEntry {
    #[must_use]
    pub const fn kind(&self) -> OutlineEntryKind {
        self.kind
    }

    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }

    /// The position, among the outline's entries, of the `.subckt` card the
    /// declaration is written inside; `None` at the top level.
    #[must_use]
    pub const fn scope(&self) -> Option<usize> {
        self.scope
    }

    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }

    #[must_use]
    pub const fn column(&self) -> usize {
        self.column
    }

    #[must_use]
    pub const fn end_line(&self) -> usize {
        self.end_line
    }
}

/// A navigator section containing source-order entry indices.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutlineSection {
    kind: OutlineSectionKind,
    entry_indices: Vec<usize>,
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
}

/// Parsed, deterministic navigation model for exact source text.
///
/// Read off the engine's [`DeckIndex`] of the text, so a card is what the
/// engine reads as one: `+` lines continue it, its comments are not part of
/// it, and it knows the subcircuit it is written in.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct NetlistOutline {
    entries: Vec<OutlineEntry>,
    sections: Vec<OutlineSection>,
}

impl NetlistOutline {
    #[must_use]
    pub fn parse(source: &str) -> Self {
        Self::parse_as(source, SourceKind::DeckRoot)
    }

    /// The outline of `source` read as the engine reads a source of `kind`.
    #[must_use]
    pub fn parse_as(source: &str, kind: SourceKind) -> Self {
        Self::from_deck(&deck_index_of(source, kind), source)
    }

    /// The outline of the deck `deck` indexed as its first file, `source`.
    #[must_use]
    pub fn from_deck(deck: &DeckIndex, source: &str) -> Self {
        let mut entries = Vec::<OutlineEntry>::new();
        // The entry each `.subckt` card became, by card.
        let mut subcircuit_entries = std::collections::HashMap::<usize, usize>::new();
        // The definitions each card writes, read once rather than per card.
        let mut defined_on = std::collections::HashMap::<usize, Vec<&DeckSymbol>>::new();
        for symbol in deck.symbols() {
            defined_on.entry(symbol.card).or_default().push(symbol);
        }
        for (card_index, card) in deck.cards().iter().enumerate() {
            if card.location.file != 0 {
                break;
            }
            let on_card = defined_on.get(&card_index).map_or(&[][..], Vec::as_slice);
            let defined = |kind: DeckSymbolKind| {
                on_card
                    .iter()
                    .copied()
                    .filter(move |symbol| symbol.kind == kind)
            };
            let words = card_words(deck, card_index, source);
            let word = |at: usize| words.get(at).cloned();
            let directive_label = match words.get(1) {
                Some(argument) => format!("{} {argument}", card.head),
                None => card.head.clone(),
            };
            let (kind, label, name, detail) = match card.kind {
                DeckCardKind::Title => {
                    let title = card.head.trim().to_owned();
                    (OutlineEntryKind::Title, title.clone(), title, None)
                }
                DeckCardKind::Include | DeckCardKind::Library => {
                    let path = deck
                        .includes()
                        .iter()
                        .find(|include| include.card == card_index)
                        .map(|include| include.path.clone())
                        .or_else(|| word(1));
                    let label = path
                        .as_ref()
                        .map_or_else(|| card.head.clone(), |path| format!("{} {path}", card.head));
                    let kind = if card.kind == DeckCardKind::Include {
                        OutlineEntryKind::Include
                    } else {
                        OutlineEntryKind::Library
                    };
                    (kind, label, path.unwrap_or_default(), None)
                }
                DeckCardKind::Parameter => {
                    let parameters = defined(DeckSymbolKind::Parameter).collect::<Vec<_>>();
                    let (name, detail) = match parameters.as_slice() {
                        [] => (directive_label.clone(), None),
                        // One name is shown with what it is bound to; several
                        // are listed without, or the row would give the first
                        // value to all of them.
                        [parameter] => (
                            parameter.name.clone(),
                            (!parameter.detail.is_empty()).then(|| parameter.detail.clone()),
                        ),
                        several => (
                            several
                                .iter()
                                .map(|parameter| parameter.name.as_str())
                                .collect::<Vec<_>>()
                                .join(", "),
                            None,
                        ),
                    };
                    (OutlineEntryKind::Parameter, directive_label, name, detail)
                }
                DeckCardKind::Model | DeckCardKind::Subcircuit | DeckCardKind::Function => {
                    let (kind, symbol_kind) = match card.kind {
                        DeckCardKind::Model => (OutlineEntryKind::Model, DeckSymbolKind::Model),
                        DeckCardKind::Subcircuit => {
                            (OutlineEntryKind::Subcircuit, DeckSymbolKind::Subcircuit)
                        }
                        _ => (OutlineEntryKind::Function, DeckSymbolKind::Function),
                    };
                    match defined(symbol_kind).next() {
                        Some(symbol) => (
                            kind,
                            format!("{} {}", card.head, symbol.name),
                            symbol.name.clone(),
                            (!symbol.detail.is_empty()).then(|| symbol.detail.clone()),
                        ),
                        None => (kind, directive_label.clone(), directive_label, None),
                    }
                }
                DeckCardKind::Instance => {
                    let detail = defined(DeckSymbolKind::Instance)
                        .next()
                        .map(|instance| instance.detail.clone())
                        .filter(|detail| !detail.is_empty());
                    (
                        OutlineEntryKind::Device,
                        card.head.clone(),
                        card.head.clone(),
                        detail,
                    )
                }
                DeckCardKind::Analysis => (
                    OutlineEntryKind::Analysis,
                    directive_label,
                    card.head.clone(),
                    (words.len() > 1).then(|| words[1..].join(" ")),
                ),
                // `.meas tran rise when v(out)=0.5`: the measurement's name,
                // and the analysis it reads.
                DeckCardKind::Measurement => (
                    OutlineEntryKind::Measurement,
                    directive_label.clone(),
                    word(2).unwrap_or(directive_label),
                    word(1),
                ),
                other => {
                    let kind = match other {
                        DeckCardKind::Global => OutlineEntryKind::Global,
                        DeckCardKind::Option => OutlineEntryKind::Option,
                        DeckCardKind::Output => OutlineEntryKind::Output,
                        DeckCardKind::Conditional => OutlineEntryKind::Conditional,
                        DeckCardKind::EndSubcircuit => OutlineEntryKind::EndSubcircuit,
                        _ => OutlineEntryKind::Control,
                    };
                    (kind, directive_label.clone(), directive_label, None)
                }
            };
            // A `.subckt` card is written in the scope around it, and so is
            // its `.ends`, which closes the scope it is read in.
            let mut scope = card.scope;
            if card.kind == DeckCardKind::EndSubcircuit {
                scope = deck
                    .scopes()
                    .get(scope)
                    .and_then(|scope| scope.parent)
                    .unwrap_or(0);
            }
            let scope = deck
                .scopes()
                .get(scope)
                .and_then(|scope| scope.symbol)
                .and_then(|symbol| deck.symbols().get(symbol))
                .and_then(|symbol| subcircuit_entries.get(&symbol.card).copied());
            if card.kind == DeckCardKind::Subcircuit {
                subcircuit_entries.insert(card_index, entries.len());
            }
            entries.push(OutlineEntry {
                id: entries.len() as u64 + 1,
                kind,
                label,
                name,
                detail,
                scope,
                line: *card.lines.start(),
                column: card.location.column,
                end_line: *card.lines.end(),
            });
        }

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
        let sections = canonical_order
            .into_iter()
            .filter_map(|kind| {
                let entry_indices = entries
                    .iter()
                    .enumerate()
                    .filter_map(|(index, entry)| (entry.kind.section() == kind).then_some(index))
                    .collect::<Vec<_>>();
                (!entry_indices.is_empty()).then_some(OutlineSection {
                    kind,
                    entry_indices,
                })
            })
            .collect();

        Self { entries, sections }
    }

    #[must_use]
    pub fn entries(&self) -> &[OutlineEntry] {
        &self.entries
    }

    #[must_use]
    pub fn sections(&self) -> &[OutlineSection] {
        &self.sections
    }

    #[cfg(test)]
    fn entry(&self, id: u64) -> Option<&OutlineEntry> {
        id.checked_sub(1)
            .and_then(|index| usize::try_from(index).ok())
            .and_then(|index| self.entries.get(index))
            .filter(|entry| entry.id == id)
    }

    #[cfg(test)]
    fn entry_at_or_before_line(&self, line: usize) -> Option<&OutlineEntry> {
        self.entries.iter().rev().find(|entry| entry.line <= line)
    }

    #[cfg(test)]
    fn filtered_entries(&self, query: &str) -> Vec<&OutlineEntry> {
        let query = query.trim().to_lowercase();
        if query.is_empty() {
            return self.entries.iter().collect();
        }
        self.entries
            .iter()
            .filter(|entry| entry.label.to_lowercase().contains(&query))
            .collect()
    }
}

/// The engine's index of `source` read as a deck of its own.
fn deck_index_of(source: &str, kind: SourceKind) -> DeckIndex {
    DeckIndex::build(&[DeckIndexSource {
        identity: "",
        text: source,
        kind,
    }])
}

/// The words of a card as written, its own first: tokens the engine's lexer
/// reads with nothing between them are one word (`v(out)`, `gain=10`), and a
/// quoted string is its text.
fn card_words(deck: &DeckIndex, card: usize, source: &str) -> Vec<String> {
    let Some(head) = deck.cards().get(card) else {
        return Vec::new();
    };
    if matches!(
        head.kind,
        DeckCardKind::Title | DeckCardKind::Control | DeckCardKind::Instance
    ) {
        return Vec::new();
    }
    let mut words = Vec::<String>::new();
    let mut end = None::<usize>;
    for token in deck.card_tokens(card, source) {
        let Some(written) = source.get(token.span.clone()) else {
            continue;
        };
        let written = if token.kind == DeckTokenKind::String {
            written.trim_matches('"')
        } else {
            written
        };
        match words.last_mut() {
            Some(word) if end == Some(token.span.start) => word.push_str(written),
            _ => words.push(written.to_owned()),
        }
        end = Some(token.span.end);
    }
    words
}

/// One exact source, its outline, and the offsets that let a caller read any
/// card back by line.
///
/// Parsing a deck costs the deck. The navigator projects the outline on every
/// frame and materializes a card for every declaration it draws, so both are
/// done once per change here rather than once per frame at the call site.
/// Nothing is stored that cannot be derived from `source`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NetlistSourceIndex {
    source: String,
    /// Byte offset of each line, one entry per line `str::lines` would yield.
    line_starts: Vec<usize>,
    deck: DeckIndex,
    outline: NetlistOutline,
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
        let mut line_starts = vec![0usize];
        for (offset, byte) in source.bytes().enumerate() {
            if byte == b'\n' {
                line_starts.push(offset + 1);
            }
        }
        // `str::lines` yields nothing after a terminating newline, and nothing
        // at all for empty text.
        if source.is_empty() {
            line_starts.clear();
        } else if source.ends_with('\n') {
            line_starts.pop();
        }
        let deck = deck_index_of(source, kind);
        let outline = NetlistOutline::from_deck(&deck, source);
        Self {
            source: source.to_owned(),
            line_starts,
            deck,
            outline,
        }
    }

    /// Whether this index still describes `source`, which is the only thing
    /// that makes it usable. A byte comparison cannot go stale behind a writer
    /// that forgets to bump a revision counter.
    #[must_use]
    pub fn describes(&self, source: &str) -> bool {
        self.source == source
    }

    #[must_use]
    pub const fn outline(&self) -> &NetlistOutline {
        &self.outline
    }

    /// The engine's index of the source, which the outline is read from.
    #[must_use]
    pub const fn deck(&self) -> &DeckIndex {
        &self.deck
    }

    #[must_use]
    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }

    /// The card at a one-based line, exactly as `str::lines` would yield it.
    #[must_use]
    pub fn card(&self, line: usize) -> &str {
        let Some(start) = line
            .checked_sub(1)
            .and_then(|index| self.line_starts.get(index))
        else {
            return "";
        };
        let end = self
            .line_starts
            .get(line)
            .copied()
            .unwrap_or(self.source.len());
        let card = &self.source[*start..end];
        let card = card.strip_suffix('\n').unwrap_or(card);
        card.strip_suffix('\r').unwrap_or(card)
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

        assert_eq!(outline.entries()[0].kind(), OutlineEntryKind::Title);
        assert_eq!(outline.entries()[3].label(), "R1");
        assert_eq!(outline.entries()[3].end_line(), 5);
        assert_eq!(outline.entries()[4].line(), 6);
        assert_eq!(outline.entry(1), outline.entries().first());
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
                OutlineSectionKind::Controls,
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
                .map(OutlineEntry::kind),
            Some(OutlineEntryKind::Include)
        );
    }

    #[test]
    fn navigator_filter_and_line_lookup_are_unicode_safe() {
        let outline = NetlistOutline::parse("Δ amplifier\n.param μ=2\nRμ n 0 1k\n.end\n");
        assert_eq!(outline.filtered_entries("Μ").len(), 2);
        assert_eq!(
            outline.entry_at_or_before_line(3).map(OutlineEntry::label),
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
            .map(OutlineEntry::kind)
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
        let amp = named("amp");
        assert_eq!(entries[amp].kind(), OutlineEntryKind::Subcircuit);
        assert_eq!(entries[amp].detail(), Some("2 ports"));
        assert_eq!(entries[amp].scope(), None);

        let resistor = &entries[named("R1")];
        assert_eq!(resistor.scope(), Some(amp));
        assert_eq!((resistor.line(), resistor.end_line()), (5, 6));
        assert_eq!(resistor.detail(), Some("{gain}"));
        let model = &entries[named("nch")];
        assert_eq!(model.scope(), Some(amp));
        assert_eq!(model.detail(), Some("nmos"));

        let instance = &entries[named("X1")];
        assert_eq!(instance.scope(), None);
        assert_eq!(instance.detail(), Some("amp"));
        let capacitor = &entries[named("C1")];
        assert_eq!(capacitor.detail(), Some("10n"));
        let parameter = &entries[named("n")];
        assert_eq!(parameter.detail(), Some("10"));
        assert!(
            entries
                .iter()
                .filter(|entry| entry.kind() == OutlineEntryKind::EndSubcircuit)
                .all(|entry| entry.scope().is_none())
        );
    }
}
