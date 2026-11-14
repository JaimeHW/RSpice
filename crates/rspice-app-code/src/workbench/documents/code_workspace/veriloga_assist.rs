//! What the Verilog-A editor offers and explains at the caret: the completion
//! list, what a name is, and the signature of the call being written.
//!
//! Everything here reads source text and the file's symbols and answers in
//! plain values; the editor draws the answers. The language comes from
//! [`VerilogACatalog`], the file's own names from [`VerilogASymbols`]. A word
//! in a comment or a string is not code and gets no answer, except the file
//! name of an `` `include ``, which is completed like any other name.

use std::ops::Range;

use crate::workbench::documents::text_editor_commands::veriloga_syntax::{
    LineState, SyntaxClass, SyntaxSpan, tokenize_line,
};

use super::veriloga_catalog::{CatalogKind, VerilogACatalog};
use super::veriloga_symbols::{VerilogASymbolKind, VerilogASymbols};

/// How far back a call is looked for: no call in real source runs longer.
const CALL_LOOK_BACK_LINES: usize = 64;

/// What the caret is in the middle of writing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum QueryKind {
    /// A plain name.
    Name,
    /// A `$` system function or task.
    System,
    /// A backtick directive or macro.
    Directive,
    /// The file name of an `` `include ``.
    Include,
}

/// The word being completed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CompletionQuery {
    pub kind: QueryKind,
    /// The bytes an accepted completion replaces, ending at the caret.
    pub range: Range<usize>,
    /// What is written so far, without the `$` or backtick of its kind.
    pub prefix: String,
}

/// One row of the completion list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Candidate {
    pub kind: &'static str,
    pub label: String,
    /// How it is written or declared: `(expr [, abstol])`.
    pub detail: String,
    pub description: String,
    pub insert: String,
}

/// What a name under the caret or the pointer is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NameInformation {
    /// `ddt(expr [, abstol])`, or a declared name.
    pub title: String,
    pub kind: &'static str,
    /// What its declaration says: `parameter real = 1e5 from (0:inf)`.
    pub detail: Option<String>,
    pub description: Option<String>,
}

impl NameInformation {
    /// The name and what it is, on one line, for a screen reader.
    pub(crate) fn sentence(&self) -> String {
        let mut sentence = format!("{}: {}", self.title, self.kind);
        for part in [self.detail.as_deref(), self.description.as_deref()]
            .into_iter()
            .flatten()
        {
            sentence.push_str(". ");
            sentence.push_str(part.trim_end_matches('.'));
        }
        sentence.push('.');
        sentence
    }
}

/// A call around the caret.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CallContext {
    pub name: String,
    /// Zero-based position of the argument being written.
    pub argument: usize,
    /// The byte of the call's `(`.
    pub open: usize,
}

/// A call's signature split around the argument being written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SignatureHelp {
    pub before: String,
    /// Empty when no argument of the signature is being written.
    pub active: String,
    pub after: String,
    pub description: String,
}

impl SignatureHelp {
    pub(crate) fn signature(&self) -> String {
        format!("{}{}{}", self.before, self.active, self.after)
    }
}

/// The classified spans of the line holding byte `offset`, that line's start
/// byte, and its text, read from the top of the file so a block comment or a
/// string opened above is known.
fn line_spans(source: &str, offset: usize) -> (Vec<SyntaxSpan>, usize, &str) {
    let mut state = LineState::default();
    let mut start = 0;
    loop {
        let end = source[start..]
            .find('\n')
            .map_or(source.len(), |index| start + index);
        let text = source[start..end].trim_end_matches('\r');
        let (spans, exit) = tokenize_line(text, state);
        if offset <= end || end == source.len() {
            return (spans, start, text);
        }
        state = exit;
        start = end + 1;
    }
}

/// The class of the span that holds the text just before `column` (a byte of
/// the line), if any does.
fn class_before(spans: &[SyntaxSpan], column: usize) -> Option<&SyntaxSpan> {
    spans
        .iter()
        .find(|span| span.range.start < column && column <= span.range.end)
}

