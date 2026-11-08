//! Format document for Verilog-A: lines indented by the block structure they
//! sit in, continuation lines placed under what they continue, and the
//! columns of a run of like declarations aligned.
//!
//! Only blanks at the start of a line and between the words of a single-line
//! declaration ever change: the formatted text lexes to the same tokens as
//! the original, or the original is kept. What cannot be moved safely is left
//! exactly as written: the inside of a block comment, a string continued over
//! lines, and a line a macro's text continues.

use std::collections::HashMap;

use crate::workbench::documents::text_editor_commands::veriloga_blocks::{
    BlockKind, BlockWord, block_words,
};
use crate::workbench::documents::text_editor_commands::veriloga_syntax::{
    LineState, SyntaxClass, SyntaxSpan, tokenize_line,
};

const INDENT_WIDTH: usize = 4;
/// Words that stand level with the statement they continue: a `begin` written
/// under its `if`, and the `else` of it.
const STANDS_LEVEL: [&str; 2] = ["begin", "else"];
/// Declaration keywords whose lists are aligned under their first name.
const DECLARATION_WORDS: [&str; 9] = [
    "parameter",
    "localparam",
    "real",
    "integer",
    "string",
    "input",
    "output",
    "inout",
    "genvar",
];

/// Where the lines an open bracket continues go: under the first thing after
/// it, or, when it ended its line, one step in from the statement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Align {
    Column(usize),
    Hang(usize),
}

struct Line<'a> {
    /// The line without its ending.
    text: &'a str,
    ending: &'a str,
    spans: Vec<SyntaxSpan>,
    /// Starts inside a block comment, a continued string or a continued
    /// macro: untouchable.
    verbatim: bool,
    indent: usize,
}

/// `source`, formatted. Idempotent: formatting the result changes nothing.
pub(crate) fn format_source(source: &str) -> String {
    let mut lines = read_lines(source);
    place_lines(&mut lines);
    let rows = aligned_rows(&lines);
    let mut output = String::with_capacity(source.len() + source.len() / 8);
    for (index, line) in lines.iter().enumerate() {
        if line.verbatim {
            output.push_str(line.text);
        } else {
            let body = rows
                .get(&index)
                .map_or_else(|| line.text.trim_start(), String::as_str);
            if !body.is_empty() {
                output.extend(std::iter::repeat_n(' ', line.indent));
                output.push_str(body);
            }
        }
        output.push_str(line.ending);
    }
    if tokens(source) == tokens(&output) {
        output
    } else {
        source.to_owned()
    }
}

/// The tokens `text` lexes to, by kind and text, as far as it lexes.
fn tokens(text: &str) -> Vec<(rspice_veriloga::TokenKind, Option<String>)> {
    let source_id = rspice_veriloga::SourceMap::new().add_source("format", String::new());
    let mut lexer = rspice_veriloga::Lexer::new(text, source_id);
    let mut read = Vec::new();
    while let Ok(token) = lexer.next_token() {
        if token.kind == rspice_veriloga::TokenKind::Eof {
            break;
        }
        read.push((token.kind, token.text.as_deref().map(str::to_owned)));
    }
    read
}

fn read_lines(source: &str) -> Vec<Line<'_>> {
    let mut state = LineState::default();
    let mut lines = Vec::new();
    let mut continues_macro = false;
    for segment in source.split_inclusive('\n') {
        let (text, ending) = match segment.strip_suffix("\r\n") {
            Some(text) => (text, "\r\n"),
            None => segment
                .strip_suffix('\n')
                .map_or((segment, ""), |text| (text, "\n")),
        };
        let verbatim = state.inside_literal() || continues_macro;
        let (spans, exit) = tokenize_line(text, state);
        state = exit;
        continues_macro = text.trim_end().ends_with('\\');
        lines.push(Line {
            text,
            ending,
            spans,
            verbatim,
            indent: 0,
        });
    }
    lines
}

