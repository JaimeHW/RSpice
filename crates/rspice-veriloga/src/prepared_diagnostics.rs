//! Compact original-source coordinates retained after preparation.
//!
//! Original documents can be released once their line ranges and expansion
//! status are indexed. Exact lines use the retained expanded input to count
//! Unicode columns; changed lines select the original macro invocation.

use crate::preprocessor::PreprocessedSource;
use crate::{CompileDiagnosticSpan, CompileError, SourceCompileDiagnostic};
use std::collections::HashMap;
use std::ops::Range;
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub(crate) struct PreparedSourceMap {
    paths: Vec<PathBuf>,
    segments: Vec<SourceSegment>,
}

#[derive(Debug)]
struct SourceSegment {
    expanded: Range<usize>,
    original: Range<usize>,
    path: usize,
    line: usize,
    content_bytes: usize,
    exact: bool,
}

impl PreparedSourceMap {
    pub(crate) fn new<'a>(
        preprocessed: &PreprocessedSource,
        source_for_path: impl Fn(&Path) -> Option<&'a str>,
    ) -> Self {
        let mut result = Self::default();
        // Index each document once, avoiding a scan from its beginning for
        // every expanded line. These borrowed documents are not retained.
        let mut documents = HashMap::new();
        for segment in &preprocessed.segments {
            let Some((path, source, lines)) = documents
                .entry(segment.logical_path.as_path())
                .or_insert_with(|| {
                    let source = source_for_path(&segment.logical_path)?;
                    let lines: Vec<_> = source
                        .split_inclusive('\n')
                        .scan(0, |start, line| {
                            let range = *start..*start + line.len();
                            *start = range.end;
                            Some(range)
                        })
                        .collect();
                    let path = result.paths.len();
                    result.paths.push(segment.logical_path.clone());
                    Some((path, source, lines))
                })
                .as_ref()
            else {
                continue;
            };
            let Some(original) = segment
                .source_line
                .checked_sub(1)
                .and_then(|line| lines.get(line))
            else {
                continue;
            };
            let expanded = segment.expanded_start..segment.expanded_end;
            let original_content = source[original.clone()].trim_end_matches(['\r', '\n']);
            let exact = preprocessed.source[expanded.clone()].trim_end_matches(['\r', '\n'])
                == original_content;
            result.segments.push(SourceSegment {
                expanded,
                original: original.clone(),
                path: *path,
                line: segment.source_line,
                content_bytes: original_content.len(),
                exact,
            });
        }
        result
    }

    pub(crate) fn diagnostics(
        &self,
        source: &str,
        error: &CompileError,
    ) -> Vec<SourceCompileDiagnostic> {
        crate::compile_diagnostics(source, error)
            .into_iter()
            .map(|diagnostic| {
                let mapped = diagnostic
                    .span
                    .as_ref()
                    .and_then(|span| self.map_span(source, span));
                SourceCompileDiagnostic {
                    severity: diagnostic.severity,
                    phase: diagnostic.phase,
                    code: diagnostic.code,
                    message: diagnostic.message,
                    path: mapped.as_ref().map(|span| span.path.display().to_string()),
                    byte_start: mapped.as_ref().map(|span| span.range.start),
                    byte_end: mapped.as_ref().map(|span| span.range.end),
                    line: mapped.as_ref().map(|span| span.line),
                    column: mapped.as_ref().map(|span| span.column),
                }
            })
            .collect()
    }

    fn map_span(
        &self,
        source: &str,
        span: &CompileDiagnosticSpan,
    ) -> Option<SourceCoordinates<'_>> {
        let offset = usize::try_from(span.byte_start).ok()?;
        let index = self
            .segments
            .partition_point(|segment| segment.expanded.end <= offset);
        let segment = self
            .segments
            .get(index)
            .filter(|segment| segment.expanded.contains(&offset))
            .or_else(|| {
                self.segments.last().filter(|segment| {
                    offset == source.len() && segment.expanded.end == source.len()
                })
            })?;
        let start = offset
            .saturating_sub(segment.expanded.start)
            .min(segment.original.len());
        let end = usize::try_from(span.byte_end)
            .ok()?
            .saturating_sub(segment.expanded.start)
            .min(segment.original.len())
            .max(start);
        let expanded_line = &source[segment.expanded.clone()];
        // Original and expanded line endings may differ. Beyond their shared
        // content, every original byte is an ASCII CR or LF boundary.
        let boundary =
            |offset| offset >= segment.content_bytes || expanded_line.is_char_boundary(offset);
        let (range, column) = if segment.exact && boundary(start) && boundary(end) {
            (
                segment.original.start + start..segment.original.start + end,
                expanded_line[..start.min(segment.content_bytes)]
                    .chars()
                    .count()
                    + start.saturating_sub(segment.content_bytes)
                    + 1,
            )
        } else {
            (segment.original.clone(), 1)
        };
        Some(SourceCoordinates {
            path: &self.paths[segment.path],
            range,
            line: segment.line,
            column,
        })
    }
}

struct SourceCoordinates<'a> {
    path: &'a Path,
    range: Range<usize>,
    line: usize,
    column: usize,
}
