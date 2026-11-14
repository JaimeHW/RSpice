//! The block structure of a Verilog-A/AMS source, as the editor folds and
//! indents it: `begin`/`end`, `case`, functions, `generate`, modules,
//! disciplines, natures and `` `ifdef `` conditionals, read through the
//! syntax classes so a word in a comment or a string is not a block word.

use super::veriloga_syntax::{LineState, SyntaxClass, tokenize_line};
use crate::workbench::documents::text_document_model::{TextDocumentModel, TextSelection};

/// The blocks the source is built of, for folding and indentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BlockKind {
    BeginEnd,
    Case,
    Function,
    Generate,
    Module,
    Discipline,
    Nature,
    /// `` `ifdef `` or `` `ifndef `` to `` `endif ``.
    Conditional,
}

/// What a word does to the block structure around it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BlockWord {
    Open(BlockKind),
    Close(BlockKind),
    /// `` `else `` or `` `elsif ``: the end of one branch of a conditional
    /// and the start of the next.
    Branch,
}

/// The words that close a block when they stand complete and alone on a
/// line, and so take the line back to the indentation of the block they
/// close.
const CLOSING_WORDS: [&str; 10] = [
    "end",
    "endcase",
    "endfunction",
    "endgenerate",
    "endmodule",
    "enddiscipline",
    "endnature",
    "`endif",
    "`else",
    "`elsif",
];

/// The block words of `line`, in order, read from `entry`, and the state it
/// leaves the next line in. Words in comments, strings and attributes are
/// not words of the source.
pub(crate) fn block_words(line: &str, entry: LineState) -> (Vec<BlockWord>, LineState) {
    let (spans, exit) = tokenize_line(line, entry);
    let words = spans
        .iter()
        .filter_map(|span| {
            let word = &line[span.range.clone()];
            match (span.class, word) {
                (SyntaxClass::Keyword, "begin") => Some(BlockWord::Open(BlockKind::BeginEnd)),
                (SyntaxClass::Keyword, "end") => Some(BlockWord::Close(BlockKind::BeginEnd)),
                (SyntaxClass::Keyword, "case" | "casex" | "casez") => {
                    Some(BlockWord::Open(BlockKind::Case))
                }
                (SyntaxClass::Keyword, "endcase") => Some(BlockWord::Close(BlockKind::Case)),
                (SyntaxClass::Keyword, "function") => Some(BlockWord::Open(BlockKind::Function)),
                (SyntaxClass::Keyword, "endfunction") => {
                    Some(BlockWord::Close(BlockKind::Function))
                }
                (SyntaxClass::Keyword, "generate") => Some(BlockWord::Open(BlockKind::Generate)),
                (SyntaxClass::Keyword, "endgenerate") => {
                    Some(BlockWord::Close(BlockKind::Generate))
                }
                (SyntaxClass::Keyword, "module" | "macromodule" | "connectmodule") => {
                    Some(BlockWord::Open(BlockKind::Module))
                }
                (SyntaxClass::Keyword, "endmodule") => Some(BlockWord::Close(BlockKind::Module)),
                (SyntaxClass::Keyword, "discipline") => {
                    Some(BlockWord::Open(BlockKind::Discipline))
                }
                (SyntaxClass::Keyword, "enddiscipline") => {
                    Some(BlockWord::Close(BlockKind::Discipline))
                }
                (SyntaxClass::Keyword, "nature") => Some(BlockWord::Open(BlockKind::Nature)),
                (SyntaxClass::Keyword, "endnature") => Some(BlockWord::Close(BlockKind::Nature)),
                (SyntaxClass::Directive, "`ifdef" | "`ifndef") => {
                    Some(BlockWord::Open(BlockKind::Conditional))
                }
                (SyntaxClass::Directive, "`else" | "`elsif") => Some(BlockWord::Branch),
                (SyntaxClass::Directive, "`endif") => {
                    Some(BlockWord::Close(BlockKind::Conditional))
                }
                _ => None,
            }
        })
        .collect();
    (words, exit)
}

/// Whether a line, up to the caret, leaves a block open for the next line:
/// its last block word opens one, or starts the next branch of a conditional.
pub(crate) fn leaves_block_open(line: &str) -> bool {
    matches!(
        block_words(line, LineState::default()).0.last(),
        Some(BlockWord::Open(_) | BlockWord::Branch)
    )
}

