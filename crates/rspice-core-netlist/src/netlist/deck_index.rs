//! One index of a deck for the editor's language tools.
//!
//! The index is built from the text of a deck and of its include files as the
//! caller resolved them; it never reads a file or resolves a path. It reads
//! cards the way the engine does: a `+` line continues the card before it,
//! comments are where the engine's comment rule puts them, a statement
//! written in Spectre is read as Spectre, and a field the engine's lexer
//! reads as a number (`10n`, `1e-9`) is a number, never a name. Every name a card
//! defines or reads is recorded with the subcircuit scope it is written in,
//! and every read is resolved to the definition SPICE scoping binds it to.

use std::collections::{HashMap, HashSet};
use std::ops::{Range, RangeInclusive};

use super::lexer::{Lexer, TokenKind, parse_spice_value_complete};
use super::source_map::{
    MappedLine, for_each_logical_line, is_param_boundary, logical_values, token_spans,
};
use super::{ParameterRedefinitionPolicy, SourceKind, comment_ranges};

/// One file of the deck, as the caller resolved it. The first source given to
/// [`DeckIndex::build`] is the deck itself; the others are the files it
/// includes.
#[derive(Debug, Clone, Copy)]
pub struct DeckIndexSource<'a> {
    /// What the caller calls the file. A location names its file by the
    /// position of its source in the list; [`DeckIndex::file_identity`] turns
    /// that back into this.
    pub identity: &'a str,
    pub text: &'a str,
    /// How the engine reads the file: whether its first line is a title,
    /// and whether it is SPICE or Spectre.
    pub kind: SourceKind,
}

/// Where something is written.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeckLocation {
    /// Position of the file among the sources the index was built from.
    pub file: usize,
    /// Byte range in that file's text.
    pub span: Range<usize>,
    /// One-based physical line the span starts on.
    pub line: usize,
    /// One-based character column the span starts at.
    pub column: usize,
}

/// What a logical card is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeckCardKind {
    /// The deck's first line, which the engine reads as its title.
    Title,
    /// `.include`, `.inc` and the Verilog-A source cards.
    Include,
    /// `.lib file section`.
    Library,
    /// `.lib name`, opening a section of a library file.
    LibrarySection,
    /// `.endl`.
    EndLibrarySection,
    /// `.param` and its relatives.
    Parameter,
    /// `.global`.
    Global,
    /// `.func`.
    Function,
    /// `.option` and `.options`.
    Option,
    Model,
    Subcircuit,
    EndSubcircuit,
    Analysis,
    Measurement,
    /// `.save`, `.probe`, `.print`, `.plot`.
    Output,
    /// `.if`, `.elseif`, `.else`, `.endif`.
    Conditional,
    /// A `.control` block, through its `.endc`; its lines are commands, not
    /// cards.
    Control,
    /// Any other dot card.
    Directive,
    /// An element.
    Instance,
    /// `.end`; nothing after it is read.
    End,
}

/// One logical card: a physical line and the `+` lines that continue it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeckCard {
    pub kind: DeckCardKind,
    /// The card's first word as written: `.subckt`, `R1`. The whole line for
    /// the title.
    pub head: String,
    /// Where the head is written.
    pub location: DeckLocation,
    /// The first and last physical line, continuation lines included.
    pub lines: RangeInclusive<usize>,
    /// The scope the card is written in. A `.subckt` card is written in the
    /// scope that encloses the subcircuit; its `.ends` inside it.
    pub scope: usize,
    /// The library section the card sits in, `.lib name` to `.endl`.
    pub section: Option<String>,
}

/// What a token is, as the engine's lexer reads it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeckTokenKind {
    Name,
    Number,
    /// `{...}` or `'...'`.
    Expression,
    /// `"..."`.
    String,
    Equals,
    Punctuation,
    /// Text the lexer does not read.
    Invalid,
}

/// One token of a card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeckToken {
    pub kind: DeckTokenKind,
    /// Byte range in the card's file.
    pub span: Range<usize>,
}

/// The top level of the deck, or one subcircuit definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeckScope {
    /// The subcircuit's name; `None` for the top level.
    pub name: Option<String>,
    pub parent: Option<usize>,
    /// The subcircuit symbol that opens the scope.
    pub symbol: Option<usize>,
    /// The subcircuit's ports, in order.
    pub ports: Vec<String>,
}

/// What a name names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeckSymbolKind {
    Parameter,
    Function,
    Subcircuit,
    Model,
    Instance,
}

/// One definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeckSymbol {
    pub kind: DeckSymbolKind,
    /// The name as written.
    pub name: String,
    /// The scope the name is defined in. A subcircuit's parameters are
    /// defined in its own scope; the subcircuit in the one enclosing it.
    pub scope: usize,
    /// Where the name is written.
    pub location: DeckLocation,
    /// The card that defines it.
    pub card: usize,
    /// The library section it is written in.
    pub section: Option<String>,
    /// A model's type, a subcircuit's port count, a parameter's value, an
    /// instance's subcircuit or model, or the value a resistor, capacitor or
    /// inductor is written with.
    pub detail: String,
    /// The scope a subcircuit opens.
    pub opens: Option<usize>,
    /// The subcircuit or model an instance is of.
    pub target: Option<usize>,
}

/// One read of a name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeckReference {
    pub kind: DeckSymbolKind,
    /// The name as written.
    pub name: String,
    /// The scope the name is read in.
    pub scope: usize,
    pub location: DeckLocation,
    pub card: usize,
    /// The definition SPICE scoping binds the read to; `None` when nothing in
    /// scope defines the name.
    pub target: Option<usize>,
}

/// One place a node is written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeckNode {
    /// The name as written.
    pub name: String,
    /// The scope the node belongs to: the top level for ground and every
    /// `.global` node, else the scope the card is written in.
    pub scope: usize,
    pub location: DeckLocation,
    pub card: usize,
}

/// An include card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeckIncludeKind {
    Include,
    Library,
}

/// One `.include` or `.lib file section` card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeckInclude {
    pub kind: DeckIncludeKind,
    /// The path as written, quotes removed.
    pub path: String,
    /// The section a `.lib` card selects.
    pub section: Option<String>,
    /// Where the path is written.
    pub location: DeckLocation,
    pub card: usize,
}

/// A name under the caret: where it is defined, or a read of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeckNameAt {
    Definition(usize),
    Reference(usize),
}

/// The index of one deck and its include files.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DeckIndex {
    /// Per file, where the comment on each physical line starts.
    comments: Vec<Vec<Option<usize>>>,
    files: Vec<String>,
    cards: Vec<DeckCard>,
    scopes: Vec<DeckScope>,
    symbols: Vec<DeckSymbol>,
    references: Vec<DeckReference>,
    nodes: Vec<DeckNode>,
    includes: Vec<DeckInclude>,
}

