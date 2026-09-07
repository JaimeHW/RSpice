//! Source location tracking and source map management
//!
//! This module provides infrastructure for tracking source locations throughout
//! the compilation pipeline, enabling precise error messages and source maps.

use std::path::PathBuf;

/// Unique identifier for a source file
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceId(u32);

impl SourceId {
    /// Create a new source ID (internal use only)
    pub(crate) fn new(id: u32) -> Self {
        Self(id)
    }

    /// Get the raw ID value
    pub fn raw(self) -> u32 {
        self.0
    }
}

/// A span in the source code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    /// Source file this span belongs to
    pub source: SourceId,
    /// Start byte offset (inclusive)
    pub start: u32,
    /// End byte offset (exclusive)
    pub end: u32,
}

impl Span {
    /// Create a new span
    pub fn new(source: SourceId, start: u32, end: u32) -> Self {
        Self { source, start, end }
    }

    /// Create a dummy span for synthesized code
    pub fn dummy() -> Self {
        Self {
            source: SourceId(0),
            start: 0,
            end: 0,
        }
    }

    /// Extend this span to include another span
    pub fn extend(self, other: Span) -> Self {
        debug_assert_eq!(self.source, other.source);
        Self {
            source: self.source,
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    /// Get the length in bytes. A reversed range contains no source text.
    pub fn len(self) -> u32 {
        self.end.saturating_sub(self.start)
    }

    /// Check if this span is empty
    pub fn is_empty(self) -> bool {
        self.start >= self.end
    }
}

/// Information about a source file
#[derive(Debug, Clone)]
pub struct SourceInfo {
    /// File path (or "<input>" for inline sources)
    pub path: PathBuf,
    /// Immutable text keeps the cached line boundaries valid, including on a
    /// cloned source document.
    content: String,
    /// Line start byte offsets (for efficient line lookup)
    line_starts: Vec<usize>,
}

impl SourceInfo {
    /// The exact text registered for this source document.
    pub fn content(&self) -> &str {
        &self.content
    }

    fn new(path: impl Into<PathBuf>, content: String) -> Self {
        let line_starts = std::iter::once(0)
            .chain(
                content
                    .bytes()
                    .enumerate()
                    .filter(|(_, b)| *b == b'\n')
                    .map(|(i, _)| i + 1),
            )
            .collect();

        Self {
            path: path.into(),
            content,
            line_starts,
        }
    }

    /// Get the one-based line and Unicode scalar column for a byte offset.
    /// Invalid offsets, including the middle of a UTF-8 character, have no
    /// source position. The end of the file is a valid position.
    pub fn line_col(&self, offset: u32) -> Option<(u32, u32)> {
        let offset = usize::try_from(offset).ok()?;
        if offset > self.content.len() || !self.content.is_char_boundary(offset) {
            return None;
        }
        let line = self
            .line_starts
            .partition_point(|&start| start <= offset)
            .saturating_sub(1);
        let col = self.content[self.line_starts[line]..offset].chars().count();
        Some((
            u32::try_from(line).ok()?.checked_add(1)?,
            u32::try_from(col).ok()?.checked_add(1)?,
        ))
    }

    /// Get the line content for a given line number (1-indexed)
    pub fn get_line(&self, line: u32) -> Option<&str> {
        let idx = usize::try_from(line.checked_sub(1)?).ok()?;
        if idx >= self.line_starts.len() {
            return None;
        }

        let start = self.line_starts[idx];
        let end = self
            .line_starts
            .get(idx + 1)
            .copied()
            .unwrap_or(self.content.len());

        Some(self.content[start..end].trim_end_matches(['\r', '\n']))
    }
}

/// Source map for tracking all source files in a compilation
#[derive(Debug, Default)]
pub struct SourceMap {
    // IDs are dense and sources are never removed. A vector retains stable
    // identities without an unused atomic counter or a separate hash index.
    sources: Vec<SourceInfo>,
}

impl SourceMap {
    /// Create a new empty source map
    pub fn new() -> Self {
        Self::default()
    }

    /// Retain a source file and return its identity in this map.
    pub fn add_source(&mut self, path: impl Into<PathBuf>, content: impl Into<String>) -> SourceId {
        let id = SourceId::new(
            u32::try_from(self.sources.len()).expect("source map exhausted its identifier space"),
        );
        self.sources.push(SourceInfo::new(path, content.into()));
        id
    }

    /// Compatibility spelling of [`Self::add_source`].
    pub fn add_source_mut(
        &mut self,
        path: impl Into<PathBuf>,
        content: impl Into<String>,
    ) -> SourceId {
        self.add_source(path, content)
    }

    /// Get source info for a source ID
    pub fn get(&self, id: SourceId) -> Option<&SourceInfo> {
        self.sources.get(id.raw() as usize)
    }

    /// Format a span for error display
    pub fn format_span(&self, span: Span) -> String {
        if let Some(source) = self.get(span.source)
            && let Some((line, col)) = source.line_col(span.start)
        {
            format!("{}:{}:{}", source.path.display(), line, col)
        } else {
            format!("<unknown>:{}-{}", span.start, span.end)
        }
    }

    /// Get the source text for a span
    pub fn span_text(&self, span: Span) -> Option<&str> {
        self.get(span.source)
            .and_then(|source| source.content.get(span.start as usize..span.end as usize))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registering_sources_preserves_text_paths_and_distinct_ids() {
        let mut map = SourceMap::new();
        let first = map.add_source("first.va", "module first;\nendmodule\n");
        let second = map.add_source_mut("second.va", "module second;\nendmodule\n");
        assert_ne!(first, second);
        let first_source = map.get(first).expect("registered source must be retained");
        assert_eq!(first_source.path, PathBuf::from("first.va"));
        assert_eq!(first_source.content(), "module first;\nendmodule\n");
        assert_eq!(first_source.get_line(1), Some("module first;"));
        assert_eq!(map.get(second).unwrap().get_line(1), Some("module second;"));
        assert_eq!(map.span_text(Span::new(first, 7, 12)), Some("first"));
        assert_eq!(map.format_span(Span::new(first, 14, 23)), "first.va:2:1");
    }

    #[test]
    fn malformed_span_ranges_are_not_source_text() {
        let mut map = SourceMap::new();
        let id = map.add_source_mut("utf8.va", "αβ\n");
        for (start, end) in [(3, 2), (0, 20), (20, 20), (1, 2), (0, 3)] {
            assert_eq!(map.span_text(Span::new(id, start, end)), None);
        }
        assert_eq!(map.span_text(Span::new(id, 0, 2)), Some("α"));
        assert_eq!(map.span_text(Span::new(id, 5, 5)), Some(""));
        assert_eq!(map.span_text(Span::new(SourceId::new(9), 0, 0)), None);
        let reversed = Span::new(id, u32::MAX, 0);
        assert!(reversed.is_empty());
        assert_eq!(reversed.len(), 0);
        assert_eq!(Span::new(id, 0, 2).len(), 2);
    }

    #[test]
    fn line_zero_and_lines_past_eof_are_absent() {
        let source = SourceInfo::new("empty.va", String::new());
        assert_eq!(source.get_line(0), None);
        assert_eq!(source.get_line(1), Some(""));
        assert_eq!(source.get_line(2), None);
        assert_eq!(source.get_line(u32::MAX), None);
    }

    #[test]
    fn locations_count_characters_and_validate_byte_boundaries() {
        let mut map = SourceMap::new();
        let id = map.add_source("utf8.va", "αβ\r\nγ");
        let source = map.get(id).unwrap();
        assert_eq!(source.line_col(0), Some((1, 1)));
        assert_eq!(source.line_col(2), Some((1, 2)));
        assert_eq!(source.line_col(6), Some((2, 1)));
        assert_eq!(source.line_col(8), Some((2, 2)));
        for offset in [1, 3, 7, 9, u32::MAX] {
            assert_eq!(source.line_col(offset), None);
        }
        assert_eq!(source.get_line(1), Some("αβ"));
        assert_eq!(map.format_span(Span::new(id, 2, 4)), "utf8.va:1:2");
        assert_eq!(map.format_span(Span::new(id, 1, 2)), "<unknown>:1-2");
    }
}
