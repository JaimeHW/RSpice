//! Diagnostics resolved against the original preprocessing dependency closure.

use crate::preprocessor::{PreprocessedDependency, PreprocessedSource};
use crate::{
    CompileDiagnosticPhase, CompileDiagnosticSeverity, CompileDiagnosticSpan, CompileError,
};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// A compiler diagnostic with locations in the source the author supplied.
///
/// Byte ranges are half-open UTF-8 ranges in `path`, not in expanded compiler
/// input. Lines and Unicode scalar columns are one-based. When preprocessing
/// changed a line (for example, a macro invocation), text it left alone keeps
/// its exact position and a location inside the expansion selects the
/// original invocation, never an expanded token as an authored range.
/// Diagnostics without a source span retain `None` locations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SourceCompileDiagnostic {
    pub severity: CompileDiagnosticSeverity,
    pub phase: CompileDiagnosticPhase,
    pub code: String,
    pub message: String,
    /// Display path, with non-Unicode filesystem names represented lossily.
    pub path: Option<String>,
    pub byte_start: Option<usize>,
    pub byte_end: Option<usize>,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

impl std::fmt::Display for SourceCompileDiagnostic {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let severity = match self.severity {
            CompileDiagnosticSeverity::Error => "Error",
            CompileDiagnosticSeverity::Warning => "Warning",
        };
        write!(formatter, "{severity}: {}", self.located())
    }
}

impl SourceCompileDiagnostic {
    /// The finding without its severity word, `path:line:column: [code]
    /// message`, for a host that marks the severity itself.
    pub fn located(&self) -> impl std::fmt::Display + '_ {
        Located(self)
    }
}

struct Located<'a>(&'a SourceCompileDiagnostic);

impl std::fmt::Display for Located<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let diagnostic = self.0;
        if let Some(path) = &diagnostic.path {
            formatter.write_str(path)?;
        }
        if let Some(line) = diagnostic.line {
            write!(formatter, ":{line}")?;
            if let Some(column) = diagnostic.column {
                write!(formatter, ":{column}")?;
            }
        }
        if diagnostic
            .path
            .as_ref()
            .is_some_and(|path| !path.is_empty())
            || diagnostic.line.is_some()
        {
            formatter.write_str(": ")?;
        }
        write!(formatter, "[{}] {}", diagnostic.code, diagnostic.message)
    }
}

impl From<&crate::PreprocessorError> for SourceCompileDiagnostic {
    fn from(error: &crate::PreprocessorError) -> Self {
        Self {
            severity: CompileDiagnosticSeverity::Error,
            phase: CompileDiagnosticPhase::Input,
            code: if error.cancelled {
                "VA-INPUT-CANCELLED"
            } else if error.resource_limit.is_some() {
                "VA-INPUT-RESOURCE-LIMIT"
            } else if error.io_error.is_some() {
                "VA-INPUT-IO"
            } else {
                "VA-INPUT-PREPROCESS"
            }
            .into(),
            message: error.message.clone(),
            path: error.file.as_ref().map(|path| path.display().to_string()),
            byte_start: None,
            byte_end: None,
            line: (error.line > 0).then_some(error.line),
            column: error.column.filter(|_| error.line > 0),
        }
    }
}

pub(crate) fn provider_diagnostics(
    error: &CompileError,
    preprocessed: &PreprocessedSource,
    dependencies: &[PreprocessedDependency],
) -> Vec<SourceCompileDiagnostic> {
    map_diagnostics(
        crate::compile_diagnostics(&preprocessed.source, error),
        preprocessed,
        dependencies,
    )
}

pub(crate) fn map_diagnostics(
    diagnostics: Vec<crate::CompileDiagnostic>,
    preprocessed: &PreprocessedSource,
    dependencies: &[PreprocessedDependency],
) -> Vec<SourceCompileDiagnostic> {
    map_diagnostics_with_sources(diagnostics, preprocessed, |path| {
        dependencies
            .iter()
            .find(|dependency| dependency.logical_path == path)
            .map(|dependency| dependency.source.as_str())
    })
}

pub(crate) fn map_diagnostics_with_sources<'a>(
    diagnostics: Vec<crate::CompileDiagnostic>,
    preprocessed: &'a PreprocessedSource,
    source_for_path: impl Fn(&Path) -> Option<&'a str>,
) -> Vec<SourceCompileDiagnostic> {
    diagnostics
        .into_iter()
        .map(|diagnostic| {
            let mapped = diagnostic
                .span
                .as_ref()
                .and_then(|span| preprocessed.map_span(span, &source_for_path));
            SourceCompileDiagnostic {
                severity: diagnostic.severity,
                phase: diagnostic.phase,
                code: diagnostic.code,
                message: diagnostic.message,
                path: mapped.as_ref().map(|span| span.path.display().to_string()),
                byte_start: mapped.as_ref().map(|span| span.byte_start),
                byte_end: mapped.as_ref().map(|span| span.byte_end),
                line: mapped.as_ref().map(|span| span.line),
                column: mapped.as_ref().map(|span| span.column),
            }
        })
        .collect()
}

