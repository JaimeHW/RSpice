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
//!
//! The index holds no copy of the text. It shares the text it was built from,
//! and a name is a span of it. A file is read in pages of whole cards, each a
//! few flat tables of small records placed from the page's first byte, so a
//! deck of a hundred megabytes is a few thousand pages of a few allocations
//! each, not millions of small ones. An edit inside one page reads that page
//! again and shares every other with the index before it
//! ([`DeckIndex::edited`]).

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::mem::MaybeUninit;
use std::ops::{Range, RangeInclusive};
use std::sync::Arc;

use super::lexer::{Lexer, TokenKind, parse_spice_value_complete};
use super::parser::spice_comment_start;
use super::source_map::{MappedLine, card_fields, is_param_boundary, logical_values};
use super::spectre_adapter::SourceComments;
use super::{ParameterRedefinitionPolicy, SourceKind};

/// Text an index reads its names from, shared with whoever else holds it
/// rather than copied.
pub trait DeckText: Send + Sync {
    /// Its length in bytes.
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The bytes `range` covers, borrowed where the text holds them in one
    /// piece. A range off the text, or off its character boundaries, is
    /// empty.
    fn slice(&self, range: Range<usize>) -> Cow<'_, str>;

    /// The text from byte `from` on, in the pieces it is held in, in order.
    /// `from` is on a character boundary.
    fn pieces(&self, from: usize) -> Box<dyn Iterator<Item = &str> + '_>;

    /// The text before byte `to`, in the pieces it is held in, last first.
    /// `to` is on a character boundary.
    fn pieces_back(&self, to: usize) -> Box<dyn Iterator<Item = &str> + '_>;
}

/// A text an index shares.
pub type SharedDeckText = Arc<dyn DeckText>;

impl DeckText for String {
    fn len(&self) -> usize {
        self.as_str().len()
    }

    fn slice(&self, range: Range<usize>) -> Cow<'_, str> {
        Cow::Borrowed(self.get(range).unwrap_or_default())
    }

    fn pieces(&self, from: usize) -> Box<dyn Iterator<Item = &str> + '_> {
        Box::new(self.get(from..).into_iter())
    }

    fn pieces_back(&self, to: usize) -> Box<dyn Iterator<Item = &str> + '_> {
        Box::new(self.get(..to).into_iter())
    }
}

/// One file of the deck, as the caller resolved it. The first source given to
/// [`DeckIndex::build`] is the deck itself; the others are the files it
/// includes.
#[derive(Clone)]
pub struct DeckIndexSource<'a> {
    /// What the caller calls the file. A location names its file by the
    /// position of its source in the list; [`DeckIndex::file_identity`] turns
    /// that back into this.
    pub identity: &'a str,
    /// The file's text, shared with the index.
    pub text: SharedDeckText,
    /// How the engine reads the file: whether its first line is a title,
    /// and whether it is SPICE or Spectre.
    pub kind: SourceKind,
}

impl<'a> DeckIndexSource<'a> {
    /// A source whose text the index shares.
    #[must_use]
    pub fn new(identity: &'a str, text: SharedDeckText, kind: SourceKind) -> Self {
        Self {
            identity,
            text,
            kind,
        }
    }

    /// A source of a text copied for the index: what a small text with no
    /// other holder gives.
    #[must_use]
    pub fn copied(identity: &'a str, text: &str, kind: SourceKind) -> Self {
        Self::new(identity, Arc::new(text.to_owned()), kind)
    }
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

/// What a name names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DeckSymbolKind {
    Parameter,
    Function,
    Subcircuit,
    Model,
    Instance,
}

/// An include card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeckIncludeKind {
    Include,
    Library,
}

/// A name under the caret: where it is defined, or a read of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeckNameAt {
    Definition(usize),
    Reference(usize),
}

/// Where an edit fell in a file: the bytes `start..old_end` of the text an
/// index was built from are the bytes `start..new_end` of the text after it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeckTextChange {
    pub start: usize,
    pub old_end: usize,
    pub new_end: usize,
}

impl DeckTextChange {
    /// Where `new` differs from `old`: from the first byte that differs to
    /// the last, compared piece by piece, a piece both share skipped without
    /// reading it. `None` when they hold the same text.
    #[must_use]
    pub fn between(old: &dyn DeckText, new: &dyn DeckText) -> Option<Self> {
        let prefix = common_run(old.pieces(0), new.pieces(0), usize::MAX, false);
        let (old_len, new_len) = (old.len(), new.len());
        if prefix == old_len && prefix == new_len {
            return None;
        }
        let room = old_len.min(new_len) - prefix;
        let suffix = common_run(
            old.pieces_back(old_len),
            new.pieces_back(new_len),
            room,
            true,
        );
        Some(Self {
            start: prefix,
            old_end: old_len - suffix,
            new_end: new_len - suffix,
        })
    }
}

/// How many bytes the two runs of pieces hold the same from their first, at
/// most `most`; `backward` when the pieces run last first. The run ends
/// where a character starts: a character the two write differently is not
/// split.
fn common_run<'a>(
    mut one: impl Iterator<Item = &'a str>,
    mut two: impl Iterator<Item = &'a str>,
    most: usize,
    backward: bool,
) -> usize {
    let continues = |byte: u8| byte & 0xC0 == 0x80;
    let (mut left, mut right): (&[u8], &[u8]) = (&[], &[]);
    let mut same = 0usize;
    // The three bytes of the run nearest where it has got to, nearest last
    // forward and first backward.
    let mut near = [0u8; 3];
    while same < most {
        if left.is_empty() {
            match one.next() {
                Some(piece) => left = piece.as_bytes(),
                None => break,
            }
            continue;
        }
        if right.is_empty() {
            match two.next() {
                Some(piece) => right = piece.as_bytes(),
                None => break,
            }
            continue;
        }
        let length = left.len().min(right.len()).min(most - same);
        let (a, b) = if backward {
            (&left[left.len() - length..], &right[right.len() - length..])
        } else {
            (&left[..length], &right[..length])
        };
        if std::ptr::eq(a, b) || a == b {
            same += length;
            if backward {
                for byte in a.iter().take(3).rev() {
                    near.rotate_right(1);
                    near[0] = *byte;
                }
                left = &left[..left.len() - length];
                right = &right[..right.len() - length];
            } else {
                for byte in a.iter().rev().take(3).rev() {
                    near.rotate_left(1);
                    near[2] = *byte;
                }
                left = &left[length..];
                right = &right[length..];
            }
            continue;
        }
        let pairs = a.iter().zip(b);
        if backward {
            let mut run = pairs.rev().take_while(|(x, y)| x == y).count();
            while run > 0 && continues(a[a.len() - run]) {
                run -= 1;
            }
            if run == 0 {
                same -= near.iter().take_while(|byte| continues(**byte)).count();
            }
            same += run;
        } else {
            let mut run = pairs.take_while(|(x, y)| x == y).count();
            while run > 0 && continues(a[run]) {
                run -= 1;
            }
            if run == 0 && continues(a[0]) {
                same -= near
                    .iter()
                    .rev()
                    .take_while(|byte| continues(**byte))
                    .count()
                    + 1;
            }
            same += run;
        }
        break;
    }
    same.min(most)
}

/// No scope, section, definition or symbol.
const NONE: u32 = u32::MAX;

/// A page closes at the first card that starts this many bytes after it
/// opened.
const PAGE_BYTES: usize = 64 * 1024;

/// A run of bytes from the first byte of the page that holds it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Span {
    start: u32,
    len: u32,
}

impl Span {
    fn of(range: &Range<usize>) -> Self {
        Self {
            start: range.start as u32,
            len: (range.end - range.start) as u32,
        }
    }

    const fn end(self) -> u32 {
        self.start + self.len
    }

    fn at(self, base: usize) -> Range<usize> {
        base + self.start as usize..base + self.end() as usize
    }
}

#[derive(Debug, Clone, Copy)]
struct CardRec {
    kind: DeckCardKind,
    head: Span,
    /// The first and last physical line, from the page's first.
    first_line: u32,
    last_line: u32,
    scope: u32,
    section: u32,
}

/// What a definition is bound to or written with, when it is not a span of
/// the text as it stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DetailKind {
    Empty,
    /// The text `detail` spans, a `+` line's break read as one blank.
    Written,
    /// A subcircuit's port count, `detail.len`.
    Ports,
    /// A function's formals, written between the parentheses `detail`
    /// spans.
    Formals,
}

#[derive(Debug, Clone, Copy)]
struct SymbolRec {
    kind: DeckSymbolKind,
    detail_kind: DetailKind,
    name: Span,
    /// The card, among the page's.
    card: u32,
    scope: u32,
    section: u32,
    detail: Span,
    opens: u32,
    /// The definition an instance is of, by its position among the index's
    /// definitions.
    target: u32,
}

#[derive(Debug, Clone, Copy)]
struct ReadRec {
    kind: DeckSymbolKind,
    name: Span,
    card: u32,
    scope: u32,
    /// What the read binds: a definition by its position among the index's
    /// definitions when `target_page` is `NONE`, else an instance by its
    /// place among the symbols of the page in slot `target_page`.
    target: u32,
    target_page: u32,
}

#[derive(Debug, Clone, Copy)]
struct NodeRec {
    name: Span,
    card: u32,
    scope: u32,
}

#[derive(Debug, Clone, Copy)]
struct IncludeRec {
    kind: DeckIncludeKind,
    /// The path as written, and without its quotes.
    written: Span,
    path: Span,
    section: Option<Span>,
    card: u32,
}

/// How the engine reads the lines from a page's first on: what the lines
/// before it leave.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Carry {
    comments: SourceComments,
    /// Whether the line reads as Spectre, as a `\` at its end sees it.
    lines_spectre: bool,
    /// Whether the cards read as Spectre, and how deep in a `{ }` block.
    spectre: bool,
    braces: u32,
    scopes: Vec<u32>,
    section: u32,
    ended: bool,
    /// Whether a `.control` block is open.
    control: bool,
    /// Whether the last line ends in a `\` that continues its card.
    continued: bool,
}

impl Carry {
    fn at_start(spectre: bool) -> Self {
        Self {
            comments: SourceComments::new(spectre),
            lines_spectre: spectre,
            spectre,
            braces: 0,
            scopes: Vec::new(),
            section: NONE,
            ended: false,
            control: false,
            continued: false,
        }
    }
}

impl Default for Carry {
    fn default() -> Self {
        Self::at_start(false)
    }
}

/// One page: whole lines of one file, and every card, name and read written
/// on them, placed from its first byte and line.
#[derive(Debug, Clone, Default)]
struct Page {
    bytes: u32,
    /// Where each physical line starts, from the page's first byte.
    line_starts: Vec<u32>,
    entry: Carry,
    exit: Carry,
    cards: Vec<CardRec>,
    /// In the order they are read, which is the order of their cards.
    symbols: Vec<SymbolRec>,
    /// In the order of where they are written.
    reads: Vec<ReadRec>,
    nodes: Vec<NodeRec>,
    includes: Vec<IncludeRec>,
    /// The lines with a comment, and the byte on the line it starts at.
    comments: Vec<(u32, u32)>,
    /// Whether each line reads as Spectre; empty when none does.
    spectre_lines: Vec<bool>,
    /// The symbols that are not instances, in order.
    definitions: Vec<u32>,
    /// The `.global` names written, in order.
    globals: Vec<Span>,
    /// The `.lib name` cards, and the name each opens.
    section_openers: Vec<(u32, Span)>,
    /// The scopes and sections the page's cards open, in order.
    opened_scopes: Vec<u32>,
    opened_sections: Vec<u32>,
    /// The symbols in the order of where they are written; empty when that
    /// is the order they are read in.
    symbols_by_place: Vec<u32>,
    /// The reads that bind a definition, by the definition and then in
    /// order; and the reads of an instance.
    reads_by_target: Vec<u32>,
    instance_reads: Vec<u32>,
    widest_symbol: u32,
    widest_read: u32,
    widest_node: u32,
    /// Whether the page reads its file's first line as the title.
    titled: bool,
}

impl Page {
    fn line_of(&self, at: u32) -> u32 {
        (self.line_starts.partition_point(|start| *start <= at) as u32).saturating_sub(1)
    }

    fn comment_on(&self, line: u32) -> Option<usize> {
        self.comments
            .binary_search_by_key(&line, |(on, _)| *on)
            .ok()
            .map(|at| self.comments[at].1 as usize)
    }

    fn spectre_on(&self, line: u32) -> bool {
        self.spectre_lines
            .get(line as usize)
            .copied()
            .unwrap_or(false)
    }

    fn line_count(&self) -> u32 {
        self.line_starts.len() as u32
    }

    /// The symbol at position `at` of the order of where they are written.
    fn symbol_by_place(&self, at: usize) -> u32 {
        self.symbols_by_place.get(at).copied().unwrap_or(at as u32)
    }

    /// The symbols, reads and nodes written on card `card`, as ranges of
    /// the page's.
    fn symbols_of(&self, card: u32) -> Range<usize> {
        let first = self.symbols.partition_point(|symbol| symbol.card < card);
        first..first + self.symbols[first..].partition_point(|symbol| symbol.card == card)
    }

    fn shrink(&mut self) {
        self.line_starts.shrink_to_fit();
        self.cards.shrink_to_fit();
        self.symbols.shrink_to_fit();
        self.reads.shrink_to_fit();
        self.nodes.shrink_to_fit();
        self.includes.shrink_to_fit();
        self.comments.shrink_to_fit();
        self.spectre_lines.shrink_to_fit();
        self.definitions.shrink_to_fit();
        self.symbols_by_place.shrink_to_fit();
        self.reads_by_target.shrink_to_fit();
        self.instance_reads.shrink_to_fit();
    }
}

/// One page as an index version holds it: where it is placed in its file,
/// and how many of each record the pages before it hold.
#[derive(Debug, Clone)]
struct PageSlot {
    page: Arc<Page>,
    file: u32,
    start: usize,
    first_line: u32,
    card_base: u32,
    symbol_base: u32,
    read_base: u32,
    node_base: u32,
}