impl DeckIndex {
    /// Index `sources`: the deck first, then the files it includes. Every
    /// file is read at the top level of the deck.
    #[must_use]
    pub fn build(sources: &[DeckIndexSource<'_>]) -> Self {
        let mut builder = Builder::new();
        for (file, source) in sources.iter().enumerate() {
            builder.read_file(file, source);
        }
        builder.finish()
    }

    #[must_use]
    pub fn file_identity(&self, file: usize) -> Option<&str> {
        self.files.get(file).map(String::as_str)
    }

    #[must_use]
    pub fn files(&self) -> &[String] {
        &self.files
    }

    /// Every card, file by file in the order the sources were given, each
    /// file's in source order.
    #[must_use]
    pub fn cards(&self) -> &[DeckCard] {
        &self.cards
    }

    #[must_use]
    pub fn scopes(&self) -> &[DeckScope] {
        &self.scopes
    }

    /// Every definition, in source order.
    #[must_use]
    pub fn symbols(&self) -> &[DeckSymbol] {
        &self.symbols
    }

    /// Every read, in source order.
    #[must_use]
    pub fn references(&self) -> &[DeckReference] {
        &self.references
    }

    /// Every place a node is written, in source order.
    #[must_use]
    pub fn nodes(&self) -> &[DeckNode] {
        &self.nodes
    }

    #[must_use]
    pub fn includes(&self) -> &[DeckInclude] {
        &self.includes
    }

    /// The reads `symbol` binds, in source order.
    pub fn references_to(&self, symbol: usize) -> impl Iterator<Item = &DeckReference> {
        self.references
            .iter()
            .filter(move |reference| reference.target == Some(symbol))
    }

    /// The card written on one-based physical `line` of `file`, continuation
    /// lines included.
    #[must_use]
    pub fn card_on_line(&self, file: usize, line: usize) -> Option<usize> {
        let after = self
            .cards
            .partition_point(|card| (card.location.file, *card.lines.start()) <= (file, line));
        let index = after.checked_sub(1)?;
        let card = &self.cards[index];
        (card.location.file == file && card.lines.contains(&line)).then_some(index)
    }

    /// The name written at `byte` of `file`.
    #[must_use]
    pub fn name_at(&self, file: usize, byte: usize) -> Option<DeckNameAt> {
        let covers = |location: &DeckLocation| {
            location.file == file && location.span.start <= byte && byte <= location.span.end
        };
        if let Some(index) = self
            .references
            .iter()
            .position(|reference| covers(&reference.location))
        {
            return Some(DeckNameAt::Reference(index));
        }
        self.symbols
            .iter()
            .position(|symbol| covers(&symbol.location))
            .map(DeckNameAt::Definition)
    }

    /// The definition of the name written at `byte` of `file`: the name's own
    /// when it is written there, else the one the read there binds.
    #[must_use]
    pub fn definition_at(&self, file: usize, byte: usize) -> Option<usize> {
        match self.name_at(file, byte)? {
            DeckNameAt::Definition(symbol) => Some(symbol),
            DeckNameAt::Reference(reference) => self.references[reference].target,
        }
    }

    /// The scope a reader at one-based `line` of `file` is in.
    #[must_use]
    pub fn scope_on_line(&self, file: usize, line: usize) -> usize {
        let Some(card) = self.card_on_line(file, line) else {
            return 0;
        };
        // A subcircuit's header reads its defaults inside the subcircuit.
        self.symbols
            .iter()
            .find(|symbol| symbol.card == card && symbol.kind == DeckSymbolKind::Subcircuit)
            .and_then(|symbol| symbol.opens)
            .unwrap_or(self.cards[card].scope)
    }

    /// `scope` and every scope enclosing it, innermost first.
    pub fn scope_chain(&self, scope: usize) -> impl Iterator<Item = usize> + '_ {
        std::iter::successors(Some(scope), |scope| {
            self.scopes.get(*scope).and_then(|scope| scope.parent)
        })
    }

    /// The parameters a reader in `scope` sees, one per name: each the
    /// definition the engine binds there.
    #[must_use]
    pub fn parameters_visible_in(&self, scope: usize) -> Vec<usize> {
        let mut seen = HashSet::new();
        let mut visible = Vec::new();
        for level in self.scope_chain(scope) {
            let names = self
                .symbols
                .iter()
                .filter(|symbol| symbol.kind == DeckSymbolKind::Parameter && symbol.scope == level)
                .map(|symbol| symbol.name.to_ascii_uppercase())
                .collect::<Vec<_>>();
            for name in names {
                if seen.insert(name.clone())
                    && let Some(symbol) =
                        self.resolve_in_scope(DeckSymbolKind::Parameter, &name, level)
                {
                    visible.push(symbol);
                }
            }
        }
        visible
    }

    /// The tokens of `card` as the engine's lexer reads them, from `text`, the
    /// text of the card's file the index was built from. A title or a
    /// `.control` block has none.
    #[must_use]
    pub fn card_tokens(&self, card: usize, text: &str) -> Vec<DeckToken> {
        let Some(card) = self.cards.get(card) else {
            return Vec::new();
        };
        if matches!(card.kind, DeckCardKind::Title | DeckCardKind::Control) {
            return Vec::new();
        }
        let Some(before) = text.get(..card.location.span.start) else {
            return Vec::new();
        };
        let start = before.rfind('\n').map_or(0, |newline| newline + 1);
        let mut end = start;
        for _ in card.lines.clone() {
            end = text[end..].find('\n').map_or(text.len(), |at| end + at + 1);
        }
        let comments = self
            .comments
            .get(card.location.file)
            .map_or(&[][..], Vec::as_slice);
        let first_line = card.lines.start() - 1;
        let mut tokens = Vec::new();
        for_each_logical_line(
            &text[start..end],
            0,
            |line, raw| Some(code_before_comment(comments, first_line + line, raw)),
            |line| {
                if tokens.is_empty() {
                    tokens = lex_card(&line)
                        .into_iter()
                        .map(|(kind, span)| DeckToken {
                            kind,
                            span: span.start + start..span.end + start,
                        })
                        .collect();
                }
            },
        );
        tokens
    }

    fn resolve_in_scope(&self, kind: DeckSymbolKind, name: &str, scope: usize) -> Option<usize> {
        let candidates = self
            .symbols
            .iter()
            .enumerate()
            .filter(|(_, symbol)| {
                symbol.kind == kind
                    && symbol.scope == scope
                    && symbol.name.eq_ignore_ascii_case(name)
            })
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        binding(kind, &candidates, &self.symbols, &HashSet::new())
    }
}

