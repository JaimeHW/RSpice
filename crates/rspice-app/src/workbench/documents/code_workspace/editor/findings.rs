//! The findings a source editor shows, kept through the edits made since
//! the check that found them.
//!
//! A check reports findings for one text. When the text changes the editor
//! keeps showing them, dimmed as outdated, with each one moved through the
//! edit: text inserted before a finding pushes it along, and a finding whose
//! text was deleted goes away with it. The next check replaces them all.

use std::collections::HashMap;
use std::ops::Range;
use std::sync::Arc;

use rspice_vocabulary::diagnostics::DiagnosticSeverity;

use crate::state::code_diagnostics::CodeDiagnosticView;
use crate::workbench::documents::virtual_text_editor::SourceMark;

/// One finding, placed in the held text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HeldFinding {
    pub severity: DiagnosticSeverity,
    pub code: Arc<str>,
    pub message: Arc<str>,
    /// The bytes of the held text the finding covers. A finding that names
    /// a place but no extent covers the word there, and is empty only at the
    /// end of a line.
    pub range: Range<usize>,
}

/// The findings of one editor and the text they are placed in.
#[derive(Debug, Clone, Default)]
pub(crate) struct HeldFindings {
    text: String,
    /// The document revision `text` is.
    revision: Option<u64>,
    /// The collection the findings were read from, while they are its own.
    origin: Option<usize>,
    outdated: bool,
    /// In source order.
    findings: Vec<HeldFinding>,
    line_starts: Vec<usize>,
    marks: Vec<SourceMark>,
    /// The most severe finding that starts on each zero-based line, the
    /// first in source order among equals.
    worst_by_line: HashMap<usize, usize>,
}

impl HeldFindings {
    /// Take the findings a check reported for `source` itself: the editor's
    /// text is the text the check read. `origin` names the collection, so a
    /// collection already taken is not read again.
    pub(crate) fn take_current(
        &mut self,
        source: &str,
        revision: u64,
        origin: usize,
        view: &CodeDiagnosticView<'_>,
    ) {
        if self.origin == Some(origin)
            && self.revision == Some(revision)
            && !self.outdated
            && self.text == source
        {
            return;
        }
        source.clone_into(&mut self.text);
        self.revision = Some(revision);
        self.origin = Some(origin);
        self.outdated = false;
        self.line_starts = line_starts(source);
        let mut findings = view
            .located()
            .into_iter()
            .map(|diagnostic| HeldFinding {
                severity: diagnostic.severity,
                code: Arc::clone(&diagnostic.canonical.code),
                message: Arc::clone(&diagnostic.message),
                range: self.place(
                    diagnostic.byte_range.clone(),
                    diagnostic.line,
                    diagnostic.column,
                ),
            })
            .collect::<Vec<_>>();
        findings.sort_by_key(|finding| (finding.range.start, finding.range.end));
        self.findings = findings;
        self.index();
    }

    /// The editor's text is `source` now, which no check has read yet: keep
    /// the findings held, moved through whatever changed.
    pub(crate) fn keep_through_edits(&mut self, source: &str, revision: u64) {
        self.revision = Some(revision);
        self.origin = None;
        self.outdated = true;
        if self.text == source {
            return;
        }
        if !self.findings.is_empty() {
            let edit = TextEdit::between(&self.text, source);
            self.findings.retain_mut(|finding| {
                edit.map(finding.range.clone())
                    .map(|range| finding.range = range)
                    .is_some()
            });
        }
        source.clone_into(&mut self.text);
        self.line_starts = line_starts(source);
        self.index();
    }

    /// True when the text changed after the check that found these.
    pub(crate) const fn is_outdated(&self) -> bool {
        self.outdated
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.findings.is_empty()
    }

    /// The stretch of each line every finding covers, by line.
    pub(crate) fn marks(&self) -> &[SourceMark] {
        &self.marks
    }

    /// The most severe finding that starts on a zero-based line.
    pub(crate) fn worst_on_line(&self, line: usize) -> Option<&HeldFinding> {
        self.worst_by_line
            .get(&line)
            .map(|index| &self.findings[*index])
    }