/// The top level of the deck, or one subcircuit definition.
#[derive(Debug, Clone, Copy)]
struct ScopeRec {
    parent: u32,
    /// The subcircuit definition that opens it, by position among the
    /// index's definitions.
    opener: u32,
}

/// What every page reads the same: the files, scopes and sections, and the
/// definitions with their order by name. An edit that leaves them as they
/// were shares them with the index before it.
#[derive(Debug, Default)]
struct Shared {
    files: Vec<String>,
    scopes: Vec<ScopeRec>,
    /// The `.lib name` sections, by the slot of the page that opens each and
    /// its place among the page's openers.
    sections: Vec<(u32, u32)>,
    /// Every definition, by the slot of its page and its place among the
    /// page's, in source order.
    definitions: Vec<(u32, u32)>,
    /// Positions in `definitions` ordered by kind, scope and name ignoring
    /// case, then in source order.
    by_name: Vec<u32>,
    /// `.global` names, by slot and place among the page's, ordered by name
    /// ignoring case.
    globals: Vec<(u32, u32)>,
}

/// The index of one deck and its include files. A clone shares every page.
#[derive(Clone)]
pub struct DeckIndex {
    texts: Vec<SharedDeckText>,
    pages: Vec<PageSlot>,
    /// Where each file's pages start among `pages`, then one past the last.
    file_pages: Vec<u32>,
    shared: Arc<Shared>,
}

impl Default for DeckIndex {
    fn default() -> Self {
        Self {
            texts: Vec::new(),
            pages: Vec::new(),
            file_pages: vec![0],
            shared: Arc::new(Shared {
                scopes: vec![ScopeRec {
                    parent: NONE,
                    opener: NONE,
                }],
                ..Shared::default()
            }),
        }
    }
}

impl std::fmt::Debug for DeckIndex {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DeckIndex")
            .field("files", &self.shared.files)
            .field("pages", &self.pages.len())
            .field("cards", &self.card_count())
            .field("symbols", &self.symbol_count())
            .finish_non_exhaustive()
    }
}

/// Two indexes are equal when they read every card, name, read and node the
/// same.
impl PartialEq for DeckIndex {
    fn eq(&self, other: &Self) -> bool {
        self.shared.files == other.shared.files
            && self.cards().eq(other.cards())
            && self.symbols().eq(other.symbols())
            && self.references().eq(other.references())
            && self.nodes().eq(other.nodes())
            && self.includes().eq(other.includes())
            && self.scopes().eq(other.scopes())
    }
}

impl Eq for DeckIndex {}

// ---------------------------------------------------------------------------
// What a reader is handed: views of the records, read from the text.

/// One logical card: a physical line and the `+` lines that continue it.
#[derive(Clone, Copy)]
pub struct DeckCard<'a> {
    deck: &'a DeckIndex,
    slot: &'a PageSlot,
    rec: &'a CardRec,
    local: u32,
}

impl<'a> DeckCard<'a> {
    /// Its position among the index's cards.
    #[must_use]
    pub const fn id(&self) -> usize {
        (self.slot.card_base + self.local) as usize
    }

    #[must_use]
    pub const fn kind(&self) -> DeckCardKind {
        self.rec.kind
    }

    /// The card's first word as written: `.subckt`, `R1`. The whole line for
    /// the title.
    #[must_use]
    pub fn head(&self) -> Cow<'a, str> {
        self.deck.text_at(self.slot, self.rec.head)
    }

    /// Where the head is written.
    #[must_use]
    pub fn location(&self) -> DeckLocation {
        self.deck.location(self.slot, self.rec.head)
    }

    #[must_use]
    pub const fn file(&self) -> usize {
        self.slot.file as usize
    }

    /// The first and last one-based physical line, continuation lines
    /// included.
    #[must_use]
    pub const fn lines(&self) -> RangeInclusive<usize> {
        let first = self.slot.first_line + 1;
        (first + self.rec.first_line) as usize..=(first + self.rec.last_line) as usize
    }

    /// The bytes the card's lines are written on, the last one's break
    /// included.
    #[must_use]
    pub fn bytes(&self) -> Range<usize> {
        let page = &self.slot.page;
        let start = page.line_starts[self.rec.first_line as usize] as usize;
        let end = page
            .line_starts
            .get(self.rec.last_line as usize + 1)
            .map_or(page.bytes as usize, |end| *end as usize);
        self.slot.start + start..self.slot.start + end
    }

    /// The scope the card is written in. A `.subckt` card is written in the
    /// scope that encloses the subcircuit; its `.ends` inside it.
    #[must_use]
    pub const fn scope(&self) -> usize {
        self.rec.scope as usize
    }

    /// The library section the card sits in, `.lib name` to `.endl`.
    #[must_use]
    pub fn section(&self) -> Option<Cow<'a, str>> {
        self.deck.section_name(self.rec.section)
    }

    /// The definitions the card writes, in the order they are read.
    pub fn symbols(&self) -> impl Iterator<Item = DeckSymbol<'a>> + 'a {
        let (deck, slot) = (self.deck, self.slot);
        slot.page
            .symbols_of(self.local)
            .map(move |local| DeckSymbol {
                deck,
                slot,
                rec: &slot.page.symbols[local],
                local: local as u32,
            })
    }

    /// The reads written on the card, in source order.
    pub fn references(&self) -> impl Iterator<Item = DeckReference<'a>> + 'a {
        let (deck, slot, card) = (self.deck, self.slot, self.local);
        let reads = &slot.page.reads;
        let first = reads.partition_point(|read| read.card < card);
        let end = first + reads[first..].partition_point(|read| read.card == card);
        (first..end).map(move |local| DeckReference {
            deck,
            slot,
            rec: &reads[local],
            local: local as u32,
        })
    }

    /// The tokens of the card as the engine's lexer reads them. A title or a
    /// `.control` block has none.
    #[must_use]
    pub fn tokens(&self) -> Vec<DeckToken> {
        self.deck.tokens_of(self)
    }
}

impl std::fmt::Debug for DeckCard<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DeckCard")
            .field("kind", &self.kind())
            .field("head", &self.head())
            .field("location", &self.location())
            .field("lines", &self.lines())
            .field("scope", &self.scope())
            .field("section", &self.section())
            .finish()
    }
}

impl PartialEq for DeckCard<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.kind() == other.kind()
            && self.head() == other.head()
            && self.location() == other.location()
            && self.lines() == other.lines()
            && self.scope() == other.scope()
            && self.section() == other.section()
    }
}

/// The top level of the deck, or one subcircuit definition.
#[derive(Clone, Copy)]
pub struct DeckScope<'a> {
    deck: &'a DeckIndex,
    id: usize,
}

impl<'a> DeckScope<'a> {
    #[must_use]
    pub const fn id(&self) -> usize {
        self.id
    }

    /// The subcircuit's name; `None` for the top level.
    #[must_use]
    pub fn name(&self) -> Option<Cow<'a, str>> {
        self.opener().map(|symbol| symbol.name())
    }

    #[must_use]
    pub fn parent(&self) -> Option<usize> {
        let parent = self.deck.shared.scopes[self.id].parent;
        (parent != NONE).then_some(parent as usize)
    }

    /// The subcircuit symbol that opens the scope.
    #[must_use]
    pub fn symbol(&self) -> Option<usize> {
        self.opener().map(|symbol| symbol.id())
    }

    fn opener(&self) -> Option<DeckSymbol<'a>> {
        let opener = self.deck.shared.scopes[self.id].opener;
        (opener != NONE).then(|| self.deck.definition(opener as usize))
    }

    /// The subcircuit's ports, in order.
    #[must_use]
    pub fn ports(&self) -> Vec<Cow<'a, str>> {
        let Some(opener) = self.opener() else {
            return Vec::new();
        };
        let count = match opener.rec.detail_kind {
            DetailKind::Ports => opener.rec.detail.len as usize,
            _ => 0,
        };
        let slot = opener.slot;
        let nodes = &slot.page.nodes;
        let card = opener.rec.card;
        let first = nodes.partition_point(|node| node.card < card);
        nodes[first..]
            .iter()
            .take_while(|node| node.card == card)
            .take(count)
            .map(|node| self.deck.text_at(slot, node.name))
            .collect()
    }
}

impl std::fmt::Debug for DeckScope<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DeckScope")
            .field("name", &self.name())
            .field("parent", &self.parent())
            .field("symbol", &self.symbol())
            .field("ports", &self.ports())
            .finish()
    }
}

impl PartialEq for DeckScope<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
            && self.parent() == other.parent()
            && self.symbol() == other.symbol()
            && self.ports() == other.ports()
    }
}

/// One definition.
#[derive(Clone, Copy)]
pub struct DeckSymbol<'a> {
    deck: &'a DeckIndex,
    slot: &'a PageSlot,
    rec: &'a SymbolRec,
    local: u32,
}

impl<'a> DeckSymbol<'a> {
    /// Its position among the index's symbols.
    #[must_use]
    pub const fn id(&self) -> usize {
        (self.slot.symbol_base + self.local) as usize
    }

    #[must_use]
    pub const fn kind(&self) -> DeckSymbolKind {
        self.rec.kind
    }

    /// The name as written.
    #[must_use]
    pub fn name(&self) -> Cow<'a, str> {
        self.deck.text_at(self.slot, self.rec.name)
    }

    /// The scope the name is defined in. A subcircuit's parameters are
    /// defined in its own scope; the subcircuit in the one enclosing it.
    #[must_use]
    pub const fn scope(&self) -> usize {
        self.rec.scope as usize
    }

    /// Where the name is written.
    #[must_use]
    pub fn location(&self) -> DeckLocation {
        self.deck.location(self.slot, self.rec.name)
    }

    #[must_use]
    pub const fn file(&self) -> usize {
        self.slot.file as usize
    }

    /// The one-based physical line the name is written on.
    #[must_use]
    pub fn line(&self) -> usize {
        (self.slot.first_line + self.slot.page.line_of(self.rec.name.start) + 1) as usize
    }

    /// The bytes the name is written on.
    #[must_use]
    pub fn span(&self) -> Range<usize> {
        self.rec.name.at(self.slot.start)
    }

    /// The card that defines it, by its position among the index's.
    #[must_use]
    pub const fn card(&self) -> usize {
        (self.slot.card_base + self.rec.card) as usize
    }

    /// The card that defines it.
    #[must_use]
    pub fn card_view(&self) -> DeckCard<'a> {
        DeckCard {
            deck: self.deck,
            slot: self.slot,
            rec: &self.slot.page.cards[self.rec.card as usize],
            local: self.rec.card,
        }
    }

    /// The library section it is written in.
    #[must_use]
    pub fn section(&self) -> Option<Cow<'a, str>> {
        self.deck.section_name(self.rec.section)
    }

    /// A model's type, a subcircuit's port count, a parameter's value, a
    /// function's formals, an instance's subcircuit or model, or the value a
    /// resistor, capacitor or inductor is written with.
    #[must_use]
    pub fn detail(&self) -> Cow<'a, str> {
        match self.rec.detail_kind {
            DetailKind::Empty => Cow::Borrowed(""),
            DetailKind::Written => joined(self.deck.text_at(self.slot, self.rec.detail)),
            DetailKind::Ports => Cow::Owned(if self.rec.detail.len == 1 {
                "1 port".to_owned()
            } else {
                format!("{} ports", self.rec.detail.len)
            }),
            DetailKind::Formals => {
                let written = joined(self.deck.text_at(self.slot, self.rec.detail));
                let formals = written
                    .split([',', ' '])
                    .map(str::trim)
                    .filter(|formal| !formal.is_empty())
                    .collect::<Vec<_>>();
                Cow::Owned(format!("({})", formals.join(", ")))
            }
        }
    }

    /// The scope a subcircuit opens.
    #[must_use]
    pub const fn opens(&self) -> Option<usize> {
        if self.rec.opens == NONE {
            None
        } else {
            Some(self.rec.opens as usize)
        }
    }

    /// The subcircuit or model an instance is of.
    #[must_use]
    pub fn target(&self) -> Option<usize> {
        (self.rec.target != NONE).then(|| self.deck.definition(self.rec.target as usize).id())
    }
}

impl std::fmt::Debug for DeckSymbol<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DeckSymbol")
            .field("kind", &self.kind())
            .field("name", &self.name())
            .field("scope", &self.scope())
            .field("location", &self.location())
            .field("card", &self.card())
            .field("section", &self.section())
            .field("detail", &self.detail())
            .field("opens", &self.opens())
            .field("target", &self.target())
            .finish()
    }
}

impl PartialEq for DeckSymbol<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.kind() == other.kind()
            && self.name() == other.name()
            && self.scope() == other.scope()
            && self.location() == other.location()
            && self.card() == other.card()
            && self.section() == other.section()
            && self.detail() == other.detail()
            && self.opens() == other.opens()
            && self.target() == other.target()
    }
}

/// One read of a name.
#[derive(Clone, Copy)]
pub struct DeckReference<'a> {
    deck: &'a DeckIndex,
    slot: &'a PageSlot,
    rec: &'a ReadRec,
    local: u32,
}

impl<'a> DeckReference<'a> {
    /// Its position among the index's reads.
    #[must_use]
    pub const fn id(&self) -> usize {
        (self.slot.read_base + self.local) as usize
    }

    #[must_use]
    pub const fn kind(&self) -> DeckSymbolKind {
        self.rec.kind
    }