fn is_word_character(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

/// The start of the word that ends at byte `end` of `source`.
fn word_start(source: &str, end: usize) -> usize {
    source[..end]
        .char_indices()
        .rev()
        .take_while(|(_, character)| is_word_character(*character))
        .last()
        .map_or(end, |(index, _)| index)
}

/// What the caret at byte `caret` is in the middle of writing, or `None`
/// where nothing completes: a comment, a string, an attribute, a number, or
/// after a `.`.
pub(crate) fn completion_query(source: &str, caret: usize) -> Option<CompletionQuery> {
    let (spans, line_start, line) = line_spans(source, caret);
    let column = caret.checked_sub(line_start)?.min(line.len());
    let holder = class_before(&spans, column);
    match holder.map(|span| span.class) {
        Some(SyntaxClass::Comment | SyntaxClass::Attribute) => return None,
        Some(SyntaxClass::String) => return include_query(line, line_start, column, &spans),
        _ => {}
    }
    let start = word_start(source, caret);
    let prefix = &source[start..caret];
    if prefix.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        return None;
    }
    let before = source[..start].chars().next_back();
    let (kind, from) = match before {
        Some('$') => (QueryKind::System, start - 1),
        Some('`') => (QueryKind::Directive, start - 1),
        Some('.') => return None,
        _ => (QueryKind::Name, start),
    };
    Some(CompletionQuery {
        kind,
        range: from..caret,
        prefix: prefix.to_owned(),
    })
}

/// The file name being written in the string of an `` `include `` on `line`.
fn include_query(
    line: &str,
    line_start: usize,
    column: usize,
    spans: &[SyntaxSpan],
) -> Option<CompletionQuery> {
    let (index, string) = spans.iter().enumerate().find(|(_, span)| {
        span.class == SyntaxClass::String && span.range.contains(&(column - 1))
    })?;
    let directive = index.checked_sub(1).map(|before| &spans[before])?;
    if directive.class != SyntaxClass::Directive || &line[directive.range.clone()] != "`include" {
        return None;
    }
    // Inside the quotes: after the opening one, not past the closing one.
    let inner = string.range.start + 1;
    if column < inner {
        return None;
    }
    Some(CompletionQuery {
        kind: QueryKind::Include,
        range: line_start + inner..line_start + column,
        prefix: line[inner..column].to_owned(),
    })
}

fn matches_prefix(name: &str, prefix: &str) -> bool {
    name.len() >= prefix.len()
        && name
            .get(..prefix.len())
            .is_some_and(|start| start.eq_ignore_ascii_case(prefix))
}

/// The text after a callable's name in the list: `(expr [, abstol])`.
fn argument_list(signature: &str, name: &str) -> String {
    signature.strip_prefix(name).unwrap_or(signature).to_owned()
}