/// The definition among `candidates`, all of one name in one scope, that
/// binds it: for a parameter or function the one the engine's redefinition
/// policy selects, for a model or subcircuit the first in a library section a
/// `.lib` card selects (or in none), else the first.
fn binding(
    kind: DeckSymbolKind,
    candidates: &[usize],
    symbols: &[DeckSymbol],
    selected_sections: &HashSet<String>,
) -> Option<usize> {
    if candidates.is_empty() {
        return None;
    }
    match kind {
        DeckSymbolKind::Parameter | DeckSymbolKind::Function => candidates
            .get(ParameterRedefinitionPolicy::default().binding_definition(candidates.len()))
            .copied(),
        DeckSymbolKind::Model | DeckSymbolKind::Subcircuit => candidates
            .iter()
            .copied()
            .find(|candidate| {
                symbols[*candidate]
                    .section
                    .as_ref()
                    .is_none_or(|section| selected_sections.contains(&section.to_ascii_uppercase()))
            })
            .or_else(|| candidates.first().copied()),
        DeckSymbolKind::Instance => candidates.first().copied(),
    }
}

/// Where the comment on each physical line of `text` starts, by the
/// engine's comment rule for a source of `kind`: a root deck's title, a
/// Spectre block comment over several lines and a `simulator lang=` switch
/// included.
fn comment_starts(text: &str, line_starts: &[usize], kind: SourceKind) -> Vec<Option<usize>> {
    let mut starts = vec![None; line_starts.len()];
    for range in comment_ranges(text, kind) {
        let line = line_starts.partition_point(|start| *start <= range.start) - 1;
        let at = range.start - line_starts[line];
        let start: &mut Option<usize> = &mut starts[line];
        *start = Some(start.map_or(at, |earlier| earlier.min(at)));
    }
    starts
}

/// The part of physical line `line` before its comment.
fn code_before_comment<'a>(comments: &[Option<usize>], line: usize, raw: &'a str) -> &'a str {
    comments
        .get(line)
        .copied()
        .flatten()
        .and_then(|at| raw.get(..at))
        .unwrap_or(raw)
}

/// The tokens of one logical card, each with its buffer range.
fn lex_card(line: &MappedLine) -> Vec<(DeckTokenKind, Range<usize>)> {
    Lexer::new(&line.text)
        .tokenize_recovering()
        .into_iter()
        .filter_map(|token| {
            let (kind, span) = match token {
                Ok(token) => (
                    match token.kind {
                        TokenKind::Ident(_) if is_number(&token.lexeme) => DeckTokenKind::Number,
                        TokenKind::Ident(_) => DeckTokenKind::Name,
                        TokenKind::Number(_) => DeckTokenKind::Number,
                        TokenKind::Expression(_) => DeckTokenKind::Expression,
                        TokenKind::StringLit(_) => DeckTokenKind::String,
                        TokenKind::Equals => DeckTokenKind::Equals,
                        _ => DeckTokenKind::Punctuation,
                    },
                    token.span.start..token.span.end,
                ),
                Err((span, _)) => (DeckTokenKind::Invalid, span.start..span.end),
            };
            Some((kind, line.original_span(span)?))
        })
        .collect()
}

/// Whether the lexer reads `field` as one number.
/// The lexer keeps a digit-led word such as `10n` or `1N4148` a name where a
/// node or model may be written; a value is a number when it reads whole as
/// one, as the parser reads a value.
fn is_number(field: &str) -> bool {
    parse_spice_value_complete(field).is_ok()
}

/// What a read names, when the card does not say: a positional passive value
/// can name a model or a parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReadKind {
    Known(DeckSymbolKind),
    ModelOrParameter,
    /// A call that is a read only when it names a `.func`.
    Call,
    /// A Spectre instance's master: a subcircuit, else a model, else a
    /// primitive the simulator provides, which is no read.
    Master,
}

struct PendingRead {
    kind: ReadKind,
    name: String,
    scope: usize,
    location: DeckLocation,
    card: usize,
    /// The instance symbol this read names the subcircuit or model of.
    instance: Option<usize>,
}

/// A transistor card, whose node count the model it names decides.
struct PendingTransistor {
    card: usize,
    scope: usize,
    instance: usize,
    /// The positional fields after the name.
    positional: Vec<(String, DeckLocation, bool)>,
    /// How many of them are nodes at least.
    min_nodes: usize,
}

struct FileState<'a> {
    file: usize,
    text: &'a str,
    line_starts: Vec<usize>,
    scopes: Vec<usize>,
    section: Option<String>,
    /// The `.control` card a block being read belongs to.
    control: Option<usize>,
    ended: bool,
    /// Whether the lines being read are Spectre: a `.scs` file, or after a
    /// `simulator lang=spectre` statement.
    spectre: bool,
    /// How deep in a Spectre `{ }` block (`statistics`) the lines are.
    braces: usize,
}

impl FileState<'_> {
    fn location(&self, span: Range<usize>) -> DeckLocation {
        let line = self
            .line_starts
            .partition_point(|start| *start <= span.start);
        let line_start = self.line_starts[line - 1];
        let column = self.text[line_start..span.start].chars().count() + 1;
        DeckLocation {
            file: self.file,
            span,
            line,
            column,
        }
    }

    fn scope(&self) -> usize {
        self.scopes.last().copied().unwrap_or(0)
    }

    /// A definition written in this file, in the library section being read.
    fn symbol(
        &self,
        kind: DeckSymbolKind,
        name: &str,
        scope: usize,
        location: DeckLocation,
        card: usize,
        detail: String,
    ) -> DeckSymbol {
        DeckSymbol {
            kind,
            name: name.to_owned(),
            scope,
            location,
            card,
            section: self.section.clone(),
            detail,
            opens: None,
            target: None,
        }
    }
}

struct Builder {
    index: DeckIndex,
    reads: Vec<PendingRead>,
    transistors: Vec<PendingTransistor>,
    globals: HashSet<String>,
}

impl Builder {
    fn new() -> Self {
        Self {
            index: DeckIndex {
                scopes: vec![DeckScope {
                    name: None,
                    parent: None,
                    symbol: None,
                    ports: Vec::new(),
                }],
                ..DeckIndex::default()
            },
            reads: Vec::new(),
            transistors: Vec::new(),
            globals: ["0", "GND"].into_iter().map(str::to_owned).collect(),
        }
    }