    /// The name as written.
    #[must_use]
    pub fn name(&self) -> Cow<'a, str> {
        self.deck.text_at(self.slot, self.rec.name)
    }

    /// The scope the name is read in.
    #[must_use]
    pub const fn scope(&self) -> usize {
        self.rec.scope as usize
    }

    #[must_use]
    pub fn location(&self) -> DeckLocation {
        self.deck.location(self.slot, self.rec.name)
    }

    #[must_use]
    pub const fn file(&self) -> usize {
        self.slot.file as usize
    }

    /// The one-based physical line the name is written on.
    #[must_use]
    pub fn line(&self) -> usize {
        (self.slot.first_line + self.slot.page.line_of(self.rec.name.start) + 1) as usize
    }

    #[must_use]
    pub fn span(&self) -> Range<usize> {
        self.rec.name.at(self.slot.start)
    }

    #[must_use]
    pub const fn card(&self) -> usize {
        (self.slot.card_base + self.rec.card) as usize
    }

    /// The definition SPICE scoping binds the read to; `None` when nothing in
    /// scope defines the name.
    #[must_use]
    pub fn target(&self) -> Option<usize> {
        match (self.rec.target_page, self.rec.target) {
            (_, NONE) => None,
            (NONE, definition) => Some(self.deck.definition(definition as usize).id()),
            (page, local) => {
                Some(self.deck.pages[page as usize].symbol_base as usize + local as usize)
            }
        }
    }
}

impl std::fmt::Debug for DeckReference<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DeckReference")
            .field("kind", &self.kind())
            .field("name", &self.name())
            .field("scope", &self.scope())
            .field("location", &self.location())
            .field("card", &self.card())
            .field("target", &self.target())
            .finish()
    }
}

impl PartialEq for DeckReference<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.kind() == other.kind()
            && self.name() == other.name()
            && self.scope() == other.scope()
            && self.location() == other.location()
            && self.card() == other.card()
            && self.target() == other.target()
    }
}

/// One place a node is written.
#[derive(Clone, Copy)]
pub struct DeckNode<'a> {
    deck: &'a DeckIndex,
    slot: &'a PageSlot,
    rec: &'a NodeRec,
    local: u32,
}

impl<'a> DeckNode<'a> {
    /// Its position among the index's nodes.
    #[must_use]
    pub const fn id(&self) -> usize {
        (self.slot.node_base + self.local) as usize
    }

    /// The name as written.
    #[must_use]
    pub fn name(&self) -> Cow<'a, str> {
        self.deck.text_at(self.slot, self.rec.name)
    }

    /// The scope the node belongs to: the top level for ground and every
    /// `.global` node, else the scope the card is written in.
    #[must_use]
    pub const fn scope(&self) -> usize {
        self.rec.scope as usize
    }

    #[must_use]
    pub fn location(&self) -> DeckLocation {
        self.deck.location(self.slot, self.rec.name)
    }

    #[must_use]
    pub const fn file(&self) -> usize {
        self.slot.file as usize
    }

    /// The one-based physical line the name is written on.
    #[must_use]
    pub fn line(&self) -> usize {
        (self.slot.first_line + self.slot.page.line_of(self.rec.name.start) + 1) as usize
    }

    #[must_use]
    pub fn span(&self) -> Range<usize> {
        self.rec.name.at(self.slot.start)
    }

    #[must_use]
    pub const fn card(&self) -> usize {
        (self.slot.card_base + self.rec.card) as usize
    }

    /// The kind of the card it is written on.
    #[must_use]
    pub fn card_kind(&self) -> DeckCardKind {
        self.slot.page.cards[self.rec.card as usize].kind
    }
}

impl std::fmt::Debug for DeckNode<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DeckNode")
            .field("name", &self.name())
            .field("scope", &self.scope())
            .field("location", &self.location())
            .field("card", &self.card())
            .finish()
    }
}

impl PartialEq for DeckNode<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
            && self.scope() == other.scope()
            && self.location() == other.location()
            && self.card() == other.card()
    }
}

/// One `.include` or `.lib file section` card.
#[derive(Clone, Copy)]
pub struct DeckInclude<'a> {
    deck: &'a DeckIndex,
    slot: &'a PageSlot,
    rec: &'a IncludeRec,
}

impl<'a> DeckInclude<'a> {
    #[must_use]
    pub const fn kind(&self) -> DeckIncludeKind {
        self.rec.kind
    }

    /// The path as written, quotes removed.
    #[must_use]
    pub fn path(&self) -> Cow<'a, str> {
        self.deck.text_at(self.slot, self.rec.path)
    }

    /// The section a `.lib` card selects.
    #[must_use]
    pub fn section(&self) -> Option<Cow<'a, str>> {
        self.rec
            .section
            .map(|section| self.deck.text_at(self.slot, section))
    }

    /// Where the path is written, its quotes included.
    #[must_use]
    pub fn location(&self) -> DeckLocation {
        self.deck.location(self.slot, self.rec.written)
    }

    #[must_use]
    pub const fn card(&self) -> usize {
        (self.slot.card_base + self.rec.card) as usize
    }
}

impl std::fmt::Debug for DeckInclude<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DeckInclude")
            .field("kind", &self.kind())
            .field("path", &self.path())
            .field("section", &self.section())
            .field("location", &self.location())
            .field("card", &self.card())
            .finish()
    }
}

impl PartialEq for DeckInclude<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.kind() == other.kind()
            && self.path() == other.path()
            && self.section() == other.section()
            && self.location() == other.location()
            && self.card() == other.card()
    }
}

