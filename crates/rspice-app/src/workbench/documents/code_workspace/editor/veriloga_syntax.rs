//! Verilog-A/AMS syntax classes for the source editor, read a line at a time
//! through the compiler's own lexer.
//!
//! A line is read from the state the line before it left: inside a block
//! comment, inside a string a backslash carried over, inside an attribute,
//! or partway through a declaration. The text the lexer skips between two
//! tokens is comment, and a stretch it refuses, as a half-typed line holds,
//! is stepped over so the rest of the line still reads as code.

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::Range;
use std::sync::{Arc, OnceLock};

use rspice_veriloga::error::LexerErrorKind;
use rspice_veriloga::{Lexer, SourceId, Token, TokenKind};

use crate::workbench::documents::text_document_model::TextDocumentModel;

/// What a stretch of Verilog-A source is, for the ink it is painted in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum SyntaxClass {
    /// Names, operators and punctuation: the editor's plain text.
    Text,
    Keyword,
    /// A discipline or a nature, or a nature's access function.
    Discipline,
    /// A system function or task: `$temperature`.
    SystemTask,
    /// An analog operator or filter: `ddt`, `transition`, `laplace_nd`.
    AnalogOperator,
    /// The contribution operator `<+`.
    Contribution,
    Number,
    String,
    Comment,
    /// A compiler directive or a macro use.
    Directive,
    /// An attribute instance, `(* … *)`.
    Attribute,
    /// The name a module declares.
    ModuleName,
    /// A port's name where the module header or a direction declares it.
    PortName,
    /// A parameter's name where it is declared.
    ParameterName,
}

/// One classified stretch of a line, in bytes of that line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SyntaxSpan {
    pub range: Range<usize>,
    pub class: SyntaxClass,
}

/// The declaration a line ends partway through, which names the identifiers
/// the next line declares.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
enum Declaration {
    #[default]
    None,
    /// After `module`: the next name is the module's.
    ModuleName,
    /// The module's port list, up to its `;`.
    ModuleHeader,
    /// An `input`, `output` or `inout` declaration.
    Ports,
    /// A `parameter`, `localparam` or `aliasparam` declaration; a name is
    /// declared next unless a default is being read.
    Parameters { expects_name: bool },
    /// After `discipline` or `nature`: the next name is the one declared.
    TypeName,
}

/// What a line leaves open for the line after it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub(crate) struct LineState {
    block_comment: bool,
    /// A string whose line ended in a backslash, which continues it.
    string: bool,
    attribute: bool,
    declaration: Declaration,
    /// Brackets open inside the declaration.
    depth: u8,
}

/// The analog operators, filters and event functions of Verilog-AMS LRM 2.4
/// sections 4.5 and 5.10, as a call names them.
const ANALOG_OPERATORS: &[&str] = &[
    "ddt",
    "idt",
    "idtmod",
    "ddx",
    "absdelay",
    "transition",
    "slew",
    "last_crossing",
    "laplace_nd",
    "laplace_np",
    "laplace_zd",
    "laplace_zp",
    "zi_nd",
    "zi_np",
    "zi_zd",
    "zi_zp",
    "limexp",
    "white_noise",
    "flicker_noise",
    "noise_table",
    "noise_table_log",
    "analysis",
    "ac_stim",
];

/// The analog events an event control names without a call.
const ANALOG_EVENTS: &[&str] = &["initial_step", "final_step"];

/// The disciplines and natures the standard header declares, and the access
/// functions of its natures, read from the header the compiler ships.
struct StandardNames {
    types: HashSet<&'static str>,
    access: HashSet<&'static str>,
}

fn standard_names() -> &'static StandardNames {
    static NAMES: OnceLock<StandardNames> = OnceLock::new();
    NAMES.get_or_init(|| {
        let mut names = StandardNames {
            types: HashSet::new(),
            access: HashSet::new(),
        };
        for line in rspice_veriloga::stdlib::DISCIPLINES_VAMS.lines() {
            let line = line.trim();
            let name = |rest: &'static str| rest.trim_end_matches(';').trim();
            if let Some(rest) = line
                .strip_prefix("discipline ")
                .or_else(|| line.strip_prefix("nature "))
            {
                names.types.insert(name(rest));
            } else if let Some(rest) = line
                .strip_prefix("access")
                .and_then(|rest| rest.trim_start().strip_prefix('='))
            {
                names.access.insert(name(rest));
            }
        }
        names
    })
}

fn line_source() -> SourceId {
    static SOURCE: OnceLock<SourceId> = OnceLock::new();
    *SOURCE
        .get_or_init(|| rspice_veriloga::SourceMap::new().add_source("editor line", String::new()))
}

