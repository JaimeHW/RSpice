//! Scalable, language-neutral source document model.
//!
//! EGUI's stock `TextEdit` remains useful for small buffers and IME plumbing,
//! but it materializes one galley for the entire document and owns only one
//! selection. This model is the backing authority for the virtualized editor:
//! it keeps large edits logarithmic, represents multiple/column selections,
//! stores fold projection, and makes undo/redo exact and transactional.

use std::ops::Range;

use ropey::Rope;

use crate::state::TextSnapshot;

/// The text an editor's undo history may keep alive. The oldest steps go
/// once the history holds more, never the newest; typing is grouped, so a
/// day's editing of a deck stays well inside it.
const HISTORY_BYTE_BUDGET: usize = 64 << 20;
/// What a step keeps besides the text it changed: the stretch of the rope
/// the edit copied rather than shared.
const STEP_OVERHEAD_BYTES: usize = 2 << 10;
/// A pause in typing this long starts a new undo step.
const TYPING_PAUSE_SECONDS: f64 = 1.0;
/// Texts the owner's edits can leave behind and still come back to.
const PARKED_HISTORY_LIMIT: usize = 64;
/// Display cells from one tab stop to the next, as terminals, pagers and
/// the vendor model files written for them assume.
pub(crate) const TAB_STOP: usize = 8;

/// The display cells `character` takes when it starts at cell `at`: a tab
/// runs to the next tab stop, a wide glyph takes two cells and a combining
/// mark none.
pub(crate) fn char_cells(character: char, at: usize) -> usize {
    if character == '\t' {
        TAB_STOP - at % TAB_STOP
    } else {
        unicode_width::UnicodeWidthChar::width(character).unwrap_or(1)
    }
}

/// Whether every character of `text` is one cell: no tab and nothing past
/// ASCII, which is most source. The search for a tab is a byte search.
fn one_cell_a_character(text: ropey::RopeSlice<'_>) -> bool {
    text.len_bytes() == text.len_chars()
        && !text.chunks().any(|chunk| chunk.as_bytes().contains(&b'\t'))
}