/// `text` as one line: each line break and the `+` that continues the card
/// after it read as one blank, as the engine joins a card's lines.
fn joined(text: Cow<'_, str>) -> Cow<'_, str> {
    if !text.contains('\n') {
        return text;
    }
    let mut out = String::with_capacity(text.len());
    for (at, line) in text.split('\n').enumerate() {
        let line = line.strip_suffix('\r').unwrap_or(line);
        if at == 0 {
            out.push_str(line);
            continue;
        }
        let rest = line.trim_start();
        let rest = rest.strip_prefix('+').unwrap_or(rest).trim_start();
        if !out.ends_with(char::is_whitespace) {
            out.push(' ');
        }
        out.push_str(rest);
    }
    Cow::Owned(out)
}

// ---------------------------------------------------------------------------
// Letting go of the index.

/// What lets go of a large index's pages: shown each table of a page as the
/// page is freed, and told when it is.
///
/// A large deck's index holds thousands of tables of tens of kilobytes.
/// Whoever frees them can hand their memory back to the system before the
/// heap takes them, and pause between pages.
pub trait PageRelease {
    /// One table of a page about to be freed: its bytes, which nothing reads
    /// again.
    fn release(&mut self, table: &mut [MaybeUninit<u8>]);

    /// A page was freed.
    fn page_freed(&mut self);
}

/// The bytes `table` holds, written or not.
fn table_bytes<T: Copy>(table: &mut Vec<T>) -> &mut [MaybeUninit<u8>] {
    let len = table.capacity() * std::mem::size_of::<T>();
    // SAFETY: the buffer holds `capacity` elements, so `len` bytes from its
    // start, all of them readable as `MaybeUninit<u8>`, and the slice
    // borrows the vector mutably for as long as it lives.
    unsafe { std::slice::from_raw_parts_mut(table.as_mut_ptr().cast::<MaybeUninit<u8>>(), len) }
}

impl Page {
    /// Show `release` each of the page's tables, before they are freed.
    fn release_tables(&mut self, release: &mut dyn PageRelease) {
        release.release(table_bytes(&mut self.line_starts));
        release.release(table_bytes(&mut self.cards));
        release.release(table_bytes(&mut self.symbols));
        release.release(table_bytes(&mut self.reads));
        release.release(table_bytes(&mut self.nodes));
        release.release(table_bytes(&mut self.includes));
        release.release(table_bytes(&mut self.comments));
        release.release(table_bytes(&mut self.spectre_lines));
        release.release(table_bytes(&mut self.definitions));
        release.release(table_bytes(&mut self.globals));
        release.release(table_bytes(&mut self.section_openers));
        release.release(table_bytes(&mut self.opened_scopes));
        release.release(table_bytes(&mut self.opened_sections));
        release.release(table_bytes(&mut self.symbols_by_place));
        release.release(table_bytes(&mut self.reads_by_target));
        release.release(table_bytes(&mut self.instance_reads));
    }
}

impl DeckIndex {
    /// Let go of the index page by page through `release`: each page it
    /// holds the last share of is shown to `release` table by table, then
    /// freed; a page another index still shares is left to that index.
    pub fn let_go(self, release: &mut dyn PageRelease) {
        let mut pages = self.pages;
        while let Some(slot) = pages.pop() {
            if let Some(mut page) = Arc::into_inner(slot.page) {
                page.release_tables(release);
                drop(page);
                release.page_freed();
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Reading the index.

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
        self.shared.files.get(file).map(String::as_str)
    }

    #[must_use]
    pub fn files(&self) -> &[String] {
        &self.shared.files
    }

    /// The text of file `file`, shared with the index.
    #[must_use]
    pub fn text(&self, file: usize) -> Option<&SharedDeckText> {
        self.texts.get(file)
    }

    /// How many lines file `file` has, as `str::lines` counts them.
    #[must_use]
    pub fn line_count(&self, file: usize) -> usize {
        self.file_slots(file).last().map_or(0, |slot| {
            (slot.first_line + slot.page.line_count()) as usize
        })
    }

    /// The bytes one-based `line` of file `file` is written on, its break
    /// included.
    #[must_use]
    pub fn line_span(&self, file: usize, line: usize) -> Option<Range<usize>> {
        let at = self.slot_on_line(file, line.checked_sub(1)? as u32)?;
        let slot = &self.pages[at];
        let page = &slot.page;
        let local = (line - 1) as u32 - slot.first_line;
        let start = page.line_starts[local as usize] as usize;
        let end = page
            .line_starts
            .get(local as usize + 1)
            .map_or(page.bytes as usize, |end| *end as usize);
        Some(slot.start + start..slot.start + end)
    }

    fn text_at<'a>(&'a self, slot: &PageSlot, span: Span) -> Cow<'a, str> {
        self.texts[slot.file as usize].slice(span.at(slot.start))
    }

    fn location(&self, slot: &PageSlot, span: Span) -> DeckLocation {
        let page = &slot.page;
        let line = page.line_of(span.start);
        let line_start = page.line_starts[line as usize];
        let before = self.texts[slot.file as usize]
            .slice(slot.start + line_start as usize..slot.start + span.start as usize);
        DeckLocation {
            file: slot.file as usize,
            span: span.at(slot.start),
            line: (slot.first_line + line + 1) as usize,
            column: before.chars().count() + 1,
        }
    }

    fn section_name(&self, section: u32) -> Option<Cow<'_, str>> {
        let &(slot, opener) = self.shared.sections.get(section as usize)?;
        let slot = &self.pages[slot as usize];
        let (_, name) = slot.page.section_openers[opener as usize];
        Some(self.text_at(slot, name))
    }

    fn file_slots(&self, file: usize) -> &[PageSlot] {
        match (self.file_pages.get(file), self.file_pages.get(file + 1)) {
            (Some(&first), Some(&end)) => &self.pages[first as usize..end as usize],
            _ => &[],
        }
    }

    /// The slot of the page of `file` holding byte `byte`.
    fn slot_at(&self, file: usize, byte: usize) -> Option<usize> {
        let first = *self.file_pages.get(file)? as usize;
        let slots = self.file_slots(file);
        let at = slots
            .partition_point(|slot| slot.start <= byte)
            .checked_sub(1)?;
        Some(first + at)
    }

    /// The slot of the page of `file` holding zero-based line `line`.
    fn slot_on_line(&self, file: usize, line: u32) -> Option<usize> {
        let first = *self.file_pages.get(file)? as usize;
        let slots = self.file_slots(file);
        let at = slots
            .partition_point(|slot| slot.first_line <= line)
            .checked_sub(1)?;
        (line < slots[at].first_line + slots[at].page.line_count()).then_some(first + at)
    }

    /// The slot holding record `id` of a kind each slot counts from `base`.
    fn slot_of(&self, base: impl Fn(&PageSlot) -> u32, id: usize) -> Option<usize> {
        self.pages
            .partition_point(|slot| base(slot) as usize <= id)
            .checked_sub(1)
    }

    fn definition(&self, definition: usize) -> DeckSymbol<'_> {
        let (slot, ordinal) = self.shared.definitions[definition];
        let slot = &self.pages[slot as usize];
        let local = slot.page.definitions[ordinal as usize];
        DeckSymbol {
            deck: self,
            slot,
            rec: &slot.page.symbols[local as usize],
            local,
        }
    }

    /// The position among the index's definitions of `symbol`; `None` for an
    /// instance.
    fn definition_of(&self, symbol: &DeckSymbol<'_>) -> Option<usize> {
        let ordinal = symbol
            .slot
            .page
            .definitions
            .binary_search(&symbol.local)
            .ok()?;
        let slot = self.slot_of(|slot| slot.symbol_base, symbol.id())?;
        self.shared
            .definitions
            .binary_search(&(slot as u32, ordinal as u32))
            .ok()
    }

    fn card_view<'a>(&'a self, slot: &'a PageSlot, local: usize) -> DeckCard<'a> {
        DeckCard {
            deck: self,
            slot,
            rec: &slot.page.cards[local],
            local: local as u32,
        }
    }

    #[must_use]
    pub fn card_count(&self) -> usize {
        self.pages
            .last()
            .map_or(0, |slot| slot.card_base as usize + slot.page.cards.len())
    }

    /// The card at position `id` among the index's.
    #[must_use]
    pub fn card(&self, id: usize) -> Option<DeckCard<'_>> {
        let slot = &self.pages[self.slot_of(|slot| slot.card_base, id)?];
        let local = id - slot.card_base as usize;
        (local < slot.page.cards.len()).then(|| self.card_view(slot, local))
    }

    /// Every card, file by file in the order the sources were given, each
    /// file's in source order.
    pub fn cards(&self) -> impl Iterator<Item = DeckCard<'_>> + '_ {
        self.pages.iter().flat_map(move |slot| {
            (0..slot.page.cards.len()).map(move |local| self.card_view(slot, local))
        })
    }

    /// The cards of file `file`, in source order.
    pub fn cards_of(&self, file: usize) -> impl Iterator<Item = DeckCard<'_>> + '_ {
        self.file_slots(file).iter().flat_map(move |slot| {
            (0..slot.page.cards.len()).map(move |local| self.card_view(slot, local))
        })
    }

    #[must_use]
    pub fn scope_count(&self) -> usize {
        self.shared.scopes.len()
    }

    #[must_use]
    pub fn scope(&self, id: usize) -> Option<DeckScope<'_>> {
        (id < self.shared.scopes.len()).then_some(DeckScope { deck: self, id })
    }

    pub fn scopes(&self) -> impl Iterator<Item = DeckScope<'_>> + '_ {
        (0..self.shared.scopes.len()).map(move |id| DeckScope { deck: self, id })
    }

    #[must_use]
    pub fn symbol_count(&self) -> usize {
        self.pages.last().map_or(0, |slot| {
            slot.symbol_base as usize + slot.page.symbols.len()
        })
    }

    #[must_use]
    pub fn symbol(&self, id: usize) -> Option<DeckSymbol<'_>> {
        let slot = &self.pages[self.slot_of(|slot| slot.symbol_base, id)?];
        let local = id - slot.symbol_base as usize;
        let rec = slot.page.symbols.get(local)?;
        Some(DeckSymbol {
            deck: self,
            slot,
            rec,
            local: local as u32,
        })
    }

    /// Every definition, in source order.
    pub fn symbols(&self) -> impl Iterator<Item = DeckSymbol<'_>> + '_ {
        self.pages.iter().flat_map(move |slot| {
            slot.page
                .symbols
                .iter()
                .enumerate()
                .map(move |(local, rec)| DeckSymbol {
                    deck: self,
                    slot,
                    rec,
                    local: local as u32,
                })
        })
    }

    /// How many definitions but instances the deck writes.
    #[must_use]
    pub fn definition_count(&self) -> usize {
        self.shared.definitions.len()
    }

    /// The definition at `position` of [`Self::definitions`]. A position
    /// stays the definition's through an edit [`Self::edited`] reads.
    #[must_use]
    pub fn nth_definition(&self, position: usize) -> Option<DeckSymbol<'_>> {
        (position < self.shared.definitions.len()).then(|| self.definition(position))
    }

    /// The one-based line of `file` holding byte `byte`.
    #[must_use]
    pub fn line_of(&self, file: usize, byte: usize) -> Option<usize> {
        let slot = &self.pages[self.slot_at(file, byte)?];
        let line = slot.page.line_of((byte - slot.start) as u32);
        Some((slot.first_line + line + 1) as usize)
    }

    /// Every definition but an instance, in source order: the subcircuits,
    /// models, parameters and functions a reader binds by scope, found
    /// without reading past the deck's devices.
    pub fn definitions(&self) -> impl Iterator<Item = DeckSymbol<'_>> + '_ {
        (0..self.shared.definitions.len()).map(move |definition| self.definition(definition))
    }

    #[must_use]
    pub fn reference_count(&self) -> usize {
        self.pages
            .last()
            .map_or(0, |slot| slot.read_base as usize + slot.page.reads.len())
    }

    #[must_use]
    pub fn reference(&self, id: usize) -> Option<DeckReference<'_>> {
        let slot = &self.pages[self.slot_of(|slot| slot.read_base, id)?];
        let local = id - slot.read_base as usize;
        let rec = slot.page.reads.get(local)?;
        Some(DeckReference {
            deck: self,
            slot,
            rec,
            local: local as u32,
        })
    }

    /// Every read, in source order.
    pub fn references(&self) -> impl Iterator<Item = DeckReference<'_>> + '_ {
        self.pages.iter().flat_map(move |slot| {
            slot.page
                .reads
                .iter()
                .enumerate()
                .map(move |(local, rec)| DeckReference {
                    deck: self,
                    slot,
                    rec,
                    local: local as u32,
                })
        })
    }

    #[must_use]
    pub fn node_count(&self) -> usize {
        self.pages
            .last()
            .map_or(0, |slot| slot.node_base as usize + slot.page.nodes.len())
    }

    #[must_use]
    pub fn node(&self, id: usize) -> Option<DeckNode<'_>> {
        let slot = &self.pages[self.slot_of(|slot| slot.node_base, id)?];
        let local = id - slot.node_base as usize;
        let rec = slot.page.nodes.get(local)?;
        Some(DeckNode {
            deck: self,
            slot,
            rec,
            local: local as u32,
        })
    }

    /// Every place a node is written, in source order.
    pub fn nodes(&self) -> impl Iterator<Item = DeckNode<'_>> + '_ {
        self.pages.iter().flat_map(move |slot| {
            slot.page
                .nodes
                .iter()
                .enumerate()
                .map(move |(local, rec)| DeckNode {
                    deck: self,
                    slot,
                    rec,
                    local: local as u32,
                })
        })
    }

    pub fn includes(&self) -> impl Iterator<Item = DeckInclude<'_>> + '_ {
        self.pages.iter().flat_map(move |slot| {
            slot.page.includes.iter().map(move |rec| DeckInclude {
                deck: self,
                slot,
                rec,
            })
        })
    }

    /// The reads `symbol` binds, in source order.
    pub fn references_to(&self, symbol: usize) -> impl Iterator<Item = DeckReference<'_>> + '_ {
        let mut found = Vec::new();
        if let Some(found_symbol) = self.symbol(symbol) {
            if found_symbol.kind() == DeckSymbolKind::Instance {
                let page = self.slot_of(|slot| slot.symbol_base, symbol).unwrap_or(0) as u32;
                for slot in &self.pages {
                    let reads = &slot.page.reads;
                    found.extend(
                        slot.page
                            .instance_reads
                            .iter()
                            .filter(|local| {
                                let read = &reads[**local as usize];
                                read.target_page == page && read.target == found_symbol.local
                            })
                            .map(|local| DeckReference {
                                deck: self,
                                slot,
                                rec: &reads[*local as usize],
                                local: *local,
                            }),
                    );
                }
            } else if let Some(definition) = self.definition_of(&found_symbol) {
                let definition = definition as u32;
                for slot in &self.pages {
                    let page = &slot.page;
                    let target = |at: &u32| page.reads[*at as usize].target;
                    let first = page
                        .reads_by_target
                        .partition_point(|at| target(at) < definition);
                    let end = page
                        .reads_by_target
                        .partition_point(|at| target(at) <= definition);
                    found.extend(page.reads_by_target[first..end].iter().map(|local| {
                        DeckReference {
                            deck: self,
                            slot,
                            rec: &page.reads[*local as usize],
                            local: *local,
                        }
                    }));
                }
            }
        }
        found.into_iter()
    }

    /// The page and zero-based line on it of one-based `line` of `file`.
    fn on_line(&self, file: usize, line: usize) -> Option<(&PageSlot, u32)> {
        let at = self.slot_on_line(file, line.checked_sub(1)? as u32)?;
        let slot = &self.pages[at];
        Some((slot, (line - 1) as u32 - slot.first_line))
    }

    /// The definitions written on one-based physical `line` of `file`, by
    /// their position among the index's symbols, in the order they are
    /// written.
    pub fn symbols_on_line(
        &self,
        file: usize,
        line: usize,
    ) -> impl Iterator<Item = (usize, DeckSymbol<'_>)> + '_ {
        self.on_line(file, line)
            .into_iter()
            .flat_map(move |(slot, local_line)| {
                let page = &slot.page;
                let line_of = |at: usize| {
                    page.line_of(page.symbols[page.symbol_by_place(at) as usize].name.start)
                };
                let count = page.symbols.len();
                let first = partition(count, |at| line_of(at) < local_line);
                let end = partition(count, |at| line_of(at) <= local_line);
                (first..end).map(move |at| {
                    let local = page.symbol_by_place(at);
                    let symbol = DeckSymbol {
                        deck: self,
                        slot,
                        rec: &page.symbols[local as usize],
                        local,
                    };
                    (symbol.id(), symbol)
                })
            })
    }

    /// The reads written on one-based physical `line` of `file`, by their
    /// position among the index's reads, in source order.
    pub fn references_on_line(
        &self,
        file: usize,
        line: usize,
    ) -> impl Iterator<Item = (usize, DeckReference<'_>)> + '_ {
        self.on_line(file, line)
            .into_iter()
            .flat_map(move |(slot, local_line)| {
                let page = &slot.page;
                let first = page
                    .reads
                    .partition_point(|read| page.line_of(read.name.start) < local_line);
                let end = page
                    .reads
                    .partition_point(|read| page.line_of(read.name.start) <= local_line);
                (first..end).map(move |local| {
                    let read = DeckReference {
                        deck: self,
                        slot,
                        rec: &page.reads[local],
                        local: local as u32,
                    };
                    (read.id(), read)
                })
            })
    }

    /// The places a node is written on one-based physical `line` of `file`,
    /// by their position among the index's nodes, in source order.
    pub fn nodes_on_line(
        &self,
        file: usize,
        line: usize,
    ) -> impl Iterator<Item = (usize, DeckNode<'_>)> + '_ {
        self.on_line(file, line)
            .into_iter()
            .flat_map(move |(slot, local_line)| {
                let page = &slot.page;
                let first = page
                    .nodes
                    .partition_point(|node| page.line_of(node.name.start) < local_line);
                let end = page
                    .nodes
                    .partition_point(|node| page.line_of(node.name.start) <= local_line);
                (first..end).map(move |local| {
                    let node = DeckNode {
                        deck: self,
                        slot,
                        rec: &page.nodes[local],
                        local: local as u32,
                    };
                    (node.id(), node)
                })
            })
    }

    /// The place a node is written over byte `byte` of `file`, the first if
    /// more than one is.
    #[must_use]
    pub fn node_at(&self, file: usize, byte: usize) -> Option<DeckNode<'_>> {
        let slot = &self.pages[self.slot_at(file, byte)?];
        let page = &slot.page;
        let local = (byte - slot.start) as u32;
        let first = page
            .nodes
            .partition_point(|node| node.name.start < local.saturating_sub(page.widest_node));
        let end = page.nodes.partition_point(|node| node.name.start <= local);
        (first..end)
            .find(|at| {
                let name = page.nodes[*at].name;
                name.start <= local && local <= name.end()
            })
            .map(|at| DeckNode {
                deck: self,
                slot,
                rec: &page.nodes[at],
                local: at as u32,
            })
    }

    /// The card written on one-based physical `line` of `file`, continuation
    /// lines included.
    #[must_use]
    pub fn card_on_line(&self, file: usize, line: usize) -> Option<usize> {
        let (slot, local_line) = self.on_line(file, line)?;
        let cards = &slot.page.cards;
        let local = cards
            .partition_point(|card| card.first_line <= local_line)
            .checked_sub(1)?;
        let card = &cards[local];
        (card.last_line >= local_line).then_some(slot.card_base as usize + local)
    }

    /// The name written at `byte` of `file`: the first read written over it,
    /// else the first definition. It is found by a search among the names of
    /// its page written at most the longest name's length before it.
    #[must_use]
    pub fn name_at(&self, file: usize, byte: usize) -> Option<DeckNameAt> {
        let slot = &self.pages[self.slot_at(file, byte)?];
        let page = &slot.page;
        let local = (byte - slot.start) as u32;
        let covers = |name: Span| name.start <= local && local <= name.end();
        let first = page
            .reads
            .partition_point(|read| read.name.start < local.saturating_sub(page.widest_read));
        let end = page.reads.partition_point(|read| read.name.start <= local);
        if let Some(found) = (first..end).find(|at| covers(page.reads[*at].name)) {
            return Some(DeckNameAt::Reference(slot.read_base as usize + found));
        }
        let place = |at: usize| page.symbols[page.symbol_by_place(at) as usize].name.start;
        let count = page.symbols.len();
        let first = partition(count, |at| {
            place(at) < local.saturating_sub(page.widest_symbol)
        });
        let end = partition(count, |at| place(at) <= local);
        (first..end)
            .map(|at| page.symbol_by_place(at))
            .filter(|at| covers(page.symbols[*at as usize].name))
            .min()
            .map(|found| DeckNameAt::Definition(slot.symbol_base as usize + found as usize))
    }

    /// The definition of the name written at `byte` of `file`: the name's own
    /// when it is written there, else the one the read there binds.
    #[must_use]
    pub fn definition_at(&self, file: usize, byte: usize) -> Option<usize> {
        match self.name_at(file, byte)? {
            DeckNameAt::Definition(symbol) => Some(symbol),
            DeckNameAt::Reference(reference) => self.reference(reference)?.target(),
        }
    }

    /// The scope a reader at one-based `line` of `file` is in.
    #[must_use]
    pub fn scope_on_line(&self, file: usize, line: usize) -> usize {
        let Some(card) = self
            .card_on_line(file, line)
            .and_then(|card| self.card(card))
        else {
            return 0;
        };
        // A subcircuit's header reads its defaults inside the subcircuit.
        card.symbols()
            .find(|symbol| symbol.kind() == DeckSymbolKind::Subcircuit)
            .and_then(|symbol| symbol.opens())
            .unwrap_or(card.scope())
    }

    /// `scope` and every scope enclosing it, innermost first.
    pub fn scope_chain(&self, scope: usize) -> impl Iterator<Item = usize> + '_ {
        std::iter::successors(Some(scope), |scope| {
            let parent = self.shared.scopes.get(*scope)?.parent;
            (parent != NONE).then_some(parent as usize)
        })
    }

    /// The definition of kind `kind` a reader in `scope` binds `name` to:
    /// the innermost scope that defines it decides.
    #[must_use]
    pub fn resolve(&self, kind: DeckSymbolKind, name: &str, scope: usize) -> Option<usize> {
        if kind == DeckSymbolKind::Instance {
            return self.scope_chain(scope).find_map(|level| {
                self.symbols()
                    .find(|symbol| {
                        symbol.kind() == DeckSymbolKind::Instance
                            && symbol.scope() == level
                            && symbol.name().eq_ignore_ascii_case(name)
                    })
                    .map(|symbol| symbol.id())
            });
        }
        let selected = self.selected_sections();
        self.scope_chain(scope)
            .find_map(|level| self.bound_in_scope(kind, name, level, &selected))
            .map(|definition| self.definition(definition as usize).id())
    }

    /// The sections the deck's `.lib file section` cards select, in capitals.
    fn selected_sections(&self) -> HashSet<String> {
        self.includes()
            .filter_map(|include| include.section())
            .map(|section| section.to_ascii_uppercase())
            .collect()
    }

    /// The definition, by its position among the index's, of kind `kind`
    /// named `name` that binds in `scope` itself.
    fn bound_in_scope(
        &self,
        kind: DeckSymbolKind,
        name: &str,
        scope: usize,
        selected: &HashSet<String>,
    ) -> Option<u32> {
        let by_name = &self.shared.by_name;
        let order = |at: &u32| {
            let symbol = self.definition(*at as usize);
            (symbol.kind(), symbol.scope())
                .cmp(&(kind, scope))
                .then_with(|| by_case(&symbol.name(), name))
        };
        let first = by_name.partition_point(|at| order(at).is_lt());
        let end = first + by_name[first..].partition_point(|at| order(at).is_eq());
        binding(kind, &by_name[first..end], |definition| {
            self.definition(definition as usize)
                .section()
                .is_none_or(|section| selected.contains(&section.to_ascii_uppercase()))
        })
    }

    /// The parameters a reader in `scope` sees, one per name: each the
    /// definition the engine binds there.
    #[must_use]
    pub fn parameters_visible_in(&self, scope: usize) -> Vec<usize> {
        let mut seen = HashSet::new();
        let mut visible = Vec::new();
        let selected = self.selected_sections();
        for level in self.scope_chain(scope) {
            let names = self
                .definitions()
                .filter(|symbol| {
                    symbol.kind() == DeckSymbolKind::Parameter && symbol.scope() == level
                })
                .map(|symbol| symbol.name().to_ascii_uppercase())
                .collect::<Vec<_>>();
            for name in names {
                if seen.insert(name.clone())
                    && let Some(definition) =
                        self.bound_in_scope(DeckSymbolKind::Parameter, &name, level, &selected)
                {
                    visible.push(self.definition(definition as usize).id());
                }
            }
        }
        visible
    }

    /// The tokens of `card` as the engine's lexer reads them. A title or a
    /// `.control` block has none.
    #[must_use]
    pub fn card_tokens(&self, card: usize) -> Vec<DeckToken> {
        self.card(card)
            .map_or_else(Vec::new, |card| self.tokens_of(&card))
    }

    fn tokens_of(&self, card: &DeckCard<'_>) -> Vec<DeckToken> {
        if matches!(card.kind(), DeckCardKind::Title | DeckCardKind::Control) {
            return Vec::new();
        }
        let page = &card.slot.page;
        let bytes = card.bytes();
        let text = self.texts[card.slot.file as usize].slice(bytes.clone());
        let mut tokens = Vec::new();
        let mut lines = LogicalLines::default();
        let mut offset = 0usize;
        let mut each = |logical: &MappedLine| {
            if tokens.is_empty() {
                tokens = lex_card(logical, bytes.start);
            }
        };
        for (at, physical) in text.split_inclusive('\n').enumerate() {
            let line = card.rec.first_line + at as u32;
            let raw = physical.strip_suffix('\n').unwrap_or(physical);
            let raw = raw.strip_suffix('\r').unwrap_or(raw);
            let code = page
                .comment_on(line)
                .and_then(|at| raw.get(..at))
                .unwrap_or(raw);
            lines.line(code, offset, at + 1, page.spectre_on(line), &mut each);
            offset += physical.len();
        }
        lines.finish(&mut each);
        tokens
    }

    /// Whether a node named `name` is ground or a `.global` node.
    fn is_global(&self, name: &str) -> bool {
        if name == "0" || name.eq_ignore_ascii_case("gnd") {
            return true;
        }
        let global = |&(slot, at): &(u32, u32)| {
            let slot = &self.pages[slot as usize];
            self.text_at(slot, slot.page.globals[at as usize])
        };
        self.shared
            .globals
            .binary_search_by(|found| by_case(&global(found), name))
            .is_ok()
    }
}