/// Borrows the exact source snapshot, so editor consumers can authenticate it
/// while filesystem consumers need not clone entire documents into each error.
pub(crate) struct MappedSourceSpan<'a> {
    pub path: &'a Path,
    pub source: &'a str,
    pub byte_start: usize,
    pub byte_end: usize,
    pub line: usize,
    pub column: usize,
}

impl PreprocessedSource {
    pub(crate) fn map_span<'a>(
        &'a self,
        span: &CompileDiagnosticSpan,
        source_for_path: impl FnOnce(&Path) -> Option<&'a str>,
    ) -> Option<MappedSourceSpan<'a>> {
        let offset = usize::try_from(span.byte_start).ok()?;
        let segment = self.segment_at(offset)?;
        let source = source_for_path(&segment.logical_path)?;
        let line_range = source_line_range(source, segment.source_line)?;
        let expanded_line = &self.source[segment.expanded_start..segment.expanded_end];
        let original_line = &source[line_range.clone()];
        let exact_line = expanded_line.trim_end_matches(['\r', '\n'])
            == original_line.trim_end_matches(['\r', '\n']);
        let local_start = offset.saturating_sub(segment.expanded_start);
        let local_end = usize::try_from(span.byte_end)
            .ok()?
            .saturating_sub(segment.expanded_start);
        let start = line_range
            .start
            .saturating_add(local_start)
            .min(line_range.end);
        let end = line_range
            .start
            .saturating_add(local_end)
            .min(line_range.end)
            .max(start);
        let (byte_start, byte_end, column) =
            if exact_line && source.is_char_boundary(start) && source.is_char_boundary(end) {
                (
                    start,
                    end,
                    source[line_range.start..start].chars().count() + 1,
                )
            } else if exact_line {
                (line_range.start, line_range.end, 1)
            } else {
                let content = expanded_line.trim_end_matches(['\r', '\n']);
                let alignment =
                    LineAlignment::new(content, original_line.trim_end_matches(['\r', '\n']));
                let (range, column) = alignment.map(content, local_start, local_end);
                (
                    (line_range.start + range.start).min(line_range.end),
                    (line_range.start + range.end).min(line_range.end),
                    column,
                )
            };
        Some(MappedSourceSpan {
            path: &segment.logical_path,
            source,
            byte_start,
            byte_end,
            line: segment.source_line,
            column,
        })
    }
}

impl crate::runtime_report::SiteLines for PreprocessedSource {
    fn site_line(&self, span: crate::source::Span) -> Option<(Option<&Path>, usize)> {
        if span.source.raw() != 0 {
            return None;
        }
        let segment = self.segment_at(usize::try_from(span.start).ok()?)?;
        Some((Some(segment.logical_path.as_path()), segment.source_line))
    }
}

/// How a line the preprocessor changed lines up with its original.
///
/// Expansion rewrites the middle of a line: the content before the first
/// invocation and after the last one is unchanged. A location in either
/// shared run maps to its exact original column; a location inside the
/// rewritten middle maps to the original invocation (or invocations).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LineAlignment {
    /// Leading content bytes both lines share.
    prefix: usize,
    /// Trailing content bytes both lines share, disjoint from `prefix`.
    suffix: usize,
    original_len: usize,
    expanded_len: usize,
    /// Unicode scalars of the original between `prefix` and `suffix`.
    changed_chars: usize,
}

impl LineAlignment {
    /// Both arguments are line content without the line terminator.
    pub(crate) fn new(expanded: &str, original: &str) -> Self {
        let prefix: usize = expanded
            .chars()
            .zip(original.chars())
            .take_while(|(a, b)| a == b)
            .map(|(a, _)| a.len_utf8())
            .sum();
        let room = expanded.len().min(original.len()) - prefix;
        let mut suffix = 0;
        for (a, b) in expanded.chars().rev().zip(original.chars().rev()) {
            if a != b || suffix + a.len_utf8() > room {
                break;
            }
            suffix += a.len_utf8();
        }
        Self {
            prefix,
            suffix,
            original_len: original.len(),
            expanded_len: expanded.len(),
            changed_chars: original[prefix..original.len() - suffix].chars().count(),
        }
    }