    /// The findings, in source order.
    pub(crate) fn iter(&self) -> std::slice::Iter<'_, HeldFinding> {
        self.findings.iter()
    }

    /// The findings that cover the character at `char_index`, the most
    /// severe first.
    pub(crate) fn covering(&self, char_index: usize) -> Vec<&HeldFinding> {
        let byte = self.byte_of_char(char_index);
        let mut covering = self
            .findings
            .iter()
            .filter(|finding| {
                finding.range.contains(&byte)
                    || (finding.range.is_empty() && finding.range.start == byte)
            })
            .collect::<Vec<_>>();
        covering.sort_by(|left, right| right.severity.cmp(&left.severity));
        covering
    }

    /// What a reader at `char_index` asks about: the finding under it, or
    /// else the most severe on its line.
    pub(crate) fn at_caret(&self, char_index: usize) -> Option<&HeldFinding> {
        self.covering(char_index).first().copied().or_else(|| {
            let (line, _) = self.line_column(self.byte_of_char(char_index));
            self.worst_on_line(line)
        })
    }

    /// The zero-based line and character column of a byte of the held text.
    pub(crate) fn line_column(&self, byte: usize) -> (usize, usize) {
        let line = self
            .line_starts
            .partition_point(|start| *start <= byte)
            .saturating_sub(1);
        let start = self.line_starts.get(line).copied().unwrap_or(0);
        let column = self
            .text
            .get(start..byte.min(self.text.len()))
            .map_or(0, |text| text.chars().count());
        (line, column)
    }

    fn byte_of_char(&self, char_index: usize) -> usize {
        self.text
            .char_indices()
            .nth(char_index)
            .map_or(self.text.len(), |(byte, _)| byte)
    }

    /// Where a finding sits in the held text: its own byte range when that
    /// agrees with the line and column it names, or else the word at that
    /// line and column.
    fn place(
        &self,
        range: Option<Range<usize>>,
        line: Option<usize>,
        column: Option<usize>,
    ) -> Range<usize> {
        let text = &self.text;
        if let Some(range) = range.filter(|range| {
            range.start <= range.end
                && range.end <= text.len()
                && text.is_char_boundary(range.start)
                && text.is_char_boundary(range.end)
        }) {
            let (at_line, at_column) = self.line_column(range.start);
            if line.is_none_or(|line| line == at_line + 1)
                && column.is_none_or(|column| column == at_column + 1)
            {
                return self.widen(range);
            }
        }
        let line = line.unwrap_or(1).saturating_sub(1);
        let Some(&start) = self.line_starts.get(line) else {
            return text.len()..text.len();
        };
        let end = self
            .line_starts
            .get(line + 1)
            .map_or(text.len(), |next| next - 1);
        let body = &text[start..end];
        let body = body.strip_suffix('\r').unwrap_or(body);
        let offset = body
            .char_indices()
            .nth(column.unwrap_or(1).saturating_sub(1))
            .map_or(body.len(), |(offset, _)| offset);
        self.widen(start + offset..start + offset)
    }

    /// A range that names a place and no extent covers the word there, or
    /// the one character there when it starts no word.
    fn widen(&self, range: Range<usize>) -> Range<usize> {
        if !range.is_empty() {
            return range;
        }
        let rest = &self.text[range.start..];
        let word = rest
            .char_indices()
            .find(|(_, character)| !(character.is_alphanumeric() || matches!(character, '_' | '$')))
            .map_or(rest.len(), |(offset, _)| offset);
        let length = match word {
            0 => rest
                .chars()
                .next()
                .filter(|character| !matches!(character, '\n' | '\r'))
                .map_or(0, char::len_utf8),
            word => word,
        };
        range.start..range.start + length
    }

    /// Rebuild the marks and the line index from the findings.
    fn index(&mut self) {
        self.worst_by_line.clear();
        self.marks.clear();
        for (index, finding) in self.findings.iter().enumerate() {
            let (first_line, first_column) = self.line_column(finding.range.start);
            let (last_line, last_column) = self.line_column(finding.range.end);
            self.worst_by_line
                .entry(first_line)
                .and_modify(|worst| {
                    if self.findings[*worst].severity < finding.severity {
                        *worst = index;
                    }
                })
                .or_insert(index);
            for line in first_line..=last_line {
                let start = if line == first_line { first_column } else { 0 };
                let end = if line == last_line {
                    last_column
                } else {
                    self.line_width(line)
                };
                self.marks.push(SourceMark {
                    line,
                    columns: start..end.max(start),
                    severity: finding.severity,
                });
            }
        }
        self.marks
            .sort_by_key(|mark| (mark.line, mark.columns.start, mark.columns.end));
    }

    /// The width of a zero-based line in characters, without its ending.
    fn line_width(&self, line: usize) -> usize {
        let start = self.line_starts[line];
        let end = self
            .line_starts
            .get(line + 1)
            .map_or(self.text.len(), |next| next - 1);
        let body = &self.text[start..end];
        body.strip_suffix('\r').unwrap_or(body).chars().count()
    }
}