    fn read_file(&mut self, file: usize, source: &DeckIndexSource<'_>) {
        self.index.files.push(source.identity.to_owned());
        let text = source.text;
        let mut state = FileState {
            file,
            text,
            line_starts: std::iter::once(0)
                .chain(text.match_indices('\n').map(|(at, _)| at + 1))
                .collect(),
            scopes: Vec::new(),
            section: None,
            control: None,
            ended: false,
            spectre: source.kind == SourceKind::Spectre,
            braces: 0,
        };
        let comments = comment_starts(text, &state.line_starts, source.kind);
        let title_lines = usize::from(source.kind == SourceKind::DeckRoot && !text.is_empty());
        if title_lines == 1 {
            let first = text.split('\n').next().unwrap_or_default();
            let first = first.strip_suffix('\r').unwrap_or(first);
            self.index.cards.push(DeckCard {
                kind: DeckCardKind::Title,
                head: first.to_owned(),
                location: state.location(0..first.len()),
                lines: 1..=1,
                scope: 0,
                section: None,
            });
        }
        for_each_logical_line(
            text,
            title_lines,
            |line, raw| Some(code_before_comment(&comments, line, raw)),
            |line| self.read_card(&mut state, &line),
        );
        self.index.comments.push(comments);
    }

    fn read_card(&mut self, state: &mut FileState<'_>, line: &MappedLine) {
        if state.ended {
            return;
        }
        let (first_line, last_line) = line.physical_lines();
        let fields = fields_of(&line.text);
        let Some(head_range) = fields.first().cloned() else {
            return;
        };
        let head = &line.text[head_range.clone()];
        let head_lower = head.to_ascii_lowercase();

        if let Some(control) = state.control {
            let card = &mut self.index.cards[control];
            card.lines = *card.lines.start()..=last_line;
            if head_lower == ".endc" {
                state.control = None;
            }
            return;
        }

        let kind = if head_lower == "simulator" {
            // `simulator lang=spectre` hands the lines after it to Spectre,
            // `simulator lang=spice` hands them back.
            for field in &fields[1..] {
                match line.text[field.clone()].to_ascii_lowercase().as_str() {
                    "lang=spectre" => state.spectre = true,
                    "lang=spice" => state.spectre = false,
                    _ => {}
                }
            }
            DeckCardKind::Directive
        } else if state.spectre {
            let inside_block = state.braces > 0;
            let opened = line.text.matches('{').count();
            let closed = line.text.matches('}').count();
            state.braces = (state.braces + opened).saturating_sub(closed);
            if inside_block || opened > 0 {
                DeckCardKind::Directive
            } else {
                spectre_card_kind(&head_lower, &fields, &line.text)
            }
        } else {
            card_kind(&head_lower, &fields, &line.text)
        };
        let card_index = self.index.cards.len();
        let Some(head_span) = line.original_span(head_range) else {
            return;
        };
        let location = state.location(head_span);
        let scope = state.scope();
        self.index.cards.push(DeckCard {
            kind,
            head: head.to_owned(),
            location,
            lines: first_line..=last_line,
            scope,
            section: state.section.clone(),
        });
        let field_text = |index: usize| fields.get(index).map(|range| &line.text[range.clone()]);
        let field_location = |state: &FileState<'_>, index: usize| {
            fields
                .get(index)
                .and_then(|range| line.original_span(range.clone()))
                .map(|span| state.location(span))
        };