    /// The original byte range (relative to the line start) and one-based
    /// column of the expanded-line range `start..end`; `expanded` is the
    /// expanded line content the alignment was built from.
    pub(crate) fn map(
        &self,
        expanded: &str,
        start: usize,
        end: usize,
    ) -> (std::ops::Range<usize>, usize) {
        let floor = |mut offset: usize| {
            offset = offset.min(expanded.len());
            while !expanded.is_char_boundary(offset) {
                offset -= 1;
            }
            offset
        };
        let changed_end = self.original_len - self.suffix;
        let suffix_start = self.expanded_len - self.suffix;
        let prefix_chars = expanded[..self.prefix].chars().count();
        // (original offset, column) of one expanded offset, and whether it
        // fell inside the rewritten middle.
        let locate = |offset: usize| {
            if offset < self.prefix {
                let offset = floor(offset);
                (offset, expanded[..offset].chars().count() + 1, false)
            } else if offset >= suffix_start {
                let beyond = offset.saturating_sub(self.expanded_len);
                let within = floor(offset) - suffix_start;
                let column = prefix_chars
                    + self.changed_chars
                    + expanded[suffix_start..suffix_start + within]
                        .chars()
                        .count()
                    + beyond
                    + 1;
                (changed_end + within + beyond, column, false)
            } else {
                (self.prefix, prefix_chars + 1, true)
            }
        };
        let (original_start, column, inside) = locate(start);
        let (original_end, _, end_inside) = locate(end.max(start));
        let original_end = if inside || end_inside {
            original_end.max(changed_end)
        } else {
            original_end
        };
        (original_start..original_end.max(original_start), column)
    }
}