/// The blocks of a Verilog-A source, read word by word so that a word in a
/// comment or a string opens nothing: `begin`/`end`, `case`, functions,
/// `generate`, modules, disciplines and natures, and each branch of a
/// `` `ifdef `` conditional. A block keeps the line that opens it and folds
/// through the line that closes it, unless that line opens the next block
/// too, as `end else begin` does.
pub(crate) fn veriloga_fold_regions(model: &TextDocumentModel) -> Vec<(usize, usize)> {
    let mut regions = Vec::new();
    let mut open = Vec::<(BlockKind, usize)>::new();
    let mut state = LineState::default();
    for (line, text) in model.line_slices().enumerate() {
        let text = text.to_string();
        let (words, exit) = block_words(text.trim_end_matches(['\n', '\r']), state);
        state = exit;
        let mut closed = Vec::new();
        let mut opens_after_close = false;
        for word in words {
            match word {
                BlockWord::Open(kind) => {
                    open.push((kind, line));
                    opens_after_close = true;
                }
                BlockWord::Close(kind) => {
                    if let Some(position) = open.iter().rposition(|(opened, _)| *opened == kind) {
                        closed.push(open[position].1);
                        open.truncate(position);
                    }
                    opens_after_close = false;
                }
                BlockWord::Branch => {
                    if let Some(position) = open
                        .iter()
                        .rposition(|(opened, _)| *opened == BlockKind::Conditional)
                    {
                        regions.push((open[position].1, line));
                        open.truncate(position);
                    }
                    open.push((BlockKind::Conditional, line));
                    opens_after_close = true;
                }
            }
        }
        let end = if opens_after_close { line } else { line + 1 };
        regions.extend(closed.into_iter().map(|start| (start, end)));
    }
    regions.retain(|(start, end)| *end > start + 1);
    regions.sort_unstable();
    regions
}

/// The indentation of the innermost block still open above `line`: the
/// indentation a word closing that block takes. At most 4096 lines are read
/// back, each from a fresh state, so a block comment opened above the first
/// line read can at worst misplace one indentation.
fn open_block_indentation(model: &TextDocumentModel, line: usize) -> Option<String> {
    let mut depth = 0usize;
    for index in (line.saturating_sub(4096)..line).rev() {
        let range = model.line_content_char_range(index)?;
        let text = model.text_range(range);
        let (words, _) = block_words(&text, LineState::default());
        for word in words.iter().rev() {
            match word {
                BlockWord::Close(_) => depth += 1,
                BlockWord::Open(_) | BlockWord::Branch if depth == 0 => {
                    return Some(leading_indentation(&text).to_owned());
                }
                BlockWord::Open(_) => depth -= 1,
                BlockWord::Branch => {}
            }
        }
    }
    None
}

fn leading_indentation(text: &str) -> &str {
    &text[..text.len() - text.trim_start_matches([' ', '\t']).len()]
}

/// Whether `character`, typed after a word, makes it a longer word.
fn continues_word(character: char) -> bool {
    character.is_alphanumeric() || matches!(character, '_' | '$')
}

/// After `typed` was put in at the caret: a closing word it completes, by
/// following it with a character no word goes on with, takes its line back
/// to the indentation of the block it closes. `end ` and `end;` come back;
/// `endpoint` stays where it is written.
pub(crate) fn dedent_after_typing(model: &mut TextDocumentModel, typed: &str) {
    if typed.chars().next().is_none_or(continues_word) {
        return;
    }
    let [selection] = model.selections() else {
        return;
    };
    if let Some(word_end) = selection.cursor.checked_sub(typed.chars().count()) {
        dedent_closing_word(model, word_end);
    }
}

/// Before the line is ended at the caret: a closing word right before the
/// caret is complete, so its line takes the indentation of the block it
/// closes.
pub(crate) fn dedent_closing_word_at_caret(model: &mut TextDocumentModel) {
    if let [selection] = model.selections() {
        let caret = selection.cursor;
        dedent_closing_word(model, caret);
    }
}

/// A closing word that is all its line holds up to `word_end`, before a
/// single caret on the same line, takes the line back to the indentation of
/// the block it closes. The caret keeps its place in the text.
fn dedent_closing_word(model: &mut TextDocumentModel, word_end: usize) {
    let [selection] = model.selections() else {
        return;
    };
    let caret = selection.cursor;
    if !selection.range().is_empty() || caret < word_end {
        return;
    }
    let line = model.line_column_for_char(word_end).0;
    let Some(content) = model.line_content_char_range(line) else {
        return;
    };
    if word_end < content.start || caret > content.end {
        return;
    }
    let word = model.text_range(content.start..word_end);
    if !CLOSING_WORDS.contains(&word.trim_start()) {
        return;
    }
    let width = leading_indentation(&word).chars().count();
    let Some(target) = open_block_indentation(model, line) else {
        return;
    };
    let target_width = target.chars().count();
    if target_width >= width {
        return;
    }
    model.set_selections(vec![TextSelection {
        anchor: content.start,
        cursor: content.start + width,
    }]);
    model.replace_at_selections(&[target]);
    model.set_selections(vec![TextSelection::caret(caret - width + target_width)]);
}

#[cfg(test)]
mod tests;