/// The first position of `0..count` at which `before` stops holding.
fn partition(count: usize, before: impl Fn(usize) -> bool) -> usize {
    let (mut low, mut high) = (0, count);
    while low < high {
        let middle = low + (high - low) / 2;
        if before(middle) {
            low = middle + 1;
        } else {
            high = middle;
        }
    }
    low
}

/// `left` against `right`, ignoring ASCII case.
fn by_case(left: &str, right: &str) -> std::cmp::Ordering {
    left.bytes()
        .map(|byte| byte.to_ascii_lowercase())
        .cmp(right.bytes().map(|byte| byte.to_ascii_lowercase()))
}

/// The definition among `candidates`, all of one name in one scope in source
/// order, that binds it: for a parameter or function the one the engine's
/// redefinition policy selects, for a model or subcircuit the first in a
/// library section a `.lib` card selects (or in none), else the first.
fn binding(
    kind: DeckSymbolKind,
    candidates: &[u32],
    selected: impl Fn(u32) -> bool,
) -> Option<u32> {
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
            .find(|candidate| selected(*candidate))
            .or_else(|| candidates.first().copied()),
        DeckSymbolKind::Instance => candidates.first().copied(),
    }
}

/// The tokens of one logical card, each with its range of the file from
/// `base`, the first byte of the text the card was joined from.
fn lex_card(line: &MappedLine, base: usize) -> Vec<DeckToken> {
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
            let span = line.original_span(span)?;
            Some(DeckToken {
                kind,
                span: base + span.start..base + span.end,
            })
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

// ---------------------------------------------------------------------------
// Joining physical lines into cards.

/// The engine's joining of physical lines into logical cards, a line at a
/// time, into one card buffer that is reused: a `+` line continues the card
/// before it, a blank or comment line ends it, and in Spectre a line ending
/// in `\` continues onto the next.
#[derive(Default)]
struct LogicalLines {
    card: MappedLine,
    open: bool,
    continued_by_backslash: bool,
}

impl LogicalLines {
    /// Read the code of one physical line, `offset` its first byte and
    /// `line` its one-based number, a trailing `\` joining the next line
    /// when `backslash_joins`. `each` gets the card the line ends.
    fn line(
        &mut self,
        code: &str,
        offset: usize,
        line: usize,
        backslash_joins: bool,
        each: &mut impl FnMut(&MappedLine),
    ) {
        let trimmed = code.trim_start();
        let lead = code.len() - trimmed.len();
        let continues_card = std::mem::take(&mut self.continued_by_backslash);
        if trimmed.is_empty() || trimmed.starts_with('*') {
            self.finish(each);
            return;
        }
        let trimmed = match trimmed.trim_end().strip_suffix('\\') {
            Some(body) if backslash_joins => {
                self.continued_by_backslash = true;
                body.trim_end()
            }
            _ => trimmed,
        };
        if continues_card && self.open {
            self.card.push_join_space();
            self.card.push_segment(trimmed, offset + lead, line);
        } else if let Some(rest) = trimmed.strip_prefix('+') {
            if self.open {
                let continued = rest.trim_start();
                let continued_lead = rest.len() - continued.len();
                self.card.push_join_space();
                self.card
                    .push_segment(continued, offset + lead + 1 + continued_lead, line);
            }
        } else {
            self.finish(each);
            self.card.clear();
            self.card.push_segment(trimmed, offset + lead, line);
            self.open = true;
        }
    }

    /// Hand `each` the card being joined, if one is.
    fn finish(&mut self, each: &mut impl FnMut(&MappedLine)) {
        if std::mem::take(&mut self.open) {
            each(&self.card);
        }
    }
}

/// Whether the code of a physical line starts a card, rather than continue
/// one or hold none.
fn starts_card(code: &str) -> bool {
    let trimmed = code.trim_start();
    !(trimmed.is_empty() || trimmed.starts_with(['*', '+']))
}

/// Whether a line reads as Spectre, from `lines_spectre`, and the switch a
/// `simulator lang=` line makes for the lines after it.
fn spectre_of_line(code: &str, lines_spectre: &mut bool) -> bool {
    let read_as_spectre = *lines_spectre;
    let mut words = code.split_whitespace();
    if words
        .next()
        .is_some_and(|head| head.eq_ignore_ascii_case("simulator"))
    {
        for word in words {
            if word.eq_ignore_ascii_case("lang=spectre") {
                *lines_spectre = true;
            } else if word.eq_ignore_ascii_case("lang=spice") {
                *lines_spectre = false;
            }
        }
    }
    read_as_spectre
}

/// Hand `each` the physical lines of `text` from byte `from` to byte `to`,
/// each with its break and the byte it starts at, a line held in two pieces
/// put in one.
fn for_each_line(text: &dyn DeckText, from: usize, to: usize, mut each: impl FnMut(usize, &str)) {
    let mut held = String::new();
    let mut at = from;
    let mut line_start = from;
    for piece in text.pieces(from) {
        let piece = &piece[..piece.len().min(to.saturating_sub(at))];
        at += piece.len();
        let mut rest = piece;
        while let Some(newline) = rest.find('\n') {
            let (line, after) = rest.split_at(newline + 1);
            if held.is_empty() {
                each(line_start, line);
                line_start += line.len();
            } else {
                held.push_str(line);
                each(line_start, &held);
                line_start += held.len();
                held.clear();
            }
            rest = after;
        }
        held.push_str(rest);
        if at >= to {
            break;
        }
    }
    if !held.is_empty() {
        each(line_start, &held);
    }
}

// ---------------------------------------------------------------------------
// Building the index.

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
    name: Span,
    scope: u32,
    card: u32,
    /// The instance symbol this read names the subcircuit or model of.
    instance: Option<u32>,
}

/// A transistor card, whose node count the model it names decides.
struct PendingTransistor {
    card: u32,
    scope: u32,
    instance: u32,
    /// The positional fields after the name, and whether each is a number.
    positional: Vec<(Span, bool)>,
    /// How many of them are nodes at least.
    min_nodes: usize,
}

/// A page read, and what its reading leaves for the names the whole deck
/// defines to settle.
#[derive(Default)]
struct PageReading {
    page: Page,
    reads: Vec<PendingRead>,
    transistors: Vec<PendingTransistor>,
}

/// Where the scopes and sections a page opens come from: new ones, numbered
/// on, or the ones it opened when it was read before.
enum Openings<'a> {
    New {
        scopes: &'a mut Vec<ScopeRec>,
        sections: &'a mut Vec<(u32, u32)>,
    },
    Again {
        scopes: std::slice::Iter<'a, u32>,
        sections: std::slice::Iter<'a, u32>,
        parents: &'a [ScopeRec],
    },
}

/// A file's lines read into pages, with what the lines before carry.
struct FileReader<'a> {
    text: &'a dyn DeckText,
    titled: bool,
    carry: Carry,
    /// The page being read, and the byte of the file it starts at.
    reading: PageReading,
    start: usize,
    /// The `.control` card a block being read belongs to.
    control: Option<u32>,
    lines: LogicalLines,
    ranges: Vec<Range<usize>>,
    /// The subcircuits the page opened scopes with: each scope, and the
    /// symbol that opens it.
    openers: Vec<(u32, u32)>,
}

impl<'a> FileReader<'a> {
    fn new(text: &'a dyn DeckText, start: usize, carry: Carry, titled: bool) -> Self {
        Self {
            text,
            titled,
            reading: PageReading {
                page: Page {
                    entry: carry.clone(),
                    titled,
                    ..Page::default()
                },
                ..PageReading::default()
            },
            carry,
            start,
            control: None,
            lines: LogicalLines::default(),
            ranges: Vec::new(),
            openers: Vec::new(),
        }
    }

    /// What the lines read so far leave the next line, with the comment rule
    /// as it stood before the line being read: `comments`.
    fn boundary(&self, comments: SourceComments) -> Carry {
        Carry {
            comments,
            control: self.control.is_some(),
            continued: self.lines.continued_by_backslash,
            ..self.carry.clone()
        }
    }

    /// Close the page being read before byte `end`, the first of the next,
    /// and hand it to `done`.
    fn close(
        &mut self,
        end: usize,
        exit: Carry,
        done: &mut dyn FnMut(PageReading, usize, Vec<(u32, u32)>),
    ) {
        let next = PageReading {
            page: Page {
                entry: exit.clone(),
                ..Page::default()
            },
            ..PageReading::default()
        };
        let mut full = std::mem::replace(&mut self.reading, next);
        let lines = full.page.line_starts.len();
        if !full.page.spectre_lines.is_empty() {
            full.page.spectre_lines.resize(lines, false);
        }
        full.page.exit = exit;
        full.page.bytes = (end - self.start) as u32;
        full.page.shrink();
        done(full, self.start, std::mem::take(&mut self.openers));
        self.start = end;
    }

