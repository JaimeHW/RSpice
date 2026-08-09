//! Scalable, language-neutral source document model.
//!
//! EGUI's stock `TextEdit` remains useful for small buffers and IME plumbing,
//! but it materializes one galley for the entire document and owns only one
//! selection. This model is the backing authority for the virtualized editor:
//! it keeps large edits logarithmic, represents multiple/column selections,
//! stores fold projection, and makes undo/redo exact and transactional.

use std::ops::Range;

use ropey::Rope;

const DEFAULT_HISTORY_LIMIT: usize = 512;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextEncoding {
    Utf8,
    Utf8Bom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LineEnding {
    None,
    Lf,
    CrLf,
    Cr,
    Mixed,
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
    original_line_ending: LineEnding,
    preferred_newline: &'static str,
    selections: Vec<TextSelection>,
    folds: Vec<FoldRegion>,
    fold_projection: Vec<FoldProjection>,
    undo: Vec<HistoryEntry>,
    redo: Vec<HistoryEntry>,
    history_limit: usize,
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
        let (original_line_ending, preferred_newline) = detect_line_endings(source);
        Ok(Self {
            text: Rope::from_str(source),
            #[cfg(test)]
            encoding,
            original_line_ending,
            preferred_newline,
            selections: vec![TextSelection::caret(0)],
            folds: Vec::new(),
            fold_projection: Vec::new(),
            undo: Vec::new(),
            redo: Vec::new(),
            history_limit: DEFAULT_HISTORY_LIMIT,
            revision: 0,
        })
    }

    pub fn from_source(source: &str) -> Self {
        Self::from_bytes(source.as_bytes()).expect("Rust strings are valid UTF-8")
    }

    #[cfg(test)]
    pub const fn encoding(&self) -> TextEncoding {
        self.encoding
    }

    #[cfg(test)]
    pub const fn original_line_ending(&self) -> LineEnding {
        self.original_line_ending
    }

    pub const fn preferred_newline(&self) -> &'static str {
        self.preferred_newline
    }

    pub const fn revision(&self) -> u64 {
        self.revision
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
    pub fn maximum_line_columns(&self) -> usize {
        self.maximum_line_columns_in_range(0, self.line_count())
    }

    pub fn maximum_line_columns_in_range(&self, start_line: usize, line_count: usize) -> usize {
        if start_line >= self.line_count() || line_count == 0 {
            return 0;
        }
        self.text
            .lines_at(start_line)
            .take(line_count.min(self.line_count() - start_line))
            .map(|line| {
                line.len_chars()
                    .saturating_sub(trailing_line_break_chars(line))
            })
            .max()
            .unwrap_or(0)
    }

    pub fn maximum_selected_line_columns(&self) -> usize {
        self.selections
            .iter()
            .map(|selection| {
                let first = self.line_column_for_char(selection.range().start).0;
                let last = self.line_column_for_char(selection.range().end).0;
                self.maximum_line_columns_in_range(first, last.saturating_sub(first) + 1)
            })
            .max()
            .unwrap_or(0)
    }

    pub fn selections(&self) -> &[TextSelection] {
        &self.selections
    }

    #[cfg(test)]
    pub fn folds(&self) -> &[FoldRegion] {
        &self.folds
    }

    pub fn to_source(&self) -> String {
        self.text.to_string()
    }

    /// Compare against an externally owned UTF-8 buffer without first
    /// materializing another complete `String` from the rope.
    pub fn source_equals(&self, source: &str) -> bool {
        if self.text.len_bytes() != source.len() {
            return false;
        }
        let mut offset = 0;
        for chunk in self.text.chunks() {
            let end = offset + chunk.len();
            if source.get(offset..end) != Some(chunk) {
                return false;
            }
            offset = end;
        }
        true
    }

    /// Accept a source revision published by the application layer. External
    /// replacement is deliberately not added to the local undo stack: the
    /// project/document service owns undo and conflict semantics across that
    /// boundary. Cursor locations are retained and clamped when possible.
    pub fn synchronize_source(&mut self, source: &str) -> bool {
        if self.source_equals(source) {
            return false;
        }
        self.text = Rope::from_str(source);
        let (line_ending, preferred_newline) = detect_line_endings(source);
        self.original_line_ending = line_ending;
        self.preferred_newline = preferred_newline;
        self.selections =
            normalize_selections(std::mem::take(&mut self.selections), self.text.len_chars());
        self.undo.clear();
        self.redo.clear();
        self.revision = self.revision.wrapping_add(1);
        self.prune_invalid_folds();
        true
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

    /// Create one selection per logical line between the two endpoints.
    /// Columns are Unicode-scalar columns and clamp before the line ending.
    pub fn set_column_selection(
        &mut self,
        anchor_line: usize,
        anchor_column: usize,
        cursor_line: usize,
        cursor_column: usize,
    ) {
        let last_line = self.line_count().saturating_sub(1);
        let first = anchor_line.min(cursor_line).min(last_line);
        let last = anchor_line.max(cursor_line).min(last_line);
        let forward = (cursor_line, cursor_column) >= (anchor_line, anchor_column);
        let mut selections = Vec::with_capacity(last.saturating_sub(first) + 1);
        for line in first..=last {
            let anchor = self.char_at_line_column(line, anchor_column);
            let cursor = self.char_at_line_column(line, cursor_column);
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
        for (index, range) in edits.iter().enumerate().rev() {
            let replacement = if replacements.len() == 1 {
                &replacements[0]
            } else {
                &replacements[index]
            };
            if !range.is_empty() {
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
        self.commit_history(before, before_selections);
        true
    }

    /// Commit a complete locally produced source transformation (formatting,
    /// line movement, normalization, or another structural edit) as one exact
    /// undo step while retaining explicitly mapped cursor selections.
    pub fn replace_source_transaction(
        &mut self,
        source: &str,
        selections: Vec<TextSelection>,
    ) -> bool {
        if self.source_equals(source) {
            self.set_selections(selections);
            return false;
        }
        let before = self.text.clone();
        let before_selections = self.selections.clone();
        self.text = Rope::from_str(source);
        let (line_ending, preferred_newline) = detect_line_endings(source);
        self.original_line_ending = line_ending;
        self.preferred_newline = preferred_newline;
        self.selections = normalize_selections(selections, self.text.len_chars());
        self.commit_history(before, before_selections);
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

    pub fn undo(&mut self) -> bool {
        let Some(entry) = self.undo.pop() else {
            return false;
        };
        self.text = entry.before.clone();
        self.selections = entry.before_selections.clone();
        self.redo.push(entry);
        self.revision = self.revision.wrapping_add(1);
        self.prune_invalid_folds();
        true
    }

    pub fn redo(&mut self) -> bool {
        let Some(entry) = self.redo.pop() else {
            return false;
        };
        self.text = entry.after.clone();
        self.selections = entry.after_selections.clone();
        self.undo.push(entry);
        self.revision = self.revision.wrapping_add(1);
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

    /// Materialize only a bounded vertical/horizontal viewport. Overscan is
    /// caller-controlled and remains bounded independently of source size.
    pub fn viewport(
        &self,
        first_visible_row: usize,
        row_count: usize,
        first_column: usize,
        column_count: usize,
        row_overscan: usize,
        column_overscan: usize,
    ) -> Vec<ViewportLine> {
        let first_row = first_visible_row.saturating_sub(row_overscan);
        let end_row = first_visible_row
            .saturating_add(row_count)
            .saturating_add(row_overscan)
            .min(self.visible_line_count());
        let first_column = first_column.saturating_sub(column_overscan);
        let column_count = column_count.saturating_add(column_overscan.saturating_mul(2));
        (first_row..end_row)
            .filter_map(|visible_row| {
                let logical_line = self.logical_line_for_visible_row(visible_row)?;
                let content = self.line_content_char_range(logical_line)?;
                let total_columns = content.len();
                let start = first_column.min(total_columns);
                let end = start.saturating_add(column_count).min(total_columns);
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
                    text,
                    total_columns,
                    folded_line_count,
                })
            })
            .collect()
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

    fn commit_history(&mut self, before: Rope, before_selections: Vec<TextSelection>) {
        self.undo.push(HistoryEntry {
            before,
            before_selections,
            after: self.text.clone(),
            after_selections: self.selections.clone(),
        });
        if self.undo.len() > self.history_limit {
            self.undo.remove(0);
        }
        self.redo.clear();
        self.revision = self.revision.wrapping_add(1);
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

fn detect_line_endings(source: &str) -> (LineEnding, &'static str) {
    let bytes = source.as_bytes();
    let mut lf = 0_usize;
    let mut crlf = 0_usize;
    let mut cr = 0_usize;
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'\r' if bytes.get(index + 1) == Some(&b'\n') => {
                crlf += 1;
                index += 2;
            }
            b'\r' => {
                cr += 1;
                index += 1;
            }
            b'\n' => {
                lf += 1;
                index += 1;
            }
            _ => index += 1,
        }
    }
    let styles = usize::from(lf > 0) + usize::from(crlf > 0) + usize::from(cr > 0);
    let kind = match (styles, lf > 0, crlf > 0, cr > 0) {
        (0, _, _, _) => LineEnding::None,
        (1, true, _, _) => LineEnding::Lf,
        (1, _, true, _) => LineEnding::CrLf,
        (1, _, _, true) => LineEnding::Cr,
        _ => LineEnding::Mixed,
    };
    let preferred = if crlf >= lf && crlf >= cr && crlf > 0 {
        "\r\n"
    } else if cr > lf && cr > 0 {
        "\r"
    } else {
        "\n"
    };
    (kind, preferred)
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

    #[test]
    fn bom_and_mixed_line_endings_round_trip_exactly() {
        let bytes = b"\xEF\xBB\xBFone\r\ntwo\nthree\r";
        let model = TextDocumentModel::from_bytes(bytes).unwrap();
        assert_eq!(model.encoding(), TextEncoding::Utf8Bom);
        assert_eq!(model.original_line_ending(), LineEnding::Mixed);
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
        assert_eq!(model.revision(), 1);
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
        assert!(model.source_equals("new"));
        assert!(!model.synchronize_source("new"));
    }

    #[test]
    fn column_selection_clamps_each_line_before_its_line_ending() {
        let mut model = TextDocumentModel::from_source("a\nlong\nxy\n");
        model.set_column_selection(0, 1, 2, 3);
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
        assert_eq!(model.maximum_line_columns_in_range(10, 4), 29);
        assert_eq!(model.maximum_line_columns_in_range(99, 4), 0);
    }

    #[test]
    #[ignore = "release-scale five-million-line qualification"]
    fn five_million_line_document_materializes_only_four_hundred_visible_rows() {
        let source = "x\n".repeat(5_000_000);
        let model = TextDocumentModel::from_source(&source);
        assert_eq!(model.line_count(), 5_000_001);
        let viewport = model.viewport(2_499_800, 400, 0, 80, 0, 0);
        assert_eq!(viewport.len(), 400);
        assert_eq!(viewport.first().unwrap().logical_line, 2_499_800);
        assert_eq!(viewport.last().unwrap().logical_line, 2_500_199);
    }
}