/// Classify `line`, which holds no line break, read from `entry`; returns its
/// spans in order and the state it leaves for the next line. Whitespace
/// between spans is left out.
pub(crate) fn tokenize_line(line: &str, entry: LineState) -> (Vec<SyntaxSpan>, LineState) {
    let mut reader = LineReader {
        line,
        spans: Vec::new(),
        state: entry,
        number_end: None,
        defines_name: false,
        attribute_star: false,
        attribute_close: false,
    };
    reader.read();
    (reader.spans, reader.state)
}

struct LineReader<'a> {
    line: &'a str,
    spans: Vec<SyntaxSpan>,
    state: LineState,
    /// Where the number just read ends: a word glued to it is its unit.
    number_end: Option<usize>,
    /// The name after `` `define `` is the macro it defines.
    defines_name: bool,
    /// The `*` of an attribute's opening `(*` is next.
    attribute_star: bool,
    /// The `)` of an attribute's closing `*)` is next.
    attribute_close: bool,
}

impl LineReader<'_> {
    fn push(&mut self, range: Range<usize>, class: SyntaxClass) {
        if range.start < range.end {
            self.spans.push(SyntaxSpan { range, class });
        }
    }

    fn read(&mut self) {
        let length = self.line.len();
        let mut position = 0;
        if self.state.block_comment {
            match self.line.find("*/") {
                Some(end) => {
                    self.push(0..end + 2, SyntaxClass::Comment);
                    self.state.block_comment = false;
                    position = end + 2;
                }
                None => {
                    self.push(0..length, SyntaxClass::Comment);
                    return;
                }
            }
        }
        if self.state.string {
            match string_end(self.line) {
                Some(end) => {
                    self.push(0..end, SyntaxClass::String);
                    self.state.string = false;
                    position = end;
                }
                None => {
                    self.push(0..length, SyntaxClass::String);
                    self.state.string = continues_string(self.line);
                    return;
                }
            }
        }
        while position < length {
            let rest = &self.line[position..];
            let mut lexer = Lexer::new(rest, line_source());
            let mut skipped_from = position;
            let refused = loop {
                match lexer.next_token() {
                    Ok(token) => {
                        let start = position + token.span.start as usize;
                        self.skipped(skipped_from..start);
                        if token.kind == TokenKind::Eof {
                            break None;
                        }
                        skipped_from = self.token(position, &token);
                    }
                    Err(error) => break Some(error),
                }
            };
            let Some(error) = refused else {
                return;
            };
            let start = position + error.span.start as usize;
            let end = (position + error.span.end as usize).min(length);
            self.skipped(skipped_from..start);
            match error.kind {
                LexerErrorKind::UnterminatedComment => {
                    self.push(start..length, SyntaxClass::Comment);
                    self.state.block_comment = true;
                    return;
                }
                LexerErrorKind::UnterminatedString => {
                    self.push(start..length, SyntaxClass::String);
                    self.state.string = continues_string(&self.line[start..]);
                    return;
                }
                kind => {
                    let next = if end > start {
                        end
                    } else {
                        start + self.line[start..].chars().next().map_or(1, char::len_utf8)
                    };
                    let class = if matches!(kind, LexerErrorKind::InvalidNumber(_)) {
                        SyntaxClass::Number
                    } else {
                        SyntaxClass::Text
                    };
                    self.push(start..next, class);
                    position = next;
                }
            }
        }
    }

    /// Text the lexer stepped over between two tokens: blanks, and the
    /// comments among them.
    fn skipped(&mut self, range: Range<usize>) {
        let text = &self.line[range.clone()];
        let trimmed = text.trim_start();
        if trimmed.is_empty() {
            return;
        }
        let start = range.start + (text.len() - trimmed.len());
        let end = range.start + text.trim_end().len();
        self.push(start..end, SyntaxClass::Comment);
    }

    /// Classify `token`, read from `offset`, and return where the text the
    /// lexer skips next begins.
    fn token(&mut self, offset: usize, token: &Token) -> usize {
        let start = offset + token.span.start as usize;
        let mut end = offset + token.span.end as usize;
        let number_end = self.number_end.take();
        let text = token.text.as_deref().unwrap_or_default();
        if self.state.attribute {
            let class = if token.kind == TokenKind::StringLiteral {
                SyntaxClass::String
            } else {
                SyntaxClass::Attribute
            };
            self.push(start..end, class);
            if self.attribute_star {
                self.attribute_star = false;
            } else if self.attribute_close && token.kind == TokenKind::RParen {
                self.attribute_close = false;
                self.state.attribute = false;
            } else if matches!(token.kind, TokenKind::Star | TokenKind::StarStar)
                && self.line[end..].starts_with(')')
            {
                self.attribute_close = true;
            }
            return end;
        }
        let class = match token.kind {
            // A directive at the start of its line spans the operand the
            // lexer still reads as tokens: only its name is the directive.
            TokenKind::Directive => {
                end = start + text.len();
                self.defines_name = text == "`define";
                SyntaxClass::Directive
            }
            TokenKind::Contribute => SyntaxClass::Contribution,
            TokenKind::IntegerLiteral | TokenKind::RealLiteral | TokenKind::FourStateLiteral => {
                self.number_end = Some(end);
                SyntaxClass::Number
            }
            TokenKind::StringLiteral => SyntaxClass::String,
            TokenKind::SystemIdentifier => SyntaxClass::SystemTask,
            TokenKind::Identifier | TokenKind::EscapedIdentifier => {
                if number_end == Some(start) {
                    SyntaxClass::Number
                } else {
                    self.identifier(text, end)
                }
            }
            TokenKind::LParen
                if self.line[end..].starts_with('*') && !self.line[end..].starts_with("*)") =>
            {
                self.state.attribute = true;
                self.attribute_star = true;
                SyntaxClass::Attribute
            }
            kind if starts_word(text) => self.keyword(kind),
            kind => {
                self.punctuation(kind);
                SyntaxClass::Text
            }
        };
        self.push(start..end, class);
        end
    }

    fn keyword(&mut self, kind: TokenKind) -> SyntaxClass {
        let declaration = &mut self.state.declaration;
        match kind {
            TokenKind::Module | TokenKind::Macromodule | TokenKind::Connectmodule => {
                *declaration = Declaration::ModuleName;
                self.state.depth = 0;
            }
            // A direction inside the header declares a port of its list.
            TokenKind::Input | TokenKind::Output | TokenKind::Inout
                if *declaration != Declaration::ModuleHeader =>
            {
                *declaration = Declaration::Ports;
                self.state.depth = 0;
            }
            TokenKind::Parameter | TokenKind::Localparam | TokenKind::Aliasparam => {
                *declaration = Declaration::Parameters { expects_name: true };
                self.state.depth = 0;
            }
            TokenKind::Discipline | TokenKind::Nature => *declaration = Declaration::TypeName,
            TokenKind::Endmodule | TokenKind::Analog | TokenKind::Begin | TokenKind::End => {
                *declaration = Declaration::None;
                self.state.depth = 0;
            }
            _ => {}
        }
        match kind {
            TokenKind::Electrical | TokenKind::Voltage | TokenKind::Current => {
                SyntaxClass::Discipline
            }
            TokenKind::Cross | TokenKind::Above | TokenKind::Timer => SyntaxClass::AnalogOperator,
            _ => SyntaxClass::Keyword,
        }
    }

    fn identifier(&mut self, name: &str, end: usize) -> SyntaxClass {
        if std::mem::take(&mut self.defines_name) {
            return SyntaxClass::Directive;
        }
        let declaration = self.state.declaration;
        match declaration {
            Declaration::ModuleName => {
                self.state.declaration = Declaration::ModuleHeader;
                return SyntaxClass::ModuleName;
            }
            Declaration::TypeName => {
                self.state.declaration = Declaration::None;
                return SyntaxClass::Discipline;
            }
            _ => {}
        }
        let names = standard_names();
        if names.types.contains(name) {
            return SyntaxClass::Discipline;
        }
        let called = self.line[end..].trim_start().starts_with('(');
        if (called && ANALOG_OPERATORS.contains(&name)) || ANALOG_EVENTS.contains(&name) {
            return SyntaxClass::AnalogOperator;
        }
        if called && names.access.contains(name) {
            return SyntaxClass::Discipline;
        }
        let depth = self.state.depth;
        match declaration {
            Declaration::ModuleHeader if depth >= 1 => SyntaxClass::PortName,
            Declaration::Ports if depth == 0 => SyntaxClass::PortName,
            Declaration::Parameters { expects_name: true } if depth == 0 => {
                self.state.declaration = Declaration::Parameters {
                    expects_name: false,
                };
                SyntaxClass::ParameterName
            }
            _ => SyntaxClass::Text,
        }
    }

    /// Follow the brackets, commas and terminators a declaration is read by.
    fn punctuation(&mut self, kind: TokenKind) {
        let state = &mut self.state;
        if kind == TokenKind::Semicolon {
            state.declaration = Declaration::None;
            state.depth = 0;
            return;
        }
        if state.declaration == Declaration::None {
            return;
        }
        match kind {
            TokenKind::LParen
            | TokenKind::LBracket
            | TokenKind::LBrace
            | TokenKind::AssignmentPatternStart => state.depth = state.depth.saturating_add(1),
            TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace => {
                state.depth = state.depth.saturating_sub(1);
            }
            TokenKind::Comma if state.depth == 0 => {
                if let Declaration::Parameters { expects_name } = &mut state.declaration {
                    *expects_name = true;
                }
            }
            TokenKind::Assign_ if state.depth == 0 => {
                if let Declaration::Parameters { expects_name } = &mut state.declaration {
                    *expects_name = false;
                }
            }
            _ => {}
        }
    }
}