    /// Read the file's lines `from..to`; with `cut`, a page is closed at the
    /// first card that starts [`PAGE_BYTES`] after it opened.
    fn read(
        &mut self,
        from: usize,
        to: usize,
        openings: &mut Openings<'_>,
        cut: bool,
        done: &mut dyn FnMut(PageReading, usize, Vec<(u32, u32)>),
    ) {
        let text = self.text;
        for_each_line(text, from, to, |line_start, physical| {
            let raw = physical.strip_suffix('\n').unwrap_or(physical);
            let raw = raw.strip_suffix('\r').unwrap_or(raw);
            let is_title = self.titled && line_start == 0;
            let comments_before = self.carry.comments;
            let comment = if is_title {
                (!raw.is_empty()).then_some(0)
            } else {
                self.carry
                    .comments
                    .line(raw, spice_comment_start, &mut self.ranges);
                self.ranges.iter().map(|range| range.start).min()
            };
            let code = comment.and_then(|at| raw.get(..at)).unwrap_or(raw);
            if cut
                && !is_title
                && line_start - self.start >= PAGE_BYTES
                && starts_card(code)
                && !self.lines.continued_by_backslash
                && self.control.is_none()
            {
                // The card before the line is the page's last.
                self.end_card(openings);
                if self.control.is_none() {
                    let exit = self.boundary(comments_before);
                    self.close(line_start, exit, done);
                }
            }
            let page_line = self.reading.page.line_starts.len() as u32;
            self.reading
                .page
                .line_starts
                .push((line_start - self.start) as u32);
            if let Some(at) = comment {
                self.reading.page.comments.push((page_line, at as u32));
            }
            if spectre_of_line(code, &mut self.carry.lines_spectre) {
                let lines = &mut self.reading.page.spectre_lines;
                lines.resize(page_line as usize, false);
                lines.push(true);
            }
            if is_title {
                self.reading.page.cards.push(CardRec {
                    kind: DeckCardKind::Title,
                    head: Span {
                        start: 0,
                        len: raw.len() as u32,
                    },
                    first_line: 0,
                    last_line: 0,
                    scope: 0,
                    section: NONE,
                });
            } else {
                let offset = line_start - self.start;
                let spectre = self.reading.page.spectre_on(page_line);
                let mut lines = std::mem::take(&mut self.lines);
                lines.line(
                    code,
                    offset,
                    page_line as usize + 1,
                    spectre,
                    &mut |logical: &MappedLine| self.read_card(logical, openings),
                );
                self.lines = lines;
            }
        });
        self.end_card(openings);
        let exit = self.boundary(self.carry.comments);
        self.close(to, exit, done);
    }

    /// Read the card being joined, if one is.
    fn end_card(&mut self, openings: &mut Openings<'_>) {
        let mut lines = std::mem::take(&mut self.lines);
        lines.finish(&mut |logical: &MappedLine| self.read_card(logical, openings));
        self.lines = lines;
    }

    fn scope(&self) -> u32 {
        self.carry.scopes.last().copied().unwrap_or(0)
    }

    fn push_symbol(&mut self, symbol: SymbolRec) -> u32 {
        let page = &mut self.reading.page;
        if symbol.kind != DeckSymbolKind::Instance {
            page.definitions.push(page.symbols.len() as u32);
        }
        page.symbols.push(symbol);
        page.symbols.len() as u32 - 1
    }

    /// A definition written on the page, in the library section being read.
    const fn symbol(&self, kind: DeckSymbolKind, name: Span, scope: u32, card: u32) -> SymbolRec {
        SymbolRec {
            kind,
            detail_kind: DetailKind::Empty,
            name,
            card,
            scope,
            section: self.carry.section,
            detail: Span { start: 0, len: 0 },
            opens: NONE,
            target: NONE,
        }
    }

    fn push_node(&mut self, name: Span, scope: u32, card: u32) {
        self.reading.page.nodes.push(NodeRec { name, card, scope });
    }

    fn push_read(&mut self, kind: ReadKind, name: Span, scope: u32, card: u32) {
        self.reading.reads.push(PendingRead {
            kind,
            name,
            scope,
            card,
            instance: None,
        });
    }

    fn read_card(&mut self, line: &MappedLine, openings: &mut Openings<'_>) {
        if self.carry.ended {
            return;
        }
        let (first_line, last_line) = line.physical_lines();
        let fields = card_fields(&line.text);
        let Some(head_range) = fields.first().cloned() else {
            return;
        };
        let head_lower = line.text[head_range.clone()].to_ascii_lowercase();

        if let Some(control) = self.control {
            self.reading.page.cards[control as usize].last_line = last_line as u32 - 1;
            if head_lower == ".endc" {
                self.control = None;
            }
            return;
        }

        let kind = if head_lower == "simulator" {
            // `simulator lang=spectre` hands the lines after it to Spectre,
            // `simulator lang=spice` hands them back.
            for field in &fields[1..] {
                match line.text[field.clone()].to_ascii_lowercase().as_str() {
                    "lang=spectre" => self.carry.spectre = true,
                    "lang=spice" => self.carry.spectre = false,
                    _ => {}
                }
            }
            DeckCardKind::Directive
        } else if self.carry.spectre {
            let inside_block = self.carry.braces > 0;
            let opened = line.text.matches('{').count() as u32;
            let closed = line.text.matches('}').count() as u32;
            self.carry.braces = (self.carry.braces + opened).saturating_sub(closed);
            if inside_block || opened > 0 {
                DeckCardKind::Directive
            } else {
                spectre_card_kind(&head_lower, &fields, &line.text)
            }
        } else {
            card_kind(&head_lower, &fields, &line.text)
        };
        let card = self.reading.page.cards.len() as u32;
        let Some(head_span) = line.original_span(head_range) else {
            return;
        };
        let scope = self.scope();
        self.reading.page.cards.push(CardRec {
            kind,
            head: Span::of(&head_span),
            first_line: first_line as u32 - 1,
            last_line: last_line as u32 - 1,
            scope,
            section: self.carry.section,
        });
        let field_span = |index: usize| {
            fields
                .get(index)
                .and_then(|range| line.original_span(range.clone()))
                .map(|span| Span::of(&span))
        };

        match kind {
            DeckCardKind::Control => self.control = Some(card),
            DeckCardKind::End => self.carry.ended = true,
            DeckCardKind::LibrarySection => {
                let name = fields
                    .get(1)
                    .map(|range| unquoted(&line.text, range.clone()))
                    .filter(|range| !range.is_empty())
                    .and_then(|range| line.original_span(range));
                self.carry.section = match name {
                    Some(name) => {
                        let page = &mut self.reading.page;
                        let opener = page.section_openers.len() as u32;
                        page.section_openers.push((card, Span::of(&name)));
                        let id = match openings {
                            Openings::New { sections, .. } => {
                                sections.push((0, opener));
                                sections.len() as u32 - 1
                            }
                            Openings::Again { sections, .. } => {
                                sections.next().copied().unwrap_or(NONE)
                            }
                        };
                        page.opened_sections.push(id);
                        id
                    }
                    None => NONE,
                };
            }
            DeckCardKind::EndLibrarySection => self.carry.section = NONE,
            DeckCardKind::Include | DeckCardKind::Library => {
                let arguments = quoted_arguments(&line.text, &fields[1..]);
                let span_of = |range: &Range<usize>| {
                    let range = unquoted(&line.text, range.clone());
                    if range.is_empty() {
                        line.original_span(range.start..range.start + 1)
                            .map(|span| Span {
                                start: span.start as u32,
                                len: 0,
                            })
                    } else {
                        line.original_span(range).map(|span| Span::of(&span))
                    }
                };
                if let Some(path) = arguments.first()
                    && let Some(written) = line.original_span(path.clone())
                    && let Some(path) = span_of(path)
                {
                    let section = arguments.get(1).and_then(span_of);
                    self.reading.page.includes.push(IncludeRec {
                        kind: if kind == DeckCardKind::Library {
                            DeckIncludeKind::Library
                        } else {
                            DeckIncludeKind::Include
                        },
                        written: Span::of(&written),
                        path,
                        section,
                        card,
                    });
                }
            }
            DeckCardKind::Global => {
                for index in 1..fields.len() {
                    if let Some(name) = field_span(index) {
                        self.reading.page.globals.push(name);
                        self.push_node(name, 0, card);
                    }
                }
                if head_lower == ".global_param" {
                    self.read_parameters(line, card, 0, true);
                }
            }
            DeckCardKind::Parameter => self.read_parameters(line, card, scope, true),
            DeckCardKind::Function => self.read_function(line, card, scope),
            DeckCardKind::Model => {
                if let Some(name) = field_span(1) {
                    let mut symbol = self.symbol(DeckSymbolKind::Model, name, scope, card);
                    if let Some(range) = fields.get(2) {
                        let written = &line.text[range.clone()];
                        let length = written.split('(').next().unwrap_or(written).len();
                        if length > 0
                            && let Some(span) =
                                line.original_span(range.start..range.start + length)
                        {
                            symbol.detail_kind = DetailKind::Written;
                            symbol.detail = Span::of(&span);
                        }
                    }
                    self.push_symbol(symbol);
                }
                self.read_parameters(line, card, scope, false);
            }
            // Spectre's `inline subckt name` names the subcircuit a field later.
            DeckCardKind::Subcircuit if head_lower == "inline" => {
                self.read_subcircuit(line, &fields[1..], card, openings);
            }
            DeckCardKind::Subcircuit => self.read_subcircuit(line, &fields, card, openings),
            DeckCardKind::EndSubcircuit => {
                if let Some(inner) = self.carry.scopes.pop() {
                    if let Some(name) = field_span(1) {
                        let parents = match openings {
                            Openings::New { scopes, .. } => &**scopes,
                            Openings::Again { parents, .. } => *parents,
                        };
                        let parent = parents
                            .get(inner as usize)
                            .map_or(NONE, |scope| scope.parent);
                        self.push_read(
                            ReadKind::Known(DeckSymbolKind::Subcircuit),
                            name,
                            if parent == NONE { 0 } else { parent },
                            card,
                        );
                    }
                    self.reading.page.cards[card as usize].scope = inner;
                }
            }
            DeckCardKind::Instance if self.carry.spectre => {
                self.read_spectre_instance(line, &fields, card, scope);
            }
            DeckCardKind::Instance => self.read_instance(line, &fields, card, scope),
            DeckCardKind::Output | DeckCardKind::Measurement | DeckCardKind::Analysis => {
                self.read_parameters(line, card, scope, false);
                self.read_probes(line, card, scope);
            }
            _ => self.read_parameters(line, card, scope, false),
        }
    }

    /// The nodes the `V()` probes of a card name outside its values:
    /// `.print tran v(out)`, `.meas ac peak max v(out)`, `.tf v(out,ref) vin`.
    /// A probe inside a braced or quoted value is read with the value.
    fn read_probes(&mut self, line: &MappedLine, card: u32, scope: u32) {
        if !line.text.contains('(') {
            return;
        }
        let tokens = Lexer::new(&line.text)
            .tokenize_recovering()
            .into_iter()
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        let mut index = 0;
        while index < tokens.len() {
            let token = &tokens[index];
            index += 1;
            let TokenKind::Ident(_) = token.kind else {
                continue;
            };
            if !token.lexeme.eq_ignore_ascii_case("v")
                || !tokens
                    .get(index)
                    .is_some_and(|next| matches!(next.kind, TokenKind::LParen))
            {
                continue;
            }
            let (probed, after) = probe_arguments(&tokens, index);
            index = after;
            for span in probed {
                if let Some(span) = line.original_span(span) {
                    self.push_node(Span::of(&span), scope, card);
                }
            }
        }
    }

    /// Every `name=value` on the card and every braced or quoted value: a
    /// name a parameter card assigns is defined in `scope`, and every name
    /// a value reads is a read in `scope`.
    fn read_parameters(&mut self, line: &MappedLine, card: u32, scope: u32, defines: bool) {
        for value in logical_values(&line.text) {
            if defines
                && let Some(name) = value.name.clone()
                && let Some(span) = line.original_span(name)
            {
                let mut symbol =
                    self.symbol(DeckSymbolKind::Parameter, Span::of(&span), scope, card);
                if let Some(detail) = line.original_span(value.token.clone()) {
                    symbol.detail_kind = DetailKind::Written;
                    symbol.detail = Span::of(&detail);
                }
                self.push_symbol(symbol);
            }
            self.read_value(line, value.body, card, scope, &[]);
        }
    }

    /// The names an expression reads: a name the lexer reads as a number is
    /// none, a name called like a function is a read of a `.func` when one
    /// is defined, and the arguments of a `V()` or `I()` probe are nodes and
    /// devices, not parameters. `formals` are names the expression binds
    /// itself.
    fn read_value(
        &mut self,
        line: &MappedLine,
        body: Range<usize>,
        card: u32,
        scope: u32,
        formals: &[&str],
    ) {
        let tokens = Lexer::new(&line.text[body.clone()])
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
                    self.read_value(line, inner, card, scope, formals);
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
                let (probed, after) = probe_arguments(&tokens, index);
                index = after;
                if name.eq_ignore_ascii_case("v") {
                    for span in probed {
                        let span = body.start + span.start..body.start + span.end;
                        if let Some(span) = line.original_span(span) {
                            self.push_node(Span::of(&span), scope, card);
                        }
                    }
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
            self.push_read(kind, Span::of(&span), scope, card);
        }
    }

    fn read_function(&mut self, line: &MappedLine, card: u32, scope: u32) {
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
            .collect::<Vec<_>>();
        if let Some(span) = line.original_span(name_range) {
            let mut symbol = self.symbol(DeckSymbolKind::Function, Span::of(&span), scope, card);
            symbol.detail_kind = DetailKind::Formals;
            symbol.detail = line
                .original_span(open + 1..close)
                .map_or(Span { start: 0, len: 0 }, |span| Span::of(&span));
            self.push_symbol(symbol);
        }
        for value in logical_values(text)
            .into_iter()
            .filter(|value| value.body.start > close)
        {
            self.read_value(line, value.body, card, scope, &formals);
        }
    }