/// The completions of `query`, the file's own names first where a plain name
/// is being written.
///
/// `symbols` are the file's declarations, when they could be read;
/// `include_files` the project files an `` `include `` may name. An exact
/// match is left out unless the list was `asked_for`, so that a finished word
/// does not keep a list open by itself.
pub(crate) fn candidates(
    query: &CompletionQuery,
    source: &str,
    symbols: Option<&VerilogASymbols>,
    include_files: &[String],
    asked_for: bool,
) -> Vec<Candidate> {
    let catalog = VerilogACatalog::get();
    let caret = query.range.end;
    // A `(` already written after the caret is the call's own.
    let paren_follows = source[caret..]
        .trim_start_matches([' ', '\t'])
        .starts_with('(');
    let opening = |insert: String, callable: bool| {
        if callable && !paren_follows {
            format!("{insert}(")
        } else {
            insert
        }
    };
    let keep = |name: &str| {
        matches_prefix(name, &query.prefix)
            && (asked_for || !name.eq_ignore_ascii_case(&query.prefix))
    };
    let from_catalog = |kinds: &[CatalogKind], out: &mut Vec<Candidate>| {
        let mut listed = catalog
            .entries()
            .iter()
            .filter(|entry| kinds.contains(&entry.kind))
            .filter(|entry| keep(entry.name.trim_start_matches(['$', '`'])))
            .collect::<Vec<_>>();
        listed.sort_by(|left, right| left.name.cmp(&right.name));
        for entry in listed {
            let callable = entry.parameters.is_some();
            out.push(Candidate {
                kind: entry.kind.label(),
                label: entry.name.clone(),
                detail: argument_list(&entry.signature(), &entry.name),
                description: entry.description.clone(),
                insert: opening(entry.name.clone(), callable),
            });
        }
    };
    let mut out = Vec::new();
    match query.kind {
        QueryKind::System => {
            from_catalog(
                &[CatalogKind::SystemFunction, CatalogKind::SystemTask],
                &mut out,
            );
        }
        QueryKind::Directive => {
            for (name, value) in defined_macros(source) {
                if keep(&name) {
                    out.push(Candidate {
                        kind: CatalogKind::Macro.label(),
                        label: format!("`{name}"),
                        detail: value.clone(),
                        description: "Defined in this file.".to_owned(),
                        insert: format!("`{name}"),
                    });
                }
            }
            from_catalog(&[CatalogKind::Directive], &mut out);
            let mut macros = catalog
                .entries()
                .iter()
                .filter(|entry| entry.kind == CatalogKind::Macro && keep(&entry.name))
                .collect::<Vec<_>>();
            macros.sort_by(|left, right| left.name.cmp(&right.name));
            for entry in macros {
                out.push(Candidate {
                    kind: entry.kind.label(),
                    label: format!("`{}", entry.name),
                    detail: String::new(),
                    description: entry.description.clone(),
                    insert: format!("`{}", entry.name),
                });
            }
        }
        QueryKind::Include => {
            let closes = source[caret..].starts_with('"');
            let closing = if closes { "" } else { "\"" };
            let headers = catalog
                .entries()
                .iter()
                .filter(|entry| entry.kind == CatalogKind::Header)
                .map(|entry| entry.name.clone());
            let builtin = headers.collect::<Vec<_>>();
            for (path, kind, description) in include_files
                .iter()
                .map(|path| (path.clone(), "file", "A file of this project."))
                .chain(builtin.iter().map(|name| {
                    (
                        name.clone(),
                        CatalogKind::Header.label(),
                        "Standard header built into RSpice.",
                    )
                }))
            {
                if keep(&path) {
                    out.push(Candidate {
                        kind,
                        label: path.clone(),
                        detail: String::new(),
                        description: description.to_owned(),
                        insert: format!("{path}{closing}"),
                    });
                }
            }
        }
        QueryKind::Name => {
            if let Some(symbols) = symbols {
                let mut visible = symbols.visible_at(caret);
                // Stable: within each group the innermost scope stays first.
                visible.sort_by_key(|declaration| {
                    matches!(
                        declaration.kind,
                        VerilogASymbolKind::Module | VerilogASymbolKind::Block
                    )
                });
                for declaration in visible {
                    if !keep(&declaration.name) {
                        continue;
                    }
                    let callable = declaration.kind == VerilogASymbolKind::Function;
                    out.push(Candidate {
                        kind: declaration.kind.label(),
                        label: declaration.name.clone(),
                        detail: declaration.detail.clone(),
                        description: declaration.description.clone().unwrap_or_default(),
                        insert: opening(declaration.name.clone(), callable),
                    });
                }
            }
            from_catalog(
                &[
                    CatalogKind::Access,
                    CatalogKind::AnalogOperator,
                    CatalogKind::Function,
                ],
                &mut out,
            );
            from_catalog(&[CatalogKind::Constant], &mut out);
            from_catalog(&[CatalogKind::Discipline, CatalogKind::Nature], &mut out);
            let offered = out
                .iter()
                .map(|candidate| candidate.label.clone())
                .collect::<std::collections::BTreeSet<_>>();
            let mut keywords = Vec::new();
            from_catalog(&[CatalogKind::Keyword], &mut keywords);
            out.extend(
                keywords
                    .into_iter()
                    .filter(|candidate| !offered.contains(&candidate.label)),
            );
        }
    }
    out
}

/// The macros `source` defines itself: name and replacement text.
fn defined_macros(source: &str) -> Vec<(String, String)> {
    let mut macros = Vec::new();
    for line in source.lines() {
        let Some(rest) = line.trim_start().strip_prefix("`define") else {
            continue;
        };
        let rest = rest.trim_start();
        let end = rest
            .find(|c: char| !is_word_character(c))
            .unwrap_or(rest.len());
        let name = &rest[..end];
        if !name.is_empty()
            && !macros
                .iter()
                .any(|(known, _): &(String, String)| known == name)
        {
            macros.push((name.to_owned(), rest[end..].trim().to_owned()));
        }
    }
    macros
}

