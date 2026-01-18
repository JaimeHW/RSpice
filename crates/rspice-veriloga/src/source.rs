//! Source location tracking and source map management
//!
//! This module provides infrastructure for tracking source locations throughout
//! the compilation pipeline, enabling precise error messages and source maps.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};

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

    /// Get the length of this span in bytes
    pub fn len(self) -> u32 {
        self.end - self.start
    }

    /// Check if this span is empty
    pub fn is_empty(self) -> bool {
        self.start == self.end
    }
}

/// Information about a source file
#[derive(Debug, Clone)]
pub struct SourceInfo {
    /// File path (or "<input>" for inline sources)
    pub path: PathBuf,
    /// Complete source text
    pub content: String,
    /// Line start byte offsets (for efficient line lookup)
    line_starts: Vec<u32>,
}

impl SourceInfo {
    fn new(path: impl Into<PathBuf>, content: String) -> Self {
        let line_starts = std::iter::once(0)
            .chain(
                content
                    .bytes()
                    .enumerate()
                    .filter(|(_, b)| *b == b'\n')
                    .map(|(i, _)| (i + 1) as u32),
            )
            .collect();

        Self {
            path: path.into(),
            content,
            line_starts,
        }
    }

    /// Get line and column (both 1-indexed) for a byte offset
    pub fn line_col(&self, offset: u32) -> (u32, u32) {
        let line = self
            .line_starts
            .partition_point(|&start| start <= offset)
            .saturating_sub(1);
        let col = offset - self.line_starts[line];
        (line as u32 + 1, col + 1)
    }

    /// Get the line content for a given line number (1-indexed)
    pub fn get_line(&self, line: u32) -> Option<&str> {
        let idx = line as usize - 1;
        if idx >= self.line_starts.len() {
            return None;
        }

        let start = self.line_starts[idx] as usize;
        let end = self
            .line_starts
            .get(idx + 1)
            .map(|&e| e as usize)
            .unwrap_or(self.content.len());

        Some(self.content[start..end].trim_end_matches(|c| c == '\r' || c == '\n'))
    }
}

/// Source map for tracking all source files in a compilation
#[derive(Debug, Default)]
pub struct SourceMap {
    sources: HashMap<SourceId, SourceInfo>,
    next_id: AtomicU32,
}

impl SourceMap {
    /// Create a new empty source map
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a source file to the map
    pub fn add_source(&self, _path: impl Into<PathBuf>, _content: impl Into<String>) -> SourceId {
        let id = SourceId::new(self.next_id.fetch_add(1, Ordering::Relaxed));
        // Note: In production, this would need interior mutability (e.g., RwLock)
        // For now, we'll require mutable access
        id
    }

    /// Add a source file (requires mutable access)
    pub fn add_source_mut(
        &mut self,
        path: impl Into<PathBuf>,
        content: impl Into<String>,
    ) -> SourceId {
        let id = SourceId::new(self.next_id.fetch_add(1, Ordering::Relaxed));
        self.sources
            .insert(id, SourceInfo::new(path, content.into()));
        id
    }

    /// Get source info for a source ID
    pub fn get(&self, id: SourceId) -> Option<&SourceInfo> {
        self.sources.get(&id)
    }

    /// Format a span for error display
    pub fn format_span(&self, span: Span) -> String {
        if let Some(source) = self.get(span.source) {
            let (line, col) = source.line_col(span.start);
            format!("{}:{}:{}", source.path.display(), line, col)
        } else {
            format!("<unknown>:{}-{}", span.start, span.end)
        }
    }

    /// Get the source text for a span
    pub fn span_text(&self, span: Span) -> Option<&str> {
        self.get(span.source)
            .map(|s| &s.content[span.start as usize..span.end as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_creation() {
        let span = Span::new(SourceId(0), 10, 20);
        assert_eq!(span.len(), 10);
        assert!(!span.is_empty());
    }

    #[test]
    fn test_span_extend() {
        let a = Span::new(SourceId(0), 10, 20);
        let b = Span::new(SourceId(0), 15, 30);
        let extended = a.extend(b);
        assert_eq!(extended.start, 10);
        assert_eq!(extended.end, 30);
    }

    #[test]
    fn test_source_info_line_col() {
        let source = SourceInfo::new("<test>", "line1\nline2\nline3".to_string());
        assert_eq!(source.line_col(0), (1, 1));
        assert_eq!(source.line_col(5), (1, 6)); // newline char
        assert_eq!(source.line_col(6), (2, 1)); // start of line 2
        assert_eq!(source.line_col(12), (3, 1)); // start of line 3
    }

    #[test]
    fn test_source_info_get_line() {
        let source = SourceInfo::new("<test>", "line1\nline2\nline3".to_string());
        assert_eq!(source.get_line(1), Some("line1"));
        assert_eq!(source.get_line(2), Some("line2"));
        assert_eq!(source.get_line(3), Some("line3"));
        assert_eq!(source.get_line(4), None);
    }
}