fn code_spans<'a>(line: &'a Line<'a>) -> impl DoubleEndedIterator<Item = &'a SyntaxSpan> + 'a {
    line.spans
        .iter()
        .filter(|span| span.class != SyntaxClass::Comment)
}

fn word<'a>(line: &'a Line<'a>, span: &SyntaxSpan) -> &'a str {
    &line.text[span.range.clone()]
}

/// Whether a line is a compiler directive, as against a macro's use: it
/// stands at the depth around it and is no part of the statement around it.
fn is_directive(line: &Line<'_>) -> bool {
    code_spans(line).next().is_some_and(|span| {
        span.class == SyntaxClass::Directive
            && word(line, span).strip_prefix('`').is_some_and(|name| {
                rspice_veriloga::language_catalog::directives().any(|known| known == name)
            })
    })
}

/// What a line's block words do to the depth and to the open conditionals.
/// A directive's line counts only its conditionals: the text of a `` `define ``
/// is not the block structure of the file.
fn follow_words(
    words: &[BlockWord],
    depth: &mut usize,
    conditionals: &mut Vec<usize>,
    only_conditionals: bool,
) {
    for word in words {
        match word {
            BlockWord::Open(BlockKind::Conditional) => conditionals.push(*depth),
            BlockWord::Close(BlockKind::Conditional) => {
                conditionals.pop();
            }
            BlockWord::Branch => {
                if let Some(start) = conditionals.last() {
                    *depth = *start;
                }
            }
            BlockWord::Open(_) if !only_conditionals => *depth += 1,
            BlockWord::Close(_) if !only_conditionals => *depth = depth.saturating_sub(1),
            BlockWord::Open(_) | BlockWord::Close(_) => {}
        }
    }
}