    fn read_subcircuit(
        &mut self,
        line: &MappedLine,
        fields: &[Range<usize>],
        card: u32,
        openings: &mut Openings<'_>,
    ) {
        let parent = self.scope();
        let Some(span) = fields
            .get(1)
            .and_then(|name| line.original_span(name.clone()))
        else {
            return;
        };
        // Spectre writes the ports in parentheses: `subckt amp (in out)`.
        let ports = fields
            .iter()
            .skip(2)
            .take_while(|field| !is_param_boundary(&line.text[(*field).clone()]))
            .filter_map(|field| without_parentheses(&line.text, field.clone()))
            .collect::<Vec<_>>();
        let mut symbol = self.symbol(DeckSymbolKind::Subcircuit, Span::of(&span), parent, card);
        symbol.detail_kind = DetailKind::Ports;
        symbol.detail = Span {
            start: 0,
            len: ports.len() as u32,
        };
        let local = self.push_symbol(symbol);
        let scope = match openings {
            Openings::New { scopes, .. } => {
                scopes.push(ScopeRec {
                    parent,
                    opener: NONE,
                });
                scopes.len() as u32 - 1
            }
            Openings::Again { scopes, .. } => scopes.next().copied().unwrap_or(NONE),
        };
        self.reading.page.opened_scopes.push(scope);
        self.openers.push((scope, local));
        self.reading.page.symbols[local as usize].opens = scope;
        self.carry.scopes.push(scope);
        for port in ports {
            if let Some(span) = line.original_span(port) {
                self.push_node(Span::of(&span), scope, card);
            }
        }
        // A formal and its default belong to the subcircuit the card opens.
        self.read_parameters(line, card, scope, true);
    }

    /// A Spectre instance, `name (nodes) master name=value ...`: its nodes,
    /// and its master read as a subcircuit or model when the deck defines
    /// one of that name. The parentheses may be left out, and the master is
    /// then the last field before the first assignment.
    fn read_spectre_instance(
        &mut self,
        line: &MappedLine,
        fields: &[Range<usize>],
        card: u32,
        scope: u32,
    ) {
        let text = &line.text;
        let head = self.reading.page.cards[card as usize].head;
        let instance = self.push_symbol(self.symbol(DeckSymbolKind::Instance, head, scope, card));
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
                && let Some(span) = line.original_span(node)
            {
                self.push_node(Span::of(&span), scope, card);
            }
        }
        if let Some(field) = master.and_then(|at| positional.get(at))
            && let Some(span) = line.original_span(field.clone())
        {
            let span = Span::of(&span);
            let symbol = &mut self.reading.page.symbols[instance as usize];
            symbol.detail_kind = DetailKind::Written;
            symbol.detail = span;
            self.reading.reads.push(PendingRead {
                kind: ReadKind::Master,
                name: span,
                scope,
                card,
                instance: Some(instance),
            });
        }
        self.read_parameters(line, card, scope, false);
    }

    fn read_instance(&mut self, line: &MappedLine, fields: &[Range<usize>], card: u32, scope: u32) {
        let text = &line.text;
        let head = &text[fields[0].clone()];
        let head_span = self.reading.page.cards[card as usize].head;
        let instance =
            self.push_symbol(self.symbol(DeckSymbolKind::Instance, head_span, scope, card));
        let positional = fields
            .iter()
            .skip(1)
            .take_while(|field| !is_param_boundary(&text[(*field).clone()]))
            .filter_map(|field| {
                let span = line.original_span(field.clone())?;
                let written = &text[field.clone()];
                Some((written, Span::of(&span), is_number(written)))
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
                self.reading.transistors.push(PendingTransistor {
                    card,
                    scope,
                    instance,
                    positional: positional
                        .iter()
                        .map(|(_, span, number)| (*span, *number))
                        .collect(),
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
                } else if let Some((_, span, _)) = positional.get(2) {
                    let symbol = &mut self.reading.page.symbols[instance as usize];
                    symbol.detail_kind = DetailKind::Written;
                    symbol.detail = *span;
                }
            }
            'V' | 'I' | 'B' => nodes = 2,
            'E' | 'G' => {
                nodes = if positional.len() >= 5 && !is_controlled_source_keyword(positional[2].0) {
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
                    for (written, span, _) in &positional[..model] {
                        let trimmed = written.trim_start_matches(['[', ']', '~']);
                        let lead = (written.len() - trimmed.len()) as u32;
                        let node = trimmed.trim_end_matches(['[', ']', '~']);
                        if !node.is_empty() && is_port(written) {
                            self.push_node(
                                Span {
                                    start: span.start + lead,
                                    len: node.len() as u32,
                                },
                                scope,
                                card,
                            );
                        }
                    }
                }
            }
            _ => {}
        }
        for (_, span, _) in positional.iter().take(nodes.min(positional.len())) {
            self.push_node(*span, scope, card);
        }
        for at in instances {
            let (_, span, _) = positional[at];
            self.push_read(ReadKind::Known(DeckSymbolKind::Instance), span, scope, card);
        }
        if let Some((kind, at)) = target {
            let (_, span, _) = positional[at];
            let symbol = &mut self.reading.page.symbols[instance as usize];
            symbol.detail_kind = DetailKind::Written;
            symbol.detail = span;
            self.reading.reads.push(PendingRead {
                kind,
                name: span,
                scope,
                card,
                instance: Some(instance),
            });
        }
        // Values: `w={w0}`, `{r0*2}`, `sin(0 {amp} 1k)`.
        self.read_parameters(line, card, scope, false);
    }
}

/// The names a probe's parentheses hold, each by its span in the lexed text,
/// and the token after its closing parenthesis. `open` is the token of the
/// opening one.
fn probe_arguments(tokens: &[super::lexer::Token], open: usize) -> (Vec<Range<usize>>, usize) {
    let mut names = Vec::new();
    let mut depth = 0usize;
    let mut index = open;
    while let Some(token) = tokens.get(index) {
        index += 1;
        match token.kind {
            TokenKind::LParen => depth += 1,
            TokenKind::RParen => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    break;
                }
            }
            TokenKind::Ident(_) | TokenKind::Number(_) if depth == 1 => {
                names.push(token.span.start..token.span.end);
            }
            _ => {}
        }
    }
    (names, index)
}

struct Builder {
    texts: Vec<SharedDeckText>,
    files: Vec<String>,
    /// Each page read: its file, the byte it starts at, and the reading.
    readings: Vec<(u32, usize, PageReading)>,
    scopes: Vec<ScopeRec>,
    sections: Vec<(u32, u32)>,
    /// Each scope a subcircuit opens, with the slot of the page and the
    /// place among the page's definitions of the subcircuit.
    scope_openers: Vec<(u32, u32, u32)>,
    file_pages: Vec<u32>,
}

impl Builder {
    fn new() -> Self {
        Self {
            texts: Vec::new(),
            files: Vec::new(),
            readings: Vec::new(),
            scopes: vec![ScopeRec {
                parent: NONE,
                opener: NONE,
            }],
            sections: Vec::new(),
            scope_openers: Vec::new(),
            file_pages: vec![0],
        }
    }

    fn read_file(&mut self, file: usize, source: &DeckIndexSource<'_>) {
        self.files.push(source.identity.to_owned());
        self.texts.push(Arc::clone(&source.text));
        let text = &*source.text;
        let titled = source.kind == SourceKind::DeckRoot && !text.is_empty();
        let first_slot = self.readings.len() as u32;
        let mut pages = Vec::new();
        {
            let mut openings = Openings::New {
                scopes: &mut self.scopes,
                sections: &mut self.sections,
            };
            let carry = Carry::at_start(source.kind == SourceKind::Spectre);
            let mut reader = FileReader::new(text, 0, carry, titled);
            reader.read(
                0,
                text.len(),
                &mut openings,
                true,
                &mut |reading, start, openers| pages.push((reading, start, openers)),
            );
        }
        for (offset, (reading, start, openers)) in pages.into_iter().enumerate() {
            let slot = first_slot + offset as u32;
            for (scope, local) in openers {
                if let Ok(ordinal) = reading.page.definitions.binary_search(&local) {
                    self.scope_openers.push((scope, slot, ordinal as u32));
                }
            }
            for id in &reading.page.opened_sections {
                if let Some(section) = self.sections.get_mut(*id as usize) {
                    section.0 = slot;
                }
            }
            self.readings.push((file as u32, start, reading));
        }
        self.file_pages.push(self.readings.len() as u32);
    }

    fn finish(self) -> DeckIndex {
        let mut pages = Vec::with_capacity(self.readings.len());
        let mut pending = Vec::with_capacity(self.readings.len());
        for (file, start, reading) in self.readings {
            pages.push(PageSlot {
                page: Arc::new(reading.page),
                file,
                start,
                first_line: 0,
                card_base: 0,
                symbol_base: 0,
                read_base: 0,
                node_base: 0,
            });
            pending.push((reading.reads, reading.transistors));
        }
        let mut definitions = Vec::new();
        for (slot, page) in pages.iter().enumerate() {
            definitions.extend((0..page.page.definitions.len() as u32).map(|at| (slot as u32, at)));
        }
        let mut scopes = self.scopes;
        for (scope, slot, ordinal) in self.scope_openers {
            if let (Some(rec), Ok(at)) = (
                scopes.get_mut(scope as usize),
                definitions.binary_search(&(slot, ordinal)),
            ) {
                rec.opener = at as u32;
            }
        }
        let mut index = DeckIndex {
            texts: self.texts,
            pages,
            file_pages: self.file_pages,
            shared: Arc::new(Shared {
                files: self.files,
                scopes,
                sections: self.sections,
                definitions,
                by_name: Vec::new(),
                globals: Vec::new(),
            }),
        };
        index.place_pages();
        index.order_shared();
        let resolver = Resolver::new(&index, pending.iter().map(|(reads, _)| reads));
        for (at, (reads, transistors)) in pending.into_iter().enumerate() {
            let settled = resolver.settle(&index, at, reads, transistors);
            settled.apply(Arc::get_mut(&mut index.pages[at].page).expect("held only here"));
        }
        index.place_pages();
        index
    }
}

/// What the reads of a page are resolved with: the sections the deck's
/// `.lib` cards select, and the first instance of each name the reads of an
/// instance name, in each scope.
struct Resolver {
    selected: HashSet<String>,
    instances: HashMap<(u32, String), (u32, u32)>,
}

/// What resolving a page's reads gives it.
struct Settled {
    reads: Vec<ReadRec>,
    nodes: Vec<NodeRec>,
    /// The instances given a target, and a transistor the model field it
    /// names.
    instances: Vec<(u32, u32, Option<Span>)>,
}

impl Resolver {
    /// The resolver of the reads `pending`, one list to a slot of `index`.
    fn new<'a>(index: &DeckIndex, pending: impl Iterator<Item = &'a Vec<PendingRead>>) -> Self {
        let mut wanted = HashSet::new();
        for (at, reads) in pending.enumerate() {
            let slot = &index.pages[at];
            for read in reads {
                if read.kind == ReadKind::Known(DeckSymbolKind::Instance) {
                    wanted.insert(index.text_at(slot, read.name).to_ascii_uppercase());
                }
            }
        }
        let mut instances = HashMap::new();
        if !wanted.is_empty() {
            let lengths = wanted.iter().map(String::len).collect::<HashSet<_>>();
            for (at, slot) in index.pages.iter().enumerate() {
                for (local, symbol) in slot.page.symbols.iter().enumerate() {
                    if symbol.kind != DeckSymbolKind::Instance
                        || !lengths.contains(&(symbol.name.len as usize))
                    {
                        continue;
                    }
                    let name = index.text_at(slot, symbol.name).to_ascii_uppercase();
                    if wanted.contains(&name) {
                        instances
                            .entry((symbol.scope, name))
                            .or_insert((at as u32, local as u32));
                    }
                }
            }
        }
        Self {
            selected: index.selected_sections(),
            instances,
        }
    }

    fn resolve(&self, index: &DeckIndex, kind: DeckSymbolKind, name: &str, scope: u32) -> u32 {
        index
            .scope_chain(scope as usize)
            .find_map(|level| index.bound_in_scope(kind, name, level, &self.selected))
            .unwrap_or(NONE)
    }

    /// Resolve what the page in slot `at` of `index` left pending: each
    /// read's definition, each transistor's nodes and model, and the scope
    /// of each node named as a global one.
    fn settle(
        &self,
        index: &DeckIndex,
        at: usize,
        reads: Vec<PendingRead>,
        transistors: Vec<PendingTransistor>,
    ) -> Settled {
        let slot = &index.pages[at];
        let page = &slot.page;
        let name = |span: Span| index.text_at(slot, span);
        let mut settled = Vec::with_capacity(reads.len());
        let mut instances = Vec::new();
        for read in &reads {
            let written = name(read.name);
            let (kind, target, target_page) = match read.kind {
                ReadKind::Known(DeckSymbolKind::Instance) => {
                    let upper = written.to_ascii_uppercase();
                    let found = index.scope_chain(read.scope as usize).find_map(|level| {
                        self.instances.get(&(level as u32, upper.clone())).copied()
                    });
                    match found {
                        Some((page, local)) => (DeckSymbolKind::Instance, local, page),
                        None => (DeckSymbolKind::Instance, NONE, NONE),
                    }
                }
                ReadKind::Known(kind) => {
                    (kind, self.resolve(index, kind, &written, read.scope), NONE)
                }
                ReadKind::ModelOrParameter => {
                    match self.resolve(index, DeckSymbolKind::Model, &written, read.scope) {
                        NONE => (
                            DeckSymbolKind::Parameter,
                            self.resolve(index, DeckSymbolKind::Parameter, &written, read.scope),
                            NONE,
                        ),
                        model => (DeckSymbolKind::Model, model, NONE),
                    }
                }
                ReadKind::Call => {
                    match self.resolve(index, DeckSymbolKind::Function, &written, read.scope) {
                        NONE => continue,
                        function => (DeckSymbolKind::Function, function, NONE),
                    }
                }
                ReadKind::Master => {
                    match self.resolve(index, DeckSymbolKind::Subcircuit, &written, read.scope) {
                        NONE => {
                            match self.resolve(index, DeckSymbolKind::Model, &written, read.scope) {
                                NONE => continue,
                                model => (DeckSymbolKind::Model, model, NONE),
                            }
                        }
                        subcircuit => (DeckSymbolKind::Subcircuit, subcircuit, NONE),
                    }
                }
            };
            if let Some(instance) = read.instance
                && target != NONE
                && kind != DeckSymbolKind::Parameter
            {
                instances.push((instance, target, None));
            }
            settled.push(ReadRec {
                kind,
                name: read.name,
                card: read.card,
                scope: read.scope,
                target,
                target_page,
            });
        }
        let mut nodes = page.nodes.clone();
        for transistor in &transistors {
            let model_of = |at: usize| {
                self.resolve(
                    index,
                    DeckSymbolKind::Model,
                    &name(transistor.positional[at].0),
                    transistor.scope,
                )
            };
            let candidates = (transistor.min_nodes..transistor.positional.len())
                .filter(|at| !transistor.positional[*at].1)
                .collect::<Vec<_>>();
            let chosen = candidates
                .iter()
                .copied()
                .find(|at| model_of(*at) != NONE)
                .or_else(|| candidates.last().copied());
            let node_count = chosen.unwrap_or(transistor.positional.len());
            nodes.extend(
                transistor.positional[..node_count]
                    .iter()
                    .map(|(span, _)| NodeRec {
                        name: *span,
                        card: transistor.card,
                        scope: transistor.scope,
                    }),
            );
            if let Some(at) = chosen {
                let (span, _) = transistor.positional[at];
                let target = model_of(at);
                instances.push((transistor.instance, target, Some(span)));
                settled.push(ReadRec {
                    kind: DeckSymbolKind::Model,
                    name: span,
                    card: transistor.card,
                    scope: transistor.scope,
                    target,
                    target_page: NONE,
                });
            }
        }
        // Only ground and the `.global` names are global; a deck without the
        // latter has no name longer than three letters to read.
        let no_globals = index.shared.globals.is_empty();
        for node in &mut nodes {
            if no_globals && !matches!(node.name.len, 1 | 3) {
                continue;
            }
            if index.is_global(&name(node.name)) {
                node.scope = 0;
            }
        }
        settled.sort_by_key(|read| read.name.start);
        nodes.sort_by_key(|node| node.name.start);
        Settled {
            reads: settled,
            nodes,
            instances,
        }
    }
}