/// Whether the lexer read `text` as a word: a reserved word keeps its text,
/// an operator has none.
fn starts_word(text: &str) -> bool {
    text.starts_with(|character: char| character.is_ascii_alphabetic())
}

/// Where a string carried over from the line before ends on this line: just
/// after its closing quote.
fn string_end(line: &str) -> Option<usize> {
    let mut escaped = false;
    for (index, character) in line.char_indices() {
        match character {
            '"' if !escaped => return Some(index + 1),
            '\\' => escaped = !escaped,
            _ => escaped = false,
        }
    }
    None
}

/// Whether an unterminated string goes on to the next line: its line ends in
/// a backslash that escapes the line break.
fn continues_string(text: &str) -> bool {
    let line = text.trim_end_matches('\r');
    (line.len() - line.trim_end_matches('\\').len()) % 2 == 1
}

/// The bound on remembered line readings: a screenful many times over.
const SPAN_CACHE_LINES: usize = 4096;

/// A line's classified spans and the state it leaves the next line in.
type LineReading = (Arc<[SyntaxSpan]>, LineState);

/// Each line's entry state and its classified spans, kept between frames so a
/// frame reads only the lines it shows and an edit rereads only from where it
/// was made.
#[derive(Clone, Default)]
pub(crate) struct VerilogAHighlighter {
    lines: Vec<LineRecord>,
    /// The document revision `verified` counts lines of.
    revision: Option<(u64, usize)>,
    /// Leading lines checked against the document this revision.
    verified: usize,
    /// Each reading of a line: its spans and the state it leaves.
    readings: HashMap<(u64, LineState), LineReading>,
}