/// Decide each line's indentation.
fn place_lines(lines: &mut [Line<'_>]) {
    let mut depth = 0usize;
    // The depth at each open `ifdef`, so every branch starts where the first
    // did: the two branches each open what the one `end` after them closes.
    let mut conditionals = Vec::<usize>::new();
    let mut brackets = Vec::<Align>::new();
    // The indentation of the statement being continued, when one is.
    let mut continuing = None::<usize>;
    // Where the rest of a declaration list goes.
    let mut list_column = None::<usize>;
    let mut state = LineState::default();
    for line in lines.iter_mut() {
        if line.verbatim {
            state = tokenize_line(line.text, state).1;
            continue;
        }
        let (words, exit) = block_words(line.text, state);
        state = exit;
        if is_directive(line) {
            let at = match words.first() {
                Some(BlockWord::Branch | BlockWord::Close(BlockKind::Conditional)) => {
                    conditionals.last().copied().unwrap_or(depth)
                }
                _ => depth,
            };
            line.indent = at * INDENT_WIDTH;
            follow_words(&words, &mut depth, &mut conditionals, true);
            continue;
        }
        let code = line.text.trim_start();
        let is_blank = code.is_empty();
        let comment_only = !is_blank && code_spans(line).next().is_none();
        let (mut lowest, mut running) = (0isize, 0isize);
        for word in &words {
            match word {
                BlockWord::Open(kind) if *kind != BlockKind::Conditional => running += 1,
                BlockWord::Close(kind) if *kind != BlockKind::Conditional => running -= 1,
                _ => {}
            }
            lowest = lowest.min(running);
        }
        // A word closing a block ends whatever was left unfinished inside it.
        if lowest < 0 {
            brackets.clear();
            continuing = None;
            list_column = None;
        }
        let first_word = code_spans(line).next().map_or("", |span| word(line, span));
        let closes_bracket = code.starts_with([')', ']', '}']);
        let level = depth.saturating_add_signed(lowest) * INDENT_WIDTH;
        line.indent = match (brackets.last(), continuing) {
            _ if is_blank => 0,
            (Some(Align::Column(column)), _) => *column,
            (Some(Align::Hang(hang)), _) if closes_bracket => hang.saturating_sub(INDENT_WIDTH),
            (Some(Align::Hang(hang)), _) => *hang,
            (None, Some(statement)) if !comment_only && !STANDS_LEVEL.contains(&first_word) => {
                list_column.unwrap_or(statement + INDENT_WIDTH)
            }
            _ => level,
        };
        follow_words(&words, &mut depth, &mut conditionals, false);
        if comment_only || is_blank {
            continue;
        }
        let was_continuing = continuing.is_some();
        let statement_indent = continuing.unwrap_or(line.indent);
        if !was_continuing {
            list_column = declarator_column(line).map(|column| line.indent + column);
        }
        follow_brackets(line, &mut brackets, statement_indent);
        let ended = brackets.is_empty() && ends_statement(line, &words, was_continuing);
        continuing = (!ended).then_some(statement_indent);
        if ended {
            list_column = None;
        }
    }
}

/// Track the brackets opened and closed on `line`, placed at `line.indent`.
fn follow_brackets(line: &Line<'_>, brackets: &mut Vec<Align>, statement_indent: usize) {
    let lead = line.text.len() - line.text.trim_start().len();
    for span in code_spans(line) {
        if span.class == SyntaxClass::String {
            continue;
        }
        match word(line, span) {
            "(" | "[" | "{" | "'{" => {
                let next = code_spans(line).find(|next| next.range.start >= span.range.end);
                brackets.push(match next {
                    Some(next) => {
                        let column = line.text[lead..next.range.start].chars().count();
                        Align::Column(line.indent + column)
                    }
                    None => Align::Hang(statement_indent + INDENT_WIDTH),
                });
            }
            ")" | "]" | "}" => {
                brackets.pop();
            }
            _ => {}
        }
    }
}

/// Whether a line with no bracket left open finishes its statement.
fn ends_statement(line: &Line<'_>, words: &[BlockWord], was_continuing: bool) -> bool {
    let mut code = code_spans(line);
    let Some(first) = code.next() else {
        return true;
    };
    let last = code.next_back().unwrap_or(first);
    let text = word(line, last);
    if text == ";" || last.class == SyntaxClass::Attribute {
        return true;
    }
    if text == "else" {
        return false;
    }
    // A case label; a `?:` split over lines reads the same way, but is
    // always a continuation by the time its `:` ends a line.
    if text == ":" {
        return !was_continuing;
    }
    if matches!(words.last(), Some(BlockWord::Open(_) | BlockWord::Close(_))) {
        return true;
    }
    // A macro used as a statement needs no `;`.
    first.class == SyntaxClass::Directive && (std::ptr::eq(first, last) || text == ")")
}

/// The keywords a declaration starts with and the span of its first name.
fn declared_name<'a>(line: &'a Line<'a>) -> Option<(&'a str, String, &'a SyntaxSpan)> {
    let mut spans = code_spans(line);
    let first = spans.next()?;
    let keyword = word(line, first);
    if first.class != SyntaxClass::Keyword || !DECLARATION_WORDS.contains(&keyword) {
        return None;
    }
    let mut keywords = keyword.to_owned();
    for span in spans {
        if matches!(span.class, SyntaxClass::Keyword | SyntaxClass::Discipline) {
            keywords.push(' ');
            keywords.push_str(word(line, span));
        } else {
            let name = word(line, span);
            return name
                .chars()
                .all(|character| character.is_alphanumeric() || character == '_')
                .then_some((keyword, keywords, span));
        }
    }
    None
}

/// How far into `line`, past its indentation, its first declared name is.
fn declarator_column(line: &Line<'_>) -> Option<usize> {
    let lead = line.text.len() - line.text.trim_start().len();
    let span = declared_name(line).map(|(_, _, span)| span).or_else(|| {
        // A net list: a discipline, then its first net.
        let mut spans = code_spans(line);
        let discipline = spans.next()?;
        (discipline.class == SyntaxClass::Discipline)
            .then(|| spans.next())
            .flatten()
    })?;
    Some(line.text[lead..span.range.start].chars().count())
}