impl Settled {
    /// Give `page` what resolving its reads found, and its orders.
    fn apply(self, page: &mut Page) {
        for (instance, target, model) in self.instances {
            let symbol = &mut page.symbols[instance as usize];
            symbol.target = target;
            if let Some(model) = model {
                symbol.detail_kind = DetailKind::Written;
                symbol.detail = model;
            }
        }
        page.reads = self.reads;
        page.nodes = self.nodes;
        let symbols = &page.symbols;
        let in_place = symbols
            .windows(2)
            .all(|pair| pair[0].name.start <= pair[1].name.start);
        page.symbols_by_place = if in_place {
            Vec::new()
        } else {
            let mut by_place = (0..symbols.len() as u32).collect::<Vec<_>>();
            by_place.sort_by_key(|at| symbols[*at as usize].name.start);
            by_place
        };
        let reads = &page.reads;
        let mut by_target = (0..reads.len() as u32)
            .filter(|at| {
                let read = &reads[*at as usize];
                read.target != NONE && read.target_page == NONE
            })
            .collect::<Vec<_>>();
        by_target.sort_by_key(|at| (reads[*at as usize].target, *at));
        page.reads_by_target = by_target;
        page.instance_reads = (0..reads.len() as u32)
            .filter(|at| {
                let read = &reads[*at as usize];
                read.target != NONE && read.target_page != NONE
            })
            .collect();
        page.widest_symbol = symbols
            .iter()
            .map(|symbol| symbol.name.len)
            .max()
            .unwrap_or(0);
        page.widest_read = reads.iter().map(|read| read.name.len).max().unwrap_or(0);
        page.widest_node = page
            .nodes
            .iter()
            .map(|node| node.name.len)
            .max()
            .unwrap_or(0);
        page.shrink();
    }
}

impl DeckIndex {
    /// Order the definitions and the `.global` names by name.
    fn order_shared(&mut self) {
        let mut globals = Vec::new();
        for (slot, page) in self.pages.iter().enumerate() {
            globals.extend((0..page.page.globals.len() as u32).map(|at| (slot as u32, at)));
        }
        let global = |&(slot, at): &(u32, u32)| {
            let slot = &self.pages[slot as usize];
            self.text_at(slot, slot.page.globals[at as usize])
        };
        globals.sort_by(|one, two| by_case(&global(one), &global(two)));
        let mut by_name = (0..self.shared.definitions.len() as u32).collect::<Vec<_>>();
        by_name.sort_by(|one, two| {
            let (left, right) = (
                self.definition(*one as usize),
                self.definition(*two as usize),
            );
            (left.kind(), left.scope())
                .cmp(&(right.kind(), right.scope()))
                .then_with(|| by_case(&left.name(), &right.name()))
                .then(one.cmp(two))
        });
        let shared = Arc::get_mut(&mut self.shared).expect("a new index holds its own tables");
        shared.globals = globals;
        shared.by_name = by_name;
    }

    /// Place each page after the lines of its file's pages before it, and
    /// count each one's records into the ones after it.
    fn place_pages(&mut self) {
        let (mut cards, mut symbols, mut reads, mut nodes) = (0u32, 0u32, 0u32, 0u32);
        let mut file = u32::MAX;
        let (mut start, mut line) = (0usize, 0u32);
        for slot in &mut self.pages {
            if slot.file != file {
                file = slot.file;
                start = 0;
                line = 0;
            }
            slot.start = start;
            slot.first_line = line;
            start += slot.page.bytes as usize;
            line += slot.page.line_count();
            slot.card_base = cards;
            slot.symbol_base = symbols;
            slot.read_base = reads;
            slot.node_base = nodes;
            cards += slot.page.cards.len() as u32;
            symbols += slot.page.symbols.len() as u32;
            reads += slot.page.reads.len() as u32;
            nodes += slot.page.nodes.len() as u32;
        }
    }
}

// ---------------------------------------------------------------------------
// An edit.

/// The cards and symbols of one page of an index, by their positions among
/// the index's: what [`DeckIndex::edited`] reads again for an edit that
/// starts in the page. Every card and symbol after them moves by as many as
/// the edit adds to the page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeckPageSpan {
    pub cards: Range<usize>,
    pub symbols: Range<usize>,
}

impl DeckIndex {
    /// The cards and symbols of the page holding byte `byte` of file `file`.
    #[must_use]
    pub fn page_at(&self, file: usize, byte: usize) -> Option<DeckPageSpan> {
        let slot = &self.pages[self.slot_at(file, byte)?];
        let (cards, symbols) = (slot.card_base as usize, slot.symbol_base as usize);
        Some(DeckPageSpan {
            cards: cards..cards + slot.page.cards.len(),
            symbols: symbols..symbols + slot.page.symbols.len(),
        })
    }

    /// The index of the deck after `change` to file `file`, whose text is
    /// then `text`: the page the edit falls in read again, every other one
    /// shared with this index. `None` when the edit reaches past its page,
    /// or changes what the deck defines or how the lines after the page
    /// read: a definition's name, scope or section, a subcircuit's or
    /// library section's bounds, a `.global` or include card, `.end`, a
    /// `simulator lang=` switch, a Spectre block comment or `{ }` block, an
    /// instance another page reads by name. Such an edit takes a new build.
    #[must_use]
    pub fn edited(
        &self,
        file: usize,
        change: DeckTextChange,
        text: SharedDeckText,
    ) -> Option<Self> {
        let old_text = self.texts.get(file)?;
        if change.start > change.old_end
            || change.start > change.new_end
            || change.old_end > old_text.len()
            || text.len() + change.old_end != old_text.len() + change.new_end
        {
            return None;
        }
        let first = *self.file_pages.get(file)? as usize;
        let slots = self.file_slots(file);
        let local = slots
            .partition_point(|slot| slot.start <= change.start)
            .checked_sub(1)?;
        let at = first + local;
        let slot = &self.pages[at];
        let old = &*slot.page;
        let end = slot.start + old.bytes as usize;
        if change.old_end > end {
            return None;
        }
        let new_end = (end + change.new_end).checked_sub(change.old_end)?;
        // A page grown past a few pages' size is cut into pages again.
        let new_bytes = new_end.checked_sub(slot.start).filter(|bytes| *bytes > 0)?;
        if new_bytes > 4 * PAGE_BYTES && new_bytes > old.bytes as usize {
            return None;
        }
        if local + 1 == slots.len() {
            if new_end != text.len() {
                return None;
            }
        } else if text.slice(new_end - 1..new_end) != "\n" {
            return None;
        }
        let mut reader = FileReader::new(&*text, slot.start, old.entry.clone(), old.titled);
        let mut openings = Openings::Again {
            scopes: old.opened_scopes.iter(),
            sections: old.opened_sections.iter(),
            parents: &self.shared.scopes,
        };
        let mut read = None;
        reader.read(
            slot.start,
            new_end,
            &mut openings,
            false,
            &mut |reading, _, _| read = Some(reading),
        );
        let reading = read?;
        let new = &reading.page;
        if (slot.start > 0 && !opens_page(&*text, slot.start, new))
            || new.exit != old.exit
            || new.opened_scopes != old.opened_scopes
            || new.opened_sections != old.opened_sections
            || !self.same_signature(slot, &*text, new)
        {
            return None;
        }
        let read_elsewhere = self
            .pages
            .iter()
            .enumerate()
            .any(|(other, slot)| other != at && !slot.page.instance_reads.is_empty());
        if read_elsewhere && !self.same_instances(slot, &*text, new) {
            return None;
        }
        let mut index = self.clone();
        index.texts[file] = text;
        let PageReading {
            page,
            reads,
            transistors,
        } = reading;
        index.pages[at].page = Arc::new(page);
        index.place_pages();
        let resolver = Resolver::new(
            &index,
            index
                .pages
                .iter()
                .enumerate()
                .map(|(other, _)| if other == at { &reads } else { &NO_READS }),
        );
        let settled = resolver.settle(&index, at, reads, transistors);
        settled.apply(Arc::get_mut(&mut index.pages[at].page).expect("a page read anew"));
        index.place_pages();
        Some(index)
    }

    /// Whether `new`, the page in `slot` read again from `text`, defines,
    /// declares global, opens and includes the same names in the same scopes
    /// and sections, in the same order.
    fn same_signature(&self, slot: &PageSlot, text: &dyn DeckText, new: &Page) -> bool {
        let old = &*slot.page;
        let old_text = &*self.texts[slot.file as usize];
        let old_name = |span: Span| old_text.slice(span.at(slot.start));
        let new_name = |span: Span| text.slice(span.at(slot.start));
        let same_name = |one: Span, two: Span| old_name(one).eq_ignore_ascii_case(&new_name(two));
        old.definitions.len() == new.definitions.len()
            && old
                .definitions
                .iter()
                .zip(&new.definitions)
                .all(|(one, two)| {
                    let (one, two) = (&old.symbols[*one as usize], &new.symbols[*two as usize]);
                    one.kind == two.kind
                        && one.scope == two.scope
                        && one.section == two.section
                        && one.opens == two.opens
                        && same_name(one.name, two.name)
                })
            && old.globals.len() == new.globals.len()
            && old
                .globals
                .iter()
                .zip(&new.globals)
                .all(|(one, two)| same_name(*one, *two))
            && old.section_openers.len() == new.section_openers.len()
            && old
                .section_openers
                .iter()
                .zip(&new.section_openers)
                .all(|(one, two)| old_name(one.1) == new_name(two.1))
            && old.includes.len() == new.includes.len()
            && old.includes.iter().zip(&new.includes).all(|(one, two)| {
                one.kind == two.kind
                    && old_name(one.path) == new_name(two.path)
                    && one.section.map(old_name) == two.section.map(new_name)
            })
    }

    /// Whether `new`, the page in `slot` read again from `text`, writes the
    /// same symbols in the same order, its instances by the same names in
    /// the same scopes: what a read of an instance on another page binds.
    fn same_instances(&self, slot: &PageSlot, text: &dyn DeckText, new: &Page) -> bool {
        let old = &*slot.page;
        let old_text = &*self.texts[slot.file as usize];
        old.symbols.len() == new.symbols.len()
            && old.symbols.iter().zip(&new.symbols).all(|(one, two)| {
                one.kind == two.kind
                    && (one.kind != DeckSymbolKind::Instance
                        || (one.scope == two.scope
                            && old_text
                                .slice(one.name.at(slot.start))
                                .eq_ignore_ascii_case(&text.slice(two.name.at(slot.start)))))
            })
    }
}

/// No reads, for a page whose reads are already resolved.
static NO_READS: Vec<PendingRead> = Vec::new();

/// Whether the first line of `page`, read from `text` at `start`, opens a
/// card of its own rather than continue the one before it.
fn opens_page(text: &dyn DeckText, start: usize, page: &Page) -> bool {
    let end = page
        .line_starts
        .get(1)
        .map_or(page.bytes as usize, |next| *next as usize);
    let line = text.slice(start..start + end);
    let raw = line.strip_suffix('\n').unwrap_or(&line);
    let raw = raw.strip_suffix('\r').unwrap_or(raw);
    let code = page
        .comment_on(0)
        .and_then(|at| raw.get(..at))
        .unwrap_or(raw);
    !code.trim_start().starts_with('+')
}

// ---------------------------------------------------------------------------
// Reading cards.

/// `range` of `text` without the quotes around it.
fn unquoted(text: &str, range: Range<usize>) -> Range<usize> {
    let written = &text[range.clone()];
    let start = range.start + (written.len() - written.trim_start_matches(['"', '\'']).len());
    let end = range.end - (written.len() - written.trim_end_matches(['"', '\'']).len());
    if start < end {
        start..end
    } else {
        range.start..range.start
    }
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
        head if super::is_options_card(head) => DeckCardKind::Option,
        ".model" => DeckCardKind::Model,
        ".subckt" => DeckCardKind::Subcircuit,
        ".ends" => DeckCardKind::EndSubcircuit,
        // The analyses are the parser's; `.temp` it reads as one, but a
        // deck states with it the temperature its analyses run at.
        head if head != ".temp" && super::is_analysis_directive(head) => DeckCardKind::Analysis,
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