/// The display cells `text`, from a line's start, takes.
fn line_cells(text: ropey::RopeSlice<'_>) -> usize {
    if one_cell_a_character(text) {
        return text.len_chars();
    }
    text.chars()
        .fold(0, |cell, character| cell + char_cells(character, cell))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextEncoding {
    Utf8,
    Utf8Bom,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub(crate) enum SourceTextError {
    #[error("source is not valid UTF-8 (first invalid byte at offset {valid_up_to})")]
    InvalidUtf8 { valid_up_to: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TextSelection {
    pub anchor: usize,
    pub cursor: usize,
}

impl TextSelection {
    pub const fn caret(char_index: usize) -> Self {
        Self {
            anchor: char_index,
            cursor: char_index,
        }
    }

    pub fn range(self) -> Range<usize> {
        self.anchor.min(self.cursor)..self.anchor.max(self.cursor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FoldRegion {
    /// Zero-based logical header line. The header remains visible.
    pub start_line: usize,
    /// Exclusive logical end line.
    pub end_line: usize,
}

impl FoldRegion {
    const fn hidden_line_count(self) -> usize {
        self.end_line.saturating_sub(self.start_line + 1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ViewportLine {
    pub visible_row: usize,
    pub logical_line: usize,
    pub first_column: usize,
    /// The display cell `first_column` starts at.
    pub first_cell: usize,
    pub text: String,
    pub total_columns: usize,
    pub folded_line_count: usize,
}

#[derive(Debug, Clone)]
struct HistoryEntry {
    before: Rope,
    before_selections: Vec<TextSelection>,
    after: Rope,
    after_selections: Vec<TextSelection>,
    /// The text the step keeps alive that no other step shares: what it
    /// removed and inserted, and the rope it copied to do so.
    bytes: usize,
}

/// A keystroke's edit: typing, or deleting a character either way.
/// Keystrokes of one kind, each where the last one left the carets and
/// without a pause between them, are one undo step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Keystroke {
    Typing,
    DeletingBackward,
    DeletingForward,
}

/// The undo step keystrokes are still being added to.
#[derive(Debug, Clone)]
struct OpenStep {
    keystroke: Keystroke,
    /// When the last keystroke of the step was made, in seconds.
    at: f64,
    /// The last character typed, which decides whether a space starts a
    /// new step.
    last_typed: Option<char>,
    /// The carets the step left, which the next keystroke must start from.
    carets: Vec<TextSelection>,
}

/// The history of a text the document's owner replaced, kept for the moment
/// the owner takes that step back.
#[derive(Debug, Clone)]
struct ParkedHistory {
    text: Rope,
    undo: Vec<HistoryEntry>,
    redo: Vec<HistoryEntry>,
}

/// The text the document's owner last published, as the editor had it then:
/// its history, and the selections from just before the first edit made
/// since. `revision` is the model's revision when it was kept.
#[derive(Debug, Clone)]
struct PublishedText {
    text: Rope,
    undo: Vec<HistoryEntry>,
    redo: Vec<HistoryEntry>,
    selections: Vec<TextSelection>,
    revision: u64,
}

#[derive(Debug, Clone, Copy)]
struct FoldProjection {
    visible_start: usize,
    cumulative_hidden: usize,
}

/// Rope-backed source bytes plus all language-neutral editor state.
#[derive(Debug, Clone)]
pub(crate) struct TextDocumentModel {
    text: Rope,
    #[cfg(test)]
    encoding: TextEncoding,
    preferred_newline: &'static str,
    selections: Vec<TextSelection>,
    folds: Vec<FoldRegion>,
    fold_projection: Vec<FoldProjection>,
    undo: Vec<HistoryEntry>,
    redo: Vec<HistoryEntry>,
    parked: Vec<ParkedHistory>,
    published: Option<PublishedText>,
    history_byte_budget: usize,
    open_step: Option<OpenStep>,
    revision: u64,
}

impl TextDocumentModel {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SourceTextError> {
        let (encoding, payload) = if let Some(payload) = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]) {
            (TextEncoding::Utf8Bom, payload)
        } else {
            (TextEncoding::Utf8, bytes)
        };
        let source =
            std::str::from_utf8(payload).map_err(|error| SourceTextError::InvalidUtf8 {
                valid_up_to: error.valid_up_to()
                    + usize::from(encoding == TextEncoding::Utf8Bom) * 3,
            })?;
        let text = Rope::from_str(source);
        let preferred_newline = detect_preferred_newline(&text);
        Ok(Self {
            text,
            #[cfg(test)]
            encoding,
            preferred_newline,
            selections: vec![TextSelection::caret(0)],
            folds: Vec::new(),
            fold_projection: Vec::new(),
            undo: Vec::new(),
            redo: Vec::new(),
            parked: Vec::new(),
            published: None,
            history_byte_budget: HISTORY_BYTE_BUDGET,
            open_step: None,
            revision: if source.is_empty() {
                TextSnapshot::default().revision()
            } else {
                TextSnapshot::next_revision()
            },
        })
    }

    pub fn from_source(source: &str) -> Self {
        Self::from_bytes(source.as_bytes()).expect("Rust strings are valid UTF-8")
    }

    #[cfg(test)]
    pub const fn encoding(&self) -> TextEncoding {
        self.encoding
    }

    pub const fn preferred_newline(&self) -> &'static str {
        self.preferred_newline
    }

    /// The revision of the text the model holds. No other text in the
    /// process carries it.
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    /// The text as it stands, shared with the model rather than copied.
    pub fn snapshot(&self) -> TextSnapshot {
        TextSnapshot::from_rope(self.text.clone(), self.revision)
    }

    pub fn len_chars(&self) -> usize {
        self.text.len_chars()
    }

    pub fn char_at(&self, char_index: usize) -> Option<char> {
        (char_index < self.text.len_chars()).then(|| self.text.char(char_index))
    }

    pub fn line_count(&self) -> usize {
        self.text.len_lines()
    }

    #[cfg(test)]
    pub fn maximum_line_cells(&self) -> usize {
        self.maximum_line_cells_in_range(0, self.line_count())
    }

    /// The display cells the widest of `line_count` lines from `start_line`
    /// takes.
    pub fn maximum_line_cells_in_range(&self, start_line: usize, line_count: usize) -> usize {
        if start_line >= self.line_count() || line_count == 0 {
            return 0;
        }
        self.text
            .lines_at(start_line)
            .take(line_count.min(self.line_count() - start_line))
            .map(|line| {
                line_cells(line.slice(..line.len_chars() - trailing_line_break_chars(line)))
            })
            .max()
            .unwrap_or(0)
    }

    pub fn maximum_selected_line_cells(&self) -> usize {
        self.selections
            .iter()
            .map(|selection| {
                let first = self.line_column_for_char(selection.range().start).0;
                let last = self.line_column_for_char(selection.range().end).0;
                self.maximum_line_cells_in_range(first, last.saturating_sub(first) + 1)
            })
            .max()
            .unwrap_or(0)
    }

    /// The display cell column `column` of `line` starts at. A column past
    /// the line's end is a cell a column past it.
    pub fn cell_at(&self, line: usize, column: usize) -> usize {
        let Some(content) = self.line_content_char_range(line) else {
            return column;
        };
        let within = column.min(content.len());
        line_cells(self.text.slice(content.start..content.start + within)) + (column - within)
    }

    /// The line `char_index` is on and the display cell it starts at.
    pub fn line_cell_for_char(&self, char_index: usize) -> (usize, usize) {
        let (line, column) = self.line_column_for_char(char_index);
        (line, self.cell_at(line, column))
    }

    /// The column of `line` whose start is nearest `cell`, a fraction of a
    /// cell being where in it a pointer is. Past the line's end it is the
    /// line's end.
    pub fn column_nearest_cell(&self, line: usize, cell: f32) -> usize {
        let Some(content) = self.line_content_char_range(line) else {
            return 0;
        };
        let text = self.text.slice(content.clone());
        if one_cell_a_character(text) {
            return (cell.round().max(0.0) as usize).min(content.len());
        }
        let mut at = 0;
        for (column, character) in text.chars().enumerate() {
            let cells = char_cells(character, at);
            if cell < at as f32 + cells as f32 * 0.5 {
                return column;
            }
            at += cells;
        }
        content.len()
    }

    /// The column of `line` whose character covers `cell`, and the cell it
    /// starts at. Past the line's end it is the line's end and its cell.
    fn column_covering_cell(&self, line: usize, cell: usize) -> (usize, usize) {
        let Some(content) = self.line_content_char_range(line) else {
            return (0, 0);
        };
        let text = self.text.slice(content.clone());
        if one_cell_a_character(text) {
            let column = cell.min(content.len());
            return (column, column);
        }
        let mut at = 0;
        for (column, character) in text.chars().enumerate() {
            let next = at + char_cells(character, at);
            if cell < next {
                return (column, at);
            }
            at = next;
        }
        (content.len(), at)
    }

    pub fn selections(&self) -> &[TextSelection] {
        &self.selections
    }

    pub fn folds(&self) -> &[FoldRegion] {
        &self.folds
    }

    /// The whole text as one string: a copy, for tests to compare.
    #[cfg(any(test, feature = "test-support"))]
    pub fn to_source(&self) -> String {
        self.text.to_string()
    }

    /// Accept a text the document's owner published. External replacement
    /// is deliberately not added to the local undo stack: the
    /// project/document service owns undo and conflict semantics across that
    /// boundary. The new text starts with no history of its own, and the
    /// history of the text it replaced is kept: when the owner takes its step
    /// back and that exact text returns, so do the edits made in it. The text
    /// the owner last published, published again over edits made since, is
    /// the owner refusing them: the refused edits are dropped, and the
    /// history and the selections are what they were before the first of
    /// them. Otherwise cursor locations are retained and clamped when
    /// possible. The model takes the snapshot's storage and revision, so
    /// owner and editor share one copy of the text.
    pub fn adopt(&mut self, snapshot: &TextSnapshot) -> bool {
        let source = snapshot.rope();
        if snapshot.revision() == self.revision || *source == self.text {
            self.keep_published();
            return false;
        }
        if let Some(published) = self
            .published
            .take()
            .filter(|published| published.text == *source)
        {
            self.undo = published.undo;
            self.redo = published.redo;
            self.selections = published.selections;
            self.text = source.clone();
        } else {
            let left = ParkedHistory {
                text: std::mem::replace(&mut self.text, source.clone()),
                undo: std::mem::take(&mut self.undo),
                redo: std::mem::take(&mut self.redo),
            };
            if !left.undo.is_empty() || !left.redo.is_empty() {
                self.parked.push(left);
                if self.parked.len() > PARKED_HISTORY_LIMIT {
                    self.parked.remove(0);
                }
            }
            if let Some(index) = self
                .parked
                .iter()
                .rposition(|parked| parked.text == self.text)
            {
                let returned = self.parked.remove(index);
                self.undo = returned.undo;
                self.redo = returned.redo;
            }
        }
        self.open_step = None;
        self.preferred_newline = detect_preferred_newline(source);
        self.selections =
            normalize_selections(std::mem::take(&mut self.selections), self.text.len_chars());
        self.revision = snapshot.revision();
        self.prune_invalid_folds();
        self.keep_published();
        true
    }

    /// [`TextDocumentModel::adopt`] for a text an owner holds as a string.
    #[cfg(test)]
    pub fn synchronize_source(&mut self, source: &str) -> bool {
        self.adopt(&TextSnapshot::new(source))
    }

    /// Keep the text as the owner published it, with its history and the
    /// selections; nothing is copied when neither changed since it was last
    /// kept.
    fn keep_published(&mut self) {
        if self
            .published
            .as_ref()
            .is_some_and(|published| published.revision == self.revision)
        {
            return;
        }
        self.published = Some(PublishedText {
            text: self.text.clone(),
            undo: self.undo.clone(),
            redo: self.redo.clone(),
            selections: self.selections.clone(),
            revision: self.revision,
        });
    }

    /// Before the first change to the text since the owner published it,
    /// keep the selections the editor had then, which a refusal returns to.
    fn keep_selections_before_change(
        published: &mut Option<PublishedText>,
        revision: u64,
        selections: &[TextSelection],
    ) {
        if let Some(published) = published
            && published.revision == revision
        {
            published.selections = selections.to_vec();
        }
    }

    #[cfg(test)]
    pub fn encoded_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(
            self.text.len_bytes() + usize::from(self.encoding == TextEncoding::Utf8Bom) * 3,
        );
        if self.encoding == TextEncoding::Utf8Bom {
            bytes.extend_from_slice(&[0xEF, 0xBB, 0xBF]);
        }
        for chunk in self.text.chunks() {
            bytes.extend_from_slice(chunk.as_bytes());
        }
        bytes
    }

    pub fn char_to_byte(&self, char_index: usize) -> usize {
        self.text
            .char_to_byte(char_index.min(self.text.len_chars()))
    }

    pub fn set_selections(&mut self, selections: Vec<TextSelection>) {
        self.selections = normalize_selections(selections, self.text.len_chars());
    }

    pub fn selected_text(&self, selection: TextSelection) -> String {
        self.text.slice(selection.range()).to_string()
    }

    pub fn text_range(&self, range: Range<usize>) -> String {
        let start = range.start.min(self.text.len_chars());
        let end = range.end.min(self.text.len_chars()).max(start);
        self.text.slice(start..end).to_string()
    }

    pub fn add_cursor(&mut self, char_index: usize) {
        let mut selections = self.selections.clone();
        selections.push(TextSelection::caret(char_index.min(self.text.len_chars())));
        self.selections = normalize_selections(selections, self.text.len_chars());
    }

    /// Create one selection per logical line between the two endpoints, at
    /// the same display cells on every line, each end at the character
    /// boundary nearest its cell and before the line ending.
    pub fn set_column_selection(
        &mut self,
        anchor_line: usize,
        anchor_cell: f32,
        cursor_line: usize,
        cursor_cell: f32,
    ) {
        let last_line = self.line_count().saturating_sub(1);
        let first = anchor_line.min(cursor_line).min(last_line);
        let last = anchor_line.max(cursor_line).min(last_line);
        let forward =
            cursor_line > anchor_line || (cursor_line == anchor_line && cursor_cell >= anchor_cell);
        let mut selections = Vec::with_capacity(last.saturating_sub(first) + 1);
        for line in first..=last {
            let anchor =
                self.char_at_line_column(line, self.column_nearest_cell(line, anchor_cell));
            let cursor =
                self.char_at_line_column(line, self.column_nearest_cell(line, cursor_cell));
            selections.push(if forward {
                TextSelection { anchor, cursor }
            } else {
                TextSelection {
                    anchor: cursor,
                    cursor: anchor,
                }
            });
        }
        self.selections = normalize_selections(selections, self.text.len_chars());
    }

    /// Replace every disjoint selection with the same text as one undoable
    /// transaction. Overlapping selections are merged before mutation.
    pub fn insert_at_selections(&mut self, inserted: &str) -> bool {
        self.replace_at_selections(&[inserted.to_owned()])
    }

    /// Replace disjoint selections with either one shared replacement or one
    /// replacement per selection as a single undo transaction.
    pub fn replace_at_selections(&mut self, replacements: &[String]) -> bool {
        let selections = normalize_selections(self.selections.clone(), self.text.len_chars());
        let edits = selections
            .iter()
            .copied()
            .map(TextSelection::range)
            .collect::<Vec<_>>();
        if edits.is_empty()
            || (replacements.len() != 1 && replacements.len() != edits.len())
            || (replacements.iter().all(String::is_empty) && edits.iter().all(Range::is_empty))
        {
            return false;
        }

        let before = self.text.clone();
        let before_selections = self.selections.clone();
        let mut bytes = STEP_OVERHEAD_BYTES;
        for (index, range) in edits.iter().enumerate().rev() {
            let replacement = if replacements.len() == 1 {
                &replacements[0]
            } else {
                &replacements[index]
            };
            bytes += replacement.len();
            if !range.is_empty() {
                bytes += self.text.slice(range.clone()).len_bytes();
                self.text.remove(range.clone());
            }
            if !replacement.is_empty() {
                self.text.insert(range.start, replacement);
            }
        }

        let mut cumulative_delta = 0_isize;
        let mut after_selections = Vec::with_capacity(edits.len());
        for (index, range) in edits.into_iter().enumerate() {
            let replacement = if replacements.len() == 1 {
                &replacements[0]
            } else {
                &replacements[index]
            };
            let inserted_chars = replacement.chars().count();
            let start = range.start.saturating_add_signed(cumulative_delta);
            let cursor = start + inserted_chars;
            after_selections.push(TextSelection::caret(cursor));
            cumulative_delta += inserted_chars as isize - range.len() as isize;
        }
        self.selections = after_selections;
        self.commit_history(before, before_selections, bytes);
        true
    }

    /// Replace the characters `range` with `replacement` as one exact undo
    /// step, with the selections the caller mapped onto the result: a
    /// structural edit that rewrote only the lines it touched.
    pub fn replace_range_transaction(
        &mut self,
        range: Range<usize>,
        replacement: &str,
        selections: Vec<TextSelection>,
    ) -> bool {
        let range = range.start.min(self.text.len_chars())..range.end.min(self.text.len_chars());
        if self.text.slice(range.clone()) == replacement {
            self.set_selections(selections);
            return false;
        }
        let before = self.text.clone();
        let before_selections = self.selections.clone();
        let bytes =
            STEP_OVERHEAD_BYTES + self.text.slice(range.clone()).len_bytes() + replacement.len();
        self.text.remove(range.clone());
        self.text.insert(range.start, replacement);
        self.selections = normalize_selections(selections, self.text.len_chars());
        self.commit_history(before, before_selections, bytes);
        true
    }

    pub fn delete_backward(&mut self) -> bool {
        let selections = self
            .selections
            .iter()
            .copied()
            .map(|selection| {
                if !selection.range().is_empty() || selection.cursor == 0 {
                    return selection;
                }
                let mut start = selection.cursor - 1;
                if self.text.char(start) == '\n' && start > 0 && self.text.char(start - 1) == '\r' {
                    start -= 1;
                }
                TextSelection {
                    anchor: start,
                    cursor: selection.cursor,
                }
            })
            .collect();
        self.set_selections(selections);
        self.insert_at_selections("")
    }

    pub fn delete_forward(&mut self) -> bool {
        let length = self.text.len_chars();
        let selections = self
            .selections
            .iter()
            .copied()
            .map(|selection| {
                if !selection.range().is_empty() || selection.cursor >= length {
                    return selection;
                }
                let mut end = selection.cursor + 1;
                if self.text.char(selection.cursor) == '\r'
                    && end < length
                    && self.text.char(end) == '\n'
                {
                    end += 1;
                }
                TextSelection {
                    anchor: selection.cursor,
                    cursor: end,
                }
            })
            .collect();
        self.set_selections(selections);
        self.insert_at_selections("")
    }

    /// One keystroke's edit at `at` seconds: `text` typed at every caret,
    /// or a character deleted either way. It joins the undo step the last
    /// keystroke made when it is of the same kind, starts where that one
    /// left the carets and follows it without a pause. A space typed after
    /// a word starts a step, so Undo takes typing back a word at a time.
    pub fn keystroke(&mut self, keystroke: Keystroke, text: &str, at: f64) -> bool {
        let carets = self.selections.clone();
        let open = self.open_step.take();
        let changed = match keystroke {
            Keystroke::Typing => self.insert_at_selections(text),
            Keystroke::DeletingBackward => self.delete_backward(),
            Keystroke::DeletingForward => self.delete_forward(),
        };
        if !changed {
            self.selections = carets;
            self.open_step = open;
            return false;
        }
        let typed = (keystroke == Keystroke::Typing)
            .then(|| text.chars().last())
            .flatten();
        let joins = open.as_ref().is_some_and(|open| {
            let space_after_word = text.starts_with(char::is_whitespace)
                && open.last_typed.is_some_and(|last| !last.is_whitespace());
            open.keystroke == keystroke
                && open.carets == carets
                && at - open.at < TYPING_PAUSE_SECONDS
                && !space_after_word
        });
        if joins
            && self.undo.len() >= 2
            && let Some(step) = self.undo.pop()
            && let Some(previous) = self.undo.last_mut()
        {
            // Undoing a run of deletions selects all it deleted, as undoing
            // one deletion selects the character it took.
            if let ([first], [next]) = (
                previous.before_selections.as_mut_slice(),
                step.before_selections.as_slice(),
            ) {
                match keystroke {
                    Keystroke::Typing => {}
                    Keystroke::DeletingBackward => first.anchor = next.range().start,
                    Keystroke::DeletingForward => first.cursor += next.range().len(),
                }
            }
            previous.after = step.after;
            previous.after_selections = step.after_selections;
            previous.bytes += step.bytes.saturating_sub(STEP_OVERHEAD_BYTES);
        }
        self.open_step = Some(OpenStep {
            keystroke,
            at,
            last_typed: typed,
            carets: self.selections.clone(),
        });
        true
    }

    /// Move each caret `by` characters within the typing step: the caret
    /// typing leaves between the two characters of a pair, or takes over a
    /// closing character that already stands there. The step, undone and
    /// redone, puts the caret back where this leaves it.
    pub fn move_carets_typing(&mut self, by: isize) {
        let continues = self
            .open_step
            .as_ref()
            .is_some_and(|open| open.carets == self.selections);
        let moved = self
            .selections
            .iter()
            .map(|selection| TextSelection::caret(selection.cursor.saturating_add_signed(by)))
            .collect();
        self.set_selections(moved);
        if continues && let Some(open) = self.open_step.as_mut() {
            open.carets = self.selections.clone();
            if let Some(step) = self.undo.last_mut() {
                step.after_selections = self.selections.clone();
            }
        }
    }

    /// Whether the history has a step to undo.
    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    /// Whether the history has a step to redo.
    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    pub fn undo(&mut self) -> bool {
        let Some(entry) = self.undo.pop() else {
            return false;
        };
        self.open_step = None;
        Self::keep_selections_before_change(&mut self.published, self.revision, &self.selections);
        self.text = entry.before.clone();
        self.selections = entry.before_selections.clone();
        self.redo.push(entry);
        self.revision = TextSnapshot::next_revision();
        self.prune_invalid_folds();
        true
    }

    pub fn redo(&mut self) -> bool {
        let Some(entry) = self.redo.pop() else {
            return false;
        };
        self.open_step = None;
        Self::keep_selections_before_change(&mut self.published, self.revision, &self.selections);
        self.text = entry.after.clone();
        self.selections = entry.after_selections.clone();
        self.undo.push(entry);
        self.revision = TextSnapshot::next_revision();
        self.prune_invalid_folds();
        true
    }

    pub fn fold_lines(&mut self, start_line: usize, end_line: usize) -> bool {
        if start_line + 1 >= end_line || end_line > self.line_count() {
            return false;
        }
        let before = self.folds.clone();
        let mut candidate = FoldRegion {
            start_line,
            end_line,
        };
        let mut retained = Vec::with_capacity(self.folds.len() + 1);
        for fold in self.folds.drain(..) {
            if candidate.start_line < fold.end_line && fold.start_line < candidate.end_line {
                candidate.start_line = candidate.start_line.min(fold.start_line);
                candidate.end_line = candidate.end_line.max(fold.end_line);
            } else {
                retained.push(fold);
            }
        }
        retained.push(candidate);
        retained.sort_by_key(|fold| fold.start_line);
        if retained == before {
            self.folds = before;
            return false;
        }
        self.folds = retained;
        self.rebuild_fold_projection();
        true
    }

    pub fn unfold_at_line(&mut self, logical_line: usize) -> bool {
        let before = self.folds.len();
        self.folds
            .retain(|fold| !(fold.start_line <= logical_line && logical_line < fold.end_line));
        if self.folds.len() == before {
            return false;
        }
        self.rebuild_fold_projection();
        true
    }

    pub fn unfold_all(&mut self) -> bool {
        if self.folds.is_empty() {
            return false;
        }
        self.folds.clear();
        self.fold_projection.clear();
        true
    }

    pub fn visible_line_count(&self) -> usize {
        let hidden = self
            .fold_projection
            .last()
            .map_or(0, |projection| projection.cumulative_hidden);
        self.line_count().saturating_sub(hidden)
    }

    pub fn logical_line_for_visible_row(&self, visible_row: usize) -> Option<usize> {
        if visible_row >= self.visible_line_count() {
            return None;
        }
        let projection_count = self
            .fold_projection
            .partition_point(|projection| projection.visible_start < visible_row);
        let hidden = projection_count
            .checked_sub(1)
            .map_or(0, |index| self.fold_projection[index].cumulative_hidden);
        Some(visible_row + hidden)
    }

    pub fn visible_row_for_logical_line(&self, logical_line: usize) -> Option<usize> {
        if logical_line >= self.line_count() {
            return None;
        }
        for fold in &self.folds {
            if fold.start_line < logical_line && logical_line < fold.end_line {
                return None;
            }
        }
        let hidden = self
            .folds
            .iter()
            .take_while(|fold| fold.end_line <= logical_line)
            .map(|fold| fold.hidden_line_count())
            .sum::<usize>();
        Some(logical_line.saturating_sub(hidden))
    }

    /// Return the row that visually represents a logical source line.
    ///
    /// A line hidden inside a fold is represented by that fold's visible
    /// header. This is used by accessibility geometry and reveal operations so
    /// a retained caret cannot be projected onto an unrelated first row.
    pub fn projected_visible_row_for_logical_line(&self, logical_line: usize) -> Option<usize> {
        self.visible_row_for_logical_line(logical_line).or_else(|| {
            self.folds
                .iter()
                .find(|fold| fold.start_line < logical_line && logical_line < fold.end_line)
                .and_then(|fold| self.visible_row_for_logical_line(fold.start_line))
        })
    }

    /// Materialize only a bounded vertical/horizontal viewport, its width in
    /// display cells. Overscan is caller-controlled and remains bounded
    /// independently of source size.
    pub fn viewport(
        &self,
        first_visible_row: usize,
        row_count: usize,
        first_cell: usize,
        cell_count: usize,
        row_overscan: usize,
        column_overscan: usize,
    ) -> Vec<ViewportLine> {
        let first_row = first_visible_row.saturating_sub(row_overscan);
        let end_row = first_visible_row
            .saturating_add(row_count)
            .saturating_add(row_overscan)
            .min(self.visible_line_count());
        let first_cell = first_cell.saturating_sub(column_overscan);
        let cell_count = cell_count.saturating_add(column_overscan.saturating_mul(2));
        (first_row..end_row)
            .filter_map(|visible_row| {
                let logical_line = self.logical_line_for_visible_row(visible_row)?;
                let content = self.line_content_char_range(logical_line)?;
                let total_columns = content.len();
                let (start, start_cell) = self.column_covering_cell(logical_line, first_cell);
                // A character is at least a cell wide, so as many characters
                // as cells cover the window.
                let end = start.saturating_add(cell_count).min(total_columns);
                let text = self
                    .text
                    .slice((content.start + start)..(content.start + end))
                    .to_string();
                let folded_line_count = self
                    .folds
                    .binary_search_by_key(&logical_line, |fold| fold.start_line)
                    .ok()
                    .map_or(0, |index| self.folds[index].hidden_line_count());
                Some(ViewportLine {
                    visible_row,
                    logical_line,
                    first_column: start,
                    first_cell: start_cell,
                    text,
                    total_columns,
                    folded_line_count,
                })
            })
            .collect()
    }

    /// Push onto `breaks`, in order, the columns where `line` breaks to fit
    /// rows `width` cells wide. A row ends after the last blank that fits,
    /// or, in a run without one, after the last character that fits. Blanks
    /// hang past the edge rather than start a row.
    pub fn wrap_columns(&self, line: usize, width: usize, breaks: &mut Vec<usize>) {
        let Some(content) = self.line_content_char_range(line) else {
            return;
        };
        let width = width.max(1);
        let text = self.text.slice(content.clone());
        if content.len() <= width && one_cell_a_character(text) {
            return;
        }
        let (mut start, mut start_cell) = (0, 0);
        let mut after_blank: Option<(usize, usize)> = None;
        let mut at = 0;
        for (column, character) in text.chars().enumerate() {
            let cells = char_cells(character, at);
            let blank = character.is_whitespace();
            while !blank && column > start && at + cells - start_cell > width {
                let (column_break, cell_break) = after_blank
                    .take()
                    .filter(|(after, _)| *after > start)
                    .unwrap_or((column, at));
                breaks.push(column_break);
                start = column_break;
                start_cell = cell_break;
            }
            if blank {
                after_blank = Some((column + 1, at + cells));
            }
            at += cells;
        }
    }

    /// Row `row` of a wrapped view: columns `columns` of `logical_line`.
    pub fn wrapped_row(
        &self,
        row: usize,
        logical_line: usize,
        columns: Range<usize>,
    ) -> Option<ViewportLine> {
        let content = self.line_content_char_range(logical_line)?;
        let total_columns = content.len();
        let start = columns.start.min(total_columns);
        let end = columns.end.clamp(start, total_columns);
        let folded_line_count = if start == 0 {
            self.folds
                .binary_search_by_key(&logical_line, |fold| fold.start_line)
                .ok()
                .map_or(0, |index| self.folds[index].hidden_line_count())
        } else {
            0
        };
        Some(ViewportLine {
            visible_row: row,
            logical_line,
            first_column: start,
            first_cell: self.cell_at(logical_line, start),
            text: self
                .text
                .slice((content.start + start)..(content.start + end))
                .to_string(),
            total_columns,
            folded_line_count,
        })
    }

    pub fn line_column_for_char(&self, char_index: usize) -> (usize, usize) {
        let char_index = char_index.min(self.text.len_chars());
        let line = self.text.char_to_line(char_index);
        (
            line,
            char_index.saturating_sub(self.text.line_to_char(line)),
        )
    }

    pub fn char_at_line_column(&self, line: usize, column: usize) -> usize {
        self.line_content_char_range(line)
            .map_or(self.text.len_chars(), |range| {
                range.start + column.min(range.len())
            })
    }

    pub fn line_content_char_range(&self, line: usize) -> Option<Range<usize>> {
        if line >= self.line_count() {
            return None;
        }
        let start = self.text.line_to_char(line);
        let raw_end = if line + 1 < self.line_count() {
            self.text.line_to_char(line + 1)
        } else {
            self.text.len_chars()
        };
        let slice = self.text.slice(start..raw_end);
        let break_chars = trailing_line_break_chars(slice);
        Some(start..raw_end.saturating_sub(break_chars))
    }

    pub fn line_slices(&self) -> impl Iterator<Item = ropey::RopeSlice<'_>> + '_ {
        self.text.lines()
    }

    /// The text, for a reader that compares it with a copy it kept: a clone
    /// shares the text's storage, so keeping one costs little.
    pub const fn rope(&self) -> &Rope {
        &self.text
    }

    fn commit_history(
        &mut self,
        before: Rope,
        before_selections: Vec<TextSelection>,
        bytes: usize,
    ) {
        Self::keep_selections_before_change(&mut self.published, self.revision, &before_selections);
        self.open_step = None;
        self.undo.push(HistoryEntry {
            before,
            before_selections,
            after: self.text.clone(),
            after_selections: self.selections.clone(),
            bytes,
        });
        // The oldest steps give way to the newest; the newest is always kept.
        let mut kept = self.undo.iter().map(|entry| entry.bytes).sum::<usize>();
        let dropped = self
            .undo
            .iter()
            .take(self.undo.len() - 1)
            .take_while(|entry| {
                let over = kept > self.history_byte_budget;
                if over {
                    kept -= entry.bytes;
                }
                over
            })
            .count();
        self.undo.drain(..dropped);
        self.redo.clear();
        self.revision = TextSnapshot::next_revision();
        self.prune_invalid_folds();
    }

    fn prune_invalid_folds(&mut self) {
        let line_count = self.line_count();
        self.folds
            .retain(|fold| fold.start_line + 1 < fold.end_line && fold.end_line <= line_count);
        self.rebuild_fold_projection();
    }

    fn rebuild_fold_projection(&mut self) {
        self.fold_projection.clear();
        let mut cumulative_hidden = 0;
        for fold in &self.folds {
            let visible_start = fold.start_line.saturating_sub(cumulative_hidden);
            cumulative_hidden += fold.hidden_line_count();
            self.fold_projection.push(FoldProjection {
                visible_start,
                cumulative_hidden,
            });
        }
        self.project_selections_to_visible_lines();
    }

    fn project_selections_to_visible_lines(&mut self) {
        let selections = self
            .selections
            .iter()
            .map(|selection| TextSelection {
                anchor: self.project_char_to_visible_line(selection.anchor),
                cursor: self.project_char_to_visible_line(selection.cursor),
            })
            .collect();
        self.selections = normalize_selections(selections, self.text.len_chars());
    }

    fn project_char_to_visible_line(&self, char_index: usize) -> usize {
        let (logical_line, column) = self.line_column_for_char(char_index);
        let Some(visible_row) = self.projected_visible_row_for_logical_line(logical_line) else {
            return char_index.min(self.text.len_chars());
        };
        let Some(projected_line) = self.logical_line_for_visible_row(visible_row) else {
            return char_index.min(self.text.len_chars());
        };
        if projected_line == logical_line {
            char_index.min(self.text.len_chars())
        } else {
            self.char_at_line_column(projected_line, column)
        }
    }
}

fn normalize_selections(selections: Vec<TextSelection>, maximum: usize) -> Vec<TextSelection> {
    let mut ranges = selections
        .into_iter()
        .map(|selection| {
            let selection = TextSelection {
                anchor: selection.anchor.min(maximum),
                cursor: selection.cursor.min(maximum),
            };
            (selection.range(), selection.anchor > selection.cursor)
        })
        .collect::<Vec<_>>();
    if ranges.is_empty() {
        return vec![TextSelection::caret(0)];
    }
    ranges.sort_by_key(|(range, _)| (range.start, range.end));
    let mut merged: Vec<(Range<usize>, bool)> = Vec::with_capacity(ranges.len());
    for (range, reverse) in ranges {
        if let Some((previous, _)) = merged.last_mut()
            && (range.start < previous.end
                || (range.start == range.end
                    && previous.start == previous.end
                    && range.start == previous.start))
        {
            previous.end = previous.end.max(range.end);
            continue;
        }
        merged.push((range, reverse));
    }
    merged
        .into_iter()
        .map(|(range, reverse)| {
            if reverse {
                TextSelection {
                    anchor: range.end,
                    cursor: range.start,
                }
            } else {
                TextSelection {
                    anchor: range.start,
                    cursor: range.end,
                }
            }
        })
        .collect()
}

/// The line break an edit inserts: the source's most common one, preferring
/// CRLF then LF on a tie, and LF when the source has none.
fn detect_preferred_newline(source: &Rope) -> &'static str {
    let mut lf = 0_usize;
    let mut crlf = 0_usize;
    let mut cr = 0_usize;
    let mut pending_cr = false;
    for chunk in source.chunks() {
        for &byte in chunk.as_bytes() {
            if pending_cr {
                pending_cr = false;
                if byte == b'\n' {
                    crlf += 1;
                    continue;
                }
                cr += 1;
            }
            match byte {
                b'\r' => pending_cr = true,
                b'\n' => lf += 1,
                _ => {}
            }
        }
    }
    cr += usize::from(pending_cr);
    if crlf >= lf && crlf >= cr && crlf > 0 {
        "\r\n"
    } else if cr > lf && cr > 0 {
        "\r"
    } else {
        "\n"
    }
}

fn trailing_line_break_chars(slice: ropey::RopeSlice<'_>) -> usize {
    let length = slice.len_chars();
    match length.checked_sub(1).map(|index| slice.char(index)) {
        Some('\n') => {
            1 + usize::from(
                length
                    .checked_sub(2)
                    .is_some_and(|index| slice.char(index) == '\r'),
            )
        }
        Some('\r' | '\u{000B}' | '\u{000C}' | '\u{0085}' | '\u{2028}' | '\u{2029}') => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Type `text` a character a keystroke, `gap` seconds apart from `at`.
    fn type_out(model: &mut TextDocumentModel, text: &str, at: f64, gap: f64) -> f64 {
        let mut now = at;
        for character in text.chars() {
            assert!(model.keystroke(Keystroke::Typing, &character.to_string(), now));
            now += gap;
        }
        now
    }

    /// Undo once at a time, keeping the text each step leaves.
    fn undo_all(model: &mut TextDocumentModel) -> Vec<String> {
        let mut texts = Vec::new();
        while model.undo() {
            texts.push(model.to_source());
        }
        texts
    }

    #[test]
    fn typing_is_undone_a_word_at_a_time() {
        let mut model = TextDocumentModel::from_source("");
        type_out(&mut model, "R1 in out 1k", 0.0, 0.1);
        assert_eq!(
            undo_all(&mut model),
            ["R1 in out", "R1 in", "R1", ""],
            "each word goes with the space before it"
        );
        while model.redo() {}
        assert_eq!(model.to_source(), "R1 in out 1k");
    }

    #[test]
    fn a_pause_or_a_caret_move_starts_a_new_step() {
        let mut model = TextDocumentModel::from_source("");
        let now = type_out(&mut model, "vdd", 0.0, 0.1);
        type_out(&mut model, "1", now + TYPING_PAUSE_SECONDS, 0.1);
        model.set_selections(vec![TextSelection::caret(0)]);
        type_out(&mut model, "V", now + TYPING_PAUSE_SECONDS + 0.2, 0.1);
        assert_eq!(undo_all(&mut model), ["vdd1", "vdd", ""]);
    }

    #[test]
    fn deleting_a_word_a_key_at_a_time_is_one_step_and_typing_after_it_another() {
        let mut model = TextDocumentModel::from_source("C1 a b 10p");
        model.set_selections(vec![TextSelection::caret(10)]);
        for step in 0..3 {
            assert!(model.keystroke(Keystroke::DeletingBackward, "", f64::from(step) * 0.1));
        }
        type_out(&mut model, "1n", 0.4, 0.1);
        assert_eq!(model.to_source(), "C1 a b 1n");
        assert_eq!(undo_all(&mut model), ["C1 a b ", "C1 a b 10p"]);
        assert_eq!(
            model.selections(),
            [TextSelection {
                anchor: 7,
                cursor: 10
            }],
            "undoing the deletions selects what they took"
        );
    }

    #[test]
    fn nothing_typed_at_normal_sizes_is_dropped_from_the_history() {
        let mut model = TextDocumentModel::from_source("");
        let mut now = 0.0;
        for word in 0..5_000 {
            now = type_out(&mut model, &format!("w{word} "), now, 0.05);
            // Each line a step of its own, as a caret move makes it.
            model.set_selections(model.selections().to_vec());
            now += TYPING_PAUSE_SECONDS;
        }
        let mut steps = 0;
        while model.undo() {
            steps += 1;
        }
        assert_eq!(model.to_source(), "", "every word typed can be undone");
        assert!(steps >= 5_000, "{steps} steps");
    }

    #[test]
    fn the_history_is_bounded_by_the_text_it_keeps_and_keeps_the_newest_step() {
        let mut model = TextDocumentModel::from_source("");
        model.history_byte_budget = 64 << 10;
        let block = "x".repeat(16 << 10);
        for _ in 0..8 {
            model.set_selections(vec![TextSelection::caret(model.len_chars())]);
            assert!(model.insert_at_selections(&block));
        }
        let kept = model.undo.iter().map(|entry| entry.bytes).sum::<usize>();
        assert!(kept <= 64 << 10, "{kept} bytes kept");
        assert_eq!(model.undo.len(), 3, "the oldest steps went first");
        assert!(model.undo());
        assert_eq!(
            model.len_chars(),
            7 * (16 << 10),
            "the newest step is undone"
        );

        model.history_byte_budget = 1;
        model.set_selections(vec![TextSelection::caret(0)]);
        assert!(model.insert_at_selections(&block));
        assert_eq!(
            model.undo.len(),
            1,
            "a step larger than the budget is still kept"
        );
    }

    #[test]
    fn bom_and_mixed_line_endings_round_trip_exactly() {
        let bytes = b"\xEF\xBB\xBFone\r\ntwo\nthree\r";
        let model = TextDocumentModel::from_bytes(bytes).unwrap();
        assert_eq!(model.encoding(), TextEncoding::Utf8Bom);
        assert_eq!(model.preferred_newline(), "\r\n");
        assert_eq!(model.encoded_bytes(), bytes);
    }

    #[test]
    fn invalid_utf8_reports_the_physical_byte_offset() {
        let error = TextDocumentModel::from_bytes(b"\xEF\xBB\xBFok\xFF").unwrap_err();
        assert_eq!(error, SourceTextError::InvalidUtf8 { valid_up_to: 5 });
    }

    #[test]
    fn multicursor_edit_is_one_exact_undoable_transaction() {
        let mut model = TextDocumentModel::from_source("alpha\nbeta\n");
        model.set_selections(vec![TextSelection::caret(0), TextSelection::caret(6)]);
        assert!(model.insert_at_selections("# "));
        assert_eq!(model.to_source(), "# alpha\n# beta\n");
        assert_ne!(
            model.revision(),
            TextDocumentModel::from_source("alpha\nbeta\n").revision(),
            "every text takes a revision of its own"
        );
        assert!(model.undo());
        assert_eq!(model.to_source(), "alpha\nbeta\n");
        assert!(model.redo());
        assert_eq!(model.to_source(), "# alpha\n# beta\n");
    }

    #[test]
    fn distinct_multicursor_replacements_preserve_order_and_one_step_undo() {
        let mut model = TextDocumentModel::from_source("one two three");
        model.set_selections(vec![
            TextSelection {
                anchor: 0,
                cursor: 3,
            },
            TextSelection {
                anchor: 8,
                cursor: 13,
            },
        ]);
        assert!(model.replace_at_selections(&["1".to_owned(), "3".to_owned()]));
        assert_eq!(model.to_source(), "1 two 3");
        assert!(model.undo());
        assert_eq!(model.to_source(), "one two three");
    }

    #[test]
    fn deletion_treats_crlf_as_one_editor_line_break() {
        let mut backward = TextDocumentModel::from_source("a\r\nb");
        backward.set_selections(vec![TextSelection::caret(3)]);
        assert!(backward.delete_backward());
        assert_eq!(backward.to_source(), "ab");

        let mut forward = TextDocumentModel::from_source("a\r\nb");
        forward.set_selections(vec![TextSelection::caret(1)]);
        assert!(forward.delete_forward());
        assert_eq!(forward.to_source(), "ab");
    }

    #[test]
    fn external_synchronization_clamps_selection_and_clears_local_history() {
        let mut model = TextDocumentModel::from_source("long source");
        model.set_selections(vec![TextSelection::caret(11)]);
        assert!(model.insert_at_selections("!"));
        assert!(model.synchronize_source("new"));
        assert_eq!(model.selections(), &[TextSelection::caret(3)]);
        assert!(!model.undo());
        assert_eq!(model.to_source(), "new");
        assert!(!model.synchronize_source("new"));
    }

    #[test]
    fn an_owner_step_taken_back_returns_the_history_of_the_text_it_replaced() {
        let mut model = TextDocumentModel::from_source("a");
        model.set_selections(vec![TextSelection::caret(1)]);
        model.insert_at_selections("b");
        assert_eq!(model.to_source(), "ab");

        // The owner formats the document: the new text has no history.
        assert!(model.synchronize_source(
            "a b
"
        ));
        assert!(!model.undo());

        // The owner undoes the format: the typing is undoable again, then
        // redoable, and the format's text finds what it had in turn.
        assert!(model.synchronize_source("ab"));
        assert!(model.undo());
        assert_eq!(model.to_source(), "a");
        assert!(model.redo());
        assert_eq!(model.to_source(), "ab");
    }

    #[test]
    fn an_edit_the_owner_refuses_leaves_the_history_and_the_selections_as_they_were() {
        let mut model = TextDocumentModel::from_source("a");
        model.set_selections(vec![TextSelection::caret(1)]);
        model.insert_at_selections("b");
        assert!(!model.synchronize_source("ab"), "the owner takes the edit");

        // The caret moves, then two edits the owner refuses.
        model.set_selections(vec![TextSelection::caret(0)]);
        model.insert_at_selections("q");
        model.insert_at_selections("r");
        assert_eq!(model.to_source(), "qrab");
        assert!(
            model.synchronize_source("ab"),
            "the owner publishes its text again"
        );

        assert_eq!(model.to_source(), "ab");
        assert_eq!(model.selections(), &[TextSelection::caret(0)]);
        assert!(
            model.undo(),
            "the edit before the refusal is still undoable"
        );
        assert_eq!(model.to_source(), "a");
        assert!(model.redo());
        assert_eq!(model.to_source(), "ab");
        assert!(!model.redo(), "the refused edits are gone");
    }

    #[test]
    fn a_refused_undo_leaves_undo_and_redo_as_they_were() {
        let mut model = TextDocumentModel::from_source("");
        model.insert_at_selections("a");
        model.insert_at_selections("b");
        assert!(!model.synchronize_source("ab"));
        assert!(model.undo());
        assert!(!model.synchronize_source("a"), "the owner takes the undo");

        assert!(model.undo());
        assert_eq!(model.to_source(), "");
        assert!(
            model.synchronize_source("a"),
            "the owner refuses the second"
        );
        assert!(model.redo(), "the first undo is still redoable");
        assert_eq!(model.to_source(), "ab");
        assert!(model.undo());
        assert!(model.undo(), "and the typing before it undoable");
        assert_eq!(model.to_source(), "");
    }

    #[test]
    fn the_owner_publishing_its_text_again_unchanged_copies_nothing() {
        let mut model = TextDocumentModel::from_source("a");
        model.set_selections(vec![TextSelection::caret(1)]);
        model.insert_at_selections("b");
        assert!(!model.synchronize_source("ab"));
        let kept = model
            .published
            .as_ref()
            .map(|published| published.undo.as_ptr());
        assert!(kept.is_some(), "the published text is kept");

        assert!(!model.synchronize_source("ab"));
        assert_eq!(
            model
                .published
                .as_ref()
                .map(|published| published.undo.as_ptr()),
            kept,
            "an unchanged text and history are not copied again"
        );
    }

    #[test]
    fn a_tab_runs_to_the_next_stop_and_a_wide_glyph_takes_two_cells() {
        let model = TextDocumentModel::from_source("a\tb\n\u{4E2D}x\n\t\t.end\n");
        assert_eq!(model.cell_at(0, 1), 1);
        assert_eq!(model.cell_at(0, 2), TAB_STOP, "b starts at the tab stop");
        assert_eq!(model.cell_at(1, 1), 2, "x after the wide glyph");
        assert_eq!(model.cell_at(2, 2), 2 * TAB_STOP);
        assert_eq!(model.cell_at(1, 5), 6, "past the end, a cell a column");
        assert_eq!(model.maximum_line_cells(), 2 * TAB_STOP + 4);
        // A pointer goes to the nearer edge of the character it is over.
        assert_eq!(model.column_nearest_cell(0, 4.0), 1);
        assert_eq!(model.column_nearest_cell(0, 5.0), 2);
        assert_eq!(model.column_nearest_cell(1, 0.9), 0);
        assert_eq!(model.column_nearest_cell(1, 1.1), 1);
        assert_eq!(model.column_nearest_cell(1, 9.0), 2);
    }

    #[test]
    fn a_column_selection_takes_the_same_cells_on_every_line() {
        let mut model = TextDocumentModel::from_source("\tR1 a\n        R2 b\n");
        model.set_column_selection(0, 8.0, 1, 10.0);
        assert_eq!(
            model.selections(),
            &[
                TextSelection {
                    anchor: 1,
                    cursor: 3
                },
                TextSelection {
                    anchor: 14,
                    cursor: 16
                },
            ],
            "R1 after the tab and R2 after eight spaces"
        );
    }

    #[test]
    fn column_selection_clamps_each_line_before_its_line_ending() {
        let mut model = TextDocumentModel::from_source("a\nlong\nxy\n");
        model.set_column_selection(0, 1.0, 2, 3.0);
        assert_eq!(
            model.selections(),
            &[
                TextSelection {
                    anchor: 1,
                    cursor: 1
                },
                TextSelection {
                    anchor: 3,
                    cursor: 5
                },
                TextSelection {
                    anchor: 8,
                    cursor: 9
                },
            ]
        );
        assert!(model.insert_at_selections("_"));
        assert_eq!(model.to_source(), "a_\nl_g\nx_\n");
    }

    #[test]
    fn folding_maps_visible_rows_to_logical_lines_without_materializing_hidden_text() {
        let mut model = TextDocumentModel::from_source("0\n1\n2\n3\n4\n5\n6\n");
        model.set_selections(vec![TextSelection::caret(model.char_at_line_column(3, 1))]);
        assert!(model.fold_lines(2, 5));
        assert_eq!(model.visible_line_count(), 6);
        assert_eq!(model.logical_line_for_visible_row(2), Some(2));
        assert_eq!(model.logical_line_for_visible_row(3), Some(5));
        assert_eq!(model.visible_row_for_logical_line(3), None);
        assert_eq!(model.projected_visible_row_for_logical_line(3), Some(2));
        assert_eq!(model.projected_visible_row_for_logical_line(5), Some(3));
        assert_eq!(
            model.line_column_for_char(model.selections()[0].cursor),
            (2, 1),
            "folding must not retain an invisible caret"
        );
        let viewport = model.viewport(1, 3, 0, 20, 0, 0);
        assert_eq!(
            viewport
                .iter()
                .map(|line| (
                    line.logical_line,
                    line.text.as_str(),
                    line.folded_line_count
                ))
                .collect::<Vec<_>>(),
            vec![(1, "1", 0), (2, "2", 2), (5, "5", 0)]
        );
        assert!(model.unfold_at_line(3));
        assert_eq!(model.visible_line_count(), model.line_count());
    }

    #[test]
    fn viewport_bounds_work_to_requested_rows_and_columns() {
        let source = (0..20)
            .map(|line| format!("{line:02}-abcdefghijklmnopqrstuvwxyz\n"))
            .collect::<String>();
        let model = TextDocumentModel::from_source(&source);
        let viewport = model.viewport(10, 4, 8, 5, 1, 2);
        assert_eq!(viewport.len(), 6);
        assert_eq!(viewport.first().unwrap().logical_line, 9);
        assert_eq!(viewport.last().unwrap().logical_line, 14);
        assert!(viewport.iter().all(|line| line.text.chars().count() <= 9));
        assert_eq!(model.maximum_line_cells_in_range(10, 4), 29);
        assert_eq!(model.maximum_line_cells_in_range(99, 4), 0);
    }

    #[test]
    fn five_million_line_document_materializes_only_four_hundred_visible_rows() {
        let source = "x\n".repeat(5_000_000);
        let model = TextDocumentModel::from_source(&source);
        assert_eq!(model.line_count(), 5_000_001);
        let viewport = model.viewport(2_499_800, 400, 0, 80, 0, 0);
        assert_eq!(viewport.len(), 400);
        assert_eq!(viewport.first().unwrap().logical_line, 2_499_800);
        assert_eq!(viewport.last().unwrap().logical_line, 2_500_199);
    }

    #[test]
    fn a_line_wraps_after_the_last_blank_that_fits_and_a_long_word_where_it_must() {
        let model = TextDocumentModel::from_source(
            "alpha beta gamma delta\nabcdefghijklmnopqrstuvwxy\nshort\n",
        );
        let breaks = |line| {
            let mut breaks = Vec::new();
            model.wrap_columns(line, 10, &mut breaks);
            breaks
        };
        // "alpha beta " | "gamma " | "delta": a blank hangs past the edge.
        assert_eq!(breaks(0), vec![11, 17]);
        assert_eq!(breaks(1), vec![10, 20], "a word wider than a row is cut");
        assert!(breaks(2).is_empty(), "a line that fits stays whole");
        let row = model.wrapped_row(1, 0, 11..17).expect("a row of the line");
        assert_eq!((row.text.as_str(), row.first_cell), ("gamma ", 11));
    }
}