/// One declaration on one line, as a run of them aligns it.
struct Row<'a> {
    keywords: String,
    name: &'a str,
    rest: &'a str,
}

/// The single-line declaration `line` holds, if a column can be aligned for
/// it: its code ends at its own `;`.
fn row_of<'a>(line: &'a Line<'a>) -> Option<(&'a str, Row<'a>)> {
    if code_spans(line).next_back().map(|span| word(line, span)) != Some(";") {
        return None;
    }
    let (keyword, keywords, name) = declared_name(line)?;
    Some((
        keyword,
        Row {
            keywords,
            name: word(line, name),
            rest: line.text[name.range.end..].trim_end(),
        },
    ))
}

/// The bodies of a run of two or more like declarations, re-spaced into
/// columns: the keywords, the name, and the `=` after it.
fn aligned_rows(lines: &[Line<'_>]) -> HashMap<usize, String> {
    fn flush(run: &mut Vec<(usize, Row<'_>)>, rows: &mut HashMap<usize, String>) {
        if run.len() >= 2 {
            let widest = |width: fn(&Row<'_>) -> usize| {
                run.iter().map(|(_, row)| width(row)).max().unwrap_or(0)
            };
            let keywords = widest(|row| row.keywords.chars().count());
            let names = widest(|row| row.name.chars().count());
            for (index, row) in run.iter() {
                let mut body = format!("{:<keywords$} {}", row.keywords, row.name);
                match row.rest.trim_start().strip_prefix('=') {
                    Some(value) => {
                        body.extend(std::iter::repeat_n(
                            ' ',
                            names - row.name.chars().count() + 1,
                        ));
                        body.push('=');
                        body.push_str(value);
                    }
                    None => body.push_str(row.rest),
                }
                rows.insert(*index, body);
            }
        }
        run.clear();
    }

    let mut rows = HashMap::new();
    let mut run = Vec::new();
    let (mut run_keyword, mut run_indent) = ("", 0);
    for (index, line) in lines.iter().enumerate() {
        match (!line.verbatim).then(|| row_of(line)).flatten() {
            Some((keyword, row)) => {
                if run_keyword != keyword || run_indent != line.indent {
                    flush(&mut run, &mut rows);
                    (run_keyword, run_indent) = (keyword, line.indent);
                }
                run.push((index, row));
            }
            None => {
                flush(&mut run, &mut rows);
                run_keyword = "";
            }
        }
    }
    flush(&mut run, &mut rows);
    rows
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::super::language_services::formatted_verilog_a;
    use super::*;

    #[test]
    fn lines_are_indented_by_the_blocks_they_sit_in() {
        let source = "module m(a, b);\ninout a, b;\nelectrical a, b;\nanalog begin\nif (V(a) > 0) begin\nV(b) <+ 1;\nend else begin\nV(b) <+ 0;\nend\ncase (x)\n1: y = 2;\ndefault: begin\ny = 3;\nend\nendcase\nend\nendmodule\n";
        assert_eq!(
            format_source(source),
            "module m(a, b);\n    inout a, b;\n    electrical a, b;\n    analog begin\n        if (V(a) > 0) begin\n            V(b) <+ 1;\n        end else begin\n            V(b) <+ 0;\n        end\n        case (x)\n            1: y = 2;\n            default: begin\n                y = 3;\n            end\n        endcase\n    end\nendmodule\n"
        );
    }

    #[test]
    fn a_statement_without_a_block_is_one_step_in_and_a_begin_under_its_if_stays_level() {
        let source = "module m;\nanalog begin\nif (a)\nb = 1;\nelse\nb = 2;\nif (a)\nbegin\nb = 3;\nend\nx = a +\nb;\ny = a ?\nb :\nc;\nend\nendmodule\n";
        assert_eq!(
            format_source(source),
            "module m;\n    analog begin\n        if (a)\n            b = 1;\n        else\n            b = 2;\n        if (a)\n        begin\n            b = 3;\n        end\n        x = a +\n            b;\n        y = a ?\n            b :\n            c;\n    end\nendmodule\n"
        );
    }

    #[test]
    fn continuation_lines_go_under_the_list_they_continue() {
        let source = "module m(a,\nb,\nc);\nreal x,\ny,\nz;\nelectrical p,\nq;\nanalog foo(\n1,\n2\n);\nendmodule\n";
        assert_eq!(
            format_source(source),
            "module m(a,\n         b,\n         c);\n    real x,\n         y,\n         z;\n    electrical p,\n               q;\n    analog foo(\n        1,\n        2\n    );\nendmodule\n"
        );
    }

    #[test]
    fn a_run_of_like_declarations_is_aligned_in_columns() {
        let source = "module m;\nparameter real gain = 1e5; // open loop\nparameter integer n=2;\nparameter real longer_name = 3 from (0:inf);\nreal x;\nreal yy[0:3];\n\nreal z, w;\nendmodule\n";
        assert_eq!(
            format_source(source),
            "module m;\n    parameter real    gain        = 1e5; // open loop\n    parameter integer n           =2;\n    parameter real    longer_name = 3 from (0:inf);\n    real x;\n    real yy[0:3];\n\n    real z, w;\nendmodule\n"
        );
    }

    /// Directives stand at the depth around them; each branch of a
    /// conditional starts at the depth the first did, so the `end` after
    /// two `begin`s closes one.
    #[test]
    fn a_conditional_with_a_begin_in_each_branch_does_not_drift() {
        let source = "module m;\nanalog\n`ifdef FAST\nbegin\n`else\nbegin : slow\n`endif\nx = 1;\nend\nendmodule\n";
        assert_eq!(
            format_source(source),
            "module m;\n    analog\n    `ifdef FAST\n    begin\n    `else\n    begin : slow\n    `endif\n        x = 1;\n    end\nendmodule\n"
        );
    }

    #[test]
    fn a_define_is_not_block_structure_and_a_macro_use_is_a_statement() {
        let source = "`define OPEN begin\nmodule m;\nanalog begin\n`OPEN\n`CALL(a, b)\nx = 1;\nend\nendmodule\n";
        assert_eq!(
            format_source(source),
            "`define OPEN begin\nmodule m;\n    analog begin\n        `OPEN\n        `CALL(a, b)\n        x = 1;\n    end\nendmodule\n"
        );
    }

    #[test]
    fn comments_macro_lines_and_continued_text_keep_their_shape() {
        let source = "module m;\n/* keep\n     this\n*/\n// note\n`MACRO(a, b)\n`define X(a) \\\n   a + 1\nreal x;\nendmodule\n";
        assert_eq!(
            format_source(source),
            "module m;\n    /* keep\n     this\n*/\n    // note\n    `MACRO(a, b)\n    `define X(a) \\\n   a + 1\n    real x;\nendmodule\n"
        );
    }

    #[test]
    fn line_endings_and_blank_lines_are_kept() {
        let source = "module m;\r\n\r\nreal x;\r\nendmodule\r\n";
        assert_eq!(
            format_source(source),
            "module m;\r\n\r\n    real x;\r\nendmodule\r\n"
        );
    }

    fn sources_under(root: &Path) -> Vec<PathBuf> {
        let mut found = Vec::new();
        let mut pending = vec![root.to_path_buf()];
        while let Some(directory) = pending.pop() {
            for entry in std::fs::read_dir(&directory).expect("the shipped models are readable") {
                let path = entry.expect("a directory entry").path();
                if path.is_dir() {
                    pending.push(path);
                } else if path
                    .extension()
                    .and_then(|extension| extension.to_str())
                    .is_some_and(|extension| matches!(extension, "va" | "inc" | "include" | "h"))
                {
                    found.push(path);
                }
            }
        }
        found.sort();
        found
    }

    fn cmc_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../models/veriloga/cmc")
    }

    fn read_source(path: &Path) -> String {
        String::from_utf8_lossy(&std::fs::read(path).expect("a shipped source")).into_owned()
    }

    /// Every shipped CMC source, through the whole command: it is not
    /// refused, formatting again changes nothing, and the text still lexes
    /// to the same tokens.
    #[test]
    fn every_shipped_cmc_source_formats_to_the_same_tokens_and_stays_formatted() {
        let sources = sources_under(&cmc_root());
        assert!(sources.len() > 100, "the shipped models were found");
        for path in sources {
            let source = read_source(&path);
            let name = path.display();
            let once = formatted_verilog_a(&name.to_string(), &source)
                .unwrap_or_else(|refusal| panic!("{name} was refused: {refusal}"));
            assert_eq!(tokens(&source), tokens(&once), "{name} keeps its tokens");
            assert_eq!(format_source(&once), once, "{name} is stable");
        }
    }

    /// The number of parameters a model declares and all it parses to, with
    /// every source position taken out.
    fn parsed_model(root: &Path) -> Result<(usize, String), String> {
        let text = rspice_veriloga::Preprocessor::new()
            .preprocess_file(root)
            .map_err(|error| error.to_string())?;
        let mut map = rspice_veriloga::SourceMap::new();
        let id = map.add_source_mut("model", &text);
        let tokens = rspice_veriloga::Lexer::new(&text, id)
            .collect_tokens()
            .map_err(|error| error.to_string())?;
        let parsed = rspice_veriloga::Parser::new(&tokens)
            .parse()
            .map_err(|error| error.to_string())?;
        let debug = format!("{parsed:?}");
        let mut kept = String::with_capacity(debug.len());
        let mut rest = debug.as_str();
        while let Some(start) = rest.find("Span {") {
            kept.push_str(&rest[..start]);
            let span = &rest[start..];
            rest = &span[span.find('}').map_or(span.len(), |end| end + 1)..];
        }
        kept.push_str(rest);
        Ok((kept.matches("ParameterDecl {").count(), kept))
    }

    /// Every shipped model that compiles on its own, each of its files
    /// formatted, parses to what it did: the same parameters, the same
    /// everything.
    #[test]
    fn the_shipped_models_parse_the_same_after_formatting() {
        let root = cmc_root();
        let copy = std::env::temp_dir().join(format!("rspice-format-{}", std::process::id()));
        let sources = sources_under(&root);
        let place = |path: &Path| copy.join(path.strip_prefix(&root).expect("under the root"));
        for path in &sources {
            let target = place(path);
            std::fs::create_dir_all(target.parent().expect("a directory"))
                .expect("a temporary directory");
            std::fs::write(target, format_source(&read_source(path))).expect("the copy");
        }
        let (mut models, mut parameters) = (0, 0);
        for path in sources
            .iter()
            .filter(|path| path.extension().is_some_and(|extension| extension == "va"))
        {
            let Ok((declared, parsed)) = parsed_model(path) else {
                continue;
            };
            let (formatted_declared, formatted_parsed) =
                parsed_model(&place(path)).expect("a formatted model still parses");
            assert_eq!(declared, formatted_declared, "{}", path.display());
            assert!(parsed == formatted_parsed, "{} changed", path.display());
            models += 1;
            parameters += declared;
        }
        std::fs::remove_dir_all(&copy).expect("the temporary directory goes");
        assert!(models >= 30, "{models} models were compared");
        assert!(parameters > 1_000, "{parameters} parameters were compared");
    }
}