        match kind {
            DeckCardKind::Control => state.control = Some(card_index),
            DeckCardKind::End => state.ended = true,
            DeckCardKind::LibrarySection => {
                state.section = field_text(1).map(unquote).map(str::to_owned);
            }
            DeckCardKind::EndLibrarySection => state.section = None,
            DeckCardKind::Include | DeckCardKind::Library => {
                let arguments = quoted_arguments(&line.text, &fields[1..]);
                if let Some(path) = arguments.first().cloned()
                    && let Some(span) = line.original_span(path.clone())
                {
                    self.index.includes.push(DeckInclude {
                        kind: if kind == DeckCardKind::Library {
                            DeckIncludeKind::Library
                        } else {
                            DeckIncludeKind::Include
                        },
                        path: unquote(&line.text[path]).to_owned(),
                        section: arguments
                            .get(1)
                            .map(|section| unquote(&line.text[section.clone()]).to_owned()),
                        location: state.location(span),
                        card: card_index,
                    });
                }
            }
            DeckCardKind::Global => {
                for index in 1..fields.len() {
                    if let (Some(name), Some(location)) =
                        (field_text(index), field_location(state, index))
                    {
                        self.globals.insert(name.to_ascii_uppercase());
                        self.push_node(name, 0, location, card_index);
                    }
                }
                if head_lower == ".global_param" {
                    self.read_parameters(state, line, card_index, 0, true);
                }
            }
            DeckCardKind::Parameter => {
                self.read_parameters(state, line, card_index, scope, true);
            }
            DeckCardKind::Function => self.read_function(state, line, card_index, scope),
            DeckCardKind::Model => {
                if let (Some(name), Some(location)) = (field_text(1), field_location(state, 1)) {
                    let detail = field_text(2)
                        .map(|kind| kind.split('(').next().unwrap_or(kind).to_owned())
                        .unwrap_or_default();
                    let symbol = state.symbol(
                        DeckSymbolKind::Model,
                        name,
                        scope,
                        location,
                        card_index,
                        detail,
                    );
                    self.push_symbol(symbol);
                }
                self.read_parameters(state, line, card_index, scope, false);
            }
            // Spectre's `inline subckt name` names the subcircuit a field later.
            DeckCardKind::Subcircuit if head_lower == "inline" => {
                self.read_subcircuit(state, line, &fields[1..], card_index);
            }
            DeckCardKind::Subcircuit => self.read_subcircuit(state, line, &fields, card_index),
            DeckCardKind::EndSubcircuit => {
                if let Some(inner) = state.scopes.pop() {
                    if let (Some(name), Some(location)) = (field_text(1), field_location(state, 1))
                    {
                        self.reads.push(PendingRead {
                            kind: ReadKind::Known(DeckSymbolKind::Subcircuit),
                            name: name.to_owned(),
                            scope: self.index.scopes[inner].parent.unwrap_or(0),
                            location,
                            card: card_index,
                            instance: None,
                        });
                    }
                    self.index.cards[card_index].scope = inner;
                }
            }
            DeckCardKind::Instance if state.spectre => {
                self.read_spectre_instance(state, line, &fields, card_index, scope);
            }
            DeckCardKind::Instance => self.read_instance(state, line, &fields, card_index, scope),
            _ => self.read_parameters(state, line, card_index, scope, false),
        }
    }

    fn push_symbol(&mut self, symbol: DeckSymbol) -> usize {
        self.index.symbols.push(symbol);
        self.index.symbols.len() - 1
    }

    fn push_node(&mut self, name: &str, scope: usize, location: DeckLocation, card: usize) {
        self.index.nodes.push(DeckNode {
            name: name.to_owned(),
            scope,
            location,
            card,
        });
    }

    fn push_read(
        &mut self,
        kind: ReadKind,
        name: &str,
        scope: usize,
        location: DeckLocation,
        card: usize,
    ) {
        self.reads.push(PendingRead {
            kind,
            name: name.to_owned(),
            scope,
            location,
            card,
            instance: None,
        });
    }

    /// Every `name=value` on the card and every braced or quoted value: a
    /// name a parameter card assigns is defined in `scope`, and every name
    /// a value reads is a read in `scope`.
    fn read_parameters(
        &mut self,
        state: &FileState<'_>,
        line: &MappedLine,
        card: usize,
        scope: usize,
        defines: bool,
    ) {
        for value in logical_values(&line.text) {
            if defines
                && let Some(name) = value.name.clone()
                && let Some(span) = line.original_span(name.clone())
            {
                let location = state.location(span);
                let detail = line.text[value.token.clone()].to_owned();
                let symbol = state.symbol(
                    DeckSymbolKind::Parameter,
                    &line.text[name],
                    scope,
                    location,
                    card,
                    detail,
                );
                self.push_symbol(symbol);
            }
            self.read_value(state, line, value.body, card, scope, &[]);
        }
    }

    /// The names an expression reads: a name the lexer reads as a number is
    /// none, a name called like a function is a read of a `.func` when one
    /// is defined, and the arguments of a `V()` or `I()` probe are nodes and
    /// devices, not parameters. `formals` are names the expression binds
    /// itself.
    fn read_value(
        &mut self,
        state: &FileState<'_>,
        line: &MappedLine,
        body: Range<usize>,
        card: usize,
        scope: usize,
        formals: &[String],
    ) {
        let text = &line.text[body.clone()];
        let tokens = Lexer::new(text)
            .tokenize_recovering()
            .into_iter()
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        let mut index = 0;
        while index < tokens.len() {
            let token = &tokens[index];
            index += 1;
            if let TokenKind::Expression(_) = token.kind {
                // A nested group: read its inside.
                let inner = body.start + token.span.start + 1..body.start + token.span.end - 1;
                if inner.start < inner.end {
                    self.read_value(state, line, inner, card, scope, formals);
                }
                continue;
            }
            let TokenKind::Ident(_) = token.kind else {
                continue;
            };
            let name = token.lexeme.as_str();
            if name.starts_with(|first: char| first.is_ascii_digit()) {
                continue;
            }
            let called = tokens
                .get(index)
                .is_some_and(|next| matches!(next.kind, TokenKind::LParen));
            if called && (name.eq_ignore_ascii_case("v") || name.eq_ignore_ascii_case("i")) {
                let mut depth = 0usize;
                while index < tokens.len() {
                    match tokens[index].kind {
                        TokenKind::LParen => depth += 1,
                        TokenKind::RParen => {
                            depth = depth.saturating_sub(1);
                            if depth == 0 {
                                index += 1;
                                break;
                            }
                        }
                        _ => {}
                    }
                    index += 1;
                }
                continue;
            }
            if formals
                .iter()
                .any(|formal| formal.eq_ignore_ascii_case(name))
            {
                continue;
            }
            let span = body.start + token.span.start..body.start + token.span.end;
            let Some(span) = line.original_span(span) else {
                continue;
            };
            let kind = if called {
                ReadKind::Call
            } else {
                ReadKind::Known(DeckSymbolKind::Parameter)
            };
            self.push_read(kind, name, scope, state.location(span), card);
        }
    }

    fn read_function(
        &mut self,
        state: &FileState<'_>,
        line: &MappedLine,
        card: usize,
        scope: usize,
    ) {
        let text = &line.text;
        let Some(head_end) = text.find(char::is_whitespace) else {
            return;
        };
        let Some(open) = text[head_end..].find('(').map(|at| head_end + at) else {
            return;
        };
        let name_range = {
            let raw = &text[head_end..open];
            let start = head_end + (raw.len() - raw.trim_start().len());
            start..head_end + raw.trim_end().len()
        };
        if name_range.is_empty() {
            return;
        }
        let Some(close) = text[open..].find(')').map(|at| open + at) else {
            return;
        };
        let formals = text[open + 1..close]
            .split([',', ' '])
            .map(str::trim)
            .filter(|formal| !formal.is_empty())
            .map(str::to_owned)
            .collect::<Vec<_>>();
        if let Some(span) = line.original_span(name_range.clone()) {
            let location = state.location(span);
            let detail = format!("({})", formals.join(", "));
            let symbol = state.symbol(
                DeckSymbolKind::Function,
                &text[name_range],
                scope,
                location,
                card,
                detail,
            );
            self.push_symbol(symbol);
        }
        for value in logical_values(text)
            .into_iter()
            .filter(|value| value.body.start > close)
        {
            self.read_value(state, line, value.body, card, scope, &formals);
        }
    }

    fn read_subcircuit(
        &mut self,
        state: &mut FileState<'_>,
        line: &MappedLine,
        fields: &[Range<usize>],
        card: usize,
    ) {
        let parent = state.scope();
        let Some(name_range) = fields.get(1).cloned() else {
            return;
        };
        let Some(span) = line.original_span(name_range.clone()) else {
            return;
        };
        let name = line.text[name_range].to_owned();
        // Spectre writes the ports in parentheses: `subckt amp (in out)`.
        let ports = fields
            .iter()
            .skip(2)
            .take_while(|field| !is_param_boundary(&line.text[(*field).clone()]))
            .filter_map(|field| without_parentheses(&line.text, field.clone()))
            .collect::<Vec<_>>();
        let location = state.location(span);
        let symbol = self.push_symbol(state.symbol(
            DeckSymbolKind::Subcircuit,
            &name,
            parent,
            location,
            card,
            if ports.len() == 1 {
                "1 port".to_owned()
            } else {
                format!("{} ports", ports.len())
            },
        ));
        let scope = self.index.scopes.len();
        self.index.scopes.push(DeckScope {
            name: Some(name),
            parent: Some(parent),
            symbol: Some(symbol),
            ports: ports
                .iter()
                .map(|port| line.text[port.clone()].to_owned())
                .collect(),
        });
        self.index.symbols[symbol].opens = Some(scope);
        state.scopes.push(scope);
        for port in ports {
            if let Some(span) = line.original_span(port.clone()) {
                let location = state.location(span);
                self.push_node(&line.text[port], scope, location, card);
            }
        }
        // A formal and its default belong to the subcircuit the card opens.
        self.read_parameters(state, line, card, scope, true);
    }

    /// A Spectre instance, `name (nodes) master name=value ...`: its nodes,
    /// and its master read as a subcircuit or model when the deck defines
    /// one of that name. The parentheses may be left out, and the master is
    /// then the last field before the first assignment.
    fn read_spectre_instance(
        &mut self,
        state: &FileState<'_>,
        line: &MappedLine,
        fields: &[Range<usize>],
        card: usize,
        scope: usize,
    ) {
        let text = &line.text;
        let instance = self.push_symbol(state.symbol(
            DeckSymbolKind::Instance,
            &text[fields[0].clone()],
            scope,
            self.index.cards[card].location.clone(),
            card,
            String::new(),
        ));
        let positional = fields[1..]
            .iter()
            .take_while(|field| !text[(*field).clone()].contains('='))
            .cloned()
            .collect::<Vec<_>>();
        let master = if text[positional.first().cloned().unwrap_or_default()].starts_with('(') {
            positional
                .iter()
                .position(|field| text[field.clone()].ends_with(')'))
                .map(|close| close + 1)
        } else {
            positional.len().checked_sub(1)
        };
        let node_count = master.unwrap_or(positional.len()).min(positional.len());
        for field in &positional[..node_count] {
            if let Some(node) = without_parentheses(text, field.clone())
                && let Some(span) = line.original_span(node.clone())
            {
                self.push_node(&text[node], scope, state.location(span), card);
            }
        }
        if let Some(field) = master.and_then(|at| positional.get(at))
            && let Some(span) = line.original_span(field.clone())
        {
            let written = text[field.clone()].to_owned();
            self.index.symbols[instance].detail.clone_from(&written);
            self.reads.push(PendingRead {
                kind: ReadKind::Master,
                name: written,
                scope,
                location: state.location(span),
                card,
                instance: Some(instance),
            });
        }
        self.read_parameters(state, line, card, scope, false);
    }

    fn read_instance(
        &mut self,
        state: &FileState<'_>,
        line: &MappedLine,
        fields: &[Range<usize>],
        card: usize,
        scope: usize,
    ) {
        let text = &line.text;
        let head = &text[fields[0].clone()];
        let instance = self.push_symbol(state.symbol(
            DeckSymbolKind::Instance,
            head,
            scope,
            self.index.cards[card].location.clone(),
            card,
            String::new(),
        ));
        let positional = fields
            .iter()
            .skip(1)
            .take_while(|field| {
                let field = &text[(*field).clone()];
                !is_param_boundary(field)
            })
            .filter_map(|field| {
                let span = line.original_span(field.clone())?;
                let written = &text[field.clone()];
                Some((written.to_owned(), state.location(span), is_number(written)))
            })
            .collect::<Vec<_>>();
        let letter = head
            .chars()
            .next()
            .map_or(' ', |letter| letter.to_ascii_uppercase());
        let mut nodes = 0usize;
        let mut target = None::<(ReadKind, usize)>;
        let mut instances = Vec::new();
        let name_at = |at: usize| positional.get(at).filter(|(_, _, number)| !number);
        match letter {
            'X' | 'N' => {
                if let Some(last) = positional.iter().rposition(|(_, _, number)| !number) {
                    nodes = last;
                    target = Some((
                        ReadKind::Known(if letter == 'X' {
                            DeckSymbolKind::Subcircuit
                        } else {
                            DeckSymbolKind::Model
                        }),
                        last,
                    ));
                } else {
                    nodes = positional.len();
                }
            }
            'Q' | 'M' => {
                self.transistors.push(PendingTransistor {
                    card,
                    scope,
                    instance,
                    positional: positional.clone(),
                    min_nodes: if letter == 'Q' { 3 } else { 4 },
                });
            }
            'D' => {
                nodes = 2;
                target = name_at(2).map(|_| (ReadKind::Known(DeckSymbolKind::Model), 2));
            }
            'J' | 'Z' => {
                nodes = 3;
                target = name_at(3).map(|_| (ReadKind::Known(DeckSymbolKind::Model), 3));
            }
            'R' | 'C' | 'L' => {
                nodes = 2;
                if name_at(3).is_some_and(|(written, _, _)| !written.starts_with(['{', '\''])) {
                    target = Some((ReadKind::Known(DeckSymbolKind::Model), 3));
                } else if name_at(2).is_some_and(|(written, _, _)| {
                    !written.starts_with(['{', '\'']) && !written.contains(['(', '{'])
                }) {
                    target = Some((ReadKind::ModelOrParameter, 2));
                } else if let Some((written, _, _)) = positional.get(2) {
                    self.index.symbols[instance].detail.clone_from(written);
                }
            }
            'V' | 'I' | 'B' => nodes = 2,
            'E' | 'G' => {
                nodes = if positional.len() >= 5 && !is_controlled_source_keyword(&positional[2].0)
                {
                    4
                } else {
                    2
                };
            }
            'F' | 'H' => {
                nodes = 2;
                if name_at(2).is_some_and(|(written, _, _)| !is_controlled_source_keyword(written))
                {
                    instances.push(2);
                }
            }
            'K' => {
                instances.extend((0..positional.len()).filter(|at| name_at(*at).is_some()));
            }
            'S' => {
                nodes = 4;
                target = name_at(4).map(|_| (ReadKind::Known(DeckSymbolKind::Model), 4));
            }
            'W' => {
                nodes = 2;
                if name_at(2).is_some() {
                    instances.push(2);
                }
                target = name_at(3).map(|_| (ReadKind::Known(DeckSymbolKind::Model), 3));
            }
            'T' => nodes = 4,
            'O' => {
                nodes = 4;
                target = name_at(4).map(|_| (ReadKind::Known(DeckSymbolKind::Model), 4));
            }
            'U' => {
                nodes = 3;
                target = name_at(3).map(|_| (ReadKind::Known(DeckSymbolKind::Model), 3));
            }
            'A' => {
                let is_port = |written: &str| {
                    !matches!(written, "[" | "]")
                        && !written.eq_ignore_ascii_case("null")
                        && !written.starts_with('%')
                };
                if let Some(model) = positional
                    .iter()
                    .rposition(|(written, _, _)| is_port(written))
                {
                    target = Some((ReadKind::Known(DeckSymbolKind::Model), model));
                    for (written, location, _) in &positional[..model] {
                        let node = written.trim_matches(['[', ']', '~']);
                        if !node.is_empty() && is_port(written) {
                            self.push_node(node, scope, location.clone(), card);
                        }
                    }
                }
            }
            _ => {}
        }
        for (written, location, _) in positional.iter().take(nodes.min(positional.len())) {
            self.push_node(written, scope, location.clone(), card);
        }
        for at in instances {
            let (written, location, _) = &positional[at];
            self.push_read(
                ReadKind::Known(DeckSymbolKind::Instance),
                written,
                scope,
                location.clone(),
                card,
            );
        }
        if let Some((kind, at)) = target {
            let (written, location, _) = &positional[at];
            self.index.symbols[instance].detail = written.clone();
            self.reads.push(PendingRead {
                kind,
                name: written.clone(),
                scope,
                location: location.clone(),
                card,
                instance: Some(instance),
            });
        }
        // Values: `w={w0}`, `{r0*2}`, `sin(0 {amp} 1k)`.
        self.read_parameters(state, line, card, scope, false);
    }

    fn finish(mut self) -> DeckIndex {
        let selected_sections = self
            .index
            .includes
            .iter()
            .filter_map(|include| include.section.as_ref())
            .map(|section| section.to_ascii_uppercase())
            .collect::<HashSet<_>>();
        let mut by_name = HashMap::<(DeckSymbolKind, usize, String), Vec<usize>>::new();
        for (index, symbol) in self.index.symbols.iter().enumerate() {
            by_name
                .entry((symbol.kind, symbol.scope, symbol.name.to_ascii_uppercase()))
                .or_default()
                .push(index);
        }
        let index = &self.index;
        let resolve = |kind: DeckSymbolKind, name: &str, scope: usize| {
            let key = name.to_ascii_uppercase();
            index.scope_chain(scope).find_map(|level| {
                let candidates = by_name.get(&(kind, level, key.clone()))?;
                binding(kind, candidates, &index.symbols, &selected_sections)
            })
        };

        let mut references = Vec::with_capacity(self.reads.len());
        let mut instance_targets = Vec::new();
        for read in &self.reads {
            let (kind, target) = match read.kind {
                ReadKind::Known(kind) => (kind, resolve(kind, &read.name, read.scope)),
                ReadKind::ModelOrParameter => {
                    match resolve(DeckSymbolKind::Model, &read.name, read.scope) {
                        Some(model) => (DeckSymbolKind::Model, Some(model)),
                        None => (
                            DeckSymbolKind::Parameter,
                            resolve(DeckSymbolKind::Parameter, &read.name, read.scope),
                        ),
                    }
                }
                ReadKind::Call => match resolve(DeckSymbolKind::Function, &read.name, read.scope) {
                    Some(function) => (DeckSymbolKind::Function, Some(function)),
                    None => continue,
                },
                ReadKind::Master => {
                    match resolve(DeckSymbolKind::Subcircuit, &read.name, read.scope) {
                        Some(subcircuit) => (DeckSymbolKind::Subcircuit, Some(subcircuit)),
                        None => match resolve(DeckSymbolKind::Model, &read.name, read.scope) {
                            Some(model) => (DeckSymbolKind::Model, Some(model)),
                            None => continue,
                        },
                    }
                }
            };
            if let (Some(instance), Some(target)) = (read.instance, target)
                && kind != DeckSymbolKind::Parameter
            {
                instance_targets.push((instance, target));
            }
            references.push(DeckReference {
                kind,
                name: read.name.clone(),
                scope: read.scope,
                location: read.location.clone(),
                card: read.card,
                target,
            });
        }
        let mut nodes = Vec::new();
        let mut details = Vec::new();
        for transistor in &self.transistors {
            let candidates = (transistor.min_nodes..transistor.positional.len())
                .filter(|at| !transistor.positional[*at].2)
                .collect::<Vec<_>>();
            let chosen = candidates
                .iter()
                .copied()
                .find(|at| {
                    resolve(
                        DeckSymbolKind::Model,
                        &transistor.positional[*at].0,
                        transistor.scope,
                    )
                    .is_some()
                })
                .or_else(|| candidates.last().copied());
            let node_count = chosen.unwrap_or(transistor.positional.len());
            for (written, location, _) in &transistor.positional[..node_count] {
                nodes.push(DeckNode {
                    name: written.clone(),
                    scope: transistor.scope,
                    location: location.clone(),
                    card: transistor.card,
                });
            }
            if let Some(at) = chosen {
                let (written, location, _) = &transistor.positional[at];
                let target = resolve(DeckSymbolKind::Model, written, transistor.scope);
                if let Some(target) = target {
                    instance_targets.push((transistor.instance, target));
                }
                details.push((transistor.instance, written.clone()));
                references.push(DeckReference {
                    kind: DeckSymbolKind::Model,
                    name: written.clone(),
                    scope: transistor.scope,
                    location: location.clone(),
                    card: transistor.card,
                    target,
                });
            }
        }
        for (instance, target) in instance_targets {
            self.index.symbols[instance].target = Some(target);
        }
        for (instance, detail) in details {
            self.index.symbols[instance].detail = detail;
        }
        self.index.nodes.extend(nodes);
        for node in &mut self.index.nodes {
            if self.globals.contains(&node.name.to_ascii_uppercase()) {
                node.scope = 0;
            }
        }
        let order = |location: &DeckLocation| (location.file, location.span.start);
        references.sort_by_key(|reference| order(&reference.location));
        self.index.nodes.sort_by_key(|node| order(&node.location));
        self.index.references = references;
        self.index
    }
}