#[derive(Clone, Copy)]
struct LineRecord {
    entry: LineState,
    /// The text hash and entry the line was read with when the next line's
    /// entry was taken from it: a line that reads differently now is read
    /// again.
    read: Option<(u64, LineState)>,
}

impl VerilogAHighlighter {
    /// The spans of `logical_line` of `document` and the line's text.
    pub(crate) fn line(
        &mut self,
        document: &TextDocumentModel,
        logical_line: usize,
    ) -> (Arc<[SyntaxSpan]>, String) {
        let key = (document.revision(), document.len_chars());
        if self.revision != Some(key) {
            self.revision = Some(key);
            self.verified = 0;
            self.lines.truncate(document.line_count().max(1));
        }
        let entry = self.entry_state(document, logical_line);
        let text = line_text(document, logical_line);
        (self.read(&text, entry).0, text)
    }

    /// `text` read from `entry`, remembered by both.
    fn read(&mut self, text: &str, entry: LineState) -> LineReading {
        let key = (text_hash(text), entry);
        if let Some((spans, exit)) = self.readings.get(&key) {
            return (Arc::clone(spans), *exit);
        }
        if self.readings.len() >= SPAN_CACHE_LINES {
            self.readings.clear();
        }
        let (spans, exit) = tokenize_line(text, entry);
        let spans: Arc<[SyntaxSpan]> = spans.into();
        self.readings.insert(key, (Arc::clone(&spans), exit));
        (spans, exit)
    }

    /// The state `logical_line` starts in, read forward from the last line
    /// this revision has already checked.
    fn entry_state(&mut self, document: &TextDocumentModel, logical_line: usize) -> LineState {
        if self.lines.is_empty() {
            self.lines.push(LineRecord {
                entry: LineState::default(),
                read: None,
            });
        }
        let target = logical_line.min(document.line_count().saturating_sub(1));
        while self.verified < target {
            let line = self.verified;
            let text = line_text(document, line);
            let reading = (text_hash(&text), self.lines[line].entry);
            if self.lines[line].read != Some(reading) || self.lines.len() <= line + 1 {
                let exit = self.read(&text, reading.1).1;
                self.lines[line].read = Some(reading);
                match self.lines.get_mut(line + 1) {
                    Some(next) => next.entry = exit,
                    None => self.lines.push(LineRecord {
                        entry: exit,
                        read: None,
                    }),
                }
            }
            self.verified += 1;
        }
        self.lines[target.min(self.lines.len() - 1)].entry
    }
}

fn line_text(document: &TextDocumentModel, line: usize) -> String {
    document
        .line_content_char_range(line)
        .map(|range| document.text_range(range))
        .unwrap_or_default()
}

fn text_hash(text: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests;
