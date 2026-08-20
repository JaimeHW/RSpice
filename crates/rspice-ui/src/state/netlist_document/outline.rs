//! Netlist outline.
//!
//! The navigable structure of a deck — subcircuits, models, analyses,
//! includes — for the outline pane.

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
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct NetlistOutline {
    entries: Vec<OutlineEntry>,
    sections: Vec<OutlineSection>,
}

impl NetlistOutline {
    #[must_use]
    pub fn parse(source: &str) -> Self {
        let mut entries = Vec::<OutlineEntry>::new();
        let mut first_source_card = true;

        for (zero_line, raw_line) in source.lines().enumerate() {
            let line = zero_line + 1;
            let trimmed = raw_line.trim_start();
            if trimmed.is_empty() || is_comment(trimmed) {
                continue;
            }

            if trimmed.starts_with('+') {
                if let Some(previous) = entries.last_mut() {
                    previous.end_line = line;
                }
                continue;
            }

            let column = raw_line.chars().take_while(|ch| ch.is_whitespace()).count() + 1;
            let tokens = tokenize_card(trimmed);
            if tokens.is_empty() {
                continue;
            }

            let head = tokens[0].text.as_str();
            let (kind, label) = if first_source_card && !head.starts_with('.') {
                first_source_card = false;
                (OutlineEntryKind::Title, trimmed.to_owned())
            } else {
                first_source_card = false;
                classify_card(&tokens, trimmed)
            };
            entries.push(OutlineEntry {
                id: entries.len() as u64 + 1,
                kind,
                label,
                line,
                column,
                end_line: line,
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
    outline: NetlistOutline,
}

impl NetlistSourceIndex {
    #[must_use]
    pub fn parse(source: &str) -> Self {
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
        Self {
            source: source.to_owned(),
            line_starts,
            outline: NetlistOutline::parse(source),
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
pub(crate) fn same_include_graph(left: &[IncludeDirective], right: &[IncludeDirective]) -> bool {
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

pub(crate) fn parse_include_directives(source: &str) -> Vec<IncludeDirective> {
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

/// The tokens of one source card, in order, with quotes and trailing comments
/// removed. An outline entry keeps only the head and one detail token, which
/// is enough to name a declaration and not enough to describe it — a navigator
/// that wants the model a device binds to reads the card through here rather
/// than tokenizing SPICE a second time.
pub(crate) fn card_tokens(card: &str) -> Vec<String> {
    tokenize_card(card.trim_start())
        .into_iter()
        .map(|token| token.text)
        .collect()
}

/// Token text and one-based Unicode column for source-intelligence actions
/// that must identify the exact declaration/reference token without
/// retokenizing SPICE differently from the outline.
pub(crate) fn card_tokens_with_columns(card: &str) -> Vec<(String, usize)> {
    let trimmed = card.trim_start();
    let leading_columns = card.chars().take_while(|ch| ch.is_whitespace()).count();
    tokenize_card(trimmed)
        .into_iter()
        .map(|token| (token.text, token.column + leading_columns))
        .collect()
}

fn classify_card(tokens: &[CardToken], trimmed: &str) -> (OutlineEntryKind, String) {
    let head = tokens[0].text.to_ascii_lowercase();
    let detail = tokens.get(1).map_or("", |token| token.text.as_str());
    let directive_label = || {
        if detail.is_empty() {
            tokens[0].text.clone()
        } else {
            format!("{} {detail}", tokens[0].text)
        }
    };

    match head.as_str() {
        ".include" | ".inc" | ".veriloga" => (OutlineEntryKind::Include, directive_label()),
        ".lib" if tokens.len() >= 3 => (OutlineEntryKind::Library, directive_label()),
        ".param" => (OutlineEntryKind::Parameter, directive_label()),
        ".global" | ".global_param" => (OutlineEntryKind::Global, directive_label()),
        ".func" => (OutlineEntryKind::Function, directive_label()),
        ".option" | ".options" => (OutlineEntryKind::Option, directive_label()),
        ".model" => (OutlineEntryKind::Model, directive_label()),
        ".subckt" => (OutlineEntryKind::Subcircuit, directive_label()),
        ".ends" => (OutlineEntryKind::EndSubcircuit, directive_label()),
        ".op" | ".dc" | ".ac" | ".tran" | ".noise" | ".pz" | ".sens" | ".tf" | ".disto"
        | ".four" | ".hb" | ".sp" | ".pss" | ".pac" | ".pnoise" | ".qpss" | ".qpnoise" | ".stb" => {
            (OutlineEntryKind::Analysis, directive_label())
        }
        ".meas" | ".measure" => (OutlineEntryKind::Measurement, directive_label()),
        ".save" | ".probe" | ".print" | ".plot" => (OutlineEntryKind::Output, directive_label()),
        ".if" | ".elseif" | ".else" | ".endif" => {
            (OutlineEntryKind::Conditional, directive_label())
        }
        _ if head.starts_with('.') => (OutlineEntryKind::Control, directive_label()),
        _ => (
            OutlineEntryKind::Device,
            tokens
                .first()
                .map_or_else(|| trimmed.to_owned(), |token| token.text.clone()),
        ),
    }
}

fn is_comment(trimmed: &str) -> bool {
    trimmed.starts_with('*') || trimmed.starts_with(';') || trimmed.starts_with("//")
}

/// Tokenize a SPICE card without normalizing or modifying the source. Both
/// quote styles are accepted and the returned text excludes the quote marks.
fn tokenize_card(card: &str) -> Vec<CardToken> {
    let mut tokens = Vec::new();
    let mut chars = card.char_indices().peekable();

    while let Some((offset, ch)) = chars.next() {
        if ch.is_whitespace() {
            continue;
        }
        if ch == ';' || ch == '$' {
            break;
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
                if current.is_whitespace() || current == ';' || current == '$' {
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
}