pub(crate) fn source_line_range(
    source: &str,
    one_based_line: usize,
) -> Option<std::ops::Range<usize>> {
    if one_based_line == 0 {
        return None;
    }
    let mut start = 0usize;
    for (index, line) in source.split_inclusive('\n').enumerate() {
        let end = start.saturating_add(line.len());
        if index + 1 == one_based_line {
            return Some(start..end);
        }
        start = end;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{ParseError, ParseErrorKind};
    use crate::source::{SourceId, Span};
    use crate::virtual_source::VirtualBundleProvider;
    use crate::{Preprocessor, VirtualCompileLimits, VirtualSourceBundle};

    #[test]
    fn prepared_mapping_matches_live_mapping_for_all_offsets() {
        let bundle = VirtualSourceBundle::from_sources(
            "root.va",
            [
                ("root.va", "`include \"first.va\"\n`include \"second.va\"\n"),
                ("first.va", "// header\r\n\u{3b1} @\r\nlast line\r\n"),
                ("second.va", "`define BAD missing\n\t`BAD\n\u{3b2}"),
            ],
        )
        .unwrap();
        let provider = VirtualBundleProvider::new(&bundle, VirtualCompileLimits::default());
        let mut pp = Preprocessor::new();
        let preprocessed = pp
            .preprocess_provider_root_mapped(&provider, Path::new("root.va"))
            .unwrap();
        let prepared = crate::prepared_diagnostics::PreparedSourceMap::new(&preprocessed, |path| {
            pp.dependency_documents()
                .iter()
                .find(|document| document.logical_path == path)
                .map(|document| document.source.as_str())
        });
        // Include non-boundaries, cross-line spans and EOF, as well as normal
        // token ranges. Neither adapter may split an original UTF-8 scalar.
        for start in 0..=preprocessed.source.len() {
            for end in [
                start,
                (start + 1).min(preprocessed.source.len()),
                preprocessed.source.len(),
            ] {
                let error = CompileError::Parser(ParseError::new(
                    ParseErrorKind::InvalidExpression,
                    Span::new(SourceId::new(0), start as u32, end as u32),
                ));
                assert_eq!(
                    prepared.diagnostics(&preprocessed.source, &error),
                    provider_diagnostics(&error, &preprocessed, pp.dependency_documents()),
                    "span {start}..{end}"
                );
            }
        }
    }

    #[test]
    fn collected_diagnostics_keep_each_original_document_and_unicode_position() {
        let first = "// header\r\n\u{3b1} @\r\n";
        let second = "`define BAD #\n`BAD\n";
        let bundle = VirtualSourceBundle::from_sources(
            "root.va",
            [
                ("root.va", "`include \"first.va\"\n`include \"second.va\"\n"),
                ("first.va", first),
                ("second.va", second),
            ],
        )
        .unwrap();
        let provider = VirtualBundleProvider::new(&bundle, VirtualCompileLimits::default());
        let mut pp = Preprocessor::new();
        let preprocessed = pp
            .preprocess_provider_root_mapped(&provider, Path::new("root.va"))
            .unwrap();
        let [first_error, second_error] = ['@', '#'].map(|token| {
            let offset = preprocessed.source.find(token).unwrap() as u32;
            CompileError::Parser(ParseError::new(
                ParseErrorKind::InvalidExpression,
                Span::new(SourceId::new(0), offset, offset + 1),
            ))
        });
        let error = CompileError::Multiple(vec![
            first_error,
            CompileError::Multiple(vec![second_error]),
            CompileError::ModuleSelection("missing module".into()),
        ]);
        let diagnostics = provider_diagnostics(&error, &preprocessed, pp.dependency_documents());
        assert_eq!(diagnostics.len(), 3);
        let exact = &diagnostics[0];
        assert_eq!(exact.path.as_deref(), Some("first.va"));
        assert_eq!(exact.line, Some(2));
        assert_eq!(exact.column, Some(3));
        assert_eq!(
            &first[exact.byte_start.unwrap()..exact.byte_end.unwrap()],
            "@"
        );
        let expanded = &diagnostics[1];
        assert_eq!(expanded.path.as_deref(), Some("second.va"));
        assert_eq!(expanded.line, Some(2));
        assert_eq!(expanded.column, Some(1));
        assert_eq!(
            &second[expanded.byte_start.unwrap()..expanded.byte_end.unwrap()],
            "`BAD"
        );
        assert_eq!(
            diagnostics[2].phase,
            CompileDiagnosticPhase::ModuleSelection
        );
        assert!(diagnostics[2].path.is_none());
        assert!(diagnostics[2].line.is_none());
        assert!(diagnostics[2].byte_start.is_none());
    }

    /// On a line a macro invocation changed, text before and after the
    /// invocation keeps its own column and a location inside the expansion
    /// selects the invocation. Both mappers agree at every offset. It used to
    /// select the whole line at column 1.
    #[test]
    fn a_macro_expanded_line_keeps_unchanged_columns_and_selects_the_invocation() {
        let root = "`define BAD (1 +)\n\u{3b1} = `BAD * \u{3b2} @;\n";
        let bundle = VirtualSourceBundle::from_sources("root.va", [("root.va", root)]).unwrap();
        let provider = VirtualBundleProvider::new(&bundle, VirtualCompileLimits::default());
        let mut pp = Preprocessor::new();
        let preprocessed = pp
            .preprocess_provider_root_mapped(&provider, Path::new("root.va"))
            .unwrap();
        let prepared = crate::prepared_diagnostics::PreparedSourceMap::new(&preprocessed, |path| {
            pp.dependency_documents()
                .iter()
                .find(|document| document.logical_path == path)
                .map(|document| document.source.as_str())
        });
        let line_start = preprocessed.source.find('\u{3b1}').unwrap();
        let locate = |needle: &str| {
            let offset =
                (line_start + preprocessed.source[line_start..].find(needle).unwrap()) as u32;
            let error = CompileError::Parser(ParseError::new(
                ParseErrorKind::InvalidExpression,
                Span::new(SourceId::new(0), offset, offset + needle.len() as u32),
            ));
            let live = provider_diagnostics(&error, &preprocessed, pp.dependency_documents());
            assert_eq!(prepared.diagnostics(&preprocessed.source, &error), live);
            let diagnostic = live.into_iter().next().unwrap();
            (
                diagnostic.column.unwrap(),
                root[diagnostic.byte_start.unwrap()..diagnostic.byte_end.unwrap()].to_string(),
            )
        };
        // `α = ` is shared, so `=` keeps column 3.
        assert_eq!(locate("="), (3, "=".into()));
        // Inside the expansion: the invocation, at its backtick.
        assert_eq!(locate("+"), (5, "`BAD".into()));
        assert_eq!(locate("(1 +)"), (5, "`BAD".into()));
        // After it: `β` and `@` keep their authored columns.
        assert_eq!(locate("\u{3b2}"), (12, "\u{3b2}".into()));
        assert_eq!(locate("@"), (14, "@".into()));
        // Spans from every offset of the line to the end of input agree too.
        for start in line_start..preprocessed.source.len() {
            if !preprocessed.source.is_char_boundary(start) {
                continue;
            }
            let error = CompileError::Parser(ParseError::new(
                ParseErrorKind::InvalidExpression,
                Span::new(
                    SourceId::new(0),
                    start as u32,
                    preprocessed.source.len() as u32,
                ),
            ));
            assert_eq!(
                prepared.diagnostics(&preprocessed.source, &error),
                provider_diagnostics(&error, &preprocessed, pp.dependency_documents()),
                "offset {start}"
            );
        }
    }
}