/// The byte each line of `text` starts at.
fn line_starts(text: &str) -> Vec<usize> {
    std::iter::once(0)
        .chain(text.match_indices('\n').map(|(index, _)| index + 1))
        .collect()
}

/// One stretch of an old text replaced by one stretch of a new text: what
/// the two share before and after it is unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TextEdit {
    /// Where the replaced stretch starts, in both texts.
    start: usize,
    /// Where it ends in the old text.
    old_end: usize,
    /// Where its replacement ends in the new text.
    new_end: usize,
}

impl TextEdit {
    fn between(old: &str, new: &str) -> Self {
        let mut start = old
            .bytes()
            .zip(new.bytes())
            .take_while(|(left, right)| left == right)
            .count();
        while !old.is_char_boundary(start) {
            start -= 1;
        }
        let room = old.len().min(new.len()) - start;
        let mut shared_end = old
            .bytes()
            .rev()
            .zip(new.bytes().rev())
            .take(room)
            .take_while(|(left, right)| left == right)
            .count();
        while !old.is_char_boundary(old.len() - shared_end) {
            shared_end -= 1;
        }
        let edit = Self {
            start,
            old_end: old.len() - shared_end,
            new_end: new.len() - shared_end,
        };
        // Lines added or deleted whole can be read as starting anywhere
        // their text repeats around them: a deleted line read from its
        // second character would leave a finding on its first behind.
        if edit.old_end == edit.start && edit.new_end > edit.start {
            edit.at_line_start(new.as_bytes(), edit.new_end - edit.start)
        } else if edit.new_end == edit.start && edit.old_end > edit.start {
            edit.at_line_start(old.as_bytes(), edit.old_end - edit.start)
        } else {
            edit
        }
    }

    /// The same insertion or deletion of `length` bytes of `text`, moved to
    /// the line start nearest where it was found, when it can start there.
    fn at_line_start(self, text: &[u8], length: usize) -> Self {
        let (mut first, mut last) = (self.start, self.start);
        while first > 0 && text[first - 1] == text[first - 1 + length] {
            first -= 1;
        }
        while last + length < text.len() && text[last] == text[last + length] {
            last += 1;
        }
        let line_start = |at: usize| at == 0 || text[at - 1] == b'\n';
        let Some(start) = (first..=last)
            .filter(|at| line_start(*at))
            .min_by_key(|at| at.abs_diff(self.start))
        else {
            return self;
        };
        Self {
            start,
            old_end: start + (self.old_end - self.start),
            new_end: start + (self.new_end - self.start),
        }
    }

    /// Where `range` of the old text is in the new one: unchanged before the
    /// edit, moved along after it, cut back where the edit took part of it,
    /// and gone when the edit took all of it.
    fn map(self, range: Range<usize>) -> Option<Range<usize>> {
        let moved = |offset: usize| offset - self.old_end + self.new_end;
        if range.end <= self.start {
            return Some(range);
        }
        if range.start >= self.old_end {
            return Some(moved(range.start)..moved(range.end));
        }
        if range.start >= self.start && range.end <= self.old_end {
            return None;
        }
        let start = if range.start < self.start {
            range.start
        } else {
            self.new_end
        };
        let end = if range.end > self.old_end {
            moved(range.end)
        } else {
            self.start
        };
        (start < end).then_some(start..end)
    }
}

#[cfg(test)]
mod tests;