/// The word holding byte `offset`, with the `$` or backtick that begins it.
fn word_at(source: &str, offset: usize) -> Option<Range<usize>> {
    let offset = offset.min(source.len());
    if !source.is_char_boundary(offset) {
        return None;
    }
    let forward = source[offset..]
        .char_indices()
        .take_while(|(_, character)| is_word_character(*character))
        .last()
        .map_or(offset, |(index, character)| {
            offset + index + character.len_utf8()
        });
    let mut start = word_start(source, offset);
    if start == forward {
        return None;
    }
    if matches!(source[..start].chars().next_back(), Some('$' | '`')) {
        start -= 1;
    }
    Some(start..forward)
}

/// What the name at byte `offset` is, when it is a name with something to
/// say: declared in the file, or known to the language. A reserved word that
/// is nothing else is not explained, and neither is anything in a comment or
/// a string.
pub(crate) fn information_at(
    source: &str,
    offset: usize,
    symbols: Option<&VerilogASymbols>,
) -> Option<NameInformation> {
    let word = word_at(source, offset)?;
    let (spans, line_start, line) = line_spans(source, word.start);
    let column = (word.start + 1).saturating_sub(line_start).min(line.len());
    if class_before(&spans, column).is_some_and(|span| {
        matches!(
            span.class,
            SyntaxClass::Comment | SyntaxClass::String | SyntaxClass::Attribute
        )
    }) {
        return None;
    }
    if let Some(symbols) = symbols
        && let Some(index) = symbols
            .name_at(word.start)
            .and_then(|name| name.declaration)
        && let Some(declaration) = symbols.declaration(index)
    {
        return Some(NameInformation {
            title: declaration.name.clone(),
            kind: declaration.kind.label(),
            detail: Some(declaration.detail.clone()),
            description: declaration.description.clone(),
        });
    }
    let entry = VerilogACatalog::get().written(&source[word])?;
    (entry.kind != CatalogKind::Keyword).then(|| NameInformation {
        title: entry.signature(),
        kind: entry.kind.label(),
        detail: None,
        description: Some(entry.description.clone()),
    })
}

struct OpenBracket {
    /// The word right before a `(`.
    name: Option<String>,
    open: usize,
    commas: usize,
}

/// The innermost call around byte `caret` whose name `known` accepts: a
/// bracket with no name, or with one it refuses (`if (`), is looked through.
/// The text is read from a bounded number of lines back, skipping comments
/// and strings; a `;` ends whatever was open.
pub(crate) fn call_at(
    source: &str,
    caret: usize,
    known: impl Fn(&str) -> bool,
) -> Option<CallContext> {
    let caret = caret.min(source.len());
    let line_starts = super::veriloga_symbols::line_starts(source);
    let caret_line = line_starts
        .partition_point(|start| *start <= caret)
        .saturating_sub(1);
    let first_line = caret_line.saturating_sub(CALL_LOOK_BACK_LINES);
    // The state the first line read starts in.
    let mut state = LineState::default();
    for (index, start) in line_starts.iter().enumerate().take(first_line) {
        let end = line_starts[index + 1] - 1;
        state = tokenize_line(source[*start..end].trim_end_matches('\r'), state).1;
    }
    let mut open = Vec::<OpenBracket>::new();
    for (index, start) in line_starts
        .iter()
        .copied()
        .enumerate()
        .take(caret_line + 1)
        .skip(first_line)
    {
        let end = line_starts
            .get(index + 1)
            .map_or(source.len(), |next| next - 1);
        let text = source[start..end].trim_end_matches('\r');
        let (spans, exit) = tokenize_line(text, state);
        state = exit;
        let read_to = if index == caret_line {
            (caret - start).min(text.len())
        } else {
            text.len()
        };
        for (at, character) in text[..read_to].char_indices() {
            if spans.iter().any(|span| {
                span.range.contains(&at)
                    && matches!(span.class, SyntaxClass::Comment | SyntaxClass::String)
            }) {
                continue;
            }
            match character {
                '(' | '[' | '{' => {
                    let word_end = text[..at].trim_end().len();
                    let begin = word_start(text, word_end);
                    let mut name = (character == '(' && begin < word_end)
                        .then(|| text[begin..word_end].to_owned());
                    if let Some(word) = name.as_mut()
                        && let Some(sigil @ ('$' | '`')) = text[..begin].chars().next_back()
                    {
                        word.insert(0, sigil);
                    }
                    open.push(OpenBracket {
                        name,
                        open: start + at,
                        commas: 0,
                    });
                }
                ')' | ']' | '}' => {
                    open.pop();
                }
                ',' => {
                    if let Some(bracket) = open.last_mut() {
                        bracket.commas += 1;
                    }
                }
                ';' => open.clear(),
                _ => {}
            }
        }
    }
    open.iter().rev().find_map(|bracket| {
        let name = bracket.name.as_ref().filter(|name| known(name))?;
        Some(CallContext {
            name: name.clone(),
            argument: bracket.commas,
            open: bracket.open,
        })
    })
}