/// The whitespace- and comma-separated fields of a logical card, with
/// `name = value` written apart joined back into the one field the parser
/// reads.
fn fields_of(text: &str) -> Vec<Range<usize>> {
    let mut fields: Vec<Range<usize>> = Vec::new();
    for span in token_spans(text) {
        let field = &text[span.clone()];
        // `name = value` is one field, and so is an expression written with
        // blanks in it (`{1 * k}`), as the engine reads them.
        let joins = fields.last().is_some_and(|previous| {
            let previous = &text[previous.clone()];
            previous.ends_with('=') || expression_left_open(previous)
        }) || field.starts_with('=');
        match fields.last_mut() {
            Some(previous) if joins => previous.end = span.end,
            _ => fields.push(span),
        }
    }
    fields
}

/// Whether `field` opens a `{...}` or `'...'` expression it does not close.
fn expression_left_open(field: &str) -> bool {
    let mut depth = 0i32;
    let mut quoted = false;
    for character in field.chars() {
        match character {
            '\'' => quoted = !quoted,
            '{' if !quoted => depth += 1,
            '}' if !quoted => depth -= 1,
            _ => {}
        }
    }
    depth > 0 || quoted
}

/// The arguments `fields` spell, a quoted one that holds blanks taken whole.
fn quoted_arguments(text: &str, fields: &[Range<usize>]) -> Vec<Range<usize>> {
    let mut arguments = Vec::new();
    let mut index = 0;
    while index < fields.len() {
        let field = fields[index].clone();
        let written = &text[field.clone()];
        let quote = written
            .chars()
            .next()
            .filter(|first| matches!(first, '"' | '\''));
        match quote {
            Some(quote) if !(written.len() > 1 && written.ends_with(quote)) => {
                let close = text[field.start + 1..]
                    .find(quote)
                    .map_or(text.len(), |at| field.start + 2 + at);
                arguments.push(field.start..close);
                index = fields.partition_point(|field| field.start < close);
            }
            _ => {
                arguments.push(field);
                index += 1;
            }
        }
    }
    arguments
}

fn unquote(field: &str) -> &str {
    field.trim_matches(['"', '\''])
}

/// Whether a controlled source's third field opens a behavioural or
/// polynomial form rather than naming a controlling node or source.
fn is_controlled_source_keyword(field: &str) -> bool {
    let lower = field.to_ascii_lowercase();
    [
        "poly", "table", "laplace", "value", "vol", "cur", "freq", "and", "or", "nand", "nor",
    ]
    .iter()
    .any(|keyword| lower == *keyword || lower.starts_with(&format!("{keyword}(")))
        || field.contains(['=', '{', '\''])
}

fn card_kind(head: &str, fields: &[Range<usize>], text: &str) -> DeckCardKind {
    match head {
        ".include" | ".inc" | ".veriloga" | ".hdl" | ".va" | ".vams" | ".ahdl_include" => {
            DeckCardKind::Include
        }
        ".lib" => match fields.len() {
            0..=1 => DeckCardKind::Directive,
            2 if !names_a_file(unquote(&text[fields[1].clone()])) => DeckCardKind::LibrarySection,
            _ => DeckCardKind::Library,
        },
        ".endl" => DeckCardKind::EndLibrarySection,
        ".param" | ".csparam" | ".params" => DeckCardKind::Parameter,
        ".global" | ".global_param" => DeckCardKind::Global,
        ".func" => DeckCardKind::Function,
        ".option" | ".options" | ".opt" => DeckCardKind::Option,
        ".model" => DeckCardKind::Model,
        ".subckt" => DeckCardKind::Subcircuit,
        ".ends" => DeckCardKind::EndSubcircuit,
        ".op" | ".dc" | ".ac" | ".tran" | ".noise" | ".pz" | ".sens" | ".tf" | ".disto"
        | ".four" | ".fourier" | ".hb" | ".sp" | ".pss" | ".pac" | ".pnoise" | ".qpss"
        | ".qpnoise" | ".stb" | ".step" => DeckCardKind::Analysis,
        ".meas" | ".measure" => DeckCardKind::Measurement,
        ".save" | ".probe" | ".print" | ".plot" => DeckCardKind::Output,
        ".if" | ".elseif" | ".else" | ".endif" => DeckCardKind::Conditional,
        ".control" => DeckCardKind::Control,
        ".end" => DeckCardKind::End,
        _ if head.starts_with('.') => DeckCardKind::Directive,
        _ if head.starts_with(|letter: char| letter.is_alphabetic()) => DeckCardKind::Instance,
        _ => DeckCardKind::Directive,
    }
}