/// Whether `name` can be called: a callable of the language, or an analog
/// function of the file.
pub(crate) fn is_callable(name: &str, symbols: Option<&VerilogASymbols>) -> bool {
    VerilogACatalog::get()
        .written(name)
        .is_some_and(|entry| entry.parameters.is_some())
        || symbols.is_some_and(|symbols| {
            symbols.declarations().iter().any(|declaration| {
                declaration.kind == VerilogASymbolKind::Function && declaration.name == name
            })
        })
}

/// The signature of `call`, with the argument being written marked.
pub(crate) fn signature_help(
    call: &CallContext,
    symbols: Option<&VerilogASymbols>,
) -> Option<SignatureHelp> {
    if let Some(symbols) = symbols
        && let Some(function) = symbols.declarations().iter().find(|declaration| {
            declaration.kind == VerilogASymbolKind::Function && declaration.name == call.name
        })
    {
        let mut before = format!("{}(", function.name);
        let mut active = String::new();
        let mut after = String::new();
        for (index, argument) in function.arguments.iter().enumerate() {
            let separator = if index == 0 { "" } else { ", " };
            if index < call.argument {
                before.push_str(separator);
                before.push_str(argument);
            } else if index == call.argument {
                before.push_str(separator);
                active.clone_from(argument);
            } else {
                after.push_str(separator);
                after.push_str(argument);
            }
        }
        after.push(')');
        return Some(SignatureHelp {
            before,
            active,
            after,
            description: function.detail.clone(),
        });
    }
    let entry = VerilogACatalog::get().written(&call.name)?;
    entry.parameters.as_ref()?;
    let (before, active, after) = entry
        .signature_parts(call.argument)
        .unwrap_or_else(|| (entry.signature(), String::new(), String::new()));
    Some(SignatureHelp {
        before,
        active,
        after,
        description: entry.description.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const MODULE: &str = "`include \"disciplines.vams\"\n`define GAIN_LIMIT 10\nmodule amp(in, out);\n  input in;\n  output out;\n  electrical in, out;\n  (* desc=\"Open-loop gain\", units=\"V/V\" *) parameter real gain = 1e5 from (0:inf);\n  real state, slope;\n  branch (out) load;\n  analog function real half;\n    input x;\n    real x;\n    half = x / 2;\n  endfunction\n  analog begin\n    state = ddt(V(in));\n    \n    V(out) <+ gain * state;\n  end\nendmodule\n";

    fn caret_after(source: &str, text: &str) -> usize {
        source.find(text).expect("the text is in the source") + text.len()
    }

    fn labels(candidates: &[Candidate]) -> Vec<&str> {
        candidates
            .iter()
            .map(|candidate| candidate.label.as_str())
            .collect()
    }

    fn with_text_at_blank_line(text: &str) -> (String, usize) {
        let blank = MODULE
            .find("    \n")
            .expect("the blank line of the analog block")
            + 4;
        let mut source = MODULE.to_owned();
        source.insert_str(blank, text);
        (source, blank + text.len())
    }

    fn list(source: &str, caret: usize, asked_for: bool) -> Vec<Candidate> {
        let query = completion_query(source, caret).expect("the caret completes");
        let line = source[..caret].matches('\n').count() + 1;
        let symbols = VerilogASymbols::read_forgiving(source, Some(line));
        candidates(&query, source, Some(&symbols), &[], asked_for)
    }

    #[test]
    fn a_dollar_lists_the_system_functions_and_tasks_with_their_signatures() {
        let (source, caret) = with_text_at_blank_line("$");
        let offered = list(&source, caret, false);
        let strobe = offered
            .iter()
            .find(|candidate| candidate.label == "$strobe")
            .expect("$strobe is offered");
        assert_eq!(strobe.detail, "(format [, arg…])");
        assert_eq!(strobe.kind, "task");
        assert_eq!(strobe.insert, "$strobe(");
        let temperature = offered
            .iter()
            .find(|candidate| candidate.label == "$temperature")
            .expect("$temperature is offered");
        assert_eq!(temperature.kind, "function");
        assert_eq!(temperature.insert, "$temperature");
        assert!(
            offered
                .iter()
                .all(|candidate| candidate.label.starts_with('$'))
        );
        let (source, caret) = with_text_at_blank_line("$str");
        assert_eq!(labels(&list(&source, caret, false)), ["$strobe"]);
    }

    /// Asked for inside the analog block, the list leads with what the
    /// module declares.
    #[test]
    fn the_list_inside_an_analog_block_offers_the_modules_own_names_first() {
        let (source, caret) = with_text_at_blank_line("");
        let offered = list(&source, caret, true);
        let names = labels(&offered);
        let first = names
            .iter()
            .position(|name| *name == "ddt")
            .expect("the analog operators are offered too");
        for name in ["in", "out", "gain", "state", "slope", "load", "half"] {
            let place = names
                .iter()
                .position(|listed| listed == &name)
                .unwrap_or_else(|| panic!("{name} is offered in {names:?}"));
            assert!(place < first, "{name} comes before the language's names");
        }
        assert_eq!(
            offered
                .iter()
                .find(|candidate| candidate.label == "half")
                .map(|candidate| candidate.insert.as_str()),
            Some("half(")
        );
        assert_eq!(
            offered
                .iter()
                .find(|candidate| candidate.label == "gain")
                .map(|candidate| candidate.detail.as_str()),
            Some("parameter real = 1e5 from (0:inf)")
        );
        for name in ["begin", "electrical", "V", "Voltage"] {
            assert!(names.contains(&name), "{name} is offered");
        }
    }

    #[test]
    fn a_function_argument_is_offered_only_inside_its_function() {
        let inside = caret_after(MODULE, "half = x");
        let offered = list(MODULE, inside, true);
        assert!(labels(&offered).contains(&"x"));
        let (source, caret) = with_text_at_blank_line("");
        assert!(!labels(&list(&source, caret, true)).contains(&"x"));
    }

    #[test]
    fn a_backtick_lists_directives_and_macros_and_an_include_lists_files() {
        let (source, caret) = with_text_at_blank_line("`");
        let offered = list(&source, caret, false);
        let names = labels(&offered);
        for name in ["`GAIN_LIMIT", "`include", "`ifdef", "`M_PI"] {
            assert!(names.contains(&name), "{name} is offered in {names:?}");
        }
        assert_eq!(names[0], "`GAIN_LIMIT", "the file's own macro comes first");

        let source = "`include \"di\nmodule m; endmodule\n";
        let caret = caret_after(source, "\"di");
        let query = completion_query(source, caret).expect("the include is completed");
        assert_eq!(query.kind, QueryKind::Include);
        assert_eq!(query.prefix, "di");
        let files = ["diode_tables.va".to_owned(), "other.va".to_owned()];
        let offered = candidates(&query, source, None, &files, false);
        assert_eq!(labels(&offered), ["diode_tables.va", "disciplines.vams"]);
        assert_eq!(offered[1].insert, "disciplines.vams\"");

        let source = "`include \"di\"\n";
        let caret = caret_after(source, "\"di");
        let query = completion_query(source, caret).expect("the include is completed");
        let offered = candidates(&query, source, None, &files, false);
        assert_eq!(offered[1].insert, "disciplines.vams", "the quote is there");
    }

    #[test]
    fn nothing_completes_in_a_comment_a_string_a_number_or_after_a_dot() {
        for (source, written) in [
            ("// gai", "gai"),
            ("/* gai", "gai"),
            ("/*\n gai", "gai"),
            ("$strobe(\"gai", "gai"),
            ("real x = 10p", "10p"),
            ("foo.ba", "ba"),
            ("(* gai", "gai"),
        ] {
            let caret = caret_after(source, written);
            assert_eq!(completion_query(source, caret), None, "{source}");
        }
    }

    #[test]
    fn an_exact_match_closes_the_list_unless_it_was_asked_for_and_a_paren_is_kept() {
        let (source, caret) = with_text_at_blank_line("ddt");
        assert!(!labels(&list(&source, caret, false)).contains(&"ddt"));
        assert!(labels(&list(&source, caret, true)).contains(&"ddt"));
        let (source, caret) = with_text_at_blank_line("dd(V(in))");
        let caret = caret - "(V(in))".len();
        let offered = list(&source, caret, false);
        assert_eq!(
            offered
                .iter()
                .find(|candidate| candidate.label == "ddt")
                .map(|candidate| candidate.insert.as_str()),
            Some("ddt"),
            "the call's parenthesis is already written"
        );
    }

    #[test]
    fn a_name_explains_itself_from_its_declaration_or_the_language() {
        let symbols = VerilogASymbols::read(MODULE);
        let at = |written: &str| {
            information_at(MODULE, caret_after(MODULE, written) - 1, Some(&symbols))
        };
        let ddt = at("state = ddt").expect("ddt");
        assert_eq!(ddt.title, "ddt(expr [, abstol])");
        assert_eq!(ddt.kind, "operator");
        assert_eq!(ddt.description.as_deref(), Some("Time derivative of expr."));
        let gain = at("<+ gain").expect("gain");
        assert_eq!(gain.title, "gain");
        assert_eq!(gain.kind, "parameter");
        assert_eq!(
            gain.detail.as_deref(),
            Some("parameter real = 1e5 from (0:inf)")
        );
        assert_eq!(gain.description.as_deref(), Some("Open-loop gain (V/V)"));
        assert_eq!(
            gain.sentence(),
            "gain: parameter. parameter real = 1e5 from (0:inf). Open-loop gain (V/V)."
        );
        assert_eq!(
            at("analog").map(|_| ()),
            None,
            "a plain keyword says nothing"
        );
        let source = "// ddt\n$strobe(\"ddt\");\n$temperature;\n`M_PI;\n";
        let symbols = VerilogASymbols::read(source);
        let at = |written: &str, nth: usize| {
            let offset = source
                .match_indices(written)
                .nth(nth)
                .expect("the text is in the source")
                .0;
            information_at(source, offset + 1, Some(&symbols)).map(|info| info.title)
        };
        assert_eq!(at("ddt", 0), None, "in a comment");
        assert_eq!(at("ddt", 1), None, "in a string");
        assert_eq!(at("$temperature", 0).as_deref(), Some("$temperature"));
        assert_eq!(at("`M_PI", 0).as_deref(), Some("M_PI"));
    }

    #[test]
    fn signature_help_marks_the_argument_being_written() {
        let symbols = VerilogASymbols::read(MODULE);
        let known = |name: &str| is_callable(name, Some(&symbols));
        let help_at = |source: &str, caret: usize| {
            let call = call_at(source, caret, known)?;
            signature_help(&call, Some(&symbols))
        };
        let source = "x = ddt(";
        let help = help_at(source, source.len()).expect("the call");
        assert_eq!(help.before, "ddt(");
        assert_eq!(help.active, "expr");
        assert_eq!(help.after, " [, abstol])");
        assert_eq!(help.signature(), "ddt(expr [, abstol])");
        let source = "x = ddt(V(a, b), ";
        let help = help_at(source, source.len()).expect("the call");
        assert_eq!(help.active, "abstol", "the inner call's comma is its own");
        let source = "x = ddt((a + b), 1, ";
        let help = help_at(source, source.len()).expect("the call");
        assert_eq!(help.active, "", "past the last argument nothing is marked");
        assert_eq!(help.before, "ddt(expr [, abstol])");
        let source = "x = V(";
        assert_eq!(
            help_at(source, source.len()).map(|help| help.active),
            Some("node".to_owned())
        );
        let source = "$strobe(\"a, (b\", ";
        let help = help_at(source, source.len()).expect("a string holds no commas");
        assert_eq!(help.active, "arg…");
        let source = "laplace_nd(V(in), {1, 2";
        let help = help_at(source, source.len()).expect("the call");
        assert_eq!(help.active, "numerator");
        let source = "x = half(";
        let help = help_at(source, source.len()).expect("the file's own function");
        assert_eq!(help.before, "half(");
        assert_eq!(help.active, "x");
        let source = "if (a";
        assert_eq!(help_at(source, source.len()), None);
        let source = "x = ddt(a);\ny = (b";
        assert_eq!(help_at(source, source.len()), None, "a statement ended it");
        let source = "x = transition(a,\n  // comma, in a comment\n  ";
        assert_eq!(
            help_at(source, source.len()).map(|help| help.active),
            Some("[td]".replace(['[', ']'], ""))
        );
    }
}