/// The analyses a named Spectre statement can run: `tran1 tran stop=1u`.
const SPECTRE_ANALYSES: &[&str] = &[
    "ac",
    "acmatch",
    "dc",
    "dcmatch",
    "envlp",
    "hb",
    "hbac",
    "hbnoise",
    "hbsp",
    "hbxf",
    "montecarlo",
    "noise",
    "pac",
    "pdisto",
    "pnoise",
    "psp",
    "pss",
    "pstb",
    "pxf",
    "pz",
    "qpac",
    "qpnoise",
    "qpss",
    "qpsp",
    "qpxf",
    "sens",
    "sp",
    "stb",
    "sweep",
    "tdr",
    "tran",
    "xf",
];

/// What a statement written in Spectre is: by the keyword that opens it, or
/// for a named statement by what follows its name (an analysis, the
/// options, `info`), and an instance otherwise.
fn spectre_card_kind(head: &str, fields: &[Range<usize>], text: &str) -> DeckCardKind {
    let second = fields
        .get(1)
        .map(|field| text[field.clone()].to_ascii_lowercase())
        .unwrap_or_default();
    match head {
        "include" | "ahdl_include" => DeckCardKind::Include,
        "subckt" => DeckCardKind::Subcircuit,
        "inline" if second == "subckt" => DeckCardKind::Subcircuit,
        "ends" => DeckCardKind::EndSubcircuit,
        "model" => DeckCardKind::Model,
        "parameters" => DeckCardKind::Parameter,
        "section" => DeckCardKind::LibrarySection,
        "endsection" => DeckCardKind::EndLibrarySection,
        "global" => DeckCardKind::Global,
        "save" => DeckCardKind::Output,
        _ if !head.starts_with(|letter: char| letter.is_alphabetic() || letter == '_') => {
            DeckCardKind::Directive
        }
        _ => match second.as_str() {
            "options" | "set" => DeckCardKind::Option,
            "info" => DeckCardKind::Output,
            analysis if SPECTRE_ANALYSES.contains(&analysis) => DeckCardKind::Analysis,
            _ if fields.len() > 2 => DeckCardKind::Instance,
            _ => DeckCardKind::Directive,
        },
    }
}

/// `field` without the parentheses Spectre writes a node list in, or `None`
/// when nothing is left.
fn without_parentheses(text: &str, field: Range<usize>) -> Option<Range<usize>> {
    let written = &text[field.clone()];
    let start = field.start + (written.len() - written.trim_start_matches('(').len());
    let end = field.end - (written.len() - written.trim_end_matches(')').len());
    (start < end).then_some(start..end)
}

/// Whether the one argument of a `.lib` card names a file rather than the
/// section it opens.
fn names_a_file(argument: &str) -> bool {
    argument.contains(['/', '\\'])
        || std::path::Path::new(argument)
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| {
                super::MODEL_SOURCE_EXTENSIONS
                    .iter()
                    .any(|known| extension.eq_ignore_ascii_case(known))
            })
}

#[cfg(test)]
mod tests;
